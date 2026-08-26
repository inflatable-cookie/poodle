# g16.002 — Selection Controls Mounted Parity

Date: 2026-08-26
Status: four proofs complete; ToggleGroup planning stop
Branch: `t3code/selection-controls-mounted-parity`
Card: `docs/roadmaps/g16/002-selection-controls-mounted-parity.md`
PR: #76

## Outcome

Four named mounted GPUI regressions drive Checkbox, Switch, RadioGroup, and
SegmentedControl through the real node/backend/input path and host rebuild.
Direct handler calls do not count for the mounted claim.

The generated ledger moves only those four GPUI mounted-behaviour rows from
`missing` to `mounted`. Summary: 29 → 33 mounted; 145 → 141 missing. ToggleGroup
stays `missing`. GPUI accessibility stays `manual`. No contract, public spec,
or public API change.

## Mounted tests

`packages/gpui/preview/tests/headless_regressions.rs`:

- `checkbox_toggle_readonly_and_disabled_rebuild_the_host_spec` — mixed
  resolves to checked on the first accepted activation; then toggles; readonly
  stays focusable and emits nothing; disabled has no focus handle and emits
  nothing.
- `switch_toggle_readonly_and_disabled_rebuild_the_host_spec` — pointer then
  Enter toggle with host rebuild; readonly and disabled match Checkbox.
- `radio_group_exclusive_focus_and_disabled_paths_through_mounted_input` —
  exclusive selection, same-value inertia, disabled-option skip, wrap, and
  disabled-group inertia.
- `segmented_control_exclusive_focus_identity_and_disabled_paths` — exclusive
  selection, same-value inertia, wrap/skip, disabled-group inertia, and two
  mounted instances with independent focus identity.

Disabled focus assertions use unique ids/scopes. GPUI `FOCUS_HANDLES` is
thread-local across `run_headless` blocks in one test, so reusing
`FIXTURE_ID` / `radio:plan:…` would falsely see a prior handle.

## Defects and repairs

GPUI tracks a focus handle only when a node is focusable and declares a focus
style patch (`packages/render/src/radio.rs`). Without that patch, enabled
Checkbox/Switch/RadioGroup/ToggleGroup items were unfocusable in the mounted
tree and disabled “no handle” assertions were unobservable.

- Checkbox: focus patch on enabled roots (readonly included); `a11y.toggled`
  from `current_state()` so mixed is Mixed, not `checked: None`.
- Switch: same focus-patch rule; `a11y.toggled` from `current_checked()` so
  uncontrolled is not Mixed.
- RadioGroup: option ids `radio:{name}:option:{value}` (`name` defaults to
  `"group"`); RadioButton a11y; roving `tab_index`; wrap and skip disabled via
  `on_key`; same-value `on_activate` omitted.
- SegmentedControl: same-value activation no longer fires; arrow wrap that
  lands on self does not emit. The deferred Jetstream re-pick note is not
  active-cohort authority; the contract’s same-value inertia stands.
- ToggleGroup: enabled items gained the same focus patch so GPUI can track
  handles. That does not close the mounted cell.

Shared-Rust unit tests cover mixed→checked, readonly, disabled, RadioGroup
same-value/arrows/disabled-group, and SegmentedControl same-value inertia.

## ToggleGroup planning stop

An earlier mounted ToggleGroup regression was withdrawn after review. It
called `ToggleGroupSpec::next_value_on_toggle` inside the host callback, so it
proved host reconstruction rather than the contracted native payload. The
renderer still exposes `Fn(&str)` and reports the activated option; the
contract requires `string | string[] | null` containing the resulting
selection. Changing that signature is a public Rust API decision.

The same proof never drove contracted single-mode Arrow Left/Right roving.
The Keyboard table names that behaviour; the machine says buttons provide
focus with no roving tabindex. Svelte and React omit it. Inventing a
GPUI-only rule would also cross this card.

Both are stop conditions. ToggleGroup stays `missing` for the orchestrator’s
separate semantic/API lane. No `g16.003`.

## Validation

Passed on the four-proof revision, entirely headless:

- focused `poodle-render` tests for the changed modules
- four named mounted tests in `headless_regressions.rs`
- `effigy regressions:native` — 74
- `effigy test:parity-evidence-ledger`
- `effigy check:parity-evidence-ledger`
- `git diff --check`

The earlier five-cell closeout also passed `effigy probe:gpui-specimens`,
`effigy ci:native`, `effigy ci:web`, `effigy docs:check`, and `effigy qa`.
This revision does not re-run that full board unless the four-proof checks
fail. No `*-windowed`, native visual, Jetstream, or release selectors.
