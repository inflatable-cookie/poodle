# ContextMenu

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `ContextMenu`
- Layer: `foundation`
- Summary: a contextual command overlay opened from a pointer location or
  keyboard context invocation
- In scope: pointer-position or target-position anchoring, menu semantics,
  item kinds (action, checkbox, radio, separator), cascading submenus
  (`children`), dismissal, keyboard context invocation parity
- Out of scope: menu bars, nested inspector popovers, custom canvas gestures

## 2. Anatomy

```text
[Root .context-menu]  <div>
  ├── [Invocation Target] (slotted child element)
  └── [Overlay .context-menu__overlay]  <div role="menu">
        └── [Item .context-menu__item | Separator .context-menu__separator]...
              └── [Meta .context-menu__meta] (optional shortcut label)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | state owner, wraps invocation target | position context |
| Invocation Target | yes | element that spawns the menu via contextmenu event | focus context |
| Overlay | conditional | positioned command list at pointer/keyboard anchor | surface, elevation, border, radius |
| Item | yes | actionable or selectable row | text, hover, focus, disabled state |
| Meta | no | shortcut label column | secondary text, code font |
| Separator | no | grouping break | border, spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `MenuItem[]` | — | yes | menu item model |
| `open` | `boolean \| null` | `null` | no | controlled open state; null = uncontrolled |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `anchorPoint` | `{ x: number; y: number } \| null` | `null` | no | pointer-based anchor position (clientX/clientY) |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `density` | `"compact" \| "default" \| "comfortable" \| null` | `null` | no | explicit density override for item spacing; when null, resolves from inherited presentation |
| `ariaLabel` | `string \| null` | `null` | no | optional menu label |

### Type Definitions

```
MenuItem: {
  value: string;
  label: string;
  kind?: "action" | "checkbox" | "radio" | "separator";
  disabled?: boolean;
  checked?: boolean;
  shortcutLabel?: string;
  children?: MenuItem[];
}
```

### Controlled And Uncontrolled

- controlled: `open` (boolean) plus `onOpenChange(open)`
- uncontrolled: `open` is null, uses `defaultOpen` as initial state
- the invocation target and anchor point are external inputs; the menu does not
  own context selection itself

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | overlay hidden |
| open | open state true | overlay visible at invocation point |
| highlight | hover or keyboard focus on item | item background highlight |
| checked | item model marks `checked` | visible checked indicator |
| disabled | item model marks `disabled` | muted non-interactive row, reduced opacity |

### Component States

Open/closed state, invocation anchor position, and current highlighted item
index are required.

### Behavior Machine

Behavior classification: machine-backed (`menuTransition` in
`@poodle/headless`)

Menu overlay machine shared by Menu and ContextMenu. Item navigation is
core machinery (`menuListNavigate`, `menuNavigableItems`,
`menuListCanActivate`): wrapping disabled-skip next/prev, enabled-boundary
first/last, separator exclusion, activation guards. The MenuSurface adapter
owns focus calls and rendering. Typeahead is not part of the current
behavior (the Svelte implementation is the parity authority).

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
- Machinery dependencies: dismissable-layer stack; pointer-anchored
  placement stays adapter-side (virtual-anchor adoption of the core
  resolver is possible later).

ContextMenu deltas: outside containment tests only the overlay (clicking
the trigger zone closes). Re-invoking contextmenu while already open
repositions the anchor without re-emitting `onOpenChange(true)` (the
pre-machine component re-fired the callback; recorded delta).

## 5. Callbacks

| Prop | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | menu opens or closes | `boolean` | pointer, keyboard, or dismissal driven |
| `onAction` | actionable item commits | `string` | disabled and separator rows never fire |

## 6. Accessibility

### Semantics

- Overlay: `role="menu"`, `aria-label` when provided
- Items: `role="menuitem"` for action kind, `role="menuitemcheckbox"` for
  checkbox kind, `role="menuitemradio"` for radio kind
- Checked items: `aria-checked` attribute reflecting `checked` state
- Disabled items: `aria-disabled="true"`
- Labeling rules: keyboard invocation must produce a meaningful focus and
  naming context even when no pointer coordinates exist

### Keyboard

| Key | Behavior |
|-----|----------|
| `ContextMenu` or `Shift+F10` | opens the menu for the focused target; keyboard anchor uses element rect center + 16px offset |
| `Arrow Down` | moves highlight to next enabled item |
| `Arrow Up` | moves highlight to previous enabled item |
| `Home` | moves to first enabled item |
| `End` | moves to last enabled item |
| `Enter` or `Space` | activates the focused item |
| `Escape` | closes the menu and restores focus to the invoking target |
| `Arrow Right` | on a submenu parent: opens the flyout and focuses its first enabled item |
| `Arrow Left` | inside a flyout: closes it and restores focus to the parent item |
| Outside click | any mousedown outside the overlay closes the menu |
| character keys | optional typeahead over enabled items |

Submenus (`children` on an item) follow the shared menu-surface behavior
defined in `menu.md` ("Submenus"): hover / `Arrow Right` flyouts anchored at
the parent item's right edge, viewport-edge flip, recursive nesting, and
leaf-only activation.

### Focus And Announcement

- focus entry: keyboard invocation keeps the invocation target knowable and
  moves active item focus into the menu
- focus restoration: close returns focus to the invoking target or nearest
  surviving fallback
- live-region behavior: none; item roles and states must be exposed through
  native menu semantics
- GPUI-native accessibility mapping notes: GPUI must support both pointer-based
  invocation and keyboard context invocation without dropping accessible origin
  context or focus restoration

## 7. Layout

### Sizing

- Overlay min-width: 14rem
- Overlay sizes to content and viewport constraints
- Long menus should compose with `ScrollShell`

### Activation

- `contextmenu` event on the root captures `clientX`/`clientY` to set the
  anchor point
- Keyboard invocation (`ContextMenu` key or `Shift+F10`) calculates anchor from
  the focused element's bounding rect with a +16px offset
- The overlay is portalled to the theme root and anchored to the pointer
  position itself — a **virtual anchor** (`pointAnchor(x, y)`), since a
  right-click has no element behind it. The shared resolver then does the
  edge-flipping (`002-anchored-overlays.md`)

### Composition

- parent expectations: list rows, canvas selections, tree nodes, workspace
  surfaces
- child expectations: menu items and separators only in this baseline
- resizing rules: collision handling should keep the menu fully reachable

## 8. Token Usage — Exact Values

### Root `.context-menu`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `min-width` | `0` |

### Overlay `.context-menu__overlay`

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `z-index` | `var(--poodle-overlay-z-menu)` |
| `min-width` | `14rem` |
| `padding` | `0.25rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))` |
| `box-shadow` | `var(--poodle-elevation-overlay)` |

### Overlay — positioning

| Property | Value |
|----------|-------|
| `left` | inline style from `anchorPoint.x` |
| `top` | inline style from `anchorPoint.y` |

### Item `.context-menu__item`

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

### Item — disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Meta `.context-menu__meta`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.6875rem` |

### Separator `.context-menu__separator`

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
- Should reuse core menu item rendering and keyboard navigation patterns from
  `Menu`, while sourcing position from the contextmenu event or focused target
  geometry
- Anchors to a virtual point (clientX/clientY) rather than to a trigger
  element, which is the one difference from `Menu`'s use of the same primitive
- Keyboard context invocation must not be treated as a lesser path than
  right-click; both must produce identical overlay and focus behavior
- Overlay and item CSS values are identical to `Menu` except for the positioning
  model (fixed vs absolute) and the class prefix (`context-menu__` vs `menu__`)
- Document-level mousedown listener closes menu on outside click; detection is
  against the overlay element (not the root), since the root wraps the
  invocation target and clicks on the target area should dismiss the menu
- Document-level keydown listener closes menu on Escape from any focus context
- After render, the overlay position is clamped to the viewport (8px padding) so
  menus opened near edges shift inward rather than clipping; the initial render
  uses `visibility: hidden` to prevent a flash at the unclamped position

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::context_menu`
- GPUI implementation must explicitly track invocation origin, anchor position,
  and restoration target; desktop-native context menus do not remove the need
  for parity review
- The fixed positioning model translates to screen-coordinate placement in GPUI

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] pointer and keyboard context invocation semantics match
- [ ] overlay role="menu" with aria-label matches
- [ ] item roles (menuitem, menuitemcheckbox, menuitemradio) match
- [ ] aria-checked on checkbox/radio items matches
- [ ] item navigation, activation, and dismissal behavior match
- [ ] outside click (mousedown outside overlay) closes the menu
- [ ] focus restoration to the invocation target matches
- [ ] openChange and action event semantics match

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] overlay uses fixed positioning at anchor point
- [ ] overlay uses elevation-overlay, radius-surface, border 72% opacity
- [ ] overlay background uses color-mix elevated 98% with panel
- [ ] item min-height 2rem, padding 0.375rem 0.5rem matches
- [ ] item hover uses accent-base 16% mix
- [ ] item border-radius uses radius-control minus 0.125rem
- [ ] meta uses code-family, 0.6875rem, text-secondary
- [ ] separator uses border-subtle 72%, 0.0625rem height
- [ ] disabled items use state-opacity-disabled

### Tier 3: Implementation Freedom

- [ ] exact collision and viewport clamping internals stay internal
- [ ] keyboard anchor offset calculation stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| pointer hotspot alignment may differ slightly | coordinate systems differ by runtime | allowed | keep invocation meaning and target restoration strict |
| GPUI may use native window overlay for context menus | desktop runtime differs from web | allowed | visual density, item semantics, and token usage must match |

## 13. Specimen Definitions

### Right-Click Target Area

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Right-click the area below | `items` array with Cut (shortcut: Cmd+X), Copy (Cmd+C), Paste (Cmd+V), separator, Select all (Cmd+A), separator, Delete (disabled) | Dashed-border target area; right-clicking opens context menu overlay with action items, shortcut labels in meta column, separator dividers, and a disabled Delete item at reduced opacity |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: list rows, panel headers, canvas selections, shell
  surfaces, tree nodes
- future follow-up: connect richer selection-aware command composition in later
  composite milestones
