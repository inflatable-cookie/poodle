// Self-referential container queries.
//
// An element cannot be matched by its own container query. A rule inside
// `@container` resolves against the nearest *ancestor* container, so a
// stylesheet that puts `container-type` on `.poodle-x` and then writes
// `@container … { .poodle-x { … } }` is not asking what it looks like it is
// asking. It either fires against the wrong box (when an ancestor container
// exists) or never fires at all (when none does).
//
// Measured instance that motivated this gate (g13-043): `detail-item.css` made
// `.poodle-detail-item` a container and targeted `.poodle-detail-item` inside
// its own `@container (max-width: 26rem)`. Those rules resolved against a
// page-wide `.poodle-detail-section`, so at an item width of 240px the query
// never fired, the label column took its 11.25rem max, and the value column
// resolved to 20px — at which `word-break` split values character by
// character.
//
// Targeting a DESCENDANT from inside the element's own `@container` is correct
// and common (`form-actions` styles `__danger`, `form-layout` styles
// `__grid`). The check must not flag those, which is the trap: a class-name
// regex of `[a-z0-9-]+` stops at the underscore and reads
// `.poodle-form-actions__danger` as `.poodle-form-actions`. The ad-hoc scan
// that found this bug had exactly that defect and over-reported two of five.

import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../..");
const stylesDir = path.join(repoRoot, "packages/core/src/styles");

/** Self-queries that are deliberate. Key: `<file>: .<class>`; value: reason.
 *  Empty by intent — a self-query is a defect, not a style choice. An entry
 *  here means the ancestor happens to be the box you wanted, in which case say
 *  so and note that the element's own `container-type` is then redundant. */
const BASELINE: Record<string, string> = {
  "detail-item.css: .poodle-detail-item":
    "g13-043 R2 — the surviving self-target is `[data-span] { grid-column: 1 / -1 }`, which is about the item's place in the SECTION's grid and is deliberately section-keyed. The item-scale rules moved to `__grid`. Resolving against the section is what this rule wants.",
  "detail-section.css: .poodle-detail-section":
    "UNVERIFIED — `[data-separated=\"true\"]::before`, a 0.125rem separator inset. A section's nearest ancestor container is its enclosing DetailSectionGroup, so this follows the group's width rather than its own. Plausibly the box you want for a shared separator inset, but it was not measured; see PAPERCUTS 2026-08-13. Delete this entry once someone checks it.",
};

/** Class tokens in a selector, underscores included. `[a-z0-9-]+` would stop
 *  at the `__` and turn a descendant into a false self-match. */
export function classesIn(selector: string): string[] {
  return [...selector.matchAll(/\.([a-zA-Z0-9_-]+)/g)].map((m) => m[1]);
}

/** Comments are not CSS. Parsing them yields phantom classes from prose — an
 *  earlier draft of this gate read "g13.043" as a class named `043`. */
export function stripComments(css: string): string {
  return css.replace(/\/\*[\s\S]*?\*\//g, "");
}

/** Classes of a selector's SUBJECT — the rightmost compound of each
 *  comma-separated part. `.a .b__c` styles `b__c`; `.a` is only an ancestor
 *  qualifier and the rule is not a self-query. Getting this wrong is the
 *  difference between five findings and one. */
export function subjectClasses(selector: string): string[] {
  const out: string[] = [];
  for (const part of selector.split(",")) {
    const compounds = part.trim().split(/\s*[>+~]\s*|\s+/).filter(Boolean);
    const subject = compounds.at(-1);
    if (subject) out.push(...classesIn(subject));
  }
  return out;
}

/** Classes a stylesheet declares `container-type` on. */
export function containerClasses(css: string): Set<string> {
  const out = new Set<string>();
  for (const m of stripComments(css).matchAll(/([^{}]+)\{([^{}]*)\}/g)) {
    if (!/container-type\s*:/.test(m[2])) continue;
    for (const c of subjectClasses(m[1])) out.add(c);
  }
  return out;
}

/** Classes targeted by rules inside an `@container` block, exactly — a
 *  descendant like `__grid` is a different class and is not a self-query. */
export function containerQueryTargets(cssInput: string): { target: string; query: string }[] {
  const out: { target: string; query: string }[] = [];
  const css = stripComments(cssInput);
  const re = /@container([^{]*)\{/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(css)) !== null) {
    // Walk to the matching close brace so nested rules stay with their block.
    let depth = 1;
    let i = re.lastIndex;
    for (; i < css.length && depth > 0; i++) {
      if (css[i] === "{") depth++;
      else if (css[i] === "}") depth--;
    }
    const body = css.slice(re.lastIndex, i - 1);
    const query = m[1].trim();
    for (const r of body.matchAll(/([^{}]+)\{[^{}]*\}/g)) {
      for (const c of subjectClasses(r[1])) out.push({ target: c, query });
    }
    re.lastIndex = i;
  }
  return out;
}

export interface SelfQueryFinding {
  file: string;
  cls: string;
  query: string;
}

export function containerQueryDrift(): { checked: number; findings: SelfQueryFinding[] } {
  const findings: SelfQueryFinding[] = [];
  let checked = 0;
  for (const f of readdirSync(stylesDir).filter((n) => n.endsWith(".css"))) {
    const css = readFileSync(path.join(stylesDir, f), "utf8");
    if (!css.includes("@container")) continue;
    checked++;
    const owners = containerClasses(css);
    const seen = new Set<string>();
    for (const { target, query } of containerQueryTargets(css)) {
      if (!owners.has(target)) continue;
      const key = `${f}: .${target}`;
      if (seen.has(key) || key in BASELINE) continue;
      seen.add(key);
      findings.push({ file: f, cls: target, query });
    }
  }
  return { checked, findings };
}

export function containerQueryDriftErrors(): string[] {
  return containerQueryDrift().findings.map(
    (f) =>
      `container-query drift: ${f.file} declares container-type on .${f.cls} and targets .${f.cls} inside its own @container (${f.query}) — an element cannot be matched by its own container query, so the rule resolves against the nearest ancestor container`,
  );
}

// Standalone report / gate: `bun scripts/container-query-drift.ts`
// (DRIFT_REPORT=1 lists findings without exiting non-zero).
if (import.meta.main) {
  const { checked, findings } = containerQueryDrift();
  console.log(`container-query-drift: checked ${checked} stylesheet(s) using @container\n`);
  if (findings.length > 0) {
    console.log(`FAIL — ${findings.length} self-referential container quer(y|ies):`);
    for (const f of findings) console.log(`  [${f.file}] .${f.cls} inside @container ${f.query}`);
    console.log("");
  } else {
    console.log("OK — no stylesheet targets a class inside that class's own @container.");
  }
  if (findings.length > 0 && process.env.DRIFT_REPORT !== "1") process.exit(1);
}
