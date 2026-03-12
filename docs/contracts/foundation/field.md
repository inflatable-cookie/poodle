# Field

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Field`
- Layer: `foundation`
- Summary: a labeled field wrapper that owns label, description, validation,
  and error-message relationships around a single form control
- In scope: label, required/optional marker, help text, error text, pending
  text, control/message relationship wiring
- Out of scope: input editing semantics, submit buttons, multi-column form
  layout systems

## 2. Anatomy

```text
[Root]
  ├── [Header Row]
  │     ├── [Label]
  │     └── [Optionality Marker] (conditional)
  ├── [Description] (conditional)
  ├── [Control Slot]
  └── [Validation Message] (conditional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | field wrapper and stack spacing | stack spacing |
| Header Row | yes | label + optionality alignment | inline spacing |
| Label | yes | accessible naming anchor for control | label typography, text color |
| Description | no | pre-validation help text | body typography, secondary text |
| Control Slot | yes | injected input/select/etc | inherited from child control |
| Validation Message | no | error or pending copy | status color, body typography |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | none | yes | base control id and relationship anchor |
| `label` | `string` | none | yes | visible field label |
| `description` | `string \| null` | `null` | no | helper or scope text |
| `error` | `string \| null` | `null` | no | invalid-state message |
| `pendingMessage` | `string \| null` | `null` | no | pending-validation message |
| `validationState` | `"none" \| "invalid" \| "valid" \| "pending"` | `"none"` | no | field-level validation posture |
| `isRequired` | `boolean` | `false` | no | required marker and semantics hint |
| `optionalLabel` | `string \| null` | `"Optional"` | no | explicit optional marker text |

### Slot Contract

- default slot: one form control surface
- slot props must expose the relationship ids the child control needs:
  `describedBy`, `descriptionId`, `errorId`, `messageId`, `validationState`

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | no validation posture | label and help text only |
| invalid | `validationState="invalid"` and `error` exists | error copy visible and relationship attached |
| valid | `validationState="valid"` | no required success copy, but child control may visually confirm |
| pending | `validationState="pending"` and `pendingMessage` exists | pending copy visible and announceable |

## 5. Accessibility

### Semantics

- Role: wrapper remains structurally neutral by default
- Required behavior: visible `<label>` or equivalent naming anchor must point to
  the child control id
- Relationship rule: description and active validation message ids must be
  passed to the child control through `aria-describedby` or GPUI-equivalent
  relationship wiring
- Message precedence: invalid message wins over pending message when both are
  present

### Keyboard

- `Field` itself is not focusable
- keyboard semantics belong to the slotted control

### Focus And Announcement

- focus stays on the child control
- validation and pending copy must be attachable to assistive technology
  announcements
- GPUI-native accessibility mapping notes: GPUI must expose the same name,
  description, and error/pending relationships in native accessibility node
  structure even though there is no HTML label element fallback

## 6. Composition

- parent expectations: forms, filter bars, dialogs, drawers, settings panes
- child expectations: exactly one addressable form control
- layout rule: `Field` owns vertical stacking and messaging, not the child
  control chrome

## 7. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Label | `semantic.typography.label.*` | label text |
| Description/Message | `semantic.typography.body.*` | supporting text |
| Description | `semantic.color.text.secondary` | help text |
| Error | `semantic.color.status.danger` | invalid messaging |
| Spacing | `semantic.space.stack.*` and `semantic.space.inline.*` | vertical and header rhythm |

## 8. Svelte Notes

- should prefer real `<label for>` wiring
- the wrapper should not swallow child focus or edit events

## 9. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::field`
- GPUI implementation must explicitly model label-to-control and
  description/error relationships in the accessible tree

## 10. Parity Checklist

- [ ] label and description relationships match
- [ ] invalid and pending message precedence matches
- [ ] required/optional labeling rules match
- [ ] field wrapper remains non-focusable in both runtimes

## Next Task

Use `Field` as the canonical wrapper around `TextInput`, `SearchField`, and
future selection controls in `g02`, instead of duplicating label and message
markup per control.
