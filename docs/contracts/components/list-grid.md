# ListGrid

Status: active
Updated: 2026-04-09

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

## 2. Accessibility

- accessibility-neutral layout primitive with no ARIA roles or landmarks
- grid items are responsible for their own accessibility attributes
- header actions row uses flex layout; action buttons must supply their own
  accessible labels
