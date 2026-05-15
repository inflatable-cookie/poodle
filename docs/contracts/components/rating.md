# Rating

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Rating`
- Layer: `foundation`
- Summary: an ordinal judgment control for choosing a bounded score using a
  row of selectable star items
- In scope: bounded item count, single-value selection, optional clear-on-repeat,
  roving focus, disabled state
- Out of scope: review workflows, written feedback, weighted scoring systems,
  half-star values

## 2. Anatomy

```text
[Root .rating]
  └── [Item .rating__item] (repeated max times)
        └── [Glyph .rating__glyph]
              └── Icon (name="star" size="sm")
```

| Part | Element | Required | Description |
|------|---------|----------|-------------|
| Root | `<div>` | yes | inline-flex container with radiogroup role |
| Item | `<button>` | yes | individual rating option, repeated `max` times |
| Glyph | `<span>` | yes | icon wrapper inside each item |
| Icon | `Icon` component | yes | star icon at size `sm` |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `number \| null` | `null` | no | controlled selected value |
| `defaultValue` | `number \| null` | `null` | no | uncontrolled initial value |
| `max` | `number` | `5` | no | total number of rating items |
| `allowClear` | `boolean` | `false` | no | whether clicking the current value deselects it |
| `disabled` | `boolean` | `false` | no | disables all items |
| `ariaLabel` | `string \| null` | `null` | no | accessible group label |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Controlled And Uncontrolled

- bindable value plus callback: `value`, `onValueChange`
- leave `value` undefined to use uncontrolled mode seeded by `defaultValue`
- pass `value={null}` to use a controlled empty state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | no value selected | all items show unfilled color |
| selected | value is set | items up to and including value show filled color |
| hover/focus | item hovered or focus-visible | highlighted background on target item |
| disabled | `disabled=true` | all items show disabled opacity, cursor not-allowed |

### Component States

| State | Type | Description |
|-------|------|-------------|
| `selectedValue` | `number \| null` | currently selected rating value |
| `focusIndex` | `number` | index of the currently focusable item (roving focus) |

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | user selects or clears a rating | `number \| null` | fires on click or keyboard select |

## 6. Accessibility

### Semantics

- Role: `radiogroup` on root element
- `aria-label` on root from `ariaLabel` prop
- Each item: `role="radio"`, `aria-checked` (true for selected, false otherwise),
  `aria-label="{index+1} of {max}"`
- Disabled items: `disabled` attribute on button

### Keyboard

| Key | Behavior |
|-----|----------|
| `ArrowRight` / `ArrowUp` | move focus to next item |
| `ArrowLeft` / `ArrowDown` | move focus to previous item |
| `Home` | move focus to first item |
| `End` | move focus to last item |
| `Enter` / `Space` | select the focused item |

### Focus And Announcement

- focus entry: roving tabindex; only the focusable item (`focusIndex`) has
  `tabindex="0"`, all others have `tabindex="-1"`
- focus wraps: arrow keys wrap from last to first and vice versa
- live-region behavior: none
- GPUI-native accessibility mapping notes: map to a radio-group-like control
  with bounded ordinal options

## 7. Layout

### Sizing

- Items pack inline with a small gap
- Each item is a fixed 2rem x 2rem touch target
- Icon glyph renders at 1rem font-size

### Composition

- parent expectations: form fields, review sections, preference capture
- child expectations: none (items are internally generated)

## 8. Token Usage

### Root `.rating`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.125rem` |

### ARIA attributes on root

| Attribute | Value |
|-----------|-------|
| `role` | `radiogroup` |
| `aria-label` | from `ariaLabel` prop |

### Item `.rating__item` (default)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `2rem` |
| `height` | `2rem` |
| `border` | `0` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `color-mix(in srgb, var(--poodle-color-text-secondary) 78%, transparent)` |
| `cursor` | `pointer` |
| `font` | `inherit` |

### ARIA attributes on each item

| Attribute | Value |
|-----------|-------|
| `role` | `radio` |
| `aria-checked` | `true` if selected, `false` otherwise |
| `aria-label` | `"{index+1} of {max}"` |
| `tabindex` | `0` for focused item, `-1` for all others |

### Item filled `.rating__item[data-filled="true"]`

| Property | Value |
|----------|-------|
| `color` | `color-mix(in srgb, var(--poodle-color-accent-base) 84%, var(--poodle-color-text-primary))` |

### Item hover/focus `.rating__item:hover:not(:disabled)`, `.rating__item:focus-visible`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 14%, transparent)` |
| `outline` | `none` |

### Item disabled `.rating__item:disabled`

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Glyph `.rating__glyph`

| Property | Value |
|----------|-------|
| `font-size` | `1rem` |
| `line-height` | `1` |

### Icon

| Prop | Value |
|------|-------|
| `name` | `"star"` |
| `size` | `"sm"` |

### Size adjustments

| Size | item width/height | glyph font-size |
|------|-------------------|-----------------|
| `xs` | `calc(icon-default + 0.375rem)` | `0.75rem` |
| `sm` | `calc(icon-default + 0.5rem)` | `0.875rem` |
| `md` | `2rem` | `1rem` |
| `lg` | `calc(icon-default + 1rem)` | `1.125rem` |
| `xl` | `calc(icon-default + 1.25rem)` | `1.25rem` |

## 9. Svelte Notes

- Uses the `Icon` component internally for star glyphs
- Roving focus is managed via a `focusIndex` state variable; only the item at
  `focusIndex` receives `tabindex="0"`
- Filled state is determined by comparing each item's index against the current
  selected value and applied via `data-filled` attribute
- Supports both controlled and uncontrolled patterns with bindable `value` and
  `defaultValue`
- `data-size` attribute on root reflects the resolved size for CSS variant styling
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::rating`
- glyph shape may differ from web star icon; the contract is ordinal selection,
  not glyph-specific branding
- `color-mix` blending should be replicated using equivalent alpha-blended color
  calculations in GPUI's color system
- roving focus should map to GPUI's focus management primitives

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `radiogroup` semantics with `radio` items
- [ ] `aria-checked` reflects selected state
- [ ] roving tabindex focus management
- [ ] keyboard navigation (arrows, home, end, enter, space)
- [ ] `allowClear` deselection behavior

### Tier 2: Visual Parity

- [ ] all five sizes visually match (height, padding, font-size per size table)
- [ ] 2rem x 2rem item touch targets
- [ ] 0.125rem gap between items
- [ ] unfilled color matches 78% secondary text mix
- [ ] filled color matches 84% accent / primary text mix
- [ ] hover/focus background matches 14% accent mix
- [ ] disabled opacity matches `--poodle-state-opacity-disabled`

### Tier 3: Implementation Freedom

- [ ] glyph rendering internals (star shape, SVG vs font icon)
- [ ] `color-mix` may be replaced by pre-computed equivalents

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| glyph style may differ | shape rendering is implementation-specific | allowed | keep ordinal meaning strict |
| `color-mix` implementation | GPUI may pre-compute blended colors | allowed | ensure visual equivalence across themes |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default (5 stars)

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Default (5 stars) | `value=3`, `ariaLabel="Rating"` | Row of 5 star items; first 3 filled (accent color), last 2 unfilled (secondary color); interactive with roving focus |

### 10-star scale

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| 10-star scale | `defaultValue=7`, `max=10`, `ariaLabel="Score out of 10"` | Row of 10 star items; first 7 filled, last 3 unfilled |

### Clearable

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Clearable | `defaultValue=4`, `allowClear=true`, `ariaLabel="Clearable rating"` | Row of 5 star items; first 4 filled; clicking the currently selected star clears the value |

### Disabled

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Disabled | `defaultValue=2`, `disabled=true`, `ariaLabel="Disabled rating"` | Row of 5 star items; first 2 filled; reduced opacity, cursor not-allowed, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: quality scoring, preference capture, review forms
- future follow-up: consider half-star or fractional rating support as a
  separate contract extension
