# Poodle Jetstream Developer Guide

Poodle renders native components from shared Rust specs. `poodle-render`
produces renderer-neutral node trees; the `jetstream-poodle` backend in the
Jetstream repository converts those trees to `JsEl` at the engine boundary.

Jetstream is currently a deferred paired-repository integration. Normal Poodle
component completion and `effigy qa` do not require the sibling engine. Use the
explicit `effigy ci:jetstream` or `effigy qa:jetstream` boards only in a
prepared paired workspace.

Poodle's Rust crates are pre-1.0 source previews and are not published to
crates.io. Use path or workspace dependencies. Jetstream and
`jetstream-poodle` are supplied by the Jetstream repository.

## Add Dependencies

```toml
[dependencies]
poodle-jetstream = { path = "../poodle/packages/jetstream/adapter" }
poodle-render = { path = "../poodle/packages/render" }
poodle-specs = { path = "../poodle/packages/contracts/components" }
poodle-tokens = { path = "../poodle/packages/contracts/tokens" }
jetstream-poodle = { path = "../jetstream/crates/jetstream-poodle" }
```

Use the actual Jetstream workspace path for `jetstream-poodle`; its location is
owned by that repository.

## Render a Component

Create a theme and spec, render the Poodle node, then convert once before
passing the element to Jetstream:

```rust
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ButtonSpec, ButtonTone, ButtonVariant};

let theme = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    .with_density(&poodle_tokens::density::DEFAULT)
    .with_control_size(&poodle_tokens::density::CONTROL_SIZE_SM);

let spec = ButtonSpec::new()
    .with_label("Save changes")
    .with_variant(ButtonVariant::Primary)
    .with_tone(ButtonTone::Default);

let node = poodle_render::button(&spec, &theme, None);
let element = jetstream_poodle::to_js_el(&node);

game_ui.render_immediate(&element);
```

Interactive render functions accept typed handlers. The application owns the
action performed by those handlers; Poodle owns when an intent is emitted and
when it is suppressed.

The exact render signature is part of the Rust API. Check the function in
`packages/render/src/` and the matching
[component contract](../contracts/components/README.md) for a component with
multiple handlers or content slots.

## Theme and Scale

`JetstreamThemeProvider` resolves the same semantic token roles used by the
web and GPUI implementations.

```rust
let theme = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::GRAPHITE)
    .with_density(&poodle_tokens::density::COMPACT)
    .with_control_size(&poodle_tokens::density::CONTROL_SIZE_MD)
    .with_contrast(0.65)
    .with_scale_factor(2.0);
```

Available theme constants are in `poodle_tokens::themes`; density and control
size constants are in `poodle_tokens::density`. Values are generated from the
same DTCG source as the web CSS.

## Ownership by Layer

| Layer | Owns |
| --- | --- |
| `poodle-specs` | Renderer-neutral component inputs and state |
| `poodle-tokens` | Generated themes and semantic values |
| `poodle-render` | Shared native composition, appearance, and interaction intent |
| `poodle-node` | Renderer-neutral output vocabulary |
| `poodle-jetstream` | Jetstream theme and shared style mapping |
| `jetstream-poodle` | Node-to-`JsEl` conversion at the engine boundary |
| Jetstream runtime | Widget tree, layout, drawing, input dispatch, and lifecycle |
| Application | Data, routing, persistence, domain state, and orchestration |

There is no Poodle-owned `poodle-jetstream-components` crate. Component recipes
belong in `poodle-render`; the Jetstream backend only interprets their output.

## Slots and Composition

Simple content is carried in specs. Rich child content and component handlers
are additional render-function arguments. Slots are `poodle_node::Node` values,
which keeps shared composition independent of Jetstream.

Build the complete Poodle node tree first, then call
`jetstream_poodle::to_js_el()` once at the boundary. Converting early moves
composition into the backend and creates cross-runtime drift.

## Run the Preview

The preview needs the sibling Jetstream repository configured as expected by
the local workspace. From the Poodle repository root:

```sh
bun install
effigy jetstream:preview
```

Use the preview for visual and interaction inspection. Use component contracts
and parity artifacts for semantic review; specimen output alone is not proof
of parity.

## Add or Change a Component

1. Update the contract under `docs/contracts/components/`.
2. Update or add the spec in `poodle-specs`.
3. Put reusable interaction logic in `poodle-headless` when appropriate.
4. Implement `Spec + Theme -> Node` in `poodle-render`.
5. Extend `poodle-node` only for a genuinely reusable rendering capability.
6. Extend `jetstream-poodle` only for Jetstream-specific interpretation.
7. Add preview coverage and update parity evidence.

Keep product and engine policy outside Poodle. Poodle can report selection,
submit, dismiss, or reorder intent; the host decides the resulting workflow.

## Related Documentation

- [System architecture](../architecture/001-poodle-system-shape.md)
- [Token architecture](../architecture/002-token-system-and-package-layout.md)
- [Component contracts](../contracts/components/README.md)
- [Jetstream adapter reference](../../packages/jetstream/adapter/README.md)
- [GPUI developer guide](gpui-developer-guide.md)
