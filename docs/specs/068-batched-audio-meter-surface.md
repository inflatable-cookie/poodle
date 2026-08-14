# 068 Batched Audio Meter Surface

Status: approved for g14.024 implementation
Updated: 2026-08-14
Depends on: `../architecture/006-headless-core-and-machine-model.md`,
`../architecture/008-audio-control-family.md`,
`../contracts/components/audio-meter.md`

## Purpose

Add an opt-in web rendering tier for high-count consoles without changing the
default per-instance `AudioMeter`. One host push updates all registered meter
channels; one animation loop advances them; one overlay canvas paints every
visible placeholder.

This is reusable audio-control infrastructure. It receives opaque channel IDs
and aggregate meter frames. It knows nothing about mixers, faders, tracks,
Loophole events, credentials, persistence, or audio-thread ownership.

## Fixed Scope

- `MeterBus` is framework-free TypeScript in `poodle-core`.
- Svelte and React share the bus, browser controller, painter, CSS, and
  observable contract.
- `AudioMeter` gains an opt-in surface tier; standalone remains the default.
- `MeterSurface` is a web rendering coordinator, not a new portable component.
- Canvas2D is the only required painter in this card. WebGL2 is deferred until
  measured evidence says it is needed.
- Native AudioMeter contracts and renderers remain unchanged. GPUI and
  Jetstream already batch node drawing at their scene boundary.

## MeterBus API

The public shape may use a class or factory, but must preserve this vocabulary:

```ts
type MeterBusChannelId = string | number;

interface MeterBusChannel {
  readonly id: MeterBusChannelId;
  readonly slot: number;
}

interface MeterBusRegistration {
  mode: AudioMeterMode;
  minDb?: number;
  maxDb?: number;
  enabled?: boolean;
}

interface MeterBus {
  register(id: MeterBusChannelId, input: MeterBusRegistration): MeterBusChannel;
  unregister(channel: MeterBusChannel): void;
  pushFrames(data: Float32Array, atMs: number, durationMs: number): number;
  resetClip(id: MeterBusChannelId): void;
  destroy(): void;
  readonly view: MeterBusView;
}
```

`pushFrames` consumes repeated `[slot, peak, meanSquare]` triples. Slot is the
small non-negative integer returned by `register`, exactly representable in a
Float32 value. String channel IDs never enter the hot buffer. The Loophole
adapter maps fader node IDs to slots once and reuses one feed buffer.

`atMs` and `durationMs` are batch-wide because one host telemetry event owns
the batch. Both are required: deriving the first duration from an arbitrary
monotonic-clock origin would corrupt VU and RMS integration. `meanSquare`
retains the existing `MeterFeedFrame` meaning; an adapter receiving RMS
amplitude squares it before pushing.

Registration and capacity growth are cold paths and may allocate. Duplicate
IDs, invalid bounds, or use-after-unregister fail clearly. The hot paths after
warm-up — `pushFrames`, animation advance, view reads, draw-pass assembly, and
painting — allocate no JS objects or arrays.

The bus stores scalar state in structure-of-arrays typed buffers. Use the type
appropriate to the value: timestamp/math precision must not be sacrificed to
force every field into Float32, and boolean/enum fields do not belong in float
arrays. `view` is a stable descriptor whose buffer fields may be replaced only
on cold-path capacity growth. A painter reads a slot directly; no per-slot
snapshot object exists.

Invalid data is isolated to its triple. A batch continues for other valid
slots. Data is invalid when the slot is not active, the slot is repeated in
one batch, values are non-finite or negative, duration is not positive, or the
timestamp is stale for that slot. `pushFrames` returns the accepted channel
count and allocates no diagnostic list.

## Ballistics And Time

Extract the existing VU, PPM, sample-peak, RMS, peak-hold, and clip operations
into pure per-channel functions. Route both `audioMeterTransition` and
`MeterBus` through those functions. Preserve all constants and standalone
golden outputs.

Surface mode may advance between telemetry pushes through explicit time-step
inputs driven by one injected animation-frame scheduler. Tests use a manual
scheduler. The bus owns one loop regardless of channel or surface count and
stops it when destroyed.

Golden parity applies to identical explicit sequences: quantize host data
through the same Float32 feed values, then apply the same push and time-step
timestamps to standalone math and one bus slot. Compare every mode's input,
ballistic, peak hold, clip, and normalized output within one documented
floating-point tolerance. Frame cadence may not select another law.

RMS history uses a preallocated per-channel ring. Its capacity and supported
minimum frame duration are explicit constructor limits. Overflow before the
300 ms window can evict a slice is rejected rather than silently approximated.
The default must comfortably cover the stated 10–15 Hz host feed.

## AudioMeter Surface Mode

The two web components add:

```ts
surface?: MeterBus | null;
channel?: MeterBusChannelId | null;
rightChannel?: MeterBusChannelId | null;
```

With `surface=null`, markup, behavior, handles, and visuals remain unchanged.
With a surface:

- `channel` is required and already registered on that bus;
- `rightChannel` optionally supplies stereo;
- the root retains the existing classes, data attributes, dimensions, recipe
  hooks, and `role="meter"` surface;
- no `AudioMeterVisual` or other paint DOM is rendered;
- `context` and `rightContext` do not become competing authority;
- `push` and `resetClip` forward to the bus using reusable scratch storage;
- a missing matching `MeterSurface` is a development error, not standalone
  fallback.

The placeholder updates `aria-valuemin`, `aria-valuemax`, `aria-valuenow`, and
`aria-valuetext` immediately on mount and at no more than 2 Hz afterward. Mono
and stereo value text matches standalone formatting. The update timer is
shared per bus/surface, not one interval per meter.

## MeterSurface And Painter

`MeterSurface` wraps one scroll container and renders one
`pointer-events:none`, `aria-hidden=true` overlay canvas. Framework context
connects descendant surface-mode AudioMeters to the nearest surface using the
same bus. The shared DOM controller owns all lifecycle and geometry policy.

The controller:

- caches placeholder rectangles and paint geometry;
- remeasures only on registration, ResizeObserver notification, density/size
  change, or explicit invalidation;
- tracks scroll through cached content coordinates plus the current scroll
  offset, without calling `getBoundingClientRect` per frame;
- scales the backing store for device pixel ratio;
- culls outside the viewport while leaving bus state active;
- resolves recipe/token colors through computed-style probe elements once on
  mount and once per observed theme/recipe invalidation;
- disposes observers, listeners, frame subscriptions, and painter resources.

The first painter is Canvas2D. Define one injectable `MeterSurfacePainter`
interface around a preallocated `MeterDrawPass`: flat geometry, slot/state
buffers, palette, viewport, and active count in; one `paint` call out. Keep the
lifecycle free of Canvas2D-specific policy so a later WebGL2 painter can
replace it. Do not build a generic plugin asset runtime or a WebGL painter in
this card.

The painter reproduces standalone bar/segment, orientation, mono/stereo,
peak-hold, clip, enabled, size, density, and recipe semantics. Theme values are
resolved outside the frame loop. A CSS gradient string is not parsed per
frame; palette probes normalize the required colors and the painter constructs
its gradient once per palette change.

## Accessibility And Browser Evidence

- Canvas never enters the accessibility tree.
- A surface placeholder exposes the same accessible name, bounds, current dB,
  and formatted value text as standalone at an accessibility sample point.
- Clip reset stays host-owned and uses a separate accessible control.
- Chromium and WebKit both run the same scroll, resize, DPR, live-theme,
  registration, unregistration, and clip-reset browser probes against Svelte
  and React previews.
- Happy-DOM tests may prove component wiring; they do not replace real-browser
  canvas and layout evidence.

## Performance Contract

Reference workload: 128 registered vertical segment meters, 12 segments each,
15 Hz aggregate data, 60 Hz paint, with viewport culling enabled. On the
operator's mid-range Apple Silicon Mac, the target is less than 2 ms per frame
for bus advance plus draw-pass assembly plus paint after warm-up.

The specimen exposes deterministic start/stop and measurement controls. Record
browser, hardware, DPR, viewport, sample count, warm-up, mean, p50, p95, and
max in the implementation log. The acceptance decision uses p95. Do not turn a
hardware-sensitive number into a broad CI gate; keep deterministic correctness
and browser probes gated, with the measured budget recorded as review evidence.

## Non-goals

- WebGL2, OffscreenCanvas, workers, SharedArrayBuffer, AudioWorklet, raw audio
  samples, or GPU asset management
- Loophole event types, fader nodes, mixer layout, selection, or persistence
- gain-reduction batch rendering
- native API or renderer changes
- changing standalone AudioMeter's default markup or feed cadence
