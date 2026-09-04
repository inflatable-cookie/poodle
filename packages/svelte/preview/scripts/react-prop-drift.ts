// Svelte <-> React public prop drift gate.
//
// CLAUDE.md and docs/contracts/001-working-rules.md mandate that Svelte is the
// reference implementation and React must match. While contract-prop-drift.ts
// compares Svelte against contract Public Props tables, this script compares
// React's public prop surface against Svelte's public prop surface.
//
// Normalization rules:
// 1. DOM attribute casing: React camelCase DOM attributes (`autoComplete`,
//    `spellCheck`, `autoCapitalize`, `autoCorrect`, `formAction`, `formNoValidate`,
//    `formTarget`, `tabIndex`, `readOnly`, `colSpan`, `rowSpan`) are the same prop
//    as Svelte lowercase attributes (`autocomplete`, `spellcheck`, `autocapitalize`,
//    `autocorrect`, `formaction`, `formnovalidate`, `formtarget`, `tabindex`,
//    `readonly`, `colspan`, `rowspan`).
// 2. Class attribute: Svelte `class` and React `className` are normalized to `className`.
// 3. For attribute: Svelte `for` and React `htmlFor` are normalized to `htmlFor`.
// 4. Slots and snippets: Svelte `Snippet`-typed props and React `children`/`render`
//    props are slot plumbing, not public props, and are excluded.
// 5. Callback arity/types: `on*` callbacks are compared by prop name only;
//    parameter signatures and callback arities are framework-idiomatic.
// 6. Rest props and index signatures: `...restProps` and `[key: string]` index
//    signatures are excluded.

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { allComponents } from "../src/component-registry.ts";
import {
  contractProps,
  snippetProps,
} from "./contract-prop-drift.ts";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../..");
const contractsDir = path.join(repoRoot, "docs/contracts/components");
const svelteDir = path.join(repoRoot, "packages/svelte/components/src");
const reactDir = path.join(repoRoot, "packages/react/components/src");

export interface BaselineEntry {
  reason: string;
  reactOnly?: string[];
  svelteOnly?: string[];
  defaultDrift?: string[];
}

// Known, accepted drift with reasoned justifications.
// Every entry MUST carry a non-empty `reason` string explaining why the delta
// requires a contract or API decision. Missing ports MUST NOT be baselined.
export const BASELINE: Record<string, BaselineEntry> = {
  // dock-region `showTabs`: Svelte carries `showTabs` as a spec-surface-pending
  // tranche awaiting DockRegionSpec tab strip modeling in g13.014 (see
  // contract-prop-drift.ts). React does not declare this pending port.
  "dock-region": {
    reason: "showTabs is a spec-surface-pending tranche awaiting DockRegionSpec tab fields (g13.014)",
    svelteOnly: ["showTabs"],
  },
};

/** Validates that all entries in the baseline register carry a non-empty reason string. */
export function validateBaseline(baseline: Record<string, unknown>): void {
  for (const [slug, raw] of Object.entries(baseline)) {
    if (!raw || typeof raw !== "object") {
      throw new Error(`Baseline entry for "${slug}" must be an object`);
    }
    const entry = raw as Record<string, unknown>;
    if (typeof entry.reason !== "string" || !entry.reason.trim()) {
      throw new Error(
        `Baseline entry for "${slug}" must have a non-empty reason string explaining why it requires a contract or API decision`,
      );
    }
  }
}

// Validate BASELINE on module load.
validateBaseline(BASELINE);

const DOM_ATTR_MAP: Record<string, string> = {
  accesskey: "accessKey",
  autocapitalize: "autoCapitalize",
  autocomplete: "autoComplete",
  autocorrect: "autoCorrect",
  class: "className",
  classname: "className",
  colspan: "colSpan",
  contenteditable: "contentEditable",
  crossorigin: "crossOrigin",
  for: "htmlFor",
  formaction: "formAction",
  formenctype: "formEncType",
  formmethod: "formMethod",
  formnovalidate: "formNoValidate",
  formtarget: "formTarget",
  htmlfor: "htmlFor",
  novalidate: "noValidate",
  readonly: "readOnly",
  rowspan: "rowSpan",
  spellcheck: "spellCheck",
  tabindex: "tabIndex",
};

/** Canonicalizes prop names to handle framework and DOM attribute casing idioms. */
export function canonicalizePropName(name: string): string {
  const lower = name.toLowerCase();
  return DOM_ATTR_MAP[lower] ?? name;
}

/** Extracts balanced brace `{ … }` body string- and comment-aware. */
export function extractBalancedBraces(src: string, openIndex: number): string | null {
  let depth = 0;
  let quote: string | null = null;
  let inLineComment = false;
  let inBlockComment = false;
  for (let i = openIndex; i < src.length; i++) {
    const ch = src[i];
    const next = src[i + 1];
    if (inLineComment) {
      if (ch === "\n") inLineComment = false;
      continue;
    }
    if (inBlockComment) {
      if (ch === "*" && next === "/") {
        inBlockComment = false;
        i++;
      }
      continue;
    }
    if (quote !== null) {
      if (ch === "\\") i++;
      else if (ch === quote) quote = null;
      continue;
    }
    if (ch === "/" && next === "/") {
      inLineComment = true;
      i++;
      continue;
    }
    if (ch === "/" && next === "*") {
      inBlockComment = true;
      i++;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === "`") {
      quote = ch;
      continue;
    }
    if (ch === "{") depth++;
    else if (ch === "}") {
      depth--;
      if (depth === 0) return src.slice(openIndex + 1, i);
    }
  }
  return null;
}

/** Finds the end index of a top-level type alias declaration (stopping at top-level `;`). */
function findTypeAliasEnd(src: string, startIdx: number): number {
  let depth = 0;
  for (let i = startIdx; i < src.length; i++) {
    const ch = src[i];
    if ("{([<".includes(ch)) depth++;
    else if ("})]>".includes(ch)) depth--;
    else if (ch === ";" && depth === 0) return i;
  }
  return src.length;
}

/** Parses property declarations from an interface or type literal body. */
function parsePropsFromBraceBody(body: string): Map<string, string> {
  const props = new Map<string, string>();
  const clean = body.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/[^\n]*/g, "");
  let depth = 0;
  let cur = "";
  let prev = "";
  for (let i = 0; i < clean.length; i++) {
    const ch = clean[i];
    const isArrow = ch === ">" && prev === "=";
    if ("{([<".includes(ch)) depth++;
    else if (!isArrow && "})]>".includes(ch)) depth--;
    prev = ch;
    if (ch === ";" && depth === 0) {
      const pm = cur.match(/^\s*([a-zA-Z_$][\w$]*)\s*\??\s*:\s*([\s\S]*)$/);
      if (pm) {
        props.set(pm[1], pm[2].trim());
      }
      cur = "";
      continue;
    }
    cur += ch;
  }
  if (cur.trim()) {
    const pm = cur.match(/^\s*([a-zA-Z_$][\w$]*)\s*\??\s*:\s*([\s\S]*)$/);
    if (pm) {
      props.set(pm[1], pm[2].trim());
    }
  }
  return props;
}

/** Extracts public props and static defaults from a Svelte component. */
export function parseSvelteProps(src: string): { props: Set<string>; defaults: Map<string, string> } {
  const props = new Set<string>();
  const defaults = new Map<string, string>();
  const anchor = src.indexOf("= $props()");
  if (anchor < 0) return { props, defaults };
  const letIdx = src.lastIndexOf("let {", anchor);
  if (letIdx < 0) return { props, defaults };
  const open = src.indexOf("{", letIdx);
  const body = extractBalancedBraces(src, open);
  if (!body) return { props, defaults };

  let d = 0;
  let cur = "";
  let quote: string | null = null;
  const parts: string[] = [];
  for (let i = 0; i < body.length; i++) {
    const ch = body[i];
    if (quote !== null) {
      cur += ch;
      if (ch === "\\" && i + 1 < body.length) {
        cur += body[i + 1];
        i++;
      } else if (ch === quote) quote = null;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === "`") {
      quote = ch;
      cur += ch;
      continue;
    }
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
    if (!t || t.startsWith("...")) continue;
    const m = t.match(/^([a-zA-Z_$][\w$]*)(?:\s*:\s*[a-zA-Z_$][\w$]*)?(?:\s*=\s*([\s\S]*))?$/);
    if (!m) continue;
    const name = m[1];
    props.add(name);
    if (m[2]) {
      let defaultVal = m[2].trim();
      const bindMatch = defaultVal.match(/^\$bindable(?:\s*<[^>]*>)?\s*\(([\s\S]*)\)$/);
      if (bindMatch) defaultVal = bindMatch[1].trim();
      // If defaultVal is a local identifier (e.g. DEFAULT_STICKY_TONES), resolve it from module-level const
      if (/^[a-zA-Z_$][\w$]*$/.test(defaultVal)) {
        const constMatch = src.match(new RegExp(`(?:const|let|var)\\s+${defaultVal}\\s*(?::\\s*[^=]+)?\\s*=\\s*([\\s\\S]*?);`));
        if (constMatch) {
          defaultVal = constMatch[1].trim();
        }
      }
      defaults.set(name, defaultVal);
    }
  }
  return { props, defaults };
}

/** Extracts public props and static defaults from a React component file. */
export function parseReactPropsFromSource(
  src: string,
  displayName: string,
  options?: { reactDir?: string },
): { props: Set<string>; defaults: Map<string, string> } {
  const dir = options?.reactDir ?? reactDir;
  const propsMap = new Map<string, string>();
  const defaults = new Map<string, string>();

  // Extract defaults from React component function destructuring
  const fnRegex = new RegExp(`export\\s+function\\s+${displayName}\\s*(?:<[^>]*>)?\\s*\\(\\s*\\{`);
  const fnMatch = src.match(fnRegex);
  if (fnMatch) {
    const openIdx = src.indexOf("{", fnMatch.index);
    const body = extractBalancedBraces(src, openIdx);
    if (body) {
      let d = 0;
      let cur = "";
      let quote: string | null = null;
      const parts: string[] = [];
      for (let i = 0; i < body.length; i++) {
        const ch = body[i];
        if (quote !== null) {
          cur += ch;
          if (ch === "\\" && i + 1 < body.length) {
            cur += body[i + 1];
            i++;
          } else if (ch === quote) quote = null;
          continue;
        }
        if (ch === '"' || ch === "'" || ch === "`") {
          quote = ch;
          cur += ch;
          continue;
        }
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
      for (const p of parts) {
        const m = p.trim().match(/^([a-zA-Z_$][\w$]*)\s*=\s*([\s\S]*)$/);
        if (m) {
          let val = m[2].trim();
          // If val is a local identifier (e.g. EMPTY), resolve it from top-level const
          if (/^[a-zA-Z_$][\w$]*$/.test(val)) {
            const constMatch = src.match(new RegExp(`(?:const|let|var)\\s+${val}\\s*(?::\\s*[^=]+)?\\s*=\\s*([\\s\\S]*?);`));
            if (constMatch) {
              val = constMatch[1].trim();
            }
          }
          defaults.set(m[1], val);
        }
      }
    }
  }

  if (displayName === "Tree") {
    const typesPath = path.join(dir, "types.ts");
    if (existsSync(typesPath)) {
      const typesSrc = readFileSync(typesPath, "utf8");
      const m = typesSrc.match(/(?:type|interface)\s+TreeCommonProps\s*(?:=\s*)?\{/);
      if (m) {
        const body = extractBalancedBraces(typesSrc, m.index! + m[0].length - 1);
        if (body) {
          for (const [k, v] of parsePropsFromBraceBody(body)) propsMap.set(k, v);
        }
      }
    }
    propsMap.set("reorderAuthority", "TreeReorderAuthority | null");
    propsMap.set("onReorder", "(from: string, to: string, position: DropPosition) => void");
  } else if (displayName === "UiPresentationProvider") {
    propsMap.set("density", "ControlDensity");
    propsMap.set("sizeScale", "ControlSize");
    propsMap.set("children", "ReactNode");
  } else if (displayName === "MotionPolicyProvider") {
    propsMap.set("policy", "MotionPolicy");
    propsMap.set("children", "ReactNode");
  } else {
    const ifaceRegex = new RegExp(`(?:export\\s+)?interface\\s+${displayName}Props\\b([^\\{]*)\\{`);
    const typeRegex = new RegExp(`(?:export\\s+)?type\\s+${displayName}Props\\s*=\\s*`);

    const ifaceMatch = src.match(ifaceRegex);
    if (ifaceMatch) {
      const extendsClause = ifaceMatch[1];
      if (extendsClause.includes("AudioPresentationProps")) {
        propsMap.set("size", "ControlSize | null");
        propsMap.set("sizeRole", "SemanticControlSizeRole");
        propsMap.set("density", "ControlDensity | null");
      }
      const openIndex = ifaceMatch.index! + ifaceMatch[0].length - 1;
      const body = extractBalancedBraces(src, openIndex);
      if (body) {
        for (const [k, v] of parsePropsFromBraceBody(body)) propsMap.set(k, v);
      }
    } else {
      const typeMatch = src.match(typeRegex);
      if (typeMatch) {
        const endIdx = findTypeAliasEnd(src, typeMatch.index!);
        const rhs = src.slice(typeMatch.index! + typeMatch[0].length, endIdx);
        // Find all identifiers in RHS (like PopoverCommonProps, PopoverTriggerProps)
        const idents = rhs.match(/[a-zA-Z_$][\w$]*/g) ?? [];
        for (const id of idents) {
          if (id === "boolean" || id === "string" || id === "number" || id === "null" || id === "undefined" || id === "never") continue;
          // Look for interface or type definition of id
          const idIface = src.match(new RegExp(`(?:export\\s+)?interface\\s+${id}\\b[^\\{]*\\{`));
          if (idIface) {
            const body = extractBalancedBraces(src, idIface.index! + idIface[0].length - 1);
            if (body) {
              for (const [k, v] of parsePropsFromBraceBody(body)) propsMap.set(k, v);
            }
          } else {
            const idType = src.match(new RegExp(`(?:export\\s+)?type\\s+${id}\\s*=\\s*`));
            if (idType) {
              const idEnd = findTypeAliasEnd(src, idType.index!);
              let idx = idType.index!;
              while ((idx = src.indexOf("{", idx + 1)) >= 0 && idx < idEnd) {
                const body = extractBalancedBraces(src, idx);
                if (body) {
                  for (const [k, v] of parsePropsFromBraceBody(body)) {
                    if (v !== "never") propsMap.set(k, v);
                  }
                }
              }
            }
          }
        }
        let idx = typeMatch.index!;
        while ((idx = src.indexOf("{", idx + 1)) >= 0 && idx < endIdx) {
          const body = extractBalancedBraces(src, idx);
          if (body) {
            for (const [k, v] of parsePropsFromBraceBody(body)) {
              if (v !== "never") propsMap.set(k, v);
            }
          }
        }
      }
    }
  }

  // Filter out snippet / slot / children props
  const props = new Set<string>();
  for (const [k, v] of propsMap) {
    if (k === "children" || k === "render") continue;
    if (/\b(?:ReactNode|Snippet)\b/.test(v)) continue;
    props.add(k);
  }

  return { props, defaults };
}

/** Resolves the underlying source file for a React component (following re-exports). */
function resolveReactComponentSource(displayName: string, reactComponentDir: string): string {
  const primaryPath = path.join(reactComponentDir, `${displayName}.tsx`);
  if (!existsSync(primaryPath)) return "";
  const src = readFileSync(primaryPath, "utf8");
  const reExport = src.match(
    new RegExp(`export\\s*\\{[^}]*?\\b${displayName}\\b[^}]*?\\}\\s*from\\s*["\x27]([^"\x27]+)["\x27]`),
  );
  if (reExport) {
    let target = path.resolve(reactComponentDir, reExport[1]);
    if (!target.endsWith(".tsx") && !target.endsWith(".ts")) {
      if (existsSync(target + ".tsx")) target += ".tsx";
      else if (existsSync(target + ".ts")) target += ".ts";
    }
    if (existsSync(target)) return readFileSync(target, "utf8");
  }
  return src;
}

export interface ReactPropDriftFinding {
  slug: string;
  displayName: string;
  svelteOnly: string[];
  reactOnly: string[];
  defaultDrift?: { prop: string; svelteDefault: string; reactDefault: string }[];
}

/** Normalizes literal default value string for comparison (collapsing whitespace/quotes/trailing commas). */
function normalizeDefaultValue(val: string): string {
  let trimmed = val.trim();
  // String literal normalize quotes
  if ((trimmed.startsWith('"') && trimmed.endsWith('"')) || (trimmed.startsWith("'") && trimmed.endsWith("'"))) {
    return `"${trimmed.slice(1, -1)}"`;
  }
  // Strip trailing commas before closing braces/brackets
  trimmed = trimmed.replace(/,\s*([\}\]])/g, "$1");
  // Normalize whitespace around punctuation
  trimmed = trimmed.replace(/\s*([{}[\]:,])\s*/g, "$1");
  return trimmed.replace(/\s+/g, " ");
}

/** Compares Svelte and React props for a single component. */
export function compareComponentProps(
  slug: string,
  displayName: string,
  allSProps: Set<string>,
  allRProps: Set<string>,
  snippets: Set<string>,
  contractPropsSet: Set<string>,
  baseline?: BaselineEntry,
  sDefaults?: Map<string, string>,
  rDefaults?: Map<string, string>,
): ReactPropDriftFinding | null {
  // Exclude Svelte snippets
  const sProps = new Set([...allSProps].filter((p) => !snippets.has(p) && p !== "children" && p !== "render"));
  const rProps = new Set([...allRProps].filter((p) => !snippets.has(p) && p !== "children" && p !== "render"));

  // Build canonicalized mapping
  const sCanonical = new Map<string, string>();
  for (const p of sProps) sCanonical.set(canonicalizePropName(p), p);

  const rCanonical = new Map<string, string>();
  for (const p of rProps) rCanonical.set(canonicalizePropName(p), p);

  const allowedSvelteOnly = baseline?.svelteOnly ?? [];
  const allowedReactOnly = baseline?.reactOnly ?? [];
  const allowedDefaultDrift = baseline?.defaultDrift ?? [];

  const svelteOnly: string[] = [];
  const reactOnly: string[] = [];

  for (const [canonical, orig] of sCanonical) {
    if (!rCanonical.has(canonical) && !allowedSvelteOnly.includes(orig)) {
      svelteOnly.push(orig);
    }
  }

  for (const [canonical, orig] of rCanonical) {
    if (!sCanonical.has(canonical) && !allowedReactOnly.includes(orig)) {
      reactOnly.push(orig);
    }
  }

  // Check static literal defaults for props present in both and documented in contract
  const defaultDrift: { prop: string; svelteDefault: string; reactDefault: string }[] = [];
  if (sDefaults && rDefaults) {
    for (const [sName, sVal] of sDefaults) {
      const canonical = canonicalizePropName(sName);
      if (rCanonical.has(canonical) && contractPropsSet.has(sName)) {
        const rName = rCanonical.get(canonical)!;
        if (rDefaults.has(rName)) {
          const rVal = rDefaults.get(rName)!;
          const sNorm = normalizeDefaultValue(sVal);
          const rNorm = normalizeDefaultValue(rVal);
          if (sNorm !== rNorm && !allowedDefaultDrift.includes(sName)) {
            defaultDrift.push({
              prop: sName,
              svelteDefault: sVal,
              reactDefault: rVal,
            });
          }
        }
      }
    }
  }

  if (svelteOnly.length > 0 || reactOnly.length > 0 || defaultDrift.length > 0) {
    return {
      slug,
      displayName,
      svelteOnly: svelteOnly.sort(),
      reactOnly: reactOnly.sort(),
      ...(defaultDrift.length > 0 ? { defaultDrift } : {}),
    };
  }
  return null;
}

export interface ReactPropDriftResult {
  checked: number;
  skipped: number;
  findings: ReactPropDriftFinding[];
}

/** Runs the public prop drift check across all catalogue components. */
export function reactPropDrift(options?: {
  repoRoot?: string;
  baseline?: Record<string, BaselineEntry>;
}): ReactPropDriftResult {
  const root = options?.repoRoot ?? repoRoot;
  const activeBaseline = options?.baseline ?? BASELINE;
  validateBaseline(activeBaseline);

  const curContractsDir = path.join(root, "docs/contracts/components");
  const curSvelteDir = path.join(root, "packages/svelte/components/src");
  const curReactDir = path.join(root, "packages/react/components/src");

  let checked = 0;
  let skipped = 0;
  const findings: ReactPropDriftFinding[] = [];

  for (const entry of allComponents) {
    const sveltePath = path.join(curSvelteDir, `${entry.displayName}.svelte`);
    const reactPath = path.join(curReactDir, `${entry.displayName}.tsx`);

    if (!existsSync(sveltePath) || !existsSync(reactPath)) {
      skipped++;
      continue;
    }

    const sSrc = readFileSync(sveltePath, "utf8");
    const rSrc = resolveReactComponentSource(entry.displayName, curReactDir);
    const sSnippets = snippetProps(sSrc);
    const { props: sProps, defaults: sDefaults } = parseSvelteProps(sSrc);
    const { props: rProps, defaults: rDefaults } = parseReactPropsFromSource(rSrc, entry.displayName, {
      reactDir: curReactDir,
    });

    const contractPath = path.join(curContractsDir, `${entry.slug}.md`);
    let contractPropsSet = new Set<string>();
    if (existsSync(contractPath)) {
      const { props } = contractProps(readFileSync(contractPath, "utf8"));
      contractPropsSet = props;
    }

    checked++;
    const finding = compareComponentProps(
      entry.slug,
      entry.displayName,
      sProps,
      rProps,
      sSnippets,
      contractPropsSet,
      activeBaseline[entry.slug],
      sDefaults,
      rDefaults,
    );

    if (finding) findings.push(finding);
  }

  return { checked, skipped, findings };
}

/** Formats findings as gate error strings. */
export function reactDriftErrors(options?: {
  repoRoot?: string;
  baseline?: Record<string, BaselineEntry>;
}): string[] {
  return reactPropDrift(options).findings.flatMap((f) => {
    const errors: string[] = [];
    if (f.svelteOnly.length > 0) {
      errors.push(
        `react prop drift: [${f.displayName}] (${f.slug}) Svelte prop(s) missing from React: ${f.svelteOnly.join(", ")}`,
      );
    }
    if (f.reactOnly.length > 0) {
      errors.push(
        `react prop drift: [${f.displayName}] (${f.slug}) React-only prop(s) absent from Svelte: ${f.reactOnly.join(", ")}`,
      );
    }
    if (f.defaultDrift && f.defaultDrift.length > 0) {
      for (const d of f.defaultDrift) {
        errors.push(
          `react prop drift: [${f.displayName}] (${f.slug}) prop "${d.prop}" default differs: Svelte="${d.svelteDefault}" vs React="${d.reactDefault}"`,
        );
      }
    }
    return errors;
  });
}

// Standalone report / gate execution: `bun packages/svelte/preview/scripts/react-prop-drift.ts`
if (import.meta.main) {
  const { checked, skipped, findings } = reactPropDrift();
  console.log(`react-prop-drift: checked ${checked}, skipped ${skipped} (missing component file)\n`);

  if (findings.length > 0) {
    const svelteMissing = findings.reduce((acc, f) => acc + f.svelteOnly.length, 0);
    const reactOnly = findings.reduce((acc, f) => acc + f.reactOnly.length, 0);
    const defaults = findings.reduce((acc, f) => acc + (f.defaultDrift?.length ?? 0), 0);
    const total = svelteMissing + reactOnly + defaults;

    console.log(
      `FAIL — ${total} drift issue(s) across ${findings.length} component(s) ` +
        `(${svelteMissing} missing in React, ${reactOnly} React-only, ${defaults} default drift):\n`,
    );

    for (const f of findings) {
      console.log(`  [${f.displayName}] (${f.slug}):`);
      if (f.svelteOnly.length > 0) {
        console.log(`    missing from React (Svelte-only): ${f.svelteOnly.join(", ")}`);
      }
      if (f.reactOnly.length > 0) {
        console.log(`    React-only: ${f.reactOnly.join(", ")}`);
      }
      if (f.defaultDrift && f.defaultDrift.length > 0) {
        for (const d of f.defaultDrift) {
          console.log(`    default drift on "${d.prop}": Svelte="${d.svelteDefault}" vs React="${d.reactDefault}"`);
        }
      }
    }
    console.log("");

    console.log("=== Grouped Findings Summary ===");
    console.log("\n[Port to React] (Svelte props absent from React):");
    for (const f of findings) {
      if (f.svelteOnly.length > 0) {
        console.log(`  - ${f.displayName} (${f.slug}): ${f.svelteOnly.join(", ")}`);
      }
    }
    console.log("\n[Candidate for Svelte Inclusion / Needs Decision] (React-only props):");
    for (const f of findings) {
      if (f.reactOnly.length > 0) {
        console.log(`  - ${f.displayName} (${f.slug}): ${f.reactOnly.join(", ")}`);
      }
    }
    if (defaults > 0) {
      console.log("\n[Default Drift] (Static literal default mismatch):");
      for (const f of findings) {
        if (f.defaultDrift && f.defaultDrift.length > 0) {
          for (const d of f.defaultDrift) {
            console.log(`  - ${f.displayName} (${f.slug}) "${d.prop}": Svelte="${d.svelteDefault}" vs React="${d.reactDefault}"`);
          }
        }
      }
    }
    console.log("");
  } else {
    console.log("OK — every public prop is aligned between Svelte and React.");
  }

  if (findings.length > 0 && process.env.DRIFT_REPORT !== "1") {
    process.exit(1);
  }
}
