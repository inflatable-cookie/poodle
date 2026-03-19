# PanelTabs

Status: seed contract
Updated: 2026-03-17

## 1. Purpose

- Component name: `PanelTabs`
- Layer: `workstation`
- Summary: a workstation-specific tab strip for switching, reordering, and
  optionally moving panels within a dock or panel group
- In scope: active panel selection, reorder posture, close/move affordances via
  context actions, keyboard focus within dense shell tabs, dock-local panel
  switching within the window-aware workstation model
- Out of scope: full dock drop orchestration, panel body rendering, document
  editor semantics, product navigation, command routing, project semantics

## 2. Anatomy

```text
[Root Tab Strip]
  └── [Panel Tab...]
        ├── [Icon] (optional)
        ├── [Label]
        └── [Close or Menu Affordance] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Tab Strip | yes | region-local panel strip | border, spacing |
| Panel Tab | yes | one panel selector | text, background, focus, selected state |
| Close/Menu Affordance | no | close or options entry point | icon, focus |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `Array<{ value: string; label: string; icon?: string; isClosable?: boolean }>` | none | yes | panel tabs |
| `value` | `string` | none | no | controlled active panel |
| `defaultValue` | `string` | none | no | uncontrolled initial active panel |
| `isReorderable` | `boolean` | `true` | no | supports reorder posture |
| `dockId` | `string \| null` | `null` | no | owning dock identity for multi-dock context |
| `ariaLabel` | `string \| null` | `null` | no | tab-strip label |
| `onValueChange` | `(value: string) => void` | none | no | active-panel callback |
| `onReorder` | `(items: string[]) => void` | none | no | reorder callback |
| `onRequestClose` | `(value: string) => void` | none | no | close-intent callback |
| `onRequestContextMenu` | `(value: string) => void` | none | no | menu-intent callback |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange`
- uncontrolled: `defaultValue`
- reorder, close, and menu actions are surfaced as host-owned intents

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| inactive | non-selected tab | subdued shell tab |
| active | selected panel | active shell tab |
| focus | roving focus reaches tab | visible focus treatment |
| drag-target | reorder hover or move mode | insertion feedback |

### Component States

Selected-tab state, roving-focus state, and optional reorder mode are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | active panel changes | panel value | selection |
| `onReorder` | reorder commits | ordered values | optional |
| `onRequestClose` | close affordance activates | panel value | optional |
| `onRequestContextMenu` | context action requested | panel value | optional |

## 6. Accessibility

### Semantics

- Role: tablist and tab semantics for panel-group navigation
- Required attributes: selected state, tab names, current-panel relationship
- Optional attributes: reorder instructions and tab-strip label
- Labeling rules: dense icon-first tabs still require stable accessible names

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Right` or `Arrow Up/Down` | moves focus between tabs |
| `Home/End` | moves focus to first/last tab |
| `Enter` or `Space` | activates focused tab |
| `Delete` | optional close intent for closable tabs |
| reorder shortcut plus arrows | reorders focused tab when supported |
| context-menu key | opens panel-tab context actions when supported |

### Focus And Announcement

- focus entry: roving focus lands on the active tab or first enabled tab
- focus restoration: after close or move, focus returns to the nearest valid tab
- live-region behavior: reorder or close actions should be announced when they
  materially change panel position or presence
- GPUI-native accessibility mapping notes: GPUI must preserve keyboard tab
  navigation, context access, and move announcements in a dense shell strip

## 7. Layout

### Sizing

- tabs may be compact and icon-forward
- overflow policy stays host-owned but active tab should remain visible

### Composition

- parent expectations: `PanelHeader`, `DockRegion`, floating panel groups
- child expectations: tab items only in the baseline contract
- resizing rules: focusability and current-tab visibility remain stable under
  compression

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Tab Strip | panel border and spacing roles | grouping |
| Panel Tab | selected, focus, text, and background roles | shell tab chrome |
| Affordances | icon and focus roles | close/menu actions |
| Reorder feedback | accent and border roles | insertion cue |

## 9. Svelte Notes

- expected substrate: `TabStrip` foundation contract with workstation-specific
- density and context affordances layered on top
- wrapper strategy: context menus and drag behavior stay Pug-owned shell
  behavior, not app-owned one-offs

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::workstation::panel_tabs`
- implementation-only details: GPUI may use native drag and menu hooks, but
  keyboard reorder, naming, and restoration semantics remain required

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] active-panel, close, and move semantics match
- [ ] keyboard navigation and context access match
- [ ] focus restoration after close or reorder matches

### Tier 2: Visual Parity

- [ ] compact tab density and active-state emphasis use comparable token roles

### Tier 3: Implementation Freedom

- [ ] drag preview and overflow mechanics stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact drag preview may differ | runtime drag systems differ | allowed | keep move meaning and keyboard parity strict |

## 13. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | 4 items with icons (Explorer/folder, Search/search, Source Control/git-branch, Debug/bug), controlled `value`, `ariaLabel="Panel tabs"` | Horizontal tab strip with icon+label tabs, one active tab highlighted, active tab value displayed below; clicking tabs switches active state |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: dock tab strips, floating panel groups
- future follow-up: connect deeper transfer/drop orchestration in later
  milestones

## Next Task

Use `PanelTabs` for region-local panel switching and let `DockRegion` own the
larger collapse and body-display semantics around the strip.
