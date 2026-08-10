# Audio Meter

Status: detailed contract
Updated: 2026-08-10

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

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `context` | `AudioMeterContext` | new sample-peak context | bindable left/mono machine |
| `rightContext` | `AudioMeterContext \| null` | `null` | bindable stereo channel |
| `style` | `"bar" \| "segments"` | `"segments"` | themed drawing style |
| `orientation` | `"vertical" \| "horizontal"` | `"vertical"` | channel axis |
| `segments` | `number` | `20` | segment geometry count |
| `ariaLabel` | `string \| null` | `null` | accessible name |

Mode and dB bounds live in each context. Contexts advance through exported
`audioMeterTransition` calls; the component also exposes `push(frame,
channel?)` and `resetClip(channel?)` handles for UI hosts.

## 4. States And Behavior Machine

Classification: machine-backed (`audioMeterTransition`). `PUSH_FRAME` applies
the constants and feed protocol in architecture 008. `RESET_CLIP` is the only
clip-clear path. `RESET` clears temporal state. Stereo uses two independent
machines and two VisualStates.

## 5. Events

The display emits no user events. Hosts advance contexts with `PUSH_FRAME` or
the Svelte handle's `push` method.

## 6. Accessibility

Root exposes meter semantics and a formatted dB value text. It is not
keyboard-focusable. Clip reset is host-owned; any visible reset control must
be a separate accessible button.

## 7. Layout

Mono renders one channel; stereo renders two adjacent channels. Orientation
selects the bar axis. Segment count changes renderer geometry only.

## 8. Token Usage

`--poodle-recipe-audio-meter-track-fill`, `-track-border`, `-bar-fill`,
`-segment-off-fill`, `-segment-on-fill`, `-segment-warning-fill`,
`-segment-clip-fill`, `-peak-fill`, `-clip-fill`, and `-disabled-opacity`.

## 9. Svelte Notes

`AudioMeterVisual` receives one channel VisualState. Stereo composes it twice.
The root owns meter semantics and imperative feed handles.

## 10. GPUI Notes

Out of scope for Phase 1. Aggregate feed frames and VisualState port without
DOM dependencies.

## 11. Parity Checklist

- identical constants, RMS window, peak hold, decay, and clip latch
- same mono/stereo and bar/segment semantics
- drawing reads VisualState only

## 12. Known Deltas

Svelte only. React, GPUI, and Jetstream implementations are not included.

## 13. Approval And Adoption Notes

Phase 1 review must exercise a real Loophole telemetry feed.
