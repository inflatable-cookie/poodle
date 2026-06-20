# Calendar

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Calendar`
- Layer: `foundation`
- Summary: a visible month grid for selecting one date value or a bounded
  date range, with month navigation, roving tabindex keyboard control, and
  week start policy
- In scope: month navigation, day-grid semantics, single-date selection,
  date-range selection (two-click start/end), week start policy, controlled
  and uncontrolled value and visible month, roving tabindex focus management,
  direct month and year jumps from the header
- Out of scope: time selection, recurrence, timezone handling, scheduling
  workflows

## 2. Anatomy

```text
[Root .calendar]  <div>
  ├── [Header .calendar__header]  <div>
  │     ├── [Previous Button .calendar__nav]  <button>
  │     ├── [Month Label .calendar__month]  <span>
  │     │     ├── [Month Trigger .calendar__month-button]  <button>
  │     │     ├── [Month Select .calendar__month-select]  <select>
  │     │     ├── [Year Trigger .calendar__year-button]  <button>
  │     │     └── [Year Input .calendar__year-input]  <input>
  │     └── [Next Button .calendar__nav]  <button>
  ├── [Weekday Row .calendar__weekdays]  <div>
  │     └── [Weekday Label .calendar__weekday]... <span>
  └── [Grid .calendar__grid]  <div role="grid">
        └── [Week .calendar__week]... <div role="row">
              └── [Cell .calendar__cell]... <div role="gridcell">
                    └── [Day Button .calendar__day]  <button>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | outer container with grid layout | gap, cell-size |
| Header | yes | month navigation row | grid columns, gap |
| Previous Button | yes | navigates to previous month | border, radius, background, color, focus ring |
| Month Label | yes | displays current visible month and year | label typography, text-align |
| Month Trigger | conditional | inline affordance that opens month selection on double-click | underline, hover color |
| Month Select | conditional | inline month dropdown while editing | border, radius, background, focus ring |
| Year Trigger | conditional | inline affordance that opens year editing on double-click | underline, hover color |
| Year Input | conditional | inline numeric year editor while editing | border, radius, background, focus ring |
| Next Button | yes | navigates to next month | border, radius, background, color, focus ring |
| Weekday Row | yes | row of day-of-week labels | grid columns |
| Weekday Label | yes | individual day-of-week abbreviation | label typography, text color, text-transform |
| Grid | yes | date grid container | grid layout, gap |
| Week | yes | one row of dates | grid columns |
| Cell | yes | wrapper around each day button | grid layout |
| Day Button | yes | interactive date button | border, radius, background, color, typography, focus ring |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `mode` | `"single" \| "range"` | `"single"` | no | selection mode — single date or date range |
| `value` | `string \| DateRangeValue \| null` | `null` | no | selected value; string in single mode, DateRangeValue in range mode; when supplied, the host owns updates through `onValueChange` |
| `defaultValue` | `string \| DateRangeValue \| null` | `null` | no | uncontrolled initial value |
| `visibleMonth` | `string \| null` | `null` | no | visible month (ISO `YYYY-MM`); when supplied, the host owns updates through `onMonthChange` |
| `weekStartsOn` | `CalendarWeekStart` | `"monday"` | no | first day of the week |
| `locale` | `string` | `"en-US"` | no | locale for month and weekday formatting |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `disabled` | `boolean` | `false` | no | disables all interaction |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the grid |

### Type Definitions

```
CalendarWeekStart: "sunday" | "monday"
DateRangeValue: { start: string | null; end: string | null }
```

### Controlled And Uncontrolled

- controlled value: supplying `value` makes it host-owned, including `null` for an explicit empty selection
- uncontrolled value: `defaultValue` sets the initial selection; component owns
  its own state
- controlled month: supplying `visibleMonth` makes month navigation host-owned through `onMonthChange`
- uncontrolled month: component manages visible month internally

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting day button | transparent background, primary text, transparent border |
| hover | pointer enters day button | tinted background and border via color-mix |
| focus | day button receives focus | tinted background and border, outline removed |
| today | date matches current date | border color via color-mix of accent and border-default |
| selected | date matches selected value (single mode) | accent background, inverse text |
| selected hover | pointer enters selected day | slightly lightened accent background |
| range-start | date matches range start (range mode) | accent background, inverse text |
| range-end | date matches range end (range mode) | accent background, inverse text |
| in-range | date falls between start and end (range mode) | tinted accent background |
| outside month | date belongs to adjacent month | secondary text color, reduced opacity |
| disabled | `disabled=true` | cursor not-allowed, reduced opacity on root |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| idle | initial render | grid shows current or visible month |
| value selected | user clicks/presses day (single mode) | `onValueChange` runs, day receives selected treatment |
| start selected | first click on a day (range mode) | start is set, end is cleared; waiting for second click |
| complete range | second click on a day (range mode) | end is set (swapped with start if before start); `onValueChange` runs |
| month navigated | user clicks prev/next or PageUp/PageDown | `onMonthChange` runs, grid rebuilds |
| month editing | user double-clicks the month label | inline month dropdown opens; choosing a month updates `visibleMonth` |
| year editing | user double-clicks the year label | inline numeric year editor opens; Enter or blur commits, Escape cancels |
| focus roving | arrow key navigation | focus moves between days via roving tabindex |

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | user selects a day | `string` (single) or `DateRangeValue` (range) | ISO date string in single mode; `{ start, end }` in range mode |
| `onMonthChange` | visible month changes via nav or keyboard | `string` | ISO month string `YYYY-MM` |

## 6. Accessibility

### Semantics

- Role: `role="grid"` on the date grid container
- Row role: `role="row"` on each week
- Cell role: `role="gridcell"` on each cell wrapper
- Day buttons: native `<button>` elements within gridcells
- `aria-selected`: set on the gridcell wrapper of the selected day (the day
  button carries `data-selected`)
- `aria-label`: each day button has a formatted date label (e.g. "March 15, 2026")
- `aria-live="polite"` on the month label to announce month changes
- Grid `aria-label`: from `ariaLabel` prop
- `aria-disabled`: set on root when `disabled`

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
| `Enter` / `Space` | selects the focused day |
| `Tab` | exits the calendar grid |

Header editing:

| Action | Behavior |
|--------|----------|
| `Double-click month` | opens inline month dropdown |
| `Double-click year` | opens inline numeric year editor |
| `Enter` in year editor | commits year change |
| `Escape` in month or year editor | cancels inline editing |
| `Arrow Up/Down` in year editor | increments or decrements the year through the native number input behavior |

### Focus And Announcement

- focus entry: focus lands on the selected day, or today if no selection, or
  the first day of the month; roving tabindex ensures only one day is tabbable
- focus exit: focus moves to next focusable element outside the calendar
- live-region behavior: month label uses `aria-live="polite"` to announce month
  changes during keyboard navigation
- GPUI-native accessibility mapping notes: GPUI must expose grid semantics with
  row and cell structure, selected state on the active day, and button
  interaction semantics for each day

## 7. Layout

### Sizing

- width: `fit-content` on root; the grid sizes from 7 columns of
  `--calendar-cell-size` (md `2.25rem`), so total width tracks cell size × 7
- calendar is self-sizing vertically based on number of weeks in the month
- overflow behavior: none; calendar always shows a complete month grid

### Composition

- parent expectations: inline placement, overlay shells (DatePicker), filter
  panels, settings forms
- child expectations: none; calendar is a leaf component
- resizing rules: calendar stretches horizontally with parent; day cells
  distribute equally across 7 columns

## 8. Token Usage — Exact Values

### Root `.calendar`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.75rem` |
| `width` | `fit-content` (grid sizes from `--calendar-cell-size`, see size table) |

### Header `.calendar__header`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `auto minmax(0, 1fr) auto` |
| `align-items` | `center` |
| `gap` | `0.5rem` |

### Month Label `.calendar__month`

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.8125rem` |
| `font-weight` | `600` |
| `letter-spacing` | `0.02em` |
| `text-align` | `center` |

### Nav Button `.calendar__nav`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `2rem` |
| `height` | `2rem` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |

### Nav Button — hover

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 82%, var(--poodle-color-background-elevated))` |

### Nav Button — focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Weekday Row `.calendar__weekdays`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `repeat(7, var(--calendar-cell-size, 2.25rem))` |
| `align-items` | `center` |
| `gap` | `0.125rem` |

### Weekday Label `.calendar__weekday`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `600` |
| `letter-spacing` | `0.04em` |
| `text-align` | `center` |
| `text-transform` | `uppercase` |

### Grid `.calendar__grid`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.125rem` |

### Week `.calendar__week`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `repeat(7, var(--calendar-cell-size, 2.25rem))` |
| `gap` | `0.125rem` |

### Cell `.calendar__cell`

| Property | Value |
|----------|-------|
| `display` | `grid` |

### Day Button `.calendar__day`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-height` | `2.25rem` |
| `padding` | `0.25rem` |
| `border` | `0.0625rem solid transparent` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `500` |

### Day Button — outside month

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `opacity` | `0.72` |

### Day Button — today

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--poodle-color-accent-base) 44%, var(--poodle-color-border-default))` |

### Day Button — selected

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-color-accent-base)` |
| `color` | `var(--poodle-color-text-inverse)` |

### Day Button — hover / focus-visible

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--poodle-color-accent-base) 46%, var(--poodle-color-border-default))` |
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 14%, transparent)` |
| `outline` | `none` |

### Day Button — selected hover

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 88%, white 8%)` |

### Day Button — in-range `[data-in-range="true"]` (range mode)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent)` |

### Day Button — range-start / range-end (range mode)

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-color-accent-base)` |
| `color` | `var(--poodle-color-text-inverse)` |

### Day Button — range endpoint hover (range mode)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 88%, white 8%)` |

### Nav Button — disabled (`.calendar__nav:disabled`)

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Day Button — disabled (`.calendar__day:disabled`)

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Size adjustments

Cell size (`--calendar-cell-size`, drives grid column width) is `2.25rem` at
md and scales per size: xs `1.75rem`, sm `2rem`, lg `2.5rem`, xl `2.75rem`.

| Size | Part | Property | Value |
|------|------|----------|-------|
| `xs` (`[data-size="xs"]`) | Root | `--calendar-cell-size` | `1.75rem` |
| `xs` (`[data-size="xs"]`) | Nav Button | `width` / `height` | `1.5rem` |
| `xs` | Day Button | `min-height` | `1.75rem` |
| `xs` | Day Button | `font-size` | `0.6875rem` |
| `xs` | Month Label | `font-size` | `0.6875rem` |
| `sm` (`[data-size="sm"]`) | Root | `--calendar-cell-size` | `2rem` |
| `sm` (`[data-size="sm"]`) | Nav Button | `width` / `height` | `1.75rem` |
| `sm` | Day Button | `min-height` | `2rem` |
| `sm` | Day Button | `font-size` | `0.6875rem` |
| `sm` | Month Label | `font-size` | `0.75rem` |
| `lg` (`[data-size="lg"]`) | Root | `--calendar-cell-size` | `2.5rem` |
| `lg` (`[data-size="lg"]`) | Nav Button | `width` / `height` | `2.25rem` |
| `lg` | Day Button | `min-height` | `2.5rem` |
| `lg` | Day Button | `font-size` | `0.8125rem` |
| `lg` | Month Label | `font-size` | `0.875rem` |
| `xl` (`[data-size="xl"]`) | Root | `--calendar-cell-size` | `2.75rem` |
| `xl` (`[data-size="xl"]`) | Nav Button | `width` / `height` | `2.5rem` |
| `xl` | Day Button | `min-height` | `2.75rem` |
| `xl` | Day Button | `font-size` | `0.875rem` |
| `xl` | Month Label | `font-size` | `0.9375rem` |

## 9. Svelte Notes

- Public value uses ISO `YYYY-MM-DD` strings rather than browser `Date`
  instances
- `mode` prop controls whether the calendar selects a single date or a range;
  when `mode="range"`, value is a `DateRangeValue` object with `start` and
  `end` fields, and selection uses two-click start/end logic with automatic
  swap normalization
- Range state is communicated via data attributes: `data-in-range="true"`,
  `data-range-start`, `data-range-end`
- Month-grid generation is implementation-owned; keyboard and selected-date
  semantics remain stable
- Module-level `nextCalendarId` counter generates unique IDs for each instance
- Controlled/uncontrolled pattern applies independently to both `value` and
  `visibleMonth`
- `focusIso` tracking maintains the currently focused date for roving tabindex
- Day buttons outside the visible month are rendered but visually muted
- `color-mix` formulas are used for intermediate visual states (today border,
  hover tint, selected hover tint)
- `data-size` data attribute on root reflects the resolved size
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- `data-mode` — current selection mode (`single` or `range`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::calendar`
- GPUI must model the grid as a 7-column structure with row and cell semantics
- Each day must be an interactive element exposing selected state and date label
- Month label must announce changes through the accessibility tree
- Nav buttons must expose button semantics with appropriate labels
- `color-mix` formulas must be replicated using platform color blending or
  pre-computed equivalents
- Roving tabindex equivalent: only one day should be focusable at a time within
  the grid

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and onValueChange semantics match (controlled and uncontrolled)
- [ ] visibleMonth and onMonthChange semantics match
- [ ] selected-day semantics match (aria-selected, visual treatment)
- [ ] month navigation and keyboard movement match (arrows, Home/End, PageUp/PageDown)
- [ ] day-grid accessibility semantics match (grid, row, gridcell roles)
- [ ] roving tabindex or equivalent focus management matches
- [ ] Enter/Space selects focused day
- [ ] today visual treatment matches
- [ ] outside-month visual treatment matches
- [ ] disabled state matches

### Tier 2: Visual Parity

- [ ] root gap (0.75rem) and fit-content width (cell-size 2.25rem at md) match
- [ ] header grid layout (auto 1fr auto, 0.5rem gap) matches
- [ ] month label typography (label-family, 0.8125rem, 600, 0.02em) matches
- [ ] nav button sizing (2rem x 2rem) and chrome match
- [ ] nav button hover background (color-mix 82%) matches
- [ ] weekday label typography (label-family, 0.6875rem, 600, 0.04em, uppercase) matches
- [ ] day button sizing (min-height 2.25rem) and typography (0.75rem, 500) match
- [ ] today border-color (color-mix accent 44%) matches
- [ ] selected treatment (accent-base background, text-inverse) matches
- [ ] hover/focus tint (color-mix accent 14%) matches
- [ ] selected hover tint (color-mix accent 88%) matches
- [ ] outside-month opacity (0.72) matches
- [ ] disabled opacity (state-opacity-disabled) matches
- [ ] focus ring on nav buttons (border-width-focus, focusRing, 0.125rem offset) matches
- [ ] all five sizes visually match (nav button, day cell, month label per size table)

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
| roving tabindex vs GPUI focus model | focus management implementation differs by platform | allowed | one-day-focusable-at-a-time behavior must match |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Default | `ariaLabel="Select a date"` | Full month grid with header navigation, weekday labels, and day buttons; no pre-selected date; today receives today border treatment; interactive |

### With pre-selected date

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With pre-selected date | `defaultValue="2026-03-14"`, `ariaLabel="Calendar with default"` | Month grid with March 14 showing selected treatment (accent background, inverse text) |

### Disabled

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Disabled | `defaultValue="2026-03-01"`, `disabled=true`, `ariaLabel="Disabled calendar"` | Month grid with March 1 selected, reduced opacity on root, cursor not-allowed, all interaction disabled |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: DatePicker (composition), DateRangePicker (composition
  via mode="range"), inline schedulers, filter calendars, settings forms,
  report filters, booking windows
- future follow-up: consider min/max date constraints if needed; consider
  hover-preview of prospective range end in range mode
