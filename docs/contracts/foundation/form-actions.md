# Form Actions

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `FormActions`
- Layer: `foundation`
- Summary: an action-row wrapper for submit, cancel, and secondary form actions
  with configurable alignment
- In scope: action alignment (start, end, between), grouping, wrapping under
  narrow widths, separation from field stack
- Out of scope: button semantics themselves, validation logic, sticky footer
  shells, status text (parent responsibility)

## 2. Anatomy

```text
[Root .form-actions]  <div>
  └── [Default Slot] (buttons and actions)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | action-row layout wrapper | spacing, alignment |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `align` | `"start" \| "end" \| "between"` | `"end"` | no | alignment rule for the action row |

### Slots

| Slot | Purpose |
|------|---------|
| default | buttons, links, or other action elements |

### Controlled And Uncontrolled

- layout-only component, no value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | actions aligned per `align` prop |
| wrapped | narrow container | actions wrap to multiple lines maintaining gap |

### Component States

State table is sufficient for this layout primitive. No interactive state transitions.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | — | — | events belong to slotted child actions |

## 6. Accessibility

### Semantics

- Role: neutral structural container (`<div>`), no implicit ARIA role
- Required behavior: action order must remain logical for keyboard and screen reader users
- Labeling rules: no accessible name needed on the container itself; child buttons own their labels

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves focus between slotted action buttons in DOM order |

### Focus And Announcement

- focus entry: FormActions itself is not focusable; focus goes to first slotted button
- focus exit: standard tab order continues past the last slotted button
- live-region behavior: none; status announcements belong to parent form logic
- GPUI-native accessibility mapping notes: GPUI must preserve action order and logical focus sequence even without HTML form-footer patterns

## 7. Layout

### Sizing

- Root stretches to parent width
- `flex-wrap: wrap` allows actions to wrap on narrow viewports
- `padding-top` separates the action row from the field stack above

### Composition

- parent expectations: forms, dialog forms, drawers, inline edit groups
- child expectations: buttons or linked actions
- resizing rules: on narrow widths the row wraps, but primary action order must remain stable

## 8. Token Usage — Exact Values

### Root `.form-actions`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--pug-space-inline-md)` |
| `align-items` | `center` |
| `padding-top` | `var(--pug-space-stack-sm)` |

### Root — `align="start"`

| Property | Value |
|----------|-------|
| `justify-content` | `flex-start` |

### Root — `align="end"` (default)

| Property | Value |
|----------|-------|
| `justify-content` | `flex-end` |

### Root — `align="between"`

| Property | Value |
|----------|-------|
| `justify-content` | `space-between` |

## 9. Svelte Notes

- Simple flex layout wrapper with `data-align` attribute for CSS targeting
- No event handling; all interaction belongs to slotted children
- Default slot accepts any content but intended for Button components
- `align` prop maps directly to `justify-content` value

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::form_actions`
- Spec struct: `FormActionsSpec` in primitives crate
- GPUI must preserve action order, wrapping behavior intent, and logical focus sequence
- The three alignment modes map to equivalent flex layout behaviors in GPUI
- No HTML form-footer defaults to rely on; explicit layout rules suffice

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] align prop values ("start", "end", "between") mean the same thing
- [ ] action order remains logical for keyboard and screen readers
- [ ] wrapped layouts preserve logical action ordering

### Tier 2: Visual Parity

- [ ] gap uses space-inline-md token
- [ ] padding-top uses space-stack-sm token
- [ ] flex-wrap behavior matches
- [ ] justify-content values match for each alignment mode

### Tier 3: Implementation Freedom

- [ ] container element type is implementation-owned
- [ ] wrapping breakpoint behavior is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| wrapping behavior may differ slightly | flex-wrap vs GPUI layout wrapping | allowed | keep action order and alignment meaning strict |

## 13. Specimen Definitions

### End-Aligned (Default)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| End-aligned (default) | default `align` (end), children: secondary "Cancel" button + primary "Save changes" button | Buttons right-aligned in the action row |

### Start-Aligned

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Start-aligned | `align="start"`, children: secondary "Back" button + primary "Continue" button | Buttons left-aligned in the action row |

### Space Between

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Space between | `align="between"`, children: danger "Delete" button + primary "Save" button | Buttons spread to opposite ends of the action row |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: forms, dialog forms, drawers, inline edit groups,
  settings panels
- future follow-up: consider whether status text adjacency needs formal contract
  support or stays as parent composition
