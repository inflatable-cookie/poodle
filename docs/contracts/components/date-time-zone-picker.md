# Date Time Zone Picker

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `DateTimeZonePicker`
- Layer: `foundation`
- Summary: a value control that combines a picker trigger with a calendar,
  time-input, and timezone-select composition in a single overlay surface
- In scope: selected date, local time, timezone, open state, calendar plus
  time-input and timezone-select composition, placeholder behavior, outside-click
  and Escape dismissal, controlled and uncontrolled value and open state
- Out of scope: recurrence, timezone conversion workflows, transport schedules,
  booking rules, offset arithmetic

## 2. Anatomy

```text
[Root .date-time-zone-picker]  <div>
  ├── [Trigger .date-time-zone-picker__trigger]  <button>
  │     ├── [Value .date-time-zone-picker__value]  <span>
  │     └── [Indicator .date-time-zone-picker__indicator]  <span>
  └── [Surface .date-time-zone-picker__surface]  <div role="dialog"> (conditional, when open)
        └── [Body .date-time-zone-picker__body]
              ├── [Calendar] (composed)
              └── [Fields .date-time-zone-picker__fields]
                    ├── [Field .date-time-zone-picker__field]
                    │     ├── [Field Label .date-time-zone-picker__label]  <label> ("Time")
                    │     └── [TimeInput] (composed)
                    └── [Field .date-time-zone-picker__field]
                          ├── [Field Label .date-time-zone-picker__label]  <label> ("Time zone")
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
| TimeInput | yes | composed time-input primitive | delegated to TimeInput contract |
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
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `disabled` | `boolean` | `false` | no | disables the trigger |
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

- controlled value: `value` plus `onValueChange` callback
- uncontrolled value: `defaultValue`; omitting `value` leaves state internal
- controlled open: `open` plus `onOpenChange` callback
- uncontrolled open: `defaultOpen`; omitting `open` leaves state internal

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| placeholder | no value selected | placeholder text in secondary color |
| partial value | some fields filled but not all | partial value displayed, overlay remains open |
| complete value | date, time, and timezone all committed | formatted zoned date-time displayed |
| open | trigger clicked or keyboard activated | surface appears below trigger |
| disabled | `disabled=true` | reduced opacity, non-interactive, cursor: not-allowed |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| value committed | user changes any date, time, or timezone field | `onValueChange` runs with current value |
| dismissed | Escape or click outside | overlay closes without changing value |

### Behavior Machine

Behavior classification: machine-backed via core machinery

All date and time math lives in `@inflatable-cookie/poodle-headless` (`date.ts`), promoted
wholesale from the Svelte date module: ISO parse/format, day/month
arithmetic (`addMonths` anchors to the 1st — month paging semantics),
comparison, range normalization (endpoints ordered), calendar-grid
construction (`buildCalendarWeeks`, full 7-day rows, `startOfWeek`,
Home/End boundary deltas), date-time and zoned value normalization, and
Intl-based labels/time-zone validation. The value types
(`DateRangeValue`, `DateTimeValue`, `DateTimeRangeValue`,
`ZonedDateTimeValue`, `CalendarWeekStart`, `TimeZoneOption`) are defined in
core so the Rust mirror shares the same shapes. Component state (month
cursor, draft editing, focus movement) stays adapter-side and calls into
this machinery.

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | user changes date, time, or timezone | `ZonedDateTimeValue` | runs on each constituent change |
| `onOpenChange` | overlay opens or closes | `boolean` | runs on open and close transitions |

## 6. Accessibility

### Semantics

- Trigger: `<button>` with `aria-haspopup="dialog"`, `aria-expanded` (true/false), `aria-controls` pointing to surface id
- Surface: `role="dialog"`, unique id referenced by `aria-controls`
- Trigger accessible name from external label or `ariaLabel` prop
- Disabled: `disabled` attribute on trigger button
- Module-level `nextDateTimeZonePickerId` counter generates unique ids for ARIA relationships
- TimeInput receives `ariaLabel` "Time"; TimeZoneSelect receives `ariaLabel` "Time zone"

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
- child expectations: composes Calendar, TimeInput, and TimeZoneSelect
  internally; no child slots
- resizing rules: trigger stretches to parent width; value text truncates with
  ellipsis

## 8. Token Usage — Exact Values

### Root `.date-time-zone-picker`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-grid` |
| `min-width` | `18rem` |

### Trigger `.date-time-zone-picker__trigger`

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

### Value — placeholder state `.date-time-zone-picker__value--placeholder`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |

### Indicator `.date-time-zone-picker__indicator`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.75rem` |

### Surface `.date-time-zone-picker__surface`

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

### Body `.date-time-zone-picker__body`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.875rem` |

### Fields `.date-time-zone-picker__fields`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.75rem` |

### Field `.date-time-zone-picker__field`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.375rem` |

### Field Label `.date-time-zone-picker__label`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `600` |
| `letter-spacing` | `0.04em` |
| `text-transform` | `uppercase` |

### Size adjustments

Size sets an **absolute** trigger height (via `--poodle-date-time-zone-picker-trigger-height`), the trigger `font-size`, and the indicator `font-size`. Size does **not** set per-size padding — horizontal padding is density-driven (see below).

| Size | Trigger height | Trigger `font-size` | Indicator `font-size` |
|------|----------------|---------------------|-----------------------|
| `xs` (`[data-size="xs"]`) | `1.5rem` | `0.75rem` | `0.625rem` |
| `sm` (`[data-size="sm"]`) | `1.75rem` | `0.8125rem` | `0.6875rem` |
| `md` (`[data-size="md"]`) | `var(--poodle-size-control-height-md, var(--poodle-size-control-height))` | `var(--poodle-typography-body-size)` (base) | `0.75rem` (base) |
| `lg` (`[data-size="lg"]`) | `2.75rem` | `0.9375rem` | `0.8125rem` |
| `xl` (`[data-size="xl"]`) | `3.25rem` | `1rem` | `0.875rem` |

### Density adjustments (trigger horizontal padding)

| Density | `padding` |
|---------|-----------|
| `compact` (`[data-density="compact"]`) | `0 calc(var(--poodle-space-control-x) - 0.125rem)` |
| `default` | `0 var(--poodle-space-control-x)` (base) |
| `comfortable` (`[data-density="comfortable"]`) | `0 calc(var(--poodle-space-control-x) + 0.125rem)` |

## 9. Svelte Notes

- Module-level `nextDateTimeZonePickerId` counter generates unique ids for each
  instance to wire ARIA relationships (`aria-controls`, `aria-expanded`)
- Controlled/uncontrolled pattern: supplying `value` makes it host-owned,
  including `null` for a controlled empty state; otherwise `defaultValue`
  seeds internal state
- Same pattern for `open`/`defaultOpen`: supplying `open` makes visibility
  host-owned
- Outside click handler closes the overlay; Escape key closes the overlay
- Composes `Calendar`, `TimeInput`, and `TimeZoneSelect` internally
- Public value uses contract-owned local date, local time, and timezone string
  fields rather than timestamps
- Hosts may provide curated `timeZoneOptions` when product policy requires them;
  if empty, implementation may source a default timezone list internally
- Partial values are allowed during editing
- `data-size` data attribute on root reflects the resolved size; it sets an absolute trigger height (`xs:1.5rem` … `xl:3.25rem`, `md` via `--poodle-size-control-height-md`) plus per-size trigger and indicator font-sizes — it does **not** set per-size padding
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`); density alone drives trigger horizontal padding (`compact` −0.125rem, `comfortable` +0.125rem from `--poodle-space-control-x`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::date_time_zone_picker`
- GPUI must implement trigger button with dialog overlay pattern
- Must expose haspopup, expanded state, and dialog relationship through native
  accessibility APIs
- `color-mix` formulas for surface border (72%), background (98%), and trigger
  hover (86%) must be replicated or approximated
- Calendar, TimeInput, and TimeZoneSelect composition: GPUI delegates to its own
  calendar, time-input, and timezone-select primitives
- Field label typography must match: label-family, 0.6875rem, weight 600,
  0.04em tracking, uppercase
- Timezone option ordering may differ due to platform timezone registries;
  keep committed timezone value semantics strict

## 10a. Jetstream Notes

- `DateTimeZonePicker::from_spec(spec, theme).on_toggle(...).on_select(...)
  .on_navigate(...).on_zone_toggle(...).on_zone_change(...)`.
- The zone list is the composed `TimeZoneSelect`, forwarded whole: the trigger
  press comes back as `on_zone_toggle` and the pressed option's id as
  `on_zone_change`. `DateTimeZonePickerSpec.zone_open` renders the zone
  option list open — native-only state that the web's Select keeps internal;
  the host flips it from `on_zone_toggle` and merges the picked zone into the
  value it holds.
- Known Delta: the time half is typed, which stays host-side — the runtime
  raises no key events.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and onValueChange semantics match (runs on constituent change)
- [ ] onOpenChange runs on open and close transitions
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
- [ ] indicator color (text-secondary) and font-size match (base `0.75rem`, scaling `xs:0.625rem` … `xl:0.875rem`)
- [ ] surface overlay: absolute positioning, 0.375rem gap below trigger
- [ ] surface border color-mix (72% border-default) matches
- [ ] surface background color-mix (98% elevated, panel) matches
- [ ] surface elevation shadow matches
- [ ] body gap (0.875rem) matches
- [ ] fields gap (0.75rem) matches
- [ ] field gap (0.375rem) matches
- [ ] field label typography matches (label-family, 0.6875rem, 600, 0.04em, uppercase)
- [ ] disabled opacity uses state-opacity-disabled token
- [ ] all five sizes visually match (height, padding, font-size per size table)

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
| Disabled | `disabled=true`, `ariaLabel="Disabled picker"` | Trigger button with default placeholder, reduced opacity, cursor not-allowed, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: publishing flows, appointments, scheduler setup,
  localized reminders, event creation forms
- future follow-up: consider timezone conversion display as a composite wrapper;
  align overlay placement with Popover rules if needed
