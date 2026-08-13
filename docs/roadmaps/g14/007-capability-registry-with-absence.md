# g14.007 Capability Registry With Absence

Status: planned
Owner: Poodle core
Depends on: `g14.001` (doctrine), `g14.002` (gap baseline)
Governing refs: `../g13/pilot-verdict-evidence.md` §4.3 and finding 5,
`../g13/018-capability-and-anatomy-amendments.md`,
`../../contracts/001-working-rules.md` (§Runtime Parity Authority)

## Objective

Make capability absence declarable and gated. The pilot's closest
stop-condition finding: Jetstream renders a text field nobody can type
into, declared identically to GPUI, which implements the whole editing
model. Absence was untyped prose. g13.018 added per-runtime capability
expression including absence; this milestone makes it the durable
mechanism and gates on it.

## Deliverables

- The per-runtime capability registry, including absence, promoted from
  the g13.018 amendment into the stable surface.
- A gate that fails when a runtime silently lacks a declared capability or
  a capability is declared nowhere.
- Every capability either exercised in parity evidence or declared absent
  with a reason — no third state.
- The Jetstream caret case resolved: implemented, or declared absent with
  the observable consequence recorded in the contract's runtime notes.

## Acceptance

- [ ] No capability absent from a runtime without a declared reason.
- [ ] The gate fails on a planted silent absence and passes clean at rest.
- [ ] Native holes found by this gate are routed to `g14.008`, not logged
  and forgotten.

## Next

`g14.008` closes the native registration gap on the `poodle-render` path.
