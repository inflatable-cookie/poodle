# g16.003 — RadioGroup Native Identity And Mounted Parity

Date: 2026-08-26
Status: complete
Branch: `t3code/radio-group-native-identity`
Card: `docs/roadmaps/g16/003-radio-group-native-identity-and-mounted-parity.md`

## Outcome

`poodle_render::radio_group` now takes required `RadioGroupHandlers::new(instance_id)`
with optional `on_change`. Option semantic ids stay `radio:{value}`; backend
runtime ids are `radio:{instance_id}:option:{value}`. Vertical groups answer
Arrow Up/Down; horizontal groups answer Arrow Left/Right; the other axis is
inert. Wrap skips disabled options. Same-value pointer selection emits nothing.

The generated ledger moves only RadioGroup's GPUI mounted-behaviour cell from
`missing` to `mounted`. Summary: 32 → 33 mounted; 142 → 141 missing. GPUI
accessibility stays `manual`. Svelte and React are unchanged. ToggleGroup is
not in this lane.

## Handler and call sites

- `packages/render/src/radio_group.rs` — `RadioGroupHandlers`; scoped ids;
  roving tab-stop, orientation-aware arrows, focus patches
- `packages/render/src/lib.rs` — export
- `packages/gpui/preview/src/node_compat.rs` — `RadioGroup::from_spec(spec, theme, instance_id)`
- `packages/gpui/preview/src/specimens/radio_group.rs` — stable descriptive
  scopes (`radio-plan`, `radio-size`, `radio-disabled`,
  `radio-disabled-option`, `radio-custom-color`, size/density specimen ids)
- `packages/jetstream/preview/src/compat.rs` and
  `packages/jetstream/preview/src/specimens/radio_group.rs` — compile-only
  scopes; no backend-behavior change

## Mounted tests

`packages/gpui/preview/tests/headless_regressions.rs`:

- `radio_group_exclusive_focus_identity_and_disabled_paths` — pointer
  selection with host rebuild; same-value inertia; disabled-option skip;
  vertical wrap; unrelated-axis inertia; horizontal wrap; disabled-group
  inertia; two mounted groups with identical option values and independent
  focus handles (`radio:left:option:free` vs `radio:right:option:free`)

## Remaining gaps

- ToggleGroup still `missing` on callback payload, roving focus, and item
  identity. Next task is orchestrator review, not compiling that card.
- GPUI accessibility remains `manual`.
- Jetstream preview was not compiled in this worktree: the crate's
  `jetstream-input` path is absent here. The call-site change is mechanical
  and compile-only.

## Validation

Passed on this revision, entirely headless:

- focused `poodle-render` RadioGroup tests — 10
- `radio_group_exclusive_focus_identity_and_disabled_paths`
- `effigy regressions:native` — 74
- `effigy probe:gpui-specimens`
- `effigy test:parity-evidence-ledger`
- `effigy check:parity-evidence-ledger`
- `effigy ci:native`
- `effigy ci:web`
- `effigy docs:check`
- `effigy qa`
- `git diff --check origin/main...HEAD`

No `*-windowed`, native visual, Jetstream preview/QA, release, tag, or
publication selectors. `effigy doctor` stayed red on the known main-base
oversized-file and broad-suppression scans; that debt was not absorbed.
