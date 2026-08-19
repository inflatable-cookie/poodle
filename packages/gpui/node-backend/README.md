# poodle-gpui-node-backend

`poodle-gpui-node-backend` interprets `poodle-node` trees as GPUI elements. It
is the runtime boundary between Poodle's shared Rust renderer and GPUI.

This crate is a pre-1.0 source preview and is not published to crates.io.

```rust
let node = poodle_render::button(&spec, &theme, on_click);
let element = poodle_gpui_node_backend::to_gpui(&node);
```

The backend owns GPUI-specific materialization, input dispatch, text editing,
focus, and event plumbing. Component composition and visual treatment belong
in `poodle-render`; semantic inputs belong in `poodle-specs`.

Runtime-owned scroll state follows the same boundary. Retain one
`TrackedScrollState` per component instance, render the content and jump
control as ordinary nodes, then pass them to `tracked_vertical_scroll`. The
state owns GPUI's non-Send `ScrollHandle`; `jump_handler()` is the send-safe
intent bridge a renderer-owned button can use. No scroll offset or GPUI type
enters `poodle-node` or a component spec.

Convert a completed node tree once near the GPUI view boundary. Do not place
shared component recipes in this crate.

See the [GPUI developer guide](../../../docs/guides/gpui-developer-guide.md)
and [`poodle-gpui` adapter reference](../adapter/README.md).
