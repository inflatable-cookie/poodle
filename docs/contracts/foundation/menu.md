# Menu

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Menu`
- Layer: `foundation`
- Summary: a triggered command list overlay for actions, toggles, or grouped
  choices
- In scope: trigger/menu relationship, item semantics, separators, disabled
  items, focus movement, dismissal
- Out of scope: arbitrary form content, multi-panel palettes, menu bars,
  cascading submenus

## 2. Anatomy

```text
[Root]
  ├── [Trigger]
  └── [Menu Overlay]
        └── [Menu Item or Separator...]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | menu host | state context |
| Trigger | yes | opens the menu | button tokens, focus ring |
| Menu Overlay | conditional | floating command list | surface, elevation, border |
| Menu Item | yes | actionable or selectable row | text, icon, hover, focus, selected state |
| Separator | no | groups item clusters | border, spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `Array<{ value: string; label: string; isDisabled?: boolean; isChecked?: boolean; shortcutLabel?: string; kind?: "action" \| "checkbox" \| "radio" \| "separator" }>` | none | yes | menu item model |
| `open` | `boolean` | `false` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `placement` | `string` | `"bottom-start"` | no | overlay placement hint |
| `ariaLabel` | `string \| null` | `null` | no | menu label when item set needs one |
| `onOpenChange` | `(open: boolean) => void` | none | no | open-state callback |
| `onAction` | `(value: string) => void` | none | no | item activation callback |

### Controlled And Uncontrolled

- controlled: `open` plus `onOpenChange`
- uncontrolled: `defaultOpen`
- selection/check state lives in `items` and remains externally owned

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | overlay hidden |
| open | open state true | overlay visible |
| highlight | active descendant or focused item | hover/focus styling |
| checked | item model marks checked | visible indicator |
| disabled | item model marks disabled | muted non-interactive item |

### Component States

Open/closed state and current highlighted item state are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | menu opens or closes | boolean | trigger and dismissal driven |
| `onAction` | actionable item commits | item value | disabled and separator rows never fire |

## 6. Accessibility

### Semantics

- Role: menu button trigger plus menu/menuitem semantics
- Required attributes: expanded state, trigger-to-menu relationship, item role,
  checked state where applicable
- Optional attributes: menu label, shortcut text as supplemental description
- Labeling rules: the trigger name and menu label should not conflict

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` or `Space` | opens menu from trigger and activates focused item when open |
| `Arrow Down/Up` | opens menu and moves highlight between enabled items |
| `Home/End` | moves to first/last enabled item |
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

- overlay width may match trigger or fit content according to usage
- long menus should compose with `ScrollShell`

### Composition

- parent expectations: toolbar actions, field affordances, shell actions
- child expectations: menu items and separators only in this baseline
- resizing rules: shortcut columns and selection indicators align consistently

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Trigger | button family roles | invocation chrome |
| Menu Overlay | surface, border, elevation, and overlay roles | floating shell |
| Menu Item | text, icon, focus, selected, and disabled roles | command row |
| Separator | separator roles | visual grouping |
| Motion | motion duration/easing roles | open and close transitions when used |

## 9. Svelte Notes

- may build on headless menu primitives, but the public contract owns trigger,
  dismissal, and keyboard semantics
- typeahead is encouraged when item counts are non-trivial

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::menu`
- GPUI implementation must intentionally manage overlay focus, highlighted-item
  semantics, and checked-item exposure while keeping the invoking control and
  menu relationship accessible

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] trigger, expanded, menu, and item semantics match
- [ ] highlight movement, activation, and dismissal behavior match
- [ ] focus restoration and checked-item exposure match

### Tier 2: Visual Parity

- [ ] overlay, item density, and grouping hierarchy use comparable token roles

### Tier 3: Implementation Freedom

- [ ] placement engine and typeahead implementation stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact placement collision strategy may differ | overlay engine internals vary by runtime | allowed | keep invocation, dismissal, and item semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: shell action menus, field menus, compact command groups
- future follow-up: define submenu and menubar behavior separately if real
  adopters require it

## Next Task

Use `Menu` for triggered command overlays and reserve `ContextMenu` for
invocation at a pointer or keyboard context position.
