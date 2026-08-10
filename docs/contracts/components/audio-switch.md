# Audio Switch

Status: detailed contract
Updated: 2026-08-10

## 1. Purpose

- Component name: `AudioSwitch`
- Layer: `foundation`
- Summary: audio-domain latch, momentary, or discrete multi-state switch with independent lamp state

## 2. Anatomy

```text
[Root] button adapter
  [Visual] aria-hidden, AudioSwitchVisualState-only renderer
    [Body] [Handle] [Lamp]
```

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `mode` | `"latch" \| "momentary" \| "multi"` | `"latch"` | activation semantics |
| `state` | `number` | `0` | bindable discrete state |
| `stateCount` | `number` | `2` | at least two states |
| `lampOn` | `boolean \| null` | `null` | explicit lamp; null follows active state |
| `stateLabels` | `string[]` | `[]` | accessible labels by state |
| `disabled` | `boolean` | `false` | interaction guard |
| `ariaLabel` | `string \| null` | `"Audio switch"` | accessible name |
| `onStateChange`, `onStateCommit` | `(state) => void` | `undefined` | state callbacks |

## 4. States And Behavior Machine

Classification: machine-backed (`audioSwitchTransition`). Latch alternates
zero and one on activation. Momentary enters one on press and returns to zero
on release or cancel. Multi advances through `stateCount` cyclically. Pointer,
Space, and Enter share these rules. Lamp state never drives behavior.

## 5. Callbacks

Latch and multi emit change plus commit atomically. Momentary emits change on
both edges and commits on release.

## 6. Accessibility

The root is a button. Latch and momentary expose `aria-pressed`; multi appends
the current state label to its accessible name. Space and Enter operate it.

## 7. Layout

The standard renderer is an inline control. Labels remain parent-owned.

## 8. Token Usage

`--poodle-recipe-audio-switch-fill`, `-border`, `-handle`, `-active`,
`-lamp-off`, `-lamp-on`, `-focus-ring`, and `-disabled-opacity`.

## 9. Svelte Notes

The adapter owns button events and accessibility. `AudioSwitchVisual` receives
only VisualState.

## 10. GPUI Notes

Out of scope for this phase.

## 11. Parity Checklist

- same latch, momentary, and multi-state transitions
- lamp stays independently driven
- renderer never reads machine context

## 12. Known Deltas

Svelte only. React, GPUI, and Jetstream implementations are not included.

## 13. Approval And Adoption Notes

Phase 2 VisualState extension approved 2026-08-10.
