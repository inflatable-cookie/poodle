// Focus-ring radius drift check (g13.037 R4).
//
// An outline follows its element's own border-radius. An element that draws a
// focus outline and declares no radius renders a hard square ring, even when
// everything around it is rounded — HistoryCentre's three full-width rows were
// fixed in 099a265, and card 037 browser-verified the corpus-wide scan of
// 2026-08-12 before gating it.
//
// The scan's regex ("this file has a :focus-visible outline rule and the base
// selector never declares border-radius") was crude on purpose: a radius can
// arrive from a shared rule, a variant selector, a comma list, or another
// stylesheet, so 21 of its 34 candidates were false positives. This gate flags
// a ring only when NO rule in the whole corpus gives the RING element a radius
// — the ring element, not the scan's naive base: checkbox draws its ring on
// `.poodle-checkbox__indicator`, card-toggle-group on `.poodle-card`.
//
// The baseline holds elements that are square by intent, each with a reason.
// Closing a drift means deleting its entry.

import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../..");
const stylesDir = path.join(repoRoot, "packages/core/src/styles");

/** Square-by-intent rings. Key: `<file>: .<class>`; value: one-line reason. */
const BASELINE: Record<string, string> = {
  "agent-subagent.css: .poodle-agent-subagent__action":
    "transparent text action button; its square ring floats clear of the card's rounded corners (measured 4.6px inside the arc, no stroke crossing).",
  "agent-plan-record.css: .poodle-agent-plan-record__toggle":
    "transparent text toggle; ring clear of any rounded edge (measured 4.6px inside the arc).",
  "agent-question.css: .poodle-agent-question__dismiss":
    "small transparent dismiss button; ring clear of rounded edges.",
  "menu.css: .poodle-menu__trigger":
    "inline text trigger; ring clear of rounded edges.",
  "resize-handle.css: .poodle-resize-handle":
    "square grip hit-target; a square ring is the intended shape.",
};

type Rule = { file: string; selector: string; body: string };

function rules(): Rule[] {
  const out: Rule[] = [];
  for (const f of readdirSync(stylesDir).filter((n) => n.endsWith(".css"))) {
    const t = readFileSync(path.join(stylesDir, f), "utf8");
    for (const m of t.matchAll(/([^{}]+)\{([^{}]*)\}/g)) {
      out.push({ file: f, selector: m[1].trim(), body: m[2] });
    }
  }
  return out;
}

/** Drop `:is(...)`/`:has(...)`/`:not(...)`/`:where(...)` bodies and attribute
 *  blocks so compound splitting sees real combinators only. */
function stripFunctionalAndAttrs(sel: string): string {
  let out = "";
  let depth = 0;
  let inAttr = false;
  let quote: string | null = null;
  for (let i = 0; i < sel.length; i++) {
    const ch = sel[i];
    if (quote !== null) {
      out += ch;
      if (ch === "\\") {
        if (i + 1 < sel.length) out += sel[++i];
        continue;
      }
      if (ch === quote) quote = null;
      continue;
    }
    if (ch === '"' || ch === "'") {
      quote = ch;
      out += ch;
      continue;
    }
    if (ch === "[") inAttr = true;
    if (!inAttr && depth === 0) out += ch;
    if (ch === "(") depth++;
    if (ch === ")") depth = Math.max(0, depth - 1);
    if (ch === "]") inAttr = false;
  }
  return out;
}

/** The class names in a selector's last compound — the element a rule's
 *  declarations actually land on. */
function lastCompoundClasses(sel: string): string[] {
  const stripped = stripFunctionalAndAttrs(sel);
  const compounds = stripped.split(/\s*[>+~]\s*|\s+/).filter(Boolean);
  const last = compounds[compounds.length - 1] ?? "";
  return [...last.matchAll(/\.([\w-]+)/g)].map((m) => m[1]);
}

type Finding = { file: string; cls: string; selector: string; stale?: boolean };

export function focusRingDrift(): {
  checked: number;
  flagged: { file: string; cls: string; selector: string }[];
  findings: Finding[];
} {
  const all = rules();
  const focusRules = all.filter((r) => {
    if (!r.selector.includes(":focus-visible")) return false;
    const outline = /outline\s*:\s*([^;}]+)/.exec(r.body);
    if (!outline) return false;
    const value = outline[1].trim();
    return !(value.startsWith("none") || value.startsWith("0") || value.startsWith("transparent"));
  });

  const flaggedKeys = new Set<string>();
  const flaggedDetails: { file: string; cls: string; selector: string }[] = [];
  for (const r of focusRules) {
    for (const part of r.selector.split(",")) {
      const ringClasses = lastCompoundClasses(part);
      const cls = ringClasses[ringClasses.length - 1];
      if (!cls) continue;
      // A radius anywhere in the corpus whose selector's last compound carries
      // this class means the ring element rounds — shared rules, variants,
      // comma lists, and other stylesheets all count.
      const hasRadius = all.some((c) => {
        if (!c.body.includes("border-radius")) return false;
        return c.selector.split(",").some((p) => lastCompoundClasses(p).includes(cls));
      });
      const key = `${r.file}: .${cls}`;
      if (!hasRadius && !flaggedKeys.has(key)) {
        flaggedKeys.add(key);
        flaggedDetails.push({ file: r.file, cls: `.${cls}`, selector: part });
      }
    }
  }

  const findings: Finding[] = [];
  for (const f of flaggedDetails) {
    const key = `${f.file}: ${f.cls}`;
    if (!(key in BASELINE)) findings.push(f);
  }
  for (const key of Object.keys(BASELINE)) {
    if (!flaggedKeys.has(key)) {
      const [file, cls] = key.split(": ");
      findings.push({ file, cls, stale: true, selector: "" });
    }
  }
  return { checked: focusRules.length, flagged: flaggedDetails, findings };
}

/** Gate errors, phrased for lint-docs' error list. */
export function focusRingDriftErrors(): string[] {
  return focusRingDrift().findings.map((f) =>
    f.stale
      ? `focus-ring drift: baseline entry ${f.file}: ${f.cls} is no longer flagged — delete the entry (the ring now has a radius, or the class changed)`
      : `focus-ring drift: ${f.file} draws a focus outline on ${f.cls} with no border-radius anywhere (${f.selector})`,
  );
}

// Standalone gate: `bun packages/svelte/preview/scripts/focus-ring-drift.ts`
// (DRIFT_REPORT=1 lists the drift without exiting non-zero).
if (import.meta.main) {
  const { checked, flagged, findings } = focusRingDrift();
  console.log(`focus-ring-drift: checked ${checked} focus-outline rules\n`);
  const stale = findings.filter((f) => f.stale);
  const fresh = findings.filter((f) => !f.stale);
  if (fresh.length > 0) {
    console.log(`FAIL — ${fresh.length} focus ring(s) with no radius:`);
    for (const f of fresh) console.log(`  [${f.file}] ${f.cls} (${f.selector})`);
  }
  if (stale.length > 0) {
    console.log(`FAIL — ${stale.length} stale baseline entr(y/ies):`);
    for (const f of stale) console.log(`  [${f.file}] ${f.cls}`);
  }
  if (fresh.length === 0 && stale.length === 0) {
    console.log("OK — every focus outline lands on an element with a radius (or a baselined square).");
  }
  const baselined = flagged.filter((f) => `${f.file}: ${f.cls}` in BASELINE);
  if (baselined.length > 0) {
    console.log(`\nBaselined squares (${baselined.length}):`);
    for (const f of baselined) console.log(`  [${f.file}] ${f.cls} — ${BASELINE[`${f.file}: ${f.cls}`]}`);
  }
  if (findings.length > 0 && process.env.DRIFT_REPORT !== "1") process.exit(1);
}
