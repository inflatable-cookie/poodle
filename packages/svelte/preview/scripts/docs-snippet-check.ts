/**
 * docs:snippet-check (g16.108) — compile every fenced `svelte` block in
 * `docs/guides/*.md` against the real poodle-svelte surface.
 *
 * Each self-contained fence becomes a component in a throwaway consumer
 * project under packages/svelte/preview/.snippet-check (removed on success)
 * that depends on the poodle packages by `file:` link exactly like
 * packages/svelte/install-smoke does. `svelte-check` then fails on any prop,
 * variant, type, or snippet shape the guides teach that the shipped
 * components no longer accept.
 *
 * Classification:
 * - A fence is emitted verbatim when every identifier its template
 *   expressions reference is declared in its own `<script>` or is a
 *   poodle-svelte export, and every component tag it uses is a poodle-svelte
 *   export.
 * - A markup-only fence whose expressions reference earlier-fence state as
 *   bare values (never property- or method-accessed) is emitted with a
 *   synthesized `<script lang="ts">`: the poodle-svelte exports its tags
 *   name, plus an implicit-any `let` stub per referenced value. The harness
 *   tsconfig keeps `strict` off, so the stubs stay assignable in both
 *   directions while the component API itself is fully typed.
 * - A fence that imports an application-local module (`$lib`, sibling
 *   packages), uses an application-owned component, or property-accesses
 *   earlier-fence state cannot compile standalone; it is skipped and
 *   counted with its reason so coverage stays visible.
 * - Fences are emitted in file order with a `NNN-slug` name per guide so a
 *   diagnostic points back at its doc.
 *
 * Runs from the svelte preview package (bun scripts/docs-snippet-check.ts).
 * Requires the built package dist (`effigy core:build` and
 * `effigy svelte:package`; `docs:check` runs both before this task) and
 * network/cache for the throwaway project's first `bun install` (svelte +
 * svelte-check), like the install-smoke project.
 */

import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const previewDir = path.resolve(scriptDir, "..");
const repoRoot = path.resolve(previewDir, "../../..");
const guidesDir = path.join(repoRoot, "docs", "guides");
const workDir = path.join(previewDir, ".snippet-check");

const SVELTE_KEYWORDS = new Set([
  "true", "false", "null", "undefined", "NaN", "this", "new", "typeof",
  "as", "in", "of", "if", "each", "await", "then", "catch", "key", "svelte",
]);

/** Browser/app globals a fragment may reference without owning them. */
const GLOBALS = new Set([
  "console", "crypto", "fetch", "window", "document", "navigator", "alert",
  "confirm", "prompt", "setTimeout", "clearTimeout", "setInterval",
  "clearInterval", "JSON", "Math", "Date", "URL", "FormData", "File", "Blob",
  "requestAnimationFrame", "location", "history", "localStorage",
  "sessionStorage", "performance", "TextEncoder", "TextDecoder",
  "AbortController", "Error", "Map", "Set", "Array", "Object", "String",
  "Number", "Boolean", "RegExp", "Symbol", "globalThis", "parseInt",
  "parseFloat", "isNaN", "encodeURIComponent", "decodeURIComponent", "btoa",
  "atob", "structuredClone", "queueMicrotask", "customElements",
]);

const ALLOWED_IMPORT_PREFIXES = [
  "@inflatable-cookie/poodle-svelte",
  "@inflatable-cookie/poodle-core",
  "svelte",
];

const FENCE_RE = /```svelte\n([\s\S]*?)```/g;

function sveltePackageExports(): Set<string> {
  const source = fs.readFileSync(
    path.join(repoRoot, "packages", "svelte", "components", "src", "index.ts"),
    "utf8",
  );
  const names = new Set<string>();
  for (const match of source.matchAll(/export\s+\{([^}]+)\}\s+from/g)) {
    for (const item of match[1].split(",")) {
      const trimmed = item.trim();
      const asMatch = trimmed.match(/^(?:default\s+as\s+)?([A-Za-z_$][\w$]*)/);
      if (asMatch) names.add(asMatch[1]);
    }
  }
  for (const match of source.matchAll(
    /^export\s+(?:type\s+)?(?:abstract\s+)?(?:class|function|const|let|var|interface|type)\s+([A-Za-z_$][\w$]*)/gm,
  )) {
    names.add(match[1]);
  }
  for (const match of source.matchAll(/^export\s+\{([^}]+)\}/gm)) {
    for (const item of match[1].split(",")) {
      const trimmed = item.trim();
      const asMatch = trimmed.match(/^(?:default\s+as\s+)?([A-Za-z_$][\w$]*)/);
      if (asMatch) names.add(asMatch[1]);
    }
  }
  return names;
}

function capitalize(value: string): string {
  return value.length > 0 ? value[0].toUpperCase() + value.slice(1) : value;
}

function scriptSection(body: string): string {
  const match = body.match(/<script[^>]*>([\s\S]*?)<\/script>/);
  return match ? match[1] : "";
}

function declaredNames(script: string): Set<string> {
  const names = new Set<string>();
  for (const match of script.matchAll(/\bimport\s*\{([^}]+)\}\s*from/g)) {
    for (const item of match[1].split(",")) {
      const trimmed = item.trim();
      const asMatch = trimmed.match(/^(?:type\s+)?([A-Za-z_$][\w$]*)/);
      if (asMatch) names.add(asMatch[1]);
    }
  }
  for (const match of script.matchAll(/\bimport\s+([A-Za-z_$][\w$]*)/g)) names.add(match[1]);
  for (const match of script.matchAll(/\b(?:let|const|var)\s+([A-Za-z_$][\w$]*)/g)) names.add(match[1]);
  for (const match of script.matchAll(/\b(?:function|class)\s+([A-Za-z_$][\w$]*)/g)) names.add(match[1]);
  return names;
}

function templateSection(body: string): string {
  const scriptClose = body.indexOf("</script>");
  return scriptClose >= 0 ? body.slice(scriptClose + "</script>".length) : body;
}

interface TemplateReference {
  name: string;
  /** Identifier is property-/method-accessed (needs a real value). */
  accessed: boolean;
}

/** Identifiers that appear in `{...}` expression slots and event/bind handlers. */
function referencedIdentifiers(template: string): TemplateReference[] {
  const found = new Map<string, boolean>();
  const expressions: string[] = [];
  for (const match of template.matchAll(/\{([^{}]*)\}/g)) {
    const expression = match[1];
    if (/^[#/:@]/.test(expression.trim())) continue;
    expressions.push(expression);
  }
  for (const match of template.matchAll(
    /\b(?:bind|on|use|transition|in|out|animate):[a-z]+\s*=\s*\{([^{}]*)\}/g,
  )) {
    expressions.push(match[1]);
  }
  for (const expression of expressions) {
    // Property keys in inline object literals are not identifier uses, and
    // string literals carry no identifiers.
    const withoutKeys = expression.replace(/([\s{,]\(?)([A-Za-z_$][\w$]*)\s*:/g, "$1");
    const expressionBody = withoutKeys.replace(/"[^"]*"|'[^']*'|`[^`]*`/g, " ");
    const paramScan = expressionBody;
    // Arrow-handler parameters are declared inside the expression.
    const params = new Set<string>();
    for (const paramMatch of expression.matchAll(/\(\s*([A-Za-z_$][\w$]*(?:\s*,\s*[A-Za-z_$][\w$]*)*)\s*\)\s*=>/g)) {
      for (const name of paramMatch[1].split(",")) {
        params.add(name.trim());
      }
    }
    for (const identMatch of expressionBody.matchAll(/[A-Za-z_$][\w$]*/g)) {
      const name = identMatch[0];
      if (params.has(name) || SVELTE_KEYWORDS.has(name) || GLOBALS.has(name)) continue;
      const after = expressionBody.slice(identMatch.index + name.length);
      const isCall = /^\s*\(/.test(after);
      const accessed = /^\s*[.[\[]/.test(after) && !isCall;
      found.set(name, (found.get(name) ?? false) || accessed);
    }
  }
  return [...found].map(([name, accessed]) => ({ name, accessed }));
}

function importSpecifiers(script: string): string[] {
  const specifiers: string[] = [];
  for (const match of script.matchAll(/\bfrom\s+["']([^"']+)["']/g)) specifiers.push(match[1]);
  for (const match of script.matchAll(/\bimport\s+["']([^"']+)["']/g)) specifiers.push(match[1]);
  return specifiers;
}

function capitalizeTags(template: string): Set<string> {
  const tags = new Set<string>();
  for (const match of template.matchAll(/<([A-Z][A-Za-z0-9_]*)\b/g)) tags.add(match[1]);
  return tags;
}

interface FenceDecision {
  kind: "emit" | "skip";
  body?: string;
  reason?: string;
}

function decideFence(body: string, exports: Set<string>): FenceDecision {
  const script = scriptSection(body);
  const template = templateSection(body);
  const hasOwnScript = /<script[^>]*>/.test(body.slice(0, 400));

  // Application-context imports ($lib, sibling packages, relative modules)
  // cannot resolve in the throwaway project.
  for (const specifier of importSpecifiers(script)) {
    if (specifier.startsWith(".") || !ALLOWED_IMPORT_PREFIXES.some((prefix) => specifier.startsWith(prefix))) {
      return { kind: "skip", reason: `imports ${specifier}` };
    }
  }

  const declared = declaredNames(script);
  const tags = capitalizeTags(template);
  const needsPoodleImports = [...tags].filter((tag) => exports.has(tag));
  const unknownTags = [...tags].filter(
    (tag) =>
      !exports.has(tag) &&
      !declared.has(tag) &&
      !declared.has(`default${tag}`) &&
      !declared.has(capitalize(tag)),
  );

  if (unknownTags.length > 0) {
    return { kind: "skip", reason: `uses application-owned component ${unknownTags[0]}` };
  }

  const references = referencedIdentifiers(template);
  const freeIdentifiers = references.filter(
    (reference) => !declared.has(reference.name) && !exports.has(reference.name),
  );
  const accessedFree = freeIdentifiers.filter((reference) => reference.accessed);
  if (accessedFree.length > 0) {
    return {
      kind: "skip",
      reason: `property-accesses earlier-fence context (${accessedFree[0].name})`,
    };
  }

  if (hasOwnScript) {
    if (freeIdentifiers.length > 0) {
      return { kind: "skip", reason: `references earlier-fence context (${freeIdentifiers[0].name})` };
    }
    return { kind: "emit", body };
  }

  const stubNames = [...new Set(freeIdentifiers.map((reference) => reference.name))].slice(0, 30);
  const importLine =
    needsPoodleImports.length > 0
      ? `  import { ${[...needsPoodleImports].sort().join(", ")} } from "@inflatable-cookie/poodle-svelte";\n`
      : "";
  const stubLines =
    stubNames.length > 0
      ? `\n  // Earlier-fence fragment context; implicit-any declarations (strict is\n  // off in the harness tsconfig) keep both prop and binding directions\n  // assignable while the component API itself stays fully typed.\n${stubNames
          .map((name) => `  let ${name};`)
          .join("\n")}\n`
      : "";
  if (stubNames.length === 0 && needsPoodleImports.length === 0) return { kind: "emit", body };
  return { kind: "emit", body: `<script lang="ts">\n${importLine}${stubLines}</script>\n${body}` };
}

function writeProject(emitted: { file: string; body: string }[]): void {
  fs.rmSync(workDir, { recursive: true, force: true });
  fs.mkdirSync(path.join(workDir, "snippets"), { recursive: true });
  fs.writeFileSync(
    path.join(workDir, "package.json"),
    JSON.stringify(
      {
        name: "poodle-docs-snippet-check",
        version: "0.0.0",
        private: true,
        type: "module",
        scripts: { check: "svelte-check --tsconfig ./tsconfig.json --threshold error" },
        dependencies: {
          "@inflatable-cookie/poodle-svelte": "file:../../components",
          "@inflatable-cookie/poodle-core": "file:../../../core",
          svelte: "^5.56.8",
        },
        devDependencies: {
          "svelte-check": "^4.7.4",
          typescript: "~6.0.0",
        },
      },
      null,
      2,
    ) + "\n",
  );
  fs.writeFileSync(
    path.join(workDir, "tsconfig.json"),
    JSON.stringify(
      {
        extends: "../../../../tsconfig.json",
        compilerOptions: {
          types: ["svelte"],
        },
        include: ["snippets/**/*.svelte"],
      },
      null,
      2,
    ) + "\n",
  );
  for (const entry of emitted) {
    fs.writeFileSync(path.join(workDir, "snippets", entry.file), entry.body);
  }
}

function requirePackageDist(): void {
  const coreTypes = path.join(repoRoot, "packages", "core", "dist", "index.d.ts");
  const svelteTypes = path.join(repoRoot, "packages", "svelte", "components", "dist", "index.d.ts");
  if (!fs.existsSync(coreTypes) || !fs.existsSync(svelteTypes)) {
    throw new Error(
      "docs:snippet-check needs the built package surfaces. Run `effigy core:build` and `effigy svelte:package` first (docs:check builds them before this task).",
    );
  }
}

function run(): void {
  requirePackageDist();
  const exports = sveltePackageExports();
  const emitted: { file: string; body: string }[] = [];
  const skipped: { file: string; reason: string }[] = [];
  const guideFiles = fs
    .readdirSync(guidesDir)
    .filter((name) => name.endsWith(".md"))
    .sort();

  for (const guide of guideFiles) {
    const markdown = fs.readFileSync(path.join(guidesDir, guide), "utf8");
    let index = 0;
    for (const match of markdown.matchAll(FENCE_RE)) {
      const fenceNumber = ++index;
      const body = match[1];
      const decision = decideFence(body, exports);
      const slug = guide.replace(/\.md$/, "");
      const fileName = `${slug}-${String(fenceNumber).padStart(2, "0")}.svelte`;
      if (decision.kind === "skip") {
        skipped.push({ file: fileName, reason: decision.reason ?? "unknown" });
        continue;
      }
      emitted.push({ file: fileName, body: decision.body ?? body });
    }
  }

  if (emitted.length === 0) {
    throw new Error("docs:snippet-check found no compilable fenced svelte blocks under docs/guides/");
  }

  writeProject(emitted);
  let svelteCheckError: string | null = null;
  try {
    execFileSync("bun", ["install", "--cwd", workDir, "--no-progress"], { stdio: "pipe" });
    execFileSync(
      path.join(workDir, "node_modules", ".bin", "svelte-check"),
      ["--tsconfig", "./tsconfig.json", "--threshold", "error"],
      { cwd: workDir, stdio: "pipe" },
    );
  } catch (error) {
    const execError = error as { stdout?: string | Buffer; stderr?: string | Buffer } | null;
    const output =
      execError?.stdout != null
        ? `${execError.stdout.toString()}${execError.stderr?.toString() ?? ""}`
        : String(error);
    svelteCheckError = output;
  }

  if (svelteCheckError) {
    console.error(svelteCheckError);
    console.error(
      `docs:snippet-check failed for ${emitted.length} guide snippets; worktree kept at ${workDir} for inspection.`,
    );
    process.exitCode = 1;
    return;
  }
  fs.rmSync(workDir, { recursive: true, force: true });
  console.log(
    `docs:snippet-check green: ${emitted.length} snippets from ${guideFiles.length} guides compile against the poodle-svelte surface; ${skipped.length} app-context fragments skipped:`,
  );
  for (const entry of skipped) {
    console.log(`  - ${entry.file}: ${entry.reason}`);
  }
}

run();
