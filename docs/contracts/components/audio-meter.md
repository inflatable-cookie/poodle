# Audio Meter

Status: detailed contract
Updated: 2026-08-14

## 1. Purpose

- Component name: `AudioMeter`
- Layer: `foundation`
- Summary: temporal mono or stereo audio level display with core-owned ballistics

## 2. Anatomy

Temporal audio level display with VU, PPM, sample-peak, or RMS ballistics. This
is distinct from the existing bounded-value `Meter`.

```text
[Root] role=meter
  [Channel] one or two
    [Visual] aria-hidden, VisualState-only renderer
      [Track] [Bar or Segments] [Peak Hold] [Clip]
```

Surface mode keeps `[Root]` as a measured layout and accessibility box but
omits every `[Visual]`. The nearest matching web `MeterSurface` paints it into
one shared accessibility-hidden canvas.

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `size` | `ControlSize \| null` | `null` | explicit `xs`–`xl`; otherwise inherited |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | semantic inherited-size offset |
| `density` | `ControlDensity \| null` | `null` | compact, default, or comfortable treatment |
| `context` | `AudioMeterContext` | new sample-peak context | bindable left/mono machine |
| `rightContext` | `AudioMeterContext \| null` | `null` | bindable stereo channel |
| `style` | `"bar" \| "segments"` | `"segments"` | themed drawing style |
| `orientation` | `"vertical" \| "horizontal"` | `"vertical"` | channel axis |
| `segments` | `number` | `20` | segment geometry count |
| `ariaLabel` | `string \| null` | `null` | accessible name |

Mode and dB bounds live in each context. Contexts advance through exported
`audioMeterTransition` calls; the component also exposes `push(frame,
channel?)` and `resetClip(channel?)` handles for UI hosts.

### Planned Surface Props (g14.024)

These props are approved but not public until g14.024 lands. The implementation
PR moves them into `Public Props` and must keep contract drift green.

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `surface` | `MeterBus \| null` | `null` | web-only opt-in batch renderer; `null` keeps standalone behavior |
| `channel` | `MeterBusChannelId \| null` | `null` | required in surface mode; must already be registered on `surface` |
| `rightChannel` | `MeterBusChannelId \| null` | `null` | optional registered stereo channel in surface mode |

`surface=null` is the existing standalone tier and remains the default.
`surface!=null` requires `channel`; `context` and `rightContext` are not state
authority in that tier. `push` and `resetClip` forward to the registered bus
slots without allocating a feed buffer per call. Missing registrations,
cross-bus channels, and mixed standalone/surface authority are development
errors, never silent fallback.

## 4. States And Behavior Machine

Classification: machine-backed (`audioMeterTransition`). `PUSH_FRAME` applies
the constants and feed protocol in architecture 008. `RESET_CLIP` is the only
clip-clear path. `RESET` clears temporal state. Stereo uses two independent
machines and two VisualStates.

`MeterBus` is an imperative engine, not a component machine. Its per-channel
math functions remain pure. The bus may schedule explicit time-advance steps
between host telemetry pushes; golden tests apply the same push/time sequence
to standalone math and batch slots. Animation cadence may differ, but a mode's
numeric law may not.

## 5. Events

The display emits no user events. Hosts advance contexts with `PUSH_FRAME` or
the Svelte handle's `push` method.

## 6. Accessibility

Root exposes meter semantics and a formatted dB value text. It is not
keyboard-focusable. Clip reset is host-owned; any visible reset control must
be a separate accessible button.

Surface mode preserves the same role, name, min, max, current value, and value
text on the placeholder. It refreshes the current value at most twice per
second from the bus and performs one immediate initial update. Canvas content
is `aria-hidden` and never duplicates the meter in the accessibility tree.

## 7. Layout

Mono renders one channel; stereo renders two adjacent channels. Orientation
selects the bar axis. Segment count changes renderer geometry only.

In surface mode the root keeps the same size, density, orientation, channel
count, recipe hooks, and flex/grid participation. It contains no visual child.
One `MeterSurface` owns one scroll container, caches placeholder geometry on
layout changes, and culls off-viewport paint without suspending ballistics.

## 8. Token Usage

Size changes meter length and channel thickness. Density changes channel and
segment gaps; it never changes ballistics or feed integration.

`--poodle-recipe-audio-meter-track-fill`, `-track-border`, `-bar-fill`,
`-segment-off-fill`, `-segment-on-fill`, `-segment-warning-fill`,
`-segment-clip-fill`, `-peak-fill`, `-clip-fill`, and `-disabled-opacity`.

## 9. Svelte Notes

`AudioMeterVisual` receives one channel VisualState. Stereo composes it twice.
The root owns meter semantics and imperative feed handles.

## 9a. React Notes

The React component exposes the same `push` and `resetClip` imperative handle,
uses the shared web integration core, and passes per-channel VisualState to
`AudioMeterVisual`.

Both web implementations expose matching surface props and compose the shared
DOM surface controller. They do not duplicate bus, geometry, palette, or
painter policy.

## 10. GPUI Notes

Host state pushes aggregate frames through the Rust meter transition. The
shared renderer consumes only the resulting per-channel VisualState.

## 10a. Jetstream Notes

Jetstream uses the same feed transition and node builder. Its preview clock
pushes deterministic aggregate frames rather than audio-rate samples.

`MeterSurface` has no native component counterpart. Native backends already
batch AudioMeter nodes in their renderer scene; this is an accepted
mechanism-only web delta, not missing observable behavior.

## 11. Parity Checklist

- identical constants, RMS window, peak hold, decay, and clip latch
- same mono/stereo and bar/segment semantics
- drawing reads VisualState only
- standalone remains the default and its public behavior is unchanged
- surface placeholders expose the same meter accessibility semantics
- batch and standalone math match for identical push/time sequences

## 12. Known Deltas

Native meters use discrete token-themed segments for both display styles;
native nodes have no CSS gradient primitive with equivalent stop semantics.
Ballistics, thresholds, stereo composition, peak hold, and clip latch are
strict.

## 13. Specimen Definitions

All four previews provide deterministic VU, PPM, sample-peak, and RMS groups;
bar and segment styles; mono and stereo; vertical and horizontal orientation;
peak hold; clipped/latched state; and manual clip reset evidence.

The Svelte and React previews additionally provide one batched surface page
with 8, 32, and 128 meters, scroll culling, live theme switching, stereo,
clip reset, and a visible performance probe. This web-only rendering evidence
does not add a native specimen requirement.

## 14. Approval And Adoption Notes

The specimen page includes the full five-size and three-density matrices.

Phase 1 review must exercise a real Loophole telemetry feed.

The batched web tier is governed by spec 068 and g14.024. WebGL2 remains
outside that card until the Canvas2D budget is measured.
