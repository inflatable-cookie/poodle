# Meter Surface

Status: detailed contract
Updated: 2026-08-14

## 1. Purpose

- Component name: `MeterSurface`
- Layer: `foundation`
- Summary: web-only batched canvas rendering coordinator for surface-mode
  AudioMeter placeholders

`MeterSurface` is a rendering strategy, not a new meter semantic. It owns one
scroll container, one accessibility-hidden overlay canvas, and one frame loop
for every descendant surface-mode `AudioMeter`. Meter meaning, ballistics, and
accessibility semantics stay on `AudioMeter` and `MeterBus` per spec 068.

## 2. Anatomy

```text
[Root]
  [Viewport] the one owned scroll container
    [Content] host-arranged surface-mode AudioMeter placeholders
  [Canvas] pointer-events none, aria-hidden overlay
```

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `bus` | `MeterBus \| null` | `null` | **Web targets only** — the one bus this surface paints from; `null` renders an inert container |
| `painter` | `MeterSurfacePainter \| null` | `null` | **Web targets only** — injectable painter seam; `null` uses the default Canvas2D painter |

Children are the host-owned content region. Descendant surface-mode
`AudioMeter` placeholders connect through framework context; the bus given to
each placeholder must be this surface's bus.

## 4. States And Behavior Machine

Classification: static composition over the shared DOM controller in
`poodle-core` (`createMeterSurfaceController`). The controller owns canvas
lifecycle, one ResizeObserver, scroll projection from cached content
coordinates, device-pixel-ratio backing-store sizing, theme/palette probes,
viewport culling, the bus frame subscription, and cleanup. There is no
component machine and no per-frame framework state.

## 5. Events

None. The surface emits no user events; the canvas is non-interactive.

## 6. Accessibility

The root and content region are plain containers. The canvas is `aria-hidden`
and never enters the accessibility tree. Meter semantics live entirely on the
surface-mode `AudioMeter` placeholders, which keep `role="meter"` and refresh
their value text through one shared cadence at most twice per second.

## 7. Layout

One surface owns exactly one scroll container and one canvas. Placeholder
geometry is cached in content coordinates on registration, resize observation,
or explicit invalidation; scroll repaints from cached geometry plus the
current scroll offset without per-frame layout reads. Offscreen placeholders
are culled from paint while their bus channels continue ballistics.

## 8. Token Usage

The canvas painter resolves the standalone AudioMeter recipe hooks
(`--poodle-recipe-audio-meter-*`) and status/token colors through computed
style probes on mount and on observed theme or recipe invalidation. It never
reads computed styles in the frame loop.

## 9. Svelte Notes

`MeterSurface.svelte` mounts the shared controller, provides the registration
context, and renders children into the content region. `bus` and `painter`
are mount-time configuration.

## 9a. React Notes

`MeterSurface.tsx` mirrors the Svelte wrapper with the same context seam and
imperative `invalidateLayout`/`refreshPalette` handle. Both wrappers stay
thin: geometry, palette, painter, and bus policy live in `poodle-core`.

## 10. GPUI Notes

Not applicable. GPUI already batches AudioMeter nodes at its renderer scene;
`MeterSurface` has no native counterpart by fixed decision (spec 068).

## 10a. Jetstream Notes

Not applicable, as for GPUI. This is an accepted web-only rendering mechanism,
not missing native behavior.

## 11. Parity Checklist

- Svelte and React render identical root/viewport/content/canvas anatomy
- both wrappers share one bus, controller, palette, and painter policy
- surface-mode placeholders match standalone AudioMeter accessibility
- painted output reproduces standalone bar/segment, orientation, mono/stereo,
  peak-hold, clip, enabled, and recipe-color semantics

## 12. Known Deltas

Native runtimes have no `MeterSurface`; their renderers already batch meter
nodes. Canvas2D is the only shipped painter; WebGL2 waits on measured
evidence per g14.024.

## 13. Specimen Definitions

The Svelte and React previews provide one batched surface page with 8, 32,
and 128 meters, all four meter modes, bar and segment styles, mono and
stereo, vertical and horizontal examples, live theme switching, a constrained
scrolling strip with offscreen culling, clip latch/reset, registration and
unregistration, destroy/remount, and a deterministic 15 Hz workload with a
performance readout. The page links to the standalone AudioMeter specimen.

## 14. Approval And Adoption Notes

Governed by spec 068 and roadmap g14.024. The 128-meter Canvas2D performance
evidence decides whether a WebGL2 painter card ever exists.
