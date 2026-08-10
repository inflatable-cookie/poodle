# Envelope Editor

Status: detailed contract
Updated: 2026-08-10

## 1. Purpose

- Component name: `EnvelopeEditor`
- Layer: `foundation`
- Summary: editable normalized point-and-curve surface for envelopes and reusable curves

## 2. Anatomy

```text
[Root] labelled group
  [Visual] aria-hidden, EnvelopeVisualState-only renderer
  [Point controls] keyboard and screen-reader adapter layer
```

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `points` | `EnvelopePoint[]` | `[]` | bindable ordered normalized points |
| `step` | `number` | `0.01` | keyboard movement |
| `disabled` | `boolean` | `false` | interaction guard |
| `ariaLabel` | `string \| null` | `null` | accessible group name |
| `snapPoint` | `(point, points) => point` | identity | host snap hook |
| `onPointsChange` | `(points) => void` | `undefined` | live edit callback |
| `onPointsCommit` | `(points) => void` | `undefined` | completed edit callback |
| `onGestureBegin` | `() => void` | `undefined` | drag start callback |
| `onGestureEnd` | `() => void` | `undefined` | drag end callback |

Each point has stable `id`, normalized `x` and `y`, and outgoing `curve` in
`-1..1`. Duplicate ids are rejected; coordinates and curves are clamped and
points are ordered by x. Curve interpolation follows Architecture 008 through
the shared `envelopeSegmentValueAt` geometry helper.

## 4. States And Behavior Machine

Classification: machine-backed (`envelopeTransition`). Double-clicking empty
space adds a point. Pointer drag moves a point. Delete/Backspace removes the
selected point. Arrows move it; Shift uses one tenth step. `[` and `]` change
its outgoing curve. Snap hooks run in the adapter and the resolved point is
supplied to the pure transition.

## 5. Callbacks

Live mutations emit `onPointsChange`; add, remove, keyboard edits, curve edits,
and drag end emit `onPointsCommit`. Gesture callbacks pair around pointer drag.

## 6. Accessibility

The root is a labelled group. Each point is an operable slider-like adapter
control with position text, selection, keyboard movement, deletion, and curve
keys. The renderer is hidden from assistive technology.

## 7. Layout

The editor fills its parent width and uses a configurable-height standard
surface. Point hit testing is core-owned normalized geometry.

## 8. Token Usage

`--poodle-recipe-envelope-editor-fill`, `-border`, `-grid`, `-curve`,
`-point-fill`, `-point-border`, `-point-selected`, `-focus-ring`, and
`-disabled-opacity`.

## 9. Svelte Notes

The adapter owns pointer capture, snap callback invocation, and ARIA controls.
`EnvelopeVisual` receives only `EnvelopeVisualState`.

## 10. GPUI Notes

Out of scope for this phase.

## 11. Parity Checklist

- same normalized ordering, add/remove/drag, snapping, curve bounds, and keys
- same point selection and formatted accessible position
- renderer never reads machine context

## 12. Known Deltas

Svelte only. React, GPUI, and Jetstream implementations are not included.

## 13. Approval And Adoption Notes

Phase 2 VisualState extension approved 2026-08-10.
