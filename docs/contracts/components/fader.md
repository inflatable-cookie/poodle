# Fader

Status: detailed contract
Updated: 2026-08-29

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
| `size` | `ControlSize \| null` | `null` | explicit `xs`–`xl`; otherwise inherited |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | semantic inherited-size offset |
| `density` | `ControlDensity \| null` | `null` | compact, default, or comfortable treatment |
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
normalized value on the active axis; orientation chooses the axis and never
enters the value law. Shift selects fine dragging, and switching modifier
re-anchors at the current value and pointer. Detents snap within the declared
normalized radius; the radius is inclusive and the first declared detent wins
an exact tie. Pointer start/end emit paired gesture effects and expose
`drag = coarse|fine|none` in VisualState. Wheel, reset, keyboard, and type-in
follow Knob semantics, including its Enter/Escape/blur entry closure. A
disabled fader is inert on every route.

## 5. Callbacks

`onValueChange` reports live changes. `onValueCommit` reports atomic changes
and drag end. `onGestureBegin` and `onGestureEnd` pair exactly once around
pointer drags. One primary pointer owns a drag; a second pointer-down is
ignored. Release, cancellation, lost pointer capture, and adapter teardown all
close an accepted drag through the same terminal, exactly once.

## 6. Accessibility

Root exposes slider role, orientation, min/max/now, formatted value text,
disabled state, and full keyboard operation. Input handling and semantics are
outside the renderer. The root's machine ring (box-shadow on the thumb) covers
root focus; the entry input draws the standard ring (`border-width-focus` solid
`accent-focusRing`, offset `0.125rem`) while editing.

## 7. Layout

Orientation is a geometry input. The parent owns overall track length.

## 8. Token Usage

Size changes track length and cross-axis footprint. Density changes rail
weight while preserving normalized value and hit-test geometry.

`--poodle-recipe-fader-track-fill`, `-track-border`, `-fill-fill`,
`-thumb-fill`, `-thumb-border`, `-thumb-shadow`, `-detent-fill`,
`-entry-fill`, `-entry-border`, `-entry-text`, `-focus-ring`, and
`-disabled-opacity`.

## 9. Svelte Notes

The root adapter owns pointer-axis mapping and ARIA. `FaderVisual` receives
VisualState plus serializable orientation and detent geometry.

## 9a. React Notes

The React shell runs the same fader machine and shared CSS. `FaderVisual`
receives only VisualState, orientation, and normalized detent positions.

## 10. GPUI Notes

The adapter maps track-relative pointer positions before calling the Rust
machine. The shared renderer receives VisualState and serializable geometry.

## 10a. Jetstream Notes

Jetstream uses the same Rust machine/spec/node builder and preserves paired
gesture begin/end effects for host automation.

## 11. Parity Checklist

- identical law, detent, fine-drag, reset, entry, and gesture behavior
- equivalent slider role, orientation, and value text
- renderer never reads machine context

## 12. Known Deltas

Pointer capture mechanics are runtime-owned. Detent resolution, values,
gesture effects, orientation, and accessible text are strict.

## 13. Specimen Definitions

All four previews provide standalone groups for vertical and horizontal
orientation, linear/log/bipolar laws, detents, coarse/fine drag, automation
touch state, type-in, disabled state, and keyboard bounds.

## 14. Approval And Adoption Notes

The specimen page includes the full five-size and three-density matrices.

Phase 1 review in Loophole gates further audio-family work.
