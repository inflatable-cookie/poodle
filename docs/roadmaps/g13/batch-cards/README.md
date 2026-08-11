# g13 Batch Cards

Status: active

Batch cards are the worker execution surface under the g13 roadmaps. The
orchestrator owns this index, card status, the dispatch ledger, review, merge,
and roadmap closeout.

## g13.001 Batches

| Batch | Purpose | Dependency | Status |
| --- | --- | --- | --- |
| `001` | authority inventory and inherited docs-baseline repair | none | dispatched |
| `002` | pilot fixture freeze and quantitative baseline | `001` merged | planned |
| `003` | crate placement ruling and g13.002 handoff | `001`–`002` merged | planned |
| `004` | Rust IR prior-art and failure-mode research | none | ready |
| `005` | pilot contract expressiveness corpus | none | ready |

Only `001` is worker-ready. Later cards remain planning placeholders until the
preceding evidence is reviewed.

- [Batch 001](001-authority-inventory-and-docs-baseline.md)
- [Batch 002](002-pilot-fixture-and-metrics-freeze.md)
- [Batch 003](003-crate-placement-ruling-and-schema-handoff.md)
- [Batch 004](004-rust-ir-prior-art-and-failure-audit.md)
- [Batch 005](005-pilot-contract-expressiveness-corpus.md)
