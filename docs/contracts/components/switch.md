# Switch

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Switch`
- Layer: `foundation`
- Summary: a binary on/off control with switch semantics built on a hidden
  native checkbox input with `role="switch"`; features a sliding thumb within a
  track with label association
- In scope: checked (on/off) state, optional label, disabled and readonly
  semantics, controlled and uncontrolled value models
- Out of scope: mixed-state semantics (see Checkbox), tri-state membership
  logic (see TriStateSwitch)

## 2. Anatomy

```text
[Root .switch]  <label>
  ├── [Control .switch__control]  <input type="checkbox" role="switch"> (visually hidden)
  ├── [Track .switch__track]  <span>
  │     └── [Thumb .switch__thumb]  <span>
  └── [Label .switch__label]  <span> (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | label element wrapping the entire control | spacing, cursor |
| Control | yes | hidden native checkbox with role="switch" for a11y | visually hidden |
| Track | yes | visible on/off track housing the thumb | background, border, shadow, transition |
| Thumb | yes | sliding indicator circle | background, shadow, transform, transition |
| Label | no | visible text label | typography, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string \| undefined` | `undefined` | no | element id for external label association |
| `checked` | `boolean \| undefined` | `undefined` | no | bindable on/off state; leave undefined for uncontrolled mode |
| `defaultChecked` | `boolean` | `false` | no | uncontrolled initial state |
| `disabled` | `boolean` | `false` | no | disables interaction, applies disabled opacity |
| `readOnly` | `boolean` | `false` | no | allows focus and reading but reverts any change attempt |
| `label` | `string \| null` | `null` | no | visible single trailing label text |
| `leftLabel` | `string \| null` | `null` | no | optional off-state label shown to the left of the track |
| `rightLabel` | `string \| null` | `null` | no | optional on-state label shown to the right of the track |
| `ariaLabel` | `string \| null` | `null` | no | accessible name; required when no visible label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target id |
| `name` | `string \| undefined` | `undefined` | no | form submission name |
| `offColor` | `string \| null` | `null` | no | optional off-state accent override used for the thumb and muted track tint |
| `onColor` | `string \| null` | `null` | no | optional on-state accent override used for the thumb and active track tint |
| `leftTone` | `"default" \| "primary" \| "success" \| "warning" \| "danger"` | `"default"` | no | semantic off-state tone used when `offColor` is not set |
| `rightTone` | `"default" \| "primary" \| "success" \| "warning" \| "danger"` | `"primary"` | no | semantic on-state tone used when `onColor` is not set |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Controlled And Uncontrolled

- controlled: bindable `checked` plus `onCheckedChange`
- uncontrolled: `defaultChecked` sets the initial state; component owns its own
  state thereafter
- visual overrides: `offColor` and `onColor` take precedence over `leftTone`
  and `rightTone` and map to CSS custom properties on the root label
- dual labels: `leftLabel` and `rightLabel` provide the common settings-toggle
  layout without requiring extra caller markup

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| off | default | thumb at left position, track has default border and muted background |
| on | `checked=true` or user toggle | thumb slides right with accent color, track border and background shift to accent tints |
| semantic tones | `leftTone` or `rightTone` set | off/on track and active side label use status/accent theme colors |
| custom colors | `offColor` or `onColor` set | off/on track, thumb, and active side label derive from the provided local override colors instead of theme defaults |
| focus | native input receives focus-visible | focus ring outline on track |
| disabled | `disabled=true` | reduced opacity, cursor not-allowed |
| readOnly | `readOnly=true` | default cursor, change reverted on toggle attempt |

### Component States

- internal checked state (uncontrolled mode)

### Behavior Machine

Behavior classification: machine-backed (`switchTransition` in
`@inflatable-cookie/poodle-core`)

Checkbox semantics without the mixed state: single implicit state, value in
context.

- Context: `checked` (controllable), `disabled`, `readOnly`
- Events: `TOGGLE { nextChecked }` (native change), `SET_CHECKED` (programmatic)
- Transitions: `TOGGLE` — disabled: inert; readOnly: effect
  `revertNativeChecked`, no callback; otherwise set `checked`, effect
  `emitCheckedChange(checked)`. `SET_CHECKED` updates without callback.
- Machinery dependencies: none (native input provides keyboard/focus).

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onCheckedChange` | user toggles the switch | `boolean` | suppressed when disabled; when readOnly, the native change is reverted so no callback fires |

## 6. Accessibility

### Semantics

- Role: `role="switch"` on the hidden checkbox input
- `id`: from prop, used for external `<label for>` association
- `aria-label`: from `label`, `ariaLabel`, or a fallback composed from `leftLabel`/`rightLabel`
- `aria-describedby`: from describedBy prop
- `aria-checked`: reflects current on/off state
- `aria-readonly`: set when isReadOnly
- `disabled`: native disabled attribute when isDisabled
- Labeling rules: visible label or programmatic ariaLabel required; the root
  `<label>` element wraps the control

### Keyboard

| Key | Behavior |
|-----|----------|
| `Space` | toggles switch when interactive |
| `Enter` | toggles switch when interactive (parity with button-like switch semantics) |
| `Tab` | moves focus into or past the switch |

### Focus And Announcement

- focus entry: visible focus ring on the track via `:focus-visible` on the
  native input + adjacent sibling selector
- focus exit: ring clears while on/off state remains visible
- live-region behavior: none; checked state is announced through switch role
  semantics
- GPUI-native accessibility mapping notes: GPUI must expose switch role,
  checked state, label, and readonly/disabled state explicitly through the
  native accessibility tree

## 7. Layout

### Sizing

- the base `.switch__track` uses a `--switch-unit: var(--poodle-size-icon-md)`
  `calc` chain (`2.25rem × 1.375rem` track at md), but every rendered switch
  carries a `data-size` class that overrides track/thumb/travel with flat rem
  literals — see the Size adjustments table in §8 for the authoritative values
- at the default `md` size: track is `2.25rem` wide by `1.375rem` tall, thumb is
  `1.125rem` diameter, thumb travels `0.875rem` horizontally between off/on
- label spacing remains stable regardless of state

### Composition

- parent expectations: settings rows, filter bars, shell toggles, Field wrappers
- child expectations: optional visible label text
- resizing rules: track/thumb remain fixed proportionally; label can flex with
  parent

## 8. Token Usage — Exact Values

### Root `.switch`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `--poodle-switch-off-color` | `var(--poodle-color-text-primary)` |
| `--poodle-switch-on-color` | `var(--poodle-color-accent-base)` |
| `--poodle-switch-off-track` | `color-mix(in srgb, var(--poodle-switch-off-color) 18%, var(--poodle-color-background-surface))` |
| `--poodle-switch-on-track` | `color-mix(in srgb, var(--poodle-switch-on-color) 24%, var(--poodle-color-background-surface))` |
| `--poodle-switch-off-thumb` | `var(--poodle-switch-off-color)` |
| `--poodle-switch-on-thumb` | `var(--poodle-switch-on-color)` |
| `--poodle-switch-off-border` | `var(--poodle-color-border-default)` |
| `--poodle-switch-on-border` | `color-mix(in srgb, var(--poodle-switch-on-thumb) 58%, var(--poodle-color-border-default))` |

### Root disabled `[data-disabled="true"]`

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Root readOnly `[data-read-only="true"]`

| Property | Value |
|----------|-------|
| `cursor` | `default` |

### Control `.switch__control`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `opacity` | `0` |
| `pointer-events` | `none` |

### Track `.switch__track`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `width` | `2.25rem` (md; per-size override — see Size adjustments) |
| `height` | `1.375rem` (md; per-size override) |
| `padding` | `0.125rem` |
| `border` | `0.0625rem solid var(--poodle-switch-off-border)` |
| `border-radius` | `999px` |
| `background` | `var(--poodle-switch-off-track)` |
| `box-shadow` | `inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent)` |
| `transition` | `background, border-color, box-shadow` at `var(--poodle-motion-duration-interaction)` with `var(--poodle-motion-easing-standard)` |

### Track checked `:checked + .switch__track`

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-switch-on-border)` |
| `background` | `var(--poodle-switch-on-track)` |

### Track focus `:focus-visible + .switch__track`

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Thumb `.switch__thumb`

| Property | Value |
|----------|-------|
| `width` | `1.125rem` (md; per-size override — see Size adjustments) |
| `height` | `1.125rem` (md; per-size override) |
| `border-radius` | `999px` |
| `background` | `var(--poodle-switch-off-thumb)` |
| `box-shadow` | `0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent)` |
| `transform` | `translateX(0)` |
| `transition` | `transform, background` at `var(--poodle-motion-duration-interaction)` with `var(--poodle-motion-easing-standard)` |

### Thumb checked `:checked + .switch__track .switch__thumb`

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-switch-on-thumb)` |
| `transform` | `translateX(0.875rem)` |

### Label `.switch__label`

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `line-height` | `var(--poodle-typography-label-lineHeight)` |

### Label color rules

| Selector | `color` |
|----------|---------|
| single label (not `[data-dual-label]`) `.switch__label` | `var(--poodle-color-text-primary)` |
| dual-label `.switch__label--left`, `.switch__label--right` (inactive) | `var(--poodle-color-text-muted)` |
| `:not(:checked) ~ .switch__label--left` (active off side) | `var(--poodle-switch-off-color)` |
| `:checked ~ .switch__label--right` (active on side) | `var(--poodle-switch-on-color)` |

In dual-label mode both side labels rest at `text-muted`; the active side
(left when off, right when on) re-tints to the off/on color. The left/right
labels also carry a `color` transition (`motion-duration-interaction`,
`motion-easing-standard`).

### Density variants

| Selector | `gap` |
|----------|-------|
| `[data-density="compact"]` | `0.25rem` |
| `[data-density="default"]` (base) | `var(--poodle-space-inline-sm)` |
| `[data-density="comfortable"]` | `var(--poodle-space-inline-md)` |

### Label size variants

| Size | Label font-size |
|------|----------------|
| `xs` | `0.75rem` |
| `sm` | `0.75rem` |
| `md` | `0.8125rem` |
| `lg` | `0.875rem` |
| `xl` | `0.875rem` |

Base `.switch__label` font-size (pre-size-class) is
`var(--poodle-typography-label-size)`; the md class resolves it to `0.8125rem`.

### Size adjustments

Per-size classes override the base track/thumb/travel with flat rem literals.
Track padding is `0.125rem` at every size.

| Size | track width | track height | track padding | thumb size | thumb travel |
|------|-------------|--------------|---------------|------------|--------------|
| `xs` | `1.75rem` | `1rem` | `0.125rem` | `0.75rem` | `0.75rem` |
| `sm` | `2rem` | `1.125rem` | `0.125rem` | `0.875rem` | `0.875rem` |
| `md` | `2.25rem` | `1.375rem` | `0.125rem` | `1.125rem` | `0.875rem` |
| `lg` | `2.75rem` | `1.625rem` | `0.125rem` | `1.375rem` | `1.125rem` |
| `xl` | `3rem` | `1.75rem` | `0.125rem` | `1.5rem` | `1.25rem` |

## 9. Svelte Notes

- Uses a hidden native `<input type="checkbox" role="switch">` for accessibility
  and form semantics, with a custom track/thumb sibling for visual rendering
- The root element is a `<label>` to associate the click target with the hidden
  input
- ReadOnly behavior: listen for the `change` event on the native input and
  immediately revert the checked state back to the controlled value, preventing
  the toggle from taking effect
- Adjacent sibling CSS selectors (`:checked +`, `:focus-visible +`) connect the
  hidden input state to the visible track and thumb
- The `~` general sibling combinator is used for checked-state styling of labels
  (e.g., `:checked ~ .switch__label`) since the label is not an immediate sibling
  of the hidden input
- `data-disabled` and `data-read-only` attributes on root drive state styling
- `color-mix` formulas create the semi-transparent track background and accent
  tints for the checked state
- Thumb `translateX(0.875rem)` slides the thumb from the off to on position
- Emits `data-size` on root element reflecting the resolved size
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::switch`
- GPUI implementation must expose switch role (not generic button or checkbox)
  with checked state through the native accessibility tree
- keyboard toggle via Space and Enter must be explicitly handled
- readonly behavior: reject toggle attempts while maintaining focusability
- thumb animation between off/on positions should use platform-appropriate
  motion matching `motion-duration-interaction`
- color-mix formulas for track background and border must achieve the same
  visual result by any means available in the rendering engine

## 10a. Jetstream Notes

- `Switch::from_spec(spec, theme).on_change(...)`, with the same
  next-state payload as `Checkbox`.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] switch role and checked semantics match across platforms
- [ ] keyboard toggle behavior (Space, Enter) matches
- [ ] readonly behavior (revert on change) matches
- [ ] disabled behavior (opacity, cursor, suppressed interaction) matches
- [ ] accessible name exposure matches

### Tier 2: Visual Parity

- [ ] track sizing (md `2.25rem x 1.375rem`; per-size table) matches
- [ ] thumb sizing (md `1.125rem` diameter; per-size table) matches
- [ ] thumb travel distance (md `0.875rem` translateX; per-size table) matches
- [ ] unchecked track uses border-default and semi-transparent background
- [ ] checked track uses accent-base tinted border and background
- [ ] thumb color transitions from text-primary to accent-base on check
- [ ] focus ring uses accent-focusRing with correct offset (0.125rem)
- [ ] thumb box-shadow matches (0.125rem 0.5rem black 18%)
- [ ] label typography uses label token family
- [ ] disabled opacity uses state-opacity-disabled
- [ ] gap between track and label uses space-inline-sm
- [ ] all five sizes visually match (track, thumb, and travel per size table)

### Tier 3: Implementation Freedom

- [ ] native checkbox with role="switch" vs GPUI switch control internals
- [ ] CSS adjacent sibling selectors are Svelte-specific
- [ ] color-mix formulas may be achieved differently in GPUI
- [ ] transition/animation timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Read-only fires no change on the natives | as Checkbox: read-only stays focusable and full strength but cannot change | accepted (by design) | none |
| thumb animation details may differ | motion internals are runtime-specific | allowed | keep on/off semantics strict |
| color-mix formulas for track tints | GPUI must achieve same visual result by any means | allowed | verify visual parity |
| CSS adjacent sibling selectors | Svelte-specific DOM pattern | allowed | GPUI uses state-driven rendering |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default

Three basic switches in a vertical stack with 12px gap:

| Label | Initial State | Notes |
|-------|--------------|-------|
| Dark mode | off | interactive |
| Auto-save drafts | off | interactive |
| Compact view | off | interactive |

### States

Three state examples in a vertical stack with 12px gap:

| Label | State | Props |
|-------|-------|-------|
| Disabled off | off | `isDisabled: true` |
| Disabled on | on | `isDisabled: true` |
| Read-only on | on | `isReadOnly: true` |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings panels, shell toggles, filter bars,
  Field-wrapped form inputs
- future follow-up: coordinate with `TriStateSwitch` for ternary membership
  semantics when that contract is detailed
