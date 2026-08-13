# 003 Frozen Baseline And Inventories

Status: ready
Milestone: `g14.002`
Owner: Poodle core
Branch: `thread/g14-003-frozen-baseline-and-inventories`
Depends on: none (measurement only; runs parallel to `g14-b002`)
Governing refs:
`docs/roadmaps/g14/002-frozen-baseline-and-inventories.md`,
`docs/roadmaps/g13/native-registration-gap.md`,
`docs/roadmaps/g13/batch-cards/047-machine-shape-consolidation.md`,
`docs/roadmaps/g13/authority-inventory.md`

## Goal

Freeze the numbers every g14 mechanism is measured against. The g13 pilot
succeeded on evidence because a baseline was frozen first; g14 repeats
that. Measurement only — fix nothing.

## Deliverables

- **Native registration gap re-measured.** Run the refresh script from
  `native-registration-gap.md`; record per-runtime missing counts and the
  table.
- **Machine inventory re-measured.** Re-derive b047's buckets, not copied:
  the 21 machines in both runtimes, canonical vs off-pattern, pinned vs
  unpinned, whole-surface vs thin vectors. Note anything 053 changed.
- **Drift-gate inventory.** Every gate named, with what it covers and what
  it cannot see. Include the known-red: `docs:contract-role-drift` fails
  today on range-slider (`g14-b002` owns the fix) — record it as the
  before-state.
- **Parity evidence state.** Which components have live specimen evidence
  per runtime, which do not.
- **Specimen inventory.** Hand-written specimen surfaces per runtime, per
  component — static vs interactive classified, sizes measured. This is
  the before-state for `g14.003`.
- `docs/roadmaps/g14/g14-baseline-manifest.md` storing all of it, numbers
  not claims.

## Acceptance

- [ ] Every number carries a re-run date and the command that produced
  it.
- [ ] The manifest names at least one hole class the pinning stack must
  catch that the existing gates cannot (behaviour divergence between the
  two machine implementations, docs-silent implementation drift, or
  similar — name the class from the measurements, not from this list).
- [ ] Nothing is fixed. A defect found while measuring goes in the log as
  a finding with a suggested owner, not a patch.

## Stop Conditions

- A script named by an inventory doc no longer exists or no longer runs —
  report the exact break instead of inventing a replacement number.

## Writable Paths

- `docs/roadmaps/g14/g14-baseline-manifest.md` (new)
- `docs/logs/2026-08/14-g14-003-frozen-baseline-and-inventories.md`
- `PAPERCUTS.md` (append only)

## Steps

1. Reset per the Thread Reuse Protocol.
2. Read the governing refs and the four inventory sources.
3. Produce each measurement; store raw outputs alongside the manifest
   where they are reproducible.
4. Write the manifest and the log, findings included.
5. Validate: `git diff --check`. No code gates — nothing executable
   changes.
6. Push with `git push -u origin thread/g14-003-frozen-baseline-and-inventories`.
   Do not merge.
