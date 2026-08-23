# poodle-jetstream

`poodle-jetstream` is Poodle's Jetstream adapter. It supplies the Jetstream
theme provider and maps renderer-neutral layout and style values into
Jetstream-compatible values.

The crate is a pre-1.0 source preview and is not published to crates.io.

## What It Owns

- `JetstreamThemeProvider`, implementing Poodle's `ThemeProvider` contract
- theme, density, control-size, contrast, and scale-factor resolution
- `map_layout()` for Taffy layout
- `map_style()` for Jetstream-compatible resolved styles
- adapter metadata and target types

It does not implement components or own the engine widget tree.
`poodle-render` produces `poodle-node` trees. The `jetstream-poodle` crate in
the Jetstream repository converts those trees to `JsEl`.

## Theme Setup

```rust
use poodle_jetstream::JetstreamThemeProvider;

let theme = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    .with_density(&poodle_tokens::density::DEFAULT)
    .with_control_size(&poodle_tokens::density::CONTROL_SIZE_SM)
    .with_contrast(0.5)
    .with_scale_factor(1.0);
```

Theme and mode constants are generated from Poodle's canonical DTCG token
schema. Components should resolve semantic tokens through the provider rather
than introduce backend-only values.

## Component Flow

Renderers take a `poodle_render::RenderContext`, not a bare `ThemeProvider`.
Build the context once from the provider and thread it down; nested scopes come
from `ctx.scoped(size, density)`.

```text
poodle-specs + RenderContext (built from JetstreamThemeProvider)
                |
          poodle-render
                |
           poodle-node
                |
       jetstream-poodle
                |
              JsEl
                |
       Jetstream runtime
```

```rust
use poodle_render::RenderContext;

let ctx = RenderContext::new(&theme);
let node = poodle_render::button(&spec, &ctx, on_click);
let element = jetstream_poodle::to_js_el(&node);
```

This crate compiles against the shared contracts and consumes the same node
tree as GPUI. That is composition reuse, not parity evidence: Jetstream's
backend integration is program-deferred and is not passing parity.

Convert the completed node tree once at the engine boundary. Shared component
composition belongs in `poodle-render`; widget materialization, layout, drawing,
and input dispatch belong in Jetstream.

## Style Mapping

`map_layout()` converts `poodle_layout::LayoutIntent` to `taffy::Style`.
`map_style()` converts `poodle_style::StyleDescriptor` to
`JetstreamMappedStyle`. Most application code renders components through
`poodle-render` instead of calling these functions directly.

## Related Packages

- `poodle-render` — shared Rust component implementation
- `poodle-node` — renderer-neutral output tree and interaction vocabulary
- `jetstream-poodle` — Node-to-`JsEl` backend in the Jetstream repository
- `poodle-specs` — component inputs and state
- `poodle-tokens` — generated themes and semantic token data
- [Jetstream developer guide](../../../docs/guides/jetstream-developer-guide.md)
