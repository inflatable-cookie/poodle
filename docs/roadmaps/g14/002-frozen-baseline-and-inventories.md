# g14.002 Frozen Baseline And Inventories

Status: planned
Owner: Poodle core
Depends on: `g14.001`
Governing refs: `../g13/002-rust-ir-schema-and-validation-core.md` (baseline
precedent), `../g13/native-registration-gap.md`, `../g13/batch-cards/047-machine-shape-consolidation.md`

## Objective

Freeze the numbers every g14 mechanism is measured against. The g13 pilot
succeeded on evidence because `g13.002` froze a baseline first; g14 repeats
that.

## Deliverables

- Native registration gap re-measured: components missing from each native
  registry, with the refresh script from `native-registration-gap.md` run
  and recorded.
- Machine inventory re-measured: 21 machines in both runtimes, which are
  canonical, which are unpinned, which exercise their whole surface
  (b047's buckets re-derived, not copied).
- Drift-gate inventory: the 13 gates named with what each covers and what
  it cannot see (docs-silent implementation drift, behaviour divergence).
- Parity evidence state: which components have live specimen evidence per
  runtime, which do not.
- Specimen inventory: hand-written specimen surfaces per runtime, per
  component — static vs interactive classified, sizes measured. This is
  the before-state for `g14.003`.
- `g14-baseline-manifest.md` storing all of it, numbers not claims.

## Acceptance

- [ ] Every number carries a re-run date and command.
- [ ] The baseline names at least one hole class the pinning stack must
  catch that the existing gates cannot.
- [ ] No inventory work fixes anything; this is measurement only.

## Next

`g14.003` starts the specimen migration; `g14.004` starts the pinning
stack. Both read the machine inventory.
