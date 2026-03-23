# Spacer

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Spacer`
- Layer: `foundation`
- Summary: a flex-style expansion primitive for distributing remaining space
  within stack or inline layouts
- In scope: consuming remaining space, weighted distribution between siblings,
  optional minimum size reservation
- Out of scope: arbitrary margin hacks, semantic grouping, focus behavior,
  visible decoration

## 2. Anatomy

```text
[Root .spacer]  <div aria-hidden="true">
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | invisible expansion node | none |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `grow` | `number` | `1` | no | relative flex growth weight |
| `minSize` | `string \| null` | `null` | no | optional lower bound applied to both min-width and min-height |

### Controlled And Uncontrolled

- no controlled value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | invisible expansion node, consumes space |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | layout primitive only |

## 6. Accessibility

### Semantics

- Role: none (presentation only)
- Required attributes: `aria-hidden="true"` on root
- Optional attributes: none
- Labeling rules: Spacer must never expose itself to the accessibility tree;
  it is purely decorative layout scaffolding

### Keyboard

| Key | Behavior |
|-----|----------|
| none | no keyboard behavior |

### Focus And Announcement

- focus entry: never focusable
- focus exit: n/a
- live-region behavior: none

## 7. Layout

### Sizing

- consumes available space based on `grow` weight relative to sibling spacers
- `minSize` reserves a minimum dimension without implying visible content
- no maximum size constraint

### Composition

- parent expectations: flex-based layout container (Stack, Inline, toolbar row)
- child expectations: none (no children)
- resizing rules: grows and shrinks with siblings according to flex layout
  engine; collapses to zero when parent has no remaining space (unless minSize
  is set)

## 8. Token Usage — Exact Values

### Root (.spacer)

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `min-height` | `0` |

### Inline styles (applied on root)

| Property | Value |
|----------|-------|
| `flex` | `{grow} 1 0%` |
| `min-width` | `{minSize}` (only when minSize is not null) |
| `min-height` | `{minSize}` (only when minSize is not null) |

### Attributes

| Attribute | Value |
|-----------|-------|
| `aria-hidden` | `"true"` |

## 9. Svelte Notes

- rendered as a `<div>` element with `aria-hidden="true"`
- flex shorthand `{grow} 1 0%` means: grow at weight `grow`, shrink at weight
  1, start from 0% basis
- CSS class `.spacer` resets min-width and min-height to 0 so the element
  can collapse fully in constrained layouts
- when `minSize` is provided, inline styles override both min-width and
  min-height to reserve space along whichever axis the parent layout uses
- no slots, no children

## 10. GPUI Notes

- expected crate/module surface: `flint_gpui::components::spacer`
- implemented as a weighted empty layout node using GPUI's flex model
- `grow` maps to flex-grow weight
- `minSize` maps to min-size constraint on the layout node
- must not appear as a named or focusable accessibility node

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] remains accessibility-neutral in both runtimes (aria-hidden or equivalent)
- [ ] growth weight semantics match
- [ ] minSize reservation semantics match

### Tier 2: Visual Parity

- [ ] space distribution feels equivalent across runtimes
- [ ] collapse behavior under constrained parents matches

### Tier 3: Implementation Freedom

- [ ] DOM flex item vs GPUI layout weight stays internal
- [ ] inline style application vs GPUI layout API stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none | n/a | n/a | n/a |

## 13. Specimen Definitions

### Group: Push items apart

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Two items pushed apart | `<Inline gap="md">` containing a Surface ("Logo"), `<Spacer />`, and a Surface ("Sign in") | First surface pinned to the left edge, second surface pinned to the right edge, with spacer consuming all remaining space between them |

### Group: Between three items

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Three items with spacers | `<Inline gap="md">` containing Surface ("Left"), `<Spacer />`, Surface ("Center"), `<Spacer />`, Surface ("Right") | Three surfaces evenly distributed across the row; two spacers split remaining space equally, pushing items to left, center, and right positions |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: shell headers, toolbar rows, inline layout groups,
  panel headers
- future follow-up: none expected
