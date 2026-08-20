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
  `poodle-gpui-node-backend`, internal GPUI/Jetstream previews, in-repo
  SegmentedControl callers
- **Public-intent entry points:** `SegmentedControlOption` (new);
  `SegmentedControlSpec::new` / `options` now take
  `Vec<SegmentedControlOption>` instead of `Vec<ChoiceOption>`; additive
  `Node.tooltip`
- **Compatibility:** clean break, pre-1.0; operator approved 2026-08-20
- **Downstream re-check:** any out-of-repo `SegmentedControlSpec` constructor
  that passed `ChoiceOption` must construct `SegmentedControlOption` instead

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
- focused render tests (16, including icon-only suppression, fallback,
  geometry, labelled-icon order/size/gap, and activation)
- focused backend test `tooltip_forces_element_state`
- GPUI specimen test `icon_only_activation_records_the_picked_value`
- `effigy ci:rust`
- `effigy check:gpui` (342 render tests, 21 backend tests, preview check)
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
2. `audio-waveform` and `piano` are in the default Lucide set. Shared
   resolve maps aliases and unknown names to `circle-x`; GPUI paints the
   resolved asset. Evidence checks the SVG file exists, not just the node.

## Unresolved

- Jetstream remains program-deferred. Its specimen constructors were
  migrated to the dedicated type so the crate compiles; no Jetstream
  icon-only teaching page was added.
- Card/front-door status, merge, and `g15.029` promotion stay with the
  orchestrator.
