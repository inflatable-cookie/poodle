# Inline

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Inline`
- Layer: `foundation`
- Summary: a horizontal flex layout primitive for arranging children in a row
  with spacing, alignment, justification, and optional wrapping
- In scope: horizontal flow, gap spacing, cross-axis alignment, main-axis
  justification, wrap control, interior padding
- Out of scope: vertical layout (use Stack), toolbar semantics, roving focus,
  menu/tab behavior

## 2. Anatomy

```text
[Root .inline]  <div>
  └── [Content] (slot)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | horizontal flex container | gap, padding, alignment, justification |
| Content | yes | inline peer children | none (caller-owned) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `gap` | `SpaceScale` | `"md"` | no | horizontal spacing between children |
| `align` | `LayoutAlign` | `"center"` | no | cross-axis (vertical) alignment |
| `justify` | `LayoutJustify` | `"start"` | no | main-axis distribution |
| `wrap` | `boolean` | `false` | no | enables multi-row flow when true |
| `padding` | `SpaceScale` | `"none"` | no | interior spacing |
| `asRole` | `string \| null` | `null` | no | explicit semantic role opt-in |
| `ariaLabel` | `string \| null` | `null` | no | accessible name when role is set |

### Shared Types

- `SpaceScale`: `"none" \| "sm" \| "md" \| "lg"`
- `LayoutAlign`: `"start" \| "end" \| "center" \| "stretch"`
- `LayoutJustify`: `"start" \| "center" \| "end" \| "between"`

### Controlled And Uncontrolled

- display primitive only, no state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | single-row horizontal flow |
| wrapped | `wrap={true}` and children overflow | multi-row horizontal flow |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | layout primitive only |

## 6. Accessibility

### Semantics

- Role: none by default (`<div>`)
- `role`: from `asRole` prop when set
- `aria-label`: from prop, used when `asRole` creates an addressable region

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive, not focusable |

### Focus And Announcement

- Not focusable by default
- No live-region behavior

## 7. Layout

### Sizing

- `display: flex` with default row direction
- Base `min-width: 0` and `min-height: 0` prevent flex/grid overflow
- Inline size follows parent constraints
- Block size grows with child size and wrap behavior

### Composition

- parent expectations: any layout or shell container
- child expectations: inline peer elements (buttons, badges, icons, text)
- resizing rules: gap remains stable; wrapping is explicit via `wrap` prop

## 8. Token Usage — Exact Values

### Root (static CSS)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `min-width` | `0` |
| `min-height` | `0` |

### Inline Styles (conditional)

| Property | Condition | Value |
|----------|-----------|-------|
| `gap` | `gap="none"` | `0` |
| `gap` | `gap="sm"` | `var(--pug-space-inline-sm)` |
| `gap` | `gap="md"` | `var(--pug-space-panel-y)` |
| `gap` | `gap="lg"` | `var(--pug-space-panel-x)` |
| `padding` | `padding="none"` | `0` |
| `padding` | `padding="sm"` | `var(--pug-space-inline-sm)` |
| `padding` | `padding="md"` | `var(--pug-space-panel-y)` |
| `padding` | `padding="lg"` | `var(--pug-space-panel-x)` |
| `align-items` | `align="start"` | `flex-start` |
| `align-items` | `align="end"` | `flex-end` |
| `align-items` | `align="center"` | `center` |
| `align-items` | `align="stretch"` | `stretch` |
| `justify-content` | `justify="start"` | `flex-start` |
| `justify-content` | `justify="center"` | `center` |
| `justify-content` | `justify="end"` | `flex-end` |
| `justify-content` | `justify="between"` | `space-between` |
| `flex-wrap` | `wrap={false}` | `nowrap` |
| `flex-wrap` | `wrap={true}` | `wrap` |

### SpaceScale Token Map

| Scale | Resolved Value |
|-------|---------------|
| `"none"` | `0` |
| `"sm"` | `var(--pug-space-inline-sm)` |
| `"md"` | `var(--pug-space-panel-y)` |
| `"lg"` | `var(--pug-space-panel-x)` |

### LayoutAlign Value Map

| Align | CSS Value |
|-------|-----------|
| `"start"` | `flex-start` |
| `"end"` | `flex-end` |
| `"center"` | `center` |
| `"stretch"` | `stretch` |

### LayoutJustify Value Map

| Justify | CSS Value |
|---------|-----------|
| `"start"` | `flex-start` |
| `"center"` | `center` |
| `"end"` | `flex-end` |
| `"between"` | `space-between` |

## 9. Svelte Notes

- Rendered as a `<div>` with class `inline`
- All layout properties applied as inline styles
- Gap and padding resolved via `scaleToSpace` helper
- Alignment resolved via `alignItemsValue` helper
- Justification resolved via `justifyContentValue` helper
- Wrap resolved to CSS `flex-wrap` value
- Slot-based content model
- `role` and `aria-label` attributes set conditionally from props
- No events, no state, no lifecycle hooks

## 10. GPUI Notes

- Expected crate/module surface: `pug_gpui::components::inline`
- Implemented with GPUI-native horizontal flex layout
- SpaceScale mapping must use the same design token values
- LayoutAlign mapping must produce equivalent cross-axis alignment
- LayoutJustify mapping must produce equivalent main-axis distribution
- Wrap behavior must be supported for multi-row flow
- When `asRole` is set, GPUI must expose equivalent native accessibility grouping
- Must not become focusable unless a higher-order contract requires it

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] horizontal flex-row layout direction matches
- [ ] non-interactive semantics match
- [ ] `asRole` opt-in meaning matches
- [ ] `ariaLabel` applied when role is set
- [ ] focus neutrality matches

### Tier 2: Visual Parity

- [ ] gap scale tokens resolve to same values
- [ ] padding scale tokens resolve to same values
- [ ] align-items mapping matches (start, end, center, stretch)
- [ ] justify-content mapping matches (start, center, end, between)
- [ ] flex-wrap toggling matches (nowrap vs wrap)
- [ ] base min-width: 0 and min-height: 0 match

### Tier 3: Implementation Freedom

- [ ] CSS flex vs GPUI layout internals stay platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none | n/a | n/a | n/a |

## 13. Specimen Definitions

### Group: Default (center-aligned)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Three inline items | `<Inline gap="md">` containing 3 Surface children with `padding="sm" border="subtle"`, including one taller item | Three bordered surfaces in a horizontal row with medium gap, vertically center-aligned (shorter items centered against tallest) |

### Group: Justify: between

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Space-between items | `<Inline gap="md" justify="between">` containing 3 Surface children with `padding="sm" border="subtle"` | Three bordered surfaces distributed across the full width; first pinned left, last pinned right, middle centered between them |

### Group: Wrapping with small gap

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Eight wrapping tags | `<Inline gap="sm" wrap>` containing 8 Surface children with `padding="sm" border="subtle"` | Eight bordered surfaces in a horizontal flow with small gap; items wrap to additional rows when they exceed the container width |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: button groups, header rows, action bars, shell utility
  strips, breadcrumbs
- future follow-up: add bidirectional/layout-direction guidance if required
