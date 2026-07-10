# Grid

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Grid`
- Layer: `foundation`
- Summary: a two-dimensional CSS grid layout primitive for placing children into
  column and row tracks with consistent spacing
- In scope: column/row track definitions, gap spacing, interior padding
- Out of scope: data-grid semantics, keyboard navigation, cell selection,
  responsive breakpoint logic

## 2. Anatomy

```text
[Root .poodle-grid]  <div>
  └── [Content] (children snippet)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | CSS grid container | columns, rows, gap, padding |
| Content | yes | grid-placed children | none (caller-owned) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `columns` | `string` | `"1fr"` | no | CSS grid-template-columns value |
| `rows` | `string \| null` | `null` | no | CSS grid-template-rows value (omitted when null) |
| `gap` | `SpaceScale` | `"md"` | no | spacing between grid cells |
| `padding` | `SpaceScale` | `"none"` | no | interior spacing |
| `asRole` | `string \| null` | `null` | no | explicit semantic role opt-in |
| `ariaLabel` | `string \| null` | `null` | no | accessible name when role is set |

### Shared Types

- `SpaceScale`: `"none" \| "sm" \| "md" \| "lg"`

### Controlled And Uncontrolled

- display primitive only, no state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | grid placement container |

### Component States

No internal state.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | layout primitive only |

## 6. Accessibility

### Semantics

- Role: none by default (`<div>`)
- `role`: from `asRole` prop when set
- `aria-label`: from prop, used when `asRole` creates an addressable region
- Important: do not assume ARIA `grid` role for visual grid layout

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive, not focusable |

### Focus And Announcement

- Not focusable by default
- No live-region behavior

## 7. Layout

### Sizing

- `display: grid`
- Base `min-width: 0` and `min-height: 0` prevent overflow
- Track sizing is caller-defined through `columns` and `rows` props
- Accepts any valid CSS grid-template value (e.g., `"1fr 1fr"`, `"repeat(3, 1fr)"`, `"200px auto 1fr"`)

### Composition

- parent expectations: any sizing context
- child expectations: direct grid-placed items
- resizing rules: placement follows track definitions; gap remains constant

## 8. Token Usage — Exact Values

### Root (static CSS)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `min-width` | `0` |
| `min-height` | `0` |

### Inline Styles (conditional)

| Property | Condition | Value |
|----------|-----------|-------|
| `grid-template-columns` | always | direct `columns` prop value |
| `grid-template-rows` | when `rows` is set | direct `rows` prop value |
| `gap` | `gap="none"` | `0` |
| `gap` | `gap="sm"` | `var(--poodle-space-inline-sm)` |
| `gap` | `gap="md"` | `var(--poodle-space-panel-y)` |
| `gap` | `gap="lg"` | `var(--poodle-space-panel-x)` |
| `padding` | `padding="none"` | `0` |
| `padding` | `padding="sm"` | `var(--poodle-space-inline-sm)` |
| `padding` | `padding="md"` | `var(--poodle-space-panel-y)` |
| `padding` | `padding="lg"` | `var(--poodle-space-panel-x)` |

### SpaceScale Token Map

| Scale | Resolved Value |
|-------|---------------|
| `"none"` | `0` |
| `"sm"` | `var(--poodle-space-inline-sm)` |
| `"md"` | `var(--poodle-space-panel-y)` |
| `"lg"` | `var(--poodle-space-panel-x)` |

## 9. Svelte Notes

- Rendered as a `<div>` with class `poodle-grid`
- All layout properties applied as inline styles
- Gap and padding resolved via `scaleToSpace` helper
- `grid-template-columns` always set from `columns` prop
- `grid-template-rows` only set when `rows` prop is non-null
- Slot-based content model
- `role` and `aria-label` attributes set conditionally from props
- No events, no state, no lifecycle hooks

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::components::grid`
- GPUI may need custom placement helpers to emulate CSS grid track definitions
- `columns` and `rows` string props must be parsed into equivalent track sizes
- SpaceScale mapping must use the same design token values
- When `asRole` is set, GPUI must expose equivalent native accessibility grouping
- Must not impose ARIA `grid` role for visual-only grid layout
- Must not become focusable unless a higher-order contract requires it

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] grid layout semantics match (visual-only, no ARIA grid role)
- [ ] non-interactive semantics match
- [ ] `asRole` opt-in meaning matches
- [ ] `ariaLabel` applied when role is set
- [ ] focus neutrality matches

### Tier 2: Visual Parity

- [ ] gap scale tokens resolve to same values
- [ ] padding scale tokens resolve to same values
- [ ] column track definitions produce equivalent layout
- [ ] row track definitions produce equivalent layout when set
- [ ] base min-width: 0 and min-height: 0 match

### Tier 3: Implementation Freedom

- [ ] CSS grid vs GPUI placement internals stay platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none | n/a | n/a | n/a |

## 13. Specimen Definitions

### Group: Three columns

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Equal thirds | `<Grid columns="1fr 1fr 1fr" gap="md">` containing 3 Surface children with `padding="md" border="subtle"` | Three equal-width bordered surfaces in a single row with medium gap |

### Group: Mixed column widths

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Sidebar + main | `<Grid columns="1fr 2fr" gap="md">` containing 2 Surface children with `padding="md" border="subtle"` | Two bordered surfaces in a row; first takes 1/3 width, second takes 2/3 width, separated by medium gap |

### Group: Auto-fit responsive

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Five auto-fit items | `<Grid columns="repeat(auto-fit, minmax(8rem, 1fr))" gap="sm">` containing 5 Surface children with `padding="sm" border="subtle"` | Five bordered surfaces that auto-wrap into rows; each cell is at least 8rem wide, with small gap between cells |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: card grids, stat grids, shell content areas, NavCardGrid
- future follow-up: separate `DataGrid` contract if interactive grid semantics
  become required
