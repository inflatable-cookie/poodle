# Checkbox

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Checkbox`
- Layer: `foundation`
- Summary: a binary or mixed-state selection control built on a hidden native
  checkbox input with a custom visual indicator; supports checked, unchecked,
  and indeterminate states with label association
- In scope: checked, unchecked, mixed (indeterminate) state, label association,
  disabled and readonly semantics, controlled and uncontrolled value models
- Out of scope: mutually exclusive group selection (see RadioGroup), range
  selection logic, checkbox tree/parent-child cascade

## 2. Anatomy

```text
[Root .checkbox]  <label>
  ├── [Control .checkbox__control]  <input type="checkbox"> (visually hidden)
  ├── [Indicator .checkbox__indicator]  <span>
  │     └── [Mark .checkbox__mark]  <span> (conditional)
  └── [Label .checkbox__label]  <span> (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | label element wrapping the entire control | spacing, cursor |
| Control | yes | hidden native checkbox input for form and a11y semantics | visually hidden |
| Indicator | yes | visible checkbox boundary and checked fill | border, background, radius, focus ring |
| Mark | no | check or mixed glyph, visible only when checked/indeterminate | icon color, sizing |
| Label | no | visible option label | typography, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string \| undefined` | `undefined` | no | element id for external label association |
| `isChecked` | `boolean` | `false` | no | controlled checked state; when provided, component is controlled |
| `defaultChecked` | `boolean` | `false` | no | uncontrolled initial checked state |
| `isMixed` | `boolean` | `false` | no | sets indeterminate visual and assistive state; controlled only |
| `isDisabled` | `boolean` | `false` | no | disables interaction, applies disabled opacity |
| `isReadOnly` | `boolean` | `false` | no | allows focus and reading but reverts any change attempt |
| `label` | `string \| null` | `null` | no | visible label text |
| `ariaLabel` | `string \| null` | `null` | no | accessible name; required when no visible label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target id |

### Controlled And Uncontrolled

- controlled: `isChecked` plus `checkedChange` event handler
- uncontrolled: `defaultChecked` sets the initial state; component owns its own
  state thereafter
- `isMixed` is controlled-only and sets the indeterminate property on the native
  input via JavaScript; the next user toggle resolves the state to
  checked/unchecked

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| unchecked | default | empty indicator with default border |
| checked | `isChecked=true` or user toggle | check mark visible, indicator filled with accent |
| mixed | `isMixed=true` | mixed glyph visible, indicator filled with accent |
| focus | native input receives focus-visible | focus ring outline on indicator |
| disabled | `isDisabled=true` | reduced opacity, cursor not-allowed |
| readOnly | `isReadOnly=true` | default cursor, change reverted on toggle attempt |

### Component States

- internal checked state (uncontrolled mode)
- indeterminate property set via JS on native input

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `checkedChange` | user toggles the control | `{ checked: boolean }` | suppressed when disabled; when readOnly, the native change is reverted so no event fires; mixed state resolves to checked on first toggle |

## 6. Accessibility

### Semantics

- Role: native `<input type="checkbox">` element
- `id`: from prop, used for external `<label for>` association
- `aria-label`: from ariaLabel prop; required when no visible label exists
- `aria-describedby`: from describedBy prop
- `aria-checked`: `"mixed"` when isMixed is true (set via indeterminate property)
- `disabled`: native disabled attribute when isDisabled
- `aria-readonly`: set when isReadOnly
- Labeling rules: visible label or programmatic ariaLabel required; the root
  `<label>` element wraps the control so clicking the label toggles the checkbox

### Keyboard

| Key | Behavior |
|-----|----------|
| `Space` | toggles checked state when interactive |
| `Tab` | moves focus into or past the control |

### Focus And Announcement

- focus entry: visible focus ring on the indicator via `:focus-visible` on the
  native input + adjacent sibling selector
- focus exit: ring clears while checked/unchecked state remains visible
- live-region behavior: none; checked and mixed state changes are announced
  through native checkbox semantics
- GPUI-native accessibility mapping notes: GPUI must expose checkbox role,
  accessible name, checked/mixed state, and readonly/disabled state through the
  native accessibility tree

## 7. Layout

### Sizing

- indicator is fixed at 1.125rem square
- mark is fixed at 0.875rem square within the indicator
- label may wrap or truncate according to parent layout policy

### Composition

- parent expectations: forms, settings, filters, tables, selection rows, Field
  wrappers
- child expectations: optional visible label text
- resizing rules: label spacing stays stable regardless of state glyph; root
  uses inline-flex so it sizes to content

## 8. Token Usage — Exact Values

### Root `.checkbox`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |

### Root disabled `[data-disabled="true"]`

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |
| `cursor` | `not-allowed` |

### Control `.checkbox__control`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `opacity` | `0` |
| `pointer-events` | `none` |

### Indicator `.checkbox__indicator`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.125rem` |
| `height` | `1.125rem` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `0.3125rem` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-inverse)` |

### Indicator checked `:checked + .checkbox__indicator`

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-color-accent-base)` |
| `background` | `var(--poodle-color-accent-base)` |

### Indicator indeterminate `:indeterminate + .checkbox__indicator`

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-color-accent-base)` |
| `background` | `var(--poodle-color-accent-base)` |

### Indicator focus `:focus-visible + .checkbox__indicator`

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Mark `.checkbox__mark`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `0.875rem` |
| `height` | `0.875rem` |
| `line-height` | `1` |

### Label `.checkbox__label`

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `line-height` | `var(--poodle-typography-label-lineHeight)` |

## 9. Svelte Notes

- Uses a hidden native `<input type="checkbox">` for form semantics and
  accessibility, with a custom `.checkbox__indicator` sibling for visual
  rendering
- The root element is a `<label>` to associate the click target with the hidden
  input
- `indeterminate` is a DOM property (not an attribute) and must be set via
  JavaScript: `inputElement.indeterminate = isMixed`
- ReadOnly behavior: listen for the `change` event on the native input and
  immediately revert the checked state back to the controlled value, preventing
  the toggle from taking effect
- Adjacent sibling CSS selectors (`:checked +`, `:indeterminate +`,
  `:focus-visible +`) connect the hidden input state to the visible indicator
- `data-disabled` attribute on root drives disabled styling

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::checkbox`
- GPUI implementation must intentionally expose checkbox role, mixed/checked
  state, and accessible name through the native accessibility tree
- indeterminate state must be accessible to assistive technology as
  `aria-checked="mixed"`
- readonly behavior: reject toggle attempts while maintaining focusability
- keyboard toggle via Space must be explicitly handled

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] checked and mixed semantics match across platforms
- [ ] accessible name and checkbox state exposure match
- [ ] keyboard toggling via Space matches
- [ ] readonly behavior (revert on change) matches
- [ ] disabled behavior (opacity, cursor, suppressed interaction) matches
- [ ] indeterminate resolves to checked on first user toggle

### Tier 2: Visual Parity

- [ ] indicator sizing (1.125rem) matches
- [ ] indicator border-radius (0.3125rem) matches
- [ ] checked/indeterminate fill uses accent-base
- [ ] focus ring uses accent-focusRing with correct offset (0.125rem)
- [ ] mark sizing (0.875rem) matches
- [ ] label typography uses label token family
- [ ] disabled opacity uses state-opacity-disabled
- [ ] gap between indicator and label uses space-inline-sm

### Tier 3: Implementation Freedom

- [ ] native input internals vs GPUI control internals stay internal
- [ ] adjacent sibling CSS selector pattern is Svelte-specific

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| indeterminate set via JS property vs GPUI state | DOM property vs native state model | allowed | same visual and a11y result required |
| CSS adjacent sibling selectors | Svelte-specific DOM pattern | allowed | GPUI achieves same visual result through state-driven rendering |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default

Three basic checkboxes in a vertical stack with 12px gap:

| Label | Initial State | Notes |
|-------|--------------|-------|
| Enable email notifications | unchecked | interactive |
| Subscribe to marketing emails | unchecked | interactive |
| I agree to the terms and conditions | unchecked | interactive |

### States

Four state examples in a vertical stack with 12px gap:

| Label | State | Props |
|-------|-------|-------|
| Disabled unchecked | unchecked | `isDisabled: true` |
| Disabled checked | checked | `isDisabled: true` |
| Mixed / indeterminate | mixed | `checked: "mixed"` |
| Read-only checked | checked | `isReadOnly: true` |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings, filters, selection lists, Field-wrapped form
  inputs, table row selection
- future follow-up: coordinate mixed-state parent/child cascade patterns with
  composite tree or table contracts
