# g16.002 — Selection Controls Mounted Parity

Date: 2026-08-26
Status: closed — partial outcome
Branch: `t3code/selection-controls-mounted-parity`
Card: `docs/roadmaps/g16/002-selection-controls-mounted-parity.md`
PR: #76

## Outcome

Three named mounted GPUI regressions drive Checkbox, Switch, and
SegmentedControl through the real node/backend/input path and host rebuild.
The generated ledger moves only those three GPUI mounted-behaviour rows from
`missing` to `mounted`. Summary: 29 → 32 mounted; 145 → 142 missing. RadioGroup
and ToggleGroup stay `missing`. GPUI accessibility stays `manual`. No contract,
public spec, or public API change.

The exact five-control goal was not completed.

## Mounted tests

`packages/gpui/preview/tests/headless_regressions.rs`:

- `checkbox_toggle_readonly_and_disabled_rebuild_the_host_spec` — mixed
  resolves to checked on the first accepted activation; then toggles; readonly
  stays focusable and emits nothing; disabled has no focus handle and emits
  nothing.
- `switch_toggle_readonly_and_disabled_rebuild_the_host_spec` — pointer then
  Enter toggle with host rebuild; readonly and disabled match Checkbox.
- `segmented_control_exclusive_focus_identity_and_disabled_paths` — exclusive
  selection, same-value inertia, wrap/skip, disabled-group inertia, and two
  mounted instances with independent focus identity.

Disabled focus assertions use unique ids/scopes. GPUI `FOCUS_HANDLES` is
thread-local across `run_headless` blocks in one test, so reusing
`FIXTURE_ID` would falsely see a prior handle.

## Defects and repairs

GPUI tracks a focus handle only when a node is focusable and declares a focus
style patch (`packages/render/src/radio.rs`). Without that patch, enabled
Checkbox/Switch items were unfocusable in the mounted tree.

- Checkbox: focus patch on enabled roots (readonly included); `a11y.toggled`
  from `current_state()` so mixed is Mixed, not `checked: None`.
- Switch: same focus-patch rule; `a11y.toggled` from `current_checked()` so
  uncontrolled is not Mixed.
- SegmentedControl: same-value activation no longer fires; arrow wrap that
  lands on self does not emit. Instance scope comes from the required name on
  the public spec, so two mounted controls keep independent focus identity.
- RadioGroup: same-value activation is inert, matching native radios. No new
  option identity was added.

## Planning stops

RadioGroup: the contract keeps `name` optional and auto-generates a unique
group name on web. The public Rust spec has no other instance identity. A
renderer fallback of `"group"` would make two unnamed groups share GPUI focus
handles. A module-level counter would change across host rebuilds. This needs
a public spec/identity decision.

ToggleGroup: native still emits the activated option as `Fn(&str)`; the
contract requires `string | string[] | null` containing the resulting
selection. Contracted single-mode Arrow Left/Right is named in the Keyboard
table and omitted by the machine, Svelte, and React. Item ids remain
`toggle:<value>`, so a focus patch would collide across instances. Keep that
repair with the later semantic/API/identity lane.

No `g16.003` from this partial close.

## Validation

Passed on this revision, entirely headless:

- focused `poodle-render` tests for the changed modules
- three named mounted tests in `headless_regressions.rs`
- `effigy regressions:native` — 73
- `effigy test:parity-evidence-ledger`
- `effigy check:parity-evidence-ledger`
- `git diff --check`

No `*-windowed`, native visual, Jetstream, or release selectors.
