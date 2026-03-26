# Pagination

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Pagination`
- Layer: `foundation`
- Summary: a low-level page navigation control for moving between discrete
  result pages, with previous/next buttons, numbered page buttons, and ellipsis
  truncation
- In scope: previous/next controls, page buttons, current-page state,
  truncated page window with ellipsis, disabled boundary controls
- Out of scope: range summaries, page-size controls, server fetch policy,
  progressive loading, total count display

## 2. Anatomy

```text
[Root .pagination]  <nav>
  ├── [Previous Button .pagination__button]  <button>
  ├── [Pages .pagination__pages]  <span>
  │     ├── [Page Button .pagination__button]...  <button>
  │     └── [Ellipsis .pagination__ellipsis]...  <span> (conditional)
  └── [Next Button .pagination__button]  <button>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | navigation wrapper | layout, gap |
| Previous Button | yes | go to previous page | border, radius, background, color, typography |
| Pages | yes | page number container | layout, gap |
| Page Button | yes | numbered page selector | border, radius, background, color, typography |
| Ellipsis | no | truncation indicator | color, typography |
| Next Button | yes | go to next page | border, radius, background, color, typography |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `currentPage` | `number` | `1` | no | active page (1-based) |
| `totalPages` | `number` | `1` | no | total number of pages |
| `siblingCount` | `number` | `1` | no | pages shown on each side of current |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for nav element |

### Controlled And Uncontrolled

- controlled: `currentPage` plus `pageChange` event
- the component does not own page state; parent drives currentPage

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | page buttons with current highlighted |
| first page | `currentPage === 1` | previous button disabled |
| last page | `currentPage === totalPages` | next button disabled |
| single page | `totalPages === 1` | both prev/next disabled, single page button |
| truncated | page window exceeds display | ellipsis shown for hidden ranges |
| hover | pointer enters button | accent background tint |
| focus | button focused | accent background tint, no outline |
| current | page matches currentPage | accent background fill, accent border tint |
| disabled | boundary reached | reduced opacity, not-allowed cursor |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| page window | siblingCount and totalPages | visible pages computed with ellipsis insertion |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `pageChange` | user clicks a page, previous, or next button | `{ page: number }` | not fired for current page or disabled buttons |

## 6. Accessibility

### Semantics

- Role: `<nav>` element with `aria-label` (defaults to "Pagination" if not provided)
- Current page button: `aria-current="page"`
- Previous/Next buttons: `aria-label="Previous page"` / `aria-label="Next page"`
- Page buttons: `aria-label="Page N"` where N is the page number
- Disabled buttons: `disabled` attribute set
- Ellipsis: `aria-hidden="true"` (decorative)

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves focus between buttons in DOM order |
| `Enter` / `Space` | activates focused button |

### Focus And Announcement

- focus entry: first non-disabled button receives focus via standard tab order
- focus exit: standard tab order
- live-region behavior: none; parent may announce page changes if needed
- GPUI-native accessibility mapping notes: GPUI must expose nav landmark, current-page state, and disabled boundaries through native accessibility tree

## 7. Layout

### Sizing

- Root: inline-flex with wrapping for narrow viewports
- Buttons: min-width 2.25rem, height slightly below control-height
- overflow behavior: buttons wrap when space is constrained

### Composition

- parent expectations: browse shells, data views, list footers
- child expectations: none (self-contained)
- resizing rules: root wraps its content; flex-wrap handles narrow containers

## 8. Token Usage — Exact Values

### Root `.pagination`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `flex-wrap` | `wrap` |

### Pages `.pagination__pages`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.25rem` |

### Button `.pagination__button`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-width` | `2.25rem` |
| `height` | `calc(var(--poodle-size-control-height) - 0.125rem)` |
| `padding` | `0 0.75rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 78%, transparent)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |
| `line-height` | `1` |

### Button — current page `.pagination__button[aria-current="page"]`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent)` |
| `border-color` | `color-mix(in srgb, var(--poodle-color-accent-base) 42%, var(--poodle-color-border-default))` |

### Button — hover / focus (not disabled, not current)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent)` |
| `outline` | `none` |

### Button — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Ellipsis `.pagination__ellipsis`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-width` | `1.5rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |

### Size adjustments

| Size | button height | button min-width | button padding | button font-size |
|------|--------------|-----------------|----------------|-----------------|
| `xs` | `calc(control-height - 0.625rem)` | `1.75rem` | `0 0.5rem` | `0.625rem` |
| `sm` | `calc(control-height - 0.375rem)` | `2rem` | `0 0.625rem` | `0.6875rem` |
| `md` | `calc(control-height - 0.125rem)` | `2.25rem` | `0 0.75rem` | `0.75rem` |
| `lg` | `calc(control-height + 0.125rem)` | `2.5rem` | `0 0.875rem` | `0.8125rem` |
| `xl` | `calc(control-height + 0.375rem)` | `2.75rem` | `0 1rem` | `0.875rem` |

## 9. Svelte Notes

- `data-size` attribute on root reflects the resolved size
- Uses `<nav>` with `<button>` elements for page controls
- Ellipsis rendered as decorative `<span>` with `aria-hidden="true"`
- Page window computation uses siblingCount to determine visible range with ellipsis insertion
- Previous/Next labels may use icon glyphs with aria-label for accessible naming
- `data-current="true"` data attribute may be used alongside `aria-current="page"`

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::pagination`
- Spec struct: `PaginationSpec` in primitives crate
- GPUI must expose nav landmark semantics and current-page state
- Ellipsis truncation logic must match Svelte implementation for consistent page windows
- Disabled boundary buttons must be non-interactive and announced as disabled
- The color-mix formulas for accent tints should be replicated as closely as possible

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] currentPage, totalPages, siblingCount mean the same thing
- [ ] pageChange event payload matches
- [ ] disabled boundary controls match (first page, last page)
- [ ] ellipsis truncation logic produces equivalent page windows
- [ ] aria-current="page" on current page button matches
- [ ] accessible labels on previous/next/page buttons match

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] button sizing (min-width 2.25rem, height calc) matches
- [ ] button typography (label-family, 0.75rem, 600) matches
- [ ] current-page accent background (18% mix) matches
- [ ] current-page border color-mix formula matches
- [ ] hover/focus accent background (12% mix) matches
- [ ] disabled opacity matches
- [ ] ellipsis styling matches
- [ ] gap spacing (0.375rem root, 0.25rem pages) matches

### Tier 3: Implementation Freedom

- [ ] exact page window computation details are implementation-owned
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact truncation window edges may differ slightly | pagination windowing is implementation-owned | allowed | keep current page and boundary meaning strict |
| color-mix formula rendering | GPUI may approximate color-mix | allowed | match visual result as closely as possible |

## 13. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `currentPage=1`, `totalPages=10`, `ariaLabel="Results pagination"` | Previous button (disabled at page 1), page buttons with page 1 highlighted as current, ellipsis truncation for hidden ranges, next button enabled |

### Middle of Range

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Middle of range | `currentPage=5`, `totalPages=20`, `siblingCount=2`, `ariaLabel="Extended pagination"` | Both previous and next buttons enabled, page 5 highlighted as current, two sibling pages shown on each side, ellipsis for truncated ranges at both ends |

### Few Pages

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Few pages | `currentPage=2`, `totalPages=3`, `ariaLabel="Short pagination"` | All three page buttons visible (no ellipsis needed), page 2 highlighted as current, previous and next buttons both enabled |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: browse shells, reports, data views, list footers
- future follow-up: page-size selector may get a separate companion component
