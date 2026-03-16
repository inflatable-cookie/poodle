# Field

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Field`
- Layer: `foundation`
- Summary: a labeled field wrapper that owns label, description, validation
  message, and error-message relationships around a single form control; it
  provides accessibility wiring to slotted controls via slot props
- In scope: label, required/optional marker, help text, error text, pending
  text, control/message relationship wiring via slot props, grid-column span
  and grid-area for form layout integration
- Out of scope: input editing semantics (owned by child controls), submit
  buttons, multi-column form layout systems (owned by parent form/grid),
  field-group or fieldset semantics

## 2. Anatomy

```text
[Root .field]  <div>
  ├── [Header .field__header]  <div>
  │     ├── [Label .field__label]  <label>
  │     └── [Optional Marker .field__optional] (conditional, when not required)
  │         OR [Required Marker .field__required] (conditional, when required)
  ├── [Description .field__description] (conditional, when description prop set)
  ├── [Control Slot] (default slot, receives describedBy and validation props)
  └── [Validation Message .field__message] (conditional)
        ├── [Error .field__message--error] (when validationState="invalid" and error set)
        └── [Pending .field__message--pending] (when validationState="pending" and pendingMessage set)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | field wrapper with vertical stack spacing | stack spacing, grid-column, grid-area |
| Header | yes | label + optionality marker alignment row | inline spacing, baseline alignment |
| Label | yes | accessible naming anchor for the slotted control | label typography, text color |
| Optional Marker | no | text marker when field is not required | body typography, secondary text color |
| Required Marker | no | visual required indicator | status-danger color |
| Description | no | pre-validation help text | body typography, secondary text color |
| Control Slot | yes | injected input/select/textarea control | inherited from child control |
| Validation Message | no | error or pending copy below the control | body typography, status color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | none | yes | base control id; used to derive description, error, and message element ids |
| `label` | `string` | none | yes | visible field label text |
| `description` | `string \| null` | `null` | no | helper or scope text below label |
| `error` | `string \| null` | `null` | no | invalid-state error message |
| `pendingMessage` | `string \| null` | `null` | no | pending-validation message |
| `validationState` | `"none" \| "invalid" \| "valid" \| "pending"` | `"none"` | no | field-level validation posture |
| `isRequired` | `boolean` | `false` | no | shows required marker and sets semantics |
| `optionalLabel` | `string \| null` | `"Optional"` | no | explicit optional marker text; null hides the marker |
| `span` | `number \| "full" \| null` | `null` | no | grid-column span for form layout |
| `gridArea` | `string \| null` | `null` | no | grid-area for named grid placement |

### Slot Contract

| Slot | Purpose | Props Passed |
|------|---------|-------------|
| default | exactly one form control | `describedBy`, `descriptionId`, `errorId`, `messageId`, `validationState` |

- `describedBy`: computed string for the child control's `aria-describedby`,
  combining description, error, and pending message ids as appropriate
- `descriptionId`: id of the description element (derived from `id` prop)
- `errorId`: id of the error message element (derived from `id` prop)
- `messageId`: id of the pending message element (derived from `id` prop)
- `validationState`: passed through so the child control can render its own
  validation border

### Controlled And Uncontrolled

- Field has no value model; it is a pure layout and relationship wrapper
- Value semantics are entirely owned by the slotted control

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | no validation posture | label, optional marker (if not required), and description visible |
| required | `isRequired=true` | required marker visible instead of optional marker |
| invalid | `validationState="invalid"` and `error` set | error message visible below control; `describedBy` includes error id |
| valid | `validationState="valid"` | no required success copy; child control may show success border |
| pending | `validationState="pending"` and `pendingMessage` set | pending message visible below control; `describedBy` includes message id |

### Message Precedence

- When both `error` and `pendingMessage` are present, the `validationState`
  determines which is shown: `"invalid"` shows the error, `"pending"` shows the
  pending message
- Only one validation message is visible at a time

## 5. Events

Field fires no events. It is a pure wrapper component. All interaction events
are owned by the slotted control.

## 6. Accessibility

### Semantics

- Role: wrapper is structurally neutral (`<div>`), not a fieldset or group
- Label: visible `<label>` element with `for` attribute pointing to the child
  control's `id`
- Required: when `isRequired` is true, the required marker is decorative; the
  child control should set `aria-required` or the native `required` attribute
- Description relationship: `aria-describedby` on the child control includes
  the description id when description is present
- Error relationship: `aria-describedby` on the child control includes the
  error id when error is present and validationState is invalid
- Pending relationship: `aria-describedby` on the child control includes the
  message id when pendingMessage is present and validationState is pending
- The `describedBy` slot prop computes the full space-separated id list so
  child controls can use it directly

### Keyboard

- Field itself is not focusable
- Clicking the label focuses the slotted control (native `<label for>` behavior)
- All keyboard semantics belong to the slotted control

### Focus And Announcement

- Focus stays on the child control
- Validation and pending copy are attachable to assistive technology
  announcements via `aria-describedby` relationship
- When validation state changes from none to invalid, screen readers announce
  the error message on the next focus event of the child control
- GPUI-native accessibility mapping notes: GPUI must expose the same name,
  description, and error/pending relationships in native accessibility node
  structure even though there is no HTML label element fallback

## 7. Layout

### Sizing

- Root uses vertical grid with stack spacing between children
- Width determined by parent (form grid or flex container)
- `span` prop sets `grid-column: span {n}` for form grid integration
- `span="full"` sets `grid-column: 1 / -1` (full width)
- `gridArea` prop sets `grid-area` for named grid placement

### Composition

- parent expectations: forms, filter bars, dialogs, drawers, settings panes
- child expectations: exactly one addressable form control (TextInput, TextArea,
  Select, etc.)
- layout rule: Field owns vertical stacking and messaging; the child control
  owns its own chrome (border, background, focus ring)

## 8. Token Usage — Exact Values

### Root `.field`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--pug-space-stack-sm)` |

### Root inline styles (conditional)

| Condition | Property | Value |
|-----------|----------|-------|
| `span` is a number | `grid-column` | `span {n}` |
| `span` is `"full"` | `grid-column` | `1 / -1` |
| `gridArea` is set | `grid-area` | `{gridArea}` |

### Header `.field__header`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `baseline` |
| `justify-content` | `space-between` |
| `gap` | `var(--pug-space-inline-md)` |

### Label `.field__label`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-primary)` |
| `font-family` | `var(--pug-typography-label-family)` |
| `font-size` | `var(--pug-typography-label-size)` |
| `font-weight` | `var(--pug-typography-label-weight)` |
| `line-height` | `var(--pug-typography-label-lineHeight)` |
| `margin` | `0` |

### Required Marker `.field__required`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-status-danger)` |

### Optional Marker `.field__optional`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |
| `font-family` | `var(--pug-typography-body-family)` |
| `font-size` | `0.75rem` |
| `line-height` | `var(--pug-typography-body-lineHeight)` |
| `margin` | `0` |

### Description `.field__description`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |
| `font-family` | `var(--pug-typography-body-family)` |
| `font-size` | `0.75rem` |
| `line-height` | `var(--pug-typography-body-lineHeight)` |
| `margin` | `0` |

### Error Message `.field__message--error`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-status-danger)` |
| `font-family` | `var(--pug-typography-body-family)` |
| `font-size` | `0.75rem` |
| `line-height` | `var(--pug-typography-body-lineHeight)` |
| `margin` | `0` |

### Pending Message `.field__message--pending`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |
| `font-family` | `var(--pug-typography-body-family)` |
| `font-size` | `0.75rem` |
| `line-height` | `var(--pug-typography-body-lineHeight)` |
| `margin` | `0` |

## 9. Svelte Notes

- Label uses `<label for="{id}">` for native label-to-control association
- Required marker renders an asterisk or similar indicator; optional marker
  renders the `optionalLabel` text (default "Optional")
- Slot passes `describedBy`, `descriptionId`, `errorId`, `messageId`, and
  `validationState` as slot props so child controls can wire up accessibility
  relationships without knowing the Field's internal id scheme
- `span` and `gridArea` are applied as inline styles on the root element for
  form grid integration
- The wrapper does not swallow child focus or edit events
- Description and message elements use derived ids: `{id}-description`,
  `{id}-error`, `{id}-message` (or similar scheme)

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::field`
- GPUI implementation must explicitly model label-to-control and
  description/error relationships in the accessible tree
- Since GPUI has no HTML `<label for>` equivalent, the accessible name
  relationship must be established through native accessibility APIs
- Grid-column span and grid-area for form layout: GPUI must support equivalent
  layout properties in its form/grid system
- Message precedence (error over pending based on validationState) must be
  enforced in the same way

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] label-to-control accessible name relationship matches
- [ ] description and error/pending `aria-describedby` relationships match
- [ ] invalid and pending message precedence matches
- [ ] required/optional marker visibility rules match
- [ ] field wrapper remains non-focusable in both runtimes
- [ ] slot props (describedBy, validationState) are passed to child controls

### Tier 2: Visual Parity

- [ ] label typography matches (label family, size, weight, line-height)
- [ ] optional/description/message typography matches (body family, size, line-height)
- [ ] required marker color matches (status-danger)
- [ ] error message color matches (status-danger)
- [ ] pending message and description color matches (text-secondary)
- [ ] stack spacing matches (space-stack-sm)
- [ ] header inline spacing matches (space-inline-md)
- [ ] header baseline alignment matches

### Tier 3: Implementation Freedom

- [ ] label-to-control wiring mechanism (HTML `<label for>` vs native accessibility API)
- [ ] grid-column/grid-area integration is platform-owned
- [ ] id derivation scheme for description/error/message elements

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| label-to-control wiring mechanism | HTML `<label for>` vs GPUI native accessibility node | allowed | same accessible name result required |
| grid layout integration | CSS grid-column/grid-area vs GPUI layout system | allowed | same visual positioning result required |
| id derivation scheme | internal implementation detail | allowed | child controls must receive correct describedBy string |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: all form-based UIs wrapping TextInput, TextArea,
  SearchField, Select, and future selection controls
- future follow-up: fieldset/field-group semantics for grouped controls
  (e.g. radio groups within a form) may need a separate FieldGroup contract
