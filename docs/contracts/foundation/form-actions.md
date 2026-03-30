# Form Actions

Status: detailed contract
Updated: 2026-03-27

## 1. Purpose

- Component name: `FormActions`
- Layer: `foundation`
- Summary: an action-row wrapper for submit, cancel, secondary, and optional
  destructive form actions
- In scope: action alignment (`start`, `end`, `between`), wrapping, field-stack
  separation, optional inline danger action content, optional collapsed overflow
  danger actions on narrow containers
- Out of scope: button semantics, validation logic, sticky footer shells,
  confirmation dialogs, status text

## 2. Anatomy

```text
[Root .form-actions]
  ├── [Children Snippet]
  ├── [Danger Snippet] (optional)
  └── [Danger Menu Trigger] (optional; narrow containers only)
```

| Part | Required | Description |
|------|----------|-------------|
| Root | yes | action-row layout wrapper |
| Danger snippet | no | inline cancel/delete content for wider containers |
| Danger menu | no | overflow treatment for danger actions on narrow containers |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `align` | `"start" \| "end" \| "between"` | `"end"` | no | alignment rule for the action row |
| `dangerItems` | `FormActionDangerItem[]` | `[]` | no | menu items used when danger content collapses |

### Snippets

| Snippet | Purpose |
|---------|---------|
| `children` | buttons, links, or other action elements |
| `danger` | optional destructive or cancel action content |

### Controlled And Uncontrolled

- layout-only component; no value model
- `dangerItems[].onSelect` is callback-driven

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | actions aligned per `align` prop |
| wrapped | narrow container | actions wrap to multiple lines maintaining gap |
| danger-inline | `danger` snippet present | danger content is rendered inline |
| danger-collapsed | `danger` snippet and `dangerItems` present in a narrow container | danger content is hidden and overflow trigger is shown |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | — | — | interaction belongs to slotted children and `dangerItems.onSelect` |

## 6. Accessibility

- neutral structural container; no implicit ARIA role
- action order must remain logical for keyboard and screen reader users
- collapsed danger trigger must expose an `aria-label`

## 7. Layout

- Root stretches to parent width
- `flex-wrap: wrap` allows actions to wrap on narrow widths
- `padding-top` separates the action row from the field stack above
- Root is a container-query boundary for responsive danger-action swapping

## 8. Token Usage

### Root `.form-actions`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-md)` |
| `align-items` | `center` |
| `padding-top` | `var(--poodle-space-stack-sm)` |
| `container-type` | `inline-size` |

### Root Alignment

| Variant | Property | Value |
|---------|----------|-------|
| `start` | `justify-content` | `flex-start` |
| `end` | `justify-content` | `flex-end` |
| `between` | `justify-content` | `space-between` |

### Danger Inline `.form-actions__danger`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-md)` |

### Danger Menu `.form-actions__danger-menu`

| Property | Value |
|----------|-------|
| `display` | `none` by default |
| `align-items` | `center` |

### Responsive Swap

| Condition | Result |
|-----------|--------|
| `@container (max-width: 31.25rem)` with both `danger` and `dangerItems` | inline danger hidden, danger menu shown |

## 9. Svelte Notes

- `children` is the main action row snippet
- `danger` is a named snippet for cancel/delete actions that should remain visually
  separated from primary actions
- if `dangerItems` is absent, inline danger content remains visible at all sizes
- if both `danger` and `dangerItems` are provided, `dangerItems` become the
  narrow-container overflow treatment

## 10. Parity Checklist

- [ ] align prop values mean the same thing
- [ ] action order remains logical for keyboard and screen readers
- [ ] wrapped layouts preserve logical action ordering
- [ ] danger snippet remains inline when `dangerItems` is absent
- [ ] danger snippet collapses into overflow only when both `danger` and
  `dangerItems` are present

## 11. Specimen Definitions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| End-aligned (default) | default `align`, children: secondary "Cancel" + primary "Save changes" | Buttons right-aligned |
| Start-aligned | `align="start"`, children: secondary "Back" + primary "Continue" | Buttons left-aligned |
| Space between | `align="between"`, children: danger "Delete" + primary "Save" | Buttons spread to opposite ends |
| Responsive danger actions | `align="end"`, `danger` snippet contains a destructive/cancel action, `dangerItems` contains matching overflow action | Danger action stays inline on wide containers and collapses to overflow on narrow containers |

## 12. Approval And Adoption Notes

- downstream adopters: forms, dialog forms, drawers, inline edit groups,
  settings panels
- next follow-up: evaluate whether status text adjacency should remain parent
  composition or become formal contract surface
