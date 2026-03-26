# Menubar

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Menubar`
- Layer: `foundation`
- Summary: a persistent horizontal menu bar with dropdown overlays supporting
  actions, checkboxes, radio items, separators, and keyboard-driven navigation
- In scope: menubar semantics, top-level menu triggers with labels, submenu
  overlays with item navigation, command activation, disabled menus and items,
  shortcut labels (meta), separators, menuitemcheckbox and menuitemradio roles,
  outside-click dismissal, hover-to-switch between triggers
- Out of scope: app-window integration, native OS menu bridges, nested
  cascading submenus beyond one level, routing or navigation semantics

## 2. Anatomy

```text
[Root .menubar]  <div>
  └── [List .menubar__list]  <div>  role="menubar"
        └── [Group .menubar__group]...  <div>
              ├── [Trigger .menubar__trigger]  <button>  role="menuitem"
              └── [Overlay .menubar__overlay]  <div>  role="menu"  (when open)
                    ├── [Item .menubar__item]  <button>  role="menuitem|menuitemcheckbox|menuitemradio"
                    │     ├── [Label]  (grid column 1)
                    │     └── [Meta .menubar__meta]  <span>  (optional, grid column 2)
                    └── [Separator .menubar__separator]  <div>  role="separator"
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | outer wrapper | inline-flex, min-width |
| List | yes | menubar container with visual chrome | border, radius, background, padding, gap |
| Group | yes | position context for trigger + overlay pair | relative positioning |
| Trigger | yes | top-level menu label button | typography, hover, focus, open state |
| Overlay | conditional | floating submenu command list | surface, elevation, border, radius |
| Item | yes | actionable or selectable row within submenu | grid layout, text, hover, focus, disabled |
| Meta | no | shortcut label column | secondary text, code font |
| Separator | no | groups item clusters within submenu | border color, spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled open menu value; null = uncontrolled |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial open menu |
| `items` | `MenubarItem[]` | — | yes | top-level menu definitions with nested items |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the menubar |

### Type Definitions

```
MenubarItem: {
  value: string;
  label: string;
  items: MenuItem[];
  disabled?: boolean;
}

MenuItem: {
  value: string;
  label: string;
  kind?: "action" | "checkbox" | "radio" | "separator";
  disabled?: boolean;
  checked?: boolean;
  shortcutLabel?: string;
}
```

### Controlled And Uncontrolled

- controlled: `value` (string) plus `valueChange` event
- uncontrolled: `value` is null, uses `defaultValue` as initial state
- `value` represents which top-level menu is currently open; null means all
  closed
- module-level `nextMenubarId` counter for unique IDs across instances
- internal tracking: `focusIndex`, `highlightIndex`, `lastOpenValue`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| all closed | default | no overlay visible, triggers in default state |
| menu open | value matches a trigger | that trigger shows open styling, overlay visible |
| trigger hover | pointer over trigger while a menu is open | switches to that menu |
| highlight | hover or keyboard focus on submenu item | item background highlight |
| checked | item model marks `checked` | visible checked indicator |
| disabled trigger | `disabled` on MenubarItem | trigger muted, non-interactive |
| disabled item | `disabled` on MenuItem | item muted, reduced opacity |

### Component States

- Active menu value (which top-level menu is open): controlled or uncontrolled
- `focusIndex`: which trigger has keyboard focus (roving tabindex)
- `highlightIndex`: which item within overlay has keyboard focus
- `lastOpenValue`: tracks last opened menu for hover-to-switch behavior
- Outside click: document mousedown listener closes open menu

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | open menu changes or closes | `{ value: string \| null }` | null when all menus close |
| `action` | submenu item commits | `{ value: string }` | disabled and separator rows never fire |

## 6. Accessibility

### Semantics

- Root: no role (container div)
- List: `role="menubar"`, `aria-label` when provided
- Triggers: `role="menuitem"`, `aria-haspopup="menu"`, `aria-expanded`
  reflecting whether their submenu is open, `aria-controls` pointing to
  overlay id
- Overlay: `role="menu"`, `aria-label={item.label}`
- Items: `role="menuitem"` for action kind, `role="menuitemcheckbox"` for
  checkbox kind, `role="menuitemradio"` for radio kind
- Checked items: `aria-checked` attribute
- Disabled triggers: `aria-disabled="true"`
- Disabled items: `aria-disabled="true"`
- Separator: `role="separator"`

### Keyboard

| Key | Context | Behavior |
|-----|---------|----------|
| `Arrow Right` | trigger focused | moves focus to next top-level trigger (wraps); if menu open, opens next menu |
| `Arrow Left` | trigger focused | moves focus to previous top-level trigger (wraps); if menu open, opens previous menu |
| `Home` | trigger focused | moves focus to first trigger |
| `End` | trigger focused | moves focus to last trigger |
| `Arrow Down` | trigger focused | opens the focused trigger's menu, focuses first item |
| `Enter` or `Space` | trigger focused | opens menu, focuses first item |
| `Escape` | trigger focused | closes the open menu |
| `Arrow Down` | item focused | moves to next enabled item (wraps) |
| `Arrow Up` | item focused | moves to previous enabled item (wraps) |
| `Arrow Right` | item focused | closes current menu, opens next trigger's menu |
| `Arrow Left` | item focused | closes current menu, opens previous trigger's menu |
| `Home` | item focused | moves to first enabled item |
| `End` | item focused | moves to last enabled item |
| `Enter` or `Space` | item focused | activates item |
| `Escape` | item focused | closes menu, returns focus to trigger |

### Focus And Announcement

- focus entry: first trigger participates in tab order; subsequent triggers
  reached via arrow keys (roving tabindex)
- focus exit: tab moves focus out of the menubar
- submenu focus: opening a menu moves active item focus into the submenu
- focus restoration: closing a menu returns focus to its owning trigger
- live-region behavior: none; roles and states carry the semantics
- GPUI-native accessibility mapping notes: GPUI must model the menubar as a
  horizontal navigation container with submenu relationships, not as
  independent buttons

## 7. Layout

### Sizing

- Root: `display: inline-flex`, `min-width: 0`
- List provides the visual chrome (border, background, padding) for the
  trigger strip
- Group: `position: relative`, `display: inline-flex`
- Trigger: inline-flex, min-height 2rem
- Overlays anchor below their owning trigger group with a 0.25rem gap
- Overlay min-width: 12rem

### Composition

- parent expectations: desktop-style app chrome, pro-tool command surfaces,
  docs or admin command bars
- child expectations: top-level triggers own submenu item sets
- resizing rules: trigger strip wraps or scrolls according to host constraints

## 8. Token Usage — Exact Values

### Root `.menubar`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `min-width` | `0` |

### List `.menubar__list`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `gap` | `0.125rem` |
| `padding` | `0.1875rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 96%, transparent)` |

### Group `.menubar__group`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-flex` |

### Trigger `.menubar__trigger`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `min-height` | `2rem` |
| `padding` | `0 0.75rem` |
| `border` | `0` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |
| `line-height` | `1` |

### Trigger — Open / Hover / Focus

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 14%, transparent)` |
| `outline` | `none` |

### Trigger — Disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Overlay `.menubar__overlay`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `calc(100% + 0.25rem)` |
| `left` | `0` |
| `z-index` | `var(--poodle-overlay-z-menu)` |
| `min-width` | `12rem` |
| `padding` | `0.25rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))` |
| `box-shadow` | `var(--poodle-elevation-overlay)` |

### Item `.menubar__item`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `minmax(0, 1fr) auto` |
| `align-items` | `center` |
| `width` | `100%` |
| `min-height` | `2rem` |
| `padding` | `0.375rem 0.5rem` |
| `border` | `0` |
| `border-radius` | `calc(var(--poodle-radius-control) - 0.125rem)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |
| `font-size` | `0.875rem` |
| `text-align` | `left` |

### Item — Hover / Focus

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent)` |
| `outline` | `none` |

### Item — Disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Meta `.menubar__meta`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.6875rem` |

### Separator `.menubar__separator`

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `height` | `0.0625rem` |
| `margin` | `0.25rem 0` |
| `background` | `color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent)` |

## 9. Svelte Notes

- Module-level `nextMenubarId` counter for unique IDs across instances
- Controlled/uncontrolled value via internal `uncontrolledValue` state
- `focusIndex` tracks roving tabindex across triggers
- `highlightIndex` tracks active item within open overlay
- `lastOpenValue` enables hover-to-switch between triggers when a menu is open
- Document-level mousedown listener closes menu on outside click
- Document-level keydown listener closes menu on Escape from any focus context
- Items use `role` based on `kind` field: menuitem, menuitemcheckbox, menuitemradio
- `aria-checked` set on checkbox and radio items
- Trigger hover while a menu is already open should immediately switch to the
  hovered trigger's menu (roving open state)
- Arrow Left/Right within a submenu should close the current menu and open the
  adjacent one, maintaining the roving open behavior
- Submenu items use identical CSS to `Menu` items (same grid, padding, height,
  hover treatment) but with the `menubar__` class prefix

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::menubar`
- Spec struct: `MenubarSpec` in primitives crate holds item tree
- Component struct: `PoodleMenubar` in components crate renders via `IntoElement`
- GPUI must model the menubar as a horizontal menu container with submenu
  relationships and roving focus semantics
- The list chrome (border, background, padding) must be reproduced as a
  visible container around the trigger strip
- GPUI must model `color-mix` as `token.opacity(token.a * multiplier)` since GPUI has no CSS color-mix
- List border opacity: 72% on border-subtle
- List bg opacity: 96% on background-panel
- Overlay border opacity: 72% on border-default
- Overlay bg: 98% elevated mixed with panel
- Trigger/item highlight: 14%/16% accent-base
- Separator: 72% on border-subtle
- Trigger typography (label-family, 0.75rem, weight 600) must match
- Outside click maps to GPUI mouse-down-outside event handling

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] root role="menubar" with aria-label on list matches
- [ ] trigger role="menuitem" with aria-haspopup="menu" and aria-expanded matches
- [ ] aria-controls on triggers links to overlay id
- [ ] overlay role="menu" with aria-labelledby matches
- [ ] submenu item roles (menuitem, menuitemcheckbox, menuitemradio) match
- [ ] aria-checked on checkbox and radio items matches
- [ ] arrow left/right roving across triggers matches (with wrapping)
- [ ] arrow down opens submenu, arrow up/down navigates items
- [ ] arrow right/left within submenu switches to adjacent menu
- [ ] home/end moves to first/last item
- [ ] enter/space activates items
- [ ] escape closes menu and restores focus to trigger
- [ ] outside click closes menu
- [ ] hover-to-switch between triggers when menu is open
- [ ] valueChange and action event semantics match
- [ ] disabled trigger and disabled item behavior match

### Tier 2: Visual Parity

- [ ] list border uses border-subtle 72%, radius-surface, panel background 96%
- [ ] list padding 0.1875rem and gap 0.125rem match
- [ ] trigger uses label-family, 0.75rem, weight 600
- [ ] trigger open/hover/focus uses accent-base 14% mix
- [ ] overlay gap from trigger is 0.25rem
- [ ] overlay min-width 12rem
- [ ] overlay uses elevated 98% mixed with panel background
- [ ] overlay border uses border-default 72%
- [ ] overlay uses elevation-overlay shadow and radius-surface
- [ ] item grid layout with minmax(0,1fr) auto columns matches
- [ ] item border-radius calc(radius-control - 0.125rem) matches
- [ ] item hover/focus uses accent-base 16% mix
- [ ] separator height 0.0625rem and border-subtle 72% match
- [ ] meta uses text-secondary, code-family, 0.6875rem
- [ ] disabled uses state-opacity-disabled

### Tier 3: Implementation Freedom

- [ ] exact submenu placement and collision internals stay internal
- [ ] roving open timing stays internal
- [ ] ID generation scheme is implementation-owned
- [ ] animation/transition on open/close is implementation-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact submenu placement details may differ | overlay internals differ by runtime | allowed | keep menubar semantics strict |
| GPUI may use native window menus for submenus | desktop runtime may leverage OS menus | allowed | item density, roles, and token usage must match |
| GPUI uses opacity multiplication instead of CSS color-mix | platform capability | allowed | visual result must match |
| GPUI outside-click uses mouse-down-outside event | platform capability | allowed | dismissal behavior must match |

## 13. Specimen Definitions

### Application Menu Bar

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Application menu bar | `ariaLabel="Application menu"`, three top-level menus: File, Edit, View | Horizontal menubar with three trigger buttons in a bordered container |

#### File Menu Items

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| New | `shortcutLabel="Cmd+N"` | Action item with shortcut meta |
| Open... | `shortcutLabel="Cmd+O"` | Action item with shortcut meta |
| Save | `shortcutLabel="Cmd+S"` | Action item with shortcut meta |
| (separator) | `kind="separator"` | Horizontal divider |
| Quit | `shortcutLabel="Cmd+Q"` | Action item with shortcut meta |

#### Edit Menu Items

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Undo | `shortcutLabel="Cmd+Z"` | Action item with shortcut meta |
| Redo | `shortcutLabel="Shift+Cmd+Z"` | Action item with shortcut meta |
| (separator) | `kind="separator"` | Horizontal divider |
| Cut | `shortcutLabel="Cmd+X"` | Action item with shortcut meta |
| Copy | `shortcutLabel="Cmd+C"` | Action item with shortcut meta |
| Paste | `shortcutLabel="Cmd+V"` | Action item with shortcut meta |

#### View Menu Items

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Zoom in | `shortcutLabel="Cmd++"` | Action item with shortcut meta |
| Zoom out | `shortcutLabel="Cmd+-"` | Action item with shortcut meta |
| (separator) | `kind="separator"` | Horizontal divider |
| Full screen | `shortcutLabel="Ctrl+Cmd+F"` | Action item with shortcut meta |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: desktop-style app chrome, pro-tool command surfaces,
  docs or admin command bars, application top bar, editor menus
- future follow-up: define cascading submenu behavior if real adopters require
  deeper nesting; keep native window menu integration outside the primitive;
  overflow handling, context menu variant
