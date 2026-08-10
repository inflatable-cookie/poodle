# 008 Audio Control Family

Status: active
Updated: 2026-08-10
Depends on: `006-headless-core-and-machine-model.md`,
`007-appearance-recipe-contract.md`
Source: Loophole `docs/research/poodle-instrument-rfc.md`

## Placement

Audio controls are reusable Poodle primitives, not Loophole widgets.

- `packages/core/src/audio/` owns framework-free laws, formatting, machines,
  hit-test math, meter integration, and serializable visual state.
- `packages/svelte/components/` owns DOM event adaptation, accessibility, and
  the standard token/recipe-themed renderers.
- No new package is added in this phase. The proposed instrument package has
  no independent runtime boundary until asset skins, host bindings, or a GPUI
  backend exist.

The existing bounded-measurement `Meter` remains unchanged. The audio-domain
component is named `AudioMeter`; it has temporal state and a feed protocol,
which are outside the existing component's contract.

## Visual State Boundary

Every audio core produces a JSON-serializable visual state. Default renderers
receive that state; they do not read machine context, bind input events,
perform hit tests, or expose accessibility semantics.

```ts
interface AudioControlVisualState {
  valueNorm: number;
  rawValue: number;
  bipolarCenter: number | null;
  hover: boolean;
  focus: boolean;
  drag: "none" | "coarse" | "fine";
  automation: "none" | "touched" | "latched" | "writing" | "read";
  enabled: boolean;
}

interface AudioMeterVisualState extends AudioControlVisualState {
  ballisticValue: number;
  peakHold: number | null;
  clip: boolean;
}
```

Phase 2 adds component-specific serializable shapes. These do not widen the
scalar base with unrelated optional fields:

```ts
interface EnvelopeVisualState {
  points: Array<{
    id: string; xNorm: number; yNorm: number; curve: number;
    selected: boolean; dragging: boolean;
  }>;
  hoverPointId: string | null;
  focus: boolean;
  enabled: boolean;
}

interface XYPadVisualState {
  xNorm: number; yNorm: number;
  rawX: number; rawY: number;
  hover: boolean; focus: boolean;
  drag: "none" | "coarse" | "fine";
  automation: AudioAutomationState;
  enabled: boolean;
}

interface AudioSwitchVisualState {
  state: number; stateCount: number;
  pressed: boolean; lampOn: boolean;
  hover: boolean; focus: boolean; enabled: boolean;
}

interface GainReductionMeterVisualState extends AudioMeterVisualState {
  reductionDb: number;
}
```

Envelope `curve` belongs to the segment starting at that point and is clamped
to `-1..1`. Segment interpolation is monotonic: zero is linear; positive
amounts use `t^(1 + 4c)`; negative amounts use
`1 - (1 - t)^(1 + 4|c|)`. `envelopeSegmentValueAt` is the canonical geometry
helper used by renderers, so flat segments remain flat and skins cannot invent
another curve law. Renderers derive segment paths from the ordered point list. XY
keeps both law-mapped axes in one atomic visual snapshot. Switch lamp state is
explicit and may differ from its selected state. Gain reduction uses the
meter visual vocabulary while retaining its positive reduction magnitude.

Component geometry is a separate serializable renderer input. It includes
orientation and display-only detent positions. A stereo meter passes two
independent `AudioMeterVisualState` values. This preserves the RFC state shape
without hiding machine access in drawing code.

DOM adapters own pointer capture, wheel cancellation, focus, keyboard event
translation, and ARIA. Core hit-test helpers operate on geometry values only.
Later skins may replace renderer components without changing those paths.

## Value Laws

All laws map a plain value in `[min, max]` to a normalized value in `[0, 1]`
and provide the inverse mapping.

- `linear`: affine mapping.
- `logarithmic`: logarithmic interpolation; requires `0 < min < max`.
- `exponential`: normalized position raised to a positive exponent.
- `stepped`: an inner continuous law followed by plain-value step snapping.
- `bipolar-center`: two linear half-ranges meeting at `valueNorm = 0.5`; the
  center must lie strictly between the bounds.

Invalid logarithmic, exponential, or bipolar definitions throw `RangeError`.
Degenerate bounds normalize to zero. All output is clamped to its declared
range. Fine adjustment uses one tenth of the configured keyboard or drag
increment.

## Gesture Contract

Knob, Fader, and DragNumberField transitions emit `beginGesture` exactly once
when a pointer drag starts and `endGesture` exactly once when it ends or is
cancelled. `drag` in VisualState exposes the active coarse/fine phase. Wheel,
keyboard, reset, and text commits are atomic value changes and commits; they
do not pretend to be sustained pointer gestures.

Fader detents are plain values. A value within the configured normalized snap
distance resolves to the nearest detent. Gesture effects remain skin
independent for later host-automation binding.

## Meter Feed And Ballistics

Hosts push aggregate level frames, not individual audio samples:

```ts
interface MeterFeedFrame {
  atMs: number;
  peak: number;       // linear amplitude, 1 = 0 dBFS
  meanSquare: number; // aggregate square mean for this frame
  durationMs: number;
}
```

`audioMeterTransition(context, { type: "PUSH_FRAME", frame })` is pure and
returns the next context. Hosts should aggregate on the audio thread and push
at UI cadence. The transition stores only the bounded RMS window and scalar
ballistic state. This avoids callbacks, timers, DOM dependencies, and
audio-rate allocation requirements in the core contract.

Frames require finite, non-negative `peak`, `meanSquare`, and `atMs`, a finite
positive `durationMs`, and monotonic timestamps. Invalid or stale frames are
ignored without changing context.

Constants:

| Mode | Attack | Release / window |
| --- | --- | --- |
| VU | 300 ms exponential integration | 300 ms |
| PPM | 10 ms | 1,500 ms |
| Sample peak | immediate | 20 dB/s |
| RMS | immediate display of window result | 300 ms weighted window |

Peak hold lasts 1,500 ms, then decays at 20 dB/s. Clip latches when peak is at
or above `1.0` and clears only on `RESET_CLIP`. Display normalization maps the
configured dB range (default `-60..0 dBFS`) linearly to `0..1`; silence maps
to the floor.

Gain-reduction frames carry a positive reduction magnitude in dB. The core
uses a 10 ms attack and 300 ms release, with zero as no reduction. Display
normalization maps `0..maxReductionDb` to `0..1`; the standard renderer draws
that magnitude from the zero end toward maximum reduction on an inverted
meter axis. Gain reduction has no clip latch.

## Appearance And Deferred Work

Standard renderers consume semantic tokens through complete
`--poodle-recipe-<component>-...` hook sets. The asset-skin runtime, Tier 2
custom renderer API, GPUI implementation, and host parameter binding remain
deferred. Phase 2 controls build on this boundary after Phase 1 review.
