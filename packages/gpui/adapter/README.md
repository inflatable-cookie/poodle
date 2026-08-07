# poodle-gpui

Status: active
Updated: 2026-04-23

The canonical GPUI integration layer for Poodle. This crate bridges Poodle's
renderer-agnostic contract crates (`poodle-specs`, `poodle-tokens`,
`poodle-layout`) into the GPUI native rendering system.

## What This Crate Owns

- **`GpuiThemeProvider`** — implements the `ThemeProvider` trait; resolves
  semantic token paths to typed values (colors, spacing, radii, opacity) that
  GPUI component structs consume
- **`map_layout()`** — translates `LayoutIntent` from `poodle-layout` into a
  `GpuiStyle` for GPUI-native layout
- **`map_style()`** — translates a `StyleDescriptor` into a complete `GpuiStyle`
- **`GpuiAdapter`** / renderer-facing support matrix — adapter manifest and parity
  metadata, registering all 96 currently supported components

## What This Crate Does NOT Own

- The GPUI rendering engine — owned by `gpui`
- Component recipes — owned by `poodle-render`, which emits backend-neutral
  `poodle-node` trees; `poodle-gpui-node-backend` interprets them as GPUI
  elements (g12.019 replaced the hand-written `poodle-gpui-components` tier)
- Token definitions and values — owned by `poodle-tokens`
- Component spec structs — owned by `poodle-specs`

## Component Pipeline

```
ComponentSpec (e.g. ButtonSpec)
    + GpuiThemeProvider (token resolution)
        ↓ Button::from_spec(spec, theme)
            (GPUI component struct; implements IntoElement)
        ↓ component.into_element()
            (materializes to AnyElement → GPUI render tree → layout → draw)
```

Components are GPUI structs (not pure functions like in Jetstream). Each
component type implements `IntoElement`, which GPUI calls during the render
pass.

## `GpuiThemeProvider`

### Construction

```rust
use poodle_gpui::GpuiThemeProvider;
use poodle_tokens::{ThemeDefinition, DensityDefinition, ControlSizeDefinition};

// Minimal — uses token defaults
let theme = GpuiThemeProvider::new();

// With theme, density, and control-size overrides
let theme = GpuiThemeProvider::new()
    .with_theme(&ThemeDefinition::Dark)
    .with_density(&DensityDefinition::Compact)
    .with_control_size(&ControlSizeDefinition::Sm);

// With DPI/zoom scaling
let theme = GpuiThemeProvider::new()
    .with_scale_factor(2.0);  // Retina / 200% zoom
```

### Token Resolution

Token resolution follows a three-level priority chain:

1. Theme/density/size overrides (checked first)
2. Typed constant defaults (`poodle-tokens::typed`, light baseline)
3. Direct hex/rgba parsing (for inline color values like `"#2d86f3"`)
4. Safe fallback — black for colors, `0.0` for sizes

```rust
use poodle_gpui_components::theme_ext::*;

// Color → Hsla (GPUI's native color type)
let fill   = resolve_color(&theme, "color.background.surface");
let text   = resolve_color(&theme, "color.text.primary");
let accent = resolve_color(&theme, "color.accent.base");

// Space / size / radius → Pixels
let height = resolve_px(&theme, "size.control.height.md");
let radius = resolve_radius(&theme, "radius.control");

// Opacity → f32
let dim    = resolve_opacity(&theme, "state.opacity.disabled");
```

Direct methods on `GpuiThemeProvider` (implementing `ThemeProvider`):

| Method | Return type | Notes |
|---|---|---|
| `resolve_color(token)` | `ColorValue` (r,g,b,a f32) | sRGB |
| `resolve_space(token)` | `f32` | Logical px × scale_factor |
| `resolve_radius(token)` | `f32` | Delegates to resolve_space |
| `resolve_border_width(token)` | `f32` | Delegates to resolve_space |
| `resolve_opacity(token)` | `f32` | 0.0..1.0 |

### Theme Variants

| `ThemeDefinition` | Description |
|---|---|
| `ThemeDefinition::Dark` | Default dark theme |
| `ThemeDefinition::Light` | Light theme |
| `ThemeDefinition::LoopholeStudio` | Loophole Studio branded dark |

## Style Mapping API

### `map_layout()`

Translates `LayoutIntent` (from `poodle-layout`) into `GpuiStyle`:

```rust
use poodle_gpui::{map_layout, GpuiStyle};
use poodle_layout::LayoutIntent;

let style: GpuiStyle = map_layout(&layout_intent);
```

### `map_style()`

Main entry point for full style descriptor translation:

```rust
use poodle_gpui::{map_style, GpuiStyle};
use poodle_style::StyleDescriptor;

let style: GpuiStyle = map_style(&descriptor);
```

Additional mapping functions:

| Function | Input | Output |
|---|---|---|
| `map_border(border)` | `&BorderDescriptor` | `(f32, GpuiColor)` |
| `map_corner_radii(radii)` | `&CornerRadii` | `GpuiCornerRadii` |
| `map_shadow(shadow)` | `&ShadowValue` | `GpuiShadow` |
| `map_typography(typo)` | `&TypographyDescriptor` | `GpuiTypography` |
| `map_cursor(hint)` | `&CursorHint` | `GpuiCursorStyle` |
| `map_edges(edges)` | `&LayoutEdges` | `GpuiEdges` |

## Public Types

| Type | Description |
|---|---|
| `GpuiThemeProvider` | Token-resolving theme provider; implements `ThemeProvider` |
| `GpuiAdapter` | Adapter manifest; `name()` → "GPUI" |
| `GpuiStyle` | Complete resolved style (layout + visual + typography + focus-ring) |
| `GpuiColor` | RGBA — r, g, b, a as f32 |
| `GpuiEdges` | Layout edges (top, right, bottom, left) as f32 |
| `GpuiCornerRadii` | Corner radii (top_left, top_right, bottom_right, bottom_left) as f32 |
| `GpuiShadow` | Box shadow (offset_x, offset_y, blur: f32; color: GpuiColor) |
| `GpuiTypography` | Typography (font_family, font_size, line_height, font_weight) |
| `GpuiFontFamily` | Sans \| Mono |
| `GpuiFlexDirection` | Row \| Column |
| `GpuiJustifyContent` | Start \| Center \| End \| SpaceBetween |
| `GpuiAlignItems` | Start \| Center \| End \| Stretch |
| `GpuiOverflow` | Visible \| Hidden \| Scroll |
| `GpuiLength` | Auto \| Definite(f32) |
| `GpuiCursorStyle` | Arrow \| PointingHand \| IBeam \| NotAllowed \| OpenHand \| ClosedHand \| ResizeColumn \| ResizeRow |

## Supported Components

Current support matrix (g09.018): 96 components across primitives, composites,
and shell surfaces. Full parity status in:

```
packages/gpui/cross-runtime-parity-report.json
```

5 documented intentional native deltas (table narration, overlay focus scope,
media renderer, announcement timing, shell dock) with approved rationale.

## Dependencies

```toml
[dependencies]
poodle-adapter     = { path = "../../contracts/adapter" }
poodle-specs       = { path = "../../contracts/components" }
poodle-tokens      = { path = "../../contracts/tokens" }
poodle-layout      = { path = "../../contracts/layout" }
poodle-style       = { path = "../../contracts/style" }
poodle-events      = { path = "../../contracts/events" }
```

## Related Crates

- `poodle-gpui-components` — component structs (`Button`, `Checkbox`, etc.) implementing `IntoElement`
- `poodle-specs` — component spec structs passed into component constructors
- `poodle-tokens` — token definitions and `ThemeDefinition`/`DensityDefinition`/`ControlSizeDefinition`
- `poodle-adapter` — `ThemeProvider` trait definition
- Developer guide: `docs/guides/gpui-developer-guide.md`
