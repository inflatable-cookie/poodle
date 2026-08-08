# Radio

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Radio`
- Layer: `foundation`
- Summary: a single radio option built on a hidden native radio input with a
  custom circular indicator; group exclusivity comes from the native `name`
  attribute, so options compose freely inside forms and custom layouts
- In scope: checked state, native name/value group membership, label
  association, disabled and readonly semantics, controlled and uncontrolled
  value models
- Out of scope: managed option-group rendering and group-level value state
  (see RadioGroup), card-styled selection (see CardRadioGroup)

## 2. Anatomy

```text
[Root .radio]  <label>
  ├── [Control .radio__control]  <input type="radio"> (visually hidden)
  ├── [Indicator .radio__indicator]  <span>
  │     └── [Dot .radio__dot]  <span>
  └── [Label .radio__label]  <span> (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | label element wrapping the entire control | spacing, cursor |
| Control | yes | hidden native radio input for form and a11y semantics | visually hidden |
| Indicator | yes | visible circular boundary | border, background, focus ring |
| Dot | yes | inner fill dot, transparent until checked | selected color |
| Label | no | visible option label | typography, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string \| undefined` | `undefined` | no | element id for external label association |
| `name` | `string \| undefined` | `undefined` | no | native radio group name; exclusivity comes from the browser |
| `value` | `string \| undefined` | `undefined` | no | native submit value |
| `checked` | `boolean \| undefined` | `undefined` | no | bindable checked state; leave undefined for uncontrolled mode |
| `defaultChecked` | `boolean` | `false` | no | uncontrolled initial checked state |
| `disabled` | `boolean` | `false` | no | disables interaction, applies disabled opacity |
| `readOnly` | `boolean` | `false` | no | allows focus and reading but reverts any change attempt |
| `label` | `string \| null` | `null` | no | visible label text |
| `ariaLabel` | `string \| null` | `null` | no | accessible name; required when no visible label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target id |
| `selectedColor` | `string \| null` | `null` | no | optional selected-state color override for the checked border and dot |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `onCheckedChange` | `(checked: boolean) => void` | `undefined` | no | fires when the user changes the checked state |

### Controlled And Uncontrolled

- controlled: bindable `checked` plus `onCheckedChange`
- uncontrolled: `defaultChecked` seeds the initial state; the component owns
  it thereafter
- native radios only fire `change` when becoming checked; unchecking happens
  through a sibling with the same `name`, so a controlled parent must clear
  siblings itself
- `selectedColor` maps to a local CSS custom property on the root label

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| unchecked | default | empty circle with default border, transparent dot |
| checked | `checked=true` or user selection | border and dot use the selected color |
| custom selected color | `selectedColor` set while checked | border and dot use the provided color |
| focus | native input receives focus-visible | focus ring outline on indicator |
| disabled | `disabled=true` | reduced opacity, cursor not-allowed |
| readOnly | `readOnly=true` | change attempts reverted |

### Component States

- internal checked state (uncontrolled mode)

### Behavior Machine

Behavior classification: machine-backed (shared `switchTransition` in
`@inflatable-cookie/poodle-headless`)

Radio reuses the switch machine: single implicit state, checked value in
context, `TOGGLE { nextChecked }` from the native change event, disabled
inert, readOnly emitting `revertNativeChecked` without a callback, and
`emitCheckedChange` as the callback effect. Group exclusivity is native
browser behavior (`name`), not machine state.

- Machinery dependencies: none (native radio provides keyboard, focus, and
  group roving).

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onCheckedChange` | user selects the radio | `boolean` | suppressed when disabled; when readOnly the native change is reverted and no callback fires; native radios do not fire on uncheck |

## 6. Accessibility

### Semantics

- Role: native `<input type="radio">` element
- `id`: from prop, used for external `<label for>` association
- `name` / `value`: native group membership and submit value
- `aria-label`: from ariaLabel prop; required when no visible label exists
- `aria-describedby`: from describedBy prop
- `disabled`: native disabled attribute when `disabled`
- Labeling rules: visible label or programmatic ariaLabel required; the root
  `<label>` wraps the control so clicking the label selects the radio

### Keyboard

| Key | Behavior |
|-----|----------|
| `Space` | selects the focused radio when interactive |
| `Arrow keys` | native roving between same-`name` radios |
| `Tab` | moves focus into or past the group |

### Focus And Announcement

- focus entry: visible focus ring on the indicator via `:focus-visible` on
  the native input + adjacent sibling selector
- focus exit: ring clears while checked state remains visible
- live-region behavior: none; state changes are announced through native
  radio semantics
- GPUI-native accessibility mapping notes: GPUI must expose radio role,
  accessible name, checked state, and group membership through the native
  accessibility tree

## 7. Layout

### Sizing

- indicator sizes from the icon-size token per control size, with a
  proportional inner dot
- label may wrap or truncate according to parent layout policy

### Composition

- parent expectations: forms, Field wrappers, custom option layouts where
  RadioGroup's managed rendering is too rigid
- child expectations: optional visible label text
- resizing rules: root uses inline-flex so it sizes to content

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `--poodle-space-inline-sm` | indicator/label gap |
| Root | `--poodle-color-text-primary` | label color |
| Root disabled | `--poodle-state-opacity-disabled` | dimming |
| Indicator | `--poodle-color-border-default` | resting border |
| Indicator | `--poodle-color-background-surface` | fill |
| Indicator | `--poodle-size-icon-*` | per-size dimensions |
| Indicator/Dot checked | `--poodle-radio-selected-color` (defaults to `--poodle-color-accent-base`) | selected border and dot |
| Indicator focus | `--poodle-border-width-focus`, `--poodle-color-accent-focusRing` | focus ring |
| Indicator/Dot | `--poodle-motion-duration-interaction`, `--poodle-motion-easing-standard` | state transitions |
| Label | `--poodle-typography-label-*` | typography |

## 9. Svelte Notes

- hidden native `<input type="radio">` with a custom sibling indicator,
  matching the Checkbox pattern
- readOnly reverts the native checked property in the change handler via the
  shared switch machine's `revertNativeChecked` effect
- adjacent sibling CSS selectors connect input state to the indicator
- emits `data-size` / `data-density` / `data-disabled` on the root

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::radio`
- expose radio role, group membership, checked state, and accessible name
  through the native accessibility tree
- keyboard selection via Space and native-style group roving must be
  explicitly handled

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] checked semantics and group exclusivity match
- [ ] accessible name and radio state exposure match
- [ ] keyboard selection via Space matches
- [ ] readonly behavior (revert on change) matches
- [ ] disabled behavior (opacity, cursor, suppressed interaction) matches

### Tier 2: Visual Parity

- [ ] indicator and dot sizing per control size match
- [ ] checked border/dot use accent-base (or selectedColor override)
- [ ] focus ring uses accent-focusRing with 0.125rem offset
- [ ] label typography uses label tokens
- [ ] disabled opacity uses state-opacity-disabled

### Tier 3: Implementation Freedom

- [ ] native input internals vs GPUI control internals stay internal
- [ ] adjacent sibling CSS selector pattern is Svelte-specific

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none | | | |

## 13. Specimen Definitions

### Default

A three-option native group (shared `name`) in a vertical stack:

| Label | Initial State |
|-------|--------------|
| Standard shipping | checked |
| Express shipping | unchecked |
| Overnight shipping | unchecked |

### States

| Label | State | Props |
|-------|-------|-------|
| Disabled unchecked | unchecked | `disabled` |
| Disabled checked | checked | `checked disabled` |
| Read-only checked | checked | `checked readOnly` |
| Custom selected color | checked | `checked selectedColor` |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: forms and custom option layouts outside RadioGroup
- future follow-up: none
