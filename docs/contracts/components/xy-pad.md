# XY Pad

Status: detailed contract
Updated: 2026-08-29

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
| `size` | `ControlSize \| null` | `null` | explicit `xs`–`xl`; otherwise inherited |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | semantic inherited-size offset |
| `density` | `ControlDensity \| null` | `null` | compact, default, or comfortable treatment |
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
both laws atomically. A coarse press moves the pair to the accepted press
position; a fine press only anchors. Shift selects fine movement at one tenth
travel, and switching modifier re-anchors both axes at the current pair and
pointer. Double-click restores the default pair. Axis sliders use arrows, Page
Up/Down, Home, and End for independent keyboard edits. A disabled pad rejects
every user mutation, while host pair replacement, automation state,
hover/focus reporting, and the terminal of a gesture accepted while enabled
stay live.

## 5. Callbacks

Pair values are never emitted separately. Live drags emit `onValueChange`;
atomic keys and reset plus drag end emit `onValueCommit`. `onGestureBegin` and
`onGestureEnd` pair exactly once around drag. One primary pointer owns a drag;
a second pointer-down is ignored. Release, cancellation, lost pointer capture,
and adapter teardown all close an accepted drag through the same terminal,
exactly once.

## 6. Accessibility

A labelled group contains X and Y slider semantics with independent bounds,
current values, and formatted value text. The renderer is aria-hidden.

## 7. Layout

The standard surface is square. Core rectangle hit testing supplies normalized
coordinates; x increases right and y increases upward.

## 8. Token Usage

Size changes pad width and height. Density changes thumb weight without
changing the two normalized axes or adapter-owned hit-testing.

`--poodle-recipe-xy-pad-fill`, `-border`, `-grid`, `-trace`, `-thumb-fill`,
`-thumb-border`, `-focus-ring`, and `-disabled-opacity`.

## 9. Svelte Notes

The root adapter owns pointer capture and child slider ARIA. `XYPadVisual`
receives only VisualState.

## 9a. React Notes

React runs the same atomic two-axis machine and passes only VisualState to
`XYPadVisual`; two hidden slider adapters own axis accessibility.

## 10. GPUI Notes

The adapter maps pointer geometry from Node `on_continuous_value` (x right,
y up) before transition and exposes two slider accessibility children.
Reset uses `on_double_activate`. Native construction requires
`XYPadHandlers::new(instance_id)` with a lifetime-stable instance scope for
the group and both axis sliders. The shared renderer consumes one atomic
VisualState.

## 10a. Jetstream Notes

Jetstream uses the same Rust transition, spec, and node builder and preserves
atomic pair effects.

## 11. Parity Checklist

- same independent laws, atomic pair effects, fine drag, reset, and keys
- same accessible axis values
- renderer never reads machine context

## 12. Known Deltas

Runtime pointer capture differs. Axis laws, pair effects, reset, value text,
and focus semantics are strict.

## 13. Specimen Definitions

All four previews provide centered/default, corners, independent nonlinear
laws, coarse/fine drag, reset, automation state, keyboard axis bounds, and
disabled groups.

## 14. Approval And Adoption Notes

The specimen page includes the full five-size and three-density matrices.

Phase 2 VisualState extension approved 2026-08-10.
