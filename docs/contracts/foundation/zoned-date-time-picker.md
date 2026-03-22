# Zoned Date Time Picker

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `ZonedDateTimePicker`
- Layer: `foundation`
- Summary: a value control that combines a picker trigger with a calendar,
  time-field, and timezone-select composition in a single overlay surface
- In scope: selected date, local time, timezone, open state, calendar plus
  time-field and timezone-select composition, placeholder behavior, outside-click
  and Escape dismissal, controlled and uncontrolled value and open state
- Out of scope: recurrence, timezone conversion workflows, transport schedules,
  booking rules, offset arithmetic

## 2. Anatomy

```text
[Root .zoned-date-time-picker]  <div>
  ├── [Trigger .zoned-date-time-picker__trigger]  <button>
  │     ├── [Value .zoned-date-time-picker__value]  <span>
  │     └── [Indicator .zoned-date-time-picker__indicator]  <span>
  └── [Surface .zoned-date-time-picker__surface]  <div role="dialog"> (conditional, when open)
        └── [Body .zoned-date-time-picker__body]
              ├── [Calendar] (composed)
              └── [Fields .zoned-date-time-picker__fields]
                    ├── [Field .zoned-date-time-picker__field]
                    │     ├── [Field Label .zoned-date-time-picker__label]  <label> ("Time")
                    │     └── [TimeField] (composed)
                    └── [Field .zoned-date-time-picker__field]
                          ├── [Field Label .zoned-date-time-picker__label]  <label> ("Time zone")
                          └── [TimeZoneSelect] (composed)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | positioning container for trigger and overlay | position, display, min-width |
| Trigger | yes | button that toggles the overlay | border, radius, background, typography, focus ring, padding |
| Value | yes | displays selected zoned date-time or placeholder text | color, text-align, truncation |
| Indicator | yes | decorative disclosure chevron | color, font-size |
| Surface | yes | overlay containing calendar, time field, and timezone select | position, border, radius, background, shadow, padding |
| Body | yes | vertical stack for calendar and fields | display, gap |
| Calendar | yes | composed calendar primitive | delegated to Calendar contract |
| Fields | yes | vertical stack for time and timezone fields | display, gap |
| Field | yes | container for label and composed control | display, gap |
| Field Label | yes | "Time" / "Time zone" heading above field | color, font-family, font-size, font-weight, letter-spacing, text-transform |
| TimeField | yes | composed time-field primitive | delegated to TimeField contract |
| TimeZoneSelect | yes | composed timezone-select primitive | delegated to TimeZoneSelect contract |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `ZonedDateTimeValue \| null` | `null` | no | controlled selected zoned date-time |
| `defaultValue` | `ZonedDateTimeValue` | `{ date: null, time: null, timeZone: null }` | no | uncontrolled initial value |
| `open` | `boolean \| null` | `null` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial open state |
| `placeholder` | `string` | `"Select date, time, and zone"` | no | shown when no value selected |
| `weekStartsOn` | `"sunday" \| "monday"` | `"monday"` | no | first day of the week |
| `locale` | `string` | `"en-US"` | no | locale for date formatting |
| `timeZoneOptions` | `TimeZoneOption[]` | `[]` | no | curated list of timezone options; empty uses internal defaults |
| `isDisabled` | `boolean` | `false` | no | disables the trigger |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |

### Type Definitions

```
ZonedDateTimeValue: {
  date: string | null;
  time: string | null;
  timeZone: string | null;
}

TimeZoneOption: {
  value: string;
  label: string;
}
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
| partial value | some fields filled but not all | partial value displayed, overlay remains open |
| complete value | date, time, and timezone all committed | formatted zoned date-time displayed |
| open | trigger clicked or keyboard activated | surface appears below trigger |
| disabled | `isDisabled=true` | reduced opacity, non-interactive, cursor: not-allowed |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| value committed | user changes any date, time, or timezone field | `valueChange` fires with current value |
| dismissed | Escape or click outside | overlay closes without changing value |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | user changes date, time, or timezone | `{ value: ZonedDateTimeValue }` | fires on each constituent change |
| `openChange` | overlay opens or closes | `{ open: boolean }` | fires on open and close transitions |

## 6. Accessibility

### Semantics

- Trigger: `<button>` with `aria-haspopup="dialog"`, `aria-expanded` (true/false), `aria-controls` pointing to surface id
- Surface: `role="dialog"`, unique id referenced by `aria-controls`
- Trigger accessible name from external label or `ariaLabel` prop
- Disabled: `disabled` attribute on trigger button
- Module-level `nextZonedDateTimePickerId` counter generates unique ids for ARIA relationships
- TimeField receives `ariaLabel` "Time"; TimeZoneSelect receives `ariaLabel` "Time zone"

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` | toggles overlay open/closed |
| `Escape` | closes overlay without changing value |
| `Tab` | when open, moves focus between calendar, time field, and timezone select; when closed, exits control |

### Focus And Announcement

- focus entry: trigger receives focus ring via outline
- focus transition: opening the overlay moves focus into the calendar
- focus restoration: closing the overlay returns focus to the trigger
- live-region behavior: none; calendar, time field, and timezone select handle their own announcements
- GPUI-native accessibility mapping notes: GPUI must expose button with haspopup, expanded state, and dialog relationship through native accessibility APIs

## 7. Layout

### Sizing

- Root min-width: `18rem`
- Trigger height follows `size-control-height` token
- Surface is absolutely positioned below trigger with a gap
- Body uses vertical grid layout with gap between calendar and fields
- Fields stack vertically with gap between time and timezone rows

### Composition

- parent expectations: publishing flows, appointments, scheduler setup,
  localized reminders
- child expectations: composes Calendar, TimeField, and TimeZoneSelect
  internally; no child slots
- resizing rules: trigger stretches to parent width; value text truncates with
  ellipsis

## 8. Token Usage — Exact Values

### Root `.zoned-date-time-picker`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-grid` |
| `min-width` | `18rem` |

### Trigger `.zoned-date-time-picker__trigger`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `space-between` |
| `gap` | `0.75rem` |
| `min-height` | `var(--pug-size-control-height)` |
| `padding` | `0 var(--pug-space-control-x)` |
| `border` | `0.0625rem solid var(--pug-color-border-default)` |
| `border-radius` | `var(--pug-radius-control)` |
| `background` | `var(--pug-color-background-surface)` |
| `color` | `var(--pug-color-text-primary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--pug-typography-body-family)` |
| `font-size` | `var(--pug-typography-body-size)` |
| `line-height` | `var(--pug-typography-body-lineHeight)` |
| `text-align` | `left` |

### Trigger — hover

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--pug-color-background-surface) 86%, var(--pug-color-background-elevated))` |

### Trigger — focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Trigger — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--pug-state-opacity-disabled)` |

### Value — placeholder state `.zoned-date-time-picker__value--placeholder`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |

### Indicator `.zoned-date-time-picker__indicator`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |
| `font-size` | `0.75rem` |

### Surface `.zoned-date-time-picker__surface`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `calc(100% + 0.375rem)` |
| `left` | `0` |
| `z-index` | `var(--pug-overlay-z-menu)` |
| `padding` | `var(--pug-space-panel-y) var(--pug-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 72%, transparent)` |
| `border-radius` | `var(--pug-radius-surface)` |
| `background` | `color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))` |
| `box-shadow` | `var(--pug-elevation-overlay)` |

### Body `.zoned-date-time-picker__body`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.875rem` |

### Fields `.zoned-date-time-picker__fields`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.75rem` |

### Field `.zoned-date-time-picker__field`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.375rem` |

### Field Label `.zoned-date-time-picker__label`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |
| `font-family` | `var(--pug-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `600` |
| `letter-spacing` | `0.04em` |
| `text-transform` | `uppercase` |

## 9. Svelte Notes

- Module-level `nextZonedDateTimePickerId` counter generates unique ids for each
  instance to wire ARIA relationships (`aria-controls`, `aria-expanded`)
- Controlled/uncontrolled pattern: if `value` prop is non-null, component
  operates in controlled mode; otherwise `defaultValue` seeds internal state
- Same pattern for `open`/`defaultOpen`
- Outside click handler closes the overlay; Escape key closes the overlay
- Composes `Calendar`, `TimeField`, and `TimeZoneSelect` internally
- Public value uses contract-owned local date, local time, and timezone string
  fields rather than timestamps
- Hosts may provide curated `timeZoneOptions` when product policy requires them;
  if empty, implementation may source a default timezone list internally
- Partial values are allowed during editing

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::zoned_date_time_picker`
- GPUI must implement trigger button with dialog overlay pattern
- Must expose haspopup, expanded state, and dialog relationship through native
  accessibility APIs
- `color-mix` formulas for surface border (72%), background (98%), and trigger
  hover (86%) must be replicated or approximated
- Calendar, TimeField, and TimeZoneSelect composition: GPUI delegates to its own
  calendar, time-field, and timezone-select primitives
- Field label typography must match: label-family, 0.6875rem, weight 600,
  0.04em tracking, uppercase
- Timezone option ordering may differ due to platform timezone registries;
  keep committed timezone value semantics strict

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and valueChange semantics match (fires on constituent change)
- [ ] openChange fires on open and close transitions
- [ ] Escape closes overlay without changing value
- [ ] outside click closes overlay
- [ ] disabled state prevents interaction
- [ ] partial values allowed during editing
- [ ] timeZoneOptions host-provided option sets remain equivalent
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
- [ ] fields gap (0.75rem) matches
- [ ] field gap (0.375rem) matches
- [ ] field label typography matches (label-family, 0.6875rem, 600, 0.04em, uppercase)
- [ ] disabled opacity uses state-opacity-disabled token

### Tier 3: Implementation Freedom

- [ ] overlay positioning/clipping strategy is platform-owned
- [ ] id generation strategy is implementation-owned
- [ ] date and time formatting details may vary by platform locale support
- [ ] exact native time-entry affordances may differ
- [ ] timezone option ordering may vary by platform registries

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| overlay positioning details may differ | GPUI overlay system differs from CSS absolute positioning | allowed | must appear anchored below trigger visually |
| color-mix approximation in GPUI | GPUI may not have CSS color-mix; equivalent blending acceptable | allowed | visual result must match |
| exact native time-entry affordances may differ | platform time controls differ | allowed | keep public value meaning strict |
| exact timezone option ordering may differ | runtime registries and host option policies differ | allowed | keep committed timezone value semantics strict |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Default | `ariaLabel="Select date, time, and zone"` | Trigger button showing placeholder text "Select date, time, and zone" with disclosure indicator; interactive, opens calendar, time field, and timezone select overlay on click |

### With default value

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With default value | `defaultValue={ date: "2026-03-14", time: "10:00", timeZone: "America/Los_Angeles" }`, `ariaLabel="Pre-filled zoned date time"` | Trigger button showing formatted date, time, and timezone instead of placeholder |

### Disabled

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Disabled | `isDisabled=true`, `ariaLabel="Disabled picker"` | Trigger button with default placeholder, reduced opacity, cursor not-allowed, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: publishing flows, appointments, scheduler setup,
  localized reminders, event creation forms
- future follow-up: consider timezone conversion display as a composite wrapper;
  align overlay placement with Popover rules if needed
