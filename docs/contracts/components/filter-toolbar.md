# FilterToolbar

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `FilterToolbar`
- Layer: `composites`
- Summary: a compact, optionally collapsible control strip for search, filtering, sorting, and result summary affordances above lists or grids
- In scope: search and filter control grouping, responsive grid layout, collapsible state, result summary, action buttons, secondary actions, size and density variants
- Out of scope: result data ownership, domain-specific filter logic, active filter pill management, command palette behavior

## 2. Anatomy

```text
[Root .filter-toolbar]  <div role="toolbar"> aria-label
  ├── [Header .filter-toolbar__header]  <div> (never a <button> — it contains controls)
  │     ├── [CollapseToggle]  (optional, when collapsible)
  │     ├── [Summary .filter-toolbar__summary]  <p>/<span> (optional, when summaryText or summary snippet)
  │     └── [Actions .filter-toolbar__actions]  <div>/<span> (optional, when actions snippet)
  │           └── [Actions]  `actions` snippet
  ├── [ControlsGrid .filter-toolbar__controls]  <div> (hidden when collapsed)
  │     └── [Children]  filter controls (TextInput type="search", Select, etc.)
  └── [Secondary .filter-toolbar__secondary]  <div> (optional, when secondary snippet)
        └── [Secondary]  `secondary` snippet
```

### Parts

| Part | Element | Required | Notes |
|------|---------|----------|-------|
| Root | `<div>` | yes | Grid container with `role="toolbar"` and `aria-label` |
| Header | `<div>` | yes | Flex row. Always a non-interactive container: it holds interactive children (CollapseToggle, action buttons), so making it a `<button>` would nest interactive controls (invalid HTML). When `collapsible`, a click handler on the row is a pointer convenience; CollapseToggle remains the real control |
| CollapseToggle | `CollapseToggle` primitive | no | Present when `collapsible` is true |
| Summary | `<p>` / `<span>` | no | Result count or active-filter summary text |
| Actions | `<div>` / `<span>` | no | Icon buttons (refresh, etc.) aligned right via `margin-left: auto` |
| ControlsGrid | `<div>` | yes | Responsive CSS grid of filter controls; hidden when collapsed |
| Secondary | `<div>` | no | Trailing actions below the grid (reset, export, etc.) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `ariaLabel` | `string` | `"Filters"` | no | Toolbar/group label |
| `summaryText` | `string \| null` | `null` | no | Visible summary copy in header |
| `collapsible` | `boolean` | `true` | no | Whether controls can be collapsed |
| `collapsed` | `boolean` | `false` | no | Current collapsed state (bindable) |
| `columns` | `number` | `4` | no | Legacy grid hint; still emitted as `--ft-columns`, but the current Svelte grid uses auto-fit rather than this count directly |
| `minItemWidth` | `string` | `"10rem"` | no | Minimum width per grid item |
| `sticky` | `boolean` | `false` | no | Sticky positioning when host supports it |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override |
| `sizeRole` | `SemanticControlSizeRole` | `"chrome"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for spacing |

### Snippets

| Snippet | Purpose |
|------|---------|
| `children` | Filter controls (TextInput type="search", Select, etc.) placed in the grid |
| `summary` | Optional summary content replacing `summaryText` |
| `actions` | Icon buttons in the header row (refresh, settings, etc.) |
| `secondary` | Trailing actions below the grid (reset, export, etc.) |

### Controlled And Uncontrolled

- `collapsed` is bindable for two-way control (`bind:collapsed`)
- Filter state is fully host-owned
- Children render inside a local `UiPresentationProvider`; explicit toolbar
  `size` and `density` cascade into nested controls unless those controls
  override presentation locally

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| expanded | default or `collapsed=false` | Header + controls grid visible |
| collapsed | `collapsible=true`, `collapsed=true` | Header only, controls hidden; clicking the header row toggles open |
| summarized | `summaryText` present | Summary text in header row |
| with-actions | actions snippet populated | Icon buttons right-aligned in header |
| sticky | `sticky=true` | Sticky positioning with elevation shadow |

### Component States

State table is sufficient.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Callbacks

No component-owned callbacks. Child controls keep their own callback contracts, and the toolbar updates `collapsed` through `CollapseToggle`'s `onToggle` callback.

## 6. Accessibility

### Semantics

- Root: `role="toolbar"` with `aria-label` (default `"Filters"`)
- Header: a non-interactive `<div>` in every state. It must not be a `<button>`:
  it contains interactive children (CollapseToggle, action buttons) and nesting
  interactive controls is invalid HTML with ambiguous activation semantics
- Collapse affordance: CollapseToggle is the single accessible control. It owns
  `aria-expanded` and the descriptive `aria-label` (including summary text when
  collapsed), and provides the keyboard/AT semantics for expanding
- Header row click: pointer-only convenience that toggles collapse, ignoring
  clicks on the actions slot and on CollapseToggle itself. Keyboard and assistive
  technology use CollapseToggle, so no behaviour is pointer-exclusive
- CollapseToggle: managed by CollapseToggle primitive accessibility

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | Moves through contained controls in DOM order |
| `Enter` / `Space` | Toggles collapse when CollapseToggle or the header button is focused |

### Focus And Announcement

- Collapsed header button: `border-width-focus` solid `accent-focusRing`, offset `0.125rem`
- The toolbar container is not focusable by default
- Sticky positioning must not create duplicate focus order

## 7. Layout

### Sizing

- Controls grid uses CSS grid with responsive breakpoints:
  - Full width: `repeat(auto-fit, minmax(min(minItemWidth, 100%), 1fr))`
  - <=640px: 1 column
- `columns` is still emitted as `--ft-columns`, but the current Svelte
  implementation does not consume it for the grid template
- Each grid item has minimum width of `minItemWidth` (default `10rem`)
- Summary and actions remain in the header row at all widths

### Composition

- Composes: `CollapseToggle` primitive, `Icon` primitive
- Parent expectations: browser-style pages, settings views with search/filter affordances
- Child expectations: TextInput type="search", Select, SegmentedControl, Button, IconButton
- Resizing rules: primary controls remain first in reading and focus order

## 8. Token Usage -- Exact Values

#### `.filter-toolbar` (Root)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-sm)` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent)` |

#### `.filter-toolbar[data-sticky="true"]`

| Property | Value |
|----------|-------|
| `box-shadow` | `var(--poodle-elevation-surface)` |

#### `.filter-toolbar__header`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |

#### `.filter-toolbar__header--button` (collapsed header)

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `inherit` |
| `text-align` | `left` |
| `cursor` | `pointer` |

#### `.filter-toolbar__header--button:focus-visible`

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

#### `.filter-toolbar__summary`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `flex` | `1` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `var(--poodle-typography-label-size, 0.75rem)` |
| `line-height` | `var(--poodle-typography-label-lineHeight, 1.4)` |

#### `.filter-toolbar__actions`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-xs, 0.25rem)` |
| `margin-left` | `auto` |

#### `.filter-toolbar__controls`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `repeat(auto-fit, minmax(min(var(--ft-min-width, 10rem), 100%), 1fr))` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `align-items` | `end` |

The `--ft-columns` and `--ft-min-width` CSS variables are set inline from the `columns` and `minItemWidth` props.

#### `.filter-toolbar__secondary`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `align-items` | `center` |

### Responsive Breakpoints

#### `@media (max-width: 640px)` -- `.filter-toolbar__controls`

| Property | Value |
|----------|-------|
| `grid-template-columns` | `1fr` |

### Size Adjustments (Summary font-size)

| Size | Summary font-size |
|------|------------------|
| `xs` | `0.6875rem` |
| `sm` | `0.71875rem` |
| `md` | (default from `--poodle-typography-label-size`) |
| `lg` | `0.8125rem` |
| `xl` | `0.875rem` |

### Density Adjustments

Padding values are `padding-block padding-inline`. Density altering vertical padding here is the documented compositional exception (panel internal padding).

| Density | Root gap | Root padding (block inline) | Controls gap |
|---------|----------|-------------|-------------|
| `compact` | `0.25rem` | `0.5rem 0.75rem` | `0.25rem` |
| `default` | `var(--poodle-space-inline-sm)` | `0.75rem 1rem` | `var(--poodle-space-inline-sm)` |
| `comfortable` | `var(--poodle-space-inline-md)` | `1rem 1.25rem` | `var(--poodle-space-inline-md)` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-sticky` | `.filter-toolbar` root | Enables elevation shadow for sticky positioning |
| `data-collapsed` | `.filter-toolbar` root | Controls collapsed state styling |
| `data-size` | `.filter-toolbar` root | Drives size variant CSS |
| `data-density` | `.filter-toolbar` root | Drives density variant CSS |

## 9. Svelte Notes

- Uses `CollapseToggle` primitive for collapse affordance
- Child controls inherit toolbar presentation through a local
  `UiPresentationProvider`
- Controls grid uses CSS custom properties (`--ft-columns`, `--ft-min-width`) driven by props
- `collapsed` prop supports `bind:collapsed` for two-way binding
- Resolves size via `resolveSemanticControlSize` with `sizeRole="chrome"` (not `"control"`)
- When collapsed, header renders as `<button>` for accessibility
- When `collapsible` is true, both collapsed and expanded header rows render as `<button>` elements
- `handleHeaderClick` does `collapsed = !collapsed` (bidirectional toggle), filtering clicks on `.filter-toolbar__actions` and `.collapse-toggle` children

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::filter_toolbar`
- GPUI may realize sticky behavior with native layout containers, but must preserve labeled-group semantics and control order

## 10a. Jetstream Notes

- `FilterToolbar::from_spec(spec, theme).child(...).actions(...).on_toggle(...)`,
  carrying the expanded state the toolbar is moving **to**.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] labeled-group semantics and control order match
- [ ] summary meaning and placement in the accessibility tree match
- [ ] collapse behavior hides/shows controls consistently
- [ ] collapsed header is keyboard-accessible

### Tier 2: Visual Parity

- [ ] control grouping, spacing, and grid layout use comparable token roles
- [ ] responsive breakpoints produce equivalent column reduction
- [ ] size and density variants match

### Tier 3: Implementation Freedom

- [ ] wrap, sticky, and grid mechanics stay internal

## 12. Specimen Definitions

### Responsive Grid Layout

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Responsive grid layout | `summaryText="Showing 24 of 156 items"`, children: TextInput type="search" + 3 Selects (status, type, owner) | Toolbar with summary text and 4 controls in responsive grid |

### Collapsible With Actions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Collapsible with actions | `collapsible`, `summaryText="Showing 24 of 156 items"`, actions slot: refresh IconButton, children: TextInput type="search" + 2 Selects | Toolbar with collapse toggle, summary, refresh button, and filter controls |

### Collapsed By Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Collapsed by default | `collapsible`, `collapsed`, `summaryText="3 filters active"`, actions slot: refresh IconButton, children: TextInput type="search" + Select | Compact header row with expand toggle, summary text, and refresh button |

### With Secondary Slot

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With secondary slot | `columns={3}`, children: TextInput type="search" + 2 Selects; secondary slot: Reset all button | Toolbar with 3-column grid and trailing Reset all button |
