# g06.006 — Style Descriptor Intermediate Representation

Status: Completed
Updated: 2026-03-14

## Objective

Define a resolved style descriptor that captures the visual properties of a
component instance after token resolution but before renderer-specific
translation.

## Deliverables

New crate: `poodle-style` at `packages/contracts/style/`

### Types

| Type | Purpose |
|------|---------|
| `StyleDescriptor` | Complete resolved visual style — colors, border, shadow, typography, opacity, cursor, layout, focus ring |
| `FontFamily` | Sans / Mono enum |
| `TypographyDescriptor` | Resolved font properties (family, size, line_height, weight) |
| `BorderDescriptor` | Width + color for border |
| `CornerRadii` | Per-corner radius values |
| `CursorHint` | Pointer behavior hint (Default, Pointer, Text, NotAllowed, etc.) |

### Design

- Uses `ColorValue` and `ShadowValue` from `poodle-tokens::typed` for numeric
  color/shadow values
- Uses `LayoutIntent` from `poodle-layout` for layout properties
- All dimension values are resolved `f32` pixels
- Builder pattern for construction
- Query methods: `is_visible()`, `has_focus_ring()`

### Dependencies

- `poodle-tokens` — typed color and shadow values
- `poodle-layout` — layout intent types

## Verification

- [x] `poodle-style` crate compiles
- [x] 4 tests pass covering defaults, builder composition, visibility, typography
- [x] Typed token re-export added to `poodle-tokens` lib.rs
