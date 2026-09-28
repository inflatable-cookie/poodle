// Contract <-> Svelte prop-surface drift check.
//
// CLAUDE.md mandates that each component's contract (docs/contracts/components/
// <slug>.md) and its implementation stay in sync, but nothing enforced it. This
// compares the contract's "### Public Props" table against the authoritative
// Svelte component's `interface Props`, failing on any drift not recorded in the
// baseline below. Both directions are enforced: a documented prop missing from
// Svelte, and an implemented prop the contract does not document.
//
// Excluded from the Svelte side (framework idiom, not public props):
//   - Snippet-typed props (slots/children — documented separately in contracts)
//   - `on*` event callbacks (contracts document these in an Events section)
//   - the `[key: string]` index signature and `...restProps` passthrough

import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { allComponents } from "../src/component-registry.ts";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../..");
const contractsDir = path.join(repoRoot, "docs/contracts/components");
const svelteDir = path.join(repoRoot, "packages/svelte/components/src");

/** Renderer exports share the editor contract, which gives each surface its
 * own structurally identical props table. */
const CONTRACT_SURFACES: Record<string, { contractSlug: string; section?: "renderer" }> = {
  "rich-text-renderer": { contractSlug: "rich-text-editor", section: "renderer" },
  "markdown-renderer": { contractSlug: "markdown-editor", section: "renderer" },
};

/** Contract documents without a standalone Svelte component surface. Keep one
 * reason beside every exclusion so the reported count stays auditable. */
const CONTRACT_SKIP_REASONS: Record<string, string> = {
  "form-shell": "shared native composition contract; Svelte form orchestration stays caller-composed",
  "size-and-density": "cross-component presentation rules, not a rendered component",
  "surface-elevation": "cross-cutting surface-context contract, not a rendered component",
  "inline-remediation": "web callers compose Callout; standalone renderer is native-only",
  "format-display-date": "pure formatting utilities, not Svelte components",
  "format-file-size": "pure formatting utility, not a Svelte component",
  "tab-strip": "Svelte tablist is built into Tabs; no standalone component exists",
};

function collectContractFiles(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true })
    .flatMap((entry) => {
      const entryPath = path.join(directory, entry.name);
      if (entry.isDirectory()) return collectContractFiles(entryPath);
      return entry.isFile() && entry.name.endsWith(".md") && entry.name !== "README.md" ? [entryPath] : [];
    })
    .sort();
}

// Known, accepted drift: slug -> { contractOnly?: string[]; svelteOnly?: string[] }.
// Closing a drift means deleting its entry.
const BASELINE: Record<string, { contractOnly?: string[]; svelteOnly?: string[] }> = {
  // g13-027 Part 2 tranche (see the batch log): web-only or spec-surface-pending
  // props the contract deliberately does not table. Tabling them would fail
  // contract-spec-drift until the poodle-specs structs carry the fields, and
  // WEB_ONLY_PROPS is out of scope for this card.
  //
  // dialog `closeButtonSize` — cross-target close-button size; DialogSpec
  //   carries `show_close_button` only, so the field is a spec-surface tranche.
  // dialog `overlayStyle` — web-only styling passthrough (the spec register
  //   excuses `overlayClassName` but not this spelling; fixing the register is
  //   out of scope here).
  dialog: { svelteOnly: ["closeButtonSize", "overlayStyle"] },
  // dock-region `tabVariant` — cross-target strip control; DockRegionSpec
  //   models `tabs_placement` only — spec-surface tranche. (`showTabs` moved
  //   into the Public Props table with `DockRegionSpec::show_tabs`, g16.100.)
  // dock-region `tabActiveEdge` / `tabActiveFill` / `tabBordered` /
  //   `tabFullWidth` / `tabReorderable` — g13-040 tab pass-throughs; the
  //   whole `tabs_placement`-adjacent surface moves into the table when
  //   g13.014 gives DockRegionSpec its tab fields — spec-surface tranche.
  "dock-region": {
    svelteOnly: [
      "tabVariant",
      "tabActiveEdge",
      "tabActiveFill",
      "tabBordered",
      "tabFullWidth",
      "tabReorderable",
    ],
  },
  // popover `triggerIsInteractive` — DOM-only switch (documented in contract
  //   prose §TriggerIsInteractive); native composes its trigger directly and
  //   has no equivalent.
  popover: { svelteOnly: ["triggerIsInteractive"] },
  // split-view `minRatio` / `maxRatio` — cross-target ratio clamps; SplitViewSpec
  //   models `ratio` only — spec-surface tranche.
  "split-view": { svelteOnly: ["minRatio", "maxRatio"] },
};

/** Framework-idiom function types that are not public props. Snippet-typed
 * props are slot plumbing documented separately in contracts — the same
 * convention contract-value-domain-drift.ts applies. */
const FRAMEWORK_TYPES: Record<string, true> = { Snippet: true };

/** True when a type expression references a framework-idiom type. */
function isFrameworkType(expr: string): boolean {
  return Object.keys(FRAMEWORK_TYPES).some((t) => new RegExp(`\\b${t}\\b`).test(expr));
}

type ContractPropSection = "component" | "renderer";

interface PropTable {
  start: number;
  heading: string;
  lines: string[];
}

function tableCells(line: string): string[] {
  const cells: string[] = [];
  let cell = "";
  let escaped = false;
  for (const ch of line.trim()) {
    if (escaped) {
      cell += ch;
      escaped = false;
    } else if (ch === "\\") {
      escaped = true;
    } else if (ch === "|") {
      cells.push(cell.trim());
      cell = "";
    } else {
      cell += ch;
    }
  }
  cells.push(cell.trim());
  if (cells[0] === "") cells.shift();
  if (cells.at(-1) === "") cells.pop();
  return cells;
}

function isTableSeparator(line: string): boolean {
  const cells = tableCells(line);
  return cells.length > 0 && cells.every((cell) => /^:?-{3,}:?$/.test(cell));
}

function isPropTableHeader(line: string): boolean {
  const cells = tableCells(line).map((cell) => cell.replace(/`/g, "").toLowerCase());
  const typedProps =
    /^(prop|input)$/.test(cells[0] ?? "") &&
    cells.slice(1).some((cell) => /^(type|value type|shape)$/.test(cell));
  const discriminatedProps =
    cells[0] === "mode" && cells.includes("required") && cells.includes("rejected");
  return typedProps || discriminatedProps;
}

function propTables(md: string): PropTable[] {
  const lines = md.split(/\r?\n/);
  const tables: PropTable[] = [];
  const headings: Array<{ level: number; title: string; start: number }> = [];

  for (let i = 0; i < lines.length; i++) {
    const heading = lines[i].match(/^(#{1,6})\s+(.+?)\s*#*\s*$/);
    if (heading) {
      const level = heading[1].length;
      while (headings.length > 0 && headings.at(-1)!.level >= level) headings.pop();
      headings.push({ level, title: heading[2], start: i });
      continue;
    }

    if (!lines[i].includes("|") || !lines[i + 1] || !isTableSeparator(lines[i + 1])) continue;
    if (!isPropTableHeader(lines[i])) continue;

    const rows = [lines[i], lines[i + 1]];
    let end = i + 2;
    while (end < lines.length && lines[end].includes("|")) rows.push(lines[end++]);
    tables.push({ start: i, heading: headings.at(-1)?.title ?? "", lines: rows });
  }

  return tables;
}

/** Finds the component's prop table(s) by their column shape, so section-title
 * wording can vary. Multiple tables inside the same top-level section form a
 * union surface (for example, mode-specific props). Renderer tables are kept
 * separate from their editor surface when a contract documents both. */
export function contractProps(
  md: string,
  section: ContractPropSection = "component",
): { props: Set<string>; targetSpecific: Set<string>; found: boolean } {
  const props = new Set<string>();
  const targetSpecific = new Set<string>();
  const lines = md.split(/\r?\n/);
  const tables = propTables(md);
  if (tables.length === 0) return { props, targetSpecific, found: false };

  const first = tables[0];
  const firstTopLevelHeading = lines
    .slice(0, first.start)
    .map((line, index) => ({ line, index }))
    .filter(({ line }) => /^##\s+/.test(line))
    .at(-1);
  const sectionStart = firstTopLevelHeading?.index ?? -1;
  const nextTopLevelHeading = lines.findIndex((line, index) => index > first.start && /^##\s+/.test(line));
  const sectionEnd = nextTopLevelHeading < 0 ? lines.length : nextTopLevelHeading;
  const surfaceTables = tables.filter((table) => table.start >= sectionStart && table.start < sectionEnd);
  const selected = surfaceTables.filter((table) =>
    section === "renderer" ? /\brenderer\b/i.test(table.heading) : !/\brenderer\b/i.test(table.heading),
  );
  if (selected.length === 0) return { props, targetSpecific, found: false };

  for (const table of selected) {
    const headers = tableCells(table.lines[0]).map((cell) => cell.replace(/`/g, "").toLowerCase());
    if (headers[0] === "mode" && headers.includes("required") && headers.includes("rejected")) {
      const requiredIndex = headers.indexOf("required");
      const rejectedIndex = headers.indexOf("rejected");
      for (const line of table.lines.slice(2)) {
        const cells = tableCells(line);
        for (const match of (cells[requiredIndex] ?? "").matchAll(/\b([a-zA-Z_$][\w$]*)\s*:/g)) {
          props.add(match[1]);
        }
        for (const rejected of (cells[rejectedIndex] ?? "").split(",")) {
          const name = rejected.trim().replace(/^`|`$/g, "");
          if (/^[a-zA-Z_$][\w$]*$/.test(name) && !/^on[A-Z]/.test(name)) props.add(name);
        }
      }
      continue;
    }
    for (const line of table.lines.slice(2)) {
      // First table cell, honoring `\|` escapes: `| `x`, `y` | … |`.
      const cell = line.match(/^\|\s*((?:[^|\\]|\\.)*?)\s*\|/);
      if (!cell) continue;
      // A prop cell may join several names: `x`, `y` or `primaryHidden` /
      // `secondaryHidden` (both spellings in the corpus).
      const names = [...cell[1].matchAll(/`([a-zA-Z_$][\w$]*)`/g)].map((m) => m[1]);
      if (names.length === 0) continue;
      // A prop the contract marks as belonging to specific targets is not drift:
      // some state the DOM owns natively has to be a controlled prop where there
      // is no DOM. TextInput's caret is the case that forced this — `<input>`
      // owns its selection, GPUI and Jetstream have to be told. Marked in the
      // notes column as "**Rust targets only**" or similar. Such props are
      // documented, so they never count as undocumented on the Svelte side
      // either — only the contract-only direction ignores them.
      const targetOnly = /\*\*[^*]*targets only\*\*/i.test(line);
      for (const name of names) {
        if (/^on[A-Z]/.test(name)) continue;
        if (targetOnly) targetSpecific.add(name);
        else props.add(name);
      }
    }
  }
  return { props, targetSpecific, found: true };
}

// Extract the top-level prop names from the component's `let { ... } = $props()`
// destructure — uniform across Svelte 5 components (unlike the type declaration,
// which may be an interface, a type alias, or inline). Commas/colons/equals
// inside default values, generics, and object literals are skipped via depth;
// string literals are skipped too — a comma inside `placeholder = "a, b"` is
// content, not a prop boundary (the depth rules never saw it, which is how
// DateTimeZonePicker's `placeholder = "Select date, time, and zone"` leaked
// `time` and `and` as props).
export function svelteProps(src: string): Set<string> {
  const props = new Set<string>();
  const anchor = src.indexOf("= $props()");
  if (anchor < 0) return props;
  // The destructure is the FIRST brace group of `let { ... }` before $props()
  // (a following `: Type` / `: { ... }` annotation must not be mistaken for it).
  const letIdx = src.lastIndexOf("let {", anchor);
  if (letIdx < 0) return props;
  const open = src.indexOf("{", letIdx);
  let depth = 0;
  let close = -1;
  let quote: string | null = null;
  for (let i = open; i < src.length; i++) {
    const ch = src[i];
    if (quote !== null) {
      if (ch === "\\") i++;
      else if (ch === quote) quote = null;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === "`") {
      quote = ch;
      continue;
    }
    if (ch === "{") depth++;
    else if (ch === "}") {
      depth--;
      if (depth === 0) {
        close = i;
        break;
      }
    }
  }
  if (close < 0) return props;
  const body = src.slice(open + 1, close);
  // Split into top-level members on commas at depth 0.
  let d = 0;
  let cur = "";
  const parts: string[] = [];
  quote = null;
  for (let i = 0; i < body.length; i++) {
    const ch = body[i];
    if (quote !== null) {
      // String literal: the matching unescaped delimiter ends it; brackets and
      // commas inside are content, not structure.
      cur += ch;
      if (ch === "\\" && i + 1 < body.length) {
        cur += body[i + 1];
        i++;
      } else if (ch === quote) {
        quote = null;
      }
      continue;
    }
    if (ch === '"' || ch === "'" || ch === "`") {
      quote = ch;
      cur += ch;
      continue;
    }
    // The `>` of an arrow function is not a closing bracket. Counting it as one
    // drives depth negative, after which no comma reads as top-level and every
    // prop declared after the first arrow-function default is silently dropped
    // — the drift gate then reports them as contract-only.
    const isArrow = ch === ">" && body[i - 1] === "=";
    if ("{([<".includes(ch)) d++;
    else if (!isArrow && "})]>".includes(ch)) d--;
    if (ch === "," && d === 0) {
      parts.push(cur);
      cur = "";
    } else {
      cur += ch;
    }
  }
  if (cur.trim()) parts.push(cur);
  for (const part of parts) {
    const t = part.trim();
    if (!t || t.startsWith("...")) continue; // rest spread
    const m = t.match(/^([a-zA-Z_$][\w$]*)/);
    if (!m) continue;
    const name = m[1];
    if (/^on[A-Z]/.test(name)) continue; // event callback (documented separately)
    props.add(name);
  }
  return props;
}

// Snippet-typed prop names from the component's `Props` interface (or the inline
// `let { … }: { … } = $props()` annotation) — slot plumbing typed as props,
// which contracts document separately. Mirrors the parser
// contract-value-domain-drift.ts uses; excluded here so the reverse drift
// direction sees props, not snippets.
/**
 * Member text of a discriminated-union Props declaration —
 * `interface CommonProps { … }` plus `type Props = CommonProps & ({ … } | { … })`
 * (LicenceActivation, Popover). Neither primary shape above matches it, so
 * without this fallback Snippet-typed members leak into the prop set as false
 * drift. Returns the concatenated member text, or null when the component
 * declares neither part.
 */
export function unionPropsBody(src: string): string | null {
  const bodies: string[] = [];
  const common = src.search(/interface CommonProps\s*\{/);
  if (common >= 0) bodies.push(...braceGroupBodies(src, common));
  const alias = src.search(/type Props\s*=/);
  if (alias >= 0) bodies.push(...braceGroupBodies(src, alias));
  return bodies.length > 0 ? bodies.join("\n") : null;
}

/** Bodies of every balanced `{ … }` group in the declaration starting at
 *  `from`, stopping at the first top-level `;`. String-aware, so braces inside
 *  string literals are content. */
function braceGroupBodies(src: string, from: number): string[] {
  const bodies: string[] = [];
  let depth = 0;
  let start = -1;
  let quote: string | null = null;
  for (let i = from; i < src.length; i++) {
    const ch = src[i];
    if (quote !== null) {
      if (ch === "\\") i++;
      else if (ch === quote) quote = null;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === "`") {
      quote = ch;
      continue;
    }
    if (ch === ";" && depth === 0) break;
    if (ch === "{") {
      if (depth === 0) start = i + 1;
      depth++;
    } else if (ch === "}") {
      depth--;
      if (depth === 0 && start >= 0) {
        bodies.push(src.slice(start, i));
        start = -1;
      }
    }
  }
  return bodies;
}

export function snippetProps(src: string): Set<string> {
  const out = new Set<string>();
  let body: string | null = null;
  const iface = src.match(/interface Props\s*\{([\s\S]*?)\n\s*\}/);
  if (iface) body = iface[1];
  else {
    const m = src.match(/\}\s*:\s*\{([\s\S]*?)\n\s*\}\s*=\s*\$props\(\)/);
    if (m) body = m[1];
  }
  if (!body) body = unionPropsBody(src);
  if (!body) return out;
  let depth = 0;
  let cur = "";
  let prev = "";
  for (const ch of body) {
    // The `>` of an arrow function is not a closing bracket (the same rule the
    // destructure parser below applies): without it, a `(v) => void` prop type
    // drives depth negative and every later `;` reads as nested, silently
    // dropping the snippet entries after it.
    const isArrow = ch === ">" && prev === "=";
    if ("{([<".includes(ch)) depth++;
    else if (!isArrow && "})]>".includes(ch)) depth--;
    prev = ch;
    if (ch === ";" && depth === 0) {
      // Doc comments precede many entries; the name regex must see the
      // declaration, not the `/** … */` block above it.
      const decl = cur.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/.*$/g, "");
      const pm = decl.match(/^\s*([a-zA-Z_$][\w$]*)\s*\??\s*:\s*/);
      if (pm && isFrameworkType(decl.slice(pm[0].length))) out.add(pm[1]);
      cur = "";
      continue;
    }
    cur += ch;
  }
  return out;
}

export interface DriftFinding {
  slug: string;
  contractPath?: string;
  contractOnly: string[];
  svelteOnly: string[];
}

/** One component's two-direction drift, or null when clean. Exported so the
 * reverse-direction rule (an implemented prop the contract does not document)
 * is unit-testable without a real component + contract pair. */
export function componentDrift(
  slug: string,
  cProps: Set<string>,
  allSProps: Set<string>,
  snippets: Set<string>,
  targetSpecific: Set<string>,
  allow: { contractOnly?: string[]; svelteOnly?: string[] },
): DriftFinding | null {
  // Snippet-typed props are implementations too (TextInput documents
  // `leading`/`trailing`), so they satisfy the contractOnly direction, but
  // they are slot plumbing, not props — they never count as undocumented on
  // the svelteOnly side.
  const sProps = new Set([...allSProps].filter((p) => !snippets.has(p)));
  const contractOnly = [...cProps]
    .filter((p) => !allSProps.has(p) && !(allow.contractOnly ?? []).includes(p))
    .sort();
  const svelteOnly = [...sProps]
    .filter(
      (p) =>
        !cProps.has(p) &&
        !targetSpecific.has(p) &&
        !(allow.svelteOnly ?? []).includes(p),
    )
    .sort();
  if (contractOnly.length || svelteOnly.length) return { slug, contractOnly, svelteOnly };
  return null;
}

export interface ContractPropDriftResult {
  checked: number;
  checkedSurfaces: number;
  skipped: Array<{ path: string; reason: string }>;
  total: number;
  errors: string[];
  findings: DriftFinding[];
}

export function contractPropDrift(): ContractPropDriftResult {
  const contractFiles = collectContractFiles(contractsDir);
  const contractSlugs = new Set(contractFiles.map((file) => path.basename(file, ".md")));
  const findings: DriftFinding[] = [];
  const errors: string[] = [];
  const skipped: Array<{ path: string; reason: string }> = [];
  let checked = 0;
  let checkedSurfaces = 0;

  for (const [slug, reason] of Object.entries(CONTRACT_SKIP_REASONS)) {
    if (!contractSlugs.has(slug)) {
      errors.push(`contract prop drift: explicit skip has no contract ${path.relative(repoRoot, path.join(contractsDir, `${slug}.md`))}`);
    }
  }

  for (const contractPath of contractFiles) {
    const contractSlug = path.basename(contractPath, ".md");
    const contractRelativePath = path.relative(repoRoot, contractPath);
    const skipReason = CONTRACT_SKIP_REASONS[contractSlug];
    if (skipReason) {
      skipped.push({ path: contractRelativePath, reason: skipReason });
      continue;
    }

    const entries = allComponents.filter(
      (entry) => (CONTRACT_SURFACES[entry.slug]?.contractSlug ?? entry.slug) === contractSlug,
    );
    if (entries.length === 0) {
      errors.push(`contract prop drift: no Svelte component registered for ${contractRelativePath}`);
      continue;
    }
    const md = readFileSync(contractPath, "utf8");
    let contractChecked = true;
    for (const entry of entries) {
      const surface = CONTRACT_SURFACES[entry.slug] ?? { contractSlug: entry.slug };
      const sveltePath = path.join(svelteDir, `${entry.displayName}.svelte`);
      if (!existsSync(sveltePath)) {
        errors.push(
          `contract prop drift: missing Svelte component ${path.relative(repoRoot, sveltePath)} for ${entry.slug}`,
        );
        contractChecked = false;
        continue;
      }
      const { props: cProps, targetSpecific, found } = contractProps(md, surface.section);
      if (!found) {
        errors.push(`contract prop drift: no props table found in ${contractRelativePath} for ${entry.slug}`);
        contractChecked = false;
        continue;
      }
      checkedSurfaces++;
      const src = readFileSync(sveltePath, "utf8");
      const finding = componentDrift(
        entry.slug,
        cProps,
        svelteProps(src),
        snippetProps(src),
        targetSpecific,
        BASELINE[entry.slug] ?? {},
      );
      if (finding) findings.push({ ...finding, contractPath: contractRelativePath });
    }
    if (contractChecked) checked++;
  }

  for (const entry of allComponents) {
    const contractSlug = CONTRACT_SURFACES[entry.slug]?.contractSlug ?? entry.slug;
    if (!contractSlugs.has(contractSlug) && !CONTRACT_SKIP_REASONS[contractSlug]) {
      const missingPath = path.relative(repoRoot, path.join(contractsDir, `${contractSlug}.md`));
      errors.push(`contract prop drift: missing contract ${missingPath} for ${entry.slug}`);
    }
  }
  return { checked, checkedSurfaces, skipped, total: contractFiles.length, errors, findings };
}

// Gate errors: drift in either direction — a Public Prop the contract documents
// but the authoritative Svelte component does not implement, or a prop the
// Svelte component implements that the contract does not document.
export function contractDriftErrors(result: ContractPropDriftResult = contractPropDrift()): string[] {
  return [
    ...result.errors,
    ...result.findings.flatMap((f) => {
      const errors: string[] = [];
      const contractPath = f.contractPath ?? `${f.slug}.md`;
      if (f.contractOnly.length > 0) {
        errors.push(
          `contract prop drift: ${contractPath} documents prop(s) not implemented in ${f.slug} Svelte component: ${f.contractOnly.join(", ")}`,
        );
      }
      if (f.svelteOnly.length > 0) {
        errors.push(
          `contract prop drift: ${f.slug} Svelte component implements prop(s) not documented in ${contractPath}: ${f.svelteOnly.join(", ")}`,
        );
      }
      return errors;
    }),
  ];
}

// Standalone report / gate: `bun scripts/contract-prop-drift.ts` (add DRIFT_REPORT=1
// to list the drift without exiting non-zero).
if (import.meta.main) {
  const result = contractPropDrift();
  const errors = contractDriftErrors(result);
  console.log(
    `contract-prop-drift: checked ${result.checked} of ${result.total} contracts (${result.checkedSurfaces} Svelte surfaces); skipped ${result.skipped.length} explicit contracts\n`,
  );
  for (const skipped of result.skipped) console.log(`  skip ${skipped.path}: ${skipped.reason}`);
  if (result.skipped.length > 0) console.log("");
  if (errors.length > 0) {
    console.log(`FAIL — ${errors.length} contract prop-drift error(s):`);
    for (const error of errors) console.log(`  ${error}`);
    console.log("");
  } else {
    console.log("OK — every documented public prop is implemented in Svelte, and every implemented prop is documented.");
  }
  if (errors.length > 0 && process.env.DRIFT_REPORT !== "1") process.exit(1);
}
