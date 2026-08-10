# Gain Reduction Meter

Status: detailed contract
Updated: 2026-08-10

## 1. Purpose

- Component name: `GainReductionMeter`
- Layer: `foundation`
- Summary: inverted audio meter for positive gain-reduction magnitude

## 2. Anatomy

```text
[Root] role=meter
  [Visual] aria-hidden, GainReductionMeterVisualState-only renderer
```

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `context` | `GainReductionMeterContext` | created default | bindable feed state |
| `style` | `"bar" \| "segments"` | `"segments"` | standard renderer style |
| `orientation` | `"horizontal" \| "vertical"` | `"vertical"` | inverted display axis |
| `segments` | `number` | `20` | segment count |
| `ariaLabel` | `string \| null` | `"Gain reduction"` | accessible name |

The component exports `push(frame)` and `reset()` methods. Frames contain
finite `atMs`, non-negative `reductionDb`, and positive `durationMs`.

## 4. States And Behavior Machine

Classification: machine-backed (`gainReductionMeterTransition`). The core
uses 10 ms attack and 300 ms release. Frames must be timestamp-monotonic.
Invalid or stale frames are inert.

## 5. Callbacks

None. Hosts own feed cadence and bindable context observation.

## 6. Accessibility

The root exposes meter semantics from zero to configured maximum reduction,
with formatted dB value text. The renderer is aria-hidden.

## 7. Layout

The standard renderer supports horizontal and vertical inverted axes, bar or
segment presentation.

## 8. Token Usage

`--poodle-recipe-gain-reduction-meter-fill`, `-border`, `-track`, `-active`,
`-segment-off`, `-segment-on`, `-text`, and `-disabled-opacity`.

## 9. Svelte Notes

`GainReductionMeterVisual` receives only VisualState and display geometry.

## 10. GPUI Notes

Out of scope for this phase.

## 11. Parity Checklist

- same positive-magnitude feed, attack/release, invalid-frame handling
- same inverted normalized scale and accessible dB text
- renderer never reads machine context

## 12. Known Deltas

Svelte only. React, GPUI, and Jetstream implementations are not included.

## 13. Approval And Adoption Notes

Phase 2 VisualState extension approved 2026-08-10.
