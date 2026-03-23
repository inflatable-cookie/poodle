# Range Slider

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `RangeSlider`
- Layer: `foundation`
- Summary: a dual-thumb range control representing lower and upper numeric
  bounds, built on two overlapping native range inputs with a custom track and
  fill visualization driven by CSS custom properties
- In scope: lower/upper value pair, min/max bounds, stepped adjustment,
  separate thumb focus, horizontal and vertical orientation, value commit
  semantics
- Out of scope: histogram overlays, arbitrary multi-thumb editing beyond two
  thumbs, single-value selection (see Slider)

## 2. Anatomy

```text
[Root .range-slider]  <div>
  ├── [Track .range-slider__track]  <span>
  │     └── [Fill .range-slider__fill]  <span>
  ├── [Lower Control .range-slider__control]  <input type="range"> (lower thumb)
  └── [Upper Control .range-slider__control]  <input type="range"> (upper thumb)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | range slider host with relative positioning | sizing, disabled opacity |
| Track | yes | full available range background bar | background, radius |
| Fill | yes | selected range window between lower and upper values | accent color, positioning |
| Lower Control | yes | native range input for lower bound thumb | thumb styling, focus ring |
| Upper Control | yes | native range input for upper bound thumb | thumb styling, focus ring |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `[number, number]` | `[0, 100]` | no | controlled lower/upper pair |
| `min` | `number` | `0` | no | lower bound |
| `max` | `number` | `100` | no | upper bound |
| `step` | `number` | `1` | no | increment size |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | layout and interaction axis |
| `isDisabled` | `boolean` | `false` | no | disables interaction, applies disabled opacity |
| `ariaLabel` | `string \| null` | `null` | no | base accessible name for the control |
| `lowerValueText` | `string \| null` | `null` | no | human-readable text for lower thumb (aria-valuetext) |
| `upperValueText` | `string \| null` | `null` | no | human-readable text for upper thumb (aria-valuetext) |

### Controlled And Uncontrolled

- controlled-only in this baseline contract; parent owns the value pair and
  updates it via the `valueChange` event
- the lower <= upper invariant is always preserved; if a thumb is dragged past
  the other, it clamps to the other thumb's position

### CSS Custom Properties

| Var | Description |
|-----|-------------|
| `--poodle-range-start` | computed as `((lower - min) / (max - min)) * 100%`; positions fill start |
| `--poodle-range-end` | computed as `((upper - min) / (max - min)) * 100%`; positions fill end |

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | both thumbs visible on track, fill spans between them |
| focus-lower | lower thumb focused | focus ring on lower thumb |
| focus-upper | upper thumb focused | focus ring on upper thumb |
| active | a thumb is being dragged or keyboard-adjusted | active interaction state |
| disabled | `isDisabled=true` | reduced opacity via disabled token |

### Component States

- lower value and upper value
- active thumb identity (which thumb is being adjusted)

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | either thumb changes value during interaction | `{ value: [number, number] }` | live updates; lower <= upper invariant preserved |
| `valueCommit` | interaction finishes on either thumb | `{ value: [number, number] }` | fires on mouseup/touchend/keyup commit |

## 6. Accessibility

### Semantics

- Role: two related `<input type="range">` elements (slider role each)
- Lower input `aria-label`: `"{ariaLabel} minimum"` when ariaLabel is provided,
  otherwise `"Minimum value"`
- Upper input `aria-label`: `"{ariaLabel} maximum"` when ariaLabel is provided,
  otherwise `"Maximum value"`
- `aria-valuemin`: from min prop on both inputs
- `aria-valuemax`: from max prop on both inputs
- `aria-valuenow`: lower value on lower input, upper value on upper input
- `aria-valuetext`: from lowerValueText on lower input, upperValueText on upper
  input
- `aria-orientation`: set to match orientation prop on both inputs
- `disabled`: native disabled attribute when isDisabled on both inputs
- Labeling rules: each thumb must be individually focusable and distinguishable
  to assistive technology as lower or upper bound

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Down` | decrements the focused thumb by step |
| `Arrow Right/Up` | increments the focused thumb by step |
| `Home` | moves the focused thumb to min (lower) or to lower value (upper) |
| `End` | moves the focused thumb to upper value (lower) or to max (upper) |
| `Tab` | moves focus between lower thumb, upper thumb, and out of control |

### Focus And Announcement

- focus entry: one thumb at a time is focusable; Tab moves between the two
  thumbs
- focus exit: both thumb values remain visible and accessible
- live-region behavior: none; per-thumb slider semantics announce value changes
- GPUI-native accessibility mapping notes: GPUI must expose lower and upper
  thumbs as distinct value controls with clear accessible naming (minimum/
  maximum) and bound semantics

## 7. Layout

### Sizing

- horizontal: width is 100% of parent; min-height 1.5rem for touch target
- vertical: width is 1.5rem; min-height 10rem; height is 100% of parent
- track thickness is 0.375rem (same as Slider)
- thumb is 1rem diameter (same as Slider)
- track length is parent-owned

### Composition

- parent expectations: filters, inspectors, range constraints, Field wrappers
- child expectations: none
- resizing rules: orientation changes axis without changing pair semantics;
  thumb overlap and crossing behavior must preserve the lower<=upper invariant

## 8. Token Usage — Exact Values

### Root `.range-slider`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-flex` |
| `align-items` | `center` |
| `width` | `100%` |
| `min-height` | `1.5rem` |

### Root vertical `[data-orientation="vertical"]`

| Property | Value |
|----------|-------|
| `width` | `1.5rem` |
| `min-width` | `1.5rem` |
| `min-height` | `10rem` |
| `height` | `100%` |
| `justify-content` | `center` |

### Root disabled `[data-disabled="true"]`

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Track `.range-slider__track`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `inset` | `50% 0 0` |
| `height` | `0.375rem` |
| `transform` | `translateY(-50%)` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 88%, transparent)` |

### Track vertical `[data-orientation="vertical"] .range-slider__track`

| Property | Value |
|----------|-------|
| `inset` | `0 auto 0 50%` |
| `width` | `0.375rem` |
| `height` | `100%` |
| `transform` | `translateX(-50%)` |

### Fill `.range-slider__fill`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `left` | `var(--poodle-range-start)` |
| `width` | `calc(var(--poodle-range-end) - var(--poodle-range-start))` |
| `height` | `100%` |
| `border-radius` | `inherit` |
| `background` | `var(--poodle-color-accent-base)` |

### Fill vertical `[data-orientation="vertical"] .range-slider__fill`

| Property | Value |
|----------|-------|
| `left` | `0` |
| `bottom` | `var(--poodle-range-start)` |
| `width` | `100%` |
| `height` | `calc(var(--poodle-range-end) - var(--poodle-range-start))` |

### Control `.range-slider__control`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `50%` |
| `left` | `0` |
| `right` | `0` |
| `transform` | `translateY(-50%)` |
| `width` | `100%` |
| `margin` | `0` |
| `background` | `transparent` |
| `appearance` | `none` |
| `-webkit-appearance` | `none` |
| `pointer-events` | `none` |

### Control vertical `[data-orientation="vertical"] .range-slider__control`

| Property | Value |
|----------|-------|
| `width` | `10rem` |
| `left` | `50%` |
| `top` | `50%` |
| `transform` | `translate(-50%, -50%) rotate(-90deg)` |

### Control focus-visible `.range-slider__control:focus-visible`

| Property | Value |
|----------|-------|
| `outline` | `none` |

### Thumb `::-webkit-slider-thumb`

| Property | Value |
|----------|-------|
| `width` | `1rem` |
| `height` | `1rem` |
| `margin-top` | `-0.3125rem` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `999px` |
| `background` | `var(--poodle-color-background-elevated)` |
| `box-shadow` | `0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent)` |
| `appearance` | `none` |
| `-webkit-appearance` | `none` |
| `pointer-events` | `auto` |

### Thumb `::-moz-range-thumb`

| Property | Value |
|----------|-------|
| `width` | `1rem` |
| `height` | `1rem` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `999px` |
| `background` | `var(--poodle-color-background-elevated)` |
| `box-shadow` | `0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent)` |
| `appearance` | `none` |
| `pointer-events` | `auto` |

### Thumb focus `:focus-visible::-webkit-slider-thumb`

| Property | Value |
|----------|-------|
| `box-shadow` | `0 0 0 0.1875rem color-mix(in srgb, var(--poodle-color-accent-focusRing) 32%, transparent), 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent)` |

### Thumb focus `:focus-visible::-moz-range-thumb`

| Property | Value |
|----------|-------|
| `box-shadow` | `0 0 0 0.1875rem color-mix(in srgb, var(--poodle-color-accent-focusRing) 32%, transparent), 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent)` |

### Track pseudo `::-webkit-slider-runnable-track`

| Property | Value |
|----------|-------|
| `height` | `0.375rem` |
| `background` | `transparent` |

### Track pseudo `::-moz-range-track`

| Property | Value |
|----------|-------|
| `height` | `0.375rem` |
| `background` | `transparent` |

## 9. Svelte Notes

- Uses two overlapping native `<input type="range">` elements, both absolutely
  positioned over the same track
- Both inputs have `pointer-events: none` on the control element, with
  `pointer-events: auto` restored on the thumb pseudo-elements, so only the
  thumbs are grabbable
- CSS custom properties `--poodle-range-start` and `--poodle-range-end` are computed
  reactively and set as inline styles on the root element
- The fill element uses `left` and `calc(end - start)` for width in horizontal
  mode, `bottom` and `calc(end - start)` for height in vertical mode
- Lower thumb input has its max clamped to the upper value; upper thumb input
  has its min clamped to the lower value, preserving the lower<=upper invariant
- `valueChange` fires on the `input` event (live during drag); `valueCommit`
  fires on the `change` event (on release)
- `data-orientation` and `data-disabled` attributes on root drive layout and
  state styling
- Per-thumb aria-label is constructed by appending "minimum"/"maximum" to the
  base ariaLabel prop, or defaults to "Minimum value"/"Maximum value"

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::range_slider`
- GPUI implementation must expose two separately focusable value handles with
  distinct accessibility names ("minimum"/"maximum") and keyboard adjustment
  behavior
- each thumb must report its own value/min/max independently
- the lower<=upper invariant must be enforced during both keyboard and pointer
  interaction
- vertical orientation must be implemented natively rather than via CSS rotation
- pointer overlap handling (determining which thumb to grab when thumbs are at
  the same position) is implementation-specific but must produce a usable result

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] lower/upper value pair semantics and invariant handling match
- [ ] per-thumb accessibility (distinct aria-labels, separate focus) matches
- [ ] keyboard adjustment behavior matches (arrows, Home, End per-thumb)
- [ ] Tab navigation between thumbs matches
- [ ] valueChange fires during interaction, valueCommit fires on release
- [ ] disabled behavior matches

### Tier 2: Visual Parity

- [ ] track thickness (0.375rem) and border-radius (999px) match
- [ ] thumb sizing (1rem diameter) matches
- [ ] thumb border, background (elevated), and box-shadow match
- [ ] fill uses accent-base color
- [ ] fill positioning uses range-start and range-end correctly
- [ ] track background uses semi-transparent surface color-mix
- [ ] focus ring uses compound box-shadow with focusRing 32% opacity
- [ ] disabled opacity uses state-opacity-disabled
- [ ] vertical layout dimensions match

### Tier 3: Implementation Freedom

- [ ] two overlapping native range inputs vs GPUI custom dual-thumb control
- [ ] CSS rotation for vertical vs native vertical implementation
- [ ] webkit/moz thumb pseudo-elements are Svelte-specific
- [ ] pointer overlap/grab priority is platform-owned
- [ ] pointer/gesture drag feel is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| pointer overlap handling may differ | gesture internals are runtime-specific | allowed | keep keyboard and value semantics strict |
| two overlapping inputs vs single custom control | DOM pattern is Svelte-specific | allowed | same interaction and a11y result required |
| vertical via CSS rotation vs native | Svelte uses rotate(-90deg); GPUI implements natively | allowed | same visual and interaction result required |
| color-mix formulas | GPUI must achieve same visual result by any means | allowed | verify visual parity |

## 13. Specimen Definitions

### Group: Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `value=[20, 80]`, `min=0`, `max=100`, `ariaLabel="Price range"` | Dual-thumb slider with fill spanning from 20% to 80% of track; live value display showing "$20 -- $80" updates on drag |

### Group: With step

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With step | `value=[25, 45]`, `min=18`, `max=65`, `step=5`, `ariaLabel="Age range"` | Dual-thumb slider snapping to increments of 5; live value display showing "Ages 25 -- 45" |

### Group: Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `value=[30, 70]`, `min=0`, `max=100`, `isDisabled=true`, `ariaLabel="Disabled range"` | Reduced opacity; thumbs non-interactive; fill visible between 30% and 70% |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: filters, inspectors, bounded-range editors, Field-wrapped
  form inputs
- future follow-up: consider a value-label wrapper once forms/composites deepen;
  coordinate with Slider contract for shared thumb/track token consistency
