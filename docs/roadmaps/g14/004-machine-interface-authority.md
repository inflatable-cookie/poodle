# g14.004 Machine Interface Authority

Status: planned
Owner: Poodle core
Depends on: `g14.002`
Governing refs: `../../architecture/006-headless-core-and-machine-model.md`,
`../g11/002-headless-machine-spec-format-and-pilot-contracts.md`,
`packages/contracts/headless/vectors/machines.json`

## Objective

Make the machine contract — states, events, effects, context types — one
schema that generates the TypeScript and Rust type declarations. Interface
only; no transitions, guards, or derivation. This is the define-once wish
scoped to where the g13 pilot proved sharing is cheap: interfaces swap 1:1
with existing hand-written types, so an event mismatch becomes a compile
error in both languages and b047's shape drift dies structurally.

## Deliverables

- Extend `machines.json` (or a sibling schema) into the authoritative
  machine-interface schema.
- One emitter generating TS and Rust machine interfaces; deterministic,
  drift-checked like `ir:check`.
- Replace hand-written machine type declarations, one machine at a time,
  pilot first: `hover`, `menu`, `modal`, `popover` (the four canonical),
  then the remaining 17.
- A gate that fails when a machine's interface is not generated from the
  schema.

## Acceptance

- [ ] The four pilot machines compile in both runtimes from generated
  interfaces with no public export renamed.
- [ ] A planted interface divergence fails the gate on the authored side,
  naming the machine.
- [ ] Generated interfaces are smaller than the declarations they replace.
- [ ] A machine whose shape cannot be expressed is reported as a finding,
  not absorbed by widening the schema into behaviour.

## Next

`g14.005` builds the differential harness on top of the generated
interfaces.
