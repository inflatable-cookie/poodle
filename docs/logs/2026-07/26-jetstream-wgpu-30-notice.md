# Jetstream notice — engine moved to wgpu 30

Date: 2026-07-26
From: Jetstream thread (g06.030)
Full record: `../jetstream/docs/roadmaps/g06/030-html-ui-tier-cef-feasibility.md`

Jetstream upgraded wgpu **29 → 30**. This affects exactly one crate here.

## Action needed

`packages/jetstream/preview/Cargo.toml` line 31 declares `wgpu = "29"`. It is
the only Poodle crate that names wgpu at all. Bump it to `"30"` or
`poodle-jetstream-preview` — and therefore the `snap` specimen renderer — stops
building against Jetstream HEAD with a version-mixing type error.

`poodle-jetstream` (adapter) and `poodle-jetstream-components` do **not**
declare wgpu, so they follow `jetstream-ui` transparently and need no change.

## API changes you may hit in the preview crate

All four are mechanical and behaviour-preserving. Jetstream's own migration
touched 86 files and needed nothing beyond these:

- `SurfaceTexture::present()` → `Queue::present(surface_texture)`
- `RequestAdapterOptions` gained `apply_limit_buckets` (use `false`)
- `SurfaceConfiguration` gained `color_space` — use
  `wgpu::SurfaceColorSpace::Auto`, which reproduces pre-30 behaviour exactly.
  Naming a concrete space changes how output is encoded.
- `RenderPipelineDescriptor.vertex.buffers` is now
  `&[Option<VertexBufferLayout>]`, so wrap each layout in `Some(...)`
- `Buffer::get_mapped_range{,_mut}()` now returns `Result`

## Why

The `cef` crate's `accelerated_osr` feature — which imports CEF's shared
texture directly as a `wgpu::Texture` via IOSurface on macOS — requires wgpu
30. That is the zero-copy path for the HTML UI tier Jetstream is evaluating for
editor tooling, and on macOS it is not optional: CEF's `on_paint` is never
called when shared textures are enabled.

Unrelated to the component sweep findings in
`26-jetstream-component-sweep-notice.md`, which still stand.
