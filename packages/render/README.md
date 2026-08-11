# poodle-render

`poodle-render` is the single Rust implementation of Poodle's native
components. It converts renderer-neutral component specs and themes into
`poodle-node` trees.

This crate is a pre-1.0 source preview and is not published to crates.io.

```text
poodle-specs + ThemeProvider + handlers + slots
                       |
                 poodle-render
                       |
                  poodle-node
                  /          \
          GPUI backend    Jetstream backend
```

## Example

```rust
use poodle_specs::{ButtonSpec, ButtonVariant};

let spec = ButtonSpec::new()
    .with_label("Save")
    .with_variant(ButtonVariant::Primary);

let node = poodle_render::button(&spec, &theme, on_click);
```

Render functions accept the component spec, a `ThemeProvider`, and typed slots
or handlers where the contract requires them. Backends interpret the completed
node tree; they do not reimplement component recipes.

Add shared native composition, token-resolved appearance, and interaction intent here.
Keep engine input, text, lifecycle, and drawing behavior in the backend.

See the [native architecture](../../docs/architecture/001-poodle-system-shape.md)
and [component contracts](../../docs/contracts/components/README.md).
