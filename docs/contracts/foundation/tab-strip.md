# TabStrip

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `TabStrip`
- Layer: `foundation`
- Summary: a reorderable, optionally closable tab row for document, panel, or
  workspace surfaces
- In scope: active tab selection, close affordances, overflow posture,
  reorderable semantics, keyboard reorder support
- Out of scope: full docking systems, split panes, persisted workspace layout
  state

## 2. Anatomy

```text
[Root]
  ├── [TabList]
  │     └── [Strip Tab...]
  └── [Overflow or Add Actions] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | strip host | layout spacing |
| TabList | yes | strip navigation container | border, gap |
| Strip Tab | yes | selectable strip item | text, background, focus, selected state |
| Close Action | no | dismisses a tab | icon, hover, focus |
| Overflow or Add Actions | no | extra strip actions | icon, spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string` | none | no | controlled active tab |
| `defaultValue` | `string` | none | no | uncontrolled initial active tab |
| `items` | `Array<{ value: string; label: string; isDisabled?: boolean; isClosable?: boolean }>` | none | yes | strip items |
| `isReorderable` | `boolean` | `false` | no | enables reorder behavior |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | strip axis |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `onValueChange` | `(value: string) => void` | none | no | active-tab callback |
| `onReorder` | `(items: string[]) => void` | none | no | new ordered value list |
| `onClose` | `(value: string) => void` | none | no | close callback |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange`
- uncontrolled: `defaultValue`
- reorder and close behavior are command-style interactions surfaced through
  callbacks

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| idle | non-selected tab | inactive styling |
| selected | active value | selected styling |
| focus | tab or close action focused | visible focus treatment |
| drag-target | reorder hover or keyboard move mode | insertion feedback |
| disabled | disabled tab | muted non-interactive tab |

### Component States

Selection state, roving-focus state, and optional reorder mode are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | active tab changes | selected value | selection only |
| `onReorder` | reorder commits | ordered value list | only when reorder is enabled |
| `onClose` | close affordance activates | item value | optional per item |

## 6. Accessibility

### Semantics

- Role: tablist and tab semantics with explicit labeling for close controls
- Required attributes: selected state, tab names, tab-to-surface relationship
  when a paired surface exists
- Optional attributes: reorder instructions or description text
- Labeling rules: close actions must not steal the tab's accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Right` or `Arrow Up/Down` | moves focus along the strip |
| `Home/End` | moves focus to first/last tab |
| `Enter` or `Space` | activates focused tab |
| `Delete` or close shortcut | closes focused tab when closable |
| reorder shortcut plus arrows | reorders the focused tab when supported |

### Focus And Announcement

- focus entry: roving focus lands on the selected tab or first enabled tab
- focus transition: close controls participate predictably without breaking tab
  navigation
- focus restoration: after close, focus moves to the nearest surviving tab or a
  documented fallback
- live-region behavior: reorder and close results should be announced when they
  materially change tab position or presence
- GPUI-native accessibility mapping notes: GPUI must model reorder actions,
  selected state, and close affordance labeling explicitly, including native
  announcement of move results where keyboard reorder exists

## 7. Layout

### Sizing

- strip may scroll or overflow according to parent shell rules
- tabs should tolerate mixed label lengths without collapsing selection clarity

### Composition

- parent expectations: editor shells, multi-surface work areas, dock headers
- child expectations: strip tabs plus optional action buttons
- resizing rules: active-tab visibility should be preserved when overflow
  mechanics exist

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| TabList | border and spacing roles | strip grouping |
| Strip Tab | text, background, selected, focus, and disabled roles | tab chrome |
| Close Action | icon and focus roles | close affordance |
| Reorder feedback | accent and border roles | insertion target |

## 9. Svelte Notes

- should build on semantic tab semantics first, layering pointer drag or
  keyboard reorder behavior on top
- reorder support should not require pointer-only interaction

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::tab_strip`
- GPUI implementation must preserve keyboard reorder access, accessible move
  announcements, and deterministic post-close focus placement

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] selection, close, and reorder semantics match
- [ ] keyboard navigation and keyboard reorder behavior match where enabled
- [ ] focus restoration after close or reorder matches

### Tier 2: Visual Parity

- [ ] strip density and active-tab emphasis use comparable token roles

### Tier 3: Implementation Freedom

- [ ] pointer-drag mechanics and overflow implementation stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| pointer drag preview details may differ | drag visuals are runtime-specific | allowed | keep reorder semantics and announcements strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: document strips, panel strips, workspace headers
- future follow-up: connect tab-strip overflow and docking composites in later
  workstation milestones

## Next Task

Use `TabStrip` for reorderable surface rows, and keep ordinary `Tabs` simpler
when the main requirement is content navigation.
