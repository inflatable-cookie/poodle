# DurationInput

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `DurationInput`
- Layer: `foundation`
- Summary: a segmented numeric input for entering time durations in
  hours, minutes, and optional seconds
- In scope: hours/minutes/seconds segments, carry logic (60s→1m, 60m→1h),
  max/min validation, keyboard increment/decrement, disabled and invalid states
- Out of scope: date/time pickers (see DatePicker, TimePicker), countdown
  timers, stopwatch functionality

## 2. Anatomy

```text
[Root .duration-input]  <div role="group">
  ├── [Hours Segment .duration-input__segment]
  │   ├── [Label .duration-input__label]  <label for="dur-hours"> "h"
  │   └── [Field .duration-input__field]  <input type="text" id="dur-hours">
  ├── [Separator .duration-input__separator]  <span> ":"
  ├── [Minutes Segment .duration-input__segment]
  │   ├── [Label .duration-input__label]  <label for="dur-minutes"> "m"
  │   └── [Field .duration-input__field]  <input type="text" id="dur-minutes">
  ├── [Separator .duration-input__separator]  <span> ":" (conditional)
  └── [Seconds Segment .duration-input__segment]  (conditional, when showSeconds)
      ├── [Label .duration-input__label]  <label for="dur-seconds"> "s"
      └── [Field .duration-input__field]  <input type="text" id="dur-seconds">
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | grouped container with focus-within styling | border, radius, background, height, padding |
| Segment | yes | column container for label + field | flex layout, gap |
| Label | yes | unit indicator above field | font-size, color, letter-spacing |
| Field | yes | numeric text input | width, font, color, text-align |
| Separator | yes | colon between segments | color, font-size, font-weight |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `hours` | `number` | `0` | no | current hours value |
| `minutes` | `number` | `0` | no | current minutes value |
| `seconds` | `number` | `0` | no | current seconds value |
| `showSeconds` | `boolean` | `true` | no | whether to display seconds segment |
| `maxHours` | `number` | `99` | no | maximum hours value |
| `minTotalSeconds` | `number` | `0` | no | minimum total duration in seconds |
| `maxTotalSeconds` | `number \| null` | `null` | no | maximum total duration in seconds |
| `isDisabled` | `boolean` | `false` | no | disables all fields |
| `ariaLabel` | `string` | `"Duration"` | no | accessible name for the group |

### Controlled And Uncontrolled

- Values are controlled via `hours`, `minutes`, `seconds` props
- Changes dispatched via `change` event; parent updates props

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | bordered container with segmented fields |
| focus-within | any field focused | accent border and focus shadow |
| segment-focus | individual field focused | segment background highlight (covers label + field) |
| disabled | `isDisabled=true` | reduced opacity, fields not editable |
| invalid | total out of min/max bounds | danger border color |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `change` | any segment value changes | `{hours: number, minutes: number, seconds: number, totalSeconds: number}` | fires after carry logic applied |

## 6. Accessibility

### Semantics

- Root: `role="group"`, `aria-label` from prop
- Each field: `<input>` with `aria-label` ("Hours", "Minutes", "Seconds")
- Labels: `<label>` elements with `for` attributes linking to hardcoded field IDs (`dur-hours`, `dur-minutes`, `dur-seconds`)
- Disabled: fields receive `disabled` attribute

### Keyboard

| Key | Behavior |
|-----|----------|
| `ArrowUp` | increment focused segment by 1 (with carry) |
| `ArrowDown` | decrement focused segment by 1 (with carry) |
| `Tab` | move focus to next segment or out of component |
| `Shift+Tab` | move focus to previous segment |
| `0-9` | type numeric value into focused segment |

### Focus And Announcement

- focus entry: first field (hours) receives focus
- focus-within: root shows accent border and shadow
- individual field focus: entire segment (label + field) highlights

## 7. Layout

### Sizing

- Root: inline-flex, width: fit-content, padding-based height
- Fields: fixed width (1.75rem), centered text
- Separators: aligned to flex-end with fields

### Composition

- parent expectations: form fields, settings panels, timer configurations
- child expectations: none (self-contained)
- resizing: inline-flex, does not grow

## 8. Token Usage — Exact Values

### Root

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `flex-end` |
| `gap` | `0.125rem` |
| `width` | `fit-content` |
| `padding` | `0.25rem var(--flint-space-control-x)` |
| `border` | `0.0625rem solid var(--flint-color-border-default)` |
| `border-radius` | `var(--flint-radius-control)` |
| `background` | `var(--flint-color-background-surface)` |
| `font-family` | `var(--flint-typography-code-family)` |
| `transition` | `border-color, box-shadow` at `motion-duration-interaction motion-easing-standard` |

### Root focus-within

| Property | Value |
|----------|-------|
| `border-color` | `var(--flint-color-accent-focusRing)` |
| `box-shadow` | `0 0 0 var(--flint-border-width-focus) color-mix(in srgb, var(--flint-color-accent-focusRing) 28%, transparent)` |

### Root disabled

| Property | Value |
|----------|-------|
| `opacity` | `var(--flint-state-opacity-disabled)` |

### Root invalid

| Property | Value |
|----------|-------|
| `border-color` | `var(--flint-color-status-danger)` |

### Segment

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `align-items` | `center` |
| `gap` | `0.125rem` |
| `padding` | `0.125rem` |
| `border-radius` | `0.1875rem` |

### Label

| Property | Value |
|----------|-------|
| `font-size` | `0.5625rem` |
| `color` | `var(--flint-color-text-secondary)` |
| `text-transform` | `uppercase` |
| `letter-spacing` | `0.05em` |
| `line-height` | `1` |
| `user-select` | `none` |

### Field

| Property | Value |
|----------|-------|
| `width` | `1.75rem` |
| `min-height` | `0` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--flint-color-text-primary)` |
| `font-family` | `var(--flint-typography-code-family)` |
| `font-size` | `var(--flint-typography-body-size)` |
| `font-variant-numeric` | `tabular-nums` |
| `text-align` | `center` |
| `line-height` | `1` |
| `outline` | `none` |

### Segment (when child field focused)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--flint-color-accent-base) 12%, transparent)` |

### Separator

| Property | Value |
|----------|-------|
| `color` | `var(--flint-color-text-secondary)` |
| `font-size` | `var(--flint-typography-body-size)` |
| `font-weight` | `600` |
| `line-height` | `1` |
| `user-select` | `none` |

## 9. Svelte Notes

- Each field uses `inputmode="numeric"` and `pattern="[0-9]*"` for mobile keyboards
- Carry logic: when seconds reach 60, increment minutes and reset seconds;
  when minutes reach 60, increment hours and reset minutes
- Values clamped to `maxHours` for hours, 0-59 for minutes/seconds
- Total seconds validated against `minTotalSeconds` and `maxTotalSeconds`
- `data-invalid` attribute set when out of bounds

## 10. GPUI Notes

- expected crate/module surface: `flint_gpui::components::duration_input`
- Spec struct: `DurationInputSpec` in primitives crate
- Component struct: `FlintDurationInput` in components crate
- Segmented input fields may use GPUI's text input with custom formatting
- Carry logic implemented as pure Rust function
- `tabular-nums` font variant may need GPUI font feature support

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] hours, minutes, seconds props accepted and displayed
- [ ] showSeconds toggles seconds segment
- [ ] carry logic matches (60→increment, 0→decrement)
- [ ] ArrowUp/Down keyboard behavior matches
- [ ] min/max validation matches
- [ ] change event payload matches

### Tier 2: Visual Parity

- [ ] root border, radius, background match
- [ ] focus-within accent border and shadow match
- [ ] field focus background highlight matches
- [ ] label typography matches (0.5625rem, uppercase, 0.05em spacing)
- [ ] separator styling matches
- [ ] disabled opacity matches
- [ ] invalid border color matches

### Tier 3: Implementation Freedom

- [ ] input handling method is platform-owned
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Input field implementation | GPUI text input vs HTML input element | allowed | same visual and behavioral result |
| tabular-nums font variant | may require GPUI font feature flag | allowed | match where possible |

## 13. Specimen Definitions

### Hours, Minutes, Seconds

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Hours, minutes, seconds | `hours={1}`, `minutes={30}`, `seconds={0}`, `showSeconds={true}` (default) | Three-segment duration input (HH:MM:SS) with colon separators; displays total duration below |

### Hours and Minutes Only

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Hours and minutes only | `hours={0}`, `minutes={45}`, `showSeconds={false}` | Two-segment duration input (HH:MM) without seconds segment |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `hours={2}`, `minutes={15}`, `seconds={30}`, `isDisabled` | Three-segment duration input with reduced opacity, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: timer settings, scheduling forms, media duration inputs
- future follow-up: milliseconds segment, custom segment labels
