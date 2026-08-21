# g15.042 — Stepper native interaction parity

Date: 2026-08-21
Card: `docs/roadmaps/g15/042-stepper-native-interaction-parity.md`
Contract: `docs/contracts/components/stepper.md`
Gap row: `docs/roadmaps/g15/release-gap-register.md`
PR: #60

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
  only. `specimens/stepper.rs` becomes `pub(crate)` for its two test-only
  probe markers; `specimen_probe.rs` gains the route probe and the render
  exclusion described below
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

- `stepper_selection_and_rerun_reach_separate_mounted_controls` — pointer
  activation, then keyboard activation of the control the pointer just
  focused, both reach the real trigger and the real rerun button through
  gpui's own dispatch. Selecting never re-runs, re-running never selects, and
  a disabled step takes neither the click nor the focus the click would have
  moved.
- `stepper_collapse_stays_independent_in_a_mounted_window` — the summary
  folds the vertical track by pointer and, once focused, by key; it carries
  the new state and selects and re-runs nothing.

Both drive the mounted controls by their painted bounds. No handler closure is
called directly.

One route probe through the changed seam
(`packages/gpui/preview/src/specimen_probe.rs`):

- `stepper_route_selection_and_rerun_run_through_the_preview_adapter` — mounts
  the production `PreviewRoot` on the Stepper route and clicks the real page.
  Each click travels the specimen's `Stepper::from_spec(..).on_change(..)`
  builders, `IntoElement`, the node backend, and the specimen event queue
  before it becomes retained text, which the test reads back off the root
  entity. Selecting the first wizard step retains it, the disabled step
  retains nothing, selecting the second step of the Re-run group moves that
  group's current step, and re-running the first step records that step
  without moving the current one.

The two boards are not redundant. Replacing either new builder body with a
no-op leaves both mounted regressions green and fails the route probe —
verified by mutation, once per builder. `drift:handlers` cannot catch it
either, because this compat wrapper stores one `handlers` field rather than
declared `on_*` fields.

## Execution friction

The route probe passed alone and failed inside `probe:gpui-specimens`, whose
sweep renders four shards in parallel. `poodle-gpui-node-backend`'s generated
element-id counter is a process global, so a second rendering thread restarts
it mid-click and gpui drops the press/release pair with no error. Worked around
with an `RwLock` in `specimen_probe.rs` — shards share it, the probe that
clicks id-less node-backed controls takes it exclusively — and recorded in
`PAPERCUTS.md`. Making the counter thread-local, where the backend's other
registries already live, is the real fix and belongs to the backend.

## Commands

- `cargo test -p poodle-render --manifest-path packages/render/Cargo.toml`:
  370 passed (365 on the rebase base)
- `effigy regressions:native`: 56 passed (54 on the rebase base)
- `effigy check:gpui`: passed — preview `cargo check`, shared-render tests,
  node-backend tests
- `effigy probe:gpui-specimens`: 8 passed (7 before), stable across repeat runs
- `effigy docs:check`: passed
- `git diff --check origin/main...HEAD`: clean

All native evidence is headless. No windowed, `test:native-visual`,
Jetstream, visual-conformance, or release selector ran.

## Routed, not fixed

GPUI's Stepper trigger, rerun, and summary set `interaction.focusable` but
declare no focus treatment, so the node backend registers no focus handle for
them and no focus ring paints. Keyboard **activation** works — gpui focuses a
focusable element on pointer-down and synthesises the click from Enter or
Space — but keyboard **entry** does not: focus cannot reach a step without a
prior pointer press. `stepper.md` §6 requires `Tab` entry and order and
activation of a focused control, and §8 requires the ring, so this is a
release gap in its own right rather than a footnote to a closed one.

It is recorded as its own row under *Native Interaction And Focus Gaps* in
`release-gap-register.md`, open, owned by no card yet, and named as a
prerequisite for `g15.013`. Every claim this batch makes about the keyboard is
narrowed to "activation after pointer focus" accordingly.

Arrow, `Home`, and `End` movement between triggers stays a separate web-only
delta, as `stepper.md` §10 already records. Closing the focus gap means
deciding a shared-render focus treatment for a control with no resting border
to take the ring, which lands on Jetstream too — outside this card's seam.
