# Poodle GPUI Developer Guide

Poodle renders native components from shared Rust specs. `poodle-render`
produces a renderer-neutral node tree; `poodle-gpui-node-backend` interprets
that tree as GPUI elements.

Poodle's Rust crates are pre-1.0 source previews and are not published to
crates.io. Use path or workspace dependencies.

## Add Dependencies

```toml
[dependencies]
gpui = "0.2.2"
poodle-gpui = { path = "../poodle/packages/gpui/adapter" }
poodle-gpui-node-backend = { path = "../poodle/packages/gpui/node-backend" }
poodle-render = { path = "../poodle/packages/render" }
poodle-specs = { path = "../poodle/packages/contracts/components" }
poodle-tokens = { path = "../poodle/packages/contracts/tokens" }
```

Adjust paths for a vendored checkout or Cargo workspace.

## Render a Component

Create a theme, build the component spec, render a node, and convert it at the
GPUI boundary:

```rust
use gpui::IntoElement;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ButtonSpec, ButtonTone, ButtonVariant};

let theme = GpuiThemeProvider::new()
    .with_theme(&poodle_tokens::themes::ECLIPSE)
    .with_density(&poodle_tokens::density::DEFAULT)
    .with_control_size(&poodle_tokens::density::CONTROL_SIZE_SM);

let spec = ButtonSpec::new()
    .with_label("Save changes")
    .with_variant(ButtonVariant::Primary)
    .with_tone(ButtonTone::Default);

let node = poodle_render::button(&spec, &theme, None);
let element = poodle_gpui_node_backend::to_gpui(&node);

element.into_element()
```

Interactive render functions accept typed handlers. For example, a button
click handler is an `Arc<dyn Fn() + Send + Sync>`:

```rust
use std::sync::Arc;

let on_save = Arc::new(|| {
    // Send an application action or update application state.
});

let node = poodle_render::button(&spec, &theme, Some(on_save));
```

The exact render signature is part of the Rust API. Check the function in
`packages/render/src/` and the matching
[component contract](../contracts/components/README.md) for a component with
multiple handlers or content slots.

## Theme and Scale

`GpuiThemeProvider` implements Poodle's renderer-neutral `ThemeProvider` trait.
It resolves semantic colors, spacing, radii, borders, and opacity into typed
values used by `poodle-render`.

```rust
let theme = GpuiThemeProvider::new()
    .with_theme(&poodle_tokens::themes::GRAPHITE)
    .with_density(&poodle_tokens::density::COMPACT)
    .with_control_size(&poodle_tokens::density::CONTROL_SIZE_MD)
    .with_contrast(0.65)
    .with_scale_factor(2.0);
```

Available theme constants are in `poodle_tokens::themes`; density and control
size constants are in `poodle_tokens::density`. Use these generated constants
instead of constructing theme values in application code.

## Ownership by Layer

| Layer | Owns |
| --- | --- |
| `poodle-specs` | Renderer-neutral component inputs and state |
| `poodle-tokens` | Generated themes and semantic values |
| `poodle-render` | Shared native composition, treatment, and interaction intent |
| `poodle-node` | Renderer-neutral output vocabulary |
| `poodle-gpui` | GPUI theme and shared style mapping |
| `poodle-gpui-node-backend` | Node interpretation, GPUI input, and event plumbing |
| Application | Data, routing, persistence, domain state, and orchestration |

Do not create a GPUI-only component implementation when the semantics are
shared with Jetstream. Add the spec and contract first, implement the component
in `poodle-render`, then extend the node backend only if the node vocabulary
cannot express it.

## Slots and Composition

Simple content is carried in specs. Rich child content and component handlers
are additional render-function arguments. Slots are `poodle_node::Node` values,
so components can compose without depending on GPUI types.

Convert the completed tree to GPUI once, near the view boundary. Avoid
converting child nodes individually and then trying to insert GPUI elements
back into shared rendering.

## Run the Preview

From the Poodle repository root:

```sh
bun install
effigy gpui:preview
```

The preview exercises the component catalogue across theme, density, and
control-size modes. Use the component contract and parity artifacts for
semantic review; specimen output alone is not proof of parity.

## Add or Change a Component

1. Update the contract under `docs/contracts/components/`.
2. Update or add the spec in `poodle-specs`.
3. Put reusable interaction logic in `poodle-headless` when appropriate.
4. Implement `Spec + Theme -> Node` in `poodle-render`.
5. Extend `poodle-node` only for a genuinely reusable rendering capability.
6. Add GPUI backend behavior only for GPUI-specific interpretation.
7. Add preview coverage and update parity evidence.

Keep application actions outside Poodle. A component reports intent; the host
decides what saving, navigation, upload, or deletion means.

## Related Documentation

- [System architecture](../architecture/001-poodle-system-shape.md)
- [Token architecture](../architecture/002-token-system-and-package-layout.md)
- [Component contracts](../contracts/components/README.md)
- [GPUI adapter reference](../../packages/gpui/adapter/README.md)
- [Jetstream developer guide](jetstream-developer-guide.md)
