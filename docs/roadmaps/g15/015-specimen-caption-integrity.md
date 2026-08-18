# g15.015 — Specimen Caption Integrity

Status: **complete** — PR #37 merged; operator live review accepted
Consumes: `g15.011` partial screening baseline
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`,
`../../contracts/001-working-rules.md`

## Outcome

Every example on a Svelte catalogue page shows the caption its author wrote,
and the gate that would have caught the failure runs.

`g15.011` found nine agent-surface pages where all 52 example captions render
blank. `SpecimenGroup` accepts `label`; those pages pass `title` and
`description`. Svelte drops unknown props silently, so the authored copy — good
copy, describing what each example teaches — never reaches the page.

The cause is systemic, not local: `check:svelte` type-checks
`packages/svelte/install-smoke` and `packages/svelte/components`, and never
`packages/svelte/preview`. The dispatch baseline is 428 errors in 25 files:
348 generated-catalogue type errors, 52 caption-prop errors, and 28 residual
preview-workspace errors. This card closes the complete measured gate deficit;
it does not add a selector that is red on arrival.

## Scope

- these nine Svelte pages, and no inferred page set:
  - `AgentMessageSpecimen.svelte` — 8 captions
  - `AgentPlanSpecimen.svelte` — 4 captions
  - `AgentPlanRecordSpecimen.svelte` — 6 captions
  - `AgentQuestionSpecimen.svelte` — 6 captions
  - `AgentQuestionRecordSpecimen.svelte` — 6 captions
  - `AgentSubagentSpecimen.svelte` — 6 captions
  - `ChangedFilesSpecimen.svelte` — 7 captions
  - `ToolCallSpecimen.svelte` — 4 captions
  - `ToolCallGroupSpecimen.svelte` — 5 captions
- `SpecimenGroup` in both web previews
- the Svelte preview workspace's measured 28 residual diagnostics
- the Svelte type-check gate's scope and task definition

## Goals

- [x] All 52 captions render, with the authored wording.
- [x] `SpecimenGroup` carries an optional `description`, so the explanatory
      sentences those pages already contain reach the reader instead of being
      deleted. React matches.
- [x] `packages/svelte/preview` is inside a type-check gate.
- [x] The 348 `readonly never[]` errors from the generated catalogue type are
      fixed at their source or at one honest generated-data typing boundary.
      Blanket suppression is not an accepted fix.
- [x] The remaining 28 diagnostics are fixed in their owning files without
      broadening into component redesign. They currently comprise 13 recipe
      inventory script errors, 6 contract-drift script errors, 5
      `ListContainerSpecimen` errors, 2 `SceneSpecimen` errors, and one each in
      `component-registry`, `DialogSpecimen`, and core licence narrowing.

## Acceptance

- [x] A live sweep of the nine pages reports zero blank captions.
- [x] A named Effigy selector type-checks `packages/svelte/preview` with zero
      errors, and `check:svelte`/`ci:web` inherit that selector.
- [x] Mutation proof: temporarily reintroducing `title=` on one scoped page
      makes the selector fail; restoring `label=` makes it pass. The mutation
      is not committed.
- [x] No component public API, contract, or semantic change.
- [x] **Operator review of the changed pages in the live Svelte and React
      previews before this card is called complete.** Unreviewed pages remain
      an explicit PR item.

## Stop Conditions

- The fix becomes a rewrite of the nine pages' examples. This card restores
  captions; content curation is `g15.018`.
- Blanket suppressions, excluding preview source from the new gate, or making
  the new selector advisory instead of required.
- A residual diagnostic exposes a public component or contract question. Stop
  and return that finding instead of changing the public surface inside this
  card.

## Writable Scope

- the nine named Svelte specimen files and their React counterparts only where
  needed to keep caption structure and copy aligned
- `SpecimenGroup` (Svelte + React)
- the two generated catalogue artifacts and their generator/typing boundary
- the 9 residual-diagnostic owners named in Goals, including
  `packages/core/src/licence.ts`; fixes must remain type-only or preserve
  existing behaviour
- Svelte preview type configuration if needed for Bun-authored scripts
- `tasks/effigy.tasks.toml` gate scope — **operator approved for this card**
- one batch log

## Validation

- focused `SpecimenGroup`/caption evidence
- the new Svelte preview type-check selector, including fail/pass mutation proof
- `effigy check:svelte`, `effigy react:build`, `effigy catalogue:check`,
  `effigy ci:web`, `effigy docs:check`
- live Svelte and React review of the nine changed pages
- `git diff --check origin/main...HEAD`
