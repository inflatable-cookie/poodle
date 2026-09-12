/**
 * g18.027 public-surface freeze audit.
 *
 * Classifies the public delta between two immutable git refs without building
 * either tree. Every row is derived from committed sources at both refs, so the
 * output is reproducible from a clean clone:
 *
 *   bun scripts/audit-public-surface-delta.ts --base v0.3.0 --head <sha>
 *
 * Surfaces covered: package entries, dependency and peer constraints, root and
 * subpath exports, Svelte component prop/default surfaces, token CSS custom
 * properties, and Rust `pub` declarations in the public-intent crates named by
 * `packages/release-manifest.json`.
 *
 * This is audit tooling, not a runtime dependency. It reads only.
 */
import { execFileSync } from "node:child_process";
import { resolve } from "node:path";

export type ChangeClass = "additive" | "behavioral" | "breaking" | "internal-only";

export type DeltaRow = {
  surface:
    | "package-version"
    | "package-entry"
    | "package-dependency"
    | "package-peer-dependency"
    | "root-export"
    | "subpath-export"
    | "svelte-prop"
    | "react-prop"
    | "icon-identifier"
    | "css-custom-property"
    | "css-recipe-hook"
    | "rust-item";
  scope: string;
  name: string;
  before: string | null;
  after: string | null;
  classification: ChangeClass;
  note: string;
};

type Inventory = Record<string, string>;

const REPO_ROOT = resolve(import.meta.dir, "..");

export function git(args: string[]): string {
  return execFileSync("git", args, { cwd: REPO_ROOT, encoding: "utf8" });
}

export function listFiles(ref: string, dir: string, pattern?: RegExp): string[] {
  const out = git(["ls-tree", "-r", "--name-only", ref, dir]);
  return out
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean)
    .filter((file) => (pattern ? pattern.test(file) : true))
    .sort();
}

export function readAt(ref: string, path: string): string | null {
  try {
    return execFileSync("git", ["show", `${ref}:${path}`], {
      cwd: REPO_ROOT,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    });
  } catch {
    return null;
  }
}

// --- TypeScript export extraction -------------------------------------------

/** Names re-exported by an `export { ... } from "..."` block, tagged by kind. */
export function parseExportNames(source: string): string[] {
  const names: string[] = [];
  const block = /export\s+(type\s+)?\{([\s\S]*?)\}\s*from\s+"[^"]+"/g;
  for (const match of source.matchAll(block)) {
    const blockIsType = Boolean(match[1]);
    for (const raw of match[2].split(",")) {
      const specifier = raw.trim();
      if (!specifier) continue;
      const specifierIsType = blockIsType || specifier.startsWith("type ");
      const body = specifier.replace(/^type\s+/, "");
      const alias = body.match(/[A-Za-z_$][\w$]*\s+as\s+([A-Za-z_$][\w$]*)$/);
      const name = alias ? alias[1] : body.split(/\s+/)[0];
      if (!name) continue;
      names.push(`${specifierIsType ? "type" : "value"} ${name}`);
    }
  }
  for (const match of source.matchAll(/export\s*\{\s*default as\s+(\w+)\s*\}/g)) {
    names.push(`value ${match[1]}`);
  }
  // Direct declarations (`export type X = ...`, `export function x() {}`).
  const declaration =
    /export\s+(type\s+)?(interface|const|function|async\s+function|class|enum|let|var|type)\s+([A-Za-z_$][\w$]*)/g;
  for (const match of source.matchAll(declaration)) {
    const kind = match[1] || match[2] === "interface" || match[2] === "type" ? "type" : "value";
    names.push(`${kind} ${match[3]}`);
  }
  return [...new Set(names)].sort();
}

// --- Package entry/dependency extraction ------------------------------------

type PackageManifest = {
  version?: string;
  exports?: Record<string, unknown>;
  dependencies?: Record<string, string>;
  peerDependencies?: Record<string, string>;
};

export function parseManifest(source: string): PackageManifest {
  return JSON.parse(source) as PackageManifest;
}

function entryConditionTargets(entry: unknown): string[] {
  if (typeof entry === "string") return [entry];
  if (!entry || typeof entry !== "object") return [];
  return Object.values(entry as Record<string, unknown>).flatMap((value) =>
    typeof value === "string" ? [value] : entryConditionTargets(value),
  );
}

export function packageEntryInventory(manifest: PackageManifest): Inventory {
  const inventory: Inventory = {};
  for (const [key, value] of Object.entries(manifest.exports ?? {})) {
    inventory[key] = entryConditionTargets(value).join("|");
  }
  return inventory;
}

/**
 * Package entry subpaths are consumer-visible import specifiers; the condition
 * targets behind each subpath are compared as a value so a target reshuffle
 * surfaces even when the subpath name is unchanged.
 */

// --- Svelte prop surface -----------------------------------------------------

type SvelteProps = { name: string; declaration: string; default?: string };

function matchBraceBlock(source: string, openIndex: number): string | null {
  let depth = 0;
  for (let index = openIndex; index < source.length; index += 1) {
    const char = source[index];
    if (char === "{") depth += 1;
    if (char === "}") {
      depth -= 1;
      if (depth === 0) return source.slice(openIndex + 1, index);
    }
  }
  return null;
}

export function parseSvelteProps(source: string): SvelteProps[] {
  const props: SvelteProps[] = [];
  const interfaceMatch = source.match(/interface\s+Props\s*\{/);
  if (interfaceMatch?.index !== undefined) {
    const openIndex = interfaceMatch.index + interfaceMatch[0].length - 1;
    const body = matchBraceBlock(source, openIndex);
    if (body) {
      for (const line of body.split("\n")) {
        const declaration = line.trim();
        if (!declaration || declaration.startsWith("//") || declaration.startsWith("*")) continue;
        const field = declaration.match(/^([A-Za-z_$][\w$]*)\s*(\?)?\s*:\s*(.+?);?$/);
        if (field) props.push({ name: field[1], declaration: `${field[1]}${field[2] ?? ""}: ${field[3]}` });
      }
    }
  }
  // Defaults live in the `let { ... }: Props = $props()` destructuring.
  const destructure = source.match(/let\s*\{([\s\S]*?)\}\s*:\s*Props\s*=\s*\$props\(\)/);
  if (destructure) {
    for (const line of destructure[1].split("\n")) {
      const entry = line.trim().replace(/,$/, "");
      if (!entry) continue;
      const assignment = entry.match(/^([A-Za-z_$][\w$]*)\s*=\s*([\s\S]+)$/);
      if (!assignment) continue;
      const existing = props.find((prop) => prop.name === assignment[1]);
      const defaultValue = assignment[2].trim();
      const bindable = defaultValue.match(/^\$bindable\(\s*([\s\S]*?)\s*\)$/);
      if (existing) existing.default = bindable ? `${bindable[1]} (bindable)` : defaultValue;
    }
  }
  return props.sort((left, right) => left.name.localeCompare(right.name));
}

export function sveltePropInventory(files: Record<string, string>): Inventory {
  const inventory: Inventory = {};
  for (const [component, source] of Object.entries(files)) {
    for (const prop of parseSvelteProps(source)) {
      const suffix = prop.default === undefined ? "" : ` = ${prop.default}`;
      inventory[`${component}.${prop.name}`] = `${prop.declaration}${suffix}`;
    }
  }
  return inventory;
}

// --- React prop surface (private packed parity) ------------------------------

export function parseReactProps(source: string): SvelteProps[] {
  const props: SvelteProps[] = [];
  const declaration = source.match(/(?:export\s+)?(?:interface|type)\s+([A-Za-z0-9_]+Props)\b[^{]*\{/);
  if (declaration?.index === undefined) return props;
  const openIndex = declaration.index + declaration[0].length - 1;
  const body = matchBraceBlock(source, openIndex);
  if (!body) return props;
  for (const line of body.split("\n")) {
    const text = line.trim();
    if (!text || text.startsWith("//") || text.startsWith("*")) continue;
    const field = text.match(/^([A-Za-z_$][\w$]*)\s*(\?)?\s*:\s*(.+?);?$/);
    if (field) props.push({ name: field[1], declaration: `${field[1]}${field[2] ?? ""}: ${field[3]}` });
  }
  return props.sort((left, right) => left.name.localeCompare(right.name));
}

export function reactPropInventory(files: Record<string, string>): Inventory {
  const inventory: Inventory = {};
  for (const [component, source] of Object.entries(files)) {
    for (const prop of parseReactProps(source)) {
      inventory[`${component}.${prop.name}`] = prop.declaration;
    }
  }
  return inventory;
}

// --- Icon registry identifiers ----------------------------------------------

/** Icon registry keys are the public names consumers pass to `Icon`. */
export function iconIdentifierInventory(source: string): Inventory {
  const inventory: Inventory = {};
  for (const match of source.matchAll(/^\s*"([a-z0-9-]+)":\s*([A-Za-z0-9_]+),/gm)) {
    inventory[match[1]] = match[1];
  }
  return inventory;
}

// --- CSS custom properties ---------------------------------------------------

/**
 * Token custom properties are the public design-token surface. Only names, not
 * values, are keys; a changed value is reported as a row so a behavioral
 * change cannot hide behind a stable name.
 */
export function parseCustomProperties(css: string): Inventory {
  const inventory: Inventory = {};
  const declaration = /(--poodle-[a-z0-9-]+)\s*:\s*([^;]+);/g;
  for (const match of css.matchAll(declaration)) {
    inventory[match[1]] = match[2].trim().replace(/\s+/g, " ");
  }
  return inventory;
}

/**
 * Public recipe hooks are documented override seams read through
 * `var(--poodle-recipe-…)`. They are tracked separately from declared custom
 * properties because a removed seam has no declaration to diff.
 */
export function recipeHookInventory(source: string): Inventory {
  const inventory: Inventory = {};
  for (const match of source.matchAll(/--poodle-recipe-[a-z0-9-]+/g)) {
    inventory[match[0]] = match[0];
  }
  return inventory;
}

// --- Rust public declarations ------------------------------------------------

export function parseRustPublicItems(source: string): Inventory {
  const inventory: Inventory = {};
  const normalise = (text: string) => text.replace(/\s+/g, " ").trim();

  const struct = /pub\s+struct\s+([A-Za-z0-9_]+)\s*(<[^>]*>)?\s*\{([\s\S]*?)\n\s*\}/g;
  for (const match of source.matchAll(struct)) {
    const fields = [...match[3].matchAll(/pub\s+([A-Za-z0-9_]+)\s*:\s*([^,]+),/g)]
      .map((field) => `${field[1]}: ${normalise(field[2])}`)
      .sort();
    inventory[`struct ${match[1]}`] = `{ ${fields.join("; ")} }`;
  }

  const enumBlock = /pub\s+enum\s+([A-Za-z0-9_]+)\s*(<[^>]*>)?\s*\{([\s\S]*?)\n\s*\}/g;
  for (const match of source.matchAll(enumBlock)) {
    const variants = [...match[3].matchAll(/^\s*([A-Z][A-Za-z0-9_]*)\s*(\([^)]*\)|\{[^}]*\})?\s*,/gm)]
      .map((variant) => `${variant[1]}${normalise(variant[2] ?? "")}`)
      .sort();
    inventory[`enum ${match[1]}`] = `{ ${variants.join("; ")} }`;
  }

  // Signatures can wrap; capture to the opening brace or semicolon and squash.
  const fn = /pub\s+(?:unsafe\s+)?fn\s+([A-Za-z0-9_]+)\s*(<[^>]*>)?\(([\s\S]*?)\)\s*(->[^{;]*)?[{;]/g;
  for (const match of source.matchAll(fn)) {
    inventory[`fn ${match[1]}`] = `(${normalise(match[3])}) ${normalise(match[4] ?? "")}`.trim();
  }

  const other =
    /pub\s+(?:unsafe\s+)?(?:const|static|type|trait)\s+([A-Za-z0-9_]+)[^;={\n]*/g;
  for (const match of source.matchAll(other)) {
    inventory[match[0].replace(/\s+/g, " ").trim()] = normalise(match[0]);
  }

  const reexport = /pub\s+use\s+([^;]+);/g;
  for (const match of source.matchAll(reexport)) {
    const text = normalise(match[1]);
    const brace = text.indexOf("{");
    // Group re-export lists under their module path so adding or removing one
    // name is a value change, not a whole-line removal plus a whole-line add.
    const key =
      brace >= 0
        ? `use ${text.slice(0, brace).replace(/:+\s*$/, "").trim()}`
        : `use ${text}`;
    inventory[key] = text;
  }

  return inventory;
}

export function rustInventory(files: Record<string, string>): Inventory {
  const inventory: Inventory = {};
  for (const [path, source] of Object.entries(files)) {
    for (const [item, signature] of Object.entries(parseRustPublicItems(source))) {
      inventory[`${path}::${item}`] = signature;
    }
  }
  return inventory;
}

// --- Diff and classification -------------------------------------------------

export function diffInventory(
  surface: DeltaRow["surface"],
  scope: string,
  before: Inventory,
  after: Inventory,
): DeltaRow[] {
  const rows: DeltaRow[] = [];
  for (const name of Object.keys(before).sort()) {
    if (!(name in after)) {
      rows.push(classify(surface, scope, name, before[name], null));
    } else if (before[name] !== after[name]) {
      rows.push(classify(surface, scope, name, before[name], after[name]));
    }
  }
  for (const name of Object.keys(after).sort()) {
    if (!(name in before)) {
      rows.push(classify(surface, scope, name, null, after[name]));
    }
  }
  return rows;
}

export function classify(
  surface: DeltaRow["surface"],
  scope: string,
  name: string,
  before: string | null,
  after: string | null,
): DeltaRow {
  const added = before === null;
  const removed = after === null;
  const row = (classification: ChangeClass, note: string): DeltaRow => ({
    surface,
    scope,
    name,
    before,
    after,
    classification,
    note,
  });

  switch (surface) {
    case "package-version":
      return row("breaking", "Package version changed; the release candidate must bump every package in lockstep.");
    case "package-entry":
      if (added) return row("additive", "New package entry subpath; existing subpaths are unaffected.");
      if (removed) return row("breaking", "Package entry subpath removed; consumers importing it fail to resolve.");
      return row("breaking", "Package entry conditions or targets changed; consumers must re-verify resolution.");
    case "package-dependency":
      if (added) return row("additive", "New runtime dependency; the package now ships the capability it powers.");
      if (removed) return row("behavioral", "Runtime dependency removed; verify no consumer relied on its hoisted presence.");
      return row("behavioral", "Runtime dependency constraint changed; downstream resolution moves.");
    case "package-peer-dependency":
      if (added) return row("breaking", "New peer dependency; consumers must install or satisfy it explicitly.");
      if (removed) return row("behavioral", "Peer dependency removed; the package no longer requires it.");
      return row("breaking", "Peer constraint changed; a downstream install range may no longer satisfy it.");
    case "root-export":
    case "subpath-export":
      if (added) return row("additive", "New public export; existing imports are unaffected.");
      if (removed) return row("breaking", "Public export removed; consumers must migrate or stop importing it.");
      return row("breaking", "Public export shape changed; consumers must re-verify the import.");
    case "svelte-prop":
      if (added) return row("additive", "New component prop with a default; existing usage is unaffected.");
      if (removed) return row("breaking", "Component prop removed; passing it no longer has an effect.");
      return row("breaking", "Component prop type or default changed; behavior and types must be re-verified.");
    case "react-prop":
      if (added) return row("additive", "New packed React prop; parity surface only, React is not published.");
      if (removed) return row("breaking", "Packed React prop removed; React consumers must migrate.");
      return row("breaking", "Packed React prop type changed; React consumers must re-verify.");
    case "icon-identifier":
      if (added) return row("additive", "New icon registry identifier; existing icon names are unaffected.");
      if (removed) return row("breaking", "Icon registry identifier removed; consumers passing it lose the glyph.");
      return row("behavioral", "Icon identifier mapping changed.");
    case "css-custom-property": {
      if (added) return row("additive", "New custom property published from a public CSS surface.");
      if (removed) {
        if (scope.includes("/tokens/generated/")) {
          return row("breaking", "Public token custom property removed; theme overrides stop applying.");
        }
        return row("internal-only", "Undocumented implementation variable removed from a public stylesheet.");
      }
      return row("behavioral", "Custom property value changed; rendered output moves without an API change.");
    }
    case "css-recipe-hook":
      if (added) return row("additive", "New public recipe hook.");
      if (removed) return row("breaking", "Public recipe hook removed; overrides silently stop applying.");
      return row("behavioral", "Recipe hook value changed.");
    case "rust-item": {
      const isReexport = name.includes("::use ");
      if (isReexport) {
        const names = (value: string | null) =>
          new Set(
            (value ?? "")
              .replace(/^[^{]*\{/, "")
              .replace(/\}/, "")
              .split(",")
              .map((part) => part.trim())
              .filter(Boolean),
          );
        const beforeNames = names(before);
        const afterNames = names(after);
        const onlyAdded = [...beforeNames].every((entry) => afterNames.has(entry));
        if (onlyAdded) return row("additive", "Public re-export list grew; existing names still resolve.");
        return row("breaking", "Public re-export list changed; downstream `use` paths may no longer resolve.");
      }
      if (added) return row("additive", "New crate public item.");
      if (removed) return row("breaking", "Crate public item removed; downstream Rust source must migrate.");
      return row("breaking", "Crate public signature or shape changed; downstream Rust source must recompile.");
    }
  }
}

// --- Repository wiring -------------------------------------------------------

type SurfaceDefinition = {
  /** Manifest path relative to the repository root. */
  packagePath: string;
  /** Scope label used in rows and reports. */
  scope: string;
  manifest?: boolean;
  exportEntrySources?: string[];
  svelteComponents?: boolean;
  reactComponents?: boolean;
};

/**
 * Only packages with published or public-intent surfaces are audited. Tooling
 * and preview packages are internal by `packages/release-manifest.json`.
 */
export const SURFACE_DEFINITIONS: SurfaceDefinition[] = [
  {
    packagePath: "packages/core",
    scope: "@inflatable-cookie/poodle-core",
    manifest: true,
    exportEntrySources: [
      "packages/core/src/index.ts",
      "packages/core/src/tokens/index.ts",
      "packages/core/src/tokens/css.ts",
      "packages/core/src/tokens/themes.ts",
      "packages/core/src/tokens/metadata.ts",
      "packages/core/src/tokens/units.ts",
      "packages/core/src/tokens/runtime.ts",
      "packages/core/src/icons/index.ts",
    ],
  },
  {
    packagePath: "packages/svelte/components",
    scope: "@inflatable-cookie/poodle-svelte",
    manifest: true,
    exportEntrySources: [
      "packages/svelte/components/src/index.ts",
      "packages/svelte/components/src/markdown.ts",
      "packages/svelte/components/src/editor.ts",
      "packages/svelte/components/src/editor-codemirror.ts",
      "packages/svelte/components/src/rich-text.ts",
      "packages/svelte/components/src/types.ts",
    ],
    svelteComponents: true,
  },
  {
    packagePath: "packages/react/components",
    scope: "@inflatable-cookie/poodle-react",
    manifest: true,
    exportEntrySources: [
      "packages/react/components/src/index.ts",
      "packages/react/components/src/markdown.ts",
      "packages/react/components/src/editor.ts",
      "packages/react/components/src/editor-codemirror.ts",
      "packages/react/components/src/rich-text.ts",
      "packages/react/components/src/types.ts",
    ],
    reactComponents: true,
  },
];

export const CSS_ARTIFACTS = [
  "packages/core/src/tokens/generated/css/poodle-tokens.css",
  "packages/core/src/tokens/generated/css/poodle-themes.css",
  "packages/core/src/tokens/generated/css/poodle-theme-clay.css",
  "packages/core/src/tokens/generated/css/poodle-theme-iceberg.css",
  "packages/core/src/tokens/generated/css/poodle-theme-meadow.css",
];

/** Directory of the published token artifacts (`./tokens/*.css` entries). */
export const CSS_TOKEN_DIRECTORY = "packages/core/src/tokens/generated/css";

/** `./styles/*` is a public entry on `@inflatable-cookie/poodle-core`. */
export const CSS_STYLE_DIRECTORY = "packages/core/src/styles";

/** Aggregated component-style surface; a cross-file move is not a removal. */
export const CORE_STYLE_SCOPE = "@inflatable-cookie/poodle-core/styles";
export const RECIPE_HOOK_SCOPE = "@inflatable-cookie/poodle-core/styles + poodle-svelte components";

export function aggregateCustomProperties(files: Record<string, string>): Inventory {
  const values: Record<string, Set<string>> = {};
  for (const source of Object.values(files)) {
    for (const [name, value] of Object.entries(parseCustomProperties(source))) {
      (values[name] ??= new Set()).add(value);
    }
  }
  return Object.fromEntries(
    Object.entries(values).map(([name, set]) => [name, [...set].sort().join(" | ")]),
  );
}

export function aggregateRecipeHooks(files: Record<string, string>): Inventory {
  const hooks = new Set<string>();
  for (const source of Object.values(files)) {
    for (const name of Object.keys(recipeHookInventory(source))) hooks.add(name);
  }
  return Object.fromEntries([...hooks].sort().map((name) => [name, name]));
}

/** `export * from "./generated"`; the registry keys are the public names. */
export const ICON_REGISTRY_SOURCE = "packages/core/src/icons/generated.ts";

export const RUST_CRATES = [
  "packages/contracts/components",
  "packages/contracts/headless",
  "packages/contracts/node",
  "packages/contracts/ir",
  "packages/contracts/adapter",
  "packages/contracts/events",
  "packages/contracts/layout",
  "packages/contracts/markdown",
  "packages/contracts/style",
  "packages/contracts/tokens",
  "packages/render",
];

function readTree(ref: string, paths: string[]): Record<string, string> {
  const files: Record<string, string> = {};
  for (const path of paths) {
    const source = readAt(ref, path);
    if (source !== null) files[path] = source;
  }
  return files;
}

function inventoriesAt(ref: string): Record<string, Inventory> {
  const inventories: Record<string, Inventory> = {};
  const key = (surface: string, scope: string) => `${surface}::${scope}`;
  for (const definition of SURFACE_DEFINITIONS) {
    const manifestSource = readAt(ref, `${definition.packagePath}/package.json`);
    if (definition.manifest && manifestSource) {
      const manifest = parseManifest(manifestSource);
      inventories[key("package-version", definition.scope)] = {
        version: manifest.version ?? "(none)",
      };
      inventories[key("package-entry", definition.scope)] = packageEntryInventory(manifest);
      inventories[key("package-dependency", definition.scope)] = {
        ...(manifest.dependencies ?? {}),
      };
      inventories[key("package-peer-dependency", definition.scope)] = {
        ...(manifest.peerDependencies ?? {}),
      };
    }
    if (definition.exportEntrySources) {
      for (const entryPath of definition.exportEntrySources) {
        const source = readAt(ref, entryPath);
        if (source === null) continue;
        const label = entryPath.replace(`${definition.packagePath}/src/`, "").replace(/\.ts$/, "");
        inventories[key(label === "index" ? "root-export" : "subpath-export", `${definition.scope}/${label}`)] =
          Object.fromEntries(parseExportNames(source).map((name) => [name, name]));
      }
    }
    if (definition.svelteComponents) {
      const componentPaths = listFiles(ref, `${definition.packagePath}/src`, /\.svelte$/);
      inventories[key("svelte-prop", definition.scope)] = sveltePropInventory(
        readTree(ref, componentPaths),
      );
    }
    if (definition.reactComponents) {
      const componentPaths = listFiles(ref, `${definition.packagePath}/src`, /\.tsx$/);
      inventories[key("react-prop", definition.scope)] = reactPropInventory(
        readTree(ref, componentPaths),
      );
    }
  }
  const stylePaths = listFiles(ref, CSS_STYLE_DIRECTORY, /\.css$/);
  const svelteStylePaths = listFiles(ref, "packages/svelte/components/src", /\.svelte$/);
  // Token artifacts keep per-file identities: theme files intentionally carry
  // different values for the same token name.
  for (const path of listFiles(ref, CSS_TOKEN_DIRECTORY, /\.css$/)) {
    const source = readAt(ref, path);
    if (source === null) continue;
    inventories[key("css-custom-property", path)] = parseCustomProperties(source);
  }
  // Component styles are aggregated: a variable or hook that moves between
  // files in the shared foundation is not a public removal.
  inventories[key("css-custom-property", CORE_STYLE_SCOPE)] = aggregateCustomProperties(
    readTree(ref, stylePaths),
  );
  inventories[key("css-recipe-hook", RECIPE_HOOK_SCOPE)] = aggregateRecipeHooks(
    readTree(ref, [...stylePaths, ...svelteStylePaths]),
  );
  const iconSource = readAt(ref, ICON_REGISTRY_SOURCE);
  if (iconSource !== null) {
    inventories[key("icon-identifier", ICON_REGISTRY_SOURCE)] = iconIdentifierInventory(iconSource);
  }
  for (const crate of RUST_CRATES) {
    const rustPaths = listFiles(ref, `${crate}/src`, /\.rs$/);
    inventories[key("rust-item", crate)] = rustInventory(readTree(ref, rustPaths));
  }
  return inventories;
}

export function auditPublicSurfaceDelta(
  baseRef: string,
  headRef: string,
): { identities: Record<string, string>; rows: DeltaRow[] } {
  const before = inventoriesAt(baseRef);
  const after = inventoriesAt(headRef);
  const rows: DeltaRow[] = [];
  for (const key of [...new Set([...Object.keys(before), ...Object.keys(after)])].sort()) {
    const [surface, scope] = key.split("::");
    rows.push(...diffInventory(surface as DeltaRow["surface"], scope, before[key] ?? {}, after[key] ?? {}));
  }
  const identities: Record<string, string> = {
    baseRef,
    baseSha: git(["rev-parse", `${baseRef}^{commit}`]).trim(),
    baseTree: git(["rev-parse", `${baseRef}^{tree}`]).trim(),
    headRef,
    headSha: git(["rev-parse", `${headRef}^{commit}`]).trim(),
    headTree: git(["rev-parse", `${headRef}^{tree}`]).trim(),
  };
  return { identities, rows };
}

export function summarize(rows: DeltaRow[]): Record<ChangeClass, number> {
  return rows.reduce(
    (summary, row) => {
      summary[row.classification] += 1;
      return summary;
    },
    { additive: 0, behavioral: 0, breaking: 0, "internal-only": 0 },
  );
}

function main(): void {
  const args = process.argv.slice(2);
  const option = (flag: string, fallback: string): string => {
    const index = args.indexOf(flag);
    return index >= 0 && args[index + 1] ? args[index + 1] : fallback;
  };
  const base = option("--base", "v0.3.0");
  const head = option("--head", "HEAD");
  const json = args.includes("--json");
  const summaryOnly = args.includes("--summary");
  const { identities, rows } = auditPublicSurfaceDelta(base, head);

  if (json) {
    console.log(JSON.stringify({ identities, summary: summarize(rows), rows }, null, 2));
    return;
  }

  console.log(`# Public surface delta ${base} -> ${head}`);
  for (const [key, value] of Object.entries(identities)) console.log(`- ${key}: ${value}`);
  const summary = summarize(rows);
  console.log(
    `\nRows: ${rows.length} (additive ${summary.additive}, behavioral ${summary.behavioral}, breaking ${summary.breaking}, internal-only ${summary["internal-only"]})`,
  );
  if (summaryOnly) {
    const bySurface: Record<string, number> = {};
    for (const row of rows) {
      const key = `${row.surface}/${row.classification}`;
      bySurface[key] = (bySurface[key] ?? 0) + 1;
    }
    console.log("\n| Surface/class | Rows |");
    console.log("| --- | ---:");
    for (const [key, count] of Object.entries(bySurface).sort()) {
      console.log(`| ${key} | ${count} |`);
    }
    return;
  }
  console.log("\n| Surface | Scope | Name | Class | Before | After |");
  console.log("| --- | --- | --- | --- | --- | --- |");
  for (const row of rows) {
    const cell = (value: string | null) => (value ?? "(absent)").replace(/\|/g, "\\|");
    console.log(
      `| ${row.surface} | ${row.scope} | ${cell(row.name)} | ${row.classification} | ${cell(row.before)} | ${cell(row.after)} |`,
    );
  }
}

if (import.meta.main) main();
