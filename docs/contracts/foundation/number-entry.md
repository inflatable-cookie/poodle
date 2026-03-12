# Number Entry

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `NumberEntry`
- Layer: `foundation`
- Summary: a numeric text-entry control with optional increment/decrement
  affordances and constrained numeric semantics
- In scope: numeric value entry, validation bounds, step behavior, optional
  steppers
- Out of scope: slider interaction, knob/fader semantics, scientific editors

## 2. Anatomy

```text
[Root]
  ├── [Input Control]
  └── [Stepper Controls] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | numeric field chrome | background, border, radius, focus ring |
| Input Control | yes | editable numeric text surface | typography, text color |
| Stepper Controls | no | increment/decrement affordances | icon/action tokens |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `number \| null` | `null` | no | controlled numeric value |
| `defaultValue` | `number \| null` | `null` | no | uncontrolled initial value |
| `placeholder` | `string \| null` | `null` | no | optional hint |
| `min` | `number \| null` | `null` | no | lower bound |
| `max` | `number \| null` | `null` | no | upper bound |
| `step` | `number` | `1` | no | increment/decrement size |
| `precision` | `number \| null` | `null` | no | optional decimal formatting hint |
| `isDisabled` | `boolean` | `false` | no | disables editing |
| `isReadOnly` | `boolean` | `false` | no | allows selection without editing |
| `validationState` | `"none" \| "invalid" \| "valid" \| "pending"` | `"none"` | no | state treatment |
| `showSteppers` | `boolean` | `false` | no | shows increment/decrement controls |
| `ariaLabel` | `string \| null` | `null` | no | required when no external label exists |
| `onValueChange` | `(value: number \| null) => void` | none | no | value change callback |
| `onSubmit` | `(value: number \| null) => void` | none | no | commit callback |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange`
- uncontrolled: `defaultValue`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | neutral numeric field |
| focus | focused | visible active treatment |
| invalid | `validationState="invalid"` or parse/bounds failure | error emphasis |
| disabled | `isDisabled=true` | muted field |
| readOnly | `isReadOnly=true` | selectable without editing |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| valid numeric text | user enters parseable value | numeric value available |
| transient text | user enters partial numeric text | edit preserved until commit/blur policy resolves it |
| bounds-adjusted | stepper or commit exceeds min/max | clamped or rejected per contract policy |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | numeric value meaningfully changes | `number \| null` | partial invalid text should not emit misleading numeric values |
| `onSubmit` | enter or explicit commit confirms current value | `number \| null` | optional |
| `onIncrement` | stepper or keyboard increment action occurs | `number \| null` | optional higher-level passthrough |
| `onDecrement` | stepper or keyboard decrement action occurs | `number \| null` | optional higher-level passthrough |

## 6. Accessibility

### Semantics

- Role: native numeric input or equivalent spinbutton semantics
- Required attributes: accessible name from label or `ariaLabel`
- Optional attributes: value, min, max, invalid state, readonly state
- Labeling rules: the control must expose numeric value and bounds semantics to
  assistive technology

### Keyboard

| Key | Behavior |
|-----|----------|
| numeric text input | edits the textual numeric representation |
| `Arrow Up` | increments by `step` when stepping is enabled |
| `Arrow Down` | decrements by `step` when stepping is enabled |
| `Home` | moves to min when that interaction is explicitly supported |
| `End` | moves to max when that interaction is explicitly supported |
| `Enter` | optional commit |
| `Tab` | exits control |

### Focus And Announcement

- focus entry: visible active treatment and editable value context
- focus exit: invalid or out-of-range state should remain programmatically
  exposed
- live-region behavior: none by default; validation errors are parent-owned
- GPUI-native accessibility mapping notes: GPUI must expose spinbutton-like
  value/min/max semantics when the number control presents stepping behavior,
  and must suppress conflicting global shortcuts while focused

## 7. Layout

### Sizing

- control height follows shared control-size tokens
- optional stepper controls remain aligned and reachable without collapsing the
  editable numeric area

### Composition

- parent expectations: forms, inspectors, parameter sheets
- child expectations: optional stepper controls only
- resizing rules: text entry remains primary; stepper controls are auxiliary

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | text-input field tokens | base numeric field chrome |
| Input Control | `semantic.typography.body.*` and text roles | numeric text |
| Stepper Controls | button/icon token roles | increment/decrement affordances |
| Focus treatment | `semantic.color.accent.focusRing` and `semantic.border.width.focus` | focus |
| Validation | `semantic.color.status.*` | invalid/pending/valid emphasis |

## 9. Svelte Notes

- may use native numeric input semantics or text-input composition with numeric
  parsing rules, but public behavior must stay consistent
- browser-specific spinner visuals should not define the contract

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::number_entry`
- GPUI implementation must intentionally expose numeric value, bounds, step
  semantics, and focused-text shortcut suppression
- partial numeric editing states should preserve user input without emitting
  misleading committed values

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value/min/max/step semantics match
- [ ] accessible numeric role/value exposure matches
- [ ] partial-entry and invalid-entry behavior matches
- [ ] keyboard increment/decrement behavior matches when enabled

### Tier 2: Visual Parity

- [ ] field sizing and stepper placement remain proportionally aligned
- [ ] validation emphasis uses the same semantic roles

### Tier 3: Implementation Freedom

- [ ] native numeric-input internals vs GPUI numeric parsing internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| browser-native spinner visuals may differ or be suppressed | platform visuals are not the contract | allowed | keep numeric semantics and accessibility strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: inspectors, parameter sheets, settings forms
- future follow-up: coordinate with slider/range primitives in `g01.009`

## Next Task

Keep numeric text entry distinct from slider or knob controls when the value
primitive family lands in `g01.009`.
