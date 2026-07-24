// Contract <-> Svelte prop-surface drift check.
//
// CLAUDE.md mandates that each component's contract (docs/contracts/components/
// <slug>.md) and its implementation stay in sync, but nothing enforced it. This
// compares the contract's "### Public Props" table against the authoritative
// Svelte component's `interface Props`, failing on any drift not recorded in the
// baseline below.
//
// Excluded from the Svelte side (framework idiom, not public props):
//   - Snippet-typed props (slots/children — documented separately in contracts)
//   - `on*` event callbacks (contracts document these in an Events section)
//   - the `[key: string]` index signature and `...restProps` passthrough

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

import { allComponents } from "../src/component-registry.ts";

const repoRoot = path.resolve(import.meta.dir, "../../../..");
const contractsDir = path.join(repoRoot, "docs/contracts/components");
const svelteDir = path.join(repoRoot, "packages/svelte/components/src");

// Known, accepted drift: slug -> { contractOnly?: string[]; svelteOnly?: string[] }.
// Closing a drift means deleting its entry.
const BASELINE: Record<string, { contractOnly?: string[]; svelteOnly?: string[] }> = {};

function contractProps(md: string): Set<string> {
  const props = new Set<string>();
  const start = md.indexOf("### Public Props");
  if (start < 0) return props;
  const rest = md.slice(start + "### Public Props".length);
  const end = rest.search(/\n#{2,4} /);
  const table = end < 0 ? rest : rest.slice(0, end);
  for (const line of table.split("\n")) {
    const m = line.match(/^\|\s*`([a-zA-Z_$][\w$]*)`\s*\|/);
    if (m && !/^on[A-Z]/.test(m[1])) props.add(m[1]);
  }
  return props;
}

// Extract the top-level prop names from the component's `let { ... } = $props()`
// destructure — uniform across Svelte 5 components (unlike the type declaration,
// which may be an interface, a type alias, or inline). Commas/colons/equals
// inside default values, generics, and object literals are skipped via depth.
function svelteProps(src: string): Set<string> {
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
  for (let i = open; i < src.length; i++) {
    if (src[i] === "{") depth++;
    else if (src[i] === "}") {
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
  for (const ch of body) {
    if ("{([<".includes(ch)) d++;
    else if ("})]>".includes(ch)) d--;
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

export interface DriftFinding {
  slug: string;
  contractOnly: string[];
  svelteOnly: string[];
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
    const cProps = contractProps(readFileSync(contractPath, "utf8"));
    const sProps = svelteProps(readFileSync(sveltePath, "utf8"));
    if (cProps.size === 0) {
      skipped++;
      continue;
    }
    checked++;
    const allow = BASELINE[entry.slug] ?? {};
    const contractOnly = [...cProps]
      .filter((p) => !sProps.has(p) && !(allow.contractOnly ?? []).includes(p))
      .sort();
    const svelteOnly = [...sProps]
      .filter((p) => !cProps.has(p) && !(allow.svelteOnly ?? []).includes(p))
      .sort();
    if (contractOnly.length || svelteOnly.length) findings.push({ slug: entry.slug, contractOnly, svelteOnly });
  }
  return { checked, skipped, findings };
}

// Gate errors: contract-only drift only — a Public Prop the contract documents
// but the authoritative Svelte component does not implement. svelte-only props
// (slots, internal, framework-idiom names) are noisier and stay report-only.
export function contractDriftErrors(): string[] {
  return contractPropDrift()
    .findings.filter((f) => f.contractOnly.length > 0)
    .map((f) => `contract prop drift: ${f.slug}.md documents prop(s) not implemented in ${f.slug} Svelte component: ${f.contractOnly.join(", ")}`);
}

// Standalone report / gate: `bun scripts/contract-prop-drift.ts` (add DRIFT_REPORT=1
// to also list informational svelte-only props and never exit non-zero).
if (import.meta.main) {
  const { checked, skipped, findings } = contractPropDrift();
  const gated = findings.filter((f) => f.contractOnly.length > 0);
  const svelteOnlyOnly = findings.filter((f) => f.contractOnly.length === 0 && f.svelteOnly.length > 0);
  console.log(`contract-prop-drift: checked ${checked}, skipped ${skipped} (no contract/svelte/props)\n`);
  if (gated.length > 0) {
    const n = gated.reduce((a, f) => a + f.contractOnly.length, 0);
    console.log(`FAIL — ${n} documented prop(s) missing from Svelte across ${gated.length} component(s):`);
    for (const f of gated) console.log(`  [${f.slug}] contract-only: ${f.contractOnly.join(", ")}`);
    console.log("");
  }
  if (process.env.DRIFT_REPORT === "1" && svelteOnlyOnly.length > 0) {
    console.log(`(info) undocumented svelte-only props across ${svelteOnlyOnly.length} component(s):`);
    for (const f of svelteOnlyOnly) console.log(`  [${f.slug}] svelte-only: ${f.svelteOnly.join(", ")}`);
    console.log("");
  }
  if (gated.length === 0) console.log("OK — every documented public prop is implemented in Svelte.");
  if (gated.length > 0 && process.env.DRIFT_REPORT !== "1") process.exit(1);
}
