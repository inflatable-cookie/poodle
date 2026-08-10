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
| `size` | `ControlSize \| null` | `null` | explicit `xs`–`xl`; otherwise inherited |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | semantic inherited-size offset |
| `density` | `ControlDensity \| null` | `null` | compact, default, or comfortable treatment |
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

Size changes meter length and thickness. Density changes track padding and
segment gaps without changing inverted-scale ballistics.

`--poodle-recipe-gain-reduction-meter-fill`, `-border`, `-track`, `-active`,
`-segment-off`, `-segment-on`, `-text`, and `-disabled-opacity`.

## 9. Svelte Notes

`GainReductionMeterVisual` receives only VisualState and display geometry.

## 9a. React Notes

React exposes the same `push` and `reset` imperative handle and passes only
VisualState plus display geometry to its renderer.

## 10. GPUI Notes

Host state pushes reduction frames through the Rust transition. The shared
renderer receives only the resulting VisualState.

## 10a. Jetstream Notes

Jetstream uses the same Rust feed transition, spec, and node builder with a
deterministic preview clock.

## 11. Parity Checklist

- same positive-magnitude feed, attack/release, invalid-frame handling
- same inverted normalized scale and accessible dB text
- renderer never reads machine context

## 12. Known Deltas

Native uses discrete token-themed segments for both styles. Positive reduction
magnitude, inverted direction, attack/release constants, and value text are
strict.

## 13. Specimen Definitions

All four previews provide no reduction, attack, release, maximum reduction,
bar and segment styles, vertical and horizontal orientation, invalid-frame
rejection, disabled state, and reset evidence.

## 14. Approval And Adoption Notes

The specimen page includes the full five-size and three-density matrices.

Phase 2 VisualState extension approved 2026-08-10.
