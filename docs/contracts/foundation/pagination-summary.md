# PaginationSummary

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `PaginationSummary`
- Layer: `foundation`
- Summary: a compact pagination control displaying page position summary text
  and previous/next navigation buttons
- In scope: "Showing X-Y of Z" summary, "Page X of Y" display, previous and
  next buttons, disabled state at boundaries
- Out of scope: page number links, items-per-page selector, infinite scroll,
  cursor-based pagination (see Pagination for full-featured control)

## 2. Anatomy

```text
[Root .pagination-summary]  <div role="group" aria-label="Pagination">
  ├── [Copy .pagination-summary__copy]  <p>
  │   └── "Showing {from}-{to} of {total}"
  └── [Actions .pagination-summary__actions]  <div>
      ├── [Previous .pagination-summary__button]  <button>
      ├── [Page indicator]  "Page {current} of {total}"
      └── [Next .pagination-summary__button]  <button>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | flex container with space-between alignment | flex layout, gap |
| Copy | yes | summary text showing item range | color, typography |
| Actions | yes | navigation button group with page indicator | flex, gap |
| Button | yes | previous/next navigation trigger | height, padding, border, radius, background, color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `currentPage` | `number` | `1` | no | current page number (1-indexed) |
| `totalPages` | `number` | `1` | no | total number of pages |
| `totalItems` | `number` | `0` | no | total item count for summary text |
| `pageSize` | `number` | `5` | no | items per page for range calculation |

### Controlled And Uncontrolled

- Page state is controlled via `currentPage` prop
- Changes dispatched via `pageChange` event; parent updates prop

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | middle page | both buttons enabled |
| first page | `currentPage=1` | previous button disabled |
| last page | `currentPage=totalPages` | next button disabled |
| single page | `totalPages=1` | both buttons disabled |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `pageChange` | previous or next button clicked | `{page: number}` | page number of target page |

## 6. Accessibility

### Semantics

- Root: `role="group"`, `aria-label="Pagination"`
- Previous button: uses text content "Previous" (no aria-label)
- Next button: uses text content "Next" (no aria-label)
- Disabled buttons: `disabled` attribute set
- Summary text: live region for page change announcements

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` | activates focused button |
| `Tab` | moves focus between buttons |

### Focus And Announcement

- focus entry: first enabled button receives focus
- page change: summary text updated, announced via live region

## 7. Layout

### Sizing

- Root: flex row, space-between, wraps
- Buttons: control-height, horizontal padding
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
| `gap` | `var(--flint-space-inline-md)` |

### Copy

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--flint-color-text-secondary)` |
| `font-family` | `var(--flint-typography-body-family)` |
| `font-size` | `var(--flint-typography-body-size)` |
| `line-height` | `var(--flint-typography-body-lineHeight)` |

### Actions

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `align-items` | `center` |
| `gap` | `var(--flint-space-inline-sm)` |

### Button `.pagination-summary__actions button`

| Property | Value |
|----------|-------|
| `min-height` | `var(--flint-size-control-height)` |
| `padding` | `0 var(--flint-space-control-x)` |
| `border` | `0.0625rem solid var(--flint-color-border-default)` |
| `border-radius` | `var(--flint-radius-control)` |
| `background` | `var(--flint-color-background-surface)` |
| `color` | `var(--flint-color-text-primary)` |
| `cursor` | `pointer` |

### Button disabled

| Property | Value |
|----------|-------|
| `opacity` | `var(--flint-state-opacity-disabled)` |
| `cursor` | `not-allowed` |

### Button focus

| Property | Value |
|----------|-------|
| `outline` | `var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

## 9. Svelte Notes

- Summary text computed: `from = (currentPage - 1) * pageSize + 1`,
  `to = Math.min(currentPage * pageSize, totalItems)`
- Display: "Showing {from}-{to} of {totalItems}"
- Page indicator: "Page {currentPage} of {totalPages}"
- Previous button dispatches `pageChange` with `{page: currentPage - 1}`
- Next button dispatches `pageChange` with `{page: currentPage + 1}`
- Buttons disabled at boundaries via computed boolean

## 10. GPUI Notes

- expected crate/module surface: `flint_gpui::components::pagination_summary`
- Spec struct: `PaginationSummarySpec` in primitives crate
- Component struct: `FlintPaginationSummary` in components crate
- Range calculation is pure function
- Previous/next callbacks receive target page number
- Boundary disabling is computed from currentPage and totalPages

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] currentPage, totalPages, totalItems, pageSize produce correct summary
- [ ] pageChange event fires with correct page number
- [ ] buttons disabled at correct boundaries
- [ ] range calculation matches (from/to/total)

### Tier 2: Visual Parity

- [ ] summary typography matches
- [ ] button dimensions and styling match
- [ ] disabled button opacity matches
- [ ] focus ring matches
- [ ] layout spacing matches

### Tier 3: Implementation Freedom

- [ ] live region announcement method is platform-owned
- [ ] button icon content (arrows) is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Live region announcement | GPUI may use different accessibility announcement method | allowed | same functional result |

## 13. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `currentPage=1`, `totalPages=8`, `totalItems=156`, `pageSize=20` | Summary text showing item range (e.g., "Showing 1-20 of 156"), page indicator ("Page 1 of 8"), previous button disabled (first page), next button enabled |

### Single Page

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Single page | `currentPage=1`, `totalPages=1`, `totalItems=12`, `pageSize=20` | Summary text showing full range (e.g., "Showing 1-12 of 12"), both previous and next buttons disabled |

### Large Dataset

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Large dataset | `currentPage=5`, `totalPages=50`, `totalItems=1000`, `pageSize=20` | Summary text showing mid-range items (e.g., "Showing 81-100 of 1000"), page indicator ("Page 5 of 50"), both buttons enabled |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: list views, table views, search results, data browsers
- future follow-up: items-per-page selector, jump-to-page input
