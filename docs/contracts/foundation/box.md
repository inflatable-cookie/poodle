# Box

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Box`
- Layer: `foundation`
- Summary: a neutral layout container for spacing, sizing, and overflow control
  without imposing directional layout behavior
- In scope: padding, width/height constraints, overflow control, semantic role
  opt-in
- Out of scope: directional layout (use Stack/Inline/Grid), scrolling ownership,
  interactive behavior, background/border styling

## 2. Anatomy

```text
[Root .box]  <div>
  └── [Content] (slot)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | neutral layout container | padding, sizing, overflow |
| Content | yes | arbitrary slotted children | none (caller-owned) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `padding` | `SpaceScale` | `"none"` | no | interior spacing via scale tokens |
| `width` | `string \| null` | `null` | no | explicit width (any CSS value) |
| `height` | `string \| null` | `null` | no | explicit height (any CSS value) |
| `minWidth` | `string \| null` | `null` | no | minimum width constraint |
| `minHeight` | `string \| null` | `null` | no | minimum height constraint |
| `overflow` | `OverflowMode` | `"visible"` | no | overflow behavior |
| `asRole` | `string \| null` | `null` | no | explicit semantic role opt-in |
| `ariaLabel` | `string \| null` | `null` | no | accessible name when role is set |

### Shared Types

- `SpaceScale`: `"none" \| "sm" \| "md" \| "lg"`
- `OverflowMode`: `"visible" \| "hidden" \| "auto" \| "scroll"`

### Controlled And Uncontrolled

- display primitive only, no state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | neutral container with no visual styling |

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

- No display override (block-level `<div>`)
- Base `min-width: 0` and `min-height: 0` prevent flex/grid overflow
- Explicit `width`, `height`, `minWidth`, `minHeight` applied as inline styles when set

### Composition

- parent expectations: any layout context (flex, grid, block flow)
- child expectations: arbitrary content
- resizing rules: follows parent constraints and explicit size props

## 8. Token Usage — Exact Values

### Root

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `min-height` | `0` |

### Inline Styles (conditional)

| Property | Condition | Value |
|----------|-----------|-------|
| `padding` | `padding="none"` | `0` |
| `padding` | `padding="sm"` | `var(--flint-space-inline-sm)` |
| `padding` | `padding="md"` | `var(--flint-space-panel-y)` |
| `padding` | `padding="lg"` | `var(--flint-space-panel-x)` |
| `overflow` | always | direct prop value (`visible`, `hidden`, `auto`, `scroll`) |
| `width` | when set | direct prop value |
| `height` | when set | direct prop value |
| `min-width` | when set | direct prop value (overrides base 0) |
| `min-height` | when set | direct prop value (overrides base 0) |

### SpaceScale Token Map

| Scale | Resolved Value |
|-------|---------------|
| `"none"` | `0` |
| `"sm"` | `var(--flint-space-inline-sm)` |
| `"md"` | `var(--flint-space-panel-y)` |
| `"lg"` | `var(--flint-space-panel-x)` |

## 9. Svelte Notes

- Rendered as a `<div>` with class `box`
- All layout properties applied as inline styles via `scaleToSpace` helper
- Slot-based content model
- `role` and `aria-label` attributes set conditionally from props
- No events, no state, no lifecycle hooks

## 10. GPUI Notes

- Expected crate/module surface: `flint_gpui::components::box_container`
- Implemented as a neutral GPUI container element
- SpaceScale mapping must use the same design token values
- When `asRole` is set, GPUI must map it into the native accessibility tree
- Must not become focusable unless a higher-order contract requires it

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] neutral non-interactive semantics match
- [ ] `asRole` opt-in meaning matches
- [ ] `ariaLabel` applied when role is set
- [ ] focus neutrality matches

### Tier 2: Visual Parity

- [ ] padding scale tokens resolve to same values
- [ ] overflow behavior matches across all modes
- [ ] width/height/minWidth/minHeight constraints match
- [ ] base min-width: 0 and min-height: 0 match

### Tier 3: Implementation Freedom

- [ ] rendering internals (div vs GPUI container) stay platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none | n/a | n/a | n/a |

## 13. Specimen Definitions

### Group: Default (no padding)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default Box | `<Box>` with text content | Neutral container with no padding; content flush against edges |

### Group: With padding

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Large padding | `<Box padding="lg">` with text content | Content inset by `lg` space scale on all sides |

### Group: Fixed dimensions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Fixed 12x6rem | `<Box padding="md" width="12rem" height="6rem">` | Box constrained to exactly 12rem wide and 6rem tall with `md` padding |

### Group: Overflow hidden

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Clipped content | `<Box padding="sm" width="10rem" height="3rem" overflow="hidden">` with long text | Box clips overflowing text; content does not escape the 10x3rem boundary |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: all foundation and composite layers, used as neutral
  wrapper throughout the system
- future follow-up: clarify polymorphic element support only if needed
