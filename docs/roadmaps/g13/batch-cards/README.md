# g13 Batch Cards

Status: active

Batch cards are the worker execution surface under the g13 roadmaps. The
orchestrator owns this index, card status, the dispatch ledger, review, merge,
and roadmap closeout.

## g13.001 Batches

| Batch | Purpose | Dependency | Status |
| --- | --- | --- | --- |
| `001` | authority inventory and inherited docs-baseline repair | none | merged (`251cc858`) |
| `002` | pilot fixture freeze and quantitative baseline | `001` merged | ready |
| `003` | crate placement ruling and g13.002 handoff | `001`–`002` merged | planned |
| `004` | Rust IR prior-art and failure-mode research | none | dead — recompile required |
| `005` | pilot contract expressiveness corpus | none | merged (`2f8dc5db`) |

`001` and `005` were reviewed, validated, and merged on 2026-08-11; evidence in
`docs/logs/2026-08/11-g13-b001-b005-review-and-merge.md`. `002` is now
dispatchable. `003` stays a planning placeholder until `002` merges; it owns the
crate-placement ruling plus the two open contract questions carried out of `005`
(`UNKNOWN-01` embedded RangeSlider `aria-orientation`, `UNKNOWN-02` Rust Button
`Danger`/`Success`).

- [Batch 001](001-authority-inventory-and-docs-baseline.md)
- [Batch 002](002-pilot-fixture-and-metrics-freeze.md)
- [Batch 003](003-crate-placement-ruling-and-schema-handoff.md)
- [Batch 004](004-rust-ir-prior-art-and-failure-audit.md)
- [Batch 005](005-pilot-contract-expressiveness-corpus.md)
