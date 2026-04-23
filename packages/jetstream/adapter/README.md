# poodle-jetstream

Status: active
Updated: 2026-04-23

The canonical Jetstream adapter for Poodle. This crate bridges Poodle's
renderer-agnostic contract crates (`poodle-specs`, `poodle-tokens`,
`poodle-layout`) into the Jetstream game-engine UI runtime.

## What This Crate Owns

- **`JetstreamThemeProvider`** — implements the `ThemeProvider` trait; resolves
  semantic token paths to typed values (colors, spacing, radii, opacity) that
  Jetstream component render functions consume
- **`map_layout()`** — translates `LayoutIntent` from `poodle-layout` into a
  `taffy::Style` for Taffy flexbox layout
- **`map_style()`** — translates a `StyleDescriptor` into `JetstreamMappedStyle`
- **`JetstreamAdapter`** / **`JetstreamTarget`** — adapter manifest and render
  target, registering all 121 supported components and wiring specs to
  `JetstreamNodeHandle` outputs

## What This Crate Does NOT Own

- Rendering, draw calls, and the widget tree — owned by `jetstream-runtime`
- Event handling and input dispatch — owned by `jetstream-runtime`
- Component render functions (`js_button`, `js_checkbox`, etc.) — owned by
  `poodle-jetstream-components`
- Token definitions and values — owned by `poodle-tokens`
- Component spec structs — owned by `poodle-specs` and `poodle-workstation`

## Component Pipeline

```
ComponentSpec (e.g. ButtonSpec)
    + JetstreamThemeProvider (token resolution)
        ↓ js_button(spec, theme) → JsEl
            (fluent builder: div().h(height).bg(fill).child(...))
        ↓ game_ui.render_immediate(&root_el)
            (materialize JsEl tree → UiTree → Taffy layout → draw commands)
```

The adapter layer sits between the spec and the render function. It is not
called directly by application code — application code calls component render
functions from `poodle-jetstream-components` and passes in a
`JetstreamThemeProvider`.

## `JetstreamThemeProvider`

### Construction

```rust
use poodle_jetstream::JetstreamThemeProvider;
use poodle_tokens::{ThemeDefinition, DensityDefinition, ControlSizeDefinition};

// Minimal — uses token defaults
let theme = JetstreamThemeProvider::from_theme(&ThemeDefinition::Dark);

// With density and control-size overrides
let theme = JetstreamThemeProvider::from_theme(&ThemeDefinition::Dark)
    .with_density(&DensityDefinition::Compact)
    .with_control_size(&ControlSizeDefinition::Sm);

// With DPI/zoom scaling
let theme = JetstreamThemeProvider::from_theme(&ThemeDefinition::Dark)
    .with_scale_factor(2.0);  // Retina / 200% zoom
```

### Token Resolution

All values are resolved from the semantic token system — never hardcoded:

```rust
use poodle_jetstream_components::theme_ext::*;

// Color → glam::Vec4 (sRGB, 0.0..1.0 per channel)
let fill = resolve_color(&theme, "color.background.surface");

// Space / size / radius → f32 (logical pixels, post scale-factor)
let height = resolve_px(&theme, "size.control.height.md");
let radius = resolve_px(&theme, "radius.control");

// Opacity → f32 (0.0..1.0)
let dim = resolve_opacity(&theme, "state.opacity.disabled");
```

Direct methods on the provider:

| Method | Returns | Notes |
|---|---|---|
| `resolve_color(token)` | `ColorValue` (r,g,b,a as f32) | sRGB |
| `resolve_linear_color(token)` | `glam::Vec4` | Linear-space, for sRGB surfaces |
| `resolve_space(token)` | `f32` | Logical px, multiplied by scale_factor |
| `resolve_radius(token)` | `f32` | Delegates to resolve_space |
| `resolve_border_width(token)` | `f32` | Delegates to resolve_space |
| `resolve_opacity(token)` | `f32` | 0.0..1.0 |

### Theme Variants

| `ThemeDefinition` | Description |
|---|---|
| `ThemeDefinition::Dark` | Default dark theme |
| `ThemeDefinition::Light` | Light theme |
| `ThemeDefinition::LoopholeStudio` | Loophole Studio branded dark |

## `map_layout()`

Translates Poodle's `LayoutIntent` into a `taffy::Style` for Taffy flexbox.

```rust
use poodle_jetstream::map_layout;
use poodle_layout::LayoutIntent;

let style: taffy::Style = map_layout(&layout_intent);
```

Key mapping rules:

- `LayoutSizing::Grow` on both axes → `flex_grow: 1` (fills remaining space)
- `LayoutSizing::Grow` on one axis only → `flex_grow: 0`, relies on `align_self: Stretch`
- `LayoutSizing::Fixed(n)` → explicit size, no grow, no shrink
- `min_size` defaults to `0` (not `auto`) so containers can shrink past content size

Component render functions call `map_layout` internally — application code
rarely calls this directly.

## Public Types

| Type | Description |
|---|---|
| `JetstreamThemeProvider` | Token-resolving theme provider |
| `JetstreamAdapter` | Adapter manifest; registers 121 supported components |
| `JetstreamTarget` | Render target; associated type is `JetstreamNodeHandle` |
| `JetstreamNodeHandle` | Per-node handle: `node_id`, `spec_type`, `widget_kind`, `mapped` style |
| `JetstreamMappedStyle` | Resolved visual style ready for the runtime |
| `JetstreamColor` | RGBA color value |
| `JetstreamEdges` | Layout edges (top, right, bottom, left) |
| `JetstreamBoxShadow` | Box shadow descriptor |
| `WidgetKind` | Widget variant enum: Panel, Label, Button, Slider, ProgressBar, Image, List, TextInput |
| `map_layout` | LayoutIntent → taffy::Style |
| `map_style` | StyleDescriptor → JetstreamMappedStyle |

## Dependencies

```toml
[dependencies]
poodle-adapter    = { path = "../../contracts/adapter" }
poodle-specs      = { path = "../../contracts/components" }
poodle-workstation = { path = "../../contracts/workstation" }
poodle-tokens     = { path = "../../contracts/tokens" }
poodle-layout     = { path = "../../contracts/layout" }
poodle-style      = { path = "../../contracts/style" }
poodle-events     = { path = "../../contracts/events" }
taffy             = "0.9"
glam              = "0.29"
```

## Related Crates

- `poodle-jetstream-components` — component render functions (`js_button`, `js_checkbox`, etc.)
- `poodle-specs` — component spec structs (`ButtonSpec`, `CheckboxSpec`, etc.)
- `poodle-tokens` — token definitions and `ThemeDefinition`/`DensityDefinition`/`ControlSizeDefinition`
- `poodle-adapter` — `ThemeProvider` trait definition
- `jetstream-runtime` — `JsEl` builder and runtime rendering
