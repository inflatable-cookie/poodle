# g15.006 — React Mirror Implementation and Gallery Closure

Status: **blocked** — orchestration hold; `g15.003` is the active card
Depends on: `g15.001` (measured gaps). Focused React test evidence for the
rest of the roster is paired into the Svelte evidence tranches
(`g15.002`–`g15.005`) so the mirror never drifts from the reference.
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../contracts/001-working-rules.md`

## Outcome

Close the measured React mirror gaps that are implementation and catalogue
work, not evidence work: the two missing React implementations/exports and
the six missing React gallery specimens. React stays tightly paired with
Svelte through the shared web CSS and framework-free behaviour substrate; the
Svelte implementation remains the reference when runtimes drift. A React gap
is not a Svelte-denominator blocker, and closing it does not change any
Svelte surface.

## Scope

Measured gaps from the register:

- implementation + export missing: AgentPlan, AgentPlanRecord
- gallery specimen missing: AgentMessage, AgentPlan, AgentPlanRecord,
  ChangedFiles, ToolCall, ToolCallGroup

Focused React tests for these components follow the same evidence threshold as
`g15.002`–`g15.005` (minimum one load-bearing contract case per component).
All other focused React evidence lands paired inside `g15.002`–`g15.005`.

## Execution Plan

- [ ] **Batch A — implementations and exports:** React counterparts of
      AgentPlan and AgentPlanRecord (idiomatic thin shells over the shared
      web substrate), matching Svelte semantics and the contract props
      tables.
- [ ] **Batch B — gallery specimens:** AgentMessage, AgentPlan,
      AgentPlanRecord, ChangedFiles, ToolCall, ToolCallGroup pages in the
      React gallery, structure/copy agreeing with the Svelte pages per the
      working rules.
- [ ] **Batch C — focused evidence:** one load-bearing test case per scoped
      component on the React side (same cases the Svelte side asserts).

## Goals

- [ ] React `exports` map and index cover the full 175 roster.
- [ ] React gallery covers all 175 components.
- [ ] Every scoped component has a named focused React test case.
- [ ] Keep the shared web substrate in `poodle-core` unchanged unless a
      measured defect is found; any change there is a separate reviewed
      change.

## Acceptance

- [ ] `effigy react:build` and `effigy test:components` pass.
- [ ] The register's React rows flip to evidence-present; the roster's React
      cells name the new files.
- [ ] No Svelte component, contract, or specimen changed.

## Stop Conditions

- Work starts on a new shared corpus, comparator, or parity authority.
- A React implementation diverges from Svelte semantics without a contract
  note.
- The mirror work expands beyond the measured gaps without a new card.

## Writable Scope

- React implementations, gallery specimens, and focused tests
- `release-baseline-roster.md` and `release-gap-register.md` (React rows only,
  no status lines)
- one August batch log under `docs/logs/2026-08/`
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `effigy react:build`
- `effigy test:components`
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
