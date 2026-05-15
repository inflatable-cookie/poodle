# PaginationSummary

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `PaginationSummary`
- Layer: `foundation`
- Summary: a compact read-only pagination summary that displays the current
  item range and total page count
- In scope: "Showing X-Y of Z" summary, total page count in accessible copy,
  live-region announcement of summary changes
- Out of scope: previous/next controls, page number links, items-per-page
  selector, infinite scroll, cursor-based pagination

## 2. Anatomy

```text
[Root .pagination-summary]  <div role="group" aria-label="Pagination">
  └── [Copy .pagination-summary__copy]  <p>
      └── "Showing {from}-{to} of {total}"
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | flex container with summary copy and live-region label | flex layout, gap |
| Copy | yes | summary text showing item range | color, typography |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `currentPage` | `number` | `1` | no | current page number (1-indexed) |
| `totalPages` | `number` | `1` | no | total number of pages |
| `totalItems` | `number` | `0` | no | total item count for summary text |
| `pageSize` | `number` | `5` | no | items per page for range calculation |

### Controlled And Uncontrolled

- Summary state is fully derived from controlled props
- The component is read-only and exposes no callback or event surface

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | `totalItems > 0` | summary shows current item range |
| empty | `totalItems=0` | summary shows `0-0 of 0` |
| single page | `totalPages=1` | ARIA label still includes total page count |

## 5. Callbacks

None.

## 6. Accessibility

### Semantics

- Root: `aria-live="polite"` and an `aria-label` that includes item range and
  total page count
- Summary text: live region for page change announcements

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | component is not focusable; host focus behavior is unaffected |

### Focus And Announcement

- page change: summary text updated, announced via live region

## 7. Layout

### Sizing

- Root: flex row, space-between, wraps
- Copy: auto-width

### Composition

- parent expectations: below list/table views, footer regions
- child expectations: none (self-contained)
- resizing: fills parent width, wraps on narrow viewports

## 8. Token Usage — Exact Values

### Root

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `justify-content` | `space-between` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-md)` |

### Copy

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |

## 9. Svelte Notes

- Summary text computed: `from = (currentPage - 1) * pageSize + 1`,
  `to = Math.min(currentPage * pageSize, totalItems)`
- Display: "Showing {from}-{to} of {totalItems}"
- ARIA label also includes total page count: "Showing X-Y of Z across N pages"

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::pagination_summary`
- Spec struct: `PaginationSummarySpec` in primitives crate
- Component struct: `PoodlePaginationSummary` in components crate
- Range calculation is pure function
- No interactive callback surface; this is a passive companion to `Pagination`

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] currentPage, totalPages, totalItems, pageSize produce correct summary
- [ ] range calculation matches (from/to/total)

### Tier 2: Visual Parity

- [ ] summary typography matches
- [ ] layout spacing matches

### Tier 3: Implementation Freedom

- [ ] live region announcement method is platform-owned
- [ ] exact live-region phrasing may be platform-owned as long as meaning matches

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Live region announcement | GPUI may use different accessibility announcement method | allowed | same functional result |

## 13. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `currentPage=1`, `totalPages=8`, `totalItems=156`, `pageSize=20` | Summary text showing item range (e.g., "Showing 1-20 of 156") with live-region label including total page count |

### Single Page

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Single page | `currentPage=1`, `totalPages=1`, `totalItems=12`, `pageSize=20` | Summary text showing full range (e.g., "Showing 1-12 of 12") with single-page ARIA copy |

### Large Dataset

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Large dataset | `currentPage=5`, `totalPages=50`, `totalItems=1000`, `pageSize=20` | Summary text showing mid-range items (e.g., "Showing 81-100 of 1000") with total page count included in ARIA copy |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: list views, table views, search results, data browsers
- future follow-up: items-per-page selector, jump-to-page input
