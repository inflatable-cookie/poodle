# Switch

Status: detailed contract
Updated: 2026-03-15

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
| `isChecked` | `boolean \| null` | `null` | no | controlled on/off state; when non-null, component is controlled |
| `defaultChecked` | `boolean` | `false` | no | uncontrolled initial state |
| `isDisabled` | `boolean` | `false` | no | disables interaction, applies disabled opacity |
| `isReadOnly` | `boolean` | `false` | no | allows focus and reading but reverts any change attempt |
| `label` | `string \| null` | `null` | no | visible label text |
| `ariaLabel` | `string \| null` | `null` | no | accessible name; required when no visible label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target id |
| `name` | `string \| undefined` | `undefined` | no | form submission name |

### Controlled And Uncontrolled

- controlled: `isChecked` (non-null) plus `checkedChange` event handler
- uncontrolled: `defaultChecked` sets the initial state; component owns its own
  state thereafter

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| off | default | thumb at left position, track has default border and muted background |
| on | `isChecked=true` or user toggle | thumb slides right with accent color, track border and background shift to accent tints |
| focus | native input receives focus-visible | focus ring outline on track |
| disabled | `isDisabled=true` | reduced opacity, cursor not-allowed |
| readOnly | `isReadOnly=true` | default cursor, change reverted on toggle attempt |

### Component States

- internal checked state (uncontrolled mode)

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `checkedChange` | user toggles the switch | `{ checked: boolean }` | suppressed when disabled; when readOnly, the native change is reverted so no event fires |

## 6. Accessibility

### Semantics

- Role: `role="switch"` on the hidden checkbox input
- `id`: from prop, used for external `<label for>` association
- `aria-label`: from ariaLabel prop; required when no visible label exists
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

- track is fixed at 2.125rem wide by 1.25rem tall
- thumb is fixed at 0.875rem diameter
- thumb travels 0.875rem horizontally between off and on positions
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
| `gap` | `var(--flint-space-inline-sm)` |
| `color` | `var(--flint-color-text-primary)` |
| `cursor` | `pointer` |

### Root disabled `[data-disabled="true"]`

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--flint-state-opacity-disabled)` |

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
| `width` | `2.125rem` |
| `height` | `1.25rem` |
| `padding` | `0.125rem` |
| `border` | `0.0625rem solid var(--flint-color-border-default)` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--flint-color-background-surface) 86%, transparent)` |
| `box-shadow` | `inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent)` |
| `transition` | `background, border-color, box-shadow` at `var(--flint-motion-duration-interaction)` with `var(--flint-motion-easing-standard)` |

### Track checked `:checked + .switch__track`

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--flint-color-accent-base) 58%, var(--flint-color-border-default))` |
| `background` | `color-mix(in srgb, var(--flint-color-accent-base) 24%, var(--flint-color-background-surface))` |

### Track focus `:focus-visible + .switch__track`

| Property | Value |
|----------|-------|
| `outline` | `var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Thumb `.switch__thumb`

| Property | Value |
|----------|-------|
| `width` | `0.875rem` |
| `height` | `0.875rem` |
| `border-radius` | `999px` |
| `background` | `var(--flint-color-text-primary)` |
| `box-shadow` | `0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent)` |
| `transform` | `translateX(0)` |
| `transition` | `transform, background` at `var(--flint-motion-duration-interaction)` with `var(--flint-motion-easing-standard)` |

### Thumb checked `:checked + .switch__track .switch__thumb`

| Property | Value |
|----------|-------|
| `background` | `var(--flint-color-accent-base)` |
| `transform` | `translateX(0.875rem)` |

### Label `.switch__label`

| Property | Value |
|----------|-------|
| `font-family` | `var(--flint-typography-label-family)` |
| `font-size` | `var(--flint-typography-label-size)` |
| `font-weight` | `var(--flint-typography-label-weight)` |
| `line-height` | `var(--flint-typography-label-lineHeight)` |

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
- `data-disabled` and `data-read-only` attributes on root drive state styling
- `color-mix` formulas create the semi-transparent track background and accent
  tints for the checked state
- Thumb `translateX(0.875rem)` slides the thumb from the off to on position

## 10. GPUI Notes

- expected crate/module surface: `flint_gpui::primitives::switch`
- GPUI implementation must expose switch role (not generic button or checkbox)
  with checked state through the native accessibility tree
- keyboard toggle via Space and Enter must be explicitly handled
- readonly behavior: reject toggle attempts while maintaining focusability
- thumb animation between off/on positions should use platform-appropriate
  motion matching `motion-duration-interaction`
- color-mix formulas for track background and border must achieve the same
  visual result by any means available in the rendering engine

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] switch role and checked semantics match across platforms
- [ ] keyboard toggle behavior (Space, Enter) matches
- [ ] readonly behavior (revert on change) matches
- [ ] disabled behavior (opacity, cursor, suppressed interaction) matches
- [ ] accessible name exposure matches

### Tier 2: Visual Parity

- [ ] track sizing (2.125rem x 1.25rem) matches
- [ ] thumb sizing (0.875rem diameter) matches
- [ ] thumb travel distance (0.875rem translateX) matches
- [ ] unchecked track uses border-default and semi-transparent background
- [ ] checked track uses accent-base tinted border and background
- [ ] thumb color transitions from text-primary to accent-base on check
- [ ] focus ring uses accent-focusRing with correct offset (0.125rem)
- [ ] thumb box-shadow matches (0.125rem 0.5rem black 18%)
- [ ] label typography uses label token family
- [ ] disabled opacity uses state-opacity-disabled
- [ ] gap between track and label uses space-inline-sm

### Tier 3: Implementation Freedom

- [ ] native checkbox with role="switch" vs GPUI switch control internals
- [ ] CSS adjacent sibling selectors are Svelte-specific
- [ ] color-mix formulas may be achieved differently in GPUI
- [ ] transition/animation timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
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
