# Range Slider

Status: detailed contract
Updated: 2026-08-11

## 1. Purpose

- Component name: `RangeSlider`
- Layer: `foundation`
- Summary: a dual-thumb range control representing lower and upper numeric
  bounds, built on two overlapping native range inputs with a custom track and
  fill visualization driven by CSS custom properties
- In scope: lower/upper value pair, min/max bounds, stepped adjustment,
  separate thumb focus, horizontal and vertical orientation, value commit
  semantics, standard and embedded variants, unipolar and bipolar reference
  geometry
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
| `variant` | `"standard" \| "embedded"` | `"standard"` | no | native-input control or dense composite control |
| `polarity` | `"unipolar" \| "bipolar"` | `"unipolar"` | no | ordinary range or range with an explicit bipolar center reference |
| `centerValue` | `number \| null` | `null` | no | bipolar reference; defaults to zero when zero is inside the range, otherwise the midpoint |
| `law` | `AudioValueLaw` | `linear` | no | embedded-variant value mapping; standard remains the native linear range path |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | layout and interaction axis |
| `disabled` | `boolean` | `false` | no | disables interaction, applies disabled opacity |
| `ariaLabel` | `string \| null` | `null` | no | base accessible name for the control |
| `lowerValueText` | `string \| null` | `null` | no | human-readable text for lower thumb (aria-valuetext) |
| `upperValueText` | `string \| null` | `null` | no | human-readable text for upper thumb (aria-valuetext) |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Controlled And Uncontrolled

- bindable value pair plus callbacks: `value`, `onValueChange`, `onValueCommit`
- the lower <= upper invariant is always preserved; if a thumb is dragged past
  the other, it clamps to the other thumb's position

### Pointer

- pressing anywhere on the track moves the **nearer** thumb to that position
- a drag keeps the thumb the press chose, even once it passes its partner — the
  clamp above applies, but the gesture never transfers to the other thumb

On the web both fall out of the two overlapping native range inputs. The Rust
targets have no native input, so a single grab overlay spanning the track
reports the pointer's position as a fraction (`Interaction::on_scrub`) and the
component picks the thumb on the press.

The fraction matters: an earlier implementation converted pixel *deltas* using a
fixed 10rem track width, so any range slider not rendered at exactly 160px
tracked the pointer at the wrong rate — and a delta cannot express "jump to
where I pressed" at all, so the track was inert.

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
| disabled | `disabled=true` | reduced opacity via disabled token |

### Component States

- lower value and upper value
- active thumb identity (which thumb is being adjusted)

### Behavior Machine

Behavior classification: machine-backed (`rangeSliderTransition` in
`@inflatable-cookie/poodle-core`)

Two-thumb variant of the Slider machine; native range inputs provide
interaction.

- Context: `value: [lower, upper]` (controllable), `min`, `max`, `step`,
  `disabled`
- Events: `INPUT { thumb, raw }`, `COMMIT { thumb, raw }`, `SET_VALUE`
- Normalization: `normalizeRangeValue` orders the pair and clamps both ends
  into `[min, safeMax]`; per-event, the raw value snaps to step and a thumb
  cannot cross its sibling (lower clamps to `[min, upper]`, upper to
  `[lower, max]`)
- Transitions: `INPUT` sets the pair, effect `emitValueChange(pair)`;
  `COMMIT` sets the pair, effect `emitValueCommit(pair)`
- Machinery dependencies: none.

### Embedded Control Variant

`variant="embedded"` uses `rangeSliderControlTransition`. The adapter reports
normalized pointer coordinates; the core selects the nearer thumb on begin,
holds that thumb for the gesture, maps through the declared law, prevents
crossing, and emits the change/commit split. `RangeSliderVisualState` publishes
both normalized thumb positions, the center reference, and separate negative
and positive selected-fill segments.

Unipolar and bipolar variants both preserve RangeSlider's low-to-high selected
window. Bipolar splits that window at the explicit center reference: the
negative segment uses the negative status color and the positive segment uses
the accent color. It does not reinterpret the pair as two unrelated scalar
values. Unipolar publishes an empty negative segment and one positive segment.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | either thumb changes value during interaction | `[number, number]` | live updates; lower <= upper invariant preserved |
| `onValueCommit` | interaction finishes on either thumb | `[number, number]` | fires on mouseup/touchend/keyup commit |

## 6. Accessibility

### Semantics

- Role: two related `<input type="range">` elements (slider role each)
- Embedded variant: two adapter-owned slider focus stops expose the same
  per-thumb semantics while pointer capture remains on the shared root
- Lower input `aria-label`: `"{ariaLabel} minimum"` when ariaLabel is provided,
  otherwise `"Minimum value"`
- Upper input `aria-label`: `"{ariaLabel} maximum"` when ariaLabel is provided,
  otherwise `"Maximum value"`
- `aria-valuemin`: from min prop on both inputs
- `aria-valuemax`: from max prop on both inputs
- `aria-valuenow`: lower value on lower input, upper value on upper input
- `aria-valuetext`: from lowerValueText on lower input, upperValueText on upper
  input
- `aria-orientation`: NOT set on the range inputs; orientation is conveyed via
  `data-orientation` on the root element only (matches Svelte and the Slider
  contract)
- `disabled`: native disabled attribute when disabled on both inputs
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

- horizontal: width is 100% of parent; cross-size follows the size table
- vertical: cross-size follows the size table; min-height 10rem; height is 100%
- track thickness follows the shared Slider size ladder: 0.1875rem (`xs`),
  0.25rem (`sm`), 0.375rem (`md`), 0.5rem (`lg`), and 0.625rem (`xl`)
- thumb diameter follows the size table (`1rem` at `md`)
- track length is parent-owned
- embedded variant is valid inside dense composites and keeps the full root as
  its pointer target

### Composition

- parent expectations: filters, inspectors, range constraints, Field wrappers
- child expectations: none
- resizing rules: orientation changes axis without changing pair semantics;
  thumb overlap and crossing behavior must preserve the lower<=upper invariant

## 8. Token Usage — Exact Values

### Recipe hooks

- `--poodle-recipe-range-slider-track-fill`
- `--poodle-recipe-range-slider-fill-fill`
- `--poodle-recipe-range-slider-fill-negative`
- `--poodle-recipe-range-slider-track-border`
- `--poodle-recipe-range-slider-center-fill`
- `--poodle-recipe-range-slider-control-fill`
- `--poodle-recipe-range-slider-control-track-fill`
- `--poodle-recipe-range-slider-control-thumb-fill`
- `--poodle-recipe-range-slider-control-thumb-shadow`
- `--poodle-recipe-range-slider-focus-ring`
- `--poodle-recipe-range-slider-focus-control-thumb-shadow`

### Root `.range-slider`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-flex` |
| `align-items` | `center` |
| `width` | `100%` |
| `min-height` | `var(--poodle-range-slider-control-min-height)` (`1.5rem` at `md`) |

### Root vertical `[data-orientation="vertical"]`

| Property | Value |
|----------|-------|
| `width` | `var(--poodle-range-slider-control-min-height)` |
| `min-width` | `var(--poodle-range-slider-control-min-height)` |
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
| `height` | `var(--poodle-range-slider-track-thickness)` (`0.375rem` at `md`) |
| `transform` | `translateY(-50%)` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 88%, transparent)` |

### Track vertical `[data-orientation="vertical"] .range-slider__track`

| Property | Value |
|----------|-------|
| `inset` | `0 auto 0 50%` |
| `width` | `var(--poodle-range-slider-track-thickness)` (`0.375rem` at `md`) |
| `height` | `100%` |
| `transform` | `translateX(-50%)` |

### Fill segments `.range-slider__fill`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `height` | `100%` |
| `border-radius` | `inherit` |
| positive `left` | `var(--poodle-range-positive-start)` |
| positive `width` | `var(--poodle-range-positive-span)` |
| positive `background` | `var(--poodle-color-accent-base)` |
| negative `left` | `var(--poodle-range-negative-start)` |
| negative `width` | `var(--poodle-range-negative-span)` |
| negative `background` | `var(--poodle-recipe-range-slider-fill-negative, var(--poodle-color-status-danger))` |

### Fill vertical `[data-orientation="vertical"] .range-slider__fill`

| Property | Value |
|----------|-------|
| `left` | `0` |
| `width` | `100%` |
| positive `bottom` | `var(--poodle-range-positive-start)` |
| positive `height` | `var(--poodle-range-positive-span)` |
| negative `bottom` | `var(--poodle-range-negative-start)` |
| negative `height` | `var(--poodle-range-negative-span)` |

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
| `height` | `var(--poodle-range-slider-track-thickness)` |
| `background` | `transparent` |

### Track pseudo `::-moz-range-track`

| Property | Value |
|----------|-------|
| `height` | `var(--poodle-range-slider-track-thickness)` |
| `background` | `transparent` |

### Size adjustments

| Size | min-height | track thickness | thumb diameter |
|------|------------|-----------------|----------------|
| `xs` | `1.25rem` | `0.1875rem` | `0.75rem` |
| `sm` | `1.375rem` | `0.25rem` | `0.875rem` |
| `md` | `1.5rem` | `0.375rem` | `1rem` |
| `lg` | `1.625rem` | `0.5rem` | `1.125rem` |
| `xl` | `1.75rem` | `0.625rem` | `1.25rem` |

The WebKit thumb margin is derived from the size metrics as
`(thumb diameter - track thickness) / -2`, keeping both thumbs centered on the
track at every size.

### Density adjustments

| Density | Padding |
|---------|---------|
| `compact` | `0.25rem 0` |
| `default` | _(none)_ |
| `comfortable` | `0.75rem 0` |

Density adds vertical (`padding-block`) padding to the root. This is an
explicit, justified exception to the repo Size/Density rule ("density must
never affect vertical padding"): the slider is a thin control whose hit area is
a touch target, so density grows the surrounding vertical hit area without
changing the track thickness, thumb size, or the visual `min-height` of the
control itself. The padding sits outside the absolutely-positioned track/fill,
so the control geometry is unchanged — only the grabbable margin grows.

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
- `onValueChange` fires on the `input` event (live during drag); `onValueCommit`
  fires on the `change` event (on release)
- `data-orientation`, `data-disabled`, `data-size`, and `data-density` attributes on root drive
  layout and state styling
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- Per-thumb aria-label is constructed by appending "minimum"/"maximum" to the
  base ariaLabel prop, or defaults to "Minimum value"/"Maximum value"
- Orientation rides on `data-orientation` on the root only; `aria-orientation`
  is not emitted on the inputs. §6 matches this (Svelte is the parity authority,
  and the Slider contract aligns the same way), so there is no contract↔Svelte
  divergence on orientation reporting.

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

## 10a. Jetstream Notes

- `RangeSlider::from_spec(spec, theme).on_change(...).on_value_commit(...)`,
  reporting `(low, high)` together: the pair is the value, and a host told about
  one alone would have to remember the other.
- A thumb stops against its sibling rather than crossing it, because
  `range_slider_transition` says so. A first attempt re-derived the rule, chose
  to swap instead, and collapsed both thumbs onto one value — driving the shared
  machine is what makes the natives and the web agree.
- Only the thumbs are draggable. Dragging the filled span moves the whole window
  on the web, which is a different gesture and needs both values to travel
  together.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] lower/upper value pair semantics and invariant handling match
- [ ] per-thumb accessibility (distinct aria-labels, separate focus) matches
- [ ] keyboard adjustment behavior matches (arrows, Home, End per-thumb)
- [ ] Tab navigation between thumbs matches
- [ ] onValueChange fires during interaction, onValueCommit fires on release
- [ ] disabled behavior matches

### Tier 2: Visual Parity

- [ ] all five sizes visually match (height, padding, font-size per size table)
- [ ] track thickness follows the `xs`–`xl` size table and border-radius
  remains 999px
- [ ] thumb diameter follows the `xs`–`xl` size table
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
| Disabled | `value=[30, 70]`, `min=0`, `max=100`, `disabled=true`, `ariaLabel="Disabled range"` | Reduced opacity; thumbs non-interactive; fill visible between 30% and 70% |

### Group: Embedded unipolar control

`value=[0.2, 0.75]`, `min=0`, `max=1`, `step=0.01`,
`variant="embedded"`, and `polarity="unipolar"`.

### Group: Embedded bipolar control

`value=[-0.6, 0.35]`, `min=-1`, `max=1`, `step=0.01`,
`variant="embedded"`, and `polarity="bipolar"`. Density specimens use the
same variant.

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: filters, inspectors, bounded-range editors, Field-wrapped
  form inputs
- future follow-up: consider a value-label wrapper once forms/composites deepen;
  coordinate with Slider contract for shared thumb/track token consistency
