# Date Range Picker

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `DateRangePicker`
- Layer: `foundation`
- Summary: a range value control that combines a picker trigger with a
  calendar-based bounded range selection overlay
- In scope: range display, open state, selected start and end dates,
  calendar (range mode) overlay, placeholder behavior, outside-click and Escape
  dismissal, controlled and uncontrolled value and open state
- Out of scope: recurring windows, time ranges, report presets, availability
  logic, preset range shortcuts

## 2. Anatomy

```text
[Root .date-range-picker]  <div>
  ├── [Trigger .date-range-picker__trigger]  <button>
  │     ├── [Value .date-range-picker__value]  <span>
  │     └── [Indicator .date-range-picker__indicator]  <span>
  └── [Surface .date-range-picker__surface]  <div role="dialog"> (conditional, when open)
        └── [Calendar mode="range"] (composed)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | positioning container for trigger and overlay | position, display, min-width |
| Trigger | yes | button that toggles the overlay | border, radius, background, typography, focus ring, padding |
| Value | yes | displays selected range or placeholder text | color, text-align, truncation |
| Indicator | yes | decorative disclosure chevron | color, font-size |
| Surface | yes | overlay containing the calendar | position, border, radius, background, shadow, padding |
| Calendar (range) | yes | composed Calendar with mode="range" | delegated to Calendar contract (range mode) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `DateRangeValue \| null` | `null` | no | controlled selected range |
| `defaultValue` | `DateRangeValue` | `{ start: null, end: null }` | no | uncontrolled initial range |
| `open` | `boolean \| null` | `null` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial open state |
| `placeholder` | `string` | `"Select date range"` | no | shown when no range selected |
| `weekStartsOn` | `"sunday" \| "monday"` | `"monday"` | no | first day of the week |
| `locale` | `string` | `"en-US"` | no | locale for date formatting |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `disabled` | `boolean` | `false` | no | disables the trigger |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |

### Type Definitions

```
DateRangeValue: { start: string | null; end: string | null }
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
| start selected | start date chosen, end pending | trigger shows `"<start> – End date"` (formatted start, en-dash, literal `End date`); overlay remains open |
| complete range | both start and end committed | trigger shows `"<start> – <end>"` (both formatted, en-dash separated); overlay auto-closes |
| open | trigger clicked or keyboard activated | surface appears below trigger |
| disabled | `disabled=true` | reduced opacity, non-interactive, cursor: not-allowed |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| value committed | user selects both start and end | `onValueChange` runs with complete range |
| auto-close | both start and end selected | overlay closes, `onOpenChange` runs with `false` |
| dismissed | Escape or click outside | overlay closes without changing value |

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | user completes a range selection | `DateRangeValue` | runs when both start and end are selected |
| `onOpenChange` | overlay opens or closes | `boolean` | runs on open and close transitions |

## 6. Accessibility

### Semantics

- Trigger: `<button>` with `aria-haspopup="dialog"`, `aria-expanded` (true/false), `aria-controls` pointing to surface id
- Surface: `role="dialog"`, unique id referenced by `aria-controls`
- Trigger accessible name from external label or `ariaLabel` prop
- Disabled: `disabled` attribute on trigger button
- Module-level `nextDateRangePickerId` counter generates unique ids for ARIA relationships

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` | toggles overlay open/closed |
| `Escape` | closes overlay without changing value |
| `Tab` | when open, moves focus into calendar; when closed, exits control |

### Focus And Announcement

- focus entry: trigger receives focus ring via outline
- focus transition: opening the overlay moves focus into the calendar
- focus restoration: closing the overlay returns focus to the trigger
- live-region behavior: none; calendar handles date announcement
- GPUI-native accessibility mapping notes: GPUI must expose button with haspopup, expanded state, and dialog relationship through native accessibility APIs

## 7. Layout

### Sizing

- Root min-width: `16rem`
- Trigger height follows `size-control-height` token
- Surface is absolutely positioned below trigger with a gap

### Composition

- parent expectations: forms, filter bars, reporting controls, inspector panels
- child expectations: composes Calendar (mode="range") internally; no child slots
- resizing rules: trigger stretches to parent width; value text truncates with ellipsis

## 8. Token Usage — Exact Values

### Root `.date-range-picker`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-grid` |
| `min-width` | `16rem` |

### Trigger `.date-range-picker__trigger`

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

### Value — placeholder state `.date-range-picker__value--placeholder`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |

### Indicator `.date-range-picker__indicator`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.75rem` |
| `line-height` | `1` |

### Surface `.date-range-picker__surface`

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

- Module-level `nextDateRangePickerId` counter generates unique ids for each
  instance to wire ARIA relationships (`aria-controls`, `aria-expanded`)
- Controlled/uncontrolled pattern: supplying `value` makes it host-owned,
  including `null` for a controlled empty state; otherwise `defaultValue`
  seeds internal state
- Same pattern for `open`/`defaultOpen`: supplying `open` makes visibility
  host-owned
- Outside click handler closes the overlay; Escape key closes the overlay
- Composes `Calendar` with `mode="range"` internally; auto-closes overlay when both start and
  end dates are committed
- Value display formats the range using `locale` prop for localized date strings
- Placeholder option rendered as `<span>` with secondary text color
- `data-size` data attribute on root reflects the resolved size
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::date_range_picker`
- GPUI must implement trigger button with dialog overlay pattern
- Must expose haspopup, expanded state, and dialog relationship through native
  accessibility APIs
- `color-mix` formulas for surface border (72%), background (98%), and trigger
  hover (86%) must be replicated or approximated
- Auto-close on range completion must match Svelte behavior
- Calendar composition: GPUI delegates to its own calendar primitive in range mode

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and onValueChange semantics match (fires on complete range only)
- [ ] onOpenChange runs on open and close transitions
- [ ] auto-close when both start and end are selected matches
- [ ] Escape closes overlay without changing value
- [ ] outside click closes overlay
- [ ] disabled state prevents interaction
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
- [ ] disabled opacity uses state-opacity-disabled token
- [ ] all five sizes visually match (height, padding, font-size per size table)

### Tier 3: Implementation Freedom

- [ ] overlay positioning/clipping strategy is platform-owned
- [ ] id generation strategy is implementation-owned
- [ ] date formatting details may vary by platform locale support

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| overlay positioning details may differ | GPUI overlay system differs from CSS absolute positioning | allowed | must appear anchored below trigger visually |
| color-mix approximation in GPUI | GPUI may not have CSS color-mix; equivalent blending acceptable | allowed | visual result must match |
| date formatting locale support may vary | platform locale registries differ | allowed | keep ISO date value semantics strict |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Default | `ariaLabel="Select date range"` | Trigger button showing placeholder text "Select date range" with disclosure indicator; interactive, opens calendar on click |

### With default range

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With default range | `defaultValue={ start: "2026-03-01", end: "2026-03-14" }`, `ariaLabel="Pre-filled range"` | Trigger button showing formatted date range instead of placeholder |

### Disabled

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Disabled | `disabled=true`, `ariaLabel="Disabled range picker"` | Trigger button with default placeholder, reduced opacity, cursor not-allowed, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: reporting filters, review windows, bounded search forms,
  analytics date range selectors
- future follow-up: consider preset range shortcuts as a composite wrapper;
  align overlay placement with Popover rules if needed
