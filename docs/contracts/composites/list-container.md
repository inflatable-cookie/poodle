# ListContainer

Status: seed contract
Updated: 2026-03-24

## 1. Purpose

- Component name: `ListContainer`
- Layer: `composites`
- Summary: a page-level shell for list and grid browse surfaces with header,
  filters, batch actions, host-owned content, and pagination placement
- In scope: page header ownership, optional breadcrumbs/actions, filter region,
  batch-action region, ready/loading/error/empty states, built-in pagination
  wiring and placement
- Out of scope: item rendering, query state, domain-specific filters, row/card
  actions, data fetching, sorting logic, selection state, and command wiring

## 2. Anatomy

```text
[Root Section]
  ├── [PageHeader]
  │     ├── [Breadcrumbs] (optional)
  │     └── [Actions] (optional)
  ├── [Filters Region] (optional)
  ├── [Batch Region] (optional)
  ├── [Content Region | State Region]
  └── [Pagination Region] (optional)
        ├── [PaginationSummary] (optional)
        └── [Pagination]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Section | yes | top-level browse shell | stack spacing |
| PageHeader | yes | title, subtitle, eyebrow, breadcrumbs, actions | delegates to PageHeader |
| Filters Region | no | host-provided filter controls, typically `FilterToolbar` | gap |
| Batch Region | no | host-provided batch actions and selection summary | gap |
| Content Region | yes in ready state | host-owned rows, cards, tables, or grids | gap |
| State Region | yes in non-ready states | loading/error/empty replacement surface | gap |
| Pagination Region | no | built-in pagination summary and pagination controls, or host override | gap, alignment |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | none | yes | visible page/list title |
| `subtitle` | `string \| null` | `null` | no | supporting descriptive copy |
| `eyebrow` | `string \| null` | `null` | no | lightweight meta label above title |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the root section |
| `state` | `"ready" \| "empty" \| "loading" \| "error"` | `"ready"` | no | browse state for content vs fallback region |
| `loadingMessage` | `string \| null` | `"Loading items..."` | no | default loading callout text |
| `errorTitle` | `string \| null` | `"Unable to load list"` | no | default error callout title |
| `errorMessage` | `string \| null` | `null` | no | default error callout message |
| `emptyTitle` | `string \| null` | `"Nothing here yet"` | no | default empty state title |
| `emptyMessage` | `string \| null` | `null` | no | default empty state message |
| `emptyVariant` | `EmptyStateVariant` | `"neutral"` | no | default empty-state variant |
| `currentPage` | `number` | `1` | no | active page for built-in pagination |
| `totalPages` | `number` | `1` | no | total page count for built-in pagination |
| `totalItems` | `number \| null` | `null` | no | total items for built-in summary |
| `pageSize` | `number \| null` | `null` | no | items per page for built-in summary |
| `siblingCount` | `number` | `1` | no | pagination sibling count |
| `paginationAriaLabel` | `string \| null` | `null` | no | accessible label for built-in pagination |
| `showPagination` | `boolean` | `true` | no | whether built-in pagination may render |
| `showPaginationSummary` | `boolean` | `true` | no | whether built-in summary may render when total data is available |

### Slots

| Slot | Purpose |
|------|---------|
| default | host-owned ready-state content |
| `breadcrumbs` | page breadcrumbs passed through to `PageHeader` |
| `actions` | page-level actions passed through to `PageHeader` |
| `filters` | filter controls; expected to use `FilterToolbar` or equivalent composition |
| `batch` | batch actions / selection strip between filters and content |
| `pagination` | full override for the pagination region |
| `loading` | loading-state override |
| `error` | error-state override |
| `empty` | empty-state override |

### Controlled And Uncontrolled

- `state`, pagination values, filters, content, and batch state are fully
  host-controlled
- `ListContainer` owns no data persistence or internal browse state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"` | header, optional filters/batch, content, and pagination render |
| loading | `state="loading"` | header remains, loading region replaces content |
| error | `state="error"` | header remains, error region replaces content |
| empty | `state="empty"` | header remains, empty region replaces content |
| paginated | `showPagination=true`, `totalPages > 1` | pagination region renders |
| summarized | `showPaginationSummary=true`, `totalItems` and `pageSize` provided | summary text renders above pagination |

### Component States

State table is sufficient.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `pageChange` | built-in pagination requests a different page | `{ page: number }` | only emitted when using built-in pagination region |

## 6. Accessibility

### Semantics

- Role: labeled `section` for the browse shell
- Required attributes: visible heading semantics from `PageHeader`
- Optional attributes: `aria-label` on the root section when the title is not
  sufficient or the region needs explicit labeling
- Labeling rules: the page title remains the primary visible heading; filter and
  pagination controls keep their own labels

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves through header actions, filter controls, batch actions, content focus targets, and pagination in DOM order |
| pagination keys | delegated to `Pagination` primitive behavior |

### Focus And Announcement

- focus entry: the container itself is not focusable by default
- focus exit: state transitions must not steal focus unless the host chooses to
  do so
- live-region behavior: built-in summary is passive; loading/error/empty
  announcements are delegated to `Callout` / `EmptyState`
- GPUI-native accessibility mapping notes: preserve heading hierarchy, state
  region semantics, and pagination affordances without collapsing the whole
  browse shell into one generic group

## 7. Layout

### Sizing

- root layout is a vertical stack with `var(--poodle-space-stack-lg)` between
  major regions
- filters, batch, content, and state regions each use
  `var(--poodle-space-stack-md)` internally
- pagination region is a vertical stack so summary and controls can separate
  cleanly on narrow widths

### Composition

- parent expectations: list pages, operational queues, admin browse views,
  content libraries, and other generic browse shells
- child expectations: `PageHeader`, `FilterToolbar`, `PaginationSummary`,
  `Pagination`, `Callout`, and `EmptyState` are the preferred Poodle-owned
  building blocks
- resizing rules: caller content remains fully responsible for row/card/grid
  responsiveness inside the content slot
- composition rule: use `ListContainer` for page-shell structure, not as a
  replacement for row rendering or data orchestration

## 8. Token Usage And Precise CSS

### Root `.list-container`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-lg)` |

### Regions `.list-container__filters`, `.list-container__batch`, `.list-container__content`, `.list-container__state`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-md)` |

### Pagination Region `.list-container__pagination`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-md)` |

### Pagination Region Composition

| Selector | Property | Value |
|----------|----------|-------|
| `.list-container__pagination :global(.pagination-summary)` | `width` | `100%` |
| `.list-container__pagination :global(.pagination)` | `justify-self` | `start` |

## 9. Svelte Notes

- expected substrate: `PageHeader`, `Callout`, `EmptyState`,
  `PaginationSummary`, and `Pagination`
- wrapper strategy: slots provide caller-owned filters, actions, batch strip,
  and ready-state content without leaking Underlay-specific dependencies
- implementation-only details: Svelte branches header rendering to avoid empty
  slot wrappers while still supporting optional `breadcrumbs` and `actions`
  slots

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::list_container`
- theme access strategy: consume the same semantic spacing roles and delegate
  sub-surfaces to the equivalent GPUI primitives/composites
- implementation-only details: GPUI can choose a different internal layout
  composition so long as the contract regions and event meaning match

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] header, filters, batch, content, and state-region responsibilities match
- [ ] ready/loading/error/empty state switching matches
- [ ] `pageChange` timing and payload meaning match
- [ ] built-in pagination summary and pagination placement match
- [ ] section labeling and heading semantics match

### Tier 2: Visual Parity

- [ ] region spacing hierarchy matches
- [ ] pagination region placement and alignment match
- [ ] fallback state treatments match within platform limits

### Tier 3: Implementation Freedom

- [ ] caller-owned content remains unconstrained by implementation details
- [ ] internal branching or layout helpers do not leak into the public contract

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none | | | |

## 13. Approval And Adoption Notes

- contract status: `implemented`
- approvers: pending
- downstream adopters: `underlay-reference/acme-admin` review route first
- future follow-up: decide whether a dedicated batch-action strip should remain
  caller-owned composition or become a separate Poodle composite
