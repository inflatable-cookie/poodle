# ContextMenu

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `ContextMenu`
- Layer: `foundation`
- Summary: a contextual command overlay opened from a pointer location or
  keyboard context invocation
- In scope: pointer-position or target-position anchoring, menu semantics,
  dismissal, keyboard context invocation parity
- Out of scope: menu bars, nested inspector popovers, custom canvas gestures

## 2. Anatomy

```text
[Root]
  ├── [Invocation Target] (external)
  └── [Context Menu Overlay]
        └── [Menu Item or Separator...]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | state owner | overlay state |
| Invocation Target | yes | element or surface that spawned the menu | focus context |
| Context Menu Overlay | conditional | positioned command list | surface, elevation, border |
| Menu Item | yes | actionable or selectable row | text, icon, hover, focus, selected state |
| Separator | no | grouping break | border, spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `Array<{ value: string; label: string; isDisabled?: boolean; kind?: "action" \| "checkbox" \| "radio" \| "separator" }>` | none | yes | item model |
| `open` | `boolean` | `false` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `anchorPoint` | `{ x: number; y: number } \| null` | `null` | no | pointer-based anchor |
| `ariaLabel` | `string \| null` | `null` | no | optional menu label |
| `onOpenChange` | `(open: boolean) => void` | none | no | open-state callback |
| `onAction` | `(value: string) => void` | none | no | item activation callback |

### Controlled And Uncontrolled

- controlled: `open` plus `onOpenChange`
- uncontrolled: `defaultOpen`
- the invocation target and anchor point are external inputs; the menu does not
  own context selection itself

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | overlay hidden |
| open | open state true | overlay visible at invocation point |
| highlight | active descendant or focused item | item highlight treatment |
| disabled | item disabled | muted non-interactive row |

### Component States

Open/closed state, invocation anchor state, and current highlighted item state
are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | menu opens or closes | boolean | pointer, keyboard, or dismissal driven |
| `onAction` | actionable item commits | item value | separator rows never fire |

## 6. Accessibility

### Semantics

- Role: context-invoked menu with menu/menuitem semantics
- Required attributes: menu role, item roles, checked state where applicable,
  accessible relationship back to the invocation target when meaningful
- Optional attributes: menu label
- Labeling rules: keyboard invocation must still produce a meaningful focus and
  naming context even when no pointer coordinates exist

### Keyboard

| Key | Behavior |
|-----|----------|
| `ContextMenu` or `Shift+F10` | opens the menu for the focused target |
| `Arrow Down/Up` | moves highlight between enabled items |
| `Home/End` | moves to first/last enabled item |
| `Enter` or `Space` | activates the focused item |
| `Escape` | closes the menu and restores focus to the invoking target |

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

- overlay sizes to content and viewport constraints
- long menus should compose with `ScrollShell`

### Composition

- parent expectations: list rows, canvas selections, tree nodes, workspace
  surfaces
- child expectations: menu items and separators only in this baseline
- resizing rules: collision handling should keep the menu fully reachable

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Context Menu Overlay | surface, border, elevation, and overlay roles | floating shell |
| Menu Item | text, icon, focus, selected, and disabled roles | command row |
| Separator | separator roles | grouping |
| Motion | motion roles | open and close transitions when used |

## 9. Svelte Notes

- should reuse core menu semantics while sourcing position from the invocation
  event or focused target geometry
- keyboard context invocation must not be treated as a lesser path than
  right-click

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::context_menu`
- GPUI implementation must explicitly track invocation origin, anchor position,
  and restoration target; desktop-native context menus do not remove the need
  for parity review

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] pointer and keyboard context invocation semantics match
- [ ] item navigation, activation, and dismissal behavior match
- [ ] focus restoration to the invocation target matches

### Tier 2: Visual Parity

- [ ] overlay hierarchy and item treatments use comparable token roles

### Tier 3: Implementation Freedom

- [ ] exact collision and placement internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| pointer hotspot alignment may differ slightly | coordinate systems differ by runtime | allowed | keep invocation meaning and target restoration strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: list rows, panel headers, canvas selections, shell
  surfaces
- future follow-up: connect richer selection-aware command composition in later
  composite milestones

## Next Task

Use `ContextMenu` when the invocation point itself matters; use ordinary `Menu`
when a visible trigger owns the interaction.
