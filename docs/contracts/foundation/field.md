# Field

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `Field`
- Layer: `foundation`
- Summary: a labeled field wrapper that owns label, info popover,
  validation message, and error-message relationships around a single
  form control; it provides accessibility wiring to slotted controls
  via slot props
- In scope: label, required/optional marker, description/hint popover
  via info icon, error text, pending text, size/density scaling,
  control/message relationship wiring via slot props, grid-column span
  and grid-area for form layout integration
- Out of scope: input editing semantics (owned by child controls), submit
  buttons, multi-column form layout systems (owned by parent form/grid),
  field-group or fieldset semantics

## 2. Anatomy

```text
[Root .field]  <div data-size data-density data-validation-state>
  ├── [Header .field__header]  <div>
  │     ├── [Label Row .field__label-row]  <div>
  │     │     ├── [Label .field__label]  <label for={id}>
  │     │     │     └── [Required Marker .field__required] (conditional)
  │     │     └── [Info Popover]  <Popover placement="top"> (conditional)
  │     │           ├── trigger: [Info Icon .field__info-icon]
  │     │           └── surface: [Info Content .field__info-content] <p>
  │     └── [Optional Marker .field__optional] (conditional)
  ├── [Control Slot] (default slot, receives describedBy and validation props)
  └── [Validation Message .field__message] (conditional)
        ├── [Error .field__message--error]
        └── [Pending .field__message--pending]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | field wrapper with vertical stack spacing | stack spacing, grid-column, grid-area, data-size, data-density |
| Header | yes | label row + optionality marker alignment | inline spacing, baseline alignment |
| Label Row | yes | label + info icon container | font-size scales with size, inline-flex with gap |
| Label | yes | accessible naming anchor for the slotted control | label typography, text color |
| Info Popover | no | Popover triggered by info icon; shows description or hint text | Popover elevated surface |
| Info Icon | no | small icon button inside label row; scales with label font-size via em units | pill radius, secondary background |
| Optional Marker | no | text marker when field is not required | body typography, secondary text color |
| Required Marker | no | visual required indicator | status-danger color |
| Control Slot | yes | injected input/select/textarea control | inherited from child control |
| Validation Message | no | error or pending copy below the control | body typography, status color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | none | yes | base control id; used to derive error and message element ids |
| `label` | `string` | none | yes | visible field label text |
| `description` | `string \| null` | `null` | no | text shown in info popover next to the label |
| `hint` | `string \| null` | `null` | no | **deprecated** — alias for `description`; kept for backward compatibility |
| `error` | `string \| null` | `null` | no | invalid-state error message |
| `pendingMessage` | `string \| null` | `null` | no | pending-validation message |
| `validationState` | `"none" \| "invalid" \| "valid" \| "pending"` | `"none"` | no | field-level validation posture |
| `required` | `boolean` | `false` | no | shows required marker and sets semantics |
| `optionalLabel` | `string \| null` | `null` | no | explicit optional marker text; opt-in only |
| `span` | `number \| "full" \| null` | `null` | no | grid-column span for form layout |
| `gridArea` | `string \| null` | `null` | no | grid-area for named grid placement |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |

### Description/Hint Merging

Both `description` and `hint` feed the same info popover. `description`
takes precedence if both are provided. The `hint` prop is a deprecated
alias — use `description` for new code.

The description text is **never rendered as an inline paragraph**. It always
renders inside a Popover triggered by an info icon next to the label. This
ensures forms align consistently without varying-height help text between
label and control.

### Slot Contract

| Slot | Purpose | Props Passed |
|------|---------|-------------|
| default | exactly one form control | `describedBy`, `descriptionId` (always null), `errorId`, `messageId`, `validationState` |

- `describedBy`: computed string for the child control's `aria-describedby`,
  containing the error or pending message id as appropriate
- `descriptionId`: always `null` (description is in a popover, not inline)
- `errorId`: id of the error message element
- `messageId`: id of the active validation message element
- `validationState`: passed through so the child control can render its own
  validation border

### Controlled And Uncontrolled

- Field has no value model; it is a pure layout and relationship wrapper

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | no validation posture | label visible; no optional marker unless `optionalLabel` is provided |
| with description | `description` set | info icon visible next to label; click/hover opens popover |
| required | `required=true` | required marker visible |
| invalid | `validationState="invalid"` and `error` set | error message visible below control |
| valid | `validationState="valid"` | no message; child control may show success border |
| pending | `validationState="pending"` and `pendingMessage` set | pending message visible below control |

## 5. Events

Field fires no events. All interaction events are owned by the slotted
control. The info popover's open/close is handled internally by the
embedded Popover component.

## 6. Accessibility

### Semantics

- Root: structurally neutral (`<div>`), not a fieldset or group
- Label: visible `<label>` element with `for` attribute pointing to `id`
- Info icon: `aria-label="More information"` on the Popover trigger
- Info popover: `role="dialog"` with `aria-label="Field description"`
- Required marker: decorative (`aria-hidden`); child should set `aria-required`
- Error/pending: `aria-live="polite"` for dynamic announcements
- `describedBy` includes error or pending message id (not description,
  since description is in a popover)

### Keyboard

- Field itself is not focusable
- Clicking the label focuses the slotted control (native `<label for>`)
- Info icon is focusable and toggles the popover on click
- Escape closes the info popover

## 7. Layout

### Sizing

- Root uses vertical grid with stack spacing
- Label row uses inline-flex with gap; font-size set per size variant
- Info icon uses `em` units (`1.25em` wrapper, `0.75em` SVG) so it
  scales proportionally with the label font-size at every size stop

### Size Adjustments

| Size | Label Row font-size | Message/Optional font-size |
|------|--------------------|-----------------------------|
| `xs` | `0.6875rem` | `0.625rem` |
| `sm` | `0.75rem` | `0.6875rem` |
| `md` | `label-size` (0.8125rem) | `0.75rem` |
| `lg` | `0.875rem` | `0.8125rem` |
| `xl` | `0.9375rem` | `0.875rem` |

### Composition

- parent expectations: forms, filter bars, dialogs, drawers, settings panes
- child expectations: exactly one addressable form control
- layout rule: Field owns vertical stacking and messaging; child control
  owns its own chrome

## 8. Token Usage — Exact Values

### Root `.field`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-sm)` |

### Header `.field__header`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `baseline` |
| `justify-content` | `space-between` |
| `gap` | `var(--poodle-space-inline-md)` |

### Label Row `.field__label-row`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `font-size` | `var(--poodle-typography-label-size)` (scales per size variant) |

### Label `.field__label`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `line-height` | `var(--poodle-typography-label-lineHeight)` |

### Info Icon `.field__info-icon`

| Property | Value |
|----------|-------|
| `width` | `1.25em` |
| `height` | `1.25em` |
| `border-radius` | `var(--poodle-radius-pill)` |
| `background` | `color-mix(in srgb, var(--poodle-color-text-secondary) 14%, transparent)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `cursor` | `pointer` |

### Info Icon SVG

| Property | Value |
|----------|-------|
| `width` | `0.75em` |
| `height` | `0.75em` |

### Info Popover Surface (via Popover component)

| Property | Value |
|----------|-------|
| `min-width` | `10rem` |
| `max-width` | `22rem` |
| `padding` | `0.5rem 0.625rem` |
| `font-size` | `0.75rem` |
| `line-height` | `1.5` |
| `placement` | `top` |
| `offset` | `6px` |

## 9. Svelte Notes

- Label uses `<label for="{id}">` for native label-to-control association
- Description and hint are merged: `infoText = description ?? hint`
- Info icon uses the Popover primitive with `placement="top"` and
  `offset={6}`; Popover surface min/max-width overridden via
  `:global(.popover__surface)` scoped selector
- Info icon uses `em` units so it scales with the label row font-size
- `data-size` and `data-density` attributes emitted on root
- `descriptionId` slot prop is always `null` (description is in popover)
- `hint` prop kept as deprecated alias for `description`
- Size variants set font-size on `.field__label-row` (not `.field__label`)
  so both label text and em-based icon scale together

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::field`
- GPUI must expose label-to-control and error/pending relationships
  in native accessible tree
- Description renders as a tooltip/popover triggered by an info icon,
  not as inline text
- Info icon size should match label font size proportionally
- Grid-column span and grid-area: GPUI must support equivalent layout

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] label-to-control accessible name relationship matches
- [ ] error/pending `aria-describedby` relationships match
- [ ] invalid and pending message precedence matches
- [ ] required/optional marker visibility rules match
- [ ] description renders in info popover, not inline
- [ ] info icon scales with label font-size across all size stops

### Tier 2: Visual Parity

- [ ] label typography matches per size variant table
- [ ] info icon size matches (1.25em wrapper, 0.75em SVG)
- [ ] optional/message typography matches per size variant table
- [ ] required marker color matches (status-danger)
- [ ] error message color matches (status-danger)
- [ ] stack spacing matches (space-stack-sm)
- [ ] header inline spacing matches (space-inline-md)
- [ ] info icon background and hover state match

### Tier 3: Implementation Freedom

- [ ] label-to-control wiring mechanism
- [ ] grid-column/grid-area integration is platform-owned
- [ ] info popover implementation details (Popover primitive vs custom)

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| label-to-control wiring | HTML `<label for>` vs GPUI native | allowed | same accessible name result |
| grid layout integration | CSS grid vs GPUI layout system | allowed | same positioning result |
| info popover implementation | Svelte uses Popover primitive; GPUI may use tooltip | allowed | same content and trigger behavior |

## 13. Specimen Definitions

### Default With Description

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default with description | `label="Display name"`, `description="This is how your name appears to other users."`, child TextInput | Field with label, info icon, and text input; clicking icon shows description in popover |

### Required

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Required | `label="Email address"`, `required`, child TextInput | Field with label and required marker |

### With Error

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With error | `label="Username"`, `error="This username is already taken."`, `validationState="invalid"`, child TextInput | Field with label, error message below control |

### Optional With Description

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Optional with description | `label="Phone number"`, `optionalLabel="Optional"`, `description="Include country code."`, child TextInput | Field with label, info icon, optional marker, and text input |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: all form-based UIs wrapping TextInput, TextArea,
  SearchInput, Select, and future selection controls
- future follow-up: fieldset/field-group semantics for grouped controls
