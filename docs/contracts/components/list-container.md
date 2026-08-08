# ListContainer

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `ListContainer`
- Layer: `composites`
- Summary: a page-level shell for list and grid browse surfaces with header,
  filters, batch actions, host-owned content, and pagination placement —
  manages ready/loading/error/empty state switching with built-in fallback
  treatments and host-overridable state slots
- In scope: page header ownership (title, subtitle, eyebrow, breadcrumbs,
  actions), optional filter region, optional batch-action region,
  ready/loading/error/empty state switching, built-in pagination wiring
  and placement with summary, host-overridable state and pagination slots
- Out of scope: item rendering, query state, domain-specific filters,
  row/card actions, data fetching, sorting logic, selection state, command
  wiring

## 2. Anatomy

```text
[Root Section .list-container]  <section>
  ├── [PageHeader]  PageHeader composite
  │     ├── [Breadcrumbs] (optional, via `breadcrumbs` snippet)
  │     └── [Actions] (optional, via `actions` snippet)
  ├── [ready state: state="ready"]
  │     ├── [Filters .list-container__filters]  <div> (optional)
  │     │     └── (`filters` snippet)
  │     ├── [Batch .list-container__batch]  <div> (optional)
  │     │     └── (`batch` snippet)
  │     ├── [Content .list-container__content]  <div>
  │     │     └── (`children` snippet)
  │     └── [Pagination .list-container__pagination]  <div> (optional)
  │           ├── [PaginationSummary] (optional)
  │           ├── [Pagination]
  │           └── (or `pagination` snippet — full override)
  ├── [loading state: state="loading"]
  │     └── [State .list-container__state]  <div>
  │           ├── (`loading` snippet — override)
  │           └── Callout (tone="pending") — default
  ├── [error state: state="error"]
  │     └── [State .list-container__state]  <div>
  │           ├── (`error` snippet — override)
  │           └── Callout (tone="danger") — default
  └── [empty state: state="empty"]
        └── [State .list-container__state]  <div>
              ├── (`empty` snippet — override)
              └── EmptyState — default
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Section | yes | `<section>` browse shell | gap (stack-lg) |
| PageHeader | yes | title, subtitle, eyebrow, breadcrumbs, actions | delegates to PageHeader contract |
| Filters | no | host-provided filter controls | gap (stack-md) |
| Batch | no | host-provided batch actions / selection strip | gap (stack-md) |
| Content | yes (ready state) | host-owned rows, cards, tables, or grids | gap (stack-md) |
| State | yes (non-ready states) | loading/error/empty replacement surface | gap (stack-md) |
| Pagination | no | built-in pagination summary and controls, or host override | gap (stack-md) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | — | yes | visible page/list title (passed to PageHeader) |
| `subtitle` | `string \| null` | `null` | no | supporting descriptive copy (passed to PageHeader) |
| `eyebrow` | `string \| null` | `null` | no | lightweight meta label above title (passed to PageHeader) |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for root section; defaults to `title` |
| `state` | `"ready" \| "empty" \| "loading" \| "error"` | `"ready"` | no | browse state for content vs fallback region |
| `loadingMessage` | `string \| null` | `"Loading items..."` | no | text for built-in loading Callout |
| `errorTitle` | `string \| null` | `"Unable to load list"` | no | title for built-in error Callout |
| `errorMessage` | `string \| null` | `null` | no | message for built-in error Callout |
| `emptyTitle` | `string \| null` | `"Nothing here yet"` | no | title for built-in EmptyState |
| `emptyMessage` | `string \| null` | `null` | no | message for built-in EmptyState |
| `emptyVariant` | `EmptyStateVariant` | `"neutral"` | no | variant for built-in EmptyState |
| `currentPage` | `number` | `1` | no | active page for built-in pagination |
| `totalPages` | `number` | `1` | no | total page count for built-in pagination |
| `totalItems` | `number \| null` | `null` | no | total items for built-in PaginationSummary |
| `pageSize` | `number \| null` | `null` | no | items per page for built-in PaginationSummary |
| `siblingCount` | `number` | `1` | no | pagination sibling count |
| `paginationAriaLabel` | `string \| null` | `null` | no | accessible label for built-in Pagination; defaults to "List pagination" |
| `showPagination` | `boolean` | `true` | no | whether built-in pagination may render |
| `showPaginationSummary` | `boolean` | `true` | no | whether built-in PaginationSummary may render when total data is available |
| `onPageChange` | `((page: number) => void) \| null` | `null` | no | callback fired when built-in pagination requests a different page |

### Types

```ts
type BrowseState = "ready" | "empty" | "loading" | "error" | "no-results";
// ListContainer accepts: Exclude<BrowseState, "no-results">

type EmptyStateVariant = "neutral" | "search" | "firstRun";
```

### Snippets

| Snippet | Purpose |
|------|---------|
| `children` | host-owned ready-state content |
| `breadcrumbs` | page breadcrumbs passed through to PageHeader |
| `actions` | page-level actions passed through to PageHeader |
| `filters` | filter controls (typically FilterToolbar) |
| `batch` | batch actions / selection strip between filters and content |
| `pagination` | full override for the pagination region |
| `loading` | loading-state override (replaces default Callout) |
| `error` | error-state override (replaces default Callout) |
| `empty` | empty-state override (replaces default EmptyState) |

### Controlled And Uncontrolled

- `state`, pagination values, filters, content, and batch state are fully
  host-controlled
- ListContainer owns no data persistence or internal browse state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"` | header, optional filters/batch, content, and pagination render |
| loading | `state="loading"` | header remains; loading region replaces content (Callout tone="pending" or slot override) |
| error | `state="error"` | header remains; error region replaces content (Callout tone="danger" with announceMode="assertive" or slot override) |
| empty | `state="empty"` | header remains; empty region replaces content (EmptyState or slot override) |
| paginated | `showPagination=true`, `state="ready"`, `totalPages > 1` | pagination region renders |
| summarized | `shouldShowPagination` and `showPaginationSummary=true` and `totalItems` and `pageSize` provided | PaginationSummary renders above Pagination |

### Component States (Derived)

- `shouldShowPagination`: `showPagination && state === "ready" && totalPages > 1`
- `shouldShowPaginationSummary`: `shouldShowPagination && showPaginationSummary && totalItems !== null && pageSize !== null`

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onPageChange` | built-in Pagination requests a different page | `number` | only used when the built-in pagination region is active |

## 6. Accessibility

### Semantics

- Root: `<section>` with `aria-label` set to `ariaLabel ?? title`
- Root: `data-state` attribute reflecting current state
- PageHeader: provides visible heading semantics
- Error Callout: `announceMode="assertive"` for immediate screen reader
  announcement
- Loading Callout: tone="pending" with default Callout semantics
- EmptyState: delegates to EmptyState contract accessibility
- Pagination: `ariaLabel` defaults to `"List pagination"` when not specified

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves through header actions, filter controls, batch actions, content focus targets, and pagination in DOM order |
| pagination keys | delegated to Pagination primitive behavior |

### Focus And Announcement

- The container itself is not focusable by default
- State transitions do not steal focus unless the host chooses to do so
- Loading/error announcements delegated to Callout semantics
- Pagination summary is passive (no live region)

## 7. Layout

### Sizing

- Root: grid layout with `var(--poodle-space-stack-lg)` between major regions
- Filters, batch, content, and state regions each use
  `var(--poodle-space-stack-md)` internal gap
- Pagination region is a grid so summary and controls can separate cleanly

### Composition

- Composes: `Callout`, `Pagination`, `PaginationSummary` from
  `@inflatable-cookie/poodle-svelte`; `EmptyState` and `PageHeader` from
  `@inflatable-cookie/poodle-svelte`
- Parent expectations: list pages, admin browse views, content libraries
- Child expectations: host provides filters, batch strip, and content
- Resizing rules: caller content is fully responsible for row/card/grid
  responsiveness inside the `children` snippet

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-state` | root `<section>` | `"ready"`, `"loading"`, `"error"`, `"empty"` |

### Root `.list-container`

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-space-stack-lg)` |

### Regions `.list-container__filters`, `.list-container__batch`, `.list-container__content`, `.list-container__state`

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-space-stack-md)` |

### Pagination Region `.list-container__pagination`

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-space-stack-md)` |

### Pagination Composition

| Selector | Property | Value |
|----------|----------|-------|
| `.list-container__pagination :global(.pagination-summary)` | width | `100%` |
| `.list-container__pagination :global(.pagination)` | justify-self | `end` |

### Composed Primitives

| Part | Delegates To |
|------|-------------|
| PageHeader | PageHeader contract (composites) |
| Loading Callout | Callout contract (foundation), `tone="pending"`, `message={loadingMessage}` |
| Error Callout | Callout contract (foundation), `tone="danger"`, `title={errorTitle}`, `message={errorMessage}`, `announceMode="assertive"` |
| EmptyState | EmptyState contract (composites), `title={emptyTitle}`, `message={emptyMessage}`, `variant={emptyVariant}` |
| PaginationSummary | PaginationSummary contract (foundation) |
| Pagination | Pagination contract (foundation), `ariaLabel={paginationAriaLabel ?? "List pagination"}` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- Composes `Callout`, `Pagination`, `PaginationSummary` from
  `@inflatable-cookie/poodle-svelte` and `EmptyState`, `PageHeader` from
  local composites
- PageHeader rendering branches on `breadcrumbs` and `actions` snippet
  presence to avoid empty wrapper output
- Filters, batch, and content regions only render in `state="ready"`
- Pagination region renders when `shouldShowPagination` is true or when the
  `pagination` snippet is provided
- Built-in Pagination's `onPageChange` callback is forwarded through the
  local `onPageChange` prop
- PaginationSummary receives `totalItems ?? 0` and `pageSize ?? 1` to
  satisfy non-null requirements

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::list_container`
- Theme access strategy: consume the same semantic spacing roles and delegate
  sub-surfaces to the equivalent GPUI primitives/composites
- GPUI can choose a different internal layout composition so long as the
  contract regions and event meaning match
- Preserve heading hierarchy, state region semantics, and pagination
  affordances

## 10a. Jetstream Notes

- `ListContainer::from_spec(spec, theme).content(...).on_page_change(...)`,
  forwarded to the composed `Pagination` rather than re-implemented, with the
  destination-page payload that implies.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] header, filters, batch, content, and state-region responsibilities match
- [ ] ready/loading/error/empty state switching matches
- [ ] `onPageChange` timing and payload meaning match
- [ ] built-in pagination and summary placement match
- [ ] section labeling: `aria-label` defaults to `title`
- [ ] error Callout uses `announceMode="assertive"`

### Tier 2: Visual Parity

- [ ] region spacing hierarchy matches (stack-lg between regions, stack-md within)
- [ ] pagination region placement and alignment match
- [ ] fallback state treatments match (Callout for loading/error, EmptyState for empty)

### Tier 3: Implementation Freedom

- [ ] caller-owned content remains unconstrained by implementation details
- [ ] internal branching or layout helpers do not leak into the public contract
- [ ] PageHeader slot-forwarding approach stays internal

## 12. Specimen Definitions

### Ready State With Pagination

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Ready state with pagination | `title="Projects"`, `subtitle="Manage your team's projects"`, actions slot with primary "New project" button, filters slot with FilterToolbar, content with list cards, `currentPage=2`, `totalPages=5`, `totalItems=48`, `pageSize=10` | full browse shell with header, filters, content, pagination summary, and pagination controls |

### Loading State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Loading state | `title="Projects"`, `state="loading"` | header remains; pending-tone Callout replaces content area with "Loading items..." message |

### Error State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Error state | `title="Projects"`, `state="error"`, `errorMessage="A network error occurred. Please try again."` | header remains; danger-tone Callout with title "Unable to load list" and error message |

### Empty State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Empty state | `title="Projects"`, `state="empty"`, `emptyMessage="Create your first project to get started."`, `emptyVariant="firstRun"` | header remains; EmptyState with firstRun variant, title, and message |

### With Breadcrumbs And Actions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With breadcrumbs and actions | `title="Team members"`, `eyebrow="Settings"`, breadcrumbs slot, actions slot with Button, content with rows | header shows breadcrumbs, eyebrow, title, and action button above content |
