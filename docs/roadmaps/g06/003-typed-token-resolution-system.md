# g06.003 — Typed Token Resolution System

Status: Completed
Updated: 2026-03-14

## Objective

Extend the token build pipeline to emit typed Rust artifacts alongside the
existing string constants, enabling Jetstream and other numeric-value consumers
to use resolved token values directly without runtime parsing.

## Changes

### Build Pipeline Extension (`build-tokens.ts`)

Added parsing functions that classify each token value and emit typed constants:

- **Colors** (`#rrggbb`, `rgba(r,g,b,a)`) → `ColorValue(f32, f32, f32, f32)`
  with values in 0.0–1.0 range, compatible with Jetstream's `Vec4`
- **Dimensions** (`Xrem`, `Xpx`) → `SpaceValue(f32)` in pixels (base: 16px/rem)
- **Durations** (`Xms`) → `DurationValue(f32)` in milliseconds
- **Shadows** (`ox oy blur color`) → `ShadowValue { offset_x, offset_y, blur, color }`
- **Pure numbers** (`400`, `0.64`) → `f32`
- **Strings** (font families, easings) → `&str` (no typed equivalent)

### Generated Artifacts

New `typed/` subdirectory in `artifacts/rust/`:

| File | Contents |
|------|----------|
| `typed/mod.rs` | Module declarations and re-exports |
| `typed/types.rs` | `ColorValue`, `SpaceValue`, `DurationValue`, `ShadowValue` structs |
| `typed/primitives.rs` | Typed constants for all primitive tokens |
| `typed/semantic.rs` | Typed constants for all semantic tokens |

### Type Definitions

```rust
struct ColorValue(pub f32, pub f32, pub f32, pub f32);  // RGBA 0.0–1.0
struct SpaceValue(pub f32);                               // Pixels
struct DurationValue(pub f32);                            // Milliseconds
struct ShadowValue { offset_x, offset_y, blur: f32, color: ColorValue }
```

Each type includes `const fn` constructors and accessor methods.

### Backward Compatibility

String constants in `primitives.rs` and `semantic.rs` are unchanged. The typed
module is additive — existing consumers continue to work unmodified.

## Verification

- [x] Token build pipeline runs without errors
- [x] All 74 semantic tokens classified and emitted as typed constants
- [x] All 79 primitive tokens classified and emitted as typed constants
- [x] Shadow values correctly parse `0 0.25rem 0.75rem rgba(...)` format
- [x] Color values correctly parse both `#hex` and `rgba()` formats
- [x] `pug-tokens` crate compiles (warnings for unused items expected)
- [x] All 45 downstream tests pass across primitives/composites/workstation
