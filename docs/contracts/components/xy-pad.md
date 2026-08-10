# XY Pad

Status: detailed contract
Updated: 2026-08-10

## 1. Purpose

- Component name: `XYPad`
- Layer: `foundation`
- Summary: atomic two-axis audio value control with independent value laws

## 2. Anatomy

```text
[Root] labelled group and pointer adapter
  [Visual] aria-hidden, XYPadVisualState-only renderer
  [X slider] [Y slider] accessible adapter controls
```

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `x`, `y` | `number` | `0` | bindable axis values |
| `minX`, `minY` | `number` | `0` | lower bounds |
| `maxX`, `maxY` | `number` | `1` | upper bounds |
| `lawX`, `lawY` | `AudioValueLaw` | linear | independent mappings |
| `defaultX`, `defaultY` | `number` | `0` | reset pair |
| `keyboardStepX`, `keyboardStepY` | `number` | `0.01` | plain-value nudges |
| `formatX`, `formatY` | `AudioValueFormat` | number | accessible value text |
| `automation` | `AudioAutomationState` | `"none"` | host display state |
| `disabled` | `boolean` | `false` | interaction guard |
| `ariaLabel` | `string \| null` | `null` | group label |
| `onValueChange`, `onValueCommit` | `(x, y) => void` | `undefined` | pair callbacks |
| `onGestureBegin`, `onGestureEnd` | `() => void` | `undefined` | gesture callbacks |

## 4. States And Behavior Machine

Classification: machine-backed (`xyPadTransition`). Pointer position maps to
both laws atomically. Shift selects fine movement at one tenth travel.
Double-click restores the default pair. Axis sliders use arrows, Page Up/Down,
Home, and End for independent keyboard edits.

## 5. Callbacks

Pair values are never emitted separately. Live drags emit change; atomic keys
and reset plus drag end emit commit. Gesture callbacks pair around drag.

## 6. Accessibility

A labelled group contains X and Y slider semantics with independent bounds,
current values, and formatted value text. The renderer is aria-hidden.

## 7. Layout

The standard surface is square. Core rectangle hit testing supplies normalized
coordinates; x increases right and y increases upward.

## 8. Token Usage

`--poodle-recipe-xy-pad-fill`, `-border`, `-grid`, `-trace`, `-thumb-fill`,
`-thumb-border`, `-focus-ring`, and `-disabled-opacity`.

## 9. Svelte Notes

The root adapter owns pointer capture and child slider ARIA. `XYPadVisual`
receives only VisualState.

## 10. GPUI Notes

Out of scope for this phase.

## 11. Parity Checklist

- same independent laws, atomic pair effects, fine drag, reset, and keys
- same accessible axis values
- renderer never reads machine context

## 12. Known Deltas

Svelte only. React, GPUI, and Jetstream implementations are not included.

## 13. Approval And Adoption Notes

Phase 2 VisualState extension approved 2026-08-10.
