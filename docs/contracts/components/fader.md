# Fader

Status: detailed contract
Updated: 2026-08-10

## 1. Purpose

- Component name: `Fader`
- Layer: `foundation`
- Summary: axis-based audio value control with detents and host gesture state

## 2. Anatomy

Audio fader with horizontal or vertical geometry, audio value laws, optional
detents, and explicit gesture state.

```text
[Root] role=slider
  [Visual] aria-hidden, VisualState-only renderer
    [Track] [Fill] [Detents] [Thumb]
  [Entry] conditional numeric text input
```

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `value` | `number` | `0` | bindable plain value |
| `min` | `number` | `0` | lower plain bound |
| `max` | `number` | `1` | upper plain bound |
| `law` | `AudioValueLaw` | `linear` | normalized mapping |
| `orientation` | `"vertical" \| "horizontal"` | `"vertical"` | axis geometry |
| `detents` | `number[]` | `[]` | plain snap values |
| `detentSnap` | `number` | `0.015` | normalized snap radius |
| `defaultValue` | `number` | `0` | reset target |
| `keyboardStep` | `number` | `0.01` | plain-value nudge |
| `format` | `AudioValueFormat` | number | display and parse vocabulary |
| `automation` | `AudioAutomationState` | `"none"` | host display state |
| `disabled` | `boolean` | `false` | interaction guard |
| `ariaLabel` | `string \| null` | `null` | accessible name |
| `onValueChange` | `(value: number) => void` | `undefined` | live value effect |
| `onValueCommit` | `(value: number) => void` | `undefined` | commit effect |
| `onGestureBegin` | `() => void` | `undefined` | gesture start effect |
| `onGestureEnd` | `() => void` | `undefined` | gesture end effect |

## 4. States And Behavior Machine

Classification: machine-backed (`faderTransition`). Pointer position maps to
normalized value on the active axis. Shift selects fine dragging. Detents snap
within the declared normalized radius. Pointer start/end emit paired gesture
effects and expose `drag = coarse|fine|none` in VisualState. Wheel, reset,
keyboard, and type-in follow Knob semantics.

## 5. Callbacks

`onValueChange` reports live changes. `onValueCommit` reports atomic changes
and drag end. Gesture callbacks pair exactly once around pointer drags.

## 6. Accessibility

Root exposes slider role, orientation, min/max/now, formatted value text,
disabled state, and full keyboard operation. Input handling and semantics are
outside the renderer.

## 7. Layout

Orientation is a geometry input. The parent owns overall track length.

## 8. Token Usage

`--poodle-recipe-fader-track-fill`, `-track-border`, `-fill-fill`,
`-thumb-fill`, `-thumb-border`, `-thumb-shadow`, `-detent-fill`,
`-entry-fill`, `-entry-border`, `-entry-text`, `-focus-ring`, and
`-disabled-opacity`.

## 9. Svelte Notes

The root adapter owns pointer-axis mapping and ARIA. `FaderVisual` receives
VisualState plus serializable orientation and detent geometry.

## 10. GPUI Notes

Out of scope for Phase 1. Gesture effects are preserved for the later host
adapter.

## 11. Parity Checklist

- identical law, detent, fine-drag, reset, entry, and gesture behavior
- equivalent slider role, orientation, and value text
- renderer never reads machine context

## 12. Known Deltas

Svelte only. React, GPUI, and Jetstream implementations are not included.

## 13. Approval And Adoption Notes

Phase 1 review in Loophole gates further audio-family work.
