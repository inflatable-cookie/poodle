# Inline

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Inline`
- Layer: `foundation`
- Summary: horizontal layout primitive for arranging items in a row with
  semantic spacing and optional wrap behavior
- In scope: row direction, gaps, alignment, wrapping
- Out of scope: toolbar semantics, roving focus, menu/tab behavior

## 2. Anatomy

```text
[Root]
  └── [Children...]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | horizontal flow container | inline gap, optional padding |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `gap` | `"sm" \| "md" \| "lg"` | `"md"` | no | semantic horizontal spacing |
| `align` | `"start" \| "center" \| "end" \| "stretch"` | `"center"` | no | cross-axis alignment |
| `justify` | `"start" \| "center" \| "end" \| "between"` | `"start"` | no | main-axis distribution |
| `wrap` | `boolean` | `false` | no | allows multi-row flow when true |
| `padding` | `"none" \| "sm" \| "md" \| "lg"` | `"none"` | no | interior spacing |
| `asRole` | `string \| null` | `null` | no | optional semantic grouping |

### Controlled And Uncontrolled

- no controlled value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | horizontal flow container |
| wrapped | `wrap=true` and children overflow | multi-row flow |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | layout primitive only |

## 6. Accessibility

### Semantics

- Role: neutral grouping by default
- Required attributes: none by default
- Optional attributes: grouping semantics when the container is more than
  visual alignment

### Keyboard

| Key | Behavior |
|-----|----------|
| none | keyboard semantics belong to children or higher-order containers |

### Focus And Announcement

- focus entry: root is not focusable by default
- live-region behavior: none

## 7. Layout

### Sizing

- inline size follows parent constraints
- block size grows with child size and wrap behavior

### Composition

- parent expectations: any layout or shell container
- child expectations: inline peers
- resizing rules: gap remains stable; wrapping is explicit

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `semantic.space.inline.*` | horizontal spacing |
| Root | semantic padding roles | optional interior spacing |

## 9. Svelte Notes

- implemented with flex row or equivalent
- if used as a toolbar or tablist, the higher-order contract must define the
  accessibility semantics rather than relying on `Inline`

## 10. GPUI Notes

- implemented with GPUI-native horizontal layout APIs
- native accessibility grouping must be added only when requested by the
  higher-order contract

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] gap, alignment, and wrap meaning match
- [ ] semantic neutrality matches
- [ ] focus neutrality matches

### Tier 2: Visual Parity

- [ ] inline spacing and wrapping behavior stay proportionally aligned

### Tier 3: Implementation Freedom

- [ ] CSS row layout vs GPUI row layout stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: button groups, header rows, shell utility strips
- future follow-up: add bidirectional/layout-direction guidance if required

## Next Task

Use `Inline` for future shell rows, action groups, and header utilities, but
attach explicit semantics when those groups become navigable composites.
