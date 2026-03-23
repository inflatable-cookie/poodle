# Research To Architecture Crossref

Status: active
Updated: 2026-03-11

## Active Research → Architecture

This document tracks which research outputs are intended to inform which architecture documents.

| Research Document | Target Architecture | Status | Translation Memo |
|-------------------|---------------------|--------|------------------|
| [hub-gpui](./source-hubs/hub-gpui.md) | [architecture/001-flint-system-shape.md](../architecture/001-flint-system-shape.md) | 🟢 complete | [tm-token-system](./translation-memos/tm-token-system.md) |
| [hub-bits](./source-hubs/hub-bits.md) | [architecture/001-flint-system-shape.md](../architecture/001-flint-system-shape.md) | 🟢 complete | [tm-svelte-substrate](./translation-memos/tm-svelte-substrate.md) |
| [tk-design-token-systems](./value-tracks/tk-design-token-systems.md) | Token system design | 🟢 complete | [tm-token-system](./translation-memos/tm-token-system.md) |
| [tk-cross-framework-contracts](./value-tracks/tk-cross-framework-contracts.md) | Contract template | 🟢 complete | [tm-contract-template](./translation-memos/tm-contract-template.md) |

## Architecture → Research Dependencies

This documents which architecture decisions depend on research.

| Architecture Decision | Blocking Research | Status |
|-----------------------|-------------------|--------|
| Token schema (g01.002) | hub-gpui, tk-design-token-systems | 🟢 **unblocked** - see [tm-token-system](./translation-memos/tm-token-system.md) |
| Token artifact emission (g01.003) | tk-design-token-systems | 🟢 **unblocked** - see [tm-token-system](./translation-memos/tm-token-system.md) |
| Component contract template (g01.004) | tk-cross-framework-contracts | 🟢 **unblocked** - see [tm-contract-template](./translation-memos/tm-contract-template.md) |
| Svelte substrate policy (g01.005) | hub-bits, tk-svelte-headless-patterns | 🟢 **unblocked** - see [tm-svelte-substrate](./translation-memos/tm-svelte-substrate.md) |
| GPUI substrate policy (g01.006) | hub-gpui, tk-gpui-idioms | 🟢 **unblocked** - hub-gpui complete |

## Translation Memos Ready for Architecture Update

| Memo | Decisions | Target Architecture |
|------|-----------|---------------------|
| [tm-token-system](./translation-memos/tm-token-system.md) | DTCG format, 3-layer taxonomy, Style Dictionary 4.0, Rust emission | Token system section |
| [tm-contract-template](./translation-memos/tm-contract-template.md) | Contract-first workflow, 12-section template, 3-tier parity | Component contract section |
| [tm-svelte-substrate](./translation-memos/tm-svelte-substrate.md) | Bits UI as substrate, wrapper pattern, contract ownership | Svelte implementation section |

## Next Task

1. Review translation memos (all three)
2. Update architecture/001-flint-system-shape.md with research decisions
3. Begin g01.002 token schema implementation
