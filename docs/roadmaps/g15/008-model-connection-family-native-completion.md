# g15.008 — Model-Connection Family Native Completion

Status: **complete** — accepted and merged in PR #33 (`6b36c7d3`)
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

## Native Binding Boundary

The web props tables are semantic authority, not literal Rust struct layouts.
Keep the native API split into data, host composition, callbacks, and pure
behaviour:

- `<Name>Spec` contains cloneable controlled display data only. It never owns
  callbacks, closures, credential values, backend/provider types, or host
  nodes.
- Web-only uncontrolled seeds (`defaultValue`, `defaultQuery`, `defaultStage`,
  `defaultOpen`) do not enter Rust specs. GPUI/AppState owns the current value
  and rerenders after a callback requests a change.
- Safe public Rust structs/enums structurally mirror the web display shapes:
  opaque IDs and safe copy; connection option and availability; picker
  state/variant; setup stage; card readiness; and catalogue state, item,
  badge, tone, and visibility-change data.
- `poodle-headless` owns pure Rust behaviour mirrors for picker
  filtering/grouping/selectability/state copy/result announcements; setup
  continue/submit guards and transitions; catalogue shown/hidden derivation,
  full shown-order requests, visibility requests, focus-after-hide, and
  announcements; and the required readiness/availability tone and copy
  mappings.
- Prove those mirrors against owner-local TS and Rust vectors that name the
  same inputs and observable outputs. Duplicated explicit cases are acceptable;
  a portable interface, shared corpus, normalized observation, comparator, or
  universal conformance mechanism is not.
- Renderer handler structs own picker query/value requests; setup
  stage/value/query/submit/cancel requests; card open/enabled requests; and
  catalogue ordering, visibility, info, keyboard, and drag intents.
- Host-composed content stays outside specs and is keyed by opaque IDs where
  rows repeat: picker option leading content and footer; setup leading,
  configuration, and aside content; card leading, badges, closed accessory,
  actions, and details; catalogue item leading/meta and custom actions. The
  exact Rust container type is an implementation choice, but provider marks
  are never resolved by a Poodle registry and host `Node`s never enter specs.
- Preserve controlled semantics. A callback requests state; the host updates
  the spec before the next render. The generic provider mark is the specimen
  fallback, not a provider catalogue.

## Execution Plan

- [x] **Batch A — declarations and behaviour:** hand-written `ModelConnectionPickerSpec`,
      `ModelConnectionSetupSpec`, `ModelConnectionCardSpec`,
      `ModelCatalogueEditorSpec` in `poodle_specs` matching the contract props
      tables through the binding boundary above; pure `poodle-headless`
      behaviour mirrors and paired owner-local vectors.
- [x] **Batch B — render and host bindings:** `poodle-render` implementations,
      handler structs, and host-composition seams for all four;
      model-catalogue editing stays structural, never schema-authoritative.
- [x] **Batch C — GPUI and evidence:** GPUI specimens plus focused tests for
      picker, setup flow, card, and catalogue editor surfaces.
- [x] **Batch D — release evidence:** reconcile the four native roster/register
      rows and write the card log with evidence named per runtime.

## Goals

- [x] Hand-written `<Name>Spec` declarations matching the contract props
      tables.
- [x] `poodle-render` implementations for all four; model-catalogue editing
      stays structural, never schema-authoritative.
- [x] GPUI specimens and focused tests for picker, setup flow, card, and
      catalogue editor surfaces.
- [x] Pure Rust behaviour mirrors and paired TS/Rust vectors cover the
      observable state transitions without introducing cross-runtime
      machinery.
- [x] Preserve approved curated specimens unchanged.

## Acceptance

- [x] Every active-cohort surface has evidence named in the card log; one
      runtime does not borrow another's pass.
- [x] Picker evidence covers filtering order, exact selection, disabled guards,
      radio/roving focus behaviour, and empty/loading/error/ready postures.
- [x] Setup evidence covers both direct submission when configuration is not
      required and configured-stage transitions, including pending guards.
- [x] Card evidence covers independent open/enabled state, closed accessory
      composition, disabled presentation, and focus restoration after close.
- [x] Catalogue evidence covers explicit moves, keyboard grab/move/drop,
      admitted pointer drag, complete ordered-ID payloads, hide/restore
      payloads, focus-after-hide, announcements, and distinct empty/error/
      disabled states.
- [x] Specs contain safe controlled data only; credentials never cross the
      component boundary and repeated host composition is keyed by opaque IDs.
- [x] `cargo test -p poodle-render`, `effigy check:gpui`, and
      `effigy regressions:native` pass.
- [x] Jetstream reported as program-deferred, not as an accepted absence.
- [x] No provider registry, credential store, or model-default policy added.

## Stop Conditions

- A portable interface, shared corpus, or comparator reappears under a new
  name.
- External authority logic (Nucleus/Swallowtail) is absorbed into Poodle.
- The intake expands beyond the four measured components without a new card.

## Writable Scope

- Rust declarations, `poodle-headless` behaviour modules, render modules,
  GPUI specimens, and focused tests for the four named components
- native-binding notes in the four named component contracts
- `release-baseline-roster.md` and `release-gap-register.md` (native rows only,
  no status lines)
- one August batch log under `docs/logs/2026-08/`
- `PAPERCUTS.md` for newly discovered execution friction

The approved Svelte/React implementations and specimens are frozen for this
card. If native evidence exposes a web contract defect or requires a public
web change, stop and return it to the orchestrator instead of widening scope.

## Validation

- `effigy test:core`
- `cargo test -p poodle-headless`
- `cargo test -p poodle-specs`
- `cargo test -p poodle-render`
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy docs:check`
- `effigy qa` once on the final rebased head
- `git diff --check origin/main...HEAD`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
