# g16.002 — Selection Controls Mounted Parity

Date: 2026-08-26
Status: complete; awaiting orchestrator review
Branch: `t3code/selection-controls-mounted-parity`
Card: `docs/roadmaps/g16/002-selection-controls-mounted-parity.md`

## Outcome

Five named mounted GPUI regressions now drive Checkbox, Switch, RadioGroup,
SegmentedControl, and ToggleGroup through the real node/backend/input path and
host rebuild. Direct handler calls do not count for the mounted claim.

The generated ledger moves only those five GPUI mounted-behaviour rows from
`missing` to `mounted`. Summary: 29 → 34 mounted; 145 → 140 missing. GPUI
accessibility stays `manual`. No contract, public spec, or public API change.

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
- `toggle_group_single_multiple_and_disabled_payloads_through_mounted_input`
  — single re-select emits the unchanged value; `allowDeactivation` emits
  empty/`null`; multiple adds then removes membership in order; option- and
  group-disabled emit nothing. Native `on_change` still emits the activated
  option (`Fn(&str)`); the host applies `ToggleGroupSpec::next_value_on_toggle`
  and rebuilds.

Disabled focus assertions use unique ids/scopes. GPUI `FOCUS_HANDLES` is
thread-local across `run_headless` blocks in one test, so reusing
`FIXTURE_ID` / `radio:plan:…` / `toggle:list` would falsely see a prior
handle.

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
- ToggleGroup: focus patch on enabled items only. Public handler signature
  stays `Fn(&str)`.

Shared-Rust unit tests cover mixed→checked, readonly, disabled, RadioGroup
same-value/arrows/disabled-group, and SegmentedControl same-value inertia.

## Unresolved gaps

- ToggleGroup’s Keyboard table names Arrow Left/Right in single mode; the
  machine says buttons provide focus with no roving tabindex. Svelte and GPUI
  both omit arrow roving. This card recorded the gap rather than inventing a
  GPUI-only rule.
- GPUI accessibility remains `manual`. Node role/state assertions support the
  mounted behaviour claim; they are not assistive-technology proof.
- No visual fixtures. No `g16.003`.

## Validation

Passed, entirely headless:

- `cargo test --manifest-path packages/render/Cargo.toml --lib` — 404
- five named mounted tests in `headless_regressions.rs`
- `effigy regressions:native` — 75
- `effigy probe:gpui-specimens` — 8
- `effigy test:parity-evidence-ledger` — 5
- `effigy check:parity-evidence-ledger`
- `effigy ci:native`
- `effigy ci:web`
- `effigy docs:check`
- `effigy qa`
- `git diff --check`

`ci:web` first failed because this worktree had no `node_modules` and bun
resolved `lucide-static` 1.34.0 from `~/.bun/install/cache`. `bun install`
restored lockfile 1.31.0; the rerun passed. Recorded in `PAPERCUTS.md`.
No `*-windowed`, native visual, Jetstream, or release selectors.
