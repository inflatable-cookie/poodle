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

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { allComponents } from "../src/component-registry.ts";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../..");
const contractsDir = path.join(repoRoot, "docs/contracts/components");
const svelteDir = path.join(repoRoot, "packages/svelte/components/src");

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
  // dock-region `showTabs` / `tabVariant` — cross-target strip controls;
  //   DockRegionSpec models `tabs_placement` only — spec-surface tranche.
  "dock-region": { svelteOnly: ["showTabs", "tabVariant"] },
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

export function contractProps(md: string): { props: Set<string>; targetSpecific: Set<string> } {
  const props = new Set<string>();
  const targetSpecific = new Set<string>();
  const start = md.indexOf("### Public Props");
  if (start < 0) return { props, targetSpecific };
  const rest = md.slice(start + "### Public Props".length);
  const end = rest.search(/\n#{2,4} /);
  const table = end < 0 ? rest : rest.slice(0, end);
  for (const line of table.split("\n")) {
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
  return { props, targetSpecific };
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
export function snippetProps(src: string): Set<string> {
  const out = new Set<string>();
  let body: string | null = null;
  const iface = src.match(/interface Props\s*\{([\s\S]*?)\n\s*\}/);
  if (iface) body = iface[1];
  else {
    const m = src.match(/\}\s*:\s*\{([\s\S]*?)\n\s*\}\s*=\s*\$props\(\)/);
    if (m) body = m[1];
  }
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

export function contractPropDrift(): { checked: number; skipped: number; findings: DriftFinding[] } {
  const findings: DriftFinding[] = [];
  let checked = 0;
  let skipped = 0;
  for (const entry of allComponents) {
    const contractPath = path.join(contractsDir, `${entry.slug}.md`);
    const sveltePath = path.join(svelteDir, `${entry.displayName}.svelte`);
    if (!existsSync(contractPath) || !existsSync(sveltePath)) {
      skipped++;
      continue;
    }
    const src = readFileSync(sveltePath, "utf8");
    const { props: cProps, targetSpecific } = contractProps(
      readFileSync(contractPath, "utf8"),
    );
    if (cProps.size === 0) {
      skipped++;
      continue;
    }
    checked++;
    const finding = componentDrift(
      entry.slug,
      cProps,
      svelteProps(src),
      snippetProps(src),
      targetSpecific,
      BASELINE[entry.slug] ?? {},
    );
    if (finding) findings.push(finding);
  }
  return { checked, skipped, findings };
}

// Gate errors: drift in either direction — a Public Prop the contract documents
// but the authoritative Svelte component does not implement, or a prop the
// Svelte component implements that the contract does not document.
export function contractDriftErrors(): string[] {
  return contractPropDrift().findings.flatMap((f) => {
    const errors: string[] = [];
    if (f.contractOnly.length > 0) {
      errors.push(
        `contract prop drift: ${f.slug}.md documents prop(s) not implemented in ${f.slug} Svelte component: ${f.contractOnly.join(", ")}`,
      );
    }
    if (f.svelteOnly.length > 0) {
      errors.push(
        `contract prop drift: ${f.slug} Svelte component implements prop(s) not documented in ${f.slug}.md: ${f.svelteOnly.join(", ")}`,
      );
    }
    return errors;
  });
}

// Standalone report / gate: `bun scripts/contract-prop-drift.ts` (add DRIFT_REPORT=1
// to list the drift without exiting non-zero).
if (import.meta.main) {
  const { checked, skipped, findings } = contractPropDrift();
  const gated = findings.filter((f) => f.contractOnly.length > 0 || f.svelteOnly.length > 0);
  console.log(`contract-prop-drift: checked ${checked}, skipped ${skipped} (no contract/svelte/props)\n`);
  if (gated.length > 0) {
    const n = gated.reduce((a, f) => a + f.contractOnly.length + f.svelteOnly.length, 0);
    console.log(`FAIL — ${n} drift prop(s) across ${gated.length} component(s):`);
    for (const f of gated) {
      if (f.contractOnly.length > 0) console.log(`  [${f.slug}] contract-only: ${f.contractOnly.join(", ")}`);
      if (f.svelteOnly.length > 0) console.log(`  [${f.slug}] svelte-only: ${f.svelteOnly.join(", ")}`);
    }
    console.log("");
  }
  if (gated.length === 0) {
    console.log("OK — every documented public prop is implemented in Svelte, and every implemented prop is documented.");
  }
  if (gated.length > 0 && process.env.DRIFT_REPORT !== "1") process.exit(1);
}
