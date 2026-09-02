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

import { allComponents } from "../src/component-registry.ts";

const repoRoot = path.resolve(import.meta.dir, "../../../..");
const contractsDir = path.join(repoRoot, "docs/contracts/components");
const specsDir = path.join(repoRoot, "packages/contracts/components/src");

/**
 * Props that never reach a Spec by design — they are web-platform plumbing, not
 * component semantics, and a native target has no use for them:
 *
 *   - escape hatches into the host's styling: `className`, `class`, `style`,
 *     `contentClassName`, `contentStyle`, `overlayClassName`
 *   - raw HTML attributes: `type`, `form*`, `list`, `id`, `name`, `spellcheck`,
 *     `autocapitalize`, `autocorrect`, `enterKeyHint`
 *   - ARIA wiring by DOM id: `describedBy` (natives label by object, not id)
 *   - the rendered element / role: `as`, `asRole`
 *   - JS callbacks and timings: `validate*`, `debounce`, `parseDebounce`,
 *     `resolveParseState`, `controller`, `compressionOptions`
 *   - DOM-node scroll targets: `scrollTarget`, `scrollOffset`
 *   - snippet slots typed as props: `leading`, `trailing`
 */
const WEB_ONLY_PROPS = new Set([
  "as",
  // AudioMeter batched surface tier (spec 068 / g14.024): a web rendering
  // strategy only. Native runtimes already batch meter nodes in their renderer
  // scene, so `MeterSurface` has no Rust spec by fixed decision and these
  // props are marked **Web targets only** in the contract.
  "surface",
  "channel",
  "rightChannel",
  // Tree virtual scroll: the contract marks both **Svelte only** in its own
  // props table. Surfaced the moment the gate learned to resolve module specs
  // — `TreeSpec` lives in `tree/mod.rs` and had never been checked.
  "virtualized",
  "virtualHeight",
  // g16.036 paired-web Tree authority adapter. Native would require pending
  // local Node commits and durable multi-row session payloads; this card does
  // not fake those through TreeSpec.
  "reorderAuthority",
  // Dialog/FormDialog initial-focus intent. Web-only *for now*, by decision:
  // the "auto"/"none" policy is portable but the third form is a CSS selector,
  // which cannot cross to native. Rather than design portable focus-intent
  // semantics for one component ad hoc, this defers to the g13 IR, which owns
  // the focus/adapter-capability boundary (spec 063 IR-05; corpus CROSS-17,
  // NEG-03). Remove this entry when the IR rules on declarative focus intent.
  "initialFocus",
  // HistoryCenter's two result feeds (g14.007). The portable claim is that
  // the host answers `loadContinuations` / `loadContinuationRun` and the
  // answer reaches the picker; the conformance corpus asserts exactly that on
  // Svelte, React and GPUI. *How* the answer arrives is shell mechanism: the
  // web shells take it as a reference-diffed prop because that suits
  // data-down flow, while a native host holds the fork tree in its own state
  // and hands the renderer a resolved view. Same boundary as TextInput's DOM
  // vs GPUI editing paths. Marked as a known delta in the contract.
  "continuationsResult",
  "runResult",
  "asRole",
  "autocapitalize",
  "autocorrect",
  // Native input attribute, same class as autocapitalize/autocorrect/spellcheck
  // above: web runtimes forward it to the element, and it stays out of the
  // portable spec (001-working-rules.md, Runtime Parity Authority).
  "autofocus",
  "class",
  "className",
  "compressionOptions",
  "contentClassName",
  "contentStyle",
  "controller",
  "debounce",
  "describedBy",
  "enterKeyHint",
  "form",
  "formaction",
  "formenctype",
  "formmethod",
  "formnovalidate",
  "formtarget",
  "id",
  "leading",
  "list",
  "name",
  "overlayClassName",
  "parseDebounce",
  "resolveParseState",
  "scrollOffset",
  "scrollTarget",
  "spellcheck",
  "style",
  "trailing",
  "type",
  "validate",
  "validateOnBlur",
  "validationContext",
  "validationDebounce",
  "validationKey",
  // Decided in g12.013: an async options loader is behaviour, not data — a
  // native target drives the same flow through `is_loading` plus `options`.
  "loadOptions",
  // Renders the platform `<select>` instead of the custom listbox. There is no
  // native equivalent to defer to, so the flag has nothing to mean off the web.
  "native",
  // Cross-window bridges are host capabilities rather than renderer-neutral
  // component data. Native hosts own equivalent traits at their window/source
  // integration boundary; copying trait objects into Specs would make host
  // authority look serializable.
  "crossWindowDragSource",
  "crossWindowDropTarget",
  "crossWindowSourceBridge",
  // AppHeader's bindable `element` escape hatch (g13-b014). Exposes the raw
  // `<header>` DOM node for host-attached behaviour (e.g. window dragging);
  // GPUI/Jetstream own window dragging as an adapter capability and have no
  // element to hand out. The React counterpart is `ref`, documented in prose
  // (React's own mechanism, not a member of AppHeaderProps).
  "element",
]);

/**
 * Web-only props scoped to one component, for cases where the same prop name
 * is a real spec field elsewhere.
 *
 * `WEB_ONLY_PROPS` above is global, so putting `defaultValue` in it would
 * exempt the ~20 components that legitimately carry `default_value` and hide
 * the next one that drops it. These entries exempt exactly one component each,
 * and every one is marked **Web targets only** in its own props table.
 */
const WEB_ONLY_BY_SLUG: Record<string, string[]> = {
  // Model-connection family (g15.008). The native binding keeps the current
  // value on the host: GPUI/AppState owns stage/value/query/open and rerenders
  // after a callback requests a change, so an uncontrolled seed has nothing to
  // seed. Stated in each contract's Native Binding note and in
  // `docs/roadmaps/g15/008-model-connection-family-native-completion.md`.
  "model-connection-card": ["defaultOpen"],
  "model-connection-picker": ["defaultQuery", "defaultValue"],
  "model-connection-setup": ["defaultStage", "defaultValue"],
  // Update family (g15.009). `observe` is a Svelte lazy-getter /
  // React `useSyncExternalStore` subscription; a native host rerenders with
  // fresh props. SettingsShell's `page` is a web snippet; native hosts pass a
  // composed Node into `poodle_render::settings_shell`, not a spec field.
  "update-status": ["observe"],
  "update-center": ["observe"],
  "settings-shell": ["page"],
  // g16.046. Closures resolve to strings before the native spec; the Spec
  // carries `visible_value_text` / `visible_lower_text` / `visible_upper_text`
  // / `visible_range_text` instead of the functions.
  slider: ["formatVisibleValue"],
  "range-slider": ["formatVisibleValue", "formatVisibleRange"],
  // g16.060. Controlled-panel focus transfer is a DOM adapter effect.
  // Native has no panel-unmount capture in this bounded consumer unblock.
  tabs: ["focusOnValueChange"],
};

/**
 * Real gaps: props the contract documents, Svelte implements, and the Spec does
 * not carry — so neither native target can render them. Tracked as debt in
 * `docs/roadmaps/g12/013-native-spec-surface-parity.md`, burned down there.
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
    const webOnly = new Set(WEB_ONLY_BY_SLUG[entry.slug] ?? []);
    const aliases = ALIASES[entry.slug] ?? {};
    const missing = props
      .filter(
        (p) =>
          !WEB_ONLY_PROPS.has(p) &&
          !webOnly.has(p) &&
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
