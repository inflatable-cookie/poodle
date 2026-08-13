# g14.006 Vector Completeness And Depth

Status: planned
Owner: Poodle core
Depends on: `g14.002` (baseline buckets); runs alongside `g14.004`/`g14.005`
Governing refs: `../g13/batch-cards/047-machine-shape-consolidation.md`
(R4, R5), `../g13/pilot-verdict-evidence.md` §4.4,
`packages/contracts/headless/vectors/machines.json`

## Objective

Make "pinned" mean covered. b047 measured 4 of 13 inventoried machines
exercise their whole surface; b045 found the slider vector has zero
two-thumb cases. Being listed in `machines.json` is not coverage. This
milestone turns that from a finding into a floor.

## Deliverables

- A completeness gate: a machine's vector must exercise every state,
  transition, and effect. A thin vector fails CI.
- Deepen the known-thin set first: dismissal-event vectors for `menu` and
  `popover` (CLOSE / ESCAPE / OUTSIDE_INTERACT, initialFocus strategies —
  PAPERCUTS 2026-08-13), `slider` two-thumb, `text` (absent entirely).
- Pin the nine unpinned machines, both implementations running each vector.
- A vector that fails on first run is a divergence finding reported, not
  tuned away (b047 R4 carried forward).

## Acceptance

- [ ] The gate fails on each known-thin vector and passes clean at rest.
- [ ] Every duplicated machine runs a vector that exercises its whole
  surface on both implementations.
- [ ] No behaviour changes; this is coverage, not logic.

## Next

Feeds `g14.005`'s hand-authored traces and `g14.010`'s reassessment.
