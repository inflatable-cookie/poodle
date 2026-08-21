# g15.042 — Stepper native interaction parity

Date: 2026-08-21
Card: `docs/roadmaps/g15/042-stepper-native-interaction-parity.md`
Contract: `docs/contracts/components/stepper.md`
Gap row: `docs/roadmaps/g15/release-gap-register.md`
PR: pending

## Outcome

GPUI's Stepper adapter accepted `poodle_render::StepperHandlers` and exposed
one builder out of three, so the specimen advertised selectable steps and
re-run buttons that did nothing while collapse worked. `node_compat.rs` now
binds `on_change` and `on_rerun` alongside `on_collapsed_change`, and the
specimen retains the current step plus one re-run receipt so the result is
visible rather than asserted.

Nothing else moved. The shared composition already rendered all three controls
and already kept them apart; `packages/render/src/stepper.rs` gained tests and
no implementation change. There is no GPUI-only Stepper, no component-specific
backend path, and no public or web API change.

## Change class

- **Packages changed:** internal `poodle-gpui-preview`; `poodle-render` tests
  only
- **Public-intent entry points:** none — the two new builders are
  `pub(crate)` on the preview adapter
- **Compatibility:** additive, pre-1.0; no alias, fallback, or contract change
- **Downstream re-check:** GPUI Stepper hosts should now pass `on_change` and
  `on_rerun` and own the current step, exactly as the contract's controlled
  mode describes

## Evidence

Five focused shared-render tests (`cargo test -p poodle-render stepper`):

- an enabled trigger emits its own value; a disabled step carries no
  activation at all, so suppression is structural rather than a guard inside a
  handler
- the rerun control is a different node from the trigger, emits the completed
  step's value, and never selects it
- rerun appears only for a `complete` step whose host asked for it
- an unwired rerun still swallows its activation, so the press cannot bubble
  into selection
- collapse carries the state it is moving to, omits the step rows rather than
  hiding them, and is ignored in horizontal orientation

Two mounted headless GPUI regressions
(`packages/gpui/preview/tests/headless_regressions.rs`):

- `stepper_selection_and_rerun_reach_separate_mounted_controls` — pointer and
  keyboard both reach the real trigger and the real rerun button through
  gpui's own dispatch. Selecting never re-runs, re-running never selects, and
  a disabled step takes neither the click nor the focus the click would have
  moved.
- `stepper_collapse_stays_independent_in_a_mounted_window` — the summary
  folds the vertical track by pointer and by key, carries the new state, and
  selects and re-runs nothing.

Both drive the mounted controls by their painted bounds. No handler closure is
called directly.

## Commands

- `cargo test -p poodle-render --manifest-path packages/render/Cargo.toml`:
  369 passed (364 before this batch)
- `effigy regressions:native`: 55 passed (53 before this batch)
- `effigy check:gpui`: passed — preview `cargo check`, 369 shared-render
  tests, 22 node-backend tests
- `effigy docs:check`: passed
- `git diff --check origin/main...HEAD`: clean

All native evidence is headless. No windowed, `test:native-visual`,
Jetstream, visual-conformance, or release selector ran.

## Observed, not fixed

GPUI's Stepper trigger, rerun, and summary carry no focus treatment, so the
node backend registers no focus handle for them and there is no visible focus
ring. Keyboard activation still works — gpui focuses a focusable element on
pointer-down and synthesises the click from Enter or Space — but focus cannot
be moved to a step without first pointing at it, and the contract's §8 focus
ring is unrendered on the natives. Arrow, Home, and End movement between
triggers remains web-only, as `stepper.md` §10 already records. Closing either
would mean a shared-render focus treatment for a control with no resting
border to take the ring, which is outside this card's seam.
