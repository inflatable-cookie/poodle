# Date Time Picker

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `DateTimePicker`
- Layer: `foundation`
- Summary: a value control that combines a picker trigger with a calendar and
  time-field composition in a single overlay surface
- In scope: selected date, selected time, open state, calendar plus time-field
  composition, placeholder behavior, outside-click and Escape dismissal,
  controlled and uncontrolled value and open state
- Out of scope: timezone selection, recurring schedules, booking availability,
  range workflows, preset time shortcuts

## 2. Anatomy

```text
[Root .date-time-picker]  <div>
  ├── [Trigger .date-time-picker__trigger]  <button>
  │     ├── [Value .date-time-picker__value]  <span>
  │     └── [Indicator .date-time-picker__indicator]  <span>
  └── [Surface .date-time-picker__surface]  <div role="dialog"> (conditional, when open)
        └── [Body .date-time-picker__body]
              ├── [Calendar] (composed)
              └── [Time Section .date-time-picker__time-section]
                    ├── [Time Label .date-time-picker__time-label]  <span>
                    └── [TimeField] (composed)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | positioning container for trigger and overlay | position, display, min-width |
| Trigger | yes | button that toggles the overlay | border, radius, background, typography, focus ring, padding |
| Value | yes | displays selected date-time or placeholder text | color, text-align, truncation |
| Indicator | yes | decorative disclosure chevron | color, font-size |
| Surface | yes | overlay containing calendar and time field | position, border, radius, background, shadow, padding |
| Body | yes | vertical stack for calendar and time section | display, gap |
| Calendar | yes | composed calendar primitive | delegated to Calendar contract |
| Time Section | yes | container for time label and time field | display, gap |
| Time Label | yes | "Time" heading above time field | color, font-family, font-size, font-weight, letter-spacing, text-transform |
| TimeField | yes | composed time-field primitive | delegated to TimeField contract |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `DateTimeValue \| null` | `null` | no | controlled selected date-time |
| `defaultValue` | `DateTimeValue` | `{ date: null, time: null }` | no | uncontrolled initial value |
| `open` | `boolean \| null` | `null` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial open state |
| `placeholder` | `string` | `"Select date and time"` | no | shown when no value selected |
| `weekStartsOn` | `"sunday" \| "monday"` | `"monday"` | no | first day of the week |
| `locale` | `string` | `"en-US"` | no | locale for date formatting |
| `disabled` | `boolean` | `false` | no | disables the trigger |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |

### Type Definitions

```
DateTimeValue: { date: string | null; time: string | null }
```

### Controlled And Uncontrolled

- controlled value: `value` plus `valueChange` event
- uncontrolled value: `defaultValue`
- controlled open: `open` plus `openChange` event
- uncontrolled open: `defaultOpen`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| placeholder | no value selected | placeholder text in secondary color |
| partial value | date or time chosen but not both | partial value displayed, overlay remains open |
| complete value | both date and time committed | formatted date-time displayed |
| open | trigger clicked or keyboard activated | surface appears below trigger |
| disabled | `disabled=true` | reduced opacity, non-interactive, cursor: not-allowed |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| value committed | user selects date and enters time | `valueChange` fires with current value |
| dismissed | Escape or click outside | overlay closes without changing value |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | user changes date or time selection | `{ value: DateTimeValue }` | fires on each date or time change |
| `openChange` | overlay opens or closes | `{ open: boolean }` | fires on open and close transitions |

## 6. Accessibility

### Semantics

- Trigger: `<button>` with `aria-haspopup="dialog"`, `aria-expanded` (true/false), `aria-controls` pointing to surface id
- Surface: `role="dialog"`, unique id referenced by `aria-controls`
- Trigger accessible name from external label or `ariaLabel` prop
- Disabled: `disabled` attribute on trigger button
- Module-level `nextDateTimePickerId` counter generates unique ids for ARIA relationships

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` | toggles overlay open/closed |
| `Escape` | closes overlay without changing value |
| `Tab` | when open, moves focus between calendar and time field; when closed, exits control |

### Focus And Announcement

- focus entry: trigger receives focus ring via outline
- focus transition: opening the overlay moves focus into the calendar
- focus restoration: closing the overlay returns focus to the trigger
- live-region behavior: none; calendar and time field handle their own announcements
- GPUI-native accessibility mapping notes: GPUI must expose button with haspopup, expanded state, and dialog relationship through native accessibility APIs

## 7. Layout

### Sizing

- Root min-width: `16rem`
- Trigger height follows `size-control-height` token
- Surface is absolutely positioned below trigger with a gap
- Body uses vertical grid layout with gap between calendar and time section

### Composition

- parent expectations: forms, publishing controls, appointment pickers, scheduler setup
- child expectations: composes Calendar and TimeField internally; no child slots
- resizing rules: trigger stretches to parent width; value text truncates with ellipsis

## 8. Token Usage — Exact Values

### Root `.date-time-picker`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-grid` |
| `min-width` | `16rem` |

### Trigger `.date-time-picker__trigger`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `space-between` |
| `gap` | `0.75rem` |
| `min-height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |
| `text-align` | `left` |

### Trigger — hover

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 86%, var(--poodle-color-background-elevated))` |

### Trigger — focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Trigger — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Value — placeholder state `.date-time-picker__value--placeholder`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |

### Indicator `.date-time-picker__indicator`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.75rem` |
| `line-height` | `1` |

### Surface `.date-time-picker__surface`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `calc(100% + 0.375rem)` |
| `left` | `0` |
| `z-index` | `var(--poodle-overlay-z-menu)` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))` |
| `box-shadow` | `var(--poodle-elevation-overlay)` |

### Body `.date-time-picker__body`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.875rem` |

### Time Section `.date-time-picker__time-section`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.375rem` |

### Time Label `.date-time-picker__time-label`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `600` |
| `letter-spacing` | `0.04em` |
| `text-transform` | `uppercase` |

## 9. Svelte Notes

- Module-level `nextDateTimePickerId` counter generates unique ids for each
  instance to wire ARIA relationships (`aria-controls`, `aria-expanded`)
- Controlled/uncontrolled pattern: if `value` prop is non-null, component
  operates in controlled mode; otherwise `defaultValue` seeds internal state
- Same pattern for `open`/`defaultOpen`
- Outside click handler closes the overlay; Escape key closes the overlay
- Composes `Calendar` and `TimeField` internally
- Partial values are allowed during editing; the public value uses
  `{ date, time }` rather than `Date` instances
- Value display formats date and time using `locale` prop for localized strings

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::date_time_picker`
- GPUI must implement trigger button with dialog overlay pattern
- Must expose haspopup, expanded state, and dialog relationship through native
  accessibility APIs
- `color-mix` formulas for surface border (72%), background (98%), and trigger
  hover (86%) must be replicated or approximated
- Calendar and TimeField composition: GPUI delegates to its own calendar and
  time-field primitives
- Time label typography must match: label-family, 0.6875rem, weight 600,
  0.04em tracking, uppercase

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and valueChange semantics match (fires on date or time change)
- [ ] openChange fires on open and close transitions
- [ ] Escape closes overlay without changing value
- [ ] outside click closes overlay
- [ ] disabled state prevents interaction
- [ ] partial values allowed (date without time, time without date)
- [ ] ARIA: haspopup="dialog", expanded, controls, dialog role on surface

### Tier 2: Visual Parity

- [ ] trigger uses control-height, control-x padding, body typography
- [ ] trigger focus ring matches (outline with focusRing color, 0.125rem offset)
- [ ] trigger hover background color-mix (86% surface, elevated) matches
- [ ] placeholder color (text-secondary) matches
- [ ] indicator color (text-secondary) and font-size (0.75rem) match
- [ ] surface overlay: absolute positioning, 0.375rem gap below trigger
- [ ] surface border color-mix (72% border-default) matches
- [ ] surface background color-mix (98% elevated, panel) matches
- [ ] surface elevation shadow matches
- [ ] body gap (0.875rem) matches
- [ ] time section gap (0.375rem) matches
- [ ] time label typography matches (label-family, 0.6875rem, 600, 0.04em, uppercase)
- [ ] disabled opacity uses state-opacity-disabled token

### Tier 3: Implementation Freedom

- [ ] overlay positioning/clipping strategy is platform-owned
- [ ] id generation strategy is implementation-owned
- [ ] date and time formatting details may vary by platform locale support
- [ ] exact native time-entry affordances may differ

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| overlay positioning details may differ | GPUI overlay system differs from CSS absolute positioning | allowed | must appear anchored below trigger visually |
| color-mix approximation in GPUI | GPUI may not have CSS color-mix; equivalent blending acceptable | allowed | visual result must match |
| exact native time-entry affordances may differ | platform time controls differ | allowed | keep public value and popup semantics strict |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Default | `ariaLabel="Select date and time"` | Trigger button showing placeholder text "Select date and time" with disclosure indicator; interactive, opens calendar and time field overlay on click |

### With default value

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With default value | `defaultValue={ date: "2026-03-14", time: "14:30" }`, `ariaLabel="Pre-filled date time"` | Trigger button showing formatted date and time instead of placeholder |

### Disabled

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Disabled | `disabled=true`, `ariaLabel="Disabled date time picker"` | Trigger button with default placeholder, reduced opacity, cursor not-allowed, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: publishing forms, reminders, appointments, scheduler
  setup flows
- future follow-up: consider timezone-aware variant (see ZonedDateTimePicker);
  align overlay placement with Popover rules if needed
