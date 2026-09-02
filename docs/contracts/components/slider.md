# Slider

Status: detailed contract
Updated: 2026-09-02

## 1. Purpose

- Component name: `Slider`
- Layer: `foundation`
- Summary: a single-value continuous or stepped range control with a track,
  fill, and focusable value control; web may use a native range input while
  native runtimes draw the same semantic control directly
- In scope: current value, min/max bounds, step behavior, keyboard and pointer
  adjustment, value commit semantics, horizontal and vertical orientation,
  standard and embedded variants, unipolar and bipolar fill geometry, an
  opt-in horizontal `appearance="block"` treatment with explicit visible
  label/value content, fit fallback, and `ltr`/`rtl` direction
- Out of scope: dual-thumb range editing (see RangeSlider), knob/fader
  semantics, tick marks, vertical block appearance, PageUp/PageDown
  convergence, invalid/read-only/indeterminate states, a generic tooltip or
  public fit-metric API

## 2. Anatomy

Track appearance (`appearance="track"`, the default):

```text
[Root .slider]  <div>
  ├── [Track .slider__track]  <span>
  │     └── [Fill .slider__fill]  <span>
  └── [Control .slider__control]  <input type="range">
```

Block appearance (`appearance="block"`), horizontal only:

```text
[Root .slider data-appearance="block"]  <div>
  ├── [Capsule .slider__capsule]  <span>
  │     ├── [Selected .slider__fill]  <span>
  │     │     └── [Label .slider__inline-label]  (when every assigned item fits)
  │     ├── [Remainder .slider__remainder]  <span>
  │     │     └── [Value .slider__inline-value]
  │     └── [Hit .slider__hit]  (44×44 effective target; small visible thumb)
  └── [Fallback .slider__fallback]  (one stable line when inline text does not fit)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | slider host with relative positioning | sizing, disabled opacity |
| Track | yes, track appearance | full value range background bar | background, radius |
| Fill | yes | completed value span driven by CSS custom property; selected capsule in block | accent / selected fill, radius |
| Control | yes, track standard | native range input overlaid on the track for interaction | thumb styling, focus ring, appearance reset |
| Capsule | yes, block appearance | labelled rounded track that is the visual subject | selected/remainder fill |
| Hit | yes, block appearance | measurable 44×44 logical-pixel effective target around a small thumb | handle fill/border, focus |
| Fallback | when inline text does not fit | one noninteractive, accessibility-hidden line immediately after the capsule | remainder text |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `number` | `0` | no | current value |
| `min` | `number` | `0` | no | lower bound |
| `max` | `number` | `100` | no | upper bound |
| `step` | `number` | `1` | no | increment size |
| `variant` | `"standard" \| "embedded"` | `"standard"` | no | native-input control or dense composite control |
| `appearance` | `"track" \| "block"` | `"track"` | no | visual treatment; orthogonal to `variant`. Omitting it preserves today's anatomy. `block` accepts omitted `orientation` or `orientation="horizontal"` only |
| `direction` | `"ltr" \| "rtl"` | `"ltr"` | no | inline direction. Horizontal geometry mirrors in `rtl`; Left/Down still decrement and Right/Up still increment |
| `visibleLabel` | `string \| null` | `null` | no | visible label for block appearance. Empty text omits the item. Never derived from `ariaLabel` |
| `formatVisibleValue` | `((value: number) => string) \| undefined` | `undefined` | no | **Web targets only** — formats the visible value from the normalized, bounds-guarded, step-snapped number. Default visible text is `String(value)`. Native specs carry the resolved string, not the closure |
| `polarity` | `"unipolar" \| "bipolar"` | `"unipolar"` | no | fill from minimum or from the resolved center |
| `centerValue` | `number \| null` | `null` | no | bipolar fill anchor; defaults to zero when zero is inside the range, otherwise the midpoint |
| `law` | `AudioValueLaw` | `linear` | no | embedded-variant value mapping; standard remains the native linear range path |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | layout and interaction axis |
| `disabled` | `boolean` | `false` | no | disables interaction, applies disabled opacity |
| `ariaLabel` | `string \| null` | `null` | no | accessible name; required when no visible label exists |
| `valueText` | `string \| null` | `null` | no | human-readable value text for assistive technology (aria-valuetext) |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Controlled And Uncontrolled

- bindable value plus callbacks: `value`, `onValueChange`, `onValueCommit`

### Bounds Guard

- When `max <= min`, the effective maximum is clamped to `min + 1` (`safeMax = max <= min ? min + 1 : max`). `safeMax` is used for percentage, the input `max`, and clamping, so a degenerate or inverted range never produces a divide-by-zero or negative span.

### CSS Custom Properties

| Var | Description |
|-----|-------------|
| `--poodle-slider-percent` | computed as `((value - min) / (safeMax - min)) * 100%` (using the bounds-guarded `safeMax`); drives fill width/height |

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | thumb on track at current value position, fill shows completed range |
| focus | thumb receives focus-visible | focus ring shadow on thumb |
| active | thumb is being dragged or keyboard-adjusted | active interaction state |
| disabled | `disabled=true` | reduced opacity via disabled token |

### Component States

- current value state
- active drag/keyboard-adjustment state (native input handles this)

### Behavior Machine

Behavior classification: machine-backed (`sliderTransition` in
`@inflatable-cookie/poodle-core`)

Keyboard and pointer input stay adapter-owned. The machine owns value
normalization and the change/commit split. Web standard track mode delegates
input mechanics to the native range input. Embedded web, block appearance, and
native runtimes convert pointer position into a normalized coordinate owned by
the adapter and interpreted by `sliderControlTransition`. Direction is applied
by the adapter before that coordinate reaches the machine; the machine stays
direction-agnostic.

Release, cancellation, lost capture, disablement, stale-pointer cleanup, and
teardown share one idempotent terminal on the control machine. The first
terminal emits one commit with the latest accepted value; later terminals are
inert. Cancellation does not roll back. Track standard native `change`
semantics are unchanged when `appearance` is omitted or `"track"`.

- Context: `value` (controllable), `min`, `max`, `step`, `disabled`
- Events: `INPUT { raw }` (native input), `COMMIT { raw }` (native change),
  `SET_VALUE` (programmatic)
- Normalization: snap to step from `min`, clamp into `[min, safeMax]` where
  a degenerate range (`max <= min`) widens to `min + 1`; non-positive step
  passes values through unsnapped
- Transitions: `INPUT` sets normalized value, effect
  `emitValueChange(value)`; `COMMIT` sets normalized value, effect
  `emitValueCommit(value)`
- Machinery dependencies: none.

### Embedded Control Variant

`variant="embedded"` is the dense, thumb-light Slider used inside composite
controls. The adapter owns pointer capture and converts the pointer position to
a normalized axis coordinate. The framework-free core owns pointer begin,
move, and end state, law mapping, step constraint, live change, and commit.
Drawing consumes `SliderVisualState`; it never reads machine context.

### Block Appearance

`appearance="block"` is additive and orthogonal to `variant`. It does not fork
value math, paging, or callback ordering.

Horizontal-only admission:

- `appearance="track"` keeps today's horizontal and vertical support.
- `appearance="block"` accepts omitted `orientation` or
  `orientation="horizontal"` only.
- `appearance="block"` with `orientation="vertical"` is invalid in Svelte,
  React, shared Rust composition, and GPUI. Adapters reject it before paint or
  construction. They must not coerce orientation, silently render track
  appearance, or split that rejection by runtime.

Visible content is a separate channel from accessibility copy:

- `visibleLabel` and `formatVisibleValue` never read `ariaLabel` or `valueText`
  and never write those fields.
- Formatter input is the normalized, bounds-guarded, step-snapped value.
- Default visible value text is `String(value)`. Empty label or formatter
  output omits that assigned item.

Inline text is all-or-nothing. Assigned regions are the selected fill (label)
and the remainder (value). For each non-empty item:

```text
available = floor(unoccluded region span - 2 * content inset)
required  = ceil(shaped inline text advance)
fits      iff available >= required
```

Equality fits. Required-minus-one falls back. When any assigned item misses,
no inline text paints and one stable, noninteractive, accessibility-hidden
line renders immediately after the capsule. The fallback never follows the
thumb and does not change on focus or overlap. Content inset is an internal
metric, not a public fit threshold.

Block value feedback is static under architecture 012. Add no motion role.

Every Slider control owns a measurable 44×44 logical-pixel effective target at
every size and density. The visible thumb may be smaller. Proof is the hit
rectangle, not only the painted thumb.

Forced-color roles for block appearance:

| Role | Web system colors | Native role names |
|------|-------------------|-------------------|
| remainder fill/text | `Canvas` / `CanvasText` | canvas / canvas-text |
| selected fill/text | `Highlight` / `HighlightText` | selection / selection-text |
| handle fill/border | `ButtonFace` / `ButtonText` | control / control-border |
| focus | `Highlight` | focus-highlight |
| disabled text/border | `GrayText` | disabled-content |

Unipolar fill starts at zero clamped to the nearest range edge. This makes
positive-only ranges grow from the minimum and negative-only ranges grow from
the maximum. Bipolar fill starts at the resolved
`centerValue` and expands toward the current value. VisualState publishes
`valueNorm`, `centerNorm`, `fillStartNorm`, `fillSpanNorm`, and `fillTone`, so
renderers do not infer polarity, center geometry, or semantic color. A bipolar
value below center publishes `fillTone="negative"`; its default recipe uses the
negative status color. All other values publish `fillTone="positive"`.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | value changes during interaction (input event) | `number` | live updates during drag or keyboard; payload is the clamped, step-snapped value (`clamp(snapToStep(raw, min, step), min, safeMax)`), not the raw input value |
| `onValueCommit` | interaction finishes (change event) | `number` | fires on mouseup/touchend/keyup commit; payload is likewise the clamped, step-snapped value |

## 6. Accessibility

### Semantics

- Role: native `<input type="range">` provides slider role automatically on
  web standard mode; custom web and native controls expose the same role
- Every custom control exposes bounds, current value, optional value text,
  orientation, disabled state, and keyboard behavior on its focusable node
- `aria-label`: from ariaLabel prop; required when no visible label exists
- `aria-valuemin`: from min prop
- `aria-valuemax`: from max prop
- `aria-valuenow`: from value prop
- `aria-valuetext`: from valueText prop when provided
- `aria-orientation`: from orientation on custom controls; native range inputs
  retain their browser-native projection
- `disabled`: native disabled attribute when disabled
- Labeling rules: visible label or programmatic ariaLabel required

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Down` | decrements value by step |
| `Arrow Right/Up` | increments value by step |
| `Home` | moves to minimum value |
| `End` | moves to maximum value |
| `Page Up` | increments by larger step (browser-native, typically step * 10) |
| `Page Down` | decrements by larger step (browser-native, typically step * 10) |
| `Tab` | moves focus to or from the control |

All four arrow keys retain the same value mapping in either orientation and
either `direction`: Left/Down decrement and Right/Up increment. Orientation
and direction change layout, pointer normalization, and accessibility
reporting; they do not change numeric key meaning. Page-key amount remains
browser-owned and is not part of strict cross-runtime parity.

### Focus And Announcement

- focus entry: thumb is focusable via native range input; visible focus ring
  via box-shadow on the thumb pseudo-element
- focus exit: focus ring clears while current value remains visible in the fill
- live-region behavior: none; value changes are announced through native slider
  semantics and optional valueText
- GPUI-native accessibility mapping notes: GPUI must expose slider role,
  current value/min/max, value text, orientation, and keyboard adjustment
  semantics through native accessibility APIs

## 7. Layout

### Sizing

- horizontal: width is 100% of parent; cross-size follows the size table
- vertical: cross-size follows the size table; min-height 10rem; height is 100%
- track thickness follows the size axis: 0.1875rem (`xs`), 0.25rem (`sm`),
  0.375rem (`md`), 0.5rem (`lg`), and 0.625rem (`xl`)
- thumb diameter follows the size table (`1rem` at `md`)
- slider length is parent-owned
- embedded variant is valid inside dense grids; its parent owns both axis
  lengths and the root remains the complete hit target

### Composition

- parent expectations: forms, inspectors, settings, value controls, Field
  wrappers
- child expectations: none
- resizing rules: orientation changes axis without changing value semantics;
  vertical orientation uses CSS transform rotate(-90deg) on the native input

## 8. Token Usage — Exact Values

### Recipe hooks

- `--poodle-recipe-slider-track-fill`
- `--poodle-recipe-slider-fill-fill`
- `--poodle-recipe-slider-fill-negative`
- `--poodle-recipe-slider-track-border`
- `--poodle-recipe-slider-center-fill`
- `--poodle-recipe-slider-control-fill`
- `--poodle-recipe-slider-control-thumb-fill`
- `--poodle-recipe-slider-control-shadow`
- `--poodle-recipe-slider-focus-ring`
- `--poodle-recipe-slider-focus-control-shadow`
- `--poodle-recipe-slider-block-selected-fill`
- `--poodle-recipe-slider-block-selected-text`
- `--poodle-recipe-slider-block-remainder-fill`
- `--poodle-recipe-slider-block-remainder-text`
- `--poodle-recipe-slider-block-handle-fill`
- `--poodle-recipe-slider-block-handle-border`
- `--poodle-recipe-slider-block-focus-ring`
- `--poodle-recipe-slider-block-fallback-text`

### Root `.slider`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-flex` |
| `align-items` | `center` |
| `width` | `100%` |
| `min-height` | `var(--poodle-slider-control-min-height)` (`1.5rem` at `md`) |

### Root vertical `[data-orientation="vertical"]`

| Property | Value |
|----------|-------|
| `width` | `var(--poodle-slider-control-min-height)` |
| `min-width` | `var(--poodle-slider-control-min-height)` |
| `min-height` | `10rem` |
| `height` | `100%` |
| `justify-content` | `center` |

### Root disabled `[data-disabled="true"]`

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Track `.slider__track`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `inset` | `50% 0 0` |
| `height` | `var(--poodle-slider-track-thickness)` (`0.375rem` at `md`) |
| `transform` | `translateY(-50%)` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 88%, transparent)` |

### Track vertical `[data-orientation="vertical"] .slider__track`

| Property | Value |
|----------|-------|
| `inset` | `0 auto 0 50%` |
| `width` | `var(--poodle-slider-track-thickness)` (`0.375rem` at `md`) |
| `height` | `100%` |
| `transform` | `translateX(-50%)` |

### Fill `.slider__fill`

| Property | Value |
|----------|-------|
| `display` | `block` |
| `width` | `var(--poodle-slider-percent)` |
| `height` | `100%` |
| `border-radius` | `inherit` |
| `background` | `var(--poodle-color-accent-base)` |

### Negative bipolar fill `[data-polarity="bipolar"][data-fill-tone="negative"] .slider__fill`

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-recipe-slider-fill-negative, var(--poodle-color-status-danger))` |

### Fill vertical `[data-orientation="vertical"] .slider__fill`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `bottom` | `0` |
| `width` | `100%` |
| `height` | `var(--poodle-slider-percent)` |

### Control `.slider__control`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `z-index` | `1` |
| `width` | `100%` |
| `margin` | `0` |
| `background` | `transparent` |
| `appearance` | `none` |
| `-webkit-appearance` | `none` |

### Control vertical `[data-orientation="vertical"] .slider__control`

| Property | Value |
|----------|-------|
| `width` | `10rem` |
| `transform` | `rotate(-90deg)` |

### Control focus-visible `.slider__control:focus-visible`

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
| `height` | `var(--poodle-slider-track-thickness)` |
| `background` | `transparent` |

### Track pseudo `::-moz-range-track`

| Property | Value |
|----------|-------|
| `height` | `var(--poodle-slider-track-thickness)` |
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
`(thumb diameter - track thickness) / -2`, keeping the thumb centered on the
track at every size.

### Block appearance metrics

Block capsule cross-size follows a size ladder that can hold inline label
text. Visible thumb diameter is smaller than the track-appearance thumb. The
effective hit target is 44×44 logical pixels at every size and density and is
not a public metric.

| Size | capsule min-height | visible thumb |
|------|--------------------|---------------|
| `xs` | `1.75rem` | `0.375rem` |
| `sm` | `1.875rem` | `0.4375rem` |
| `md` | `2rem` | `0.5rem` |
| `lg` | `2.25rem` | `0.5625rem` |
| `xl` | `2.5rem` | `0.625rem` |

Content inset used by the fit law is an internal `0.5rem` on each inline edge
of an assigned region. Do not expose it.

### Density And Vertical Padding

- Density must not alter the slider's vertical padding, `min-height`, or thumb position — those are size-axis properties. Density on a single-thumb slider has no compositional vertical effect; it carries no contract-mandated padding change.
- Known Svelte deviation: the Svelte target writes `padding: 0.25rem 0` (compact) and `padding: 0.75rem 0` (comfortable) on the root, i.e. density-driven vertical padding. This violates size/density orthogonality and is a Svelte bug, not the contract rule. Rust targets must follow this contract (no density vertical padding), not replicate the Svelte deviation.

### Root block `[data-appearance="block"]`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `width` | `100%` |
| `min-height` | capsule size table (`2rem` at `md`) |
| `dir` | from `direction` |

Block remainder fill uses `--poodle-recipe-slider-block-remainder-fill` falling back to a surface mix. Selected fill uses `--poodle-recipe-slider-block-selected-fill` falling back to accent. Inline label/value use the selected/remainder text hooks. The visible thumb uses the handle fill/border hooks.

## 9. Svelte Notes

- Uses a native `<input type="range">` overlaid on a custom track/fill for
  visual rendering
- The CSS custom property `--poodle-slider-percent` is computed reactively:
  `((value - min) / (max - min)) * 100 + '%'` and set as an inline style on
  the root element
- The native range input is styled with `appearance: none` and transparent
  track pseudo-elements so the custom track shows through
- Thumb is styled via both `::-webkit-slider-thumb` and `::-moz-range-thumb`
  for cross-browser support
- Focus ring uses a compound `box-shadow` on the thumb pseudo-element rather
  than outline, combining the focus ring with the existing drop shadow
- Vertical orientation uses `transform: rotate(-90deg)` on the native input
- `onValueChange` fires on the `input` event (live during drag); `onValueCommit`
  fires on the `change` event (on release)
- `data-orientation`, `data-disabled`, `data-size`, `data-density`,
  `data-appearance`, and `data-direction` attributes on root drive layout and
  state styling
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- Block appearance sets `dir` from `direction` and uses logical inline
  geometry. Vertical block input throws before paint.

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::slider`
- GPUI implementation must intentionally expose control focus, keyboard
  adjustments (all arrows, Home, End), and current value semantics through
  the shared node accessibility intent
- slider role, value/min/max, and value text must be carried by the shared
  node accessibility intent; native assistive-technology projection remains a
  separately measured backend evidence level
- orientation must affect layout, pointer normalization, and accessibility
  reporting; arrow-value mapping stays fixed across orientations and directions
- pointer/gesture drag behavior is platform-specific but must produce the same
  value snapping and commit semantics
- vertical orientation must be implemented natively rather than via CSS rotation
- `appearance="block"` with `orientation="vertical"` panics at construction,
  matching the web throw, before any node is returned
- native specs carry resolved `visible_label` and `visible_value_text`
  strings, never formatter closures
- GPUI block metadata and hit bounds are not mounted assistive-technology proof

## 10a. Jetstream Notes

Jetstream backend admission is deferred. Its shared-Rust caller stays
compile-compatible with the renderer-neutral Slider API, but this contract
does not require Jetstream execution or evidence while that deferral stands.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value/min/max/step semantics match
- [ ] keyboard adjustment semantics match (arrows, Home, End)
- [ ] onValueChange fires during interaction, onValueCommit fires on release
- [ ] slider accessibility exposure matches (role, value, min, max, valuetext)
- [ ] orientation affects layout, pointer axis, and accessibility reporting
- [ ] disabled behavior matches

### Tier 2: Visual Parity

- [ ] all five sizes visually match (height, padding, font-size per size table)
- [ ] track thickness follows the `xs`–`xl` size table and border-radius
  remains 999px
- [ ] thumb diameter follows the `xs`–`xl` size table
- [ ] thumb border, background (elevated), and box-shadow match
- [ ] fill uses accent-base color
- [ ] track background uses semi-transparent surface color-mix
- [ ] focus ring uses compound box-shadow with focusRing 32% opacity
- [ ] disabled opacity uses state-opacity-disabled
- [ ] vertical layout dimensions match (1.5rem width, 10rem min-height)

### Tier 3: Implementation Freedom

- [ ] native range input vs GPUI custom control internals
- [ ] CSS rotation for vertical vs native vertical implementation
- [ ] webkit/moz thumb pseudo-elements are Svelte-specific
- [ ] pointer/gesture drag feel is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| pointer drag feel may differ slightly | input-engine internals are runtime-specific | allowed | keep keyboard and value semantics strict |
| vertical via CSS rotation vs native | Svelte uses rotate(-90deg); GPUI implements natively | allowed | same visual and interaction result required |
| webkit/moz thumb pseudo-elements | browser-specific CSS selectors | allowed | GPUI renders thumb directly |
| color-mix formulas | GPUI must achieve same visual result by any means | allowed | verify visual parity |
| Page Up/Down increment amount | native range inputs keep browser-owned paging behavior | allowed | strict parity covers arrows, Home, and End |
| vertical block appearance | native RangeSlider axis geometry is still deferred; block stays horizontal in every runtime | allowed | later all-runtime migration after mounted native axis proof |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default

One basic slider:

| Label | Min | Max | Value | Step |
|-------|-----|-----|-------|------|
| Volume | 0 | 100 | 65 | 1 |

### With step

One slider with explicit step:

| Label | Min | Max | Value | Step |
|-------|-----|-----|-------|------|
| Opacity | 0 | 100 | 100 | 10 |

### Disabled

One disabled slider:

| Label | Min | Max | Value | Props |
|-------|-----|-----|-------|-------|
| Disabled | 0 | 100 | 40 | `disabled: true` |

### Block appearance

One horizontal block slider with an explicit visible label. Do not render a
vertical block specimen.

| Label | Min | Max | Value | Props |
|-------|-----|-----|-------|-------|
| Blur | 0 | 100 | 67 | `appearance: "block"`, `visibleLabel: "Blur"` |

### Embedded controls

One unipolar `0..1` control and one bipolar `-1..1` control. Both use
`variant="embedded"`, expose size and density axes, and render only from
`SliderVisualState`.

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings, inspectors, value controls, Field-wrapped
  form inputs
- future follow-up: coordinate with `RangeSlider` for dual-thumb variant;
  consider value-label wrapper once forms/composites deepen
