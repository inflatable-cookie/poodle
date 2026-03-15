# g06.007 — Renderer Adapter Trait Definition

Status: Completed
Updated: 2026-03-14

## Objective

Define the traits that rendering adapters implement to map Pug component specs
and resolved styles to a target renderer's native element types.

## Deliverables

New crate: `pug-adapter` at `packages/contracts/adapter/`

### Traits

| Trait | Purpose | GPUI Implementation | Jetstream Implementation |
|-------|---------|--------------------|-----------------------|
| `ThemeProvider` | Resolve token paths to typed values | GPUI theme system | Jetstream `Theme` struct |
| `EventSink` | Receive semantic events from components | GPUI event handler | Jetstream `UiEvent` handler |
| `RenderTarget` | Define the native output handle type | `AnyElement` | `UiTree` node ID |
| `RenderComponent<Spec>` | Render a spec + style to native output | Per-component GPUI element | Per-component widget |
| `AdapterManifest` | Declare supported/unsupported components | g07 manifest | g08 manifest |

### ThemeProvider Methods

- `resolve_color(token) -> ColorValue` — RGBA [f32; 4]
- `resolve_space(token) -> f32` — Pixel value
- `resolve_border_width(token) -> f32` — Pixel value
- `resolve_radius(token) -> f32` — Pixel value
- `resolve_opacity(token) -> f32` — 0.0–1.0

### Dependencies

- `pug-events` — `SemanticEvent` type for `EventSink`
- `pug-style` — `StyleDescriptor` for `RenderComponent`
- `pug-tokens` — `ColorValue` for `ThemeProvider`
- `pug-layout` — transitive via `pug-style`

## Verification

- [x] `pug-adapter` crate compiles
- [x] 3 tests pass with mock implementations of all traits
- [x] `RenderComponent<Spec>` is generic over any spec type
- [x] `AdapterManifest` supports intentional unsupported deltas
