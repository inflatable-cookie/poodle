# Time Field

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `TimeField`
- Layer: `foundation`
- Summary: a time-only value control for local wall-clock entry using a native
  time input with Poodle field chrome
- In scope: time value entry, min/max constraints, step sizing, disabled state,
  controlled and uncontrolled value models
- Out of scope: timezone conversion, date ownership, recurrence, schedule
  workflows, custom time picker overlays

## 2. Anatomy

```text
[Input .time-field]  <input type="time">
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Input | yes | native time input element | background, border, radius, color, typography, focus ring |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string \| null` | `null` | no | HTML id for label association |
| `value` | `string \| null` | `null` | no | controlled value in HH:MM or HH:MM:SS format |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial value |
| `min` | `string \| null` | `null` | no | earliest allowed time |
| `max` | `string \| null` | `null` | no | latest allowed time |
| `step` | `number` | `60` | no | step increment in seconds |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `disabled` | `boolean` | `false` | no | disables editing and interaction |
| `ariaLabel` | `string \| null` | `null` | no | required when no external label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |

### Controlled And Uncontrolled

- controlled: `value` (non-null) plus `valueChange` event
- uncontrolled: `defaultValue` sets the initial value; component owns its own state
- do not mix controlled and uncontrolled modes simultaneously

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | no value set | input shows platform placeholder |
| populated | value is set | time value displayed |
| focus | input receives focus | focus ring via outline |
| disabled | `disabled=true` | reduced opacity, non-interactive |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty -> populated | user enters time or value prop set | valueChange fires |
| populated -> empty | value cleared | valueChange fires with null |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | user changes the time value | `{ value: string \| null }` | fires on native change/input event |

## 6. Accessibility

### Semantics

- Role: native `<input type="time">` provides built-in accessibility
- Required attributes: accessible name from external label or `ariaLabel`
- Optional attributes: `aria-describedby` from `describedBy`
- `disabled` attribute set when `disabled`
- `min`, `max`, `step` attributes set on native input when provided

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Up` / `Arrow Down` | increment/decrement time segment (platform-native) |
| `Tab` | moves between time segments or exits control (platform-native) |
| number keys | direct entry of time digits |

### Focus And Announcement

- focus entry: input receives visible focus ring
- focus exit: focus ring clears
- live-region behavior: none; native time input handles value announcement
- GPUI-native accessibility mapping notes: GPUI must expose time-input semantics with min/max/step constraints through native accessibility tree

## 7. Layout

### Sizing

- minimum height follows `size-control-height` token
- width determined by parent container
- overflow behavior: text truncates within input

### Composition

- parent expectations: forms, settings rows, datetime pickers, Field wrapper
- child expectations: none (self-contained)
- resizing rules: input stretches to parent width

## 8. Token Usage — Exact Values

### Input `.time-field`

| Property | Value |
|----------|-------|
| `min-height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |

### Input — focus

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Input — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Size adjustments

| Size | Property | Value |
|------|----------|-------|
| `xs` (`[data-size="xs"]`) | `min-height` | `calc(var(--poodle-size-control-height) - 0.5rem)` |
| `xs` | `padding` | `0 calc(var(--poodle-space-control-x) - 0.125rem)` |
| `xs` | `font-size` | `0.6875rem` |
| `sm` (`[data-size="sm"]`) | `min-height` | `calc(var(--poodle-size-control-height) - 0.375rem)` |
| `sm` | `padding` | `0 calc(var(--poodle-space-control-x) - 0.125rem)` |
| `sm` | `font-size` | `0.75rem` |
| `lg` (`[data-size="lg"]`) | `min-height` | `calc(var(--poodle-size-control-height) + 0.375rem)` |
| `lg` | `padding` | `0 calc(var(--poodle-space-control-x) + 0.125rem)` |
| `lg` | `font-size` | `0.875rem` |
| `xl` (`[data-size="xl"]`) | `min-height` | `calc(var(--poodle-size-control-height) + 0.5rem)` |
| `xl` | `padding` | `0 calc(var(--poodle-space-control-x) + 0.1875rem)` |
| `xl` | `font-size` | `0.9375rem` |

## 9. Svelte Notes

- Uses native `<input type="time">` for platform accessibility and time-entry UX
- `appearance: none` may be needed for consistent cross-browser styling
- Public value uses local time strings in HH:MM or HH:MM:SS form
- Browser-native time picker UI is allowed; Poodle does not override it
- Treatment tokens may be added for themed styling with fallbacks
- `data-size` data attribute on the input reflects the resolved size
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::time_field`
- Spec struct: `TimeFieldSpec` in primitives crate
- GPUI must provide its own time-entry editing UI since there is no native input[type="time"]
- Must expose time value, min/max constraints, and step through accessibility tree
- Focus ring treatment must match outline spec

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and valueChange semantics match
- [ ] min, max, step constraints match
- [ ] disabled state matches
- [ ] accessible name from label or ariaLabel matches
- [ ] describedBy relationship matches

### Tier 2: Visual Parity

- [ ] control height uses control-height token
- [ ] padding uses space-control-x token
- [ ] border and border-radius match
- [ ] background uses background-surface token
- [ ] typography (body-family, body-size, body-lineHeight) matches
- [ ] focus ring (border-width-focus, accent-focusRing, 0.125rem offset) matches
- [ ] disabled opacity matches
- [ ] all five sizes visually match (height, padding, font-size per size table)

### Tier 3: Implementation Freedom

- [ ] native time-entry UI vs GPUI custom time editing stays internal
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native editing affordances may differ | platform time-entry controls differ | allowed | keep public value meaning strict |
| GPUI provides custom time editing UI | no native input[type="time"] in GPUI | allowed | must preserve value format and constraints |

## 13. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `ariaLabel="Start time"` | Empty time input with platform placeholder; selecting a time displays selected value below |

### With Default Value

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With default value | `defaultValue="14:30"`, `ariaLabel="Meeting time"` | Time input pre-filled with 14:30 |

### With Min/Max Constraints

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With min/max constraints | `defaultValue="09:00"`, `min="08:00"`, `max="18:00"`, `ariaLabel="Office hours"` | Time input constrained to 08:00-18:00 range, showing 09:00 |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `defaultValue="12:00"`, `disabled` | Time input showing 12:00, reduced opacity, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings rows, booking fields, datetime pickers,
  DateTimePicker composite
- future follow-up: consider custom time picker overlay for consistency if
  browser-native pickers prove too inconsistent
