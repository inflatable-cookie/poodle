# Range Calendar

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `RangeCalendar`
- Layer: `foundation`
- Summary: a visible month grid for selecting a bounded date range, with
  two-click start/end selection, in-range highlighting, month navigation, and
  roving tabindex keyboard control
- In scope: range start and end selection, in-range display, month navigation,
  week start policy, controlled and uncontrolled value and visible month,
  start/end swap normalization
- Out of scope: time ranges, recurring windows, booking-specific availability
  logic, multi-month views (parent-owned)

## 2. Anatomy

```text
[Root .range-calendar]  <div>
  ├── [Header .range-calendar__header]  <div>
  │     ├── [Previous Button .range-calendar__nav]  <button>
  │     ├── [Month Label .range-calendar__month]  <span>
  │     └── [Next Button .range-calendar__nav]  <button>
  ├── [Weekday Row .range-calendar__weekdays]  <div>
  │     └── [Weekday Label .range-calendar__weekday]... <span>
  └── [Grid .range-calendar__grid]  <div role="grid">
        └── [Week .range-calendar__week]... <div role="row">
              └── [Cell .range-calendar__cell]... <div role="gridcell">
                    └── [Day Button .range-calendar__day]  <button>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | outer container with grid layout | gap, min-width |
| Header | yes | month navigation row | grid columns, gap |
| Previous Button | yes | navigates to previous month | border, radius, background, color, focus ring |
| Month Label | yes | displays current visible month and year | label typography, text-align |
| Next Button | yes | navigates to next month | border, radius, background, color, focus ring |
| Weekday Row | yes | row of day-of-week labels | grid columns |
| Weekday Label | yes | individual day-of-week abbreviation | label typography, text color, text-transform |
| Grid | yes | date grid container | grid layout, gap |
| Week | yes | one row of dates | grid columns |
| Cell | yes | wrapper around each day button | grid layout |
| Day Button | yes | interactive date button with range state data attributes | border, radius, background, color, typography, focus ring |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `DateRangeValue \| null` | `null` | no | controlled selected range |
| `defaultValue` | `DateRangeValue` | `{ start: null, end: null }` | no | uncontrolled initial range |
| `visibleMonth` | `string \| null` | `null` | no | controlled visible month (ISO `YYYY-MM`) |
| `weekStartsOn` | `CalendarWeekStart` | `"monday"` | no | first day of the week |
| `locale` | `string` | `"en-US"` | no | locale for month and weekday formatting |
| `isDisabled` | `boolean` | `false` | no | disables all interaction |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the grid |

### Type Definitions

```
DateRangeValue: { start: string | null; end: string | null }
CalendarWeekStart: "sunday" | "monday"
```

### Controlled And Uncontrolled

- controlled value: `value` (non-null) plus `valueChange` event
- uncontrolled value: `defaultValue` sets the initial range; component owns its
  own state
- controlled month: `visibleMonth` (non-null) plus `monthChange` event
- uncontrolled month: component manages visible month internally

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting day button | transparent background, primary text, transparent border |
| hover | pointer enters day button | tinted background and border via color-mix |
| focus | day button receives focus | tinted background and border, outline removed |
| today | date matches current date | border color via color-mix of accent and border-default |
| range-start | date matches range start | accent background, inverse text |
| range-end | date matches range end | accent background, inverse text |
| in-range | date falls between start and end | tinted accent background |
| outside month | date belongs to adjacent month | secondary text color, reduced opacity |
| disabled | `isDisabled=true` | cursor not-allowed, reduced opacity on root |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| no range | initial render, no value | grid shows with no selection |
| start selected | first click on a day | start is set, end is cleared; waiting for second click |
| complete range | second click on a day | end is set (swapped with start if before start); `valueChange` fires |
| month navigated | user clicks prev/next or PageUp/PageDown | `monthChange` fires, grid rebuilds |
| focus roving | arrow key navigation | focus moves between days via roving tabindex |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | user completes a range selection | `{ value: DateRangeValue }` | fires with `{ start, end }` ISO date strings |
| `monthChange` | visible month changes via nav or keyboard | `{ month: string }` | ISO month string `YYYY-MM` |

## 6. Accessibility

### Semantics

- Role: `role="grid"` on the date grid container
- Row role: `role="row"` on each week
- Cell role: `role="gridcell"` on each cell wrapper
- Day buttons: native `<button>` elements within gridcells
- `aria-selected`: set on range-start and range-end day buttons
- `aria-label`: each day button has a formatted date label (e.g. "March 15, 2026")
- `aria-live="polite"` on the month label to announce month changes
- Grid `aria-label`: from `ariaLabel` prop
- `aria-disabled`: set on root when `isDisabled`
- Range state communicated via data attributes (`data-in-range`, `data-range-start`, `data-range-end`)

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Right` | moves focus to next day (+1) |
| `Arrow Left` | moves focus to previous day (-1) |
| `Arrow Down` | moves focus to same weekday next week (+7) |
| `Arrow Up` | moves focus to same weekday previous week (-7) |
| `Home` | moves focus to start of current week |
| `End` | moves focus to end of current week |
| `Page Down` | moves focus to same day next month |
| `Page Up` | moves focus to same day previous month |
| `Enter` / `Space` | selects the focused day (sets start or end) |
| `Tab` | exits the calendar grid |

### Focus And Announcement

- focus entry: focus lands on range-start if set, or today, or first day of
  month; roving tabindex ensures only one day is tabbable
- focus exit: focus moves to next focusable element outside the calendar
- live-region behavior: month label uses `aria-live="polite"` to announce month
  changes during keyboard navigation
- GPUI-native accessibility mapping notes: GPUI must expose grid semantics with
  row and cell structure, selected state on range endpoints, and button
  interaction semantics for each day

## 7. Layout

### Sizing

- minimum width: `16rem` on root
- calendar is self-sizing vertically based on number of weeks in the month
- overflow behavior: none; calendar always shows a complete month grid

### Composition

- parent expectations: inline placement, overlay shells (DateRangePicker),
  filter panels, report date selectors
- child expectations: none; range calendar is a leaf component
- resizing rules: calendar stretches horizontally with parent; day cells
  distribute equally across 7 columns

## 8. Token Usage — Exact Values

### Root `.range-calendar`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.75rem` |
| `min-width` | `16rem` |

### Header `.range-calendar__header`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `auto minmax(0, 1fr) auto` |
| `align-items` | `center` |
| `gap` | `0.5rem` |

### Month Label `.range-calendar__month`

| Property | Value |
|----------|-------|
| `font-family` | `var(--flint-typography-label-family)` |
| `font-size` | `0.8125rem` |
| `font-weight` | `600` |
| `letter-spacing` | `0.02em` |
| `text-align` | `center` |

### Nav Button `.range-calendar__nav`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `2rem` |
| `height` | `2rem` |
| `border` | `0.0625rem solid var(--flint-color-border-default)` |
| `border-radius` | `var(--flint-radius-control)` |
| `background` | `var(--flint-color-background-surface)` |
| `color` | `var(--flint-color-text-primary)` |
| `cursor` | `pointer` |

### Nav Button — hover

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--flint-color-background-surface) 82%, var(--flint-color-background-elevated))` |

### Nav Button — focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Weekday Row `.range-calendar__weekdays`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `repeat(7, minmax(0, 1fr))` |
| `align-items` | `center` |

### Weekday Label `.range-calendar__weekday`

| Property | Value |
|----------|-------|
| `color` | `var(--flint-color-text-secondary)` |
| `font-family` | `var(--flint-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `600` |
| `letter-spacing` | `0.04em` |
| `text-align` | `center` |
| `text-transform` | `uppercase` |

### Grid `.range-calendar__grid`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.125rem` |

### Week `.range-calendar__week`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `repeat(7, minmax(0, 1fr))` |
| `gap` | `0.125rem` |

### Cell `.range-calendar__cell`

| Property | Value |
|----------|-------|
| `display` | `grid` |

### Day Button `.range-calendar__day`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-height` | `2.25rem` |
| `padding` | `0.25rem` |
| `border` | `0.0625rem solid transparent` |
| `border-radius` | `var(--flint-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--flint-color-text-primary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--flint-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `500` |

### Day Button — outside month

| Property | Value |
|----------|-------|
| `color` | `var(--flint-color-text-secondary)` |
| `opacity` | `0.72` |

### Day Button — today

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--flint-color-accent-base) 44%, var(--flint-color-border-default))` |

### Day Button — in-range `[data-in-range="true"]`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--flint-color-accent-base) 16%, transparent)` |

### Day Button — range-start `[data-range-start]`

| Property | Value |
|----------|-------|
| `background` | `var(--flint-color-accent-base)` |
| `color` | `var(--flint-color-text-inverse)` |

### Day Button — range-end `[data-range-end]`

| Property | Value |
|----------|-------|
| `background` | `var(--flint-color-accent-base)` |
| `color` | `var(--flint-color-text-inverse)` |

### Day Button — hover / focus-visible

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--flint-color-accent-base) 46%, var(--flint-color-border-default))` |
| `background` | `color-mix(in srgb, var(--flint-color-accent-base) 14%, transparent)` |
| `outline` | `none` |

### Day Button — range-start/range-end hover

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--flint-color-accent-base) 88%, white 8%)` |

### Root — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--flint-state-opacity-disabled)` |

## 9. Svelte Notes

- Public value uses `{ start, end }` ISO-date objects rather than `Date`
  instances
- Module-level `nextRangeCalendarId` counter generates unique IDs for each
  instance
- Selection logic: first click sets start and clears end; second click sets end
  (or swaps with start if the clicked date is before the current start)
- Implementation may normalize start and end ordering internally
- Controlled/uncontrolled pattern applies independently to both `value` and
  `visibleMonth`
- `focusIso` tracking maintains the currently focused date for roving tabindex
- Range state is communicated via data attributes: `data-in-range="true"`,
  `data-range-start`, `data-range-end`
- All base grid CSS is identical to Calendar but with `range-calendar__` class
  prefix

## 10. GPUI Notes

- expected crate/module surface: `flint_gpui::primitives::range_calendar`
- GPUI must model the grid as a 7-column structure with row and cell semantics
- Each day must be an interactive element exposing selected state and date label
- Range-start and range-end days must both expose `aria-selected` state
- In-range visual treatment must be present but does not require individual
  accessibility exposure
- Month label must announce changes through the accessibility tree
- `color-mix` formulas must be replicated using platform color blending or
  pre-computed equivalents
- Selection logic (two-click start/end with swap) must match across platforms

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and valueChange semantics match (controlled and uncontrolled)
- [ ] visibleMonth and monthChange semantics match
- [ ] two-click selection logic matches (first sets start, second sets end, swap if before)
- [ ] start/end and in-range semantics match
- [ ] keyboard movement and month navigation match (arrows, Home/End, PageUp/PageDown)
- [ ] Enter/Space selects focused day
- [ ] day-grid accessibility semantics match (grid, row, gridcell roles)
- [ ] roving tabindex or equivalent focus management matches
- [ ] disabled state matches

### Tier 2: Visual Parity

- [ ] root gap (0.75rem) and min-width (16rem) match
- [ ] header grid layout (auto 1fr auto, 0.5rem gap) matches
- [ ] month label typography (label-family, 0.8125rem, 600, 0.02em) matches
- [ ] nav button sizing (2rem x 2rem) and chrome match
- [ ] nav button hover background (color-mix 82%) matches
- [ ] weekday label typography (label-family, 0.6875rem, 600, 0.04em, uppercase) matches
- [ ] day button sizing (min-height 2.25rem) and typography (0.75rem, 500) match
- [ ] today border-color (color-mix accent 44%) matches
- [ ] range-start/range-end treatment (accent-base background, text-inverse) matches
- [ ] in-range treatment (color-mix accent 16%) matches
- [ ] hover/focus tint (color-mix accent 14%) matches
- [ ] range endpoint hover tint (color-mix accent 88%) matches
- [ ] outside-month opacity (0.72) matches
- [ ] disabled opacity (state-opacity-disabled) matches
- [ ] focus ring on nav buttons (border-width-focus, focusRing, 0.125rem offset) matches

### Tier 3: Implementation Freedom

- [ ] grid generation algorithm is platform-owned
- [ ] date formatting internals may differ by platform
- [ ] module-level ID counter is implementation-specific
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| locale formatting details may differ slightly | platform date-format internals differ | allowed | keep value and navigation semantics strict |
| color-mix formula implementation | GPUI may use pre-computed color blending | allowed | same visual result required |
| exact range-preview visuals may differ | render polish is implementation-specific | allowed | keep start, end, and in-range semantics strict |
| roving tabindex vs GPUI focus model | focus management implementation differs by platform | allowed | one-day-focusable-at-a-time behavior must match |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Default | `ariaLabel="Select a date range"` | Full month grid with header navigation, weekday labels, and day buttons; no pre-selected range; today receives today border treatment; interactive |

### With pre-selected range

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With pre-selected range | `defaultValue={ start: "2026-03-05", end: "2026-03-12" }`, `ariaLabel="Pre-selected range"` | Month grid with March 5 and March 12 showing range endpoint treatment (accent background, inverse text); days between showing in-range tint |

### Disabled

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Disabled | `isDisabled=true`, `ariaLabel="Disabled range calendar"` | Month grid with no selection, reduced opacity on root, cursor not-allowed, all interaction disabled |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: DateRangePicker (composition), report filters, booking
  windows, review spans
- future follow-up: consider min/max date constraints if needed; align with
  Calendar for shared grid styling; consider hover-preview of prospective range
  end
