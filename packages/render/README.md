# poodle-render

`poodle-render` is the single Rust implementation of Poodle's native
components. It converts renderer-neutral component specs and themes into
`poodle-node` trees.

This crate is a pre-1.0 source preview and is not published to crates.io.

```text
poodle-specs + RenderContext + handlers + slots
                       |
                 poodle-render
                       |
                  poodle-node
                  /          \
          GPUI backend    Jetstream backend
```

## Example

```rust
use poodle_render::RenderContext;
use poodle_specs::{ButtonSpec, ButtonVariant};

// The construction context borrows the theme and carries the effective
// presentation defaults (root: md size scale, default density).
let ctx = RenderContext::new(&theme);

let spec = ButtonSpec::new()
    .with_label("Save")
    .with_variant(ButtonVariant::Primary);

let node = poodle_render::button(&spec, &ctx, on_click);
```

Render functions accept the component spec, a `RenderContext`, and typed slots
or handlers where the contract requires them. Backends interpret the completed
node tree; they do not reimplement component recipes.

## Presentation Cascade

`RenderContext` is the construction-time presentation boundary (architecture
010). It is built once from a `ThemeProvider` and threaded down; renderers
reach the theme through `ctx.theme()` rather than taking one directly.

Semantic spec inputs are `Option<ControlSize>` / `Option<ControlDensity>`.
`None` inherits the surrounding scope; an explicit value is final and is never
rescaled — `size_role` maps inherited scale only. `ctx.scoped(size, density)`
opens a nested scope, and `ui_presentation_provider` is the provider boundary
that returns its built child unchanged. Host slots that open a scope take a
`SlotBuilder` (`FnOnce(&RenderContext<'_>) -> Node`) rather than a built
`Node`, so the child is constructed inside the scope it belongs to.

`RenderContext`, `SlotBuilder`, and `ui_presentation_provider` are re-exported
at the crate root; use that path.

Add shared native composition, token-resolved appearance, and interaction intent here.
Keep engine input, text, lifecycle, and drawing behavior in the backend.

See the [native architecture](../../docs/architecture/001-poodle-system-shape.md)
and [component contracts](../../docs/contracts/components/README.md).
