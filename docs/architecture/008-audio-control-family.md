# 008 Audio Control Family

Status: accepted
Updated: 2026-08-10
Depends on: `006-headless-core-and-machine-model.md`,
`007-appearance-recipe-contract.md`
Source: Loophole `docs/research/poodle-instrument-rfc.md`

## Placement

Audio controls are reusable Poodle primitives, not Loophole widgets.

- `packages/core/src/audio/` owns framework-free laws, formatting, machines,
  hit-test math, meter integration, and serializable visual state.
- `packages/svelte/components/` and `packages/react/components/` own DOM event
  adaptation, accessibility, and thin standard token/recipe-themed shells over
  the shared web cores.
- `packages/contracts/headless/` owns the native equivalents of the audio laws,
  formatting, machines, feed integration, hit-test math, and serializable
  visual states.
- `packages/contracts/components/` carries renderer-neutral native specs whose
  public fields resolve to those visual states.
- `packages/render/` consumes native visual states and emits `poodle-node`
  trees. GPUI and Jetstream remain backend interpreters plus input, lifecycle,
  hit-testing, and accessibility adapters; neither backend owns audio drawing
  policy.
- No new package is added. The proposed instrument package still has no
  independent runtime boundary until asset skins or host bindings exist.

The existing bounded-measurement `Meter` remains unchanged. The audio-domain
component is named `AudioMeter`; it has temporal state and a feed protocol,
which are outside the existing component's contract.

## Presentation Axes

Every audio component implements Poodle's two presentation axes. `size`
accepts `xs`, `sm`, `md`, `lg`, or `xl`; `density` accepts `compact`,
`default`, or `comfortable`. Null web props inherit `UiProvider` presentation.
Native specs carry `size`, `size_role`, and `density` and resolve semantic size
roles before building geometry.

Size owns the component's main footprint and type scale. Density owns internal
spacing or visual weight. Neither axis enters machine context or VisualState,
changes value laws or meter ballistics, or moves hit-testing into the renderer.
All four specimen systems expose complete size and density matrices for every
audio component.

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

Phase 3 closes the family with three renderer-complete shapes:

```ts
interface KeyboardVisualState {
  orientation: "horizontal" | "vertical";
  firstNote: number; lastNote: number; octaveShift: number;
  keys: Array<{
    note: number; kind: "white" | "black";
    startNorm: number; lengthNorm: number; breadthNorm: number;
    held: boolean; externallyHeld: boolean; velocity: number | null;
    focused: boolean;
  }>;
  heldNotes: number[]; externalHeldNotes: number[];
  enabled: boolean;
}

interface WaveformVisualState {
  sampleCount: number; visibleStart: number; visibleEnd: number;
  columns: Array<{ min: number; max: number }>;
  cursorSample: number | null;
  selection: { start: number; end: number } | null;
  focus: boolean; enabled: boolean;
}

interface ModMatrixVisualState {
  sources: Array<{ id: string; label: string }>;
  destinations: Array<{ id: string; label: string }>;
  cells: Array<{
    sourceId: string; destinationId: string;
    amount: number; amountNorm: number; enabled: boolean; focused: boolean;
  }>;
  focus: { sourceId: string; destinationId: string } | null;
  enabled: boolean;
}
```

Keyboard geometry is resolved by the core. Horizontal orientation runs low to
high from left to right. Vertical orientation is the piano-roll gutter form:
high notes are at the top and key depth runs left to right. Pointer velocity is
the clamped depth within the key, quantized to MIDI `1..127`; renderers receive
the result but never calculate it. Local held notes and caller-supplied external
highlights remain distinct in VisualState so host playback does not fabricate
input gestures.

Waveform VisualState contains the exact reduced columns to draw. It never
contains raw samples or asks a renderer to choose a pyramid level. Mod-matrix
VisualState contains caller IDs and labels plus a row-major cell snapshot;
renderers do not infer synthesizer meaning or inspect machine context.

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
translation, and ARIA. Native adapters own the equivalent pointer, focus,
keyboard, accessibility-tree, and host-lifecycle translation. Core hit-test
helpers operate on geometry values only. Later skins may replace renderer
components or node builders without changing those paths.

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

## Keyboard Gestures

Keyboard note numbers are integers in MIDI range `0..127`. The configured
inclusive range is clamped to that domain. Octave shift changes computer-key
mapping in twelve-semitone increments without moving the visible key range.
The default map is `A W S E D F T G Y H U J K`, spanning one chromatic
octave; callers may supply another key-to-semitone map.

Pointer and computer-key press transitions emit one `noteOn` effect with the
resolved note and velocity. Release, cancellation, range removal, disablement,
or octave-shift removal emits one matching `noteOff`. Repeated computer-key
keydown events are ignored while that physical key is held. A note remains in
the local held set until every input source that holds it has released it.
External highlights never emit effects. This paired edge contract is the
per-note gesture lifecycle consumed by future MIDI and host adapters.

## Waveform Peak-pyramid Boundary

Waveform input is a peak pyramid, never raw PCM:

```ts
interface WaveformPeakPyramid {
  sampleCount: number;
  levels: Array<{
    samplesPerPeak: number;
    peaks: Array<{ min: number; max: number }>;
  }>;
}
```

Levels use positive integer `samplesPerPeak`, sorted from finest to coarsest.
Pairs are finite amplitudes clamped to `-1..1` with `min <= max`. The core
chooses the finest level that fits the requested viewport width, reduces it to
at most `WAVEFORM_MAX_COLUMNS = 4096`, and publishes only those columns in
VisualState. Malformed pyramids are rejected at the adapter boundary.

The component is limited to inspector and preview surfaces: clip thumbnails,
sample browsers, and plugin displays. It has no timeline tiles, streaming
cache, background reduction, raw-sample ingestion, or GPU scene ownership.
Timeline-scale rendering belongs to the consuming workstation's scene
substrate. Raising the 4,096-column ceiling or adding timeline facilities is an
architecture change, not a renderer option.

Cursor and ordered selection bounds are sample indices owned by the pure
machine. Pointer hit testing resolves a sample before transition. Arrow keys
move the cursor, Shift extends selection, Home/End move to viewport bounds,
and Escape clears selection. Drawing remains read-only: no transition mutates
audio or peak data.

## Mod-matrix Model

The mod matrix accepts stable source and destination IDs plus caller labels.
Each addressable cell owns `enabled` and a bipolar amount in `-1..1`, mapped
through the shared bipolar-center law. The core rejects duplicate IDs and
normalizes missing cells to disabled zero values. It contains no oscillator,
envelope, MIDI, or routing semantics.

Arrow keys move cell focus with clamped row and column navigation. Home/End
move to row bounds; Control+Home/End move to grid bounds. Space toggles the
focused cell, and Page Up/Down or modified arrows nudge amount. Adapters expose
row and column headers plus each cell's enabled state and formatted bipolar
amount through platform grid semantics. Renderers receive the flattened
VisualState only.

## Runtime Parity And Appearance

Standard renderers consume semantic tokens through complete
`--poodle-recipe-<component>-...` hook sets on web and the same semantic token
meanings through `ThemeProvider` on native. Svelte and React share CSS and web
machines. GPUI and Jetstream share Rust headless logic, specs, and node
builders; runtime differences are limited to backend input and accessibility
capabilities and must be recorded in each component contract.

Every audio component ships a standalone specimen page in all four previews.
The page must cover the contract's named states rather than merely prove the
component can mount. Parity evidence includes web interaction tests, native
headless/render tests, registry and adapter-manifest drift checks, preview
builds, and the applicable accessibility/visual reports.

The asset-skin runtime, Tier 2 custom renderer API, and host parameter binding
remain deferred.
