# Rating

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Rating`
- Layer: `foundation`
- Summary: an ordinal judgment control for choosing a bounded score using a
  row of selectable star items, with optional fractional display and stepped
  fractional input
- In scope: bounded item count, single-value selection, optional clear-on-repeat,
  roving focus for whole-step mode, slider-style stepped interaction for
  fractional mode, disabled state, partial star fill rendering
- Out of scope: review workflows, written feedback, weighted scoring systems,
  arbitrary pointer precision beyond the configured input step

## 2. Anatomy

```text
[Root .rating]
  └── [Item .rating__item] (repeated max times)
        └── [Glyph .rating__glyph]
              ├── [Base .rating__glyph-base]
              │     └── Icon (name="star" size="sm")
              └── [Fill .rating__glyph-fill]
                    └── [FillInner .rating__glyph-fill-inner]
                          └── Icon (name="star" size="sm")
```

| Part | Element | Required | Description |
|------|---------|----------|-------------|
| Root | `<div>` | yes | inline-flex container with `radiogroup` role in whole-step mode or `slider` role in fractional mode |
| Item | `<button>` | yes | individual rating option, repeated `max` times |
| Glyph | `<span>` | yes | icon wrapper inside each item |
| Base | `<span>` | yes | unfilled star layer |
| Fill | `<span>` | yes | clipped filled-star layer sized by per-star fill ratio |
| FillInner | `<span>` | yes | fixed-width fill glyph holder inside the clipped layer |
| Icon | `Icon` component | yes | star icon at the resolved size |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `number \| null` | `null` | no | controlled selected value |
| `defaultValue` | `number \| null` | `null` | no | uncontrolled initial value |
| `max` | `number` | `5` | no | total number of rating items |
| `step` | `number` | `0.5` | no | interactive input increment; values below `1` enable fractional mode, incoming display values may still be arbitrary fractions |
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
- incoming controlled/uncontrolled values are clamped to the valid range for
  display, but are not quantized to `step`
- user-generated changes are quantized to `step`

### Input vs Display Value

- Display accepts any fractional value within range, for example `3.7`
- Interactive input snaps to the configured `step`
- With the default `step={0.5}`, clicking, hovering, and keyboard changes snap
  to half-stars
- `step={1}` preserves whole-star selection behavior

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | no value selected | all items show unfilled color |
| selected | value is set | each star fills by its per-star ratio; full stars fill completely, the active partial star fills proportionally |
| hover/focus | item hovered or focus-visible | highlighted background on target item or glow on hovered fractional segment |
| disabled | `disabled=true` | all items show disabled opacity, cursor not-allowed |
| fractional | `step < 1` | root switches to slider semantics; stars remain button elements for pointer interaction but are hidden from accessibility tree |

### Component States

| State | Type | Description |
|-------|------|-------------|
| `selectedValue` | `number \| null` | currently selected rating value |
| `focusIndex` | `number` | index of the currently focusable item (roving focus) |
| `hoverValue` | `number \| null` | fractional hover preview value in fractional mode |

### Behavior Machine

Behavior classification: machine-backed via shared machinery

Machine-backed via core machinery (g11 extraction sweep): step resolution
(capped at 1), snap/clamp normalization, clear-on-reselect, pointer-ratio
value resolution (snap-up within an item, minimum one step), fill ratios,
fraction display trimming, and keyboard stepping (both directions floor at
`minSelectableValue`) live in `@poodle/headless` `rating.ts`. Hover state
and DOM geometry reads stay adapter-side.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | user selects or clears a rating | `number \| null` | fires on click or keyboard select |

## 6. Accessibility

### Semantics

- Whole-step mode (`step >= 1`):
  - root role: `radiogroup`
  - each item: `role="radio"`, `aria-checked`, `aria-label="{index+1} of {max}"`
- Fractional mode (`step < 1`):
  - root role: `slider`
  - root exposes `aria-valuemin="0"`, `aria-valuemax="{max}"`,
    `aria-valuenow`, and `aria-valuetext`
  - items render as non-interactive `<span>` pointer targets with
    `aria-hidden="true"` — `role="slider"` may not contain focusable elements,
    so they must not be `<button>` in this mode
- `aria-label` on root from `ariaLabel` prop
- Disabled items: `disabled` attribute on button

### Keyboard

| Key | Behavior |
|-----|----------|
| Whole-step: `ArrowRight` / `ArrowUp` | move focus to next item |
| Whole-step: `ArrowLeft` / `ArrowDown` | move focus to previous item |
| Whole-step: `Home` | move focus to first item |
| Whole-step: `End` | move focus to last item |
| Whole-step: `Enter` / `Space` | select the focused item |
| Fractional: `ArrowRight` / `ArrowUp` | increase value by `step` |
| Fractional: `ArrowLeft` / `ArrowDown` | decrease value by `step` |
| Fractional: `Home` | set to minimum selectable value (`0` when clearable, otherwise `step`) |
| Fractional: `End` | set to `max` |
| Fractional: `Enter` / `Space` | clear when `allowClear=true` and a value is present |

### Focus And Announcement

- whole-step focus entry: roving tabindex; only the focusable item (`focusIndex`) has
  `tabindex="0"`, all others have `tabindex="-1"`
- fractional focus entry: root slider receives `tabindex="0"`
- live-region behavior: none
- GPUI-native accessibility mapping notes:
  - whole-step mode maps to a radio-group-like control
  - fractional mode maps to a stepped slider-like control

## 7. Layout

### Sizing

- Items pack inline with a small gap
- Each item is a fixed 2rem x 2rem touch target
- Icon glyph renders at 1rem font-size
- Partial fills are achieved via a clipped overlay rather than alternate glyphs

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
| `role` | `radiogroup` when `step >= 1`; `slider` when `step < 1` |
| `aria-label` | from `ariaLabel` prop |
| `aria-valuemin` | `0` in fractional mode |
| `aria-valuemax` | `max` in fractional mode |
| `aria-valuenow` | current numeric value or `0` in fractional mode |
| `aria-valuetext` | `"No rating selected out of {max}"` or `"{value} out of {max}"` in fractional mode |

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
| `color` | `color-mix(in srgb, var(--poodle-color-text-secondary) 48%, transparent)` |
| `cursor` | `pointer` |
| `font` | `inherit` |

### ARIA attributes on each item

| Attribute | Value |
|-----------|-------|
| `role` | `radio` in whole-step mode |
| `aria-checked` | `true` if selected, `false` otherwise in whole-step mode |
| `aria-label` | `"{index+1} of {max}"` in whole-step mode |
| `aria-hidden` | `"true"` in fractional mode |
| `tabindex` | `0` for focused item and `-1` for others in whole-step mode; `-1` for all items in fractional mode |

### Item hover/focus

| Selector | Property | Value |
|----------|----------|-------|
| `.rating__item[data-hovered="true"]` | `filter` | `drop-shadow(0 0 0.375rem color-mix(in srgb, var(--poodle-color-accent-base) 52%, transparent))` |
| `.rating__item:focus-visible` | `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `.rating[data-mode="fractional"]:focus-visible` | `outline` | same focus ring on root slider |

### Item disabled `.rating__item:disabled`

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Glyph `.rating__glyph`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-flex` |

### Glyph Fill `.rating__glyph-fill`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `inset` | `0 auto 0 0` |
| `overflow` | `hidden` |
| `color` | `var(--poodle-color-accent-base)` |

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
- Roving focus is used only in whole-step mode
- Fractional mode uses a slider-like root with button children retained for
  pointer hit-targets
- Partial fill is calculated per star by clipping an accent-colored overlay
- Incoming values are clamped for display but not quantized to `step`
- User-generated values are quantized to `step`
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

- [ ] whole-step radiogroup semantics with radio items
- [ ] fractional slider semantics with stepped keyboard input
- [ ] `aria-checked` reflects selected state in whole-step mode
- [ ] `aria-valuenow` / `aria-valuetext` reflect fractional state
- [ ] roving tabindex focus management in whole-step mode
- [ ] keyboard navigation (whole-step roving, fractional step increments)
- [ ] `allowClear` deselection behavior

### Tier 2: Visual Parity

- [ ] all five sizes visually match (height, padding, font-size per size table)
- [ ] 2rem x 2rem item touch targets
- [ ] 0.125rem gap between items
- [ ] unfilled color matches 48% secondary text mix
- [ ] partial and full fills use accent overlay clipping cleanly
- [ ] hovered item glow matches accent drop-shadow
- [ ] disabled opacity matches `--poodle-state-opacity-disabled`

### Tier 3: Implementation Freedom

- [ ] glyph rendering internals (star shape, SVG vs font icon)
- [ ] `color-mix` may be replaced by pre-computed equivalents

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Jetstream has no half-star precision by click | a click reports the star it lands on; sub-star precision needs pointer position within the glyph | accepted, tracked | g12.017 |
| glyph style may differ | shape rendering is implementation-specific | allowed | keep ordinal meaning strict |
| `color-mix` implementation | GPUI may pre-compute blended colors | allowed | ensure visual equivalence across themes |

## 12a. Jetstream Notes

- `Rating::from_spec(spec, theme).on_change(...)`, carrying the rating the
  pressed star sets — 1-based, so the third star reports `3` rather than an
  index.
- Read-only and disabled ratings ignore clicks.

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

### Half-star steps

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Half-star steps | `value=3.5`, `step=0.5`, `allowClear=true`, `ariaLabel="Half-star rating"` | Row of 5 star items; first 3 stars fully filled, fourth star half-filled, fifth unfilled; pointer and keyboard input snap to half-stars |

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
- future follow-up: decide whether arbitrary display fractions plus stepped
  input should remain the permanent model or become separate explicit props
