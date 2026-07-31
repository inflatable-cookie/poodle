# Slider

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Slider`
- Layer: `foundation`
- Summary: a single-value continuous or stepped range control built on a native
  range input overlaid on a custom track with fill visualization; supports
  horizontal and vertical orientation via CSS custom property for fill width
- In scope: current value, min/max bounds, step behavior, keyboard and pointer
  adjustment, value commit semantics, horizontal and vertical orientation
- Out of scope: dual-thumb range editing (see RangeSlider), knob/fader
  semantics, tick marks, value labels

## 2. Anatomy

```text
[Root .slider]  <div>
  ├── [Track .slider__track]  <span>
  │     └── [Fill .slider__fill]  <span>
  └── [Control .slider__control]  <input type="range">
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | slider host with relative positioning | sizing, disabled opacity |
| Track | yes | full value range background bar | background, radius |
| Fill | yes | completed value span driven by CSS custom property | accent color, radius |
| Control | yes | native range input overlaid on the track for interaction | thumb styling, focus ring, appearance reset |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `number` | `0` | no | current value |
| `min` | `number` | `0` | no | lower bound |
| `max` | `number` | `100` | no | upper bound |
| `step` | `number` | `1` | no | increment size |
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
`@poodle/headless`)

Keyboard and pointer interaction come from the native range input; the
machine owns value normalization and the change/commit split.

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

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | value changes during interaction (input event) | `number` | live updates during drag or keyboard; payload is the clamped, step-snapped value (`clamp(snapToStep(raw, min, step), min, safeMax)`), not the raw input value |
| `onValueCommit` | interaction finishes (change event) | `number` | fires on mouseup/touchend/keyup commit; payload is likewise the clamped, step-snapped value |

## 6. Accessibility

### Semantics

- Role: native `<input type="range">` provides slider role automatically
- `aria-label`: from ariaLabel prop; required when no visible label exists
- `aria-valuemin`: from min prop
- `aria-valuemax`: from max prop
- `aria-valuenow`: from value prop
- `aria-valuetext`: from valueText prop when provided
- `aria-orientation`: NOT currently set on the range input; orientation is conveyed via `data-orientation` on the root element only
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

- horizontal: width is 100% of parent; min-height 1.5rem for touch target
- vertical: width is 1.5rem; min-height 10rem; height is 100% of parent
- track thickness is 0.375rem
- thumb is 1rem diameter
- slider length is parent-owned

### Composition

- parent expectations: forms, inspectors, settings, value controls, Field
  wrappers
- child expectations: none
- resizing rules: orientation changes axis without changing value semantics;
  vertical orientation uses CSS transform rotate(-90deg) on the native input

## 8. Token Usage — Exact Values

### Root `.slider`

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

### Track `.slider__track`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `inset` | `50% 0 0` |
| `height` | `0.375rem` |
| `transform` | `translateY(-50%)` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 88%, transparent)` |

### Track vertical `[data-orientation="vertical"] .slider__track`

| Property | Value |
|----------|-------|
| `inset` | `0 auto 0 50%` |
| `width` | `0.375rem` |
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
| `height` | `0.375rem` |
| `background` | `transparent` |

### Track pseudo `::-moz-range-track`

| Property | Value |
|----------|-------|
| `height` | `0.375rem` |
| `background` | `transparent` |

### Size adjustments

| Size | min-height | thumb diameter | thumb margin-top |
|------|------------|----------------|------------------|
| `xs` | `1.25rem` | `0.75rem` | `-0.1875rem` |
| `sm` | `1.375rem` | `0.875rem` | `-0.25rem` |
| `md` | `1.5rem` | `1rem` | `-0.3125rem` |
| `lg` | _(base)_ | `1.125rem` | `-0.375rem` |
| `xl` | _(base)_ | `1.25rem` | `-0.4375rem` |

### Density And Vertical Padding

- Density must not alter the slider's vertical padding, `min-height`, or thumb position — those are size-axis properties. Density on a single-thumb slider has no compositional vertical effect; it carries no contract-mandated padding change.
- Known Svelte deviation: the Svelte target writes `padding: 0.25rem 0` (compact) and `padding: 0.75rem 0` (comfortable) on the root, i.e. density-driven vertical padding. This violates size/density orthogonality and is a Svelte bug, not the contract rule. Rust targets must follow this contract (no density vertical padding), not replicate the Svelte deviation.

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
- `data-orientation`, `data-disabled`, `data-size`, and `data-density` attributes on root drive
  layout and state styling
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::slider`
- GPUI implementation must intentionally expose thumb focus, keyboard
  adjustments (arrow keys, Home, End), and current value semantics through
  native accessibility APIs
- slider role, value/min/max/step, and valuetext must be reflected in the
  accessibility tree
- orientation must affect both keyboard navigation direction and accessibility
  tree reporting
- pointer/gesture drag behavior is platform-specific but must produce the same
  value snapping and commit semantics
- vertical orientation must be implemented natively rather than via CSS rotation

## 10a. Jetstream Notes

- `Slider::from_spec(spec, theme).on_change(...).on_value_commit(...)`, driven
  by a real drag.
- The value comes from the drag's per-frame **delta**, not its position. A
  handler is built before layout runs, so it cannot know where its track ended
  up; a pixel delta over a known track width is a value delta, and that stays
  correct when the track is nested inside something scrolled or offset.
- Snapping and clamping come from `slider_transition` in the shared headless
  core — the same machine the web target drives — rather than being re-derived.
- Every segment of the track carries the handler, not just their container:
  unlike clicks, drags do **not** bubble to the nearest handler above, so the
  gesture would be lost on whichever segment the pointer happened to land on.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value/min/max/step semantics match
- [ ] keyboard adjustment semantics match (arrows, Home, End)
- [ ] onValueChange fires during interaction, onValueCommit fires on release
- [ ] slider accessibility exposure matches (role, value, min, max, valuetext)
- [ ] orientation affects keyboard navigation axis
- [ ] disabled behavior matches

### Tier 2: Visual Parity

- [ ] all five sizes visually match (height, padding, font-size per size table)
- [ ] track thickness (0.375rem) and border-radius (999px) match
- [ ] thumb sizing (1rem diameter) matches
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

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings, inspectors, value controls, Field-wrapped
  form inputs
- future follow-up: coordinate with `RangeSlider` for dual-thumb variant;
  consider value-label wrapper once forms/composites deepen
