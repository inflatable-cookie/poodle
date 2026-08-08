# Date Time Range Picker

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `DateTimeRangePicker`
- Layer: `foundation`
- Summary: a bounded range value control that combines a picker trigger with a
  calendar (range mode) and paired start/end time-input composition in a single overlay
  surface
- In scope: selected start and end date values, selected start and end local
  time values, open state, calendar (range mode) plus paired time-input composition,
  placeholder behavior, outside-click and Escape dismissal, controlled and
  uncontrolled value and open state
- Out of scope: timezone selection, recurrence, booking availability, transport
  or schedule workflows, preset range shortcuts

## 2. Anatomy

```text
[Root .date-time-range-picker]  <div>
  ├── [Trigger .date-time-range-picker__trigger]  <button>
  │     ├── [Value .date-time-range-picker__value]  <span>
  │     └── [Indicator .date-time-range-picker__indicator]  <span>
  └── [Surface .date-time-range-picker__surface]  <div role="dialog"> (conditional, when open)
        └── [Body .date-time-range-picker__body]
              ├── [Calendar mode="range"] (composed)
              └── [Times Row .date-time-range-picker__times]
                    ├── [Time Section .date-time-range-picker__time-section]
                    │     ├── [Time Label .date-time-range-picker__time-label]  <span> ("Start time")
                    │     └── [TimeInput] (composed, start)
                    └── [Time Section .date-time-range-picker__time-section]
                          ├── [Time Label .date-time-range-picker__time-label]  <span> ("End time")
                          └── [TimeInput] (composed, end)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | positioning container for trigger and overlay | position, display, min-width |
| Trigger | yes | button that toggles the overlay | border, radius, background, typography, focus ring, padding |
| Value | yes | displays selected range or placeholder text | color, text-align, truncation |
| Indicator | yes | decorative disclosure chevron | color, font-size |
| Surface | yes | overlay containing calendar and time fields | position, border, radius, background, shadow, padding |
| Body | yes | vertical stack for calendar and times row | display, gap |
| Calendar (range) | yes | composed Calendar with mode="range" | delegated to Calendar contract (range mode) |
| Times Row | yes | horizontal grid for start and end time sections | display, grid-template-columns, gap |
| Time Section | yes | container for time label and time field | display, gap |
| Time Label | yes | "Start time" / "End time" heading above time field | color, font-family, font-size, font-weight, letter-spacing, text-transform |
| TimeInput | yes | composed time-input primitives (start and end) | delegated to TimeInput contract |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `DateTimeRangeValue \| null` | `null` | no | controlled selected range |
| `defaultValue` | `DateTimeRangeValue` | `{ start: { date: null, time: null }, end: { date: null, time: null } }` | no | uncontrolled initial value |
| `open` | `boolean \| null` | `null` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial open state |
| `placeholder` | `string` | `"Select date and time range"` | no | shown when no value selected |
| `weekStartsOn` | `"sunday" \| "monday"` | `"monday"` | no | first day of the week |
| `locale` | `string` | `"en-US"` | no | locale for date formatting |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `disabled` | `boolean` | `false` | no | disables the trigger |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |

### Type Definitions

```
DateTimeRangeValue: {
  start: { date: string | null; time: string | null };
  end: { date: string | null; time: string | null };
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
| partial range | some fields filled but not all | partial value displayed, overlay remains open |
| complete range | all start/end date and time values committed | formatted range displayed |
| open | trigger clicked or keyboard activated | surface appears below trigger |
| disabled | `disabled=true` | reduced opacity, non-interactive, cursor: not-allowed |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| value committed | user changes any date or time field | `onValueChange` runs with current range |
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
| `onValueChange` | user changes any date or time selection | `DateTimeRangeValue` | runs on each constituent change |
| `onOpenChange` | overlay opens or closes | `boolean` | runs on open and close transitions |

## 6. Accessibility

### Semantics

- Trigger: `<button>` with `aria-haspopup="dialog"`, `aria-expanded` (true/false), `aria-controls` pointing to surface id
- Surface: `role="dialog"`, unique id referenced by `aria-controls`
- Trigger accessible name from external label or `ariaLabel` prop
- Disabled: `disabled` attribute on trigger button
- Module-level `nextDateTimeRangePickerId` counter generates unique ids for ARIA relationships
- Each TimeInput receives a descriptive `ariaLabel` ("Start time", "End time")

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` | toggles overlay open/closed |
| `Escape` | closes overlay without changing value |
| `Tab` | when open, moves focus between calendar, start time field, and end time field; when closed, exits control |

### Focus And Announcement

- focus entry: trigger receives focus ring via outline
- focus transition: opening the overlay moves focus into the calendar
- focus restoration: closing the overlay returns focus to the trigger
- live-region behavior: none; calendar and time fields handle their own announcements
- GPUI-native accessibility mapping notes: GPUI must expose button with haspopup, expanded state, and dialog relationship through native accessibility APIs

## 7. Layout

### Sizing

- Root min-width: `18rem`
- Trigger height follows `size-control-height` token
- Surface is absolutely positioned below trigger with a gap
- Body uses vertical grid layout with gap between calendar and times row
- Times row uses two equal columns for start and end time sections

### Composition

- parent expectations: report filters, booking windows, publishing ranges,
  scheduled review windows
- child expectations: composes Calendar (mode="range") and two TimeInput instances
  internally; no child slots
- resizing rules: trigger stretches to parent width; value text truncates with
  ellipsis

## 8. Token Usage — Exact Values

### Root `.date-time-range-picker`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-grid` |
| `min-width` | `18rem` |

### Trigger `.date-time-range-picker__trigger`

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

### Value — placeholder state `.date-time-range-picker__value--placeholder`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |

### Indicator `.date-time-range-picker__indicator`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.75rem` |
| `line-height` | `1` |

### Surface `.date-time-range-picker__surface`

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

### Body `.date-time-range-picker__body`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.875rem` |

### Times Row `.date-time-range-picker__times`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `repeat(2, minmax(0, 1fr))` |
| `gap` | `0.75rem` |

### Time Section `.date-time-range-picker__time-section`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.375rem` |

### Time Label `.date-time-range-picker__time-label`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `600` |
| `letter-spacing` | `0.04em` |
| `text-transform` | `uppercase` |

### Size adjustments

Size sets trigger `min-height`, trigger `font-size`, and indicator `font-size`.
Svelte ships absolute trigger heights; the `md` base resolves from
`var(--poodle-size-control-height-md, var(--poodle-size-control-height))`. Size
does **not** alter horizontal padding — density owns that (see below).

| Size | trigger `min-height` | trigger `font-size` | indicator `font-size` |
|------|----------------------|---------------------|-----------------------|
| `xs` | `1.5rem` | `0.75rem` | `0.625rem` |
| `sm` | `1.75rem` | `0.8125rem` | `0.6875rem` |
| `md` | `var(--poodle-size-control-height-md, var(--poodle-size-control-height))` | `var(--poodle-typography-body-size)` | `0.75rem` |
| `lg` | `2.75rem` | `0.9375rem` | `0.8125rem` |
| `xl` | `3.25rem` | `1rem` | `0.875rem` |

### Density adjustments

Density owns trigger horizontal padding only (no height/font change).

| Density | trigger `padding` |
|---------|-------------------|
| `compact` | `0 calc(var(--poodle-space-control-x) - 0.125rem)` |
| `default` | `0 var(--poodle-space-control-x)` |
| `comfortable` | `0 calc(var(--poodle-space-control-x) + 0.125rem)` |

## 9. Svelte Notes

- Module-level `nextDateTimeRangePickerId` counter generates unique ids for each
  instance to wire ARIA relationships (`aria-controls`, `aria-expanded`)
- Controlled/uncontrolled pattern: supplying `value` makes it host-owned,
  including `null` for a controlled empty state; otherwise `defaultValue`
  seeds internal state
- Same pattern for `open`/`defaultOpen`: supplying `open` makes visibility
  host-owned
- Outside click handler closes the overlay; Escape key closes the overlay
- Composes `Calendar` with `mode="range"` and two `TimeInput` instances internally
- Public value uses nested local-value objects rather than `Date` instances
- Partial values are allowed during editing without forcing timezone or
  timestamp normalization into the public contract
- Value display formats the range using `locale` prop for localized strings
- `data-size` data attribute on root reflects the resolved size
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::date_time_range_picker`
- GPUI must implement trigger button with dialog overlay pattern
- Must expose haspopup, expanded state, and dialog relationship through native
  accessibility APIs
- `color-mix` formulas for surface border (72%), background (98%), and trigger
  hover (86%) must be replicated or approximated
- Calendar and TimeInput composition: GPUI delegates to its own
  calendar (range mode) and time-input primitives
- Two-column time layout: GPUI must replicate the equal-width column grid for
  start and end time sections
- Time label typography must match: label-family, 0.6875rem, weight 600,
  0.04em tracking, uppercase

## 10a. Jetstream Notes

- `DateTimeRangePicker::from_spec(spec, theme).on_toggle(...).on_select(...).on_navigate(...)`.
- As with `DateTimePicker`, the time halves are typed and carry no handler.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and onValueChange semantics match (runs on constituent change)
- [ ] onOpenChange runs on open and close transitions
- [ ] Escape closes overlay without changing value
- [ ] outside click closes overlay
- [ ] disabled state prevents interaction
- [ ] partial values allowed during editing
- [ ] start and end time fields have descriptive ariaLabels
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
- [ ] times row two-column grid with 0.75rem gap matches
- [ ] time section gap (0.375rem) matches
- [ ] time label typography matches (label-family, 0.6875rem, 600, 0.04em, uppercase)
- [ ] disabled opacity uses state-opacity-disabled token
- [ ] all five sizes visually match (height, padding, font-size per size table)

### Tier 3: Implementation Freedom

- [ ] overlay positioning/clipping strategy is platform-owned
- [ ] id generation strategy is implementation-owned
- [ ] date and time formatting details may vary by platform locale support
- [ ] exact close posture after completion may differ

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| overlay positioning details may differ | GPUI overlay system differs from CSS absolute positioning | allowed | must appear anchored below trigger visually |
| color-mix approximation in GPUI | GPUI may not have CSS color-mix; equivalent blending acceptable | allowed | visual result must match |
| exact close posture after completion may differ | completion timing is implementation-owned | allowed | keep committed value semantics strict |
| exact native time-entry affordances may differ | platform time controls differ | allowed | keep public value meaning strict |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Default | `ariaLabel="Select date and time range"` | Trigger button showing placeholder text "Select date and time range" with disclosure indicator; interactive, opens calendar and paired time fields overlay on click |

### With default range

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With default range | `defaultValue={ start: { date: "2026-03-10", time: "09:00" }, end: { date: "2026-03-14", time: "17:00" } }`, `ariaLabel="Pre-filled range"` | Trigger button showing formatted date-time range instead of placeholder |

### Disabled

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Disabled | `disabled=true`, `ariaLabel="Disabled range picker"` | Trigger button with default placeholder, reduced opacity, cursor not-allowed, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: report filters, booking windows, publishing ranges,
  scheduled review windows
- future follow-up: consider timezone-aware variant (see DateTimeZonePicker
  for single datetime with timezone); align overlay placement with Popover rules
  if needed
