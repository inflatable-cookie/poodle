# Menubar

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `Menubar`
- Layer: `foundation`
- Summary: a persistent top-level command menu bar with submenu overlays
- In scope: menubar semantics, top-level menu triggers, submenu item
  navigation, command activation
- Out of scope: app-window integration, native OS menu bridges, nested
  cascading submenus beyond one submenu level

## 2. Anatomy

```text
[Root]
  └── [Menubar]
        └── [Top-Level Menu Trigger...]
              └── [Submenu Overlay]
```

## 3. Props And Inputs

- `value`: `string | null`
- `defaultValue`: `string | null`
- `items`: `Array<{ value: string; label: string; isDisabled?: boolean; items: MenuItem[] }>`
- `ariaLabel`: `string | null`

## 4. States

- closed
- open top-level menu
- highlighted submenu item
- disabled menu or item

## 5. Events

- `onValueChange`
- `onAction`

## 6. Accessibility

- role: menubar, menuitem, and submenu menu semantics
- required semantics: top-level menu relationships, expanded state, submenu
  item roles, focus restoration
- keyboard: left and right across top-level menus, down opens submenu, up/down
  move submenu items, home/end jump bounds, escape closes and restores focus

## 7. Layout

- top-level triggers sit in a persistent horizontal bar
- submenus anchor below their owning trigger

## 8. Token Usage

- menubar surface, trigger, submenu overlay, item, separator, and focus roles

## 9. Svelte Notes

- may share item models with `Menu`, but the persistent top-level menubar
  semantics are distinct

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::menubar`

## 11. Parity Checklist

- [ ] top-level menubar and submenu semantics match
- [ ] keyboard traversal across triggers and submenu items matches
- [ ] action firing and focus restoration match

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact submenu placement details may differ | overlay internals differ by runtime | allowed | keep menubar semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: desktop-style app chrome, pro-tool command surfaces,
  docs or admin command bars

## Next Task

Use `Menubar` for persistent top-level command menus, and keep native window
menu integration or shell-specific command ownership outside the primitive.
