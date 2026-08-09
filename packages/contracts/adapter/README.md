# poodle-adapter

Renderer-neutral adapter contracts for Poodle's native targets.

`ThemeProvider` resolves semantic design tokens. `RenderAdapter` and the
adapter manifest types describe renderer capabilities without importing GPUI,
Jetstream, or another runtime. Backend-specific conversion belongs in the
target adapter, not here.

This crate is a pre-1.0 source preview and is not yet published to crates.io.
See the [native architecture](../../../docs/architecture/001-poodle-system-shape.md)
for the full render flow.
