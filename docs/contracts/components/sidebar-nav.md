# SidebarNav

Status: detailed contract
Updated: 2026-09-29 (endLabel and per-item context menu admitted portable)

## 1. Purpose

- Component name: `SidebarNav`
- Layer: `composites`
- Summary: grouped sidebar navigation list for catalogue, settings, inspector,
  and verification surfaces
- In scope: active-item state with accent rail, optional section headings,
  grouped and ungrouped list posture, anchor or button items, compact sidebar
  presentation, size and density scaling, disabled items, group separators,
  focus-visible ring, compact end-aligned item metadata (`endLabel`), per-item
  context menu (`contextMenuItems`)
- Out of scope: router ownership, page layout, breadcrumb trails, global shell
  toolbars, nested tree disclosure, drag-and-drop reordering, a new menu
  surface (the overlay is the shared ContextMenu)

## 2. Anatomy

```text
[Root <nav> .poodle-sidebar-nav]
  └── [Group <section> .poodle-sidebar-nav__group]*
        ├── [GroupTitle <h2> .poodle-sidebar-nav__group-title]  (optional)
        └── [ItemList <ul> .poodle-sidebar-nav__list]
              └── [Item <li>]*
                    └── [ItemLink <a> .poodle-sidebar-nav__item] or [ItemButton <button> .poodle-sidebar-nav__item]
                          ├── [Label <span> .poodle-sidebar-nav__label]          (when endLabel set)
                          └── [EndLabel <span> .poodle-sidebar-nav__end-label]   (when endLabel set)
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| Root | `<nav>` | Class `poodle-sidebar-nav`, `data-size`, `data-density`, `data-size-role`, optional `aria-label` |
| Group | `<section>` | Class `poodle-sidebar-nav__group`, `data-separated` attribute, optional `aria-label` from group label |
| GroupTitle | `<h2>` | Class `poodle-sidebar-nav__group-title`, uppercase label, accent color. Carries `title={label}` **unconditionally** (R4a, g13-039): group labels render on one line — a host's long label is clipped by CSS, never wrapped — and the native `title` gives the sighted pointer user the full label on hover. The group `<section>` already carries `aria-label={label}`, so assistive tech is served without the `title`; the attribute exists for the clipped-visible-text case and is set without detecting truncation, matching the 13 components that already use native `title=` |
| ItemList | `<ul>` | Class `poodle-sidebar-nav__list`, unstyled list container |
| Item | `<li>` | List item wrapper |
| ItemLink | `<a>` | Class `poodle-sidebar-nav__item`, rendered when `item.href` is set and item is not disabled |
| ItemButton | `<button>` | Class `poodle-sidebar-nav__item`, rendered when no href or when disabled |
| Label | `<span>` | Class `poodle-sidebar-nav__label`, flexible item label. Rendered only when `endLabel` is set; otherwise the label is the item's direct text |
| EndLabel | `<span>` | Class `poodle-sidebar-nav__end-label`, rendered only when `endLabel` is set. Compact muted metadata aligned after the flexible label. Carries a generated `id` and `aria-hidden="true"`; the item references it with `aria-describedby` |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `groups` | `SidebarNavGroup[]` | `[]` | yes | Each group contains zero or more nav items |
| `value` | `string \| null` | `null` | no | Currently active item value; two-way bindable (`$bindable`), mutated on activation |
| `ariaLabel` | `string \| null` | `null` | no | Accessible label for the navigation region |
| `size` | `ControlSize \| null` | `null` | no | Explicit absolute sizing override |
| `sizeRole` | `SemanticControlSizeRole` | `"chrome"` | no | Semantic size intent |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override |
| `onValueChange` | `((value: string) => void) \| undefined` | `undefined` | no | Fires when a non-disabled item is activated; payload is the item `value` |
| `onContextAction` | `((itemValue: string, actionValue: string) => void) \| undefined` | `undefined` | no | Fires when a built-in per-item context-menu row is activated. First argument is the nav item's `value`; second is the menu item's `value` |

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
| `endLabel` | `string \| null` | no | Default `null`. Compact end-aligned metadata such as a count ("198" in "Videos 198"). Exposed as the item's accessible description, never its name. Put counts here, not in `label` |
| `contextMenuItems` | `MenuItem[] \| null` | no | Default `null`. Same shape as ListCard's `contextMenuItems`. When non-empty, right-click or keyboard `ContextMenu`/`Shift+F10` on that item opens the shared ContextMenu. Unset, `null`, or empty leaves native item behaviour unchanged. Disabled items never open a menu |
| `contextMenuAriaLabel` | `string \| null` | no | Default `null`. Accessible name for that item's context-menu overlay. When unset, the overlay is labelled `{item.label} actions` |

### Slots

None.

### Controlled And Uncontrolled

Active item is driven by the `value` prop. In the Svelte target `value` is
two-way bindable (`$bindable`): on activation the component both mutates `value`
internally and fires `onValueChange`, so callers may bind `value` or treat it as
controlled.

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| plain list | One untitled group | Items render as one continuous list without extra group chrome |
| grouped | Multiple groups or titled group | Each group reads as a distinct section through spacing and separators |
| active | Item value matches `value` | Active item shows accent fill, left border accent indicator, bolder weight, inset box-shadow |
| hover | Mouse over non-disabled item | Text color primary, elevated background |
| disabled | Item `disabled: true` | Reduced opacity, `cursor: not-allowed`, no activation |
| focus-visible | Keyboard focus on item | Focus ring via `--poodle-border-width-focus` and `--poodle-color-accent-focusRing` |
| end label | Item `endLabel` set | Item lays out as a row; label flexes, end label sits end-aligned in muted tertiary text. Stays visible, in its muted colour, for active, hover and disabled items (disabled opacity applies to the whole item) |
| context menu | Item `contextMenuItems` non-empty; right-click or keyboard `ContextMenu`/`Shift+F10` | Shared ContextMenu overlay opens at the pointer or keyboard origin for that item. Activation does not change the nav `value`. Unset items do not intercept contextmenu |

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep. Per-item
context menus delegate open, dismiss, keyboard navigation, and action to
ContextMenu (`trigger={false}`); SidebarNav owns which item invoked the
overlay and the anchor point.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | User activates a non-disabled item | `string` | Called for both link and button items |
| `onContextAction` | A context-menu item is activated | `itemValue: string, actionValue: string` | Nav item `value`, then menu item `value`. Separators and disabled menu rows fire nothing; the menu closes after it fires. Never fires for a disabled nav item |

## 6. Accessibility

- Root is a semantic `<nav>` region
- `ariaLabel` should be provided whenever surrounding context does not already label the navigation
- Active items expose `aria-current="page"` on both anchor and button elements
- Group sections have `aria-label` from the group `label` prop when provided
- Keyboard interaction follows native link/button behavior; the component does not implement roving focus or composite-menu semantics
- When `contextMenuItems` is non-empty on a non-disabled item, `ContextMenu` or
  `Shift+F10` on that focused link or button opens the shared ContextMenu at the
  item, with overlay `role="menu"` and `aria-label` from `contextMenuAriaLabel`
  or `{item.label} actions`. Menu keyboard behaviour is ContextMenu's
- Disabled items use the native `disabled` attribute on `<button>`
- The item's accessible name is exactly `label`. When `endLabel` is set, the
  end-label element carries `aria-hidden="true"` so name-from-content skips
  it, and the item sets `aria-describedby` to that element's generated `id`, so
  assistive tech announces the metadata as the item's description ("Videos,
  link, 198"). A hidden element referenced by `aria-describedby` still
  contributes its text to the description. Unset or `null` `endLabel` renders
  no end-label element and no `aria-describedby`
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
- hierarchy guidance: item labels should stay as leaf navigation labels, not
  breadcrumb chains or multi-segment trails
- use group labels for section context; do not fake hierarchy inside item text
- if the UI needs dimmed ancestors and chevrons, use `PageHeader` or
  `ListCard`, not `SidebarNav`
- context menu: pass `contextMenuItems` on the nav item (and optional
  `contextMenuAriaLabel`) to use the built-in ContextMenu overlay. Do not wrap
  the item in a detached ContextMenu for per-item actions such as Delete on a
  saved view. Unset or empty `contextMenuItems` leaves link and button
  semantics, including the browser's native context menu on anchors, unchanged

## 8. Token Usage

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` | Root | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` (or absent) |
| `data-density` | Root | `"compact"`, `"default"`, `"comfortable"` (or absent) |
| `data-size-role` | Root | `"chrome"`, `"control"`, `"prominent"` |
| `data-separated` | Group | `"true"` when multiple visible groups |
| `data-end-label` | Item | `"true"` when the item has an `endLabel` (or absent) |

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
| `--poodle-sidebar-nav-end-label-gap` | `calc(var(--poodle-sidebar-nav-item-padding-inline) * 0.5)` | Gap between label and end label; scales with density |
| `--poodle-sidebar-nav-end-label-font-size` | `calc(var(--poodle-sidebar-nav-item-font-size) * 0.85)` | End-label font size; scales with size |

### `.poodle-sidebar-nav` (Root)

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

### `.poodle-sidebar-nav__group`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.3125rem` |
| `min-width` | `0` |

### Group separator (`[data-separated="true"] + .poodle-sidebar-nav__group`)

| Property | Value |
|----------|-------|
| `margin-top` | `0.125rem` |
| `padding-top` | `calc(var(--poodle-sidebar-nav-group-gap) - 0.125rem)` |
| `border-top` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 54%, transparent)` |

### `.poodle-sidebar-nav__group-title`

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

### `.poodle-sidebar-nav__list`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.125rem` |
| `min-width` | `0` |
| `list-style` | `none` |
| `margin` | `0` |
| `padding` | `0` |

### `.poodle-sidebar-nav__item`

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

### `.poodle-sidebar-nav__item[data-end-label="true"]`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `baseline` |
| `gap` | `var(--poodle-sidebar-nav-end-label-gap)` |

### `.poodle-sidebar-nav__label`

| Property | Value |
|----------|-------|
| `flex` | `1 1 auto` |
| `min-width` | `0` |

### `.poodle-sidebar-nav__end-label`

| Property | Value |
|----------|-------|
| `flex` | `0 0 auto` |
| `margin-left` | `auto` |
| `color` | `var(--poodle-color-text-tertiary)` |
| `font-size` | `var(--poodle-sidebar-nav-end-label-font-size)` |
| `font-variant-numeric` | `tabular-nums` |
| `font-weight` | `500` |

The end label keeps its muted colour and weight on hover and active items; the
item's colour and weight changes apply to the label only.

### `.poodle-sidebar-nav__item:hover:not(:disabled)`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 60%, transparent)` |

### `.poodle-sidebar-nav__item--active`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |
| `font-weight` | `600` |
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent)` |
| `box-shadow` | `inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-accent-base) 20%, transparent)` |
| `border-left-color` | `var(--poodle-color-accent-base)` |

The active indicator is implemented as a left border on the item element itself. When inactive, the left border is transparent. When active, it takes the accent color. This replaces the previous `::before` pseudo-element approach.

### `.poodle-sidebar-nav__item:focus-visible`

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### `.poodle-sidebar-nav__item:disabled`

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
- `endLabel` is portable: the Rust spec carries it and the shared renderer
  lays the row out (flexible label plus end-aligned metadata)
- `contextMenuItems` / `contextMenuAriaLabel` are portable item fields on the
  same terms. Invocation is on the item link or button; the overlay is
  ContextMenu with `trigger={false}`. `onContextAction(itemValue,
  actionValue)` fires for a committed menu row

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::sidebar_nav`
- Active indicator is a left border (not a pseudo-element); GPUI uses a border or equivalent edge element
- Size/density scaling must match the custom property override tables
- `SidebarNavItem.endLabel` renders through the shared renderer: the item
  becomes a row with a flexible, wrapping label and a fixed, one-line end
  label at the contract's 0.85× item size in muted tertiary. The item's
  accessible name is the explicit `label`; the end label's generated id is
  the item's `described_by`, the native carrier of `aria-describedby`
- Per-item context menus follow the Tree host pattern: items carrying rows
  raise `on_context_menu(value, origin)` on secondary click and on the
  keyboard menu gestures (`NodeKey::ContextMenu`, or `NodeKey::F10` with
  Shift). The pointer origin carries the window point; a keyboard origin
  anchors at the invoking item (`rect + 16px`), which the host resolves from
  the item's painted bounds. The host mounts the shared ContextMenu overlay
  at that anchor, routes `onContextAction(itemValue, actionValue)`, moves
  focus to the menu's first enabled row, and returns focus to the item —
  which the renderer names with a stable `sidebar-nav-{value}` id — when the
  overlay closes. Disabled items and items without rows never intercept

## 10a. Jetstream Notes

- `SidebarNav::from_spec(spec, theme).on_change(...)`, carrying the chosen
  item's value. Disabled items never fire.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] active item detection matches (value comparison)
- [ ] `aria-current="page"` applied to active items
- [ ] disabled items suppress activation
- [ ] group filtering removes empty groups
- [ ] event name and payload match
- [ ] per-item context menu opens from right-click and `ContextMenu`/`Shift+F10` when `contextMenuItems` is set
- [ ] unset `contextMenuItems` leaves item semantics unchanged
- [ ] `onContextAction` payload is `(itemValue, actionValue)`

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

### End Labels

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Library counts | One titled group "Library" with items carrying `endLabel` counts (Videos 198, Audio 42, Images 1,204) plus one disabled item with a count, `value="videos"` | Counts sit end-aligned in muted tabular figures; the active item keeps its count muted |

### Item Context Menu

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Saved views | One titled group "Saved views" with items carrying `contextMenuItems` (Rename, separator, Delete) on two rows, plus one row without a menu, `value="q4"` | Right-click or the context-menu key on a saved view opens the shared ContextMenu; the row without items keeps native item behaviour |
