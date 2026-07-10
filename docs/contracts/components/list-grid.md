# ListGrid

Status: active
Updated: 2026-07-10

## 1. Purpose

- Component name: `ListGrid`
- Layer: `foundation`
- Summary: a responsive CSS-grid layout primitive for card and tile
  collections that adapts column count based on available width, with an
  optional header actions row
- In scope: auto-fill grid columns with configurable minimum item width,
  configurable gap, compact variant with single-column stacking, optional
  header slot for actions
- Out of scope: data fetching, selection state, pagination, empty-state
  display, virtual scrolling

## Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `variant` | `"default" \| "compact"` | `"default"` | no | Compact uses single-column layout |
| `minItemWidth` | `number \| string \| null` | `null` | no | Minimum column width; numbers treated as `em`, strings used as-is; defaults to `360px` |
| `maxColumns` | `number \| null` | `3` | no | Upper bound on auto-fill columns (default variant only); `null` removes the cap. The value is floored and clamped to a minimum of `1` |
| `gap` | `number \| string \| null` | `null` | no | Grid gap; numbers treated as `px`, strings used as-is; defaults to `1.25rem` (default) or `0.5rem` (compact) |
| `class` | `string` | `""` | no | Additional CSS class |
| `actions` | `Snippet` | -- | no | Optional header actions row |
| `children` | `Snippet` | -- | no | Grid item content |

## Anatomy

```text
[Root .list-grid]  <div>
  ├── [Header .list-grid__header]  <div> (optional, when actions snippet provided)
  │     └── (slot: actions)
  └── [Content .list-grid__content]  <div style="display: grid">
        └── (slot: children)
```

## Layout Behavior

- Default variant: `grid-template-columns` uses `repeat(auto-fill, minmax(…, 1fr))`.
  - With `maxColumns = null`, the track floor is `min(minItemWidth, 100%)` — columns grow purely by available width.
  - With a `maxColumns` cap (default `3`), the track floor becomes
    `min(100%, max(minItemWidth, calc((100% - (maxColumns - 1) * gap) / maxColumns)))`, so the grid never exceeds `maxColumns` columns even on wide viewports while still collapsing on narrow ones.
- Compact variant: single column (`1fr`); `maxColumns` does not apply.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 2. Accessibility

- accessibility-neutral layout primitive with no ARIA roles or landmarks
- grid items are responsible for their own accessibility attributes
- header actions row uses flex layout; action buttons must supply their own
  accessible labels
