# Grid

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Grid`
- Layer: `foundation`
- Summary: a structural placement primitive for two-dimensional layout
- In scope: columns, rows, gaps, item placement
- Out of scope: data-grid semantics, keyboard navigation, cell selection

## 2. Anatomy

```text
[Root]
  └── [Children...]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | two-dimensional layout container | spacing and optional padding |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `columns` | `string` | `"1fr"` | no | layout track definition |
| `rows` | `string \| null` | `null` | no | optional row definition |
| `gap` | `"sm" \| "md" \| "lg"` | `"md"` | no | semantic grid gap |
| `padding` | `"none" \| "sm" \| "md" \| "lg"` | `"none"` | no | interior spacing |
| `asRole` | `string \| null` | `null` | no | optional semantic role |

### Controlled And Uncontrolled

- no controlled value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | grid placement container |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | layout primitive only |

## 6. Accessibility

### Semantics

- Role: neutral layout by default
- Required attributes: none by default
- Optional attributes: semantic grouping only when requested
- Labeling rules: do not assume `grid` role for visual grid layout

### Keyboard

| Key | Behavior |
|-----|----------|
| none | no intrinsic keyboard model |

### Focus And Announcement

- focus entry: root is not focusable by default
- live-region behavior: none

## 7. Layout

### Sizing

- track sizing is caller-defined through `columns` and `rows`
- gap remains semantic and token-backed

### Composition

- parent expectations: any sizing context
- child expectations: direct placed items
- resizing rules: placement follows track definitions, not content-specific
  semantics

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `semantic.space.inline.*` and `semantic.space.stack.*` | grid gap mapping |
| Root | semantic padding roles | interior spacing |

## 9. Svelte Notes

- implemented with CSS grid or equivalent
- data-table or spreadsheet semantics must be separate higher-order contracts

## 10. GPUI Notes

- implemented with GPUI-native layout constraints or custom placement helpers
- native accessibility semantics remain neutral unless a higher-order contract
  explicitly opts into table/grid semantics

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] visual-layout-only semantics match
- [ ] no accidental `grid` accessibility role is implied
- [ ] token-backed gap meaning matches

### Tier 2: Visual Parity

- [ ] track placement and gap proportions match

### Tier 3: Implementation Freedom

- [ ] CSS grid vs GPUI placement internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: card grids, stat grids, shell content areas
- future follow-up: separate `DataGrid` contract if interactive grid semantics
  become required

## Next Task

Keep `Grid` structural and reserve interactive grid semantics for higher-order
 contracts in later milestones.
