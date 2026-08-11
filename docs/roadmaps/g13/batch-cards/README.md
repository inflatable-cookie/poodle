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
| `003` | crate placement ruling and g13.002 handoff | `001`–`002` merged | complete — rulings recorded |
| `004` | Rust IR prior-art and failure-mode research | none | retired — superseded by `015` |
| `005` | pilot contract expressiveness corpus | none | merged (`2f8dc5db`) |
| `006` | button-family tone parity (CSS/specimens) | contract amendments `282ce489` | merged (`22337a31`) — Button + IconButton; SplitButton deferred to `008` |
| `007` | value-domain drift inventory (report-only) | contract amendments `282ce489` | merged (`8090521c`) |
| `008` | split-button tone parity + primary status shadow | `006` merged + shadow ruling | merged (`f59adac0`) |
| `009` | dialog initial-focus hook (side-quest) | `b1a4a5e7` | merged (`e4af527e`) |
| `010` | TextInput focus parity: autofocus + focus() to React | `b1a4a5e7` | merged (`b2aa40ae`) |
| `011` | `poodle-ir` schema core (**g13.002**) | `003` rulings | merged (`4a22c8d8`) |
| `012` | IR bounded expression vocabulary | `011` merged | merged (`911fdfd8`) |
| `013` | Tabs variant consolidation (side-quest) | maintainer ruling | dispatched |
| `014` | AppHeader element access (side-quest) | maintainer ruling | dispatched |
| `015` | deterministic emission + drift-gating research (codegen precursor) | supersedes `004` | merged (`7878c537`) |
| `016` | shared `ActiveFill` type + NavigationMenu switches | Tabs switches merged | dispatched |
| `017` | AppHeader centre region | `014` merged | dispatched |
| `018` | Tabs `bordered` defaults to false | — | retired — folded into `020` |
| `019` | HistoryCenter — web reference | none | dispatched |
| `020` | ActiveEdge + strip consolidation + Tabs defaults | `016` merged | merged (`2115e5c1`) |
| `021` | `ActiveFill` gains `none` (strip equivalence) | `020` merged | dispatched |
| `022` | HistoryCenter — native parity | `019` merged | planned |

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
- [Batch 008](008-split-button-tone-parity.md)
- [Batch 009](009-dialog-initial-focus.md)
- [Batch 010](010-text-input-focus-parity.md)
- [Batch 011](011-poodle-ir-schema-core.md)
- [Batch 012](012-ir-expression-vocabulary.md)
- [Batch 013](013-tabs-variant-consolidation.md)
- [Batch 014](014-app-header-element-access.md)
- [Batch 015](015-deterministic-emission-and-drift-gating.md)
- [Batch 016](016-active-fill-shared-type-and-navigation-menu.md)
- [Batch 017](017-app-header-center-region.md)
- [Batch 018](018-tabs-bordered-default-false.md)
- [Batch 019](019-history-center-web-reference.md)
- [Batch 020](020-tabs-active-edge-and-strip-consolidation.md)
- [Batch 021](021-active-fill-none.md)
- [Batch 022](022-poodle-codegen-emitter-core.md)
