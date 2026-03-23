# g06.015 — Generation Closeout and g07 Cutover Plan

Status: Completed
Updated: 2026-03-14

## Objective

Verify all g06 milestones are complete, document deferred items, and confirm
that g07 (GPUI build-out) and g08 (Jetstream build-out) can proceed.

## Milestone Summary

All 15 milestones completed:

| Lane | Milestones | Status |
|------|-----------|--------|
| Foundation (001–007) | Architecture audit, crate restructuring, typed tokens, layout, events, style, adapter traits | Completed |
| Expansion (008–012) | 53 new specs across 5 batches | Completed |
| Alignment (013) | Jetstream constraint document | Completed |
| Hardening (014) | Parity validation report | Completed |
| Closure (015) | This document | Completed |

## Deliverables

### New Contract Crates

| Crate | Path | Purpose |
|-------|------|---------|
| `flint-tokens` | `packages/contracts/tokens/` | Token constants + typed module |
| `flint-primitives` | `packages/contracts/primitives/` | 64 primitive specs |
| `flint-composites` | `packages/contracts/composites/` | 41 composite specs |
| `flint-workstation` | `packages/contracts/workstation/` | 13 workstation specs |
| `flint-layout` | `packages/contracts/layout/` | Layout intent types |
| `flint-events` | `packages/contracts/events/` | Semantic event types |
| `flint-style` | `packages/contracts/style/` | Style descriptor IR |
| `flint-adapter` | `packages/contracts/adapter/` | Renderer adapter traits |

### Spec Coverage

- **118 total Rust specs** (64 primitive + 41 composite + 13 workstation)
- **124 Svelte components** mapped (6 via spec configuration)
- **53 new specs** added in this generation

### Test Results

- Primitives: 29 tests passing
- Composites: 10 tests passing
- Workstation: 6 tests passing
- Layout: 4 tests passing
- Events: 5 tests passing
- Style: 4 tests passing
- Adapter: 3 tests passing
- **Total: 61 Rust tests passing**

## Deferred Items

None. All planned milestones completed.

## g07 Cutover Readiness

g07 (GPUI Rendering Build-Out) can proceed:

- [x] Shared contract crates are published with renderer-neutral naming
- [x] Typed token system provides `[f32; 4]` colors and `f32` pixel values
- [x] Layout intent types map to GPUI's styling API
- [x] Semantic events map to GPUI's event subscription model
- [x] Style descriptor provides resolved visual properties
- [x] `RenderComponent<Spec>` trait is defined for adapter implementation
- [x] `ThemeProvider` trait is defined for GPUI theme integration

## g08 Cutover Readiness

g08 (Jetstream Rendering Build-Out) can proceed (in parallel with g07):

- [x] Typed tokens emit `Vec4`-compatible colors and `f32` pixel values
- [x] Layout intent maps to Jetstream's `UiStyle` (flexbox, `Direction`, `Sizing`)
- [x] Semantic events map to Jetstream's `UiEvent` enum
- [x] Jetstream rendering constraint document published
- [x] Known deltas documented (no grid, no rich text, vertical scroll only)
- [x] `AdapterManifest` trait supports intentional unsupported component listing
- [x] Hard dependency on Jetstream g04.016 (UI Rendering Infrastructure) documented
