# Number Entry

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `NumberEntry`
- Layer: `foundation`
- Summary: a numeric text-entry control with optional increment/decrement
  stepper affordances and constrained numeric semantics
- In scope: numeric value entry, validation bounds (min/max), step behavior,
  precision formatting, optional stepper buttons, blur-commit clamping
- Out of scope: slider interaction, knob/fader semantics, scientific notation
  editors, currency formatting

## 2. Anatomy

```text
[Root .number-entry]  <div>
  ├── [Input Control .number-entry__control]  <input type="text" inputmode="decimal">
  └── [Steppers .number-entry__steppers] (conditional, when showSteppers)
        ├── [Decrement Button]  <button>
        └── [Increment Button]  <button>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | numeric field chrome container | background, border, radius, focus ring |
| Input Control | yes | editable numeric text surface | typography, text color, padding |
| Steppers | no | increment/decrement button pair | background, border-radius, color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | — | yes | HTML id for the input element |
| `value` | `number \| null` | `null` | no | controlled numeric value |
| `defaultValue` | `number \| null` | `null` | no | uncontrolled initial value |
| `placeholder` | `string \| null` | `null` | no | hint text when empty |
| `min` | `number \| null` | `null` | no | lower bound; null means unbounded |
| `max` | `number \| null` | `null` | no | upper bound; null means unbounded |
| `step` | `number` | `1` | no | increment/decrement size |
| `precision` | `number \| null` | `null` | no | decimal places for formatting; null means auto |
| `name` | `string \| undefined` | `undefined` | no | form field name |
| `isDisabled` | `boolean` | `false` | no | disables editing and steppers |
| `isReadOnly` | `boolean` | `false` | no | allows selection without editing |
| `validationState` | `"none" \| "invalid" \| "valid" \| "pending"` | `"none"` | no | visual and assistive validation state |
| `showSteppers` | `boolean` | `false` | no | shows increment/decrement stepper buttons |
| `ariaLabel` | `string \| null` | `null` | no | required when no external label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |

### Controlled And Uncontrolled

- controlled: `value` plus `valueChange` event
- uncontrolled: `defaultValue`
- do not mix controlled and uncontrolled modes simultaneously

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | neutral numeric field chrome |
| focus | keyboard or pointer focus | focus ring via `box-shadow` on root |
| invalid | `validationState="invalid"` | border-color changes to `status-danger` |
| valid | `validationState="valid"` | border-color changes to `status-success` |
| pending | `validationState="pending"` | border-color changes to `accent-base` |
| disabled | `isDisabled=true` | non-interactive, stepper buttons show `cursor: not-allowed`, `opacity: state-opacity-disabled` |
| readOnly | `isReadOnly=true` | selectable but not editable |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| valid numeric text | user enters parseable value | numeric value emitted via valueChange |
| transient text | user enters partial numeric text | edit preserved until blur resolves it |
| bounds-adjusted | blur or stepper exceeds min/max | value clamped to bounds |
| step-snapped | blur with step constraint | value snapped to nearest step increment |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | numeric value changes (commit) | `{ value: number \| null }` | fires on blur-commit, stepper action, or keyboard step |
| `submit` | Enter key pressed | none | signals explicit commit |
| `increment` | value stepped up | none | via stepper button or ArrowUp |
| `decrement` | value stepped down | none | via stepper button or ArrowDown |
| `focus` | control receives focus | native event | passthrough |
| `blur` | control loses focus | native event | triggers clamp-and-snap logic |

## 6. Accessibility

### Semantics

- Role: uses `<input type="text" inputmode="decimal">` (not `type="number"` to avoid browser-native spinner conflicts)
- Required attributes: accessible name from external label or `ariaLabel`
- Optional attributes: `aria-describedby` from `describedBy`, `aria-invalid` when validationState is `"invalid"`, `aria-readonly` when `isReadOnly`
- Labeling rules: placeholder text never counts as the accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| numeric/decimal text input | edits the textual numeric representation |
| `Arrow Up` | increments value by `step` |
| `Arrow Down` | decrements value by `step` |
| `Enter` | fires submit event, commits current value |
| `Tab` | exits control; blur triggers clamp-and-snap |

### Focus And Announcement

- focus entry: root receives visible focus ring, input is editable
- focus exit: blur triggers value clamping to min/max and snapping to step; validation state remains programmatically exposed
- live-region behavior: none by default; validation errors are parent-owned
- GPUI-native accessibility mapping notes: GPUI must expose spinbutton-like value/min/max semantics when stepping is enabled, and must suppress conflicting global shortcuts while focused

## 7. Layout

### Sizing

- Root min-height follows `size-control-height` token
- Steppers display as a vertically stacked pair (increment on top, decrement below) within a grid column
- Input control fills remaining horizontal space via `minmax(0, 1fr)`

### Composition

- parent expectations: forms, inspectors, parameter sheets, settings rows
- child expectations: optional stepper controls only
- resizing rules: text entry remains primary; stepper controls are auxiliary and do not collapse the editable area

## 8. Token Usage — Exact Values

### Root `.number-entry`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `minmax(0, 1fr) auto` |
| `align-items` | `stretch` |
| `height` | `var(--pug-size-control-height)` |
| `overflow` | `hidden` |
| `border` | `0.0625rem solid var(--pug-color-border-default)` |
| `border-radius` | `var(--pug-radius-control)` |
| `background` | `var(--pug-color-background-surface)` |

### Root — validation states

| State | Property | Value |
|-------|----------|-------|
| `invalid` | `border-color` | `var(--pug-color-status-danger)` |
| `valid` | `border-color` | `var(--pug-color-status-success)` |
| `pending` | `border-color` | `var(--pug-color-accent-base)` |

### Root — focus-within

| Property | Value |
|----------|-------|
| `box-shadow` | `0 0 0 var(--pug-border-width-focus) color-mix(in srgb, var(--pug-color-accent-focusRing) 28%, transparent)` |

### Input Control `.number-entry__control`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `padding` | `0 var(--pug-space-control-x)` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--pug-color-text-primary)` |
| `font-family` | `var(--pug-typography-body-family)` |
| `font-size` | `var(--pug-typography-body-size)` |
| `line-height` | `var(--pug-typography-body-lineHeight)` |
| `outline` | `0` |

### Steppers Container `.number-entry__steppers`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-rows` | `1fr 1fr` |
| `gap` | `0` |
| `padding` | `0.0625rem` |

### Stepper Button `.number-entry__steppers button`

| Property | Value |
|----------|-------|
| `width` | `1.25rem` |
| `border` | `0` |
| `border-radius` | `calc(var(--pug-radius-control) - 0.125rem)` |
| `background` | `color-mix(in srgb, var(--pug-color-background-elevated) 88%, transparent)` |
| `color` | `var(--pug-color-text-primary)` |
| `cursor` | `pointer` |
| `padding` | `0` |

### Stepper Button — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--pug-state-opacity-disabled)` |

## 9. Svelte Notes

- Uses `<input type="text" inputmode="decimal">` instead of `type="number"` to avoid browser-native spinner conflicts and gain full control over step/clamp behavior
- Browser-native number spinners are not rendered; custom steppers replace them when `showSteppers` is enabled
- Blur handler performs clamp-to-bounds and snap-to-step logic before emitting final value
- Precision prop controls decimal formatting on blur commit

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::number_entry`
- GPUI implementation must intentionally expose numeric value, bounds, step semantics, and focused-text shortcut suppression
- Partial numeric editing states should preserve user input without emitting misleading committed values
- Stepper buttons must remain reachable via pointer; keyboard ArrowUp/ArrowDown handle stepping without requiring steppers to be visible

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value/min/max/step/precision semantics match
- [ ] accessible numeric role/value exposure matches
- [ ] partial-entry and invalid-entry behavior matches
- [ ] keyboard increment/decrement behavior matches
- [ ] blur clamp-and-snap behavior matches
- [ ] submit (Enter) behavior matches

### Tier 2: Visual Parity

- [ ] field sizing uses the same control-height token
- [ ] stepper button sizing and border-radius match
- [ ] stepper background color-mix formula matches
- [ ] validation border-color states match
- [ ] focus ring box-shadow formula matches (28% mix)
- [ ] disabled opacity matches

### Tier 3: Implementation Freedom

- [ ] native text input internals vs GPUI numeric parsing internals stay internal
- [ ] step-snap rounding strategy is implementation-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| browser-native spinner visuals suppressed in Svelte | `type="text"` eliminates native spinners; GPUI has none | allowed | keep numeric semantics strict |
| stepper visual details may differ slightly in GPUI | rendering model differs | allowed | keep stepping behavior and bounds strict |

## 13. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `value={1}`, `min={0}`, `max={100}`, `ariaLabel="Quantity"` | Numeric field showing value 1; increment/decrement via keyboard; displays current quantity below |

### With Steppers

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With steppers | `value={29.99}`, `min={0}`, `step={0.01}`, `precision={2}`, `showSteppers`, `ariaLabel="Price"` | Numeric field with visible increment/decrement stepper buttons; value formatted to 2 decimal places; displays price below |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `value={42}`, `isDisabled` | Numeric field with reduced opacity, non-interactive |

### Invalid

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Invalid | `value={-5}`, `min={0}`, `validationState="invalid"` | Numeric field with danger border color indicating invalid state |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: inspectors, parameter sheets, settings forms, property editors
- future follow-up: coordinate with slider/range primitives; share clamp/snap utility logic
