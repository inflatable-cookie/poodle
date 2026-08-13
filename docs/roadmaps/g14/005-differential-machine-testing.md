# g14.005 Differential Machine Testing

Status: planned
Owner: Poodle core
Depends on: `g14.004`
Governing refs: `../../architecture/006-headless-core-and-machine-model.md`
(machines are pure functions), `../g11/006-rust-headless-mirror.md`,
`packages/contracts/headless/vectors/`

## Objective

Replace "each implementation against prose" with "the two implementations
against each other." Machines are pure:
`transition(state, context, event) → (state, context, effects)`. Identical
event traces through the TS and Rust machines must produce identical
results. Prose contracts define intent; this defines equivalence, by
execution.

## Deliverables

- A harness that runs one trace through both implementations and diffs
  state, context, and effects, with a normalization layer for payload
  representation differences.
- Hand-authored semantic traces per machine, written from the contract's
  Behavior Machine section.
- Generated traces: bounded exhaustive over small event alphabets, seeded
  random beyond that. Traces cover what nobody thought to assert.
- Invariant checks (e.g. lower ≤ upper for sliders) run at every step of
  every trace.
- Wired into CI: a planted divergence in either implementation fails,
  naming the machine, the trace, and the diverging step.

## Acceptance

- [ ] Planted divergence caught in both directions (TS-only and Rust-only).
- [ ] Every machine with a Rust mirror runs in the harness.
- [ ] A divergence that cannot be normalized is reported as a finding
  (likely a capability gap for `g14.007`), not papered over.

## Next

`g14.006` makes vector coverage a gate and deepens the thin vectors.
