# g13 Batch Cards

Status: active

Batch cards are the worker execution surface under the g13 roadmaps. The
orchestrator owns this index, card status, the dispatch ledger, review, merge,
and roadmap closeout.

## g13.001 Batches

| Batch | Purpose | Dependency | Status |
| --- | --- | --- | --- |
| `001` | authority inventory and inherited docs-baseline repair | none | merged (`251cc858`) |
| `002` | pilot fixture freeze and quantitative baseline | `001` merged | merged (`89debbcb`) |
| `003` | crate placement ruling and g13.002 handoff | `001`–`002` merged | ready to compile — dependencies met |
| `004` | Rust IR prior-art and failure-mode research | none | dead — recompile required |
| `005` | pilot contract expressiveness corpus | none | merged (`2f8dc5db`) |
| `006` | button-family tone parity (CSS/specimens) | contract amendments `282ce489` | ready |
| `007` | value-domain drift inventory (report-only) | contract amendments `282ce489` | ready |

`001`, `005`, and `002` were reviewed, validated, and merged on 2026-08-11;
evidence in `docs/logs/2026-08/11-g13-b001-b005-review-and-merge.md`. All of
`003`'s dependencies are now met, but it is a maintainer-judgment card: it owns
the crate-placement ruling plus the two open contract questions carried out of
`005` (`UNKNOWN-01` embedded RangeSlider `aria-orientation`, `UNKNOWN-02` Rust
Button `Danger`/`Success`). Those are not worker decisions — the orchestrator
must rule before `003` can be compiled into a dispatchable card.

- [Batch 001](001-authority-inventory-and-docs-baseline.md)
- [Batch 002](002-pilot-fixture-and-metrics-freeze.md)
- [Batch 003](003-crate-placement-ruling-and-schema-handoff.md)
- [Batch 004](004-rust-ir-prior-art-and-failure-audit.md)
- [Batch 005](005-pilot-contract-expressiveness-corpus.md)
- [Batch 006](006-button-family-tone-parity.md)
- [Batch 007](007-value-domain-drift-inventory.md)
