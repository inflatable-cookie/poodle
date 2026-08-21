// Contract <-> implementation value-domain drift check.
//
// The prop-surface drift gates (`contract-prop-drift.ts`, `contract-spec-drift.ts`)
// verify that a prop *exists* on both sides; nothing verifies that its *permitted
// values* agree. That blind spot let `ButtonTone` fragment across three contracts
// and three stylesheets undetected. This compares, per component, the permitted
// value set of each enumerated prop:
//
//   - Contract side — the union in the component contract's §3 "Public Props"
//     table, whether written inline (`"a" | "b"`) or as a named type reference
//     (`ButtonTone`, `StatusTone`, `ControlSize`, …).
//   - TypeScript side — the prop's type in the component's `Props` interface,
//     with named types resolved through `packages/svelte/components/src/types.ts`,
//     component-local aliases, and `@inflatable-cookie/poodle-core`.
//   - Rust side — the corresponding enum in `packages/contracts/components/src`
//     (types.rs or the component's spec module), reached through the matching
//     `<Name>Spec` struct field.
//
// Report-only by default: the script exits 0 regardless of findings. Set
// `VALUE_DOMAIN_ENFORCE=1` to exit 1 on any finding — the inverse of the
// `DRIFT_REPORT=1` escape the sibling drift scripts use. This is deliberately
// NOT wired into `docs:check`: the backlog is unknown until this inventory
// exists, and a gate that fails the build on day one blocks everyone.
//
// Normalisation:
//   - `null` / `undefined` union members are absence markers, not domain values;
//     they are stripped on every side before comparison.
//   - Rust enum variant names project to their string literal via kebab-case
//     (`TopStart` -> "top-start"). Divergences between that projection and the
//     web literal (`AlertDialog` -> "alert-dialog" vs "alertdialog",
//     `FirstRun` -> "first-run" vs "firstRun", `SpaceBetween` -> "space-between"
//     vs "between") are reported as findings, not silently forgiven — the
//     orchestrator decides whether a spelling is a convention.
//   - Array-wrapped unions (`("icon" | "count")[]`) compare their inner set.
//   - Named types that cannot be resolved from `docs/` are findings of class
//     `unresolved-type` — never guessed. A named type that resolves to a
//     function/object/interface shape on the TypeScript side is a non-enumerated
//     prop (callbacks/objects are out of scope) and is skipped.
//
// Documented exceptions (accepted deviation, cited, not drift):
//   - `ButtonVariant::Danger` (types.rs) — retained for backward compatibility
//     only; equivalent to `Primary` + `danger` tone. Not part of the authored
//     vocabulary per docs/contracts/004-shared-control-types.md. Filtered from
//     the Rust `ButtonVariant` set so the three button contracts compare clean.
//   - Snippet-typed props (`leading`, `trailing`, …) — framework slot idiom,
//     excluded the same way `contract-prop-drift.ts` excludes them.

import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";

import { allComponents } from "../src/component-registry.ts";
import { unionPropsBody } from "./contract-prop-drift.ts";

const repoRoot = path.resolve(import.meta.dir, "../../../..");
const contractsDir = path.join(repoRoot, "docs/contracts/components");
const svelteDir = path.join(repoRoot, "packages/svelte/components/src");
const rustDir = path.join(repoRoot, "packages/contracts/components/src");
const docsRoot = path.join(repoRoot, "docs");

const TYPES_TS = path.join(svelteDir, "types.ts");
const SHARED_TYPES_CONTRACT = "docs/contracts/004-shared-control-types.md";

/** Rust enum variants kept for backward compatibility but not part of the
 * authored vocabulary — filtered before comparison, with the doc citation. */
const RUST_LEGACY_VARIANTS: Record<string, string[]> = {
  // docs/contracts/004-shared-control-types.md: "ButtonVariant::Danger is
  // retained in Rust for backward compatibility only and is equivalent to
  // Primary + Danger tone. It is not part of the authored vocabulary…"
  ButtonVariant: ["Danger"],
};

/** Framework-idiom function types that are not value domains. Snippet-typed
 * props are slot plumbing documented separately in contracts; excluded here the
 * same way contract-prop-drift.ts excludes them. */
const FRAMEWORK_TYPES: Record<string, true> = { Snippet: true };

// ---------------------------------------------------------------------------
// Contract side
// ---------------------------------------------------------------------------

/** Split a markdown table line on unescaped pipes (cells may contain `\|`). */
function splitCells(line: string): string[] {
  const cells: string[] = [];
  let cur = "";
  for (let i = 0; i < line.length; i++) {
    if (line[i] === "|" && (i === 0 || line[i - 1] !== "\\")) {
      cells.push(cur);
      cur = "";
    } else cur += line[i];
  }
  cells.push(cur);
  return cells.map((c) => c.trim());
}

/** prop -> raw type cell from the "### Public Props" table. */
function contractPropTypes(md: string): Map<string, string> {
  const props = new Map<string, string>();
  const start = md.indexOf("### Public Props");
  if (start < 0) return props;
  const rest = md.slice(start + "### Public Props".length);
  const end = rest.search(/\n#{2,4} /);
  const table = end < 0 ? rest : rest.slice(0, end);
  for (const line of table.split("\n")) {
    if (!line.trim().startsWith("|")) continue;
    const cells = splitCells(line);
    if (cells.length < 3 || cells[1] === "Prop") continue;
    const m = cells[1].match(/^`([a-zA-Z_$][\w$]*)`$/);
    if (!m) continue;
    props.set(
      m[1],
      cells[2].replace(/^`|`$/g, "").replace(/\\\|/g, "|").replace(/\s+/g, " ").trim(),
    );
  }
  return props;
}

/** Split a union expression on top-level `|`. */
function splitUnion(expr: string): string[] {
  const parts: string[] = [];
  let depth = 0;
  let cur = "";
  for (let i = 0; i < expr.length; i++) {
    const ch = expr[i];
    if ("{([<".includes(ch)) depth++;
    else if ("})]>".includes(ch)) depth--;
    if (ch === "|" && depth === 0) {
      parts.push(cur.trim());
      cur = "";
    } else cur += ch;
  }
  if (cur.trim()) parts.push(cur.trim());
  return parts.filter(Boolean);
}

/** Extract string/number literal members from a union expression. */
function literalMembers(expr: string): string[] {
  const members: string[] = [];
  const re = /"([^"]*)"|'([^']*)'|\b(\d+)\b/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(expr)) !== null) members.push(m[1] ?? m[2] ?? m[3]);
  return members;
}

export type ExprInfo = {
  named: string[];
  literals: string[];
  /** True when every member is a literal or a resolvable named type. */
  enumerated: boolean;
};

/** Classify a type expression. `null`/`undefined` are stripped (absence, not a
 * domain value). An array wrapper is unwrapped only when the element is a
 * parenthesized literal union — `("icon" | "count")[]` compares its inner set,
 * while `MenuItem[]` stays an object-array prop (out of scope). */
export function exprInfo(expr: string): ExprInfo {
  let e = expr.trim();
  if (e.endsWith("[]")) {
    const inner = e.slice(0, -2).trim();
    if (inner.startsWith("(") || /^"|^'|^\d/.test(inner)) e = inner.slice(1, -1).trim();
  }
  // Hybrid form `SurfaceTone: "panel" | "canvas" | "elevated"` — a named type
  // with an inline restatement in the same cell. `Name:` -> `Name |` so the
  // restatement reads as literal members alongside the reference.
  e = e.replace(/^([A-Z][A-Za-z0-9_]*)\s*:\s*(?=["'\d])/, "$1 | ");
  const named: string[] = [];
  const literals: string[] = [];
  let enumerated = true;
  for (const part of splitUnion(e)) {
    if (/^(null|undefined)$/.test(part)) continue;
    const lit = part.match(/^"([^"]*)"$/);
    const num = part.match(/^\d+$/);
    if (lit) {
      literals.push(lit[1]);
      continue;
    }
    if (num) {
      literals.push(num[0]);
      continue;
    }
    if (/^[A-Z][A-Za-z0-9_]*$/.test(part)) {
      named.push(part);
      continue;
    }
    enumerated = false; // function, object, array element, `string` mix, DOM types…
  }
  return { named, literals, enumerated };
}

// ---------------------------------------------------------------------------
// Docs named-type resolution (contract side)
// ---------------------------------------------------------------------------

type DocsDef = { file: string; exprs: string[] };

function walkMdFiles(dir: string): string[] {
  const files: string[] = [];
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) files.push(...walkMdFiles(p));
    else if (e.name.endsWith(".md")) files.push(p);
  }
  return files;
}

/** `type X = …` statements inside fenced code blocks. */
function codeBlockTypes(md: string): Record<string, string[]> {
  const out: Record<string, string[]> = {};
  const fenceRe = /```[a-zA-Z]*\n([\s\S]*?)```/g;
  let m: RegExpExecArray | null;
  while ((m = fenceRe.exec(md)) !== null) {
    const lines = m[1].split("\n");
    for (let i = 0; i < lines.length; i++) {
      const lm = lines[i].match(/^(?:type\s+)?([A-Z][A-Za-z0-9_]*)\s*[=:]\s*(.*)$/);
      if (!lm) continue;
      let expr = lm[2].trim();
      let j = i + 1;
      while (j < lines.length) {
        const nl = lines[j].trim();
        if (nl === "" || nl.startsWith("```")) break;
        if (/^\|/.test(nl) || (expr === "" && /^["'\d]/.test(nl)) || /[|,]\s*$/.test(expr)) {
          expr += " " + nl;
          j++;
        } else break;
      }
      i = j - 1;
      expr = expr.replace(/;\s*$/, "").trim();
      if (!out[lm[1]]) out[lm[1]] = [];
      out[lm[1]].push(expr);
    }
  }
  return out;
}

/** `- \`X\`: \`"a" | "b"\`` bullet restatements. */
function bulletTypes(md: string): Record<string, string> {
  const out: Record<string, string> = {};
  const re = /^\s*-\s*`([A-Z][A-Za-z0-9_]*)`\s*:\s*`([^`]+)`/gm;
  let m: RegExpExecArray | null;
  while ((m = re.exec(md)) !== null) {
    const members = literalMembers(m[2].replace(/\\\|/g, "|"));
    if (members.length > 0 && /^["'\d]/.test(m[2].trim()))
      out[m[1]] = members.map((x) => (/^\d+$/.test(x) ? x : `"${x}"`)).join(" | ");
  }
  return out;
}

/** `` `X = "a" | "b"` `` / `` `X: "a" | "b"` `` inline-code forms. */
function inlineCodeTypes(md: string): Record<string, string> {
  const out: Record<string, string> = {};
  const re = /`([A-Z][A-Za-z0-9_]*)\s*[=:]\s*([^`]+)`/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(md)) !== null) {
    const members = literalMembers(m[2].replace(/\\\|/g, "|"));
    if (members.length > 0 && /^["'\d]/.test(m[2].trim()))
      out[m[1]] = members.map((x) => (/^\d+$/.test(x) ? x : `"${x}"`)).join(" | ");
  }
  return out;
}

/** Every `type X = …` literal-union definition found anywhere in docs/. */
function collectDocsDefs(): Map<string, DocsDef[]> {
  const defs = new Map<string, DocsDef[]>();
  const push = (name: string, exprs: string[], file: string) => {
    if (!defs.has(name)) defs.set(name, []);
    defs.get(name)!.push({ file, exprs });
  };
  for (const f of walkMdFiles(docsRoot)) {
    const md = readFileSync(f, "utf8");
    const rel = path.relative(repoRoot, f);
    for (const [n, exprs] of Object.entries(codeBlockTypes(md))) push(n, exprs, rel);
    for (const [n, e] of Object.entries(bulletTypes(md))) push(n, [e], rel);
    for (const [n, e] of Object.entries(inlineCodeTypes(md))) push(n, [e], rel);
  }
  return defs;
}

const docsDefs = collectDocsDefs();

/** Rank a definition for a given component: 004 is canonical, then the
 * component's own contract, then other contracts (deterministic by file), then
 * remaining docs. */
function defRank(file: string, ownRel: string): number {
  if (file === SHARED_TYPES_CONTRACT) return 0;
  if (file === ownRel) return 1;
  if (file.startsWith("docs/contracts/components/")) return 2;
  if (file.startsWith("docs/contracts/")) return 3;
  return 4;
}

function resolveDocsDef(name: string, ownRel: string): DocsDef | null {
  const defs = docsDefs.get(name);
  if (!defs || defs.length === 0) return null;
  let best: DocsDef | null = null;
  let bestRank = 5;
  for (const d of defs) {
    const rank = defRank(d.file, ownRel);
    if (rank < bestRank || (rank === bestRank && (!best || d.file < best.file))) {
      best = d;
      bestRank = rank;
    }
  }
  return best;
}

/** Resolve a contract-side type expression to its literal set. Named refs whose
 * docs definition carries no literals (`type MenuItem = { … }`) are object
 * shapes — the prop is not an enumeration. */
function resolveContractSet(
  expr: string,
  ownRel: string,
): { set: Set<string>; unresolved: string[]; nonEnumNamed: string[] } {
  const info = exprInfo(expr);
  const set = new Set(info.literals);
  const unresolved: string[] = [];
  const nonEnumNamed: string[] = [];
  for (const n of info.named) {
    const def = resolveDocsDef(n, ownRel);
    if (!def) {
      unresolved.push(n);
      continue;
    }
    const members = def.exprs.flatMap((e) => literalMembers(e.replace(/\\\|/g, "|")));
    if (members.length === 0) nonEnumNamed.push(n);
    else for (const l of members) set.add(l);
  }
  return { set, unresolved, nonEnumNamed };
}

// ---------------------------------------------------------------------------
// TypeScript side
// ---------------------------------------------------------------------------

type AliasMap = Record<string, string>;

/** `export type X = …` aliases from a TS source. */
function tsAliases(src: string): AliasMap {
  const out: AliasMap = {};
  const re = /export\s+type\s+([A-Z][A-Za-z0-9_]*)\s*=\s*([\s\S]*?);/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(src)) !== null) {
    out[m[1]] = m[2].trim().replace(/\s+/g, " ").replace(/\s*\|\s*/g, " | ");
  }
  return out;
}

/** `export interface X` names — interfaces are object shapes, never enums. */
function tsInterfaces(src: string): Set<string> {
  const out = new Set<string>();
  const re = /export\s+interface\s+([A-Z][A-Za-z0-9_]*)/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(src)) !== null) out.add(m[1]);
  return out;
}

/** `export type { X as Y } from "pkg"` re-export names. */
function tsReexports(src: string): Map<string, string> {
  const out = new Map<string, string>();
  const re = /export\s+type\s*\{([^}]+)\}\s*from\s*["']([^"']+)["']/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(src)) !== null) {
    for (const part of m[1].split(",")) {
      const pm = part.trim().match(/^([A-Za-z0-9_]+)(?:\s+as\s+([A-Za-z0-9_]+))?$/);
      if (!pm) continue;
      out.set(pm[2] ?? pm[1], m[2]);
    }
  }
  return out;
}

/** Prop -> type expression from a component's Props (interface or inline
 * `let { … }: { … } = $props()` form). */
function tsProps(src: string): Record<string, string> {
  const out: Record<string, string> = {};
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
  for (const ch of body) {
    if ("{([<".includes(ch)) depth++;
    else if ("})]>".includes(ch)) depth--;
    if (ch === ";" && depth === 0) {
      const pm = cur.match(/^\s*([a-zA-Z_$][\w$]*)\s*\??\s*:\s*/);
      if (pm)
        out[pm[1]] = cur
          .slice(pm[0].length)
          .trim()
          .replace(/\s+/g, " ")
          .replace(/^\s*\|/, "")
          .trim();
      cur = "";
      continue;
    }
    cur += ch;
  }
  return out;
}

/** Component-local `type X = …` aliases and `interface X` markers. */
function localTypes(src: string): { aliases: AliasMap; interfaces: Set<string> } {
  const aliases: AliasMap = {};
  const interfaces = new Set<string>();
  const re = /(?:^|\n)\s*type\s+([A-Z][A-Za-z0-9_]*)\s*=\s*([\s\S]*?);(?=\n)/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(src)) !== null) {
    aliases[m[1]] = m[2].trim().replace(/\s+/g, " ").replace(/\s*\|\s*/g, " | ");
  }
  const ire = /(?:^|\n)\s*interface\s+([A-Z][A-Za-z0-9_]*)/g;
  while ((m = ire.exec(src)) !== null) interfaces.add(m[1]);
  return { aliases, interfaces };
}

function coreTsSurface(): { aliases: AliasMap; interfaces: Set<string> } {
  const aliases: AliasMap = {};
  const interfaces = new Set<string>();
  const dir = path.join(repoRoot, "packages/core/src");
  const files: string[] = [];
  const stack = [dir];
  while (stack.length > 0) {
    const d = stack.pop()!;
    for (const e of readdirSync(d, { withFileTypes: true })) {
      const p = path.join(d, e.name);
      if (e.isDirectory()) stack.push(p);
      else if (e.name.endsWith(".ts")) files.push(p);
    }
  }
  for (const f of files) {
    const src = readFileSync(f, "utf8");
    Object.assign(aliases, tsAliases(src));
    for (const n of tsInterfaces(src)) interfaces.add(n);
  }
  return { aliases, interfaces };
}

const typesTsAliases = tsAliases(readFileSync(TYPES_TS, "utf8"));
const typesTsInterfaces = tsInterfaces(readFileSync(TYPES_TS, "utf8"));
const coreTs = coreTsSurface();

/** Resolve a TS-side type expression to its literal set. Returns null when the
 * expression is not an enumeration, or `unresolved:<name>` when a named type
 * cannot be resolved locally. */
function resolveTsSet(
  expr: string,
  locals: { aliases: AliasMap; interfaces: Set<string> },
  seen = new Set<string>(),
): Set<string> | null | string {
  const info = exprInfo(expr);
  if (!info.enumerated) return null;
  const set = new Set(info.literals);
  for (const n of info.named) {
    if (seen.has(n)) continue;
    seen.add(n);
    const local = locals.aliases[n];
    const tts = typesTsAliases[n];
    let def: string | null = null;
    if (local) def = local;
    else if (tts) def = tts;
    else if (typesTsInterfaces.has(n) || locals.interfaces.has(n) || coreTs.interfaces.has(n)) return null;
    else if (coreTs.aliases[n]) def = coreTs.aliases[n];
    else return `unresolved:${n}`;
    const sub = resolveTsSet(def, locals, seen);
    if (sub === null || typeof sub === "string") return sub;
    for (const l of sub) set.add(l);
  }
  return set;
}

/** True when a named type resolves to a non-enumeration (object/function) in
 * the TS sources — used to classify contract-side named types that docs do not
 * define, without guessing. */
function tsNamedTypeIsNonEnum(
  name: string,
  locals: { aliases: AliasMap; interfaces: Set<string> },
  seen = new Set<string>(),
): boolean {
  if (seen.has(name)) return false;
  seen.add(name);
  if (FRAMEWORK_TYPES[name]) return true;
  const local = locals.aliases[name];
  const tts = typesTsAliases[name];
  let def: string | null = null;
  if (local) def = local;
  else if (tts) def = tts;
  else if (typesTsInterfaces.has(name) || locals.interfaces.has(name) || coreTs.interfaces.has(name)) return true;
  else if (coreTs.aliases[name]) def = coreTs.aliases[name];
  else return false;
  const info = exprInfo(def);
  if (!info.enumerated) return true;
  return info.named.some((n) => tsNamedTypeIsNonEnum(n, locals, seen));
}

// ---------------------------------------------------------------------------
// Rust side
// ---------------------------------------------------------------------------

/** camelCase -> snake_case (matches contract-spec-drift.ts). */
function snake(name: string): string {
  return name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
}

/** CamelCase -> kebab-case — the string-literal projection of a Rust variant
 * name (`TopStart` -> "top-start"). */
function kebab(name: string): string {
  return name.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase();
}

/** Unit-variant enums across the whole crate: name -> variants. */
function rustEnums(): Map<string, string[]> {
  const enums = new Map<string, string[]>();
  const files: string[] = [];
  const stack = [rustDir];
  while (stack.length > 0) {
    const d = stack.pop()!;
    for (const e of readdirSync(d, { withFileTypes: true })) {
      const p = path.join(d, e.name);
      if (e.isDirectory()) stack.push(p);
      else if (e.name.endsWith(".rs")) files.push(p);
    }
  }
  const re = /pub enum\s+(\w+)\s*\{([\s\S]*?)\n\}/g;
  for (const f of files) {
    const src = readFileSync(f, "utf8");
    let m: RegExpExecArray | null;
    while ((m = re.exec(src)) !== null) {
      const variants: string[] = [];
      for (const line of m[2].split("\n")) {
        const v = line.match(/^\s*(\w+)\s*(?:\(|,|$)/);
        if (v) variants.push(v[1]);
      }
      // Drop payload variants (`Single(String)`), which are not enumerations.
      const pure = variants.filter((v) => !m[2].split("\n").some((l) => l.trim() === `${v}(`));
      if (pure.length > 0) enums.set(m[1], pure);
    }
  }
  return enums;
}

const rustEnumMap = rustEnums();

function specFile(displayName: string): string | null {
  const s = snake(displayName);
  const flat = path.join(rustDir, `${s}.rs`);
  const mod = path.join(rustDir, s, "mod.rs");
  if (existsSync(flat)) return flat;
  if (existsSync(mod)) return mod;
  return null;
}

/** `<Name>Spec` struct fields: snake_case field -> type expression. */
function specFields(displayName: string): Record<string, string> | null {
  const f = specFile(displayName);
  if (!f) return null;
  const src = readFileSync(f, "utf8");
  const re = new RegExp(`pub struct\\s+${displayName}Spec\\s*\\{([\\s\\S]*?)\\n\\}`);
  const m = src.match(re);
  if (!m) return null;
  const fields: Record<string, string> = {};
  for (const line of m[1].split("\n")) {
    const fm = line.match(/^\s*pub\s+([a-z_][a-z0-9_]*)\s*:\s*(.+?),?\s*$/);
    if (fm) fields[fm[1]] = fm[2].trim().replace(/,$/, "");
  }
  return fields;
}

/** `Option<Vec<MenuEntry>>` -> `MenuEntry`. */
function bareType(ty: string): string {
  let t = ty.trim().replace(/,$/, "");
  for (;;) {
    const m = t.match(/^(?:Option|Vec|Box|Arc|Rc)<(.+)>$/);
    if (!m) break;
    t = m[1].trim();
  }
  return t;
}

/** Resolve a prop's Rust value set from its Spec field's enum. Null when the
 * spec does not carry the prop, the field is not an enum, or there is no spec. */
function resolveRustSet(
  displayName: string,
  prop: string,
): { set: Set<string>; enumName: string } | null {
  const fields = specFields(displayName);
  if (!fields) return null;
  const field = fields[snake(prop)];
  if (!field) return null;
  const base = bareType(field);
  const variants = rustEnumMap.get(base);
  if (!variants) return null;
  const legacy = RUST_LEGACY_VARIANTS[base];
  return {
    set: new Set(variants.filter((v) => !legacy?.includes(v)).map(kebab)),
    enumName: base,
  };
}

// ---------------------------------------------------------------------------
// Comparison
// ---------------------------------------------------------------------------

export type ValueDomainFinding = {
  slug: string;
  prop: string;
  side: "ts" | "rust";
  classification: "contract-wider" | "impl-wider";
  contract: string[];
  impl: string[];
  onlyContract: string[];
  onlyImpl: string[];
};

export type UnresolvedTypeFinding = {
  slug: string;
  prop: string;
  typeName: string;
  cell: string;
};

export function contractValueDomainDrift(): {
  checkedComponents: number;
  checkedProps: number;
  skippedNonEnum: number;
  skippedNoTsSide: number;
  skippedNoRustSide: number;
  skippedTsUnresolvable: number;
  findings: ValueDomainFinding[];
  unresolved: UnresolvedTypeFinding[];
  legacyApplied: { slug: string; prop: string; enumName: string; variants: string[] }[];
} {
  const findings: ValueDomainFinding[] = [];
  const unresolved: UnresolvedTypeFinding[] = [];
  const legacyApplied: { slug: string; prop: string; enumName: string; variants: string[] }[] = [];
  let checkedComponents = 0;
  let checkedProps = 0;
  let skippedNonEnum = 0;
  let skippedNoTsSide = 0;
  let skippedNoRustSide = 0;
  let skippedTsUnresolvable = 0;

  for (const entry of allComponents) {
    const contractPath = path.join(contractsDir, `${entry.slug}.md`);
    const sveltePath = path.join(svelteDir, `${entry.displayName}.svelte`);
    if (!existsSync(contractPath) || !existsSync(sveltePath)) continue;
    const md = readFileSync(contractPath, "utf8");
    const svSrc = readFileSync(sveltePath, "utf8");
    const props = tsProps(svSrc);
    const locals = localTypes(svSrc);
    const contractPropsMap = contractPropTypes(md);
    if (contractPropsMap.size === 0) continue;
    checkedComponents++;
    const ownRel = `docs/contracts/components/${entry.slug}.md`;

    for (const [prop, cell] of contractPropsMap) {
      const info = exprInfo(cell);
      if (!info.enumerated) {
        skippedNonEnum++;
        continue;
      }
      const contractRes = resolveContractSet(cell, ownRel);
      if (contractRes.nonEnumNamed.length > 0) {
        // The named reference resolves in docs to an object shape — a
        // non-enumerated prop (objects are out of scope).
        skippedNonEnum++;
        continue;
      }
      if (contractRes.unresolved.length > 0) {
        // A named type docs cannot resolve. Non-enumerated shapes (callbacks,
        // objects, interfaces) are out of scope — classify via the TS side
        // rather than guessing. Genuinely enumerated types docs fail to define
        // are the finding.
        const stillUnresolved = contractRes.unresolved.filter(
          (n) => !tsNamedTypeIsNonEnum(n, locals),
        );
        for (const n of stillUnresolved)
          unresolved.push({ slug: entry.slug, prop, typeName: n, cell });
        if (stillUnresolved.length > 0) continue;
        skippedNonEnum++;
        continue;
      }
      checkedProps++;

      // TypeScript side
      const tsCell = props[prop];
      let tsSet: Set<string> | null = null;
      let tsSkipped = false;
      if (tsCell !== undefined) {
        const r = resolveTsSet(tsCell, locals);
        if (r === null || typeof r === "string") {
          skippedTsUnresolvable++;
          tsSkipped = true;
        } else tsSet = r;
      } else {
        skippedNoTsSide++;
        tsSkipped = true;
      }

      // Rust side
      const rRes = resolveRustSet(entry.displayName, prop);
      if (rRes === null) skippedNoRustSide++;
      else if (RUST_LEGACY_VARIANTS[rRes.enumName])
        legacyApplied.push({
          slug: entry.slug,
          prop,
          enumName: rRes.enumName,
          variants: RUST_LEGACY_VARIANTS[rRes.enumName],
        });

      if (tsSet && !tsSkipped) {
        const onlyContract = [...contractRes.set].filter((v) => !tsSet!.has(v)).sort();
        const onlyImpl = [...tsSet].filter((v) => !contractRes.set.has(v)).sort();
        if (onlyContract.length > 0 || onlyImpl.length > 0) {
          findings.push({
            slug: entry.slug,
            prop,
            side: "ts",
            classification: onlyContract.length > 0 ? "contract-wider" : "impl-wider",
            contract: [...contractRes.set].sort(),
            impl: [...tsSet].sort(),
            onlyContract,
            onlyImpl,
          });
        }
      }
      if (rRes) {
        const onlyContract = [...contractRes.set].filter((v) => !rRes.set.has(v)).sort();
        const onlyImpl = [...rRes.set].filter((v) => !contractRes.set.has(v)).sort();
        if (onlyContract.length > 0 || onlyImpl.length > 0) {
          findings.push({
            slug: entry.slug,
            prop,
            side: "rust",
            classification: onlyContract.length > 0 ? "contract-wider" : "impl-wider",
            contract: [...contractRes.set].sort(),
            impl: [...rRes.set].sort(),
            onlyContract,
            onlyImpl,
          });
        }
      }
    }
  }

  return {
    checkedComponents,
    checkedProps,
    skippedNonEnum,
    skippedNoTsSide,
    skippedNoRustSide,
    skippedTsUnresolvable,
    findings,
    unresolved,
    legacyApplied,
  };
}

if (import.meta.main) {
  const r = contractValueDomainDrift();
  console.log(
    `contract-value-domain-drift: checked ${r.checkedComponents} components / ${r.checkedProps} enumerated props ` +
      `(skipped ${r.skippedNonEnum} non-enumerated props, ${r.skippedNoTsSide} with no TS side, ` +
      `${r.skippedNoRustSide} with no Rust side, ${r.skippedTsUnresolvable} with non-comparable TS types)\n`,
  );

  if (r.unresolved.length > 0) {
    console.log(
      `${r.unresolved.length} named type(s) referenced by contracts have no resolvable definition in docs/ (unresolved-type):`,
    );
    for (const f of r.unresolved)
      console.log(`  [${f.slug}] ${f.prop}: ${f.typeName}  (cell: \`${f.cell}\`)`);
    console.log("");
  }

  if (r.legacyApplied.length > 0) {
    const byEnum = new Map<string, string[]>();
    for (const l of r.legacyApplied) {
      if (!byEnum.has(l.enumName)) byEnum.set(l.enumName, []);
      byEnum.get(l.enumName)!.push(`${l.slug}.${l.prop}`);
    }
    console.log("documented exceptions applied (not findings):");
    for (const [enumName, props] of byEnum)
      console.log(
        `  ${enumName} variants ${JSON.stringify(RUST_LEGACY_VARIANTS[enumName])} dropped for ` +
          `${props.join(", ")} — per docs/contracts/004-shared-control-types.md`,
      );
    console.log("");
  }

  if (r.findings.length > 0) {
    const n = r.findings.length;
    console.log(`${n} value-domain disagreement(s) across ${new Set(r.findings.map((f) => f.slug)).size} component(s):`);
    for (const f of r.findings) {
      console.log(
        `  [${f.slug}] ${f.prop} (${f.side}) ${f.classification}\n` +
          `      contract={${f.contract.join(", ")}}\n` +
          `      impl={${f.impl.join(", ")}}\n` +
          `      contract-only={${f.onlyContract.join(", ")}} impl-only={${f.onlyImpl.join(", ")}}`,
      );
    }
    console.log("");
  } else {
    console.log("OK — every enumerated prop's permitted value set agrees across contract, Svelte, and poodle-specs.");
  }

  const byClass = new Map<string, number>();
  for (const f of r.findings) byClass.set(f.classification, (byClass.get(f.classification) ?? 0) + 1);
  console.log(
    `summary: ${r.findings.length} findings (${[...byClass.entries()].map(([c, n]) => `${c}: ${n}`).join(", ")}), ` +
      `${r.unresolved.length} unresolved-type`,
  );
  // Report-only by default. VALUE_DOMAIN_ENFORCE=1 mirrors the DRIFT_REPORT
  // escape inverted: the gate fails only when explicitly asked to enforce.
  if (r.findings.length > 0 && process.env.VALUE_DOMAIN_ENFORCE === "1") process.exit(1);
}
