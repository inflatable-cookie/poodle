# Detail Section Group

Status: contract
Updated: 2026-06-11

## 1. Purpose

- Component name: `DetailSectionGroup`
- Layer: `composites`
- Summary: responsive grouping container for multiple `DetailSection` blocks,
  handling section-grid layout, density-driven spacing, and max-column caps
- In scope: grid or stack layout for grouped detail sections, responsive column
  collapse, inherited density, semantic region labeling
- Out of scope: section header rendering, row content, page identity, loading or
  error state ownership

## 2. Anatomy

```text
[Root .detail-section-group] <div>
  └── children()
      └── expected child surfaces such as DetailSection
```

| Part | Required | Description |
|------|----------|-------------|
| Root | yes | responsive layout container for grouped detail sections |
| Children | yes | host-owned detail sections or equivalent section blocks |

## 3. Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `density` | `ControlDensity \| null` | `null` | no | explicit density override; otherwise inherits presentation density |
| `layout` | `"grid" \| "stack"` | `"grid"` | no | controls whether sections auto-flow in columns or stack vertically |
| `minColumnWidth` | `string` | `"14rem"` | no | minimum width for each section column in grid mode |
| `itemMinColumnWidth` | `string` | `"12rem"` | no | forwarded to descendant `DetailSection` surfaces via CSS custom property |
| `maxColumns` | `2 \| 3 \| 4 \| 5` | `4` | no | upper bound for auto-fit column behavior |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the grouping region |

## 4. Behavior Rules

- `layout="grid"` uses auto-fit grid columns constrained by `minColumnWidth`
  and `maxColumns`
- `layout="stack"` forces a single column regardless of available width
- narrow containers collapse to one column automatically
- the component does not inject section chrome; it only owns layout

## 5. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| grid | `layout="grid"` | sections flow across responsive columns |
| stack | `layout="stack"` | sections render in one vertical column |
| compact | compact density | tighter inter-section gap |
| comfortable | comfortable density | looser inter-section gap |

## 6. Accessibility

- root element is a plain `<div>`
- `aria-label` applies when callers need a named grouping region
- child sections remain responsible for their own heading and row semantics

## 7. Layout

- root uses CSS grid and container queries
- grouped sections align to the start edge and preserve equal column widths
- descendant `DetailSection` surfaces receive
  `--poodle-detail-section-item-min: itemMinColumnWidth`

## 8. Usage Notes

- use `DetailSectionGroup` when several peer `DetailSection` blocks should share
  one responsive layout decision
- use `DetailShell` when the page also needs header, state, or body-shell
  behavior
- this surface is currently contract-only in preview coverage
