# g14.023 — Headless GPUI Conformance Execution

Status: planned — next after g14.004
Depends on: `g14.004`
Governing refs: `../../architecture/009-cross-runtime-component-conformance.md`,
`../../specs/066-executable-component-conformance.md`,
`../../contracts/001-working-rules.md`, `conformance-estate.md`

## Outcome

Replace the focus-stealing AppKit conformance runner with deterministic GPUI
execution on GPUI 0.2.2's in-memory test platform. The complete active-cohort
board must execute Svelte, React, renderer-neutral Rust, and GPUI without
opening or activating an operating-system window.

This is a runtime-harness migration, not a new component model. Preserve the
landed interfaces, case corpora, normalized observation schema, renderer
pipeline, and strict completion semantics.

## Goals

- [ ] Execute every landed GPUI Button, primitive, RangeSlider, and Tabs case
      through `TestAppContext`, `VisualTestContext`, and `TestWindow`.
- [ ] Render the real `poodle-render` → `poodle-node` →
      `poodle-gpui-node-backend` path before observing or interacting.
- [ ] Dispatch pointer, drag, focus, and keyboard actions through GPUI's test
      platform event tree; never invoke component handlers as test shortcuts.
- [ ] Make the ordinary complete conformance selector headless and safe in any
      local worktree.
- [ ] Delete the AppKit activation/calibration/retry path and its windowed task
      family once equivalence is proved.

## Execution Plan

### Batch A — In-memory runtime driver

- [ ] Introduce one generic GPUI test-platform driver for mounting nodes,
      drawing frames, locating focus targets, dispatching input, draining
      tasks, and collecting backend observations.
- [ ] Port the primitive probes and Button cases first. Keep capability IDs,
      case IDs, reports, and `component-observation.v1` unchanged.
- [ ] Prove focus and activation through the backend focus registry and bound
      listener path. A driver that calls a Poodle callback directly fails this
      card.

### Batch B — Controlled and collection cases

- [ ] Port RangeSlider press/drag/release and keyboard execution without a
      component-specific branch in the generic driver.
- [ ] Run the landed Tabs collection/navigation cases through the same action
      vocabulary and driver.
- [ ] Add planted failures for an inert listener, wrong focus target, missing
      selected state, and broken drag/keyboard event order.

### Batch C — Gate and estate consolidation

- [ ] Replace `conformance:complete-windowed` and
      `ci:conformance-windowed` with a single headless
      `conformance:complete` path. `ci:conformance` must execute it locally and
      in CI without environment-dependent behavior.
- [ ] Remove `--windowed`, AppKit activation, affine click calibration,
      first-click retry, foreground opt-in plumbing, and obsolete task aliases.
- [ ] Update the cost report, conformance estate, spec evidence, task comments,
      and one August batch log with before/after runtime and source cost.

## Acceptance Criteria

- [ ] Running the complete conformance board never creates an AppKit window,
      changes the active application, or takes keyboard focus.
- [ ] All landed active-runtime cases execute and compare; GPUI is execution
      evidence, not compile-only coverage.
- [ ] GPUI observations retain the same normalized shape and strict failure
      identity as the windowed baseline.
- [ ] Pointer, drag, keyboard, focus, and event-order planted failures all fail
      for the expected runtime/case/step/field.
- [ ] Removing a GPUI registration or backend listener still fails completion.
- [ ] No component name, part list, fixture, or assertion enters the generic
      driver.
- [ ] Jetstream remains explicitly deferred.

## Stop Conditions

- GPUI's test platform bypasses the node backend or cannot exercise its normal
  dispatch tree for a required action.
- Matching the old report requires retained AppKit calibration assumptions or
  component-specific generic-runner branches.
- A case relies on pixel equivalence. Visual regression remains a separate
  concern and must not be smuggled into semantic conformance.

Stop with the failed capability, smallest reproduction, and options. Do not
fall back to a foreground local runner.

## Writable Scope

- `packages/gpui/preview/src/**conformance**`
- `packages/gpui/preview/src/primitive_probes_gpui.rs`
- focused GPUI conformance tests and test-platform support
- `tasks/effigy.tasks.toml`, `scripts/run-conformance-board.ts`
- `packages/core/scripts/conformance-cost.ts`
- `test/conformance/`
- `docs/specs/066-executable-component-conformance.md`
- `docs/roadmaps/g14/conformance-estate.md`
- this roadmap, the g14 index, one August batch log, and `PAPERCUTS.md`

Do not edit component contracts, component implementations, corpus semantics,
generated interface declarations, Rust node vocabulary, Jetstream, release
workflows, or unrelated roadmap status.

## Validation

All validation for this card is headless:

- focused GPUI test-platform driver tests
- all landed Button, primitive, RangeSlider, and Tabs cases
- `effigy conformance:check`
- the new `effigy conformance:complete`
- `effigy conformance:cost`
- `effigy ci:web`
- `effigy ci:rust`
- `effigy ci:native`
- `effigy docs:check`
- `git diff --check`

Do not run `test:native-visual` or any legacy `*-windowed` selector as evidence
for this card.
