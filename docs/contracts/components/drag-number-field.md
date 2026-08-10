# Drag Number Field

Status: detailed contract
Updated: 2026-08-10

## 1. Purpose

- Component name: `DragNumberField`
- Layer: `foundation`
- Summary: compact numeric entry with pointer drag and direct type-in

Compact editable numeric field supporting horizontal drag and direct type-in,
using the shared audio formatter.

## 2. Anatomy

```text
[Root] role=spinbutton
  [Visual] aria-hidden, VisualState plus core-formatted text
  [Entry] conditional labelled input
```

## 3. Props, States, And Behavior

### Public Props

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `value` | `number` | `0` | bindable plain value |
| `min` | `number` | minimum safe integer | lower bound |
| `max` | `number` | maximum safe integer | upper bound |
| `step` | `number` | `1` | snap and keyboard step |
| `dragSensitivity` | `number` | `0.1` | plain units per pixel |
| `format` | `AudioValueFormat` | number | display and parse vocabulary |
| `disabled` | `boolean` | `false` | interaction guard |
| `ariaLabel` | `string \| null` | `null` | accessible name |
| `onValueChange` | `(value: number) => void` | `undefined` | live value effect |
| `onValueCommit` | `(value: number) => void` | `undefined` | commit effect |
| `onGestureBegin` | `() => void` | `undefined` | gesture start effect |
| `onGestureEnd` | `() => void` | `undefined` | gesture end effect |

Classification: machine-backed (`dragNumberTransition`). Pointer drag emits
paired gesture effects; Shift is fine adjustment. Click or Enter opens text
entry. Escape cancels. Enter or blur commits valid parsed text. Arrows nudge,
Home/End select bounds.

The display renderer consumes serializable VisualState only. The adapter owns
the input element, hit-testing, pointer capture, focus, and ARIA spinbutton
semantics.

## 4. States

Idle, hover, focus, coarse drag, fine drag, direct entry, and disabled states
are core-owned.

## 5. Callbacks

Value callbacks split live change from commit. Gesture callbacks pair exactly
once around pointer drag begin and end.

## 6. Accessibility

Root exposes spinbutton role, name, min/max/now, formatted value text, disabled
state, and arrow/Home/End keyboard operation. Type-in uses a labelled input.

## 7. Layout

Compact intrinsic-width field with tabular numerals. Direct entry overlays the
same footprint.

## 8. Token Usage

`--poodle-recipe-drag-number-field-fill`, `-text`, `-border`, `-hover-fill`,
`-drag-fill`, `-entry-fill`, `-entry-border`, `-entry-text`, `-focus-ring`, and
`-disabled-opacity`.

## 9. Svelte Notes

The adapter owns pointer capture and the conditional input. `ValueVisual`
receives VisualState plus core-formatted text.

## 10. GPUI Notes

Out of scope for Phase 1. Native direct-entry affordances need later design.

## 11. Parity Checklist

- same drag, fine, parse, clamp, keyboard, and gesture behavior
- equivalent spinbutton value semantics
- renderer never handles input

## 12. Known Deltas

Svelte only. React, GPUI, and Jetstream implementations are not included.

## 13. Approval And Adoption Notes

Phase 1 review in Loophole gates wider adoption.
