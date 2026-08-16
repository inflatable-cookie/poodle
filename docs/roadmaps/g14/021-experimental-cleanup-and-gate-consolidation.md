# g14.021 — Rejected Pilot Cleanup And Evidence Retention

Status: complete
Log: `../../logs/2026-08/16-g14-021-experimental-cleanup-and-gate-consolidation.md`
Depends on: `g14.008`
Governing refs: `008-pilot-verdict.md`,
`../../contracts/001-working-rules.md`

## Outcome

Remove the rejected executable-conformance authority without losing shipped
component fixes or the defects it taught us to test. Leave one honest,
headless regression board and no active document, selector, generated file, or
package API claiming the six-profile pipeline was adopted.

This is cleanup, not a replacement parity architecture.

## Starting State

`g14.008` rejected architecture 009/spec 066 after measuring 22,746 source LOC
against 472 LOC replaced. The audit also repaired the comparator's omitted
HistoryCenter registration and three omitted observation fields. The full
board now fails with 1,205 HistoryCenter differences. That red state is
intentional evidence; do not restore green by hiding the corpus again.

The pilot nevertheless landed valuable component, shared-renderer,
`poodle-node`, GPUI backend, and headless-test-platform fixes. Preserve those.

## Batch 1 — Freeze Retained Evidence

- Build a defect ledger from the g14.001–g14.007 logs and the retained-value
  section of `g14.008`.
- For each caught defect, name the smallest surviving owner:
  - core/headless state-machine or value-law test
  - Svelte/React component interaction test
  - shared renderer/node test
  - GPUI headless backend/component test
- Reuse existing focused tests where they already prove the claim. Add only
  the missing high-value regressions needed before deleting a pilot harness.
- Keep GPUI's in-memory test-platform helpers generic and headless. Remove
  corpus-specific fixture adapters from them.
- Record any pilot case deliberately not retained and why its claim is
  redundant, presentation-only, or invalid cross-runtime comparison.

Do not mechanically translate 104 cases into 104 new tests. Preserve claims,
not the rejected data format.

## Batch 2 — Restore Honest Authorities

- Restore one hand-written Rust component-spec authority where generated
  declarations currently depend on the rejected TypeScript interface/codegen
  path. Preserve the shipped public shape and compatibility fields.
- Move Button, Tabs, Popover, and TextInput web shells off inferred pilot
  interface types without changing their public APIs. RangeSlider and
  HistoryCenter already demonstrate the independent-authority posture.
- Remove generated Rust conformance declarations, their module wiring, and
  byte-exact authority checks after the hand-written replacements compile.
- Remove portable interface modules when no shipped component or retained test
  consumes them.
- Keep existing core behaviour machines, shared Rust composition, CSS,
  component APIs, component implementations, and approved specimens.

No cross-language IDL, scene tree, schema replacement, or new component
factory enters this card.

## Batch 3 — Retire The Pilot Plane

Remove or reduce the following after retained tests and declarations are in
place:

- typed case corpora and generated interface/case/primitive JSON
- conformance codegen parser, Rust target, CLI wiring, fixtures, and tests
- Svelte/React corpus hosts and adapters
- GPUI component corpus adapters and component-specific fixture support
- normalized observation comparator and manual component registries
- primitive capability report, corpus projection, and pilot cost script
- conformance-only native visual repair if no standing visual workflow owns it
- rejected task selectors, documentation commands, package scripts, and stale
  generated artifacts

`ci:conformance` may remain as the workflow-compatible name during this card,
but only if it becomes a small headless regression board and its output states
that scope. It may not claim portable-interface, shared-corpus, normalized
observation, primitive certification, or six-profile completion.

Do not edit `.github/workflows/`. Workflow mutation requires separate explicit
operator approval. Keep its existing `effigy ci:conformance` entrypoint valid
through task-level consolidation, then let `g14.022` decide whether a rename or
workflow deletion is worth that approval.

## Documentation And Estate

- Execute every final disposition in `g14.008` and update
  `conformance-estate.md` from historical pilot sequence to a concise retained
  estate ledger.
- Keep g14.001–g14.008 logs and Git history. Do not rewrite historical results
  as if the mechanism never existed.
- Remove active package guides and README claims that the rejected path is the
  way to build a component.
- Keep the human-centred specimen boundary. Remove projection-only catalogue
  wiring; do not change curated specimen content.
- Update `g14.017` and `g14.020` with discovered surviving native work, but do
  not design their replacement execution method here.
- Write one August cleanup log containing the defect ledger, deleted/replaced
  inventory, retained selector scope, and recovery commit.

## Acceptance

- No shipped API or component behaviour changes except a demonstrated bug fix
  needed to preserve a retained regression.
- Every product/backend defect credited to the pilot has a named retained test
  or an explicit evidence disposition.
- No active source import depends on the portable interface/case authority or
  generated conformance declarations.
- No active task claims six-profile cross-runtime completion.
- `effigy ci:conformance`, if retained, is headless, bounded, and accurately
  named in its output/docs as a regression board.
- Svelte, React, Rust shared-renderer, and GPUI focused component boards pass.
- Curated specimens and preview navigation are unchanged.
- Orphan/generated-source checks identify no residue owned by the rejected
  pilot. Unrelated baseline findings remain explicitly outside this card.
- Architecture 009, spec 066, roadmap 008, and historical logs remain as
  rejected evidence; active front doors point to the cleanup state.

## Stop Conditions

- A component fix or high-value regression would be lost without a clear
  replacement test. Preserve it first.
- Cleanup starts inventing a new parity schema, universal component model,
  renderer abstraction, or code generator. Stop and defer design.
- Green is obtained by skipping HistoryCenter, deleting a failing assertion
  without disposition, or relabelling missing evidence.
- The work expands into specimen redesign, Licence/model native completion,
  Jetstream parity, or unrelated doctor cleanup.
- A required change reaches `.github/workflows/`; return it to the orchestrator
  for explicit operator approval.

## Writable Scope

- `packages/core/src/conformance/**` and conformance-owned tests/scripts
- `packages/codegen/**` conformance target, fixtures, and tests only
- `packages/contracts/components/**` generated declaration restoration
- `packages/contracts/headless/**` retained regression tests only
- `packages/render/**` conformance probes/support and retained tests
- `packages/gpui/**` conformance harnesses plus retained headless tests/helpers
- `test/conformance/**` and conformance-owned native-visual support
- Svelte/React component type imports needed to detach rejected declarations
- `tasks/effigy.tasks.toml`, package manifests/exports, active docs, roadmap
  statuses, one August log, and `PAPERCUTS.md`

Do not change curated specimen implementations, component styling, public API
meaning, Jetstream code, application integrations, unrelated test suites, or
`.github/workflows/`.

## Validation

Use Effigy selectors discovered from the post-cleanup task inventory. At
minimum:

- focused retained core/headless regressions
- focused Svelte and React component tests
- focused shared-renderer and GPUI headless tests
- `effigy check:svelte`
- `effigy react:build`
- `effigy check:gpui`
- `effigy docs:check`
- the retained `effigy ci:conformance` compatibility entrypoint, if present
- `effigy doctor` and owned orphan/generated-source scans
- `git diff --check`

Never run a `*-windowed` selector. Do not run Jetstream or the full release
board for this cleanup.
