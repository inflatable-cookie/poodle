# g14.010 Reassess And Consolidate

Status: planned
Owner: Poodle maintainer
Depends on: `g14.003`–`g14.009`

## Objective

Measure whether the pinning stack earned its keep, with the same honesty
the g13 pilot verdict used. The mechanisms are individually cheap, but the
stack as a whole must be judged against the baseline, not against the
momentum of having built it.

## Deliverables

- Re-measurement against `g14.002`: drift caught per mechanism, native
  holes closed, per-machine coverage, corpus LOC delta.
- A named accounting of what each mechanism did **not** catch, including
  any hole class the stack missed that a future generation must cover.
- A verdict on the durable pinning stack: which mechanisms are promoted
  into stable architecture, which are retired, which need revision.
- Closeout of the generation: front doors refreshed, stale specs purged,
  one named first task for the next programme.

## Acceptance

- [ ] The verdict names tradeoffs and failed assumptions, not only green
  checks.
- [ ] Nothing provisional is left promoted into stable architecture.
- [ ] The drift the user was sweeping for is either gated or named as a
  known uncovered class.

## Next

Decided here.
