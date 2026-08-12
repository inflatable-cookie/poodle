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
| `size` | `ControlSize \| null` | `null` | explicit `xs`–`xl`; otherwise inherited |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | semantic inherited-size offset |
| `density` | `ControlDensity \| null` | `null` | compact, default, or comfortable treatment |
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
and drag end emit `onPointsCommit`. `onGestureBegin` and `onGestureEnd` pair
around pointer drag.

## 6. Accessibility

The root is a labelled group. Each point is an operable slider-like adapter
control with position text, selection, keyboard movement, deletion, and curve
keys. The renderer is hidden from assistive technology.

## 7. Layout

The editor fills its parent width and uses a configurable-height standard
surface. Point hit testing is core-owned normalized geometry.

## 8. Token Usage

Size changes editor width and height. Density changes curve and point-control
weight while normalized point geometry and hit-testing remain core-owned.

`--poodle-recipe-envelope-editor-fill`, `-border`, `-grid`, `-curve`,
`-point-fill`, `-point-border`, `-point-selected`, `-focus-ring`, and
`-disabled-opacity`.

## 9. Svelte Notes

The adapter owns pointer capture, snap callback invocation, and ARIA controls.
`EnvelopeVisual` receives only `EnvelopeVisualState`.

## 9a. React Notes

The React shell runs the same point machine, snap hook, hit tests, and curve
helper. `EnvelopeVisual` receives only `EnvelopeVisualState`.

## 10. GPUI Notes

The adapter owns point hit testing, drag capture, focus, and key translation.
The shared node builder consumes ordered point VisualState only.

## 10a. Jetstream Notes

Jetstream consumes the same Rust point machine, spec, and node builder. Point
controls are exposed as individually named accessibility children.

## 11. Parity Checklist

- same normalized ordering, add/remove/drag, snapping, curve bounds, and keys
- same point selection and formatted accessible position
- renderer never reads machine context

## 12. Known Deltas

Web renders the exact sampled curve as SVG. Native renderers show the same
ordered points and sampled monotonic curve as node segments; antialiasing and
segment rasterization may differ.

## 13. Specimen Definitions

All four previews provide ADSR-like default, positive/negative curve amounts,
selected and dragging points, add/remove, snapped movement, keyboard nudges,
curve nudges, disabled state, and a flat-segment regression case.

## 14. Approval And Adoption Notes

The specimen page includes the full five-size and three-density matrices.

Phase 2 VisualState extension approved 2026-08-10.
