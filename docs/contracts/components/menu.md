# Menu

Status: detailed contract
Updated: 2026-07-10

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
  ├── [Trigger .menu__trigger]  <div role="button"> (wraps trigger snippet)
  └── [Overlay .menu__overlay]  <div role="menu">
        └── [Item .menu__item | Separator .menu__separator]...
              └── [Meta .menu__meta] (optional shortcut label)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | menu host, position context | relative positioning |
| Trigger | yes | opens the menu | button tokens, focus ring, optional aria label |
| Overlay | conditional | floating command list | surface, elevation, border, radius |
| Item | yes | actionable or selectable row | text, hover, focus, disabled state |
| Meta | no | shortcut label column | secondary text, code font |
| Separator | no | groups item clusters | border, spacing |

## 3. Props And Inputs

### Snippets

| Snippet | Purpose |
|------|---------|
| `trigger` | named snippet for custom trigger content; wrapped in `<div class="menu__trigger">` with `role="button"` |

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `MenuItem[]` | — | yes | menu item model |
| `open` | `boolean \| null` | `null` | no | controlled open state; null = uncontrolled |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `placement` | `OverlayPlacement` | `"bottom-start"` | no | overlay placement hint |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `density` | `"compact" \| "default" \| "comfortable" \| null` | `null` | no | explicit density override for item spacing; when null, resolves from inherited presentation |
| `ariaLabel` | `string \| null` | `null` | no | menu label when item set needs one |
| `triggerAriaLabel` | `string \| null` | `null` | no | accessible label for the trigger wrapper, useful for icon-only triggers |
| `onOpenChange` | `(open: boolean) => void` | `undefined` | no | called when the menu opens or closes |
| `onAction` | `(value: string) => void` | `undefined` | no | called when an actionable item commits |

### Type Definitions

```
MenuItem: {
  value: string;
  label: string;
  kind?: "action" | "checkbox" | "radio" | "separator";
  disabled?: boolean;
  checked?: boolean;
  shortcutLabel?: string;
  tone?: "default" | "danger";
}

OverlayPlacement: "bottom-start" | "bottom-end" | "top-start" | "top-end"
```

### Controlled And Uncontrolled

- controlled: `open` (boolean) plus `onOpenChange(open)`
- uncontrolled: `open` is null, uses `defaultOpen` as initial state
- selection/check state lives in `items` and remains externally owned

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | overlay hidden |
| open | open state true | overlay visible below/above trigger |
| highlight | hover or keyboard focus on item | item background highlight |
| checked | item model marks `checked` | visible checked indicator |
| disabled | item model marks `disabled` | muted non-interactive item, reduced opacity |

### Component States

Open/closed state and current highlighted item index are required.

### Behavior Machine

Behavior classification: machine-backed (`menuTransition` in
`@poodle/headless`)

Menu overlay machine shared by Menu and ContextMenu. Item navigation
(roving focus, typeahead) currently lives in the MenuSurface adapter and
joins the machine in a later batch (recorded sweep debt).

- States: `closed` | `open`; open state controllable
- Events: `TOGGLE` (trigger click), `OPEN` (Enter/Space/ArrowDown on
  trigger; contextmenu / Shift+F10 for ContextMenu), `CLOSE`, `ESCAPE` and
  `OUTSIDE_INTERACT` (dismissable-layer stack), `ACTION { value }` (item
  activation)
- Transitions: `ACTION` emits `emitAction(value)` then closes with
  `emitOpenChange(false)`; escape/outside close via the layer stack
  (innermost-first). Closing does not restore trigger focus (matches
  pre-machine behavior).
- Effects: `emitOpenChange`, `emitAction`, `focusFirstItem` (executed after
  the surface renders and is positioned)
- Machinery dependencies: dismissable-layer stack; anchor positioning stays
  adapter-side until the Floating UI swap.

## 5. Callbacks

| Prop | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | menu opens or closes | `boolean` | trigger and dismissal driven |
| `onAction` | actionable item commits | `string` | disabled and separator rows never fire |

## 6. Accessibility

### Semantics

- Trigger: `role="button"`, `aria-expanded` reflecting open state
- Overlay: `role="menu"`, `aria-label` when provided
- Items: `role="menuitem"` for action kind, `role="menuitemcheckbox"` for
  checkbox kind, `role="menuitemradio"` for radio kind
- Checked items: `aria-checked` attribute reflecting `checked` state
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
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Overlay `.menu__overlay`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `z-index` | `var(--poodle-overlay-z-menu)` |
| `min-width` | `14rem` |
| `padding` | `0.25rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))` |
| `box-shadow` | `var(--poodle-elevation-overlay)` |

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
| `border-radius` | `calc(var(--poodle-radius-control) - 0.125rem)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |
| `font-size` | `0.875rem` |
| `text-align` | `left` |

### Item — hover / focus

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent)` |
| `outline` | `none` |

### Item — destructive tone

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-danger-base)` |
| `background` | `color-mix(in srgb, var(--poodle-color-danger-base) 14%, transparent)` on hover/focus |

### Item — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Meta `.menu__meta`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.6875rem` |

### Separator `.menu__separator`

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `height` | `0.0625rem` |
| `margin` | `0.25rem 0` |
| `background` | `color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent)` |

### Size adjustments

| Size | item min-height | item padding | item font-size |
|------|----------------|--------------|----------------|
| `xs` | `1.5rem` | `0.25rem 0.375rem` | `0.75rem` |
| `sm` | `1.75rem` | `0.3125rem 0.4375rem` | `0.8125rem` |
| `md` | `2rem` | `0.375rem 0.5rem` | `0.875rem` |
| `lg` | `2.25rem` | `0.4375rem 0.5625rem` | `0.9375rem` |
| `xl` | `2.5rem` | `0.5rem 0.625rem` | `1rem` |

## 9. Svelte Notes

- `data-size` attribute on root reflects the resolved size
- May build on headless menu primitives, but the public contract owns trigger,
  dismissal, and keyboard semantics
- Typeahead is encouraged when item counts are non-trivial
- Items use a two-column grid layout: label fills available space, meta
  (shortcut label) is auto-sized in the second column
- Separator items are rendered as non-interactive dividers outside the grid
  item pattern

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::menu`
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

- [ ] all five sizes visually match per size table
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

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### With shortcuts

Menu with keyboard shortcut annotations:

| Label | Shortcut | State | Type |
|-------|----------|-------|------|
| New file | ⌘N | enabled | item |
| Open… | ⌘O | enabled | item |
| Save | ⌘S | enabled | item |
| — | — | — | separator |
| Export as PDF | — | enabled | item |
| Print… | ⌘P | disabled | item |

### With checkboxes

Menu with checkbox items:

| Label | Checked | Type |
|-------|---------|------|
| Dark mode | true | checkbox |
| Notifications | false | checkbox |
| — | — | separator |
| Settings… | — | item |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: shell action menus, field menus, compact command groups,
  toolbar overflow menus
- future follow-up: define submenu and cascading behavior separately if real
  adopters require it
