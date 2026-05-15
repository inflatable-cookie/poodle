# Radio Group

Status: detailed contract
Updated: 2026-03-24

## 1. Purpose

- Component name: `RadioGroup`
- Layer: `foundation`
- Summary: a mutually exclusive selection control composed of labeled radio
  options with hidden native radio inputs and custom visual indicators; supports
  vertical and horizontal orientation with roving focus
- In scope: selected value, option labeling, per-option disabled state,
  group-level disabled, orientation-driven layout, controlled and uncontrolled
  value models
- Out of scope: segmented visual presentation (see SegmentedControl),
  multi-select behavior, rich option content beyond text labels

## 2. Anatomy

```text
[Root .radio-group]  <div role="radiogroup">
  └── [Option .radio-group__option...]  <label>
        ├── [Control .radio-group__control]  <input type="radio"> (visually hidden)
        ├── [Indicator .radio-group__indicator]  <span>
        │     └── [Dot .radio-group__dot]  <span>
        └── [Label .radio-group__label]  <span>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | radiogroup container with orientation-driven layout | gap, grid |
| Option | yes | label wrapping a single radio choice | spacing, cursor, text color |
| Control | yes | hidden native radio input for form and a11y semantics | visually hidden |
| Indicator | yes | circular border showing selected/unselected state | border, background, transition, focus ring |
| Dot | yes | inner filled circle visible when selected | background, transition |
| Label | yes | visible option text | typography, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null \| undefined` | `undefined` | no | controlled selected value; leave undefined for uncontrolled mode |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial value |
| `options` | `RadioGroupOption[]` | none | yes | array of selectable options |
| `orientation` | `"horizontal" \| "vertical"` | `"vertical"` | no | layout and navigation axis |
| `disabled` | `boolean` | `false` | no | disables entire group |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the group; required when no visible group label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target id |
| `name` | `string \| undefined` | `undefined` | no | shared form name for all radio inputs; auto-generated via module-level counter when not provided |
| `selectedColor` | `string \| null` | `null` | no | optional selected-state color override used for the selected indicator border and dot |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### RadioGroupOption Type

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `value` | `string` | yes | unique value identifier |
| `label` | `string` | yes | visible option label |
| `disabled` | `boolean` | no | disables this specific option |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange` callback; leave `value` undefined for uncontrolled mode
- uncontrolled: `defaultValue` sets the initial selection; component owns its own
  state thereafter
- a module-level `nextRadioGroupId` counter generates unique name attributes when
  the `name` prop is not provided, ensuring radio grouping without collisions
- `selectedColor` maps to a local CSS custom property on the root group and
  affects only the selected state visuals for that radio group instance

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| unselected | default option state | empty indicator with default border |
| selected | option value equals current value | dot visible with accent color, indicator border accent |
| custom selected color | `selectedColor` set while an option is selected | selected indicator border and dot use the provided local selected color |
| focus | radio input receives focus-visible | focus ring outline on indicator |
| disabled-group | `isDisabled=true` on group | all options muted with disabled opacity and cursor |
| disabled-option | `isDisabled=true` on option | individual option muted with disabled opacity and cursor |

### Component States

- internal selected value (uncontrolled mode)
- roving focus tracked via native radio group behavior

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | user selects a different option | `string` | only fires when selection changes; suppressed when disabled |

## 6. Accessibility

### Semantics

- Role: `role="radiogroup"` on root container
- `aria-label`: from ariaLabel prop on the root; required when no visible group
  label exists
- `aria-describedby`: from describedBy prop on the root
- `aria-orientation`: NOT currently set on the radiogroup element; orientation is conveyed via `data-orientation` attribute only
- Each radio input: native `<input type="radio">` with shared `name` attribute
- Per-option `disabled`: native disabled attribute
- Labeling rules: group must have an accessible name; each option label is
  associated via wrapping `<label>` element

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | enters the group on the selected or first enabled option; exits on next Tab |
| `Arrow Down/Right` | moves selection/focus to next option (wraps) |
| `Arrow Up/Left` | moves selection/focus to previous option (wraps) |
| `Space` | selects focused option if not already selected |
| `Home` | moves to first enabled option (optional) |
| `End` | moves to last enabled option (optional) |

### Focus And Announcement

- focus entry: one option is in the tab sequence at a time (roving focus via
  native radio group behavior)
- focus exit: roving focus position is preserved for re-entry
- live-region behavior: none; selection change is announced via native radio
  semantics
- GPUI-native accessibility mapping notes: GPUI must implement radiogroup role,
  per-option selection state, roving-focus behavior, and orientation-aware
  keyboard navigation explicitly

## 7. Layout

### Sizing

- vertical: options stack with `space-stack-sm` gap
- horizontal: options flow in columns with `space-inline-md` gap
- option spacing stays consistent regardless of selected state

### Composition

- parent expectations: forms, settings, filter groups, Field wrappers
- child expectations: options are group-owned via the `options` prop; not
  arbitrary slotted content in this baseline contract
- resizing rules: orientation drives gap axis and navigation axis together;
  labels truncate with `min-width: 0`

## 8. Token Usage — Exact Values

### Root `.radio-group`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-sm)` |
| `--poodle-radio-selected-color` | `var(--poodle-color-accent-base)` |

### Root horizontal `[data-orientation="horizontal"]`

| Property | Value |
|----------|-------|
| `grid-auto-flow` | `column` |
| `grid-auto-columns` | `minmax(0, max-content)` |
| `gap` | `var(--poodle-space-inline-md)` |
| `align-items` | `center` |

### Option `.radio-group__option`

| Property | Value |
|----------|-------|
| `display` | `inline-grid` |
| `grid-template-columns` | `auto minmax(0, 1fr)` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `min-width` | `0` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |

### Option disabled `[data-disabled="true"]`

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Control `.radio-group__control`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `opacity` | `0` |
| `pointer-events` | `none` |

### Indicator `.radio-group__indicator`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.125rem` |
| `height` | `1.125rem` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `999px` |
| `background` | `var(--poodle-color-background-surface)` |
| `transition` | `border-color, box-shadow` at `var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

### Indicator checked `:checked + .radio-group__indicator`

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-radio-selected-color)` |

### Indicator focus `:focus-visible + .radio-group__indicator`

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Dot `.radio-group__dot`

| Property | Value |
|----------|-------|
| `width` | `0.5rem` |
| `height` | `0.5rem` |
| `border-radius` | `999px` |
| `background` | `transparent` |
| `transition` | `background` at `var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

### Dot checked `:checked + .radio-group__indicator .radio-group__dot`

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-radio-selected-color)` |

### Label `.radio-group__label`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `line-height` | `var(--poodle-typography-label-lineHeight)` |

### Size adjustments

| Size | indicator | dot |
|------|-----------|-----|
| `xs` | `calc(icon-default - 0.125rem)` | `calc(icon-default * 0.4)` |
| `sm` | `icon-default` | `calc(icon-default * 0.45)` |
| `md` | `1.125rem` | `0.5rem` |
| `lg` | `calc(icon-default + 0.375rem)` | `calc(icon-default * 0.55)` |
| `xl` | `calc(icon-default + 0.625rem)` | `calc(icon-default * 0.6)` |

## 9. Svelte Notes

- Uses hidden native `<input type="radio">` elements for form semantics and
  roving focus behavior
- A module-level `nextRadioGroupId` counter auto-generates unique `name`
  attributes when the `name` prop is omitted, preventing cross-group collisions
- The root container is a `<div>` with `role="radiogroup"`, `aria-label`, and
  `aria-describedby`
- Each option is a `<label>` wrapping the hidden input and custom indicator
- Adjacent sibling CSS selectors (`:checked +`, `:focus-visible +`) connect
  hidden input state to the visible indicator and dot
- `data-orientation` attribute on root drives layout via CSS attribute selectors
- `data-disabled` attribute on each option drives per-option disabled styling
- Native radio group keyboard behavior (arrow keys move selection) is inherited
  from the browser
- Emits `data-size` on root element reflecting the resolved size
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::radio_group`
- GPUI implementation must explicitly maintain one tabbable option (roving
  focus), directional selection movement, and native radiogroup accessibility
  semantics
- each option must expose its selected state and accessible name individually
- orientation must be reflected in both keyboard navigation axis and the
  accessibility tree via `aria-orientation`
- disabled options must be skipped during keyboard navigation

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] single-selection semantics match (only one option selected at a time)
- [ ] roving-focus behavior matches (arrow keys move selection, Tab enters/exits)
- [ ] orientation-based keyboard navigation matches
- [ ] group and per-option accessibility semantics match
- [ ] per-option disabled behavior matches (skipped in navigation)
- [ ] group-level disabled matches

### Tier 2: Visual Parity

- [ ] indicator sizing (1.125rem circle) matches
- [ ] dot sizing (0.5rem) and accent color match
- [ ] indicator border transitions use same motion tokens
- [ ] focus ring uses accent-focusRing with correct offset (0.125rem)
- [ ] label typography uses label token family
- [ ] disabled opacity uses state-opacity-disabled
- [ ] vertical gap uses space-stack-sm
- [ ] horizontal gap uses space-inline-md
- [ ] all five sizes visually match (indicator and dot per size table)

### Tier 3: Implementation Freedom

- [ ] DOM radio inputs vs GPUI option entities stay internal
- [ ] module-level ID counter is Svelte-specific
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| module-level ID counter for name generation | Svelte-specific; GPUI handles grouping differently | allowed | same grouping semantics required |
| CSS adjacent sibling selectors | Svelte-specific DOM pattern | allowed | GPUI achieves same visual result through state-driven rendering |
| transition timing | GPUI may not support CSS-style transitions | allowed | match visual feel where possible |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Vertical (default)

Vertical radio group with three options:

| Label | Options | Initial | Orientation |
|-------|---------|---------|-------------|
| Plan | Free, Pro, Enterprise | Pro | vertical |

### Horizontal

Horizontal radio group with four options:

| Label | Options | Initial | Orientation |
|-------|---------|---------|-------------|
| Size | Small, Medium, Large, Extra large | Medium | horizontal |

### Disabled

Disabled vertical radio group:

| Label | Options | Initial | Props |
|-------|---------|---------|-------|
| Plan | Free, Pro, Enterprise | Free | `isDisabled: true` |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings, mutually exclusive control groups, Field-wrapped
  form inputs, filter groups
- future follow-up: coordinate segmented presentation with `SegmentedControl`
  contract when visual toggle-bar variants are needed
