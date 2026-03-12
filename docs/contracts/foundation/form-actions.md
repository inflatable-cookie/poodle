# Form Actions

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `FormActions`
- Layer: `foundation`
- Summary: an action-row wrapper for submit, cancel, and secondary form actions
- In scope: action alignment, grouping, status text adjacency, wrapping under
  narrow widths
- Out of scope: button semantics themselves, validation logic, sticky footer
  shells

## 2. Anatomy

```text
[Root]
  ├── [Status / Secondary Content] (optional)
  └── [Action Cluster]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | action-row layout wrapper | stack and inline spacing |
| Status / Secondary Content | no | validation or save-state summary | body typography, secondary text |
| Action Cluster | yes | submit/cancel/secondary actions | inline spacing |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `align` | `"start" \| "end" \| "between"` | `"end"` | no | alignment rule for the row |

## 4. States

State table is sufficient for the base layout primitive.

## 5. Accessibility

### Semantics

- Role: neutral structural container by default
- required behavior: action order must remain logical for keyboard and screen
  reader users
- labeling rule: status text must stay textual, not icon-only

### Keyboard

- `FormActions` itself is not focusable
- keyboard order follows DOM order of slotted actions

### Focus And Announcement

- submit, cancel, and status-announcement semantics belong to child actions and
  parent form logic
- GPUI-native accessibility mapping notes: GPUI must preserve action order and
  any adjacent status text relationships even though there is no implicit HTML
  form footer pattern

## 6. Composition

- parent expectations: forms, dialog forms, drawers, inline edit groups
- child expectations: buttons or linked actions, plus optional status text
- layout rule: on narrow widths the row may wrap, but primary action order must
  remain stable

## 7. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `semantic.space.inline.*` | action spacing |
| Root | `semantic.space.stack.*` | separation from field stack |
| Status text | `semantic.typography.body.*` and `semantic.color.text.secondary` | supporting status copy |

## 8. Svelte Notes

- action rows should remain simple flex layout wrappers
- native form submit buttons can live inside the slot without `FormActions`
  taking over event semantics

## 9. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::form_actions`
- GPUI implementation must preserve action order, wrapping behavior intent, and
  nearby status text visibility without relying on HTML form-footer defaults

## 10. Parity Checklist

- [ ] action order and grouping match
- [ ] alignment behavior matches
- [ ] wrapped layouts preserve logical action ordering

## Next Task

Use `FormActions` in `g02.001` and later composite forms so action-row
structure stops being redefined ad hoc per screen.
