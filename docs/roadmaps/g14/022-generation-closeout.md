# g14.022 — Generation Closeout

Status: complete
Log: `../../logs/2026-08/16-g14-022-generation-closeout.md`
Depends on: `g14.021`
Governing refs: `008-pilot-verdict.md`,
`../../logs/2026-08/16-g14-021-experimental-cleanup-and-gate-consolidation.md`,
`conformance-estate.md`, `../../contracts/001-working-rules.md`

## Outcome

Close g14 as a rejected architecture pilot that still improved components,
native substrate behaviour, headless testing, and specimen ownership. Remove
post-cleanup planning drift. Leave one release-first checkpoint for Poodle
v0.2.0 without designing a third parity architecture inside this generation.

This is a documentation, evidence, and rollover tranche. Do not change
component APIs or implementations.

## Operator Rulings To Preserve

- Poodle v0.2.0 needs the full supported Svelte public roster. The denominator
  is every public Svelte component export, not a representative subset.
- Keep React tightly paired with Svelte through shared CSS and framework-free
  web behaviour. Record its applicable mirror honestly; do not let an
  experimental React gap make the Svelte release denominator vague.
- Native release evidence names an explicit certified GPUI subset until the
  full Rust roster is complete. Do not call that full cross-runtime parity.
- Jetstream remains program-deferred. Its eventual admission is renderer and
  backend work, not an assumed consequence of `poodle-node` reuse.
- Contracts remain semantic authority. Small owner-local behaviour tests are
  the current evidence shape. The rejected universal interface/corpus/
  observation plane does not return.
- Curated specimens remain human-facing documentation. They are not parity
  snapshots or exhaustive variant matrices.
- A future visual-conformance lane may inventory bounded component variants,
  capture the same named fixtures per runtime, and compare geometry, tokens,
  and pixels within renderer-aware tolerances. It starts with primitives and
  reuses the retained headless/native capture foundation. Do not design or
  implement that harness here.

## Batch 1 — Closeout Audit

- [x] Read the g14.008 verdict, g14.021 cleanup log, retained-estate ledger,
      architecture 009, and spec 066 before editing status surfaces.
- [x] Confirm the rejected pilot plane is absent from active source, package
      exports, task selectors, and standing docs. Historical roadmaps, logs,
      architecture, and specs remain readable and explicitly rejected.
- [x] Confirm every product/backend defect credited to the pilot has the live
      owner named in the g14.021 ledger.
- [x] Record the final evidence honestly:
  - pilot: 22,746 source LOC against 472 LOC replaced
  - corrected comparator: 1,205 HistoryCenter differences
  - cleanup: raw additions/deletions split between source, tests, generated
    artifacts, and docs; do not treat generated deletion as implementation
    cost saved
  - six retained headless native regressions
- [x] Inspect retained visual infrastructure without running any local
      windowed capture. Record the generic pieces available to a later lane.
- [x] Run a stale-reference sweep. Repair active front doors only; do not
      rewrite historical point-in-time evidence.

## Batch 2 — Resolve The Remaining g14 Cards

- [x] Mark `g14.017` and `g14.020` as superseded execution plans. Preserve
      their approved web references and component requirements, but state that
      native completion must be recompiled under the next release runway.
- [x] Mark `g14.026` as carried forward rather than completed. Preserve its
      human-centred rubric and bounded shared specimen-plan boundary.
- [x] Close or rehome every other live g14 status. Retired cards stay retired;
      completed independent lanes stay completed.
- [x] Update `g14/README.md`, `roadmaps/README.md`, and
      `roadmaps/generation-index.md` so no surface still names cleanup or
      g14.021 as current work.

## Batch 3 — Leave The v0.2.0 Planning Checkpoint

- [x] Create the next-generation front door and exactly one initial roadmap
      card. Keep it release-first, not architecture-first.
- [x] The initial card must inventory the complete Svelte public roster and
      freeze the v0.2.0 denominator from source/package exports. For each
      component it should record contract, implementation, export, specimen,
      focused test, package-install, and downstream-use posture.
- [x] Record React mirror coverage, the certified GPUI subset, and deferred
      Jetstream separately. Missing evidence remains missing; one runtime does
      not borrow another runtime's pass.
- [x] Compile later work into bounded family tranches only after the inventory
      exists. Do not pre-author a component rollout from memory.
- [x] Carry the approved Licence and model-connection web suites, the specimen
      catalogue audit, primitive-first visual conformance, and native
      completion into the next-generation envelope without implementing them.
- [x] Leave one clear next task: execute the roster/release-baseline inventory.

## CI Workflow Decision

`.github/workflows/ci-conformance.yml` is stale: its name, comments, paths,
job name, and cache key describe the removed pilot, while its command now runs
only `regressions:native`. `ci:native` already includes that board.

- [x] Inspect branch-protection/check-name implications and record a concrete
      recommendation: delete the redundant workflow, or replace it with an
      honestly named focused-native workflow if a separate required check is
      still needed.
- [x] Do **not** edit `.github/workflows/` in this worker PR. Workflow mutation
      needs explicit operator approval. Keep `ci:conformance` as a compatibility
      alias until the orchestrator acts on the recommendation.

## Deliverables

- one August g14 closeout log with result, evidence, residual gaps, retained
  value, known limits, CI recommendation, and next-program question
- g14 status and roadmap front doors closed coherently
- `g14.017`, `g14.020`, and `g14.026` given explicit carry-forward dispositions
- next-generation README plus one release-baseline inventory card
- no component, runtime, package API, specimen, or workflow mutation

## Acceptance

- The rejected conformance plane is absent from active source and standing
  architecture; historical evidence remains readable.
- Every retained component/backend fix and regression has a live owner.
- Licence and model-connection web references remain approved; native/runtime
  gaps are replanned rather than falsely completed.
- The human-centred specimen catalogue lane has one honest carry-forward state
  and no exhaustive corpus projection.
- Architecture, specs, contracts, package docs, task selectors, roadmap front
  doors, and the closeout log agree on the post-reject system.
- The next runway names the full Svelte roster as the v0.2.0 denominator,
  React and GPUI posture separately, and Jetstream as deferred.
- Active Svelte, React, Rust, GPUI, docs, health, and diff boards pass or carry
  a named pre-existing owner. No windowed or Jetstream validation runs.
- Generation evidence records defects caught, parity gaps still open, total
  pilot cost, cleanup delta, known limits, and the question the next design
  must answer.

## Stop Conditions

- Closeout presents the pilot as adopted, the active cohort as complete, or a
  deleted gate as proof of parity.
- A retained experiment remains required by active tooling without explicit
  promotion.
- Work starts designing the replacement component authority, shared corpus,
  comparator, or visual harness.
- The initial release inventory silently samples the Svelte roster.
- Any change reaches component/runtime source, curated specimens,
  `.github/workflows/`, Jetstream, or downstream application repositories.

## Writable Scope

- `docs/roadmaps/g14/**` status/currentness edits
- `docs/roadmaps/README.md` and `docs/roadmaps/generation-index.md`
- one next-generation README and one initial release-baseline roadmap
- one August closeout log
- active architecture/spec/contract/package docs only when a stale standing
  claim contradicts g14.008/g14.021
- `PAPERCUTS.md` only for newly discovered execution friction

## Validation

Run one meaningful final batch through Effigy:

- `effigy qa` — headless local release board
- `effigy docs:check`
- `effigy regressions:native`
- `effigy doctor` — record the known generated-in-src, god-file, stale-
  suppression, graph-index, and comment-ratio baseline; do not expand this
  closeout to fix it
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or any
Jetstream selector. Do not use a sibling Jetstream checkout.
