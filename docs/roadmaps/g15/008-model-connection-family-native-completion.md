# g15.008 — Model-Connection Family Native Completion

Status: **blocked** — orchestration hold; `g15.003` is the active card
Depends on: `g15.001` (measured gaps); carries `g14.020` requirements with
approved web references (`g14.018`/`g14.019`) intact
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../roadmaps/g14/020-model-connection-active-runtime-completion.md`,
`../../roadmaps/g14/conformance-estate.md`, `../../contracts/001-working-rules.md`

## Outcome

Complete the native surface for the model-connection family —
ModelConnectionPicker, ModelConnectionSetup, ModelConnectionCard,
ModelCatalogueEditor — which the inventory measured as web-complete but
missing Rust declaration, Rust render, and GPUI specimen. Poodle owns
presentation and interaction only; Nucleus and Swallowtail remain external
authorities. No provider registry, credential authority, or model-default
policy enters this intake.

## Scope

- ModelConnectionPicker, ModelConnectionSetup, ModelConnectionCard,
  ModelCatalogueEditor
- new Rust declarations in `poodle_specs`
- new `poodle-render` implementations
- new GPUI specimens and focused headless tests

## Execution Plan

- [ ] **Batch A — declarations:** hand-written `ModelConnectionPickerSpec`,
      `ModelConnectionSetupSpec`, `ModelConnectionCardSpec`,
      `ModelCatalogueEditorSpec` in `poodle_specs` matching the contract props
      tables.
- [ ] **Batch B — render:** `poodle-render` implementations for all four;
      model-catalogue editing stays structural, never schema-authoritative.
- [ ] **Batch C — GPUI and evidence:** GPUI specimens plus focused tests for
      picker, setup flow, card, and catalogue editor surfaces.

## Goals

- [ ] Hand-written `<Name>Spec` declarations matching the contract props
      tables.
- [ ] `poodle-render` implementations for all four; model-catalogue editing
      stays structural, never schema-authoritative.
- [ ] GPUI specimens and focused tests for picker, setup flow, card, and
      catalogue editor surfaces.
- [ ] Preserve approved curated specimens unchanged.

## Acceptance

- [ ] Every active-cohort surface has evidence named in the card log; one
      runtime does not borrow another's pass.
- [ ] `cargo test -p poodle-render`, `effigy check:gpui`, and
      `effigy regressions:native` pass.
- [ ] Jetstream reported as program-deferred, not as an accepted absence.
- [ ] No provider registry, credential store, or model-default policy added.

## Stop Conditions

- A portable interface, shared corpus, or comparator reappears under a new
  name.
- External authority logic (Nucleus/Swallowtail) is absorbed into Poodle.
- The intake expands beyond the four measured components without a new card.

## Writable Scope

- Rust declarations, render modules, GPUI specimens, focused tests
- bounded contract-first fixes to scoped defects the new evidence exposes
- `release-baseline-roster.md` and `release-gap-register.md` (native rows only,
  no status lines)
- one August batch log under `docs/logs/2026-08/`
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `cargo test -p poodle-render`
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
