# Pagination

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `Pagination`
- Layer: `foundation`
- Summary: a full-featured pagination control that supports numbered page
  buttons, simple item-range display, and full first/last navigation; includes
  an optional record-count info row, an optional page-size limit selector, and
  integrates with an external pagination controller for data-driven list views
- In scope: previous/next navigation, numbered page buttons with ellipsis
  truncation, simple variant showing item ranges, full variant with first/last
  buttons and page-of-total summary, optional info row displaying "Showing X to
  Y of Z", optional limit selector with configurable options, controller-driven
  pagination integration, scroll targeting after page changes, compact mode,
  loading state, size and density variants, responsive stacking at narrow
  viewports
- Out of scope: server fetch policy, query serialization, pagination controller
  creation, data fetching, infinite scroll

## 2. Anatomy

```text
[Root .pagination]  <nav>
  ├── [Info .pagination__info]  <div> (optional — showInfo && total > 0)
  └── [Controls Wrapper .pagination__controls-wrapper]  <div>
        ├── [Limit Selector .pagination__limit]  <div> (optional — showLimitSelector)
        │     ├── <label>  "Show"
        │     ├── <select>  page-size dropdown
        │     └── <span>  "per page"
        └── [Controls .pagination__controls]  <div>
              ├── [First Button .pagination__button]  <button> (full variant only, requires goToPage)
              ├── [Previous Button .pagination__button]  <button>
              ├── [Pages .pagination__pages]  <div> (numbered variant only)
              │     ├── [Page Button .pagination__button]...  <button>
              │     └── [Ellipsis .pagination__ellipsis]...  <span> (conditional)
              ├── [Summary .pagination__summary]  <span> (simple and full variants)
              ├── [Next Button .pagination__button]  <button>
              └── [Last Button .pagination__button]  <button> (full variant only, requires goToPage)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | `<nav>` element wrapping all pagination content | layout, gap, padding, border-top, background |
| Info | no | record-count summary ("Showing X to Y of Z") | color, typography |
| Controls Wrapper | yes | flex container for limit selector and page controls | layout, gap |
| Limit Selector | no | page-size dropdown with label and suffix | color, typography, border, radius, background |
| Controls | yes | inline-flex row of navigation buttons | layout, gap |
| First Button | no | jump to page 1 (full variant with goToPage only) | border, radius, background, color, typography |
| Previous Button | yes | navigate to previous page | border, radius, background, color, typography |
| Pages | no | numbered page button row (numbered variant only) | layout, gap |
| Page Button | no | individual numbered page selector | border, radius, background, color, typography |
| Ellipsis | no | truncation indicator between page ranges | color, typography |
| Summary | no | text summary for simple/full variants | color, typography |
| Next Button | yes | navigate to next page | border, radius, background, color, typography |
| Last Button | no | jump to last page (full variant with goToPage only) | border, radius, background, color, typography |

## 3. Props And Inputs

### Controller Interface

The component accepts an optional `controller` object that drives pagination
state. When a controller is provided its values take precedence over individual
props. The controller interface is:

```typescript
interface PaginationControllerLike {
  currentPage: number;
  pageSize: number;
  total: number | null;
  totalPages: number | null;
  showingFrom: number;
  showingTo: number;
  hasPrevPage: boolean;
  hasNextPage: boolean;
  loading?: boolean;
  prevPage: () => void | Promise<void>;
  nextPage: () => void | Promise<void>;
  setPageSize: (pageSize: number) => void | Promise<void>;
  goToPage?: (page: number) => void | Promise<void>;
}
```

When the controller provides `goToPage`, the full variant shows first/last
buttons. When the controller is used and the user navigates to adjacent pages,
the component calls `prevPage()` / `nextPage()` directly. For non-adjacent
jumps it calls `goToPage()` if available, otherwise falls back to the
`pageChange` event.

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `controller` | `PaginationControllerLike \| null` | `null` | no | external pagination controller; when set, its state values override individual props |
| `currentPage` | `number \| null` | `null` | no | active page (1-based); overridden by controller |
| `totalPages` | `number \| null` | `null` | no | total page count; overridden by controller or computed from `total` / `limit` |
| `page` | `number \| null` | `null` | no | alias for currentPage (lower precedence than currentPage) |
| `limit` | `number \| null` | `null` | no | items per page; overridden by controller.pageSize; falls back to 20 |
| `total` | `number \| null` | `null` | no | total item count; overridden by controller.total |
| `siblingCount` | `number` | `1` | no | how many page numbers to show on each side of the current page |
| `showLimitSelector` | `boolean` | `false` | no | show the page-size selector dropdown |
| `limitOptions` | `number[]` | `[30, 50, 100]` | no | options presented in the limit selector dropdown |
| `showInfo` | `boolean` | `true` | no | show "Showing X to Y of Z" info row |
| `compact` | `boolean` | `false` | no | reduced padding and gap for tight layouts |
| `variant` | `"numbered" \| "full" \| "simple"` | `"numbered"` | no | which navigation layout to render |
| `scrollTarget` | `HTMLElement \| string \| false` | `false` | no | element or CSS selector to scroll into view after page changes; false disables |
| `scrollOffset` | `number` | `16` | no | pixel offset from the scroll container top when scrolling to target |
| `className` | `string` | `""` | no | additional CSS class on root element |
| `loading` | `boolean` | `false` | no | loading state; overridden by controller.loading |
| `chrome` | `boolean` | `false` | no | when true, renders with container padding, border-top, and background |
| `standalone` | `boolean` | `false` | no | **deprecated** — inverse alias for `chrome`; use `chrome` instead |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the nav element; defaults to "Pagination" |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl" \| null` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `"compact" \| "default" \| "comfortable" \| null` | `null` | no | explicit density override; when null, resolves from inherited presentation |

### Prop Precedence

Page state resolves with the following priority (highest first):

1. `controller.currentPage` / `controller.pageSize` / `controller.total` / `controller.totalPages`
2. Individual props: `page` (or `currentPage`), `limit`, `total`, `totalPages`
3. Computed: `totalPages` = ceil(`total` / `limit`); `currentPage` defaults to 1; `limit` defaults to 20

The `currentPage` prop takes precedence over `page` when both are provided.
Total pages are clamped to a minimum of 1. Current page is clamped between 1
and totalPages.

### Variants

| Variant | Center Content | First/Last Buttons | Description |
|---------|---------------|-------------------|-------------|
| `numbered` | Numbered page buttons with ellipsis truncation | no | Traditional page-number navigation |
| `full` | "Page X of Y" text summary | yes (when controller has goToPage) | Full navigation with first/last/prev/next |
| `simple` | "X-Y of Z" item-range summary | no | Minimal prev/next with item range display |

### Controlled And Uncontrolled

- Controlled mode: parent passes `page` / `currentPage` and handles `onPageChange`
- Controller mode: parent passes a `controller` object; the component calls controller methods directly for prev/next/goToPage/setPageSize
- The component does not own page state; the parent or controller drives it

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | page buttons with current page highlighted |
| first page | `currentPage === 1` | previous button disabled; first button disabled (full variant) |
| last page | `currentPage === totalPages` | next button disabled; last button disabled (full variant) |
| single page | `totalPages <= 1` and no limit selector | entire component hidden |
| truncated | page window exceeds sibling range | ellipsis markers shown for hidden page ranges |
| hover | pointer enters non-disabled button | accent background tint |
| focus | button receives focus-visible | accent background tint, outline removed |
| current | page button matches current page | accent background fill, accent border tint |
| disabled button | boundary reached or loading | reduced opacity, not-allowed cursor |
| loading | `loading` prop or `controller.loading` | entire component at 0.7 opacity with pointer-events disabled |
| compact | `compact` prop | reduced padding and tighter gap |

### Visibility Rules

The entire component is hidden when both conditions are true:
- `totalPages <= 1` (only one page of results)
- `showLimitSelector` is false (no limit selector to show)

When `showInfo` is true but `total` is 0 or null, the info row is hidden.

### Page Window Algorithm

The visible page list always includes page 1 and the last page. It also
includes `siblingCount` pages on each side of the current page. Between any
two consecutive visible pages where the gap is greater than 1, an ellipsis is
inserted. For example, with `siblingCount=1` and `currentPage=5` of 20 pages:

```
1  ...  4  5  6  ...  20
```

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onPageChange` | user clicks a page, previous, next, first, or last button | `number` | not called for clicks on current page, disabled buttons, or when controller handles the navigation directly |
| `onLimitChange` | user selects a new page size from the limit dropdown | `number` | not called when controller handles the change directly via `setPageSize()` |

### Callback Dispatch Logic

When a controller is present:
- Adjacent page moves (prev/next) call `controller.prevPage()` / `controller.nextPage()` directly; no event dispatched
- Non-adjacent jumps call `controller.goToPage(page)` if available; no event dispatched
- Non-adjacent jumps call `onPageChange` only when `controller.goToPage` is not available
- Limit changes call `controller.setPageSize(limit)` directly; no event dispatched

When no controller is present:
- All page changes call `onPageChange(page)`
- All limit changes call `onLimitChange(limit)`

After any page or limit change, scroll targeting executes if configured.

## 6. Accessibility

### Semantics

- Role: `<nav>` element with `aria-label` (defaults to `"Pagination"` when `ariaLabel` is null)
- Current page button: `aria-current="page"` and `data-current="true"`
- First button: `aria-label="First page"` (full variant only)
- Previous button: `aria-label="Previous page"`
- Page buttons: `aria-label="Page N"` where N is the page number
- Next button: `aria-label="Next page"`
- Last button: `aria-label="Last page"` (full variant only)
- Disabled buttons: `disabled` attribute set (disables interaction and announces as disabled)
- Ellipsis spans: `aria-hidden="true"` (decorative)
- Limit selector label: `<label for="pagination-limit">` associated with the `<select>` element

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves focus between buttons and limit selector in DOM order |
| `Enter` / `Space` | activates focused button |

### Focus And Announcement

- Focus entry: first non-disabled button receives focus via standard tab order
- Focus exit: standard tab order
- Live-region behavior: none; parent may announce page changes if needed
- The limit selector is a native `<select>` and follows standard select keyboard behavior
- GPUI-native accessibility mapping notes: GPUI must expose nav landmark, current-page state, and disabled boundaries through native accessibility tree

## 7. Layout

### Sizing

- Root: flex with wrapping, `space-between` justification for info and controls
- Buttons: min-width equals control-height token, height slightly below control-height
- Overflow behavior: flex-wrap ensures buttons and controls wrap on narrow viewports

### Responsive Behavior

At viewports 40rem or narrower:
- Root switches to column layout with stretch alignment
- In compact mode the info row is hidden
- Controls wrapper centers itself

### Scroll Targeting

When `scrollTarget` is set (element reference or CSS selector string), the
component scrolls the target into view after any page or limit change:

1. Resolve the target element (querySelector for strings, direct reference for elements)
2. Walk up the DOM to find the nearest scrollable parent (overflowY auto or scroll with scrollHeight > clientHeight)
3. Compute the relative top of the target within the scroll container
4. If the target is already within `scrollOffset` pixels of the container top, skip scrolling
5. Smooth-scroll to position the target `scrollOffset` pixels from the container top, clamped to valid scroll range

### Composition

- Parent expectations: browse shells, data views, list footers, table footers
- Child expectations: none (self-contained)
- Resizing rules: root wraps its content; flex-wrap handles narrow containers

## 8. Token Usage — Exact Values

### Root `.pagination`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `align-items` | `center` |
| `justify-content` | `space-between` |
| `gap` | `1rem` |
| `margin-top` | `var(--poodle-space-panel-y)` |
| `padding` | `var(--poodle-space-control-y) var(--poodle-space-panel-x)` |
| `border-top` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent)` |

### Root compact `.pagination--compact`

| Property | Value |
|----------|-------|
| `padding` | `0.5rem 0.75rem` |
| `gap` | `0.75rem` |

### Root standalone `.pagination--standalone`

| Property | Value |
|----------|-------|
| `margin-top` | `0` |
| `padding` | `0` |
| `border-top` | `none` |
| `background` | `transparent` |

### Root loading `.pagination--loading`

| Property | Value |
|----------|-------|
| `opacity` | `0.7` |
| `pointer-events` | `none` |

### Info `.pagination__info`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |

### Controls Wrapper `.pagination__controls-wrapper`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `1rem` |
| `flex-wrap` | `wrap` |

### Limit Selector `.pagination__limit`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |

### Limit Selector `<select>`

| Property | Value |
|----------|-------|
| `padding` | `0.25rem 0.5rem` |
| `font` | `inherit` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 78%, transparent)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |

### Limit Selector `<select>` focus

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Controls `.pagination__controls`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `flex-wrap` | `wrap` |

### Pages `.pagination__pages`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |

### Button `.pagination__button`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-width` | `var(--poodle-size-control-height)` |
| `height` | `calc(var(--poodle-size-control-height) - 0.125rem)` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 78%, transparent)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |
| `line-height` | `1` |

### Button — current page `.pagination__button[data-current="true"]`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent)` |
| `border-color` | `color-mix(in srgb, var(--poodle-color-accent-base) 42%, var(--poodle-color-border-default))` |

### Button — hover / focus-visible (not disabled)

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

### Summary `.pagination__summary`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |
| `white-space` | `nowrap` |
| `padding` | `0 0.5rem` |

### Size Adjustments

| Size | Button Height | Button Min-Width | Button Font-Size |
|------|--------------|-----------------|-----------------|
| `xs` | `calc(var(--poodle-size-control-height) - 0.625rem)` | `calc(var(--poodle-size-control-height) - 0.625rem)` | `0.6875rem` |
| `sm` | `calc(var(--poodle-size-control-height) - 0.375rem)` | `calc(var(--poodle-size-control-height) - 0.375rem)` | `0.75rem` (default) |
| `md` | `calc(var(--poodle-size-control-height) - 0.125rem)` | `var(--poodle-size-control-height)` | `0.75rem` |
| `lg` | `calc(var(--poodle-size-control-height) + 0.125rem)` | `calc(var(--poodle-size-control-height) + 0.125rem)` | `0.875rem` |
| `xl` | `calc(var(--poodle-size-control-height) + 0.375rem)` | `calc(var(--poodle-size-control-height) + 0.375rem)` | `0.9375rem` |

### Density Adjustments

| Density | Controls Gap | Pages Gap |
|---------|-------------|-----------|
| `compact` | `0.0625rem` | `0.0625rem` |
| `default` | `var(--poodle-space-inline-sm)` (inherited) | `var(--poodle-space-inline-sm)` (inherited) |
| `comfortable` | `0.25rem` | `0.25rem` |

### Variant-Specific Content

| Variant | Center Content | Previous Label | Next Label | First/Last Buttons |
|---------|---------------|----------------|------------|-------------------|
| `numbered` | Page number buttons with ellipsis | "Previous" | "Next" | none |
| `full` | "Page X of Y" summary | "Previous" | "Next" | "<<" and ">>" (when goToPage available) |
| `simple` | "X-Y of Z" item-range summary | "Prev" | "Next" | none |

## 9. Svelte Notes

- `data-size` attribute on root reflects the resolved size (`xs`, `sm`, `md`, `lg`, `xl`)
- `data-density` attribute on root reflects the resolved density (`compact`, `default`, `comfortable`)
- Root is a `<nav>` element; all buttons are native `<button type="button">` elements
- Ellipsis rendered as `<span>` with `aria-hidden="true"` containing the `...` character
- First/last button text uses `<<` and `>>` Unicode characters
- Previous button text is "Prev" in simple variant, "Previous" in numbered and full variants
- Current page button uses `data-current="true"` attribute alongside `aria-current="page"`
- The limit selector select element has `id="pagination-limit"` paired with a `<label for="pagination-limit">`
- Page window computation uses `siblingCount` to determine the visible range; always includes page 1 and the last page; inserts ellipsis between non-adjacent visible pages
- Component auto-hides entirely when `totalPages <= 1` and `showLimitSelector` is false
- Info row shows "Showing X to Y of Z" when total is known, or "Showing X to Y" when total is null
- Simple variant summary shows "X-Y of Z" when total is known, or "X-Y" when total is null
- Full variant summary shows "Page X of Y"
- `className` prop is appended to the root element's class list
- Scroll-into-view is called after both page changes and limit changes

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::primitives::pagination`
- Spec struct: `PaginationSpec` in primitives crate
- GPUI must expose nav landmark semantics and current-page state
- Ellipsis truncation logic must match Svelte implementation for consistent page windows
- Disabled boundary buttons must be non-interactive and announced as disabled
- The color-mix formulas for accent tints should be replicated as closely as possible
- Controller integration pattern should use an equivalent trait or callback struct
- The scroll-targeting behavior is web-specific and may not apply to GPUI

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] Controller interface fields and methods match
- [ ] All three variants (numbered, full, simple) render equivalent content
- [ ] `page`, `currentPage`, `totalPages`, `limit`, `total`, `siblingCount` resolve identically
- [ ] `pageChange` and `limitChange` event payloads match
- [ ] Controller dispatch logic matches (prevPage/nextPage for adjacent, goToPage for jumps)
- [ ] Disabled boundary controls match (first page, last page, loading)
- [ ] Ellipsis truncation algorithm produces equivalent page windows
- [ ] `aria-current="page"` on current page button matches
- [ ] All aria-labels match (First page, Previous page, Page N, Next page, Last page)
- [ ] Component hides when totalPages <= 1 and showLimitSelector is false
- [ ] Info row text format matches ("Showing X to Y of Z")
- [ ] Limit selector label/select/suffix structure matches

### Tier 2: Visual Parity

- [ ] All five sizes visually match per size table
- [ ] All three densities visually match per density table
- [ ] Button sizing (min-width, height calc) matches
- [ ] Button typography (label-family, 0.75rem, 600) matches
- [ ] Current-page accent background (18% mix) matches
- [ ] Current-page border color-mix formula matches
- [ ] Hover/focus accent background (12% mix) matches
- [ ] Disabled opacity uses `var(--poodle-state-opacity-disabled)`
- [ ] Loading state opacity (0.7) and pointer-events match
- [ ] Compact mode padding and gap match
- [ ] Ellipsis styling matches
- [ ] Summary and info typography matches
- [ ] Root background (92% elevated mix), border-top, and padding match
- [ ] Limit selector select styling matches

### Tier 3: Implementation Freedom

- [ ] Scroll targeting behavior is web-specific; other platforms may omit or adapt
- [ ] Exact responsive breakpoint (40rem) is implementation-owned
- [ ] Transition/animation timing is platform-owned
- [ ] Unicode characters for first/last buttons may be replaced with icons

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Scroll targeting is web-only | GPUI/Jetstream have different scroll models | allowed | platforms may implement equivalent scroll behavior |
| color-mix formula rendering | GPUI may approximate color-mix | allowed | match visual result as closely as possible |
| First/last button glyphs | Unicode vs icon is a rendering detail | allowed | visual result should match intent |
| Responsive column stacking at 40rem | breakpoints are platform-specific | allowed | each platform handles narrow layout appropriately |

## 13. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `currentPage=1`, `totalPages=10`, `ariaLabel="Results pagination"` | Previous button (disabled at page 1), numbered page buttons with page 1 highlighted as current, ellipsis truncation for hidden ranges, next button enabled |

### Sizes

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Sizes | `currentPage=1`, `totalPages=10`, each of xs/sm/md/lg/xl | Five pagination rows at progressively larger button sizes |

### Densities

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Densities | `totalPages=10`, each of compact/default/comfortable | Three pagination rows with progressively tighter or wider button gaps |

### Middle of Range

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Middle of range | `currentPage=5`, `totalPages=20`, `siblingCount=2`, `ariaLabel="Extended pagination"` | Both previous and next buttons enabled, page 5 highlighted as current, two sibling pages shown on each side, ellipsis for truncated ranges at both ends |

### Few Pages

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Few pages | `currentPage=2`, `totalPages=3`, `ariaLabel="Short pagination"` | All three page buttons visible (no ellipsis needed), page 2 highlighted as current |

### Simple Variant with Info and Page Size

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Simple variant | `page=3`, `limit=25`, `total=248`, `variant="simple"`, `showLimitSelector=true`, `limitOptions=[10, 25, 50, 100]`, `ariaLabel="Simple pagination"` | Info row showing "Showing 51 to 75 of 248", limit dropdown showing 25, center summary showing "51-75 of 248", prev/next buttons |

### Full Variant

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Full variant | `page=1`, `limit=20`, `total=140`, `ariaLabel="Full pagination"` | "Page 1 of 7" center summary, previous button disabled, next button enabled; first/last buttons shown only when controller with goToPage is provided |

## 14. Approval And Adoption Notes

- Contract status: `detailed contract`
- Approvers: pending
- Downstream adopters: browse shells, data views, list footers, table footers, report views
- Future follow-up: none planned
