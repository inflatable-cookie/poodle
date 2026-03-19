# FilterToolbar

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `FilterToolbar`
- Layer: `composites`
- Summary: a compact control strip for search, filtering, sorting, and result
  summary affordances above lists or grids
- In scope: search and filter control grouping, result summary slot, secondary
  actions, responsive stacking
- Out of scope: result data ownership, domain-specific filter logic, command
  palette behavior

## 2. Anatomy

```text
[Root Toolbar]
  ├── [Primary Controls]
  ├── [Summary] (optional)
  └── [Secondary Actions] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Toolbar | yes | control grouping container | surface, spacing, separator |
| Primary Controls | yes | search/filter/sort controls | gap, wrap |
| Summary | no | result count or active-filter summary | typography, text color |
| Secondary Actions | no | clear, create, export, or related actions | action spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `summaryText` | `string \| null` | `null` | no | visible summary copy |
| `isSticky` | `boolean` | `false` | no | allows sticky browse controls when host supports it |
| `ariaLabel` | `string` | `"Filters"` | no | toolbar/group label |

### Controlled And Uncontrolled

- declarative control-group composite
- filter state is fully host-owned

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| simple | controls only | compact control strip |
| summarized | `summaryText` present | summary region visible |
| no-results summary | host reports zero matches | summary remains textual rather than icon-only |
| sticky | `isSticky=true` | sticky positioning treatment where supported |

### Component States

State table is sufficient.

## 5. Events

No component-owned events beyond child control behavior.

## 6. Accessibility

### Semantics

- Role: toolbar, group, or labeled region depending on content density and
  action posture
- Required attributes: stable accessible label for the control group
- Optional attributes: summary description relationship when helpful
- Labeling rules: the summary supplements the toolbar; it does not replace the
  accessible label

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves through contained controls in DOM or view order |
| toolbar-internal arrows | only where child controls define them |

### Focus And Announcement

- focus entry: the toolbar container is not focusable by default
- focus exit: sticky positioning must not create duplicate focus order
- live-region behavior: summary changes may be announced only when the host
  decides the result update is material; the toolbar itself should not force a
  live region
- GPUI-native accessibility mapping notes: GPUI must preserve labeled group or
  toolbar semantics and logical control order even when layout compacts or wraps

## 7. Layout

### Sizing

- controls may wrap into multiple lines at narrow widths
- summary and actions may stack below controls when constrained

### Composition

- parent expectations: `ListShell`, `GridShell`, browser-style pages, settings
  views with search/filter affordances
- child expectations: search fields, selects, segmented controls, buttons,
  badges, pills
- resizing rules: primary controls remain first in reading and focus order
- host rule: changing query or filters may reset pagination or progressive-load
  windows, but that policy stays host-owned and explicit

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Toolbar | surface, spacing, and separator roles | grouping shell |
| Summary | subdued text roles | result metadata |
| Secondary Actions | action gap roles | trailing controls |
| Sticky posture | surface/elevation roles | persistent browse chrome |

## 9. Svelte Notes

- expected substrate: `Inline`, `Stack`, `Surface`, and foundation controls
- wrapper strategy: sticky behavior is allowed implementation detail when it
  does not change semantics or focus order

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::filter_toolbar`
- implementation-only details: GPUI may realize sticky behavior with native
  layout containers, but must preserve labeled-group semantics and control order

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] labeled-group semantics and control order match
- [ ] summary meaning and placement in the accessibility tree match

### Tier 2: Visual Parity

- [ ] control grouping, spacing, and sticky emphasis use comparable token roles

### Tier 3: Implementation Freedom

- [ ] wrap and sticky mechanics stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| sticky realization may differ | runtime layout systems differ | allowed | keep order and label semantics strict |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### With filters and summary

A toolbar with search, select filter, and result summary:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With filters and summary | `summaryText="Showing 24 of 156 items"`, `ariaLabel="Item filters"`, children: SearchField + Select (status filter with All/Active/Archived/Draft options) | toolbar strip with search input, status dropdown, and summary text |

### With secondary actions

A toolbar with search and a secondary action button:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With secondary actions | `ariaLabel="Project filters"`, children: SearchField; secondary slot with Reset button | toolbar with search input on start and Reset button on end |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings search/filter bars, collection browsers,
  inspector filter strips
- future follow-up: pair with richer query-builder composites if needed later

## Next Task

Use `FilterToolbar` inside `BrowseSearchShell`, `ListShell`, `GridShell`, and
`DataTable` compositions, and keep query/filter execution owned by the host.
