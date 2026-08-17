# g15.010 — Display, Workstation & Agent GPUI Specimen Closure

Status: **blocked** — orchestration hold; `g15.007` is next
Depends on: `g15.001` (measured gaps); GPUI-only closure after the native
tranches `g15.007`–`g15.009` land their families
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../roadmaps/g14/conformance-estate.md`, `../../contracts/001-working-rules.md`

## Outcome

Add the missing GPUI specimens for the 18 components that already have Rust
declarations and render implementations but no GPUI specimen. This is the
remaining measured GPUI surface after the Licence, model-connection, update,
settings, and Radio families move through their own tranches. Specimens teach
the same component the Svelte page teaches; they are not parity snapshots.

## Scope

Avatar, Callout, RemediationBanner, MetaItem, Pill, Spinner, EmptyState,
StateTile, ActionDiscoveryPanel, DockRegion, AgentMessage, AgentPlan,
AgentPlanRecord, AgentQuestionRecord, AgentSubagent, ChangedFiles, ToolCall,
ToolCallGroup

## Execution Plan

- [ ] **Batch A — display & status specimens (8):** Avatar, Callout,
      RemediationBanner, MetaItem, Pill, Spinner, EmptyState, StateTile
- [ ] **Batch B — workstation specimens (2):** ActionDiscoveryPanel,
      DockRegion (DockRegion may reuse the retained headless driver)
- [ ] **Batch C — agent specimens (8):** AgentMessage, AgentPlan,
      AgentPlanRecord, AgentQuestionRecord, AgentSubagent, ChangedFiles,
      ToolCall, ToolCallGroup

## Goals

- [ ] One GPUI specimen per scoped component, composed from the existing
      `poodle-render` output through the node backend.
- [ ] Focused headless cases where the specimen exercises interaction;
      infrastructure-only proofs do not count as component evidence.
- [ ] DockRegion specimen may reuse the retained headless driver; no new
      shared corpus or registry beyond the generated catalogue.

## Acceptance

- [ ] Every scoped component has a named GPUI specimen file in the preview
      catalogue.
- [ ] `effigy check:gpui`, `effigy regressions:native` pass.
- [ ] The roster's GPUI column flips to evidence-present for all 18.

## Stop Conditions

- A portable interface, shared corpus, or comparator reappears under a new
  name.
- A specimen is made exhaustive to prove parity.
- Work expands beyond the 18 scoped components without a new card.

## Writable Scope

- GPUI specimen files and focused headless tests
- bounded contract-first fixes to scoped defects the new evidence exposes
- `release-baseline-roster.md` and `release-gap-register.md` (GPUI rows only,
  no status lines)
- one August batch log under `docs/logs/2026-08/`
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `effigy check:gpui`
- `effigy regressions:native`
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
