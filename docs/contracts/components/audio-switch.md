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
| `size` | `ControlSize \| null` | `null` | explicit `xs`–`xl`; otherwise inherited |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | semantic inherited-size offset |
| `density` | `ControlDensity \| null` | `null` | compact, default, or comfortable treatment |
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

Latch and multi emit `onStateChange` plus `onStateCommit` atomically.
Momentary emits `onStateChange` on both edges and `onStateCommit` on release.

## 6. Accessibility

The root is a button. Latch and momentary expose `aria-pressed`; multi appends
the current state label to its accessible name. Space and Enter operate it.

## 7. Layout

The standard renderer is an inline control. Labels remain parent-owned.

## 8. Token Usage

Size changes the switch footprint and lamp/body scale. Density changes
internal padding and gap without changing latch or momentary semantics.

`--poodle-recipe-audio-switch-fill`, `-border`, `-handle`, `-active`,
`-lamp-off`, `-lamp-on`, `-focus-ring`, and `-disabled-opacity`.

## 9. Svelte Notes

The adapter owns button events and accessibility. `AudioSwitchVisual` receives
only VisualState.

## 9a. React Notes

React runs the same switch transition and shared CSS. `AudioSwitchVisual`
receives only VisualState.

## 10. GPUI Notes

The adapter owns press/release/cancel and focus translation. The shared node
builder consumes VisualState with lamp state distinct from selected state.

## 10a. Jetstream Notes

Jetstream consumes the same Rust transition, spec, and node builder and maps
the result to button/toggle accessibility.

## 11. Parity Checklist

- same latch, momentary, and multi-state transitions
- lamp stays independently driven
- renderer never reads machine context

## 12. Known Deltas

Multi-state switches expose their current state through accessible label text
because platform toggle roles are binary. Latch and momentary toggles use the
native pressed/toggled channel.

## 13. Specimen Definitions

All four previews provide off/on latch, held/released momentary, three-state
cycling with labels, lamp override independent of selection, pressed/focused,
and disabled groups.

## 14. Approval And Adoption Notes

The specimen page includes the full five-size and three-density matrices.

Phase 2 VisualState extension approved 2026-08-10.
