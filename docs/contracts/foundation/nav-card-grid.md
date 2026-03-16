# NavCardGrid

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `NavCardGrid`
- Layer: `foundation`
- Summary: a responsive grid layout container for arranging NavCard components
  in uniform columns
- In scope: configurable column count (1-4), responsive collapse to single
  column, gap spacing
- Out of scope: masonry layouts, infinite scroll, card sorting/filtering,
  non-NavCard children

## 2. Anatomy

```text
[Root .nav-card-grid]  <nav>
  └── [Slot: default]  (NavCard children)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | CSS grid container with responsive columns | grid-template-columns, gap |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `columns` | `1 \| 2 \| 3 \| 4` | `2` | no | number of grid columns |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for navigation landmark |

### Slots

| Slot | Purpose |
|------|---------|
| default | NavCard children |

### Controlled And Uncontrolled

- Layout primitive only; no internal state.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | grid layout with configured column count |
| responsive | viewport <= 640px | single-column layout |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| — | — | — | NavCardGrid emits no events; delegates to NavCard children |

## 6. Accessibility

### Semantics

- Root: `<nav>` element for navigation landmark
- `aria-label`: from prop when provided
- Children expected to be NavCard components with proper link/button semantics

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves focus between NavCard children |

### Focus And Announcement

- Navigation landmark announced by screen readers
- Individual NavCard focus handled by NavCard component

## 7. Layout

### Sizing

- Default: `repeat(columns, 1fr)` grid
- Responsive (max-width: 640px): `1fr` single column
- Gap: `0.75rem` between cards

### Composition

- parent expectations: navigation pages, settings hubs, documentation indexes
- child expectations: NavCard components
- resizing: fills parent width, columns auto-size equally

## 8. Token Usage — Exact Values

### Root

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `repeat(var(--columns, 2), 1fr)` |
| `gap` | `0.75rem` |

### CSS variable

| Var | Value |
|-----|-------|
| `--columns` | set via `style` attribute from `columns` prop |

### Responsive (max-width: 640px)

| Property | Value |
|----------|-------|
| `grid-template-columns` | `1fr` |

## 9. Svelte Notes

- `--columns` CSS variable set via inline `style` attribute: `style="--columns: {columns}"`
- `<nav>` element with optional `aria-label`
- Responsive handled via CSS `@media (max-width: 640px)`
- No data attributes needed for this simple layout component

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::components::nav_card_grid`
- Spec struct: `NavCardGridSpec` in primitives crate
- Component struct: `PugNavCardGrid` in components crate
- Grid layout maps to GPUI's layout system with equal-width columns
- Responsive breakpoint uses GPUI layout measurements for available width
- Navigation landmark semantics may map to GPUI's accessibility tree

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] columns prop produces correct number of equal-width columns
- [ ] responsive collapse to single column matches

### Tier 2: Visual Parity

- [ ] gap spacing matches (0.75rem)
- [ ] column widths are equal (1fr each)
- [ ] responsive breakpoint matches (640px)

### Tier 3: Implementation Freedom

- [ ] responsive detection method is platform-owned
- [ ] CSS variable vs direct style is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Responsive breakpoint detection | GPUI may use layout-based detection vs CSS media query | allowed | same visual result |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: navigation panels, settings hubs, documentation indexes
- future follow-up: auto-fit columns based on available width
