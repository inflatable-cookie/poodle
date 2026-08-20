# g15.038 — SegmentedControl native option parity

Date: 2026-08-20
Card: `docs/roadmaps/g15/038-segmented-control-native-option-parity.md`
Handoff: `docs/handoffs/20260820-162916-g15-038-segmented-control-native-option-parity.md`
PR: #52

## Outcome

Shared Rust no longer reuses `ChoiceOption` for SegmentedControl. A dedicated
public `SegmentedControlOption` carries every portable contract field: value,
label, optional icon name, icon-only, disabled, optional accessible label, and
optional title. Shared rendering emits labelled icons and icon-only segments
with the contracted accessible-name, tooltip, supporting-visual icon size,
icon/text gap, and compact square geometry. GPUI teaches the contract's
Effects/Instruments icon-only example with live selection.

This is a **breaking, pre-1.0, operator-approved** public Rust API migration.
No alias, `From<ChoiceOption>` conversion, overloaded constructor, deprecated
twin, or silent fallback remains. `ChoiceOption` is unchanged for Select,
RadioGroup, CardRadioGroup, and every other family.

## Change class

- **Packages changed:** `poodle-specs`, `poodle-node`, `poodle-render`,
  `poodle-gpui-node-backend`, `@inflatable-cookie/poodle-core`, internal
  GPUI/Jetstream previews, in-repo SegmentedControl callers
- **Public-intent entry points:** `SegmentedControlOption` (new);
  `SegmentedControlSpec::new(instance_id, options)` now requires a stable
  native instance scope and takes `Vec<SegmentedControlOption>` instead of
  `Vec<ChoiceOption>`; shared `color_picker`, `filter_builder`, and
  `model_picker` render entry points now require their parent instance scope;
  additive `Node.tooltip`; additive
  default-icon names `audioWaveform` and `piano` on
  `@inflatable-cookie/poodle-core/icons`
- **Compatibility:** clean break on the option type and native constructor /
  composed-render signatures, pre-1.0; operator approved 2026-08-20. Icon
  names and `Node.tooltip` are additive.
- **Downstream re-check:** any out-of-repo `SegmentedControlSpec` constructor
  that passed `ChoiceOption` must construct `SegmentedControlOption` instead
  and supply a lifetime-stable instance scope. Direct shared-render callers
  for ColorPicker, FilterBuilder, or ModelPicker must supply their parent
  scope; nested SegmentedControl scopes are derived from it.
  Apps that import the default Lucide set gain `audioWaveform` and `piano`.

## Implementation

- Dedicated option type with `is_icon_only()`, `accessible_name_override()`,
  and `tooltip_text()` matching Svelte: hide the visible label only when
  `iconOnly && icon`; accessible name is explicit `aria_label` then required
  `label` for icon-only; tooltip is explicit `title` then required `label`
  for icon-only.
- Renderer: named `Node::icon` at the supporting-visual `IconSize` stop,
  icon before text, `0.375rem` gap, square width when `equal_width=false`.
  Icon-only requested without an icon keeps the visible label.
- Smallest reusable tooltip field: `Node.tooltip: Option<String>`. GPUI
  projects it through the engine's native `.tooltip()` onto a text view.
  Not the Tooltip overlay component and not a new overlay architecture.
- GPUI specimen: live "Icon-only options" section, default `effects`,
  `equalWidth=false`, `size=sm`, group label `Plugin kind`.

## Evidence

- spec: `SegmentedControlOption` carries every portable field; icon-only
  without an icon is not icon-only
- render: icon disappearance, visible-label suppression, accessible
  fallback, explicit aria/title precedence, square geometry, labelled-icon
  order/size/gap, and icon-only activation
- backend: a tooltip forces the stateful path GPUI needs for `.tooltip()`
- specimen: activating Instruments records `segmented-icon-value`

## Audit and register

- `specimen-catalogue-audit.md` revision 6: SegmentedControl returns to
  GPUI A / `keep`. Totals: GPUI 100 A / 68 B; worst-of-three 58 A / 48 B;
  keep 56; contract/runtime-blocker 0.
- `release-gap-register.md`: SegmentedControl option presentation row
  closed by this card.
- `segmented-control.md` Known Deltas: the provisional native icon row is
  removed.

## Validation

Headless only. No windowed, native-visual, Jetstream, or release selector ran.

- focused spec test `segmented_control_option_carries_the_contract_icon_surface`
- focused render tests (25, including icon-only suppression, fallback,
  geometry, labelled-icon order/size/gap, activation, stable explicit focus
  identity, and repeated-composition isolation)
- focused backend test `tooltip_forces_element_state`
- core icon-catalogue boundary updated for the two admitted default assets
- GPUI specimen test `icon_only_activation_records_the_picked_value`
- `effigy ci:rust`
- `effigy check:gpui` (353 render tests, 22 backend tests, preview check)
- `effigy regressions:native` (50 passed)
- `effigy probe:gpui-specimens` (7 passed)
- `effigy docs:check`
- `effigy qa`
- `git diff --check` clean on the working tree

## Review round 1 (PR #52)

Two HIGH findings, both addressed on this branch:

1. Segments now emit `RadioButton` role, selected/toggled state, roving
   `tab_index`, and `interaction.disabled`. Arrow/Home/End move through
   enabled options and skip disabled ones. The old "disabled option stays
   focusable and undimmed" test now asserts the contract.
2. `audio-waveform` and `piano` are in the default Lucide set, with SVG
   existence evidence.

## Review round 2 (PR #52)

1. Each control takes an `instance_id`. Segment `runtime_id`s are scoped
   (`segmented:{scope}:option:{value}`). Enabled segments carry a focus-ring
   patch so GPUI `tracks_focus` creates retrievable handles. Roving keys
   return that scoped id. Multi-instance evidence checks that `a` and `b`
   cannot steal each other's focus keys.
2. Global icon-name rewriting was removed. Shared render and the GPUI
   backend pass the node name through to the app-owned asset source. Custom
   names such as `company-logo` stay intact. No generic native fallback.
3. Change-class now records the additive `poodle-core` default-icon exports.

## Review round 3 (PR #52)

The first repair made focus identity mandatory but generated missing scopes
from a per-frame render-order counter. Orchestrator review rejected it: when a
preceding conditional control disappeared, a persistent control changed from
`auto-1` to `auto-0`, invalidating GPUI focus handles. That implementation and
its reset API were removed.

## Review round 4 (PR #52)

Native identity is explicit and lifetime-stable.
`SegmentedControlSpec::new(instance_id, options)` requires the scope; shared
render always emits `segmented:{scope}:option:{value}` and contains no global
counter or reset lifecycle. ColorPicker, FilterBuilder, and ModelPicker take a
parent scope and derive `:mode`, `:boolean:{field}`, and `:axis:{axis}` child
scopes. All GPUI and deferred Jetstream compile callers now supply stable
scopes. Regression evidence proves a persistent control keeps its runtime id
when a preceding control disappears, while repeated composed controls remain
isolated.

## Unresolved

- Jetstream remains program-deferred. Its specimen constructors were
  migrated to the dedicated type so the crate compiles; no Jetstream
  icon-only teaching page was added.
- Card/front-door status, merge, and `g15.029` promotion stay with the
  orchestrator.
