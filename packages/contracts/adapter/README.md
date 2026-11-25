# poodle-adapter

Renderer-neutral adapter contracts for Poodle's native targets.

`ThemeProvider` resolves semantic design tokens for the node renderer.
`RenderComponent`, `RenderTarget` and `AdapterManifest` describe the legacy
GPUI direct path. The crate keeps these contracts independent of any runtime.

This crate is a pre-1.0 source preview and is not yet published to crates.io.
See the [native architecture](../../../docs/knowledge/architecture/001-poodle-system-shape.md)
for the full render flow.
