# g16.013 — TriStateSwitch Contract And Mounted Parity

Date: 2026-08-27
Status: complete — review follow-up on `t3code/tri-state-switch`
Branch: `t3code/tri-state-switch`
Card: `docs/roadmaps/g16/013-tri-state-switch-contract-and-mounted-parity.md`
Source triage: `docs/triage/20260827-160028-post-g16-012-native-lane-decision.md`

## Outcome

Shared Rust TriStateSwitch stored legacy `CheckState`, defaulted to excluded,
exposed undocumented `label`, made every segment focusable, re-fired on
same-value activation, and stamped every GPUI instance with one root id. The
contract now stores `TriStateValue`, defaults to `Default`, requires
`TriStateSwitchHandlers::new(instance_id)`, and the renderer exposes one roving
tab stop, structured focus rings, same-value inertia, and Left/Right wrap with
focus movement matching the web `singleSelectTransition` authority.

The generated ledger moves only TriStateSwitch's GPUI mounted-behaviour cell:
`missing` → `mounted` (41 → 42 mounted, 133 → 132 missing). Known-delta totals
stay 115 present / 60 not-applicable.

## Approved break

- `TriStateSwitchSpec.state: CheckState` → `value: TriStateValue`
- `with_state(...)` → `with_value(...)` with no alias or fallback
- default Excluded → Default
- removed undocumented `label` / `with_label` and CheckState conversion helpers
- Poodle GPUI and deferred Jetstream call sites migrated directly

## Mounted evidence

`packages/gpui/preview/tests/headless_regressions.rs#tri_state_switch_value_focus_identity_and_disabled_paths`
proves, through production hit testing, focus, and key dispatch:

- labelled `RadioGroup` and three labelled `RadioButton` nodes, selected/toggled
  state, exactly one `tab_index=0`, and structured focus rings on every enabled
  segment
- real pointer targets register backend bounds before `pointer_activate_id`
- Space on a focused unselected production target emits and rebuilds; Space on
  the selected segment is inert
- pointer and Left/Right selection with host rebuild, wrap, and requested focus
- same-value pointer activation is inert
- a disabled control between real tab stops is skipped by sequential focus and
  registers no focus handle
- after rebuilding one of two same-valued instances, both scoped identities and
  focus handles remain independent

## Explicit non-claims

- no Svelte/React public prop or behavior change
- no Jetstream admission or behavioral repair
- no visual comparison or broad native accessibility proof
- no NumberInput, EditableLabel, Accordion, or sibling component work

## Validation

Focused `poodle-specs` and `poodle-render` TriStateSwitch tests, Svelte and
React TriStateSwitch tests (unchanged), named mounted regression,
`effigy regressions:native`, `effigy probe:gpui-specimens`, `effigy drift:handlers`,
`effigy drift:events`, `effigy test:parity-evidence-ledger`,
`effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`,
`effigy ci:web`, `effigy docs:check`, `effigy qa`, and
`git diff --check origin/main...HEAD`.

`effigy drift:roles` is blocked in a normal active-cohort worktree: it resolves
the deferred Jetstream preview and fails on the absent sibling checkout
(`PAPERCUTS.md`, recorded on `main` at `ded8b5e71`).

`effigy doctor` baseline (generated-in-src, god-files, stale-suppressions)
unchanged.

## Remaining gaps

- native accessibility, visual comparison, and Jetstream admission unchanged
- orchestrator checkpoint at 42 mounted / 132 missing
