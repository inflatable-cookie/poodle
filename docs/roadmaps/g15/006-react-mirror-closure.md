# g15.006 — React Mirror Closure

Status: **blocked** — pending orchestrator review of `g15.001`
Depends on: `g15.001` (measured gaps), mirrors the Svelte focused-evidence
tranches `g15.002`–`g15.005`
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../contracts/001-working-rules.md`

## Outcome

Close every measured React mirror gap through the existing shared web CSS and
framework-free behaviour substrate. React stays tightly paired with Svelte;
the Svelte implementation remains the reference when runtimes drift. A React
gap is not a Svelte-denominator blocker, and closing it does not change any
Svelte surface.

## Scope

Measured gaps from the register:

- implementation + export missing: AgentPlan, AgentPlanRecord
- gallery specimen missing: AgentMessage, AgentPlan, AgentPlanRecord,
  ChangedFiles, ToolCall, ToolCallGroup
- focused React test gaps mirroring the Svelte tranches: every component
  whose Svelte counterpart gains focused evidence in `g15.002`–`g15.005`
  should mirror the same contract cases on the React side

## Goals

- [ ] Add the two missing React implementations and exports without changing
      Svelte behaviour.
- [ ] Add the six missing React gallery specimens (React counterparts of the
      Svelte pages; structure/copy agrees per the working rules).
- [ ] Mirror focused React test evidence for the components covered by
      `g15.002`–`g15.005`.
- [ ] Keep the shared web substrate in `poodle-core` unchanged unless a
      measured defect is found; any change there is a separate reviewed
      change.

## Acceptance

- [ ] React `exports` map and index cover the full 175 roster.
- [ ] React gallery covers all 175 components.
- [ ] `effigy react:build` and `effigy test:components` pass.
- [ ] No Svelte component, contract, or specimen changed.

## Stop Conditions

- Work starts on a new shared corpus, comparator, or parity authority.
- A React implementation diverges from Svelte semantics without a contract
  note.
- The mirror work expands beyond the measured gaps without a new card.

## Writable Scope

- React implementations, gallery specimens, and focused tests
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `effigy react:build`
- `effigy test:components`
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
