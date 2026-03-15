# g08.001 — Jetstream Adapter Crate Setup and Token Bridge

Status: Completed
Updated: 2026-03-14

## Objective

Create the `pug-jetstream` crate with Cargo.toml, module structure, core types,
and the `JetstreamAdapter` implementing `AdapterManifest`. Establish the type
bridge between Pug typed tokens and Jetstream's native types.

## Deliverables

### Crate structure

- `packages/jetstream/adapter/Cargo.toml` — depends on all 8 contract crates
  (pug-tokens, pug-layout, pug-events, pug-style, pug-adapter, pug-primitives,
  pug-composites, pug-workstation)

### Core types (lib.rs)

- `WidgetKind` enum: Panel, Label, Button, Slider, ProgressBar, Image, List,
  TextInput — maps Pug specs to Jetstream widget variants
- `JetstreamNodeHandle`: node_id (String), spec_type (&'static str),
  widget_kind (WidgetKind)
- `JetstreamTarget` implementing `RenderTarget`
- `JetstreamAdapter` with theme provider storage

### Adapter manifest

- `SUPPORTED_PRIMITIVES`: 49 primitive spec names
- `SUPPORTED_COMPOSITES`: 23 composite spec names
- `UNSUPPORTED_COMPONENTS`: 28 entries with reasons (game-inappropriate)
- `AdapterManifest` implementation for `JetstreamAdapter`

### Module registration

Modules: demo_scene, render_action, render_composites, render_feedback,
render_input, render_overlay, render_selection, render_structural, style_map,
theme

## Verification

```
cargo check — clean compilation
4 lib.rs tests passing
```
