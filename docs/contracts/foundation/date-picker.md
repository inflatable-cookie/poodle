# Date Picker

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `DatePicker`
- Layer: `foundation`
- Summary: a date value control that combines a trigger button with a Calendar
  overlay for single-date selection, supporting controlled and uncontrolled
  value and open state
- In scope: trigger semantics, open/close state, selected date display,
  calendar overlay, placeholder, outside-click and Escape dismissal,
  controlled and uncontrolled value and open state
- Out of scope: time input, free-form locale parsing, recurrence or schedule
  workflows, date range selection (see DateRangePicker)

## 2. Anatomy

```text
[Root .date-picker]  <div>
  ├── [Trigger .date-picker__trigger]  <button>
  │     ├── [Value / Placeholder text]
  │     └── [Indicator .date-picker__indicator]  <span>
  └── [Surface .date-picker__surface]  <div role="dialog"> (conditional, when open)
        └── [Calendar] (composed Calendar component)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | positioning wrapper | position, display, min-width |
| Trigger | yes | button that opens/closes the calendar overlay | background, border, radius, typography, focus ring |
| Value / Placeholder | yes | displays selected date or placeholder text | text color (primary or secondary) |
| Indicator | yes | decorative disclosure icon | text color, font-size |
| Surface | yes (when open) | overlay panel containing the Calendar | position, z-index, padding, border, radius, background, shadow |
| Calendar | yes (when open) | composed Calendar component for date selection | (see Calendar contract) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled selected date (ISO `YYYY-MM-DD`) |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial selected date |
| `open` | `boolean \| null` | `null` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial open state |
| `placeholder` | `string` | `"Select date"` | no | text shown when no date is selected |
| `weekStartsOn` | `CalendarWeekStart` | `"monday"` | no | passed through to composed Calendar |
| `locale` | `string` | `"en-US"` | no | passed through to composed Calendar |
| `isDisabled` | `boolean` | `false` | no | disables the trigger and prevents opening |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the trigger |

### Type Definitions

```
CalendarWeekStart: "sunday" | "monday"
```

### Controlled And Uncontrolled

- controlled value: `value` (non-null) plus `valueChange` event
- uncontrolled value: `defaultValue` sets the initial selection; component owns
  its own state
- controlled open: `open` (non-null) plus `openChange` event
- uncontrolled open: `defaultOpen` sets the initial open state; component owns
  its own state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| placeholder | no value selected | placeholder text in secondary color |
| selected | value is set | formatted date displayed in primary color |
| hover | pointer enters trigger | background shifts via color-mix |
| focus | trigger receives focus | focus ring outline |
| open | trigger clicked or Enter/Space | surface appears below trigger, calendar rendered |
| disabled | `isDisabled=true` | cursor not-allowed, reduced opacity |

### Component States

```
                  ┌──────────┐
                  │  closed  │
                  └────┬─────┘
         click/Enter/  │  \  Escape/outside-click/
           Space       │   \  value-select
                  ┌────▼─────┐
                  │   open   │
                  └──────────┘
```

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | initial, Escape, outside click, date selected | surface hidden, trigger shows value or placeholder |
| open | trigger click, Enter, Space | surface visible with Calendar, `openChange` fires |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | user selects a date in the Calendar | `{ value: string }` | ISO date string; also closes the picker |
| `openChange` | open state changes | `{ open: boolean }` | fires on open and close |

## 6. Accessibility

### Semantics

- Trigger: `aria-haspopup="dialog"` indicating a dialog-like popup
- Trigger: `aria-expanded` reflecting open state
- Trigger: `aria-controls` pointing to the surface element ID
- Surface: `role="dialog"` on the overlay container
- Trigger `aria-label`: from `ariaLabel` prop; required when no external label
- `aria-disabled`: set on trigger when `isDisabled`
- Calendar within surface: inherits full Calendar accessibility contract

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` | toggles open state on trigger |
| `Escape` | closes the surface and returns focus to trigger |
| `Tab` | moves focus out of the trigger (when closed) |
| Calendar keys | when open, all Calendar keyboard rules apply (arrows, Home/End, PageUp/PageDown, Enter/Space) |

### Focus And Announcement

- focus entry: trigger receives visible focus ring treatment
- focus on open: when surface opens, focus moves to the Calendar grid (selected
  day, today, or first day)
- focus on close: focus returns to the trigger button
- live-region behavior: Calendar month label handles month-change announcements
- GPUI-native accessibility mapping notes: GPUI must expose trigger as a button
  with haspopup and expanded state; overlay must present as a dialog containing
  the calendar grid

## 7. Layout

### Sizing

- minimum width: `14rem` on root
- trigger follows shared control sizing (`size-control-height`)
- surface anchors below the trigger with a gap

### Composition

- parent expectations: forms, filter bars, inspector controls, settings rows
- child expectations: composes Calendar internally; no external children
- resizing rules: trigger stretches to parent width; surface width is
  determined by the Calendar min-width

## 8. Token Usage — Exact Values

### Root `.date-picker`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-grid` |
| `min-width` | `14rem` |

### Trigger `.date-picker__trigger`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `space-between` |
| `gap` | `0.75rem` |
| `min-height` | `var(--flint-size-control-height)` |
| `padding` | `0 var(--flint-space-control-x)` |
| `border` | `0.0625rem solid var(--flint-color-border-default)` |
| `border-radius` | `var(--flint-radius-control)` |
| `background` | `var(--flint-color-background-surface)` |
| `color` | `var(--flint-color-text-primary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--flint-typography-body-family)` |
| `font-size` | `var(--flint-typography-body-size)` |
| `line-height` | `var(--flint-typography-body-lineHeight)` |
| `text-align` | `left` |

### Trigger — placeholder state (no value selected)

| Property | Value |
|----------|-------|
| `color` | `var(--flint-color-text-secondary)` |

### Trigger — hover

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--flint-color-background-surface) 86%, var(--flint-color-background-elevated))` |

### Trigger — focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Trigger — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--flint-state-opacity-disabled)` |

### Indicator `.date-picker__indicator`

| Property | Value |
|----------|-------|
| `color` | `var(--flint-color-text-secondary)` |
| `font-size` | `0.75rem` |

### Surface `.date-picker__surface`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `calc(100% + 0.375rem)` |
| `left` | `0` |
| `z-index` | `var(--flint-overlay-z-menu)` |
| `padding` | `var(--flint-space-panel-y) var(--flint-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--flint-color-border-default) 72%, transparent)` |
| `border-radius` | `var(--flint-radius-surface)` |
| `background` | `color-mix(in srgb, var(--flint-color-background-elevated) 98%, var(--flint-color-background-panel))` |
| `box-shadow` | `var(--flint-elevation-overlay)` |

## 9. Svelte Notes

- Public value uses ISO `YYYY-MM-DD` strings rather than browser `Date`
  instances
- Module-level `nextDatePickerId` counter generates unique IDs for each
  instance
- Composes the Calendar component internally; passes through `weekStartsOn`,
  `locale`, and manages `visibleMonth` tracking
- Controlled/uncontrolled pattern applies independently to both `value` and
  `open`
- Outside click detection closes the surface
- Escape key closes the surface and returns focus to the trigger
- When a date is selected in the Calendar, the picker auto-closes
- Surface uses `color-mix` for border and background blending
- Trigger displays the formatted selected date or the placeholder text

## 10. GPUI Notes

- expected crate/module surface: `flint_gpui::primitives::date_picker`
- GPUI must model the trigger as a button with popup ownership semantics
- The overlay must present as a dialog containing a Calendar grid
- Focus management: opening must move focus into the calendar; closing must
  return focus to the trigger
- Outside-click dismissal and Escape handling must match
- `color-mix` formulas for surface background and border must be replicated
  using platform color blending or pre-computed equivalents
- Overlay positioning may differ from CSS absolute positioning but must appear
  anchored below the trigger

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and valueChange semantics match (controlled and uncontrolled)
- [ ] open and openChange semantics match (controlled and uncontrolled)
- [ ] trigger-to-popup relationship matches (haspopup, expanded, controls)
- [ ] surface role="dialog" matches
- [ ] Escape closes and returns focus to trigger
- [ ] outside click closes
- [ ] date selection closes the picker
- [ ] placeholder display matches when no value
- [ ] disabled state matches (trigger non-interactive)
- [ ] Calendar keyboard behavior matches when open

### Tier 2: Visual Parity

- [ ] root min-width (14rem) matches
- [ ] trigger sizing (control-height, control-x padding) matches
- [ ] trigger typography (body-family, body-size, body-lineHeight) matches
- [ ] trigger border and radius (border-default, radius-control) match
- [ ] trigger hover background (color-mix 86%) matches
- [ ] trigger focus ring (border-width-focus, focusRing, 0.125rem offset) matches
- [ ] placeholder color (text-secondary) matches
- [ ] indicator color (text-secondary) and size (0.75rem) match
- [ ] surface positioning (calc(100% + 0.375rem) below trigger) matches
- [ ] surface border (color-mix border-default 72%) matches
- [ ] surface background (color-mix elevated 98% with panel) matches
- [ ] surface shadow (elevation-overlay) matches
- [ ] surface padding (panel-y, panel-x) matches
- [ ] surface radius (radius-surface) matches
- [ ] surface z-index (overlay-z-menu) matches
- [ ] disabled opacity (state-opacity-disabled) matches

### Tier 3: Implementation Freedom

- [ ] overlay positioning strategy (CSS absolute vs platform anchoring) is platform-owned
- [ ] outside-click detection mechanism is implementation-specific
- [ ] module-level ID counter is implementation-specific
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| popup positioning details may differ | overlay runtime details differ by platform | allowed | keep value, open, and focus semantics strict |
| color-mix formula implementation | GPUI may use pre-computed color blending | allowed | same visual result required |
| outside-click detection mechanism | CSS/JS vs native platform event handling | allowed | dismissal behavior must match |
| surface anchoring gap | platform overlay anchoring may differ slightly | allowed | surface must appear below and aligned to trigger |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Default | `ariaLabel="Select date"` | Trigger button showing placeholder text "Select date" with disclosure indicator; interactive, opens calendar on click |

### With default value

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With default value | `defaultValue="2026-03-14"`, `ariaLabel="Pre-filled date"` | Trigger button showing formatted date (March 14, 2026) instead of placeholder |

### Disabled

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Disabled | `placeholder="Disabled"`, `isDisabled=true`, `ariaLabel="Disabled date picker"` | Trigger button with "Disabled" placeholder, reduced opacity, cursor not-allowed, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: forms, filter bars, inspector controls, settings rows,
  DateTimePicker (composition)
- future follow-up: align surface styling with Popover and Menu overlay
  contracts; consider validation state support if needed; consider min/max date
  constraints passed through to Calendar
