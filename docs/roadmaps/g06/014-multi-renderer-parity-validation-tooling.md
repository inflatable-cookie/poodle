# g06.014 — Multi-Renderer Parity Validation Tooling

Status: Completed
Updated: 2026-03-14

## Objective

Validate that the shared contract layer provides complete coverage for both
rendering targets (GPUI and Jetstream).

## Validation Report

Published at `docs/specs/multi-renderer-parity-validation.json`.

### Spec Coverage

| Layer | Specs | Status |
|-------|-------|--------|
| Primitives | 64 | Full coverage (42 existing + 22 new in g06.008–010) |
| Composites | 41 | Full coverage (16 existing + 25 new in g06.011–012) |
| Workstation | 13 | Unchanged (carried from g05) |
| **Total** | **118** | 124 Svelte components mapped (6 via spec configuration) |

### Contract Infrastructure

| Crate | Status | Verification |
|-------|--------|-------------|
| `poodle-tokens` | Typed module added | Colors, spaces, shadows, durations all parse correctly |
| `poodle-layout` | Complete | All layout properties map to both targets |
| `poodle-events` | Complete | All semantic events map to both targets |
| `poodle-style` | Complete | StyleDescriptor covers all visual properties |
| `poodle-adapter` | Complete | All trait definitions published |

### Known Deltas (Intentional)

| Delta | Reason |
|-------|--------|
| `overflow_x: Scroll` → Hidden in Jetstream | Vertical scroll only |
| `Hovered` event not emitted by Jetstream | Gamepad/touch primary input |
| Grid layouts → nested flexbox in Jetstream | No CSS Grid support |
| Rich text → plain text in Jetstream | Single-style text runs only |
| Gradients → solid color fallback | No gradient support |

### Deferred to g07/g08

- Actual `RenderComponent<Spec>` implementations for each spec
- `AdapterManifest` implementations declaring supported/unsupported components
- Compile-time verification that all specs have adapter implementations

## Verification

- [x] Parity validation report published
- [x] All 118 specs compile across 3 contract crates
- [x] 45 existing tests pass (29 primitives + 10 composites + 6 workstation)
- [x] All 4 new infrastructure crates compile with tests passing
