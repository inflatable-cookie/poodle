# Separator

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Separator`
- Layer: `foundation`
- Summary: a low-emphasis visual division primitive between adjacent content
  groups
- In scope: horizontal and vertical separators, decorative versus semantic
  separators
- Out of scope: resize handles, draggable splitters, tab indicators

## 2. Anatomy

```text
[Root]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | dividing rule | border color, stroke width, spacing context |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | axis of the rule |
| `decorative` | `boolean` | `true` | no | when false, semantic separator is exposed |
| `tone` | `"subtle" \| "default"` | `"subtle"` | no | divider emphasis |

### Controlled And Uncontrolled

- no controlled value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| subtle | default | low-emphasis divider |
| default | `tone="default"` | stronger divider |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | visual primitive only |

## 6. Accessibility

### Semantics

- Role: none when `decorative=true`; separator role or native equivalent when
  `decorative=false`
- Required attributes: orientation semantics when the platform requires them
- Labeling rules: decorative separators must be hidden from assistive
  technology

### Keyboard

| Key | Behavior |
|-----|----------|
| none | no intrinsic keyboard behavior |

### Focus And Announcement

- focus entry: never focusable
- live-region behavior: none

## 7. Layout

### Sizing

- horizontal separators span available inline size with minimal block size
- vertical separators span available block size with minimal inline size

### Composition

- parent expectations: adjacent content groups
- child expectations: none
- resizing rules: follows axis and parent constraints

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `semantic.color.border.subtle/default` | divider color |
| Root | `semantic.border.width.default` | stroke width |

## 9. Svelte Notes

- semantic HTML `<hr>` may be appropriate for horizontal semantic separators
- decorative separators should use `aria-hidden="true"` or equivalent

## 10. GPUI Notes

- GPUI implementation must distinguish decorative rules from semantic
  separators in the native accessibility tree
- decorative rules should not surface as focusable or named nodes

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] decorative vs semantic meaning matches
- [ ] orientation meaning matches
- [ ] separator never becomes focusable

### Tier 2: Visual Parity

- [ ] weight and contrast match

### Tier 3: Implementation Freedom

- [ ] HTML `<hr>` vs custom GPUI drawing stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: menus, forms, panels, detail layouts
- future follow-up: keep draggable splitters in a separate contract

## Next Task

Treat resize handles and split dividers as distinct interactive contracts,
not as variants of `Separator`.
