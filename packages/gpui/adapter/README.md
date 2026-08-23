# poodle-gpui

`poodle-gpui` is Poodle's GPUI adapter. It supplies the GPUI theme provider and
maps renderer-neutral layout and style values into GPUI-compatible values.

The crate is a pre-1.0 source preview and is not published to crates.io.

## What It Owns

- `GpuiThemeProvider`, implementing Poodle's `ThemeProvider` contract
- theme, density, control-size, contrast, and scale-factor resolution
- `map_layout()` and `map_style()` for shared layout and style descriptors
- GPUI-compatible color, edge, radius, shadow, typography, and cursor types

It does not implement components. `poodle-render` produces `poodle-node` trees;
`poodle-gpui-node-backend` interprets those trees as GPUI elements.

## Theme Setup

```rust
use poodle_gpui::GpuiThemeProvider;

let theme = GpuiThemeProvider::new()
    .with_theme(&poodle_tokens::themes::ECLIPSE)
    .with_density(&poodle_tokens::density::DEFAULT)
    .with_control_size(&poodle_tokens::density::CONTROL_SIZE_SM)
    .with_contrast(0.5)
    .with_scale_factor(1.0);
```

Theme and mode constants are generated from Poodle's canonical DTCG token
schema. Components should resolve semantic tokens through the provider rather
than introduce local values.

## Component Flow

Renderers take a `poodle_render::RenderContext`, not a bare `ThemeProvider`.
Build the context once from the provider and thread it down; nested scopes come
from `ctx.scoped(size, density)`.

```text
poodle-specs + RenderContext (built from GpuiThemeProvider)
              |
        poodle-render
              |
         poodle-node
              |
poodle-gpui-node-backend
              |
         GPUI element
```

```rust
use poodle_render::RenderContext;

let ctx = RenderContext::new(&theme);
let node = poodle_render::button(&spec, &ctx, on_click);
let element = poodle_gpui_node_backend::to_gpui(&node);
```

Convert the completed node tree once at the GPUI boundary. Shared component
composition belongs in `poodle-render`, not in this adapter or the backend.

## Style Mapping

`map_layout()` converts `poodle_layout::LayoutIntent` to `GpuiStyle`.
`map_style()` converts `poodle_style::StyleDescriptor` to the same resolved
style type. These functions support adapter and backend work; most application
code renders components through `poodle-render` instead of calling them.

## Related Packages

- `poodle-render` — shared Rust component implementation
- `poodle-node` — renderer-neutral output tree and interaction vocabulary
- `poodle-gpui-node-backend` — GPUI interpretation and input plumbing
- `poodle-specs` — component inputs and state
- `poodle-tokens` — generated themes and semantic token data
- [GPUI developer guide](../../../docs/guides/gpui-developer-guide.md)
