# g15.042 — Stepper Native Interaction Parity

Status: **ready** — independent of `g15.041`; may run in parallel
Found by: `g15.025`, confirmed by `g15.026`
Depends on: `g15.026` (real headless GPUI specimen construction)
Unblocks: `g15.013`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/stepper.md`, `release-gap-register.md`

## Problem

The shared Rust Stepper already renders separate selection, re-run, and
collapse controls. Its GPUI preview adapter exposes only
`on_collapsed_change`. The specimen therefore shows selectable steps and
re-run buttons that do nothing, while collapse works. This is a real active-
cohort interaction gap, not a specimen presentation defect.

## Goal

Wire the existing shared Stepper interactions through GPUI, make the specimen
demonstrate them honestly, and prove the three actions remain distinct:
selection changes the current step, re-run emits only for the named complete
step, and collapse only folds the vertical stepper.

## Scope

- Add `on_change` and `on_rerun` builder seams to the existing preview/native
  Stepper adapter. Reuse `poodle_render::StepperHandlers`; do not create a
  second Stepper implementation.
- Give the GPUI specimen retained state for the selected step and a visible,
  concise re-run receipt. Keep Examples human-centred.
- Preserve disabled-step suppression, separate selection/re-run controls,
  vertical-only collapse, focus movement, and existing keyboard activation.
- Add mounted headless GPUI evidence through the real node backend. Do not call
  handler closures directly.
- Close only the Stepper row in `release-gap-register.md` and record one August
  batch log.

## Acceptance

- [ ] Activating an enabled step updates the specimen's current step and emits
      the exact value once.
- [ ] A disabled step cannot select or emit.
- [ ] Re-run is present only where the contract permits it and emits the exact
      completed-step value without selecting that step.
- [ ] Pointer and keyboard activation reach the same mounted controls.
- [ ] Collapse remains independent and vertical-only.
- [ ] The specimen makes the live result visible without becoming an event
      transcript or exhaustive matrix.
- [ ] Focused Rust/render and mounted GPUI regressions pass; the release-gap
      row closes with evidence.

## Stop Conditions

- The existing `Node` activation model cannot distinguish the step trigger
  from its re-run control.
- Correct focus movement requires a Stepper-wide backend focus architecture
  rather than stable ids on the existing controls.
- The fix requires changing Stepper's public contract or web APIs.

## Writable Scope

- `packages/gpui/preview/src/node_compat.rs`
- `packages/gpui/preview/src/specimens/stepper.rs`
- focused Stepper evidence under `packages/render/` and
  `packages/gpui/preview/tests/`
- `release-gap-register.md` and one August batch log
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- focused Stepper Rust tests
- `effigy regressions:native`
- `effigy check:gpui`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Headless only. Never run `*-windowed`, `test:native-visual`, Jetstream, or
release selectors.
