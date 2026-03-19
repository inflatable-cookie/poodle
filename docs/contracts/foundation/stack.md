# Stack

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Stack`
- Layer: `foundation`
- Summary: a vertical flex layout primitive for arranging children in a column
  with consistent spacing and cross-axis alignment
- In scope: vertical flow, gap spacing, cross-axis alignment, interior padding
- Out of scope: horizontal layout (use Inline), grid placement (use Grid),
  interactive list semantics, scroll behavior

## 2. Anatomy

```text
[Root .stack]  <div>
  └── [Content] (slot)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | vertical flex container | gap, padding, alignment |
| Content | yes | ordered block children | none (caller-owned) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `gap` | `SpaceScale` | `"md"` | no | vertical spacing between children |
| `align` | `LayoutAlign` | `"stretch"` | no | cross-axis (horizontal) alignment |
| `padding` | `SpaceScale` | `"none"` | no | interior spacing |
| `asRole` | `string \| null` | `null` | no | explicit semantic role opt-in |
| `ariaLabel` | `string \| null` | `null` | no | accessible name when role is set |

### Shared Types

- `SpaceScale`: `"none" \| "sm" \| "md" \| "lg"`
- `LayoutAlign`: `"start" \| "end" \| "center" \| "stretch"`

### Controlled And Uncontrolled

- display primitive only, no state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | vertical flow container with gap between children |

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

- `display: flex` with `flex-direction: column`
- Base `min-width: 0` and `min-height: 0` prevent flex/grid overflow
- Fills available inline size when `align="stretch"` (default)
- Block size follows children

### Composition

- parent expectations: any layout or surface container
- child expectations: ordered block children
- resizing rules: gap remains constant regardless of child growth

## 8. Token Usage — Exact Values

### Root (static CSS)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
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

## 9. Svelte Notes

- Rendered as a `<div>` with class `stack`
- All layout properties applied as inline styles
- Gap and padding resolved via `scaleToSpace` helper
- Alignment resolved via `alignItemsValue` helper
- Slot-based content model
- `role` and `aria-label` attributes set conditionally from props
- No events, no state, no lifecycle hooks

## 10. GPUI Notes

- Expected crate/module surface: `pug_gpui::components::stack`
- Implemented with GPUI-native vertical flex layout
- SpaceScale mapping must use the same design token values
- LayoutAlign mapping must produce equivalent cross-axis alignment
- When `asRole` is set, GPUI must expose equivalent native accessibility grouping
- Must not become focusable unless a higher-order contract requires it

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] vertical flex-column layout direction matches
- [ ] non-interactive semantics match
- [ ] `asRole` opt-in meaning matches
- [ ] `ariaLabel` applied when role is set
- [ ] focus neutrality matches

### Tier 2: Visual Parity

- [ ] gap scale tokens resolve to same values
- [ ] padding scale tokens resolve to same values
- [ ] align-items mapping matches (start, end, center, stretch)
- [ ] base min-width: 0 and min-height: 0 match

### Tier 3: Implementation Freedom

- [ ] CSS flex vs GPUI layout internals stay platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none | n/a | n/a | n/a |

## 13. Specimen Definitions

### Group: Default (md gap)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Three stacked items | `<Stack gap="md">` containing 3 Surface children with `padding="sm" border="subtle"` | Three bordered surfaces arranged vertically with medium gap between them |

### Group: Large gap with center alignment

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Center-aligned pair | `<Stack gap="lg" align="center">` containing 2 Surface children | Two bordered surfaces stacked vertically with large gap, horizontally centered (not stretched to full width) |

### Group: Small gap, compact

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Four compact items | `<Stack gap="sm">` containing 4 Surface children with `padding="sm" border="subtle"` | Four bordered surfaces stacked tightly with small gap between them |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: forms, cards, shell sections, detail displays, panels
- future follow-up: add scroll-aware stacked collection rules only if needed
