// Focus-ring drift check (g13.037 R4, g13.038 R4).
//
// 037 gated ring radius: an outline follows its element's own border-radius.
// An element that draws a focus outline and declares no radius renders a hard
// square ring, even when everything around it is rounded — HistoryCentre's
// three full-width rows were fixed in 099a265, and card 037 browser-verified
// the corpus-wide scan of 2026-08-12 before gating it.
//
// 038 adds two coverage checks, both browser-verified in the card's log:
//
//  1. Absent treatment: a component that owns a focusable element and declares
//     no focus treatment at all. "Owns" is the R1 rule: a focusable rendered
//     by a NESTED Poodle component draws its own ring (dialog's close
//     IconButton, action-discovery-panel's ListCard), so it is not a gap here.
//     Only elements styled by the component's own stylesheet count. Keyboard-
//     unreachable backdrops (drawer/dialog) are baselined.
//
//  2. Stacked UA outline: a component whose focus treatment is not an outline
//     on the focused element (a box-shadow, an inset ring, or a ring drawn on
//     a sibling/descendant) must set outline: none on the focused element,
//     or Chrome's :focus-visible `outline: auto` draws a second ring on top.
//     Checkbox/radio/switch/tri-state/segmented-control hide a native input
//     (ring on the sibling indicator), and the machine-driven controls draw
//     their ring on a visual child — all measured in the 038 log.
//
// The baseline holds elements that are exempt by intent, each with a reason.
// Closing a drift means deleting its entry.

import { readdirSync, readFileSync, existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../..");
const stylesDir = path.join(repoRoot, "packages/core/src/styles");
const componentsDir = path.join(repoRoot, "packages/svelte/components/src");

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

/**
 * Components that own a focusable element yet declare no focus treatment.
 * Key: `<file>`; value: one-line reason. Baselinable only when the focusable
 * is genuinely unreachable by keyboard or already covered by a nested
 * component's ring (R1), not because it has not been looked at yet.
 */
const ABSENT_BASELINE: Record<string, string> = {
  "dialog.css":
    "full-viewport backdrop button is keyboard-unreachable (focus trap wraps inside the surface); the close IconButton is a nested component with its own ring.",
  "drawer.css":
    "full-viewport backdrop button is keyboard-unreachable (focus trap wraps inside the surface); surface content is nested components with their own rings.",
};

/**
 * Non-outline focus treatments whose focused element must set outline: none
 * (otherwise Chrome stacks its :focus-visible auto outline on top).
 * Key: `<file>: .<class>`; value: one-line reason.
 */
const STACK_BASELINE: Record<string, string> = {};

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

// ────────────────────────────────────────────────────────────────────────────
// g13.038 checks — absent treatment (R1) and stacked UA outline (R2).
// Both are component-level: they walk each stylesheet against its Svelte
// component's markup, so "owns a focusable" means the focusable element's
// class is styled by THIS sheet — a focusable inside a nested Poodle
// component (IconButton, ListCard, …) draws its own ring and is not a gap.
// ────────────────────────────────────────────────────────────────────────────

/** PascalCase the stylesheet stem, like the card's scan. */
function componentName(file: string): string {
  return file
    .slice(0, -4)
    .split("-")
    .map((w) => w[0].toUpperCase() + w.slice(1))
    .join("");
}

/** Escape a class name for use in a RegExp. */
function esc(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/**
 * The focusable elements the component's OWN stylesheet styles.
 * A focusable element is "owned" when its poodle-* class appears in this
 * sheet's selectors; a classless focusable (bare `<button>`, `<input>`) is
 * owned when the sheet styles that tag under a poodle-* scope. Nested Svelte
 * components (capitalized tags) are skipped.
 */
function ownedFocusables(file: string): { cls: string; tag: string }[] {
  const comp = `${componentsDir}/${componentName(file)}.svelte`;
  if (!existsSync(comp)) return [];
  const markup = readFileSync(comp, "utf8");
  const sheetSelectors = rules().filter((r) => r.file === file).map((r) => r.selector);
  const owned: { cls: string; tag: string }[] = [];
  const seen = new Set<string>();
  const tagRe = /<(\w+)\b([^>]*)>/g;
  for (const m of markup.matchAll(tagRe)) {
    const tag = m[1];
    if (tag[0] === tag[0].toUpperCase()) continue; // nested Svelte component
    const attrs = m[2];
    // tabindex="-1" elements are not Tab-reachable and never draw the UA
    // :focus-visible outline; type="hidden" inputs are never rendered.
    const hidden = tag === "input" && /\btype=["']hidden["']/.test(attrs);
    const tabIndexNegative = /\btabindex=\s*(?:["']?-1["']?|\{-1\})/.test(attrs);
    const focusable =
      tag === "button" ||
      tag === "input" ||
      tag === "select" ||
      tag === "textarea" ||
      (tag === "a" && /\bhref=/.test(attrs)) ||
      (/\btabindex=/.test(attrs) && !tabIndexNegative);
    if (!focusable || hidden) continue;
    const clsMatch = /\bclass=["']([^"']*)["']|\bclass=\{`([^`]*)`\}|\bclass=\{([^}]*)\}/.exec(attrs);
    const cls = clsMatch ? (clsMatch[1] ?? clsMatch[2] ?? clsMatch[3]).split(/\s+/).find((c) => c.startsWith("poodle-")) : null;
    if (cls) {
      const key = cls;
      if (!seen.has(key) && sheetSelectors.some((s) => new RegExp(`\\.${esc(cls)}\\b`).test(s))) {
        seen.add(key);
        owned.push({ cls, tag });
      }
    } else {
      const key = `__${tag}`;
      const ownedByTag = sheetSelectors.some((s) => {
        const stripped = stripFunctionalAndAttrs(s);
        return /\.poodle-[\w-]+/.test(stripped) && new RegExp(`\\b${tag}\\b`).test(stripped);
      });
      if (!seen.has(key) && ownedByTag) {
        seen.add(key);
        owned.push({ cls: key, tag });
      }
    }
  }
  return owned;
}

/** Does the stylesheet declare any focus treatment at all? */
function declaresFocusTreatment(selector: string, body: string): boolean {
  const focusish = /:focus-visible|:focus-within|:focus\b|\[data-focus|\[data-focused|--active|--focused/.test(selector);
  if (!focusish) return false;
  return /outline|box-shadow|border-color|border\b|filter/.test(body);
}

/** The bare tag in a selector's last compound, if any (e.g. "button" in
 *  `.poodle-number-input__steppers button:focus-visible`). */
function lastCompoundTag(sel: string): string | null {
  const stripped = stripFunctionalAndAttrs(sel);
  const compounds = stripped.split(/\s*[>+~]\s*|\s+/).filter(Boolean);
  const last = compounds[compounds.length - 1] ?? "";
  const m = /^([a-z][\w-]*)/.exec(last);
  return m ? m[1] : null;
}

/**
 * The focused element is protected from the UA's :focus-visible auto outline
 * when any rule lands an outline declaration on it: `outline: none|0`
 * suppresses it, and any other author outline replaces it (author origin
 * beats the UA sheet, so exactly one outline shows either way).
 * Class-based coverage is corpus-wide (shared comma lists, rings living in
 * another sheet such as selection-summary's chip-activate); bare-tag coverage
 * counts only within the component's own sheet — `.poodle-breadcrumbs
 * button:focus-visible` must not excuse number-input's classless steppers.
 */
function outlineCovered(cls: string, file: string, all: Rule[]): boolean {
  return all.some((c) => {
    if (!/\boutline\s*:/.test(c.body)) return false;
    return c.selector.split(",").some((p) => {
      if (lastCompoundClasses(p).includes(cls)) return true;
      return c.file === file && lastCompoundTag(p) === cls;
    });
  });
}

/**
 * Per-sheet focus coverage. `absent` = owns a focusable + declares no focus
 * treatment. `stacked` = draws a focus treatment that is not an outline on
 * the focused element itself, while an owned focusable lacks outline:none.
 */
export function focusCoverage(): {
  absent: { file: string; selector: string }[];
  stacked: { file: string; cls: string; selector: string }[];
} {
  const all = rules();
  const absent: { file: string; selector: string }[] = [];
  const stacked: { file: string; cls: string; selector: string }[] = [];

  for (const f of readdirSync(stylesDir).filter((n) => n.endsWith(".css"))) {
    const sheetRules = all.filter((r) => r.file === f);
    const owned = ownedFocusables(f);
    if (owned.length === 0) continue; // nothing this component styles is focusable

    // ── Check 1: no focus treatment + owns a focusable. ────────────────────
    const treated = sheetRules.some((r) => declaresFocusTreatment(r.selector, r.body));
    if (!treated) absent.push({ file: f, selector: "owns a focusable element and declares no focus treatment" });

    // ── Check 2: stacked UA outline. The UA draws `outline: auto` on the
    //    :focus-visible element unless the component sets outline: none.
    //    Replaced, not stacked, only when the component's own indicator IS an
    //    outline landing on the focused element itself. Every other shape —
    //    box-shadow indicator, ring on a sibling/descendant (checkbox
    //    indicator, tree row, card option, info icon), :focus-within wrapper
    //    ring, machine-driven [data-focus] visual ring — needs outline:none
    //    on the focused element, which is one of this sheet's owned focusables.
    const focusRules = sheetRules.filter((r) =>
      /:focus-visible|:focus\b|:focus-within|\[data-focus|\[data-focused/.test(r.selector),
    );
    const stackedTreatment = focusRules.some((r) => {
      if (!/outline|box-shadow|border-color|border\b/.test(r.body)) return false;
      // machine ring on a visual child: ring is not on the focused element
      if (/\[data-focus|\[data-focused/.test(r.selector)) return true;
      // :focus-within draws on the wrapper; the focused element is a descendant
      if (r.selector.includes(":focus-within")) return true;
      // :has(:focus-visible) rings the wrapper; the focused element is the
      // :has argument (video-player's progress bar), which must be
      // outline-suppressed instead.
      if (/:has\([^)]*:focus(?:-visible)?/.test(r.selector)) return true;
      // box-shadow indicators always stack with the UA outline
      if (/box-shadow\s*:/.test(r.body)) return true;
      // An outline ring replaces the UA outline only when it lands on the
      // compound it focuses. Comma lists are evaluated per part, so a rule
      // mixing both shapes counts as stacked; a descendant combinator
      // (.foo:focus-visible .bar) draws the ring on a non-focused element.
      return r.selector.split(",").some((p) => {
        if (!/:focus-visible|:focus\b/.test(p)) return false;
        const stripped = stripFunctionalAndAttrs(p);
        const focused = /([.#]?[\w-]+):focus(?:-visible)?/.exec(stripped);
        if (!focused) return false;
        const last = lastCompoundClasses(p).at(-1);
        return last !== null && last !== focused[1].replace(/^[.#]/, "");
      });
    });
    if (!stackedTreatment) continue;
    for (const o of owned) {
      const cls = o.cls.startsWith("__") ? o.tag : o.cls;
      if (!outlineCovered(cls, f, all)) {
        stacked.push({ file: f, cls: o.cls.startsWith("__") ? `<${o.tag}>` : `.${o.cls}`, selector: "focused element lacks outline: none" });
      }
    }
  }
  return { absent, stacked };
}

/** Gate errors, phrased for lint-docs' error list. */
export function focusRingDriftErrors(): string[] {
  const { findings } = focusRingDrift();
  const errs = findings.map((f) =>
    f.stale
      ? `focus-ring drift: baseline entry ${f.file}: ${f.cls} is no longer flagged — delete the entry (the ring now has a radius, or the class changed)`
      : `focus-ring drift: ${f.file} draws a focus outline on ${f.cls} with no border-radius anywhere (${f.selector})`,
  );

  const { absent, stacked } = focusCoverage();
  const freshAbsent = absent.filter((a) => !(a.file in ABSENT_BASELINE));
  const staleAbsent = Object.keys(ABSENT_BASELINE).filter((k) => !absent.some((a) => a.file === k));
  for (const a of freshAbsent) {
    errs.push(`focus coverage: ${a.file} owns a focusable element and declares no focus treatment (${a.selector})`);
  }
  for (const k of staleAbsent) {
    errs.push(`focus coverage: baseline entry ${k} is no longer flagged — delete the entry (the component now declares a focus treatment)`);
  }

  const freshStack = stacked.filter((s) => !(`${s.file}: ${s.cls}` in STACK_BASELINE));
  const staleStack = Object.keys(STACK_BASELINE).filter((k) => !stacked.some((s) => `${s.file}: ${s.cls}` === k));
  for (const s of freshStack) {
    errs.push(`focus coverage: ${s.file} draws a non-outline focus indicator on ${s.cls} without outline: none on the focused element (${s.selector})`);
  }
  for (const k of staleStack) {
    errs.push(`focus coverage: baseline entry ${k} is no longer flagged — delete the entry (the focused element now sets outline: none)`);
  }
  return errs;
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

  const { absent, stacked } = focusCoverage();
  const freshAbsent = absent.filter((a) => !(a.file in ABSENT_BASELINE));
  const staleAbsent = Object.keys(ABSENT_BASELINE).filter((k) => !absent.some((a) => a.file === k));
  if (freshAbsent.length > 0) {
    console.log(`\nFAIL — ${freshAbsent.length} component(s) own a focusable with no focus treatment:`);
    for (const a of freshAbsent) console.log(`  [${a.file}] ${a.selector}`);
  }
  if (staleAbsent.length > 0) {
    console.log(`\nFAIL — ${staleAbsent.length} stale absent-treatment baseline entr(y/ies):`);
    for (const k of staleAbsent) console.log(`  [${k}]`);
  }
  if (freshAbsent.length === 0 && staleAbsent.length === 0) {
    console.log("OK — every component that owns a focusable declares a focus treatment (or is baselined).");
  }

  const freshStack = stacked.filter((s) => !(`${s.file}: ${s.cls}` in STACK_BASELINE));
  const staleStack = Object.keys(STACK_BASELINE).filter((k) => !stacked.some((s) => `${s.file}: ${s.cls}` === k));
  if (freshStack.length > 0) {
    console.log(`\nFAIL — ${freshStack.length} non-outline focus indicator(s) without outline: none on the focused element:`);
    for (const s of freshStack) console.log(`  [${s.file}] ${s.cls} (${s.selector})`);
  }
  if (staleStack.length > 0) {
    console.log(`\nFAIL — ${staleStack.length} stale stack baseline entr(y/ies):`);
    for (const k of staleStack) console.log(`  [${k}]`);
  }
  if (freshStack.length === 0 && staleStack.length === 0) {
    console.log("OK — every non-outline focus indicator suppresses the UA outline on the focused element.");
  }
  if ((findings.length > 0 || freshAbsent.length > 0 || staleAbsent.length > 0 || freshStack.length > 0 || staleStack.length > 0) && process.env.DRIFT_REPORT !== "1") process.exit(1);
}
