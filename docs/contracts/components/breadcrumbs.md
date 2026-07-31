# Breadcrumbs

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Breadcrumbs`
- Layer: `foundation`
- Summary: a compact hierarchical path navigation trail for product pages or
  detail surfaces, showing the user's location within a navigational hierarchy
- In scope: path items with link or callback navigation, current-page indication,
  truncation/overflow via ellipsis when items exceed a threshold, separator icons
  between items, size and density scaling
- Out of scope: global navigation bars, history stacks, tab navigation, dropdown
  menus for collapsed items, breadcrumb editing

## 2. Anatomy

```text
[Root .breadcrumbs]  <nav aria-label="...">
  └── [List .breadcrumbs__list]  <ol>
        └── [Item .breadcrumbs__item]  <li> (repeated)
              ├── [Link]  <a href="..."> | <button> | <span aria-current="page"> | <span aria-hidden="true">
              └── [Separator .breadcrumbs__separator]  <span aria-hidden="true"> (except last)
                    └── [Icon]  chevron-right
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | `<nav>` landmark wrapping the breadcrumb trail | `data-size`, `data-density` |
| List | yes | `<ol>` ordered list of path items | flex layout, gap, color, font-size, line-height |
| Item | yes | `<li>` one hierarchy step | inline-flex, gap |
| Link (anchor) | no | `<a>` when `href` is provided on the item | color inherited from list |
| Link (button) | no | `<button>` for callback-driven navigation | color inherited, transparent background |
| Current label | no | `<span aria-current="page">` for the current/last item | color: text-primary |
| Ellipsis label | no | `<span aria-hidden="true">` for truncation indicator | color inherited |
| Separator | no | chevron-right icon between items (not on last) | opacity: 0.4 |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `BreadcrumbItem[]` | `[]` | no | hierarchy items to display |
| `ariaLabel` | `string` | `"Breadcrumb"` | no | accessible label for the `<nav>` element |
| `maxVisibleItems` | `number \| null` | `null` | no | when set and items exceed threshold, collapses middle items to ellipsis |
| `forceLastItemCurrent` | `boolean` | `true` | no | when true, the last visible item is treated as current (`aria-current="page"`) even without `current: true`; set false to opt out |
| `size` | `ControlSize \| null` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `SemanticControlSizeRole` | `"chrome"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for gap spacing |

### BreadcrumbItem Type

```typescript
type BreadcrumbItem = {
  value: string;
  label: string;
  href?: string;
  current?: boolean;
};
```

### Controlled And Uncontrolled

- Declarative path model; parent provides the complete `items` array
- Navigation may be link-driven (`href`) or callback-driven (via `onNavigate`)

### Truncation Behavior

When `maxVisibleItems` is set and `items.length > maxVisibleItems`:
- First item is always shown
- Middle items collapse to a single ellipsis entry (`"..."`)
- Last `maxVisibleItems - 1` items are shown
- The ellipsis entry has `value="__ellipsis__"` and is non-interactive

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | intermediate item without `current` | link-style item in secondary text color, clickable |
| current | `current=true`, or last visible item when `forceLastItemCurrent` is true (default) | non-link `<span>` with `aria-current="page"`, primary text color |
| truncated | `items.length > maxVisibleItems` | first item, ellipsis, then last N-1 items shown |
| hover (link/button) | pointer over interactive item | browser default link/button hover |

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Callbacks

| Callback | When It Fires | Signature | Notes |
|----------|---------------|-----------|-------|
| `onNavigate` | non-current, non-ellipsis item is clicked (button path only) | `(value: string) => void` | items with `href` still navigate via native anchor behavior instead |

## 6. Accessibility

### Semantics

- Root: `<nav>` element with `aria-label` (defaults to `"Breadcrumb"`)
- List: `<ol>` providing ordered list semantics
- Current item: `aria-current="page"` on the `<span>` for the current item (or the last visible item when `forceLastItemCurrent` is true)
- Ellipsis: `aria-hidden="true"` so screen readers skip the truncation indicator
- Separator: `aria-hidden="true"` on all separator icons
- Interactive items: rendered as `<a>` (when `href` provided) or `<button>` (callback-driven)

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves focus through interactive path items (links and buttons) in document order |
| `Enter` | activates focused link or button |
| `Space` | activates focused button |

### Focus And Announcement

- focus entry: only interactive path items (links, buttons) participate in tab order
- focus exit: current-page item is not focusable (it is a `<span>`)
- live-region behavior: none

## 7. Layout

### Sizing

- List uses flex-wrap so items can wrap to multiple lines in narrow containers
- Separators are inline-flex and secondary to path items
- No explicit width constraints; fills available space

### Composition

- parent expectations: page headers, detail shells, nested settings views
- child expectations: only `BreadcrumbItem` data; no slot content
- resizing: wraps naturally; current page remains visible when truncation occurs
- composition rule: breadcrumbs provide hierarchy context before local page
  identity; they do not replace the page heading

## 8. Token Usage -- Exact Values

### Root `.breadcrumbs`

The `<nav>` root carries `data-size` and `data-density` attributes but has no
visual styles of its own.

### List `.breadcrumbs__list`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `margin` | `0` |
| `padding` | `0` |
| `list-style` | `none` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |

### Item `.breadcrumbs__item`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |

### Link (anchor and button)

| Property | Value |
|----------|-------|
| `border` | `0` |
| `padding` | `0` |
| `background` | `transparent` |
| `color` | `inherit` |
| `cursor` | `pointer` |
| `font` | `inherit` |
| `text-decoration` | `none` |

### Current item `[aria-current="page"]`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |

### Separator `.breadcrumbs__separator`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `opacity` | `0.4` |

### Size adjustments

Size affects list and item gap, and list font-size.

| Size | list/item gap | font-size |
|------|---------------|-----------|
| `xs` | `0.25rem` | `0.6875rem` |
| `sm` | `0.375rem` | `0.78125rem` |
| `md` | `var(--poodle-space-inline-sm)` | `var(--poodle-typography-body-size)` |
| `lg` | `0.625rem` | `1rem` |
| `xl` | `0.75rem` | `1.125rem` |

### Density adjustments

Density controls list and item gap only. It does NOT affect font-size.

| Density | list/item gap |
|---------|---------------|
| `compact` | `0.25rem` |
| `default` | `var(--poodle-space-inline-sm)` (base) |
| `comfortable` | `0.75rem` |

## 9. Svelte Notes

- `data-size` attribute on root `<nav>` reflects the resolved size
- `data-density` attribute on root `<nav>` reflects the resolved density (`compact`, `default`, or `comfortable`)
- Root is a `<nav>` element; list is an `<ol>` with `list-style: none`
- Items with `href` render as `<a>` elements for native link behavior
- Items without `href` and not current render as `<button type="button">` for callback navigation
- Current or last item renders as `<span aria-current="page">`
- Ellipsis item renders as `<span aria-hidden="true">` and is not interactive
- Separator uses the `Icon` component with `name="chevron-right"`
- Size and density resolve from `UiPresentationProvider` context when not explicitly set

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::breadcrumbs`
- Spec struct: `BreadcrumbsSpec` in primitives crate
- GPUI may render separators and overflow using native layout, but path semantics
  and current location must be explicitly mapped
- Truncation behavior must match: first item + ellipsis + last N-1 items

## 10a. Jetstream Notes

- `Breadcrumbs::from_spec(spec, theme).on_navigate(...)`, carrying the crumb's
  `href`.
- The current crumb never fires — you are already there — and neither does a
  crumb with no `href`, which has nowhere to send you.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `<nav>` landmark with `aria-label` matches
- [ ] `aria-current="page"` on current/last item
- [ ] `aria-hidden="true"` on separators and ellipsis
- [ ] `onNavigate` callback receives the correct `value`
- [ ] truncation shows first item + ellipsis + last N-1 items
- [ ] items with `href` use anchor navigation; items without use callback

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table (gap and font-size)
- [ ] all three densities match per density table (gap)
- [ ] secondary text color for intermediate items
- [ ] primary text color for current item
- [ ] separator opacity 0.4 with chevron-right icon
- [ ] links and buttons have no visible border, padding, or background
- [ ] list has no margin, padding, or list-style markers

### Tier 3: Implementation Freedom

- [ ] overflow presentation and wrap behavior stay internal
- [ ] link vs button decision is renderer-specific

## 12. Specimen Definitions

### Group: Basic

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Basic | `items=[Home, Projects, Poodle (current)]` | Three-item trail with link-style intermediate items and non-link current item; clicking an intermediate item shows its value |

### Group: Sizes

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Size variants | `items=[Home, Projects, Poodle (current)]`, one row per size (xs, sm, md, lg, xl) | Five breadcrumb trails at increasing font-size and gap |

### Group: Densities

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Density variants | `items=[Home, Projects, Poodle (current)]`, one row per density (compact, default, comfortable) | Three breadcrumb trails with increasing gap between items |

### Group: Deep path

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Deep path | `items=[Home, Workspace, Projects, Poodle Design System, Primitives, Button (current)]` | Full six-item trail with all intermediate links and current-page terminus |

### Group: Collapsed (max 3 visible)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Collapsed | same deep items, `maxVisibleItems=3` | Home > ... > Primitives > Button with ellipsis replacing middle items |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: nested settings, detail pages, catalog hierarchies
- future follow-up: add richer overflow breadcrumb menu if real adopters need it
