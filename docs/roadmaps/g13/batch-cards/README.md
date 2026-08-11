# g13 Batch Cards

Status: active

Batch cards are the worker execution surface under the g13 roadmaps. The
orchestrator owns this index, card status, the dispatch ledger, review, merge,
and roadmap closeout.

## g13.001 Batches

| Batch | Purpose | Dependency | Status |
| --- | --- | --- | --- |
| `001` | authority inventory and inherited docs-baseline repair | none | worker complete — review pending |
| `002` | pilot fixture freeze and quantitative baseline | `001` merged | planned |
| `003` | crate placement ruling and g13.002 handoff | `001`–`002` merged | planned |
| `004` | Rust IR prior-art and failure-mode research | none | dead — recompile required |
| `005` | pilot contract expressiveness corpus | none | worker complete — review pending |

Worker output for `001` and `005` is pushed but remains untrusted until
orchestrator review and merge. Later cards remain planning placeholders until
their dependencies are reviewed.

- [Batch 001](001-authority-inventory-and-docs-baseline.md)
- [Batch 002](002-pilot-fixture-and-metrics-freeze.md)
- [Batch 003](003-crate-placement-ruling-and-schema-handoff.md)
- [Batch 004](004-rust-ir-prior-art-and-failure-audit.md)
- [Batch 005](005-pilot-contract-expressiveness-corpus.md)
