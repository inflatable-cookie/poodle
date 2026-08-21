// Contract <-> Svelte callback-surface drift check.
//
// contract-prop-drift.ts deliberately skips `on*` props, on the stated grounds
// that "contracts document these in an Events section". Nothing enforced that.
// The hole was live: b033 shipped HistoryCenter's `onDeleteContinuation` — a
// destructive host operation — with no prop row, no effect row and no callback
// row anywhere in its contract, and every gate stayed green. This closes it by
// comparing each component's function-typed `on*` props against the names its
// contract's Callbacks/Events section actually mentions.
//
// A callback is decided by TYPE, not by name. `on[A-Z]` alone is wrong in both
// directions: Switch's `onColor` is a colour string, not a handler, and it must
// not be demanded here — while it is also skipped by contract-prop-drift for
// looking like a callback, so before this file it was checked by nothing.

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { allComponents } from "../src/component-registry.ts";
import { unionPropsBody } from "./contract-prop-drift.ts";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../..");
const contractsDir = path.join(repoRoot, "docs/contracts/components");
const svelteDir = path.join(repoRoot, "packages/svelte/components/src");

// Known, accepted drift: slug -> callback names the contract need not mention.
// Closing a drift means deleting its entry. Empty by intent — the 41 gaps this
// check first found were documented rather than baselined, so an entry here
// should be a genuine exception with a reason, not a backlog parking space.
const BASELINE: Record<string, string[]> = {};

/** The body of `interface Props { … }`, or of the inline `let { … }: { … }`
 *  annotation. Brace-depth counted, NOT a non-greedy regex: a lazy match ends
 *  at the first `\n  }`, which is the close of a nested object or function
 *  type, not of the interface. That silently truncated HistoryCenter at 25 of
 *  its props and hid the very callback this file exists to catch. */
export function propsBody(src: string): string | null {
  let open = -1;
  const iface = src.search(/interface Props\s*\{/);
  if (iface >= 0) {
    open = src.indexOf("{", iface);
  } else {
    // `let { … }: { … } = $props()` — the annotation brace, not the destructure.
    const anchor = src.indexOf("= $props()");
    if (anchor < 0) return unionPropsBody(src);
    const colon = src.lastIndexOf("}:", anchor);
    if (colon < 0) return unionPropsBody(src);
    // `}: Props = $props()` (a named Props type — including the
    // `type Props = CommonProps & ({ … } | { … })` discriminated union) is not
    // the inline shape; only a `{` right after `}:` is.
    const annotation = /^\s*\{/.exec(src.slice(colon + 2));
    if (!annotation) return unionPropsBody(src);
    open = colon + 2 + annotation[0].length - 1;
  }
  if (open < 0) return null;
  let depth = 0;
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
      if (depth === 0) return src.slice(open + 1, i);
    }
  }
  return null;
}

/** Prop name -> declared type text, from `interface Props` or the inline
 *  `let { … }: { … } = $props()` annotation. Mirrors the parser in
 *  contract-prop-drift.ts, but keeps the type instead of discarding it. */
export function propTypes(src: string): Map<string, string> {
  const out = new Map<string, string>();
  const raw = propsBody(src);
  if (raw === null) return out;
  // Strip comments BEFORE splitting, not per-chunk. Doc comments contain
  // semicolons — HistoryCenter's `/** Entry activation; always the entry … */`
  // is one — and a depth-0 `;` inside a comment splits mid-comment, leaving the
  // tail glued to the next declaration where the name anchor no longer matches.
  // That silently dropped onNavigateEntry, onCheckoutContinuation and
  // onDeleteContinuation.
  const body = raw.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/[^\n]*/g, "");
  let depth = 0;
  let cur = "";
  let prev = "";
  for (const ch of body) {
    // The `>` of an arrow function is not a closing bracket — the same rule
    // contract-prop-drift.ts applies. Without it a `(v) => void` prop type
    // drives depth negative and every later `;` reads as nested.
    const isArrow = ch === ">" && prev === "=";
    if ("{([<".includes(ch)) depth++;
    else if (!isArrow && "})]>".includes(ch)) depth--;
    prev = ch;
    if (ch === ";" && depth === 0) {
      const pm = cur.match(/^\s*([a-zA-Z_$][\w$]*)\s*\??\s*:\s*/);
      if (pm) out.set(pm[1], cur.slice(pm[0].length).trim());
      cur = "";
      continue;
    }
    cur += ch;
  }
  return out;
}

/** A callback is a function-typed prop: an arrow in the type, or a named
 *  handler alias (`OverlaySurfaceGeometryChangeHandler`). Backticked text is
 *  stripped first so a doc comment cannot fake either shape. */
export function isCallbackType(type: string): boolean {
  const bare = type.replace(/`[^`]*`/g, "");
  return /=>/.test(bare) || /\bHandler\b/.test(bare);
}

/** The component's callback props: `on*` AND function-typed. */
export function svelteCallbacks(src: string): Set<string> {
  const out = new Set<string>();
  for (const [name, type] of propTypes(src)) {
    if (/^on[A-Z]/.test(name) && isCallbackType(type)) out.add(name);
  }
  return out;
}

/** The body of the contract's Callbacks/Events section, or null when it has
 *  none. Contracts number these inconsistently (`## 5. Callbacks`,
 *  `## 6. Events`, `## Callbacks`), so the heading match is loose. */
export function callbackSection(md: string): string | null {
  const m = md.match(/\n#{2,3} (?:\d+\.\s*)?(?:Callbacks|Events)\b/);
  if (!m) return null;
  const rest = md.slice(md.indexOf(m[0]) + m[0].length);
  const end = rest.search(/\n#{1,2} /);
  return end < 0 ? rest : rest.slice(0, end);
}

/** Callback names the section names, in backticks. */
export function documentedCallbacks(section: string): Set<string> {
  return new Set([...section.matchAll(/`(on[A-Z][\w$]*)`/g)].map((m) => m[1]));
}

export interface CallbackFinding {
  slug: string;
  /** true when the contract has no Callbacks/Events section at all. */
  noSection: boolean;
  undocumented: string[];
}

/** One component's drift, or null when clean. Exported so the rules are
 *  unit-testable without a real component + contract pair. */
export function componentCallbackDrift(
  slug: string,
  callbacks: Set<string>,
  md: string,
  allow: string[],
): CallbackFinding | null {
  if (callbacks.size === 0) return null;
  const section = callbackSection(md);
  const remaining = [...callbacks].filter((c) => !allow.includes(c)).sort();
  if (remaining.length === 0) return null;
  if (section === null) return { slug, noSection: true, undocumented: remaining };
  const documented = documentedCallbacks(section);
  const undocumented = remaining.filter((c) => !documented.has(c));
  if (undocumented.length === 0) return null;
  return { slug, noSection: false, undocumented };
}

export function contractCallbackDrift(): {
  checked: number;
  skipped: number;
  findings: CallbackFinding[];
} {
  const findings: CallbackFinding[] = [];
  let checked = 0;
  let skipped = 0;
  for (const entry of allComponents) {
    const contractPath = path.join(contractsDir, `${entry.slug}.md`);
    const sveltePath = path.join(svelteDir, `${entry.displayName}.svelte`);
    if (!existsSync(contractPath) || !existsSync(sveltePath)) {
      skipped++;
      continue;
    }
    const callbacks = svelteCallbacks(readFileSync(sveltePath, "utf8"));
    if (callbacks.size === 0) {
      skipped++;
      continue;
    }
    checked++;
    const finding = componentCallbackDrift(
      entry.slug,
      callbacks,
      readFileSync(contractPath, "utf8"),
      BASELINE[entry.slug] ?? [],
    );
    if (finding) findings.push(finding);
  }
  return { checked, skipped, findings };
}

/** Gate errors, phrased for lint-docs' error list. */
export function contractCallbackDriftErrors(): string[] {
  return contractCallbackDrift().findings.map((f) =>
    f.noSection
      ? `contract callback drift: ${f.slug}.md has no Callbacks/Events section, but the ${f.slug} Svelte component emits: ${f.undocumented.join(", ")}`
      : `contract callback drift: ${f.slug} Svelte component emits callback(s) its contract does not document: ${f.undocumented.join(", ")}`,
  );
}

// Standalone report / gate: `bun scripts/contract-callback-drift.ts`
// (DRIFT_REPORT=1 lists the drift without exiting non-zero).
if (import.meta.main) {
  const { checked, skipped, findings } = contractCallbackDrift();
  console.log(
    `contract-callback-drift: checked ${checked}, skipped ${skipped} (no contract/svelte/callbacks)\n`,
  );
  if (findings.length > 0) {
    const n = findings.reduce((a, f) => a + f.undocumented.length, 0);
    console.log(`FAIL — ${n} undocumented callback(s) across ${findings.length} component(s):`);
    for (const f of findings) {
      const why = f.noSection ? "no Callbacks/Events section" : "undocumented";
      console.log(`  [${f.slug}] ${why}: ${f.undocumented.join(", ")}`);
    }
    console.log("");
  } else {
    console.log("OK — every callback a Svelte component emits is named in its contract.");
  }
  if (findings.length > 0 && process.env.DRIFT_REPORT !== "1") process.exit(1);
}
