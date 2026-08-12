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
| `013` | Tabs variant consolidation (side-quest) | maintainer ruling | merged (`0b347022`) |
| `014` | AppHeader element access (side-quest) | maintainer ruling | merged (`b47a2580`) |
| `015` | deterministic emission + drift-gating research (codegen precursor) | supersedes `004` | merged (`7878c537`) |
| `016` | shared `ActiveFill` type + NavigationMenu switches | Tabs switches merged | merged (`95c78a61`) |
| `017` | AppHeader centre region | `014` merged | merged (`e63c7f35`) |
| `018` | Tabs `bordered` defaults to false | — | retired — folded into `020` |
| `019` | HistoryCenter — web reference | none | merged (`0c31f2f9`) — superseded by v2, then v3 |
| `020` | ActiveEdge + strip consolidation + Tabs defaults | `016` merged | merged (`2115e5c1`) |
| `021` | `ActiveFill` gains `none` (strip equivalence) | `020` merged | merged (`239e9776`) |
| `022` | `poodle-codegen` emitter core (**g13.003**) | `b011`/`b012`/`b015` merged | merged (`143c63a1`) |
| `023` | HistoryCentre v2 — tree stitcher and machine | `019` merged | merged (`0e1bb49a`) — **model superseded by v3** |
| `024` | HistoryCentre v2 — web rendering and lanes | `023` | merged (`bd341ad2`) — **superseded by v3** |
| `025` | codegen remaining emitters — closes `g13.003` | `022` merged | merged (`5d9edc9d`) |
| `026` | `dismissOnOutsideInteract` across the overlay family | none | merged (`4418eb58`) |
| `027` | dismiss native parity + bidirectional drift gate | `026` merged | merged (`2eb94a16`) |
| `028` | HistoryCentre v3 — flat list, node-owned forks (core) | `024` merged | merged (`680f3c64`) |
| `029` | HistoryCentre v3 — rendering, picker, nested list | `028` | merged (`16b68e61` → `2a6d3af9`) |
| — | HistoryCentre — native parity | `029` merged | planned (renumbered; `022` was reused for codegen) |

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
- [Batch 023](023-history-center-tree-core.md)
- [Batch 024](024-history-center-tree-web.md)
- [Batch 025](025-codegen-remaining-emitters.md)
- [Batch 026](026-dismiss-on-outside-interact-prop.md)
- [Batch 027](027-dismiss-native-parity-and-drift-direction.md)
- [Batch 028](028-history-center-v3-core.md)
- [Batch 029](029-history-center-v3-web.md)
