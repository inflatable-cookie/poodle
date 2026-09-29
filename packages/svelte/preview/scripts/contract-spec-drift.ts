// Contract <-> poodle-specs prop-surface drift check.
//
// The sibling `contract-prop-drift.ts` guards the web side: every documented
// public prop exists on the Svelte component. Nothing guarded the native side,
// and both native targets read their props from one place — the `poodle-specs`
// crate. A prop that lands in the contract and in Svelte but never reaches the
// Spec struct is invisible to GPUI and Jetstream, and no gate could see it.
//
// This compares the contract's "### Public Props" table against the fields of
// the matching `<Name>Spec` struct.
//
// Normalisation, because Rust and TS spell the same prop differently:
//   - camelCase -> snake_case
//   - booleans take an `is_` / `has_` prefix in Rust (`disabled` -> `is_disabled`)
//   - `on*` callbacks are excluded on both sides (contracts document them under
//     Events; specs are data, not behaviour)

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { allComponents } from "../src/component-registry.ts";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../..");
const contractsDir = path.join(repoRoot, "docs/contracts/components");
const specsDir = path.join(repoRoot, "packages/contracts/components/src");

/**
 * One keyed register of web-only prop exemptions.
 *
 * Outer key `"*"` is every component. Any other key is a slug: the same prop
 * name can be a real Spec field elsewhere, so a slug-scoped entry must not
 * leak. Each inner value is the reason the prop stays out of the portable
 * spec.
 */
export const WEB_ONLY_GLOBAL = "*";

export const WEB_ONLY: Record<string, Record<string, string>> = {
  [WEB_ONLY_GLOBAL]: {
    as: "rendered element / role; native has no equivalent",
    asRole: "rendered element / role; native has no equivalent",
    surface:
      "AudioMeter batched surface tier (spec 068 / g14.024): web rendering strategy only; native runtimes already batch meter nodes",
    channel:
      "AudioMeter batched surface tier (spec 068 / g14.024): web rendering strategy only; native runtimes already batch meter nodes",
    rightChannel:
      "AudioMeter batched surface tier (spec 068 / g14.024): web rendering strategy only; native runtimes already batch meter nodes",
    virtualized: "Tree virtual scroll; contract marks Svelte only",
    virtualHeight: "Tree virtual scroll; contract marks Svelte only",
    reorderAuthority:
      "g16.036 paired-web Tree authority adapter; native would need pending local Node commits and durable multi-row session payloads",
    initialFocus:
      "Dialog/FormDialog focus intent: auto/none is portable but the third form is a CSS selector (spec 063 IR-05). Remove when the IR rules on declarative focus intent.",
    continuationsResult:
      "HistoryCenter result feed (g14.007): web takes a reference-diffed prop; a native host holds the fork tree and hands the renderer a resolved view",
    runResult:
      "HistoryCenter result feed (g14.007): web takes a reference-diffed prop; a native host holds the fork tree and hands the renderer a resolved view",
    autocapitalize: "native HTML attribute; web runtimes forward it, portable spec does not (Runtime Parity Authority)",
    autocorrect: "native HTML attribute; web runtimes forward it, portable spec does not (Runtime Parity Authority)",
    autofocus: "native HTML attribute; web runtimes forward it, portable spec does not (Runtime Parity Authority)",
    spellcheck: "native HTML attribute; web runtimes forward it, portable spec does not (Runtime Parity Authority)",
    enterKeyHint: "native HTML attribute; web runtimes forward it, portable spec does not (Runtime Parity Authority)",
    class: "escape hatch into the host's styling",
    className: "escape hatch into the host's styling",
    style: "escape hatch into the host's styling",
    contentClassName: "escape hatch into the host's styling",
    contentStyle: "escape hatch into the host's styling",
    overlayClassName: "escape hatch into the host's styling",
    compressionOptions: "JS callback / options bag; not component semantics",
    controller: "JS callback / controller; not component semantics",
    debounce: "JS timing; not component semantics",
    parseDebounce: "JS timing; not component semantics",
    validationDebounce: "JS timing; not component semantics",
    resolveParseState: "JS callback; not component semantics",
    validate: "JS callback; not component semantics",
    validateOnBlur: "JS callback; not component semantics",
    validationContext: "JS callback / context; not component semantics",
    validationKey: "JS callback / key; not component semantics",
    describedBy: "ARIA wiring by DOM id; natives label by object, not id",
    form: "raw HTML form attribute",
    formaction: "raw HTML form attribute",
    formenctype: "raw HTML form attribute",
    formmethod: "raw HTML form attribute",
    formnovalidate: "raw HTML form attribute",
    formtarget: "raw HTML form attribute",
    type: "raw HTML attribute",
    list: "raw HTML attribute",
    id: "raw HTML attribute",
    name: "raw HTML attribute",
    leading: "snippet slot typed as a prop",
    trailing: "snippet slot typed as a prop",
    scrollOffset: "DOM-node scroll target",
    scrollTarget: "DOM-node scroll target",
    loadOptions:
      "g12.013: async options loader is behaviour, not data; native drives the same flow through is_loading plus options",
    native: "renders the platform <select> instead of the custom listbox; no native equivalent",
    crossWindowDragSource:
      "cross-window bridge is a host capability, not renderer-neutral component data",
    crossWindowDropTarget:
      "cross-window bridge is a host capability, not renderer-neutral component data",
    crossWindowSourceBridge:
      "cross-window bridge is a host capability, not renderer-neutral component data",
    element:
      "AppHeader bindable DOM node (g13-b014) for host-attached behaviour; GPUI/Jetstream own window dragging as an adapter capability",
  },
  "model-connection-card": {
    defaultOpen:
      "g15.008: native binding keeps the current value on the host; an uncontrolled seed has nothing to seed",
  },
  "model-connection-picker": {
    defaultQuery:
      "g15.008: native binding keeps the current value on the host; an uncontrolled seed has nothing to seed",
    defaultValue:
      "g15.008: native binding keeps the current value on the host; an uncontrolled seed has nothing to seed",
  },
  "model-connection-setup": {
    defaultStage:
      "g15.008: native binding keeps the current value on the host; an uncontrolled seed has nothing to seed",
    defaultValue:
      "g15.008: native binding keeps the current value on the host; an uncontrolled seed has nothing to seed",
  },
  "update-status": {
    observe:
      "g15.009: Svelte lazy-getter / React useSyncExternalStore; a native host rerenders with fresh props",
  },
  "update-center": {
    observe:
      "g15.009: Svelte lazy-getter / React useSyncExternalStore; a native host rerenders with fresh props",
  },
  "settings-shell": {
    page: "g15.009: web snippet; native hosts pass a composed Node into poodle_render::settings_shell, not a spec field",
  },
  slider: {
    formatVisibleValue:
      "g16.046: closures resolve to strings before the native spec; Spec carries visible_value_text instead",
  },
  "range-slider": {
    formatVisibleValue:
      "g16.046: closures resolve to strings before the native spec; Spec carries visible_*_text instead",
    formatVisibleRange:
      "g16.046: closures resolve to strings before the native spec; Spec carries visible_range_text instead",
  },
  tabs: {
    focusOnValueChange:
      "g16.060: controlled-panel focus transfer is a DOM adapter effect; native has no panel-unmount capture here",
  },
  text: {
    wrap: "operator ruling 2026-09-27: web-admitted; native admission pending (`lane:native-admission`)",
  },
  code: {
    wrap: "operator ruling 2026-09-27: web-admitted; native admission pending (`lane:native-admission`)",
  },
  "list-card": {
    eyebrow:
      "operator ruling 2026-09-27: web-admitted; native admission pending (`lane:native-admission`)",
  },
  pill: {
    dismissible:
      "operator ruling 2026-09-27: web-admitted Public Prop; native admission pending (`lane:native-admission`)",
    dismissLabel:
      "operator ruling 2026-09-27: web-admitted Public Prop; native admission pending (`lane:native-admission`)",
  },
  keyboard: {
    computerBaseNote:
      "planner ruling 2026-09-28: portable computer-key mapping; web-admitted until the headless keyboard machine gains it (`lane:native-admission`)",
  },
};

/** True when `prop` is a sanctioned web-only exemption for `slug`. */
export function isWebOnly(slug: string, prop: string): boolean {
  return WEB_ONLY[WEB_ONLY_GLOBAL]?.[prop] !== undefined || WEB_ONLY[slug]?.[prop] !== undefined;
}

/**
 * Real gaps: props the contract documents, Svelte implements, and the Spec does
 * not carry — so neither native target can render them. Tracked as debt in
 * task g12.013 (Git history), burned down there.
 *
 * This is a baseline, not an allowlist. Closing a gap means deleting its entry;
 * adding one means a prop shipped to the web without reaching the shared spec
 * surface, which is the thing this gate exists to stop.
 */
const OPEN_GAPS: Record<string, string[]> = {};

/**
 * Contract prop -> Spec field, where the two deliberately differ. The prop IS
 * carried; only the spelling moved.
 */
const ALIASES: Record<string, Record<string, string>> = {
  // The contract renamed `name` to `icon` and deprecated the old spelling; the
  // Spec still stores it as `name`, which 229 native call sites construct by.
  icon: { icon: "name" },
  // Collections keep a domain name on the Spec rather than the generic `items`.
  "card-radio-group": { items: "options" },
  tabs: { items: "tabs" },
  "toast-stack": { items: "toasts" },
  // The ternary state is one field, not a value/label pair.
  // The pair is stored as two scalars, which is what a thumb renderer wants.
  "range-slider": { value: "low" },
  // The pager stores the page it is on and the size of a page; `total` is the
  // item count, `limit` the page size.
  pagination: { page: "current_page", total: "total_items", limit: "page_size" },
  "pagination-summary": { currentPage: "page" },
  // `override` is a reserved word in Rust. A raw identifier would carry the
  // spelling at the cost of `r#override` at every call site.
  "agent-question": { override: "override_text" },
  // The spec's only placeholder is the add-input's, which is what the contract
  // names; a second field would be two names for one thing.
  "editable-list": { addPlaceholder: "placeholder" },
  // `kind` is the contract's deprecated name for the dialog's role.
  dialog: { kind: "role" },
  // The contract calls the code text `source`; the Spec calls it `content`.
  code: { source: "content" },
  // A custom accent is a colour string.
  pill: { accent: "accent_color" },
  // The contract's `options` record is decomposed into one field per state.
  "tri-state-switch": { value: "state", options: "excluded_label" },
  // The Spec names the instant it renders, not the HTML attribute that carries it.
  "time-ago": { datetime: "timestamp" },
  "block-editor": { blockTypeItems: "block_types" },
  // The Spec stores the bounds as resolved rem, which is what a renderer wants;
  // the contract states them as CSS strings.
  popover: { surfaceMinWidth: "surface_min_width_rem", surfaceMaxWidth: "surface_max_width_rem" },
  // IconProvider's web `icons` set is a name on the native spec — GPUI uses a
  // shared registry, so the spec records which set was requested, not the
  // SVG payload (g15.009).
  "icon-provider": { icons: "icon_set_name" },
};

/** Components with no Spec struct at all, with the reason. */
const NO_SPEC: Record<string, string> = {
  "error-boundary": "framework error boundary — no native equivalent",
  "toast-host": "imperative host, driven by the toast machine rather than a spec",
};

function snake(name: string): string {
  return name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
}

function contractProps(md: string): string[] {
  const start = md.indexOf("### Public Props");
  if (start < 0) return [];
  const rest = md.slice(start + "### Public Props".length);
  const end = rest.search(/\n#{2,4} /);
  const table = end < 0 ? rest : rest.slice(0, end);
  const props: string[] = [];
  for (const line of table.split("\n")) {
    const m = line.match(/^\|\s*`([a-zA-Z_$][\w$]*)`\s*\|/);
    if (m && !/^on[A-Z]/.test(m[1])) props.push(m[1]);
  }
  return props;
}

/** Every `pub struct` in the crate: name -> [field, resolvedTypeName][]. */
function collectStructs(): Map<string, Array<[string, string]>> {
  const structs = new Map<string, Array<[string, string]>>();
  const files = new Bun.Glob("**/*.rs").scanSync({ cwd: specsDir, absolute: true });
  const re = /pub struct\s+(\w+)\s*\{([\s\S]*?)\n\}/g;

  for (const file of files) {
    const src = readFileSync(file, "utf8");
    let m: RegExpExecArray | null;
    while ((m = re.exec(src)) !== null) {
      const fields: Array<[string, string]> = [];
      for (const line of m[2].split("\n")) {
        const f = line.match(/^\s*pub\s+([a-z_][a-z0-9_]*)\s*:\s*(.+?),?\s*$/);
        if (f) fields.push([f[1], bareType(f[2])]);
      }
      structs.set(m[1], fields);
    }
  }
  return structs;
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

/**
 * Field names reachable from a struct, following composition.
 *
 * Specs delegate: `ContextMenuSpec` holds a `MenuSpec`, so the contract's
 * `items` prop is carried one level down. A checker that only looked at the
 * top level would report a gap that is not there.
 */
function reachableFields(root: string, structs: Map<string, Array<[string, string]>>): Set<string> {
  const fields = new Set<string>();
  const seen = new Set<string>();
  const queue = [root];

  while (queue.length > 0) {
    const name = queue.pop()!;
    if (seen.has(name)) continue;
    seen.add(name);
    for (const [field, ty] of structs.get(name) ?? []) {
      fields.add(field);
      if (structs.has(ty)) queue.push(ty);
    }
  }
  return fields;
}

/** True when the Spec carries this contract prop under any accepted spelling. */
function covered(prop: string, fields: Set<string>): boolean {
  const s = snake(prop);
  const variants = [
    s,
    `is_${s}`,
    `has_${s}`,
    // Only a prop that already reads as a "show" toggle may match the Rust
    // `show_` spelling. Without this guard `seconds` matched `show_seconds`,
    // reporting a scalar value prop as covered by an unrelated boolean.
    //
    // There were plural/singular variants here too (`items` <-> `item`). They
    // matched nothing once the real gaps closed, and a rule that covers no
    // case but can still fire is only a way to hide the next one.
    ...(s.startsWith("show_") ? [s.replace(/^show_/, ""), s.replace(/^show_/, "shows_")] : []),
  ];
  return variants.some((v) => fields.has(v));
}

export type SpecDriftFinding = { slug: string; missing: string[] };

export function contractSpecDrift(): {
  checked: number;
  skipped: number;
  findings: SpecDriftFinding[];
} {
  const findings: SpecDriftFinding[] = [];
  const structs = collectStructs();
  let checked = 0;
  let skipped = 0;

  for (const entry of allComponents) {
    if (entry.slug in NO_SPEC) {
      skipped++;
      continue;
    }
    const contractPath = path.join(contractsDir, `${entry.slug}.md`);
    // A spec may be a single file or a module directory. Resolving only the
    // flat form meant `TreeSpec` — which lives in `tree/mod.rs` — was skipped
    // silently for as long as it has existed, so a Tree prop could be
    // documented without ever reaching the spec and nothing would say so.
    const specName = snake(entry.displayName);
    const specPath = [
      path.join(specsDir, `${specName}.rs`),
      path.join(specsDir, specName, "mod.rs"),
    ].find(existsSync);
    if (!existsSync(contractPath) || specPath === undefined) {
      skipped++;
      continue;
    }
    const props = contractProps(readFileSync(contractPath, "utf8"));
    if (props.length === 0) {
      skipped++;
      continue;
    }
    checked++;

    const fields = reachableFields(`${entry.displayName}Spec`, structs);
    const allow = OPEN_GAPS[entry.slug] ?? [];
    const aliases = ALIASES[entry.slug] ?? {};
    const missing = props
      .filter(
        (p) =>
          !isWebOnly(entry.slug, p) &&
          !covered(p, fields) &&
          !(aliases[p] && fields.has(aliases[p])) &&
          !allow.includes(p),
      )
      .sort();
    if (missing.length > 0) findings.push({ slug: entry.slug, missing });
  }

  return { checked, skipped, findings };
}

export function contractSpecDriftErrors(): string[] {
  return contractSpecDrift().findings.map(
    (f) =>
      `contract/spec drift: ${f.slug}.md documents prop(s) absent from its poodle-specs Spec: ${f.missing.join(", ")}`,
  );
}

if (import.meta.main) {
  const { checked, skipped, findings } = contractSpecDrift();
  console.log(`contract-spec-drift: checked ${checked}, skipped ${skipped} (no contract/spec/props)\n`);
  if (findings.length > 0) {
    const n = findings.reduce((a, f) => a + f.missing.length, 0);
    console.log(`${n} documented prop(s) missing from poodle-specs across ${findings.length} component(s):`);
    for (const f of findings) console.log(`  [${f.slug}] ${f.missing.join(", ")}`);
    console.log("");
  } else {
    console.log("OK — every documented public prop reaches poodle-specs.");
  }
  if (findings.length > 0 && process.env.DRIFT_REPORT !== "1") process.exit(1);
}
