# SidebarNav

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `SidebarNav`
- Layer: `composites`
- Summary: grouped sidebar navigation list for catalogue, settings, inspector,
  and verification surfaces
- In scope: active-item state with accent rail, optional section headings,
  grouped and ungrouped list posture, anchor or button items, compact sidebar
  presentation, size and density scaling, disabled items, group separators,
  focus-visible ring
- Out of scope: router ownership, page layout, breadcrumb trails, global shell
  toolbars, nested tree disclosure, drag-and-drop reordering

## 2. Anatomy

```text
[Root <nav>]
  └── [Group <section>]*
        ├── [GroupTitle <h2>]  (optional)
        └── [ItemList <ul>]
              └── [Item <li>]*
                    └── [ItemLink <a>] or [ItemButton <button>]
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| Root | `<nav>` | Class `sidebar-nav`, `data-size`, `data-density`, `data-size-role`, optional `aria-label` |
| Group | `<section>` | Class `sidebar-nav__group`, `data-separated` attribute, optional `aria-label` from group label |
| GroupTitle | `<h2>` | Class `sidebar-nav__group-title`, uppercase label, accent color |
| ItemList | `<ul>` | Class `sidebar-nav__list`, unstyled list container |
| Item | `<li>` | List item wrapper |
| ItemLink | `<a>` | Class `sidebar-nav__item`, rendered when `item.href` is set and item is not disabled |
| ItemButton | `<button>` | Class `sidebar-nav__item`, rendered when no href or when disabled |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `groups` | `SidebarNavGroup[]` | `[]` | yes | Each group contains zero or more nav items |
| `value` | `string \| null` | `null` | no | Currently active item value |
| `ariaLabel` | `string \| null` | `null` | no | Accessible label for the navigation region |
| `size` | `ControlSize \| null` | `null` | no | Explicit absolute sizing override |
| `sizeRole` | `SemanticControlSizeRole` | `"chrome"` | no | Semantic size intent |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override |

### Type: SidebarNavGroup

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `id` | `string` | yes | Stable key for the group |
| `label` | `string \| null` | no | Optional visual group title |
| `items` | `SidebarNavItem[]` | yes | Items rendered in order |

### Type: SidebarNavItem

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `value` | `string` | yes | Stable active key |
| `label` | `string` | yes | Visible item label |
| `href` | `string \| null` | no | When present, renders an anchor |
| `disabled` | `boolean` | no | Disabled items render inertly |

### Slots

None.

### Controlled And Uncontrolled

Active item is controlled via `value` prop. The component reports activation
through `onValueChange`.

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| plain list | One untitled group | Items render as one continuous list without extra group chrome |
| grouped | Multiple groups or titled group | Each group reads as a distinct section through spacing and separators |
| active | Item value matches `value` | Active item shows accent fill, left border accent indicator, bolder weight, inset box-shadow |
| hover | Mouse over non-disabled item | Text color primary, elevated background |
| disabled | Item `disabled: true` | Reduced opacity, `cursor: not-allowed`, no activation |
| focus-visible | Keyboard focus on item | Focus ring via `--poodle-border-width-focus` and `--poodle-color-accent-focusRing` |

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | User activates a non-disabled item | `string` | Called for both link and button items |

## 6. Accessibility

- Root is a semantic `<nav>` region
- `ariaLabel` should be provided whenever surrounding context does not already label the navigation
- Active items expose `aria-current="page"` on both anchor and button elements
- Group sections have `aria-label` from the group `label` prop when provided
- Keyboard interaction follows native link/button behavior; the component does not implement roving focus or composite-menu semantics
- Disabled items use the native `disabled` attribute on `<button>`
- Focus ring uses `outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` with `outline-offset: 0.125rem`

## 7. Layout

### Sizing

- Root uses `display: grid` with padding `var(--poodle-space-panel-y) 0.375rem`
- Groups filtered to remove empty groups before rendering
- Single untitled groups read as one continuous list
- Titled or multiple groups visually separate via spacing and border separators
- Item content wraps cleanly for long titles
- Group titles: uppercase, smaller than items, accent-colored, heavier weight

### Composition

- Parent expectations: narrow sidebar columns, stacked verification/catalogue rails
- Child expectations: none (self-contained)
- Resizing rules: min-width 0, items stretch to fill available width

## 8. Token Usage

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` | Root | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` (or absent) |
| `data-density` | Root | `"compact"`, `"default"`, `"comfortable"` (or absent) |
| `data-size-role` | Root | `"chrome"`, `"control"`, `"prominent"` |
| `data-separated` | Group | `"true"` when multiple visible groups |

### CSS Custom Properties (Internal)

| Property | Default | Purpose |
|----------|---------|---------|
| `--poodle-sidebar-nav-item-height` | `1.875rem` | Item min-height |
| `--poodle-sidebar-nav-group-gap` | `var(--poodle-space-panel-y)` | Gap between groups |
| `--poodle-sidebar-nav-item-padding-inline` | `var(--poodle-space-control-x)` | Item horizontal padding |
| `--poodle-sidebar-nav-item-padding-block` | `0.375rem` | Item vertical padding |
| `--poodle-sidebar-nav-item-font-size` | `var(--poodle-typography-label-size)` | Item font size |
| `--poodle-sidebar-nav-title-font-size` | `calc(var(--poodle-typography-label-size) * 0.75)` | Group title font size |
| `--poodle-sidebar-nav-title-letter-spacing` | `0.18em` | Group title tracking |
| `--poodle-sidebar-nav-title-gap` | `calc(var(--poodle-space-panel-y) * 0.375)` | Gap between title and list |

### `.sidebar-nav` (Root)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-sidebar-nav-group-gap)` |
| `min-width` | `0` |
| `align-content` | `start` |
| `padding` | `var(--poodle-space-panel-y) 0.375rem` |

### Size Variants

| Size | Item Height | Item Font | Title Font |
|------|-------------|-----------|------------|
| xs | `1.375rem` | `0.6875rem` | `0.46875rem` |
| sm | `1.625rem` | `0.75rem` | `0.5rem` |
| md | `1.875rem` | `0.8125rem` | `0.5625rem` |
| lg | `2.125rem` | `0.875rem` | `0.59375rem` |
| xl | `2.375rem` | `0.9375rem` | `0.625rem` |

### Density Variants

| Density | Group Gap | Item Padding Inline | Item Padding Block | Title Tracking | Title Gap |
|---------|-----------|--------------------|--------------------|---------------|-----------|
| compact | `0.625rem` | `0.5rem` | `0.3125rem` | `0.2em` | `0.125rem` |
| default | `0.75rem` | `0.75rem` | `0.375rem` | `0.18em` | `0.1875rem` |
| comfortable | `0.875rem` | `0.875rem` | `0.4375rem` | `0.16em` | `0.25rem` |

### `.sidebar-nav__group`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.3125rem` |
| `min-width` | `0` |

### Group separator (`[data-separated="true"] + .sidebar-nav__group`)

| Property | Value |
|----------|-------|
| `margin-top` | `0.125rem` |
| `padding-top` | `calc(var(--poodle-sidebar-nav-group-gap) - 0.125rem)` |
| `border-top` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 54%, transparent)` |

### `.sidebar-nav__group-title`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `padding` | `0 var(--poodle-sidebar-nav-item-padding-inline) var(--poodle-sidebar-nav-title-gap)` |
| `color` | `var(--poodle-color-accent-base)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-sidebar-nav-title-font-size)` |
| `font-weight` | `700` |
| `letter-spacing` | `var(--poodle-sidebar-nav-title-letter-spacing)` |
| `line-height` | `1.2` |
| `text-transform` | `uppercase` |

### `.sidebar-nav__list`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.125rem` |
| `min-width` | `0` |
| `list-style` | `none` |
| `margin` | `0` |
| `padding` | `0` |

### `.sidebar-nav__item`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `block` |
| `width` | `100%` |
| `min-width` | `0` |
| `min-height` | `var(--poodle-sidebar-nav-item-height)` |
| `padding` | `var(--poodle-sidebar-nav-item-padding-block) var(--poodle-sidebar-nav-item-padding-inline)` |
| `border` | `0` |
| `border-left` | `0.1875rem solid transparent` |
| `border-radius` | `0.1875rem calc(var(--poodle-radius-control) - 0.125rem) calc(var(--poodle-radius-control) - 0.125rem) 0.1875rem` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-sidebar-nav-item-font-size)` |
| `font-weight` | `500` |
| `line-height` | `1.3` |
| `text-align` | `left` |
| `text-decoration` | `none` |
| `cursor` | `pointer` |
| `transition` | `color, background, box-shadow` via `--poodle-motion-duration-interaction` and `--poodle-motion-easing-standard` |

### `.sidebar-nav__item:hover:not(:disabled)`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 60%, transparent)` |

### `.sidebar-nav__item--active`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |
| `font-weight` | `600` |
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent)` |
| `box-shadow` | `inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-accent-base) 20%, transparent)` |
| `border-left-color` | `var(--poodle-color-accent-base)` |

The active indicator is implemented as a left border on the item element itself. When inactive, the left border is transparent. When active, it takes the accent color. This replaces the previous `::before` pseudo-element approach.

### `.sidebar-nav__item:focus-visible`

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### `.sidebar-nav__item:disabled`

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |
| `cursor` | `not-allowed` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- `data-size`, `data-density`, `data-size-role` set on root `<nav>`
- Filters out empty groups before rendering (`visibleGroups` derived)
- Items with `href` and not disabled render as `<a>`, otherwise as `<button>`
- `aria-current="page"` applied to active items regardless of element type
- `data-separated` attribute on groups tracks whether multiple visible groups exist
- Item activation calls `onValueChange` unless the item is disabled
- Uses callback props instead of a dispatcher event surface

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::sidebar_nav`
- Active indicator is a left border (not a pseudo-element); GPUI should use a border or equivalent edge element
- Size/density scaling must match the custom property override tables

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] active item detection matches (value comparison)
- [ ] `aria-current="page"` applied to active items
- [ ] disabled items suppress activation
- [ ] group filtering removes empty groups
- [ ] event name and payload match

### Tier 2: Visual Parity

- [ ] active fill, left border indicator, and inset box-shadow match
- [ ] hover background and color match
- [ ] group separator border matches
- [ ] group title typography (uppercase, accent color, weight) matches
- [ ] size variant scaling matches all 5 sizes
- [ ] density variant scaling matches all 3 densities
- [ ] focus ring matches

### Tier 3: Implementation Freedom

- [ ] transition timing is platform-owned
- [ ] rendering internals stay internal

## 12. Specimen Definitions

### Single Group (Plain List)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Plain list | One untitled group with items (Overview, Components, Tokens, Guides), `value="components"` | Continuous list with "Components" active, left border indicator visible |

### Multiple Groups

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Grouped list | Two groups: "Foundation" (Button, Checkbox, Switch) and "Composites" (DataTable, FormDialog), `value="button"` | Two labelled sections separated by border, "Button" active with left border indicator |

### Disabled Items

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With disabled | One group with items where one is `disabled: true` | Disabled item at reduced opacity, non-interactive |
