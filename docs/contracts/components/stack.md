# Stack

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Stack`
- Layer: `foundation`
- Summary: a general-purpose flex container supporting both vertical and
  horizontal layout with consistent spacing and cross-axis alignment
- In scope: vertical and horizontal flow, gap spacing, cross-axis alignment,
  main-axis justification, wrapping, interior padding
- Out of scope: grid placement (use Grid), interactive list semantics, scroll
  behavior

## 2. Anatomy

```text
[Root .stack]  <div>
  └── [Content] (children snippet)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | vertical flex container | gap, padding, alignment |
| Content | yes | ordered child content | none (caller-owned) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `direction` | `"column" \| "row"` | `"column"` | no | flex direction (vertical or horizontal) |
| `gap` | `SpaceScale` | `"md"` | no | spacing between children |
| `align` | `LayoutAlign` | direction-aware (`column` → `"stretch"`, `row` → `"center"`) | no | cross-axis alignment; when unset resolves from `direction` |
| `justify` | `LayoutJustify` | `"start"` | no | main-axis justification (always emitted) |
| `wrap` | `boolean` | `false` | no | whether children wrap to next line |
| `padding` | `SpaceScale` | `"none"` | no | interior spacing |
| `width` | `string \| null` | `null` | no | explicit inline width override |
| `height` | `string \| null` | `null` | no | explicit block height override |
| `minWidth` | `string \| null` | `null` | no | min-width override (replaces base `min-width: 0`) |
| `minHeight` | `string \| null` | `null` | no | min-height override (replaces base `min-height: 0`) |
| `overflow` | `OverflowMode` | `"visible"` | no | overflow behavior |
| `asRole` | `string \| null` | `null` | no | explicit semantic role opt-in |
| `ariaLabel` | `string \| null` | `null` | no | accessible name when role is set |
| `class` | `string` | `""` | no | extra class names appended to `poodle-stack` |

### Shared Types

- `SpaceScale`: `"none" \| "sm" \| "md" \| "lg"`
- `LayoutAlign`: `"start" \| "center" \| "end" \| "stretch"`
- `LayoutJustify`: `"start" \| "center" \| "end" \| "between"`
- `OverflowMode`: `"visible" \| "hidden" \| "clip"`

### Controlled And Uncontrolled

- display primitive only, no state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | vertical flow container with gap between children |

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
| `box-sizing` | `border-box` |
| `display` | `flex` |
| `flex-direction` | `column` (default; `row` when `direction="row"`) |
| `min-width` | `0` (overridable by `minWidth` prop) |
| `min-height` | `0` (overridable by `minHeight` prop) |

### Inline Styles (conditional)

| Property | Condition | Value |
|----------|-----------|-------|
| `gap` | `gap="none"` | `0` |
| `gap` | `gap="sm"` | `var(--poodle-space-inline-sm)` |
| `gap` | `gap="md"` | `var(--poodle-space-panel-y)` |
| `gap` | `gap="lg"` | `var(--poodle-space-panel-x)` |
| `padding` | `padding="none"` | `0` |
| `padding` | `padding="sm"` | `var(--poodle-space-inline-sm)` |
| `padding` | `padding="md"` | `var(--poodle-space-panel-y)` |
| `padding` | `padding="lg"` | `var(--poodle-space-panel-x)` |
| `align-items` | `align="start"` | `flex-start` |
| `align-items` | `align="end"` | `flex-end` |
| `align-items` | `align="center"` | `center` |
| `align-items` | `align="stretch"` | `stretch` |
| `justify-content` | `justify="start"` | `flex-start` |
| `justify-content` | `justify="end"` | `flex-end` |
| `justify-content` | `justify="center"` | `center` |
| `justify-content` | `justify="between"` | `space-between` |
| `flex-wrap` | `wrap=true` | `wrap` |
| `flex-wrap` | `wrap=false` | `nowrap` |
| `flex-direction` | `direction="row"` | `row` |
| `width` | `width` set | `{width}` |
| `height` | `height` set | `{height}` |
| `min-width` | `minWidth` set | `{minWidth}` |
| `min-height` | `minHeight` set | `{minHeight}` |
| `overflow` | always | `{overflow}` (`visible` default) |

### SpaceScale Token Map

| Scale | Resolved Value |
|-------|---------------|
| `"none"` | `0` |
| `"sm"` | `var(--poodle-space-inline-sm)` |
| `"md"` | `var(--poodle-space-panel-y)` |
| `"lg"` | `var(--poodle-space-panel-x)` |

### LayoutAlign Value Map

| Align | CSS Value |
|-------|-----------|
| `"start"` | `flex-start` |
| `"end"` | `flex-end` |
| `"center"` | `center` |
| `"stretch"` | `stretch` |

## 9. Svelte Notes

- Rendered as a `<div>` with class `poodle-stack` (plus any `class` prop appended)
- All layout properties applied as inline styles
- Gap and padding resolved via `scaleToSpace` helper
- Alignment resolved via `alignItemsValue` helper; `align` defaults are direction-aware
  (`column` → `stretch`, `row` → `center`) when the prop is unset
- `justify-content` is always emitted (`start` default) via `justifyContentValue`
- `overflow` always emitted; `width`/`height`/`minWidth`/`minHeight` emitted only when set
- Slot-based content model
- `role` and `aria-label` attributes set conditionally from props
- No events, no state, no lifecycle hooks

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::components::stack`
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
