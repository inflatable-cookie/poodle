# Menu

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Menu`
- Layer: `foundation`
- Summary: a triggered command list overlay for actions, toggles, or grouped
  choices
- In scope: trigger/menu relationship, item semantics (action, checkbox, radio,
  separator), disabled items, shortcut labels, focus movement, dismissal,
  placement
- Out of scope: arbitrary form content, multi-panel palettes, menu bars,
  cascading submenus

## 2. Anatomy

```text
[Root .menu]  <div>
  ├── [Trigger .menu__trigger]  <button or slot>
  └── [Overlay .menu__overlay]  <div role="menu">
        └── [Item .menu__item | Separator .menu__separator]...
              └── [Meta .menu__meta] (optional shortcut label)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | menu host, position context | relative positioning |
| Trigger | yes | opens the menu | button tokens, focus ring |
| Overlay | conditional | floating command list | surface, elevation, border, radius |
| Item | yes | actionable or selectable row | text, hover, focus, disabled state |
| Meta | no | shortcut label column | secondary text, code font |
| Separator | no | groups item clusters | border, spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `MenuItem[]` | — | yes | menu item model |
| `open` | `boolean \| null` | `null` | no | controlled open state; null = uncontrolled |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `placement` | `OverlayPlacement` | `"bottom-start"` | no | overlay placement hint |
| `ariaLabel` | `string \| null` | `null` | no | menu label when item set needs one |

### Type Definitions

```
MenuItem: {
  value: string;
  label: string;
  kind?: "action" | "checkbox" | "radio" | "separator";
  isDisabled?: boolean;
  isChecked?: boolean;
  shortcutLabel?: string;
}

OverlayPlacement: "bottom-start" | "bottom-end" | "top-start" | "top-end"
```

### Controlled And Uncontrolled

- controlled: `open` (boolean) plus `openChange` event
- uncontrolled: `open` is null, uses `defaultOpen` as initial state
- selection/check state lives in `items` and remains externally owned

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | overlay hidden |
| open | open state true | overlay visible below/above trigger |
| highlight | hover or keyboard focus on item | item background highlight |
| checked | item model marks `isChecked` | visible checked indicator |
| disabled | item model marks `isDisabled` | muted non-interactive item, reduced opacity |

### Component States

Open/closed state and current highlighted item index are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `openChange` | menu opens or closes | `{ open: boolean }` | trigger and dismissal driven |
| `action` | actionable item commits | `{ value: string }` | disabled and separator rows never fire |

## 6. Accessibility

### Semantics

- Trigger: `role="button"`, `aria-expanded` reflecting open state
- Overlay: `role="menu"`, `aria-label` when provided
- Items: `role="menuitem"` for action kind, `role="menuitemcheckbox"` for
  checkbox kind, `role="menuitemradio"` for radio kind
- Checked items: `aria-checked` attribute reflecting `isChecked` state
- Disabled items: `aria-disabled="true"`
- Shortcut labels: supplemental descriptive text within the item

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` or `Space` | opens menu from trigger; activates focused item when open |
| `Arrow Down` | opens menu from trigger; moves highlight to next enabled item |
| `Arrow Up` | moves highlight to previous enabled item |
| `Home` | moves to first enabled item |
| `End` | moves to last enabled item |
| `Escape` | closes menu and restores focus to trigger |
| character keys | optional typeahead over enabled items |

### Focus And Announcement

- focus entry: trigger participates in tab order; opening moves active item
  focus into the menu
- focus exit: dismissal restores focus to the trigger unless action semantics
  intentionally move focus elsewhere
- live-region behavior: none; item kind, checked state, and disabled state must
  be exposed through menu semantics
- GPUI-native accessibility mapping notes: GPUI must expose the menu as a
  navigable command list with highlighted-item movement, checked state, and
  focus restoration rather than as a plain stacked container

## 7. Layout

### Sizing

- Overlay min-width: 14rem
- Overlay width may fit content beyond min-width
- Long menus should compose with `ScrollShell`

### Composition

- parent expectations: toolbar actions, field affordances, shell actions
- child expectations: menu items and separators only in this baseline
- resizing rules: shortcut columns and selection indicators align consistently
  via grid layout

## 8. Token Usage — Exact Values

### Root `.menu`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-flex` |

### Trigger `.menu__trigger`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |

### Trigger — focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Overlay `.menu__overlay`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `z-index` | `var(--pug-overlay-z-menu)` |
| `min-width` | `14rem` |
| `padding` | `0.25rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 72%, transparent)` |
| `border-radius` | `var(--pug-radius-surface)` |
| `background` | `color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))` |
| `box-shadow` | `var(--pug-elevation-overlay)` |

### Overlay — placement: bottom-start (default)

| Property | Value |
|----------|-------|
| `top` | `calc(100% + 0.375rem)` |
| `left` | `0` |

### Overlay — placement: bottom-end

| Property | Value |
|----------|-------|
| `top` | `calc(100% + 0.375rem)` |
| `left` | `auto` |
| `right` | `0` |

### Overlay — placement: top-start

| Property | Value |
|----------|-------|
| `bottom` | `calc(100% + 0.375rem)` |
| `left` | `0` |

### Overlay — placement: top-end

| Property | Value |
|----------|-------|
| `bottom` | `calc(100% + 0.375rem)` |
| `left` | `auto` |
| `right` | `0` |

### Item `.menu__item`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `minmax(0, 1fr) auto` |
| `align-items` | `center` |
| `width` | `100%` |
| `min-height` | `2rem` |
| `padding` | `0.375rem 0.5rem` |
| `border` | `0` |
| `border-radius` | `calc(var(--pug-radius-control) - 0.125rem)` |
| `background` | `transparent` |
| `color` | `var(--pug-color-text-primary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |
| `font-size` | `0.875rem` |
| `text-align` | `left` |

### Item — hover / focus

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--pug-color-accent-base) 16%, transparent)` |
| `outline` | `none` |

### Item — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--pug-state-opacity-disabled)` |

### Meta `.menu__meta`

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |
| `font-family` | `var(--pug-typography-code-family)` |
| `font-size` | `0.6875rem` |

### Separator `.menu__separator`

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `height` | `0.0625rem` |
| `margin` | `0.25rem 0` |
| `background` | `color-mix(in srgb, var(--pug-color-border-subtle) 72%, transparent)` |

## 9. Svelte Notes

- May build on headless menu primitives, but the public contract owns trigger,
  dismissal, and keyboard semantics
- Typeahead is encouraged when item counts are non-trivial
- Items use a two-column grid layout: label fills available space, meta
  (shortcut label) is auto-sized in the second column
- Separator items are rendered as non-interactive dividers outside the grid
  item pattern

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::menu`
- GPUI implementation must intentionally manage overlay focus, highlighted-item
  semantics, and checked-item exposure while keeping the invoking control and
  menu relationship accessible
- The two-column grid layout for label + shortcut can be modeled as a fixed
  layout with right-aligned shortcut text

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] trigger role="button" with aria-expanded matches
- [ ] overlay role="menu" with aria-label matches
- [ ] item roles (menuitem, menuitemcheckbox, menuitemradio) match
- [ ] aria-checked on checkbox/radio items matches
- [ ] highlight movement, activation, and dismissal behavior match
- [ ] focus restoration to trigger matches
- [ ] openChange and action event semantics match

### Tier 2: Visual Parity

- [ ] overlay uses elevation-overlay, radius-surface, border 72% opacity
- [ ] overlay background uses color-mix elevated 98% with panel
- [ ] item min-height 2rem, padding 0.375rem 0.5rem matches
- [ ] item hover uses accent-base 16% mix
- [ ] item border-radius uses radius-control minus 0.125rem
- [ ] meta uses code-family, 0.6875rem, text-secondary
- [ ] separator uses border-subtle 72%, 0.0625rem height
- [ ] disabled items use state-opacity-disabled
- [ ] placement gap 0.375rem matches

### Tier 3: Implementation Freedom

- [ ] placement engine and collision strategy stay internal
- [ ] typeahead implementation stays internal
- [ ] exact transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact placement collision strategy may differ | overlay engine internals vary by runtime | allowed | keep invocation, dismissal, and item semantics strict |
| GPUI may use native window overlay instead of CSS absolute | desktop runtime differs from web | allowed | visual density and token usage must match |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: shell action menus, field menus, compact command groups,
  toolbar overflow menus
- future follow-up: define submenu and cascading behavior separately if real
  adopters require it
