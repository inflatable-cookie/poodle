# Stack

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Stack`
- Layer: `foundation`
- Summary: vertical layout primitive for ordered blocks with consistent spacing
- In scope: vertical flow, alignment, gaps, optional wrapping suppression
- Out of scope: grid placement, inline wrapping, interactive list semantics

## 2. Anatomy

```text
[Root]
  └── [Children...]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | vertical flow container | stack gap, optional padding |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `gap` | `"sm" \| "md" \| "lg"` | `"md"` | no | semantic vertical spacing |
| `align` | `"start" \| "center" \| "stretch" \| "end"` | `"stretch"` | no | cross-axis alignment |
| `padding` | `"none" \| "sm" \| "md" \| "lg"` | `"none"` | no | interior spacing |
| `asRole` | `string \| null` | `null` | no | optional semantic role, not list semantics by default |

### Controlled And Uncontrolled

- no controlled value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | vertical flow container |

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
- Optional attributes: caller-owned role/labeling when `Stack` acts as a named
  group or region
- Labeling rules: use explicit semantics when the stack is more than visual
  spacing

### Keyboard

| Key | Behavior |
|-----|----------|
| none | keyboard behavior belongs to children or higher-order composites |

### Focus And Announcement

- focus entry: root is not focusable by default
- live-region behavior: none

## 7. Layout

### Sizing

- fills available inline size unless constrained by parent
- block size follows children and parent constraints

### Composition

- parent expectations: any layout or surface container
- child expectations: ordered block children
- resizing rules: gap remains constant regardless of child growth

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `semantic.space.stack.*` | vertical gap |
| Root | `semantic.space.panel.*` or semantic padding roles | optional interior spacing |

## 9. Svelte Notes

- implemented with CSS flex column or equivalent
- semantic list markup should be chosen by higher-order contracts when list
  semantics matter

## 10. GPUI Notes

- implemented with GPUI-native vertical flex/layout idioms
- if a caller applies semantic grouping, GPUI must expose equivalent native
  accessibility grouping rather than treating the stack as visual-only

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] gap and alignment semantics match
- [ ] grouping neutrality matches unless caller opts in
- [ ] focus neutrality matches

### Tier 2: Visual Parity

- [ ] spacing scale matches
- [ ] stretch/start/end alignment looks proportional

### Tier 3: Implementation Freedom

- [ ] CSS flex vs GPUI layout internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: forms, cards, shell sections, detail displays
- future follow-up: add scroll-aware stacked collection rules only if needed

## Next Task

Use `Stack` as the default vertical layout primitive for `g01.011` product
composites and header/body shells.
