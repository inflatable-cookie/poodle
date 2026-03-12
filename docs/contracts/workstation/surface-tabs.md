# SurfaceTabs

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `SurfaceTabs`
- Layer: `workstation`
- Summary: a shell tab strip for switching, reordering, renaming, and moving
  high-level workspace surfaces across windows
- In scope: active surface selection, reorder posture, rename intent, move to
  another window intent, add/remove affordances
- Out of scope: workspace persistence storage, panel transfer logic inside a
  surface, app-specific surface content

## 2. Anatomy

```text
[Root Tab Strip]
  ├── [Surface Tab...]
  └── [Add Surface Affordance] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Tab Strip | yes | workspace-surface strip | border, spacing |
| Surface Tab | yes | one workspace surface selector | text, background, focus, selected state |
| Add Surface Affordance | no | creates or reveals another surface | icon, focus |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `Array<{ value: string; label: string; isClosable?: boolean }>` | none | yes | surface list |
| `value` | `string` | none | no | controlled active surface |
| `defaultValue` | `string` | none | no | uncontrolled initial active surface |
| `isReorderable` | `boolean` | `true` | no | supports reorder posture |
| `ariaLabel` | `string \| null` | `null` | no | strip label |
| `onValueChange` | `(value: string) => void` | none | no | active-surface callback |
| `onReorder` | `(items: string[]) => void` | none | no | reorder callback |
| `onRequestRename` | `(value: string) => void` | none | no | rename-intent callback |
| `onRequestMove` | `(value: string) => void` | none | no | move-to-window intent callback |
| `onRequestClose` | `(value: string) => void` | none | no | close/remove intent callback |
| `onRequestAdd` | `() => void` | none | no | add-surface intent callback |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange`
- uncontrolled: `defaultValue`
- rename, move, close, and add behaviors remain host-owned intents

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| inactive | non-selected surface | subdued shell tab |
| active | selected surface | active shell tab |
| focus | roving focus reaches tab | visible focus treatment |
| renaming | host enters rename mode | inline-rename posture or equivalent |
| drag-target | reorder or move posture | insertion feedback |

### Component States

Selected-surface state, roving-focus state, and optional rename/reorder modes
are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | active surface changes | surface value | selection |
| `onReorder` | reorder commits | ordered values | optional |
| `onRequestRename` | rename requested | surface value | optional |
| `onRequestMove` | move requested | surface value | optional |
| `onRequestClose` | close/remove requested | surface value | optional |
| `onRequestAdd` | add surface requested | none | optional |

## 6. Accessibility

### Semantics

- Role: tablist and tab semantics for workspace-surface navigation
- Required attributes: selected state, stable tab names, current-surface
  relationship
- Optional attributes: reorder instructions and strip label
- Labeling rules: rename interactions must preserve a stable accessible identity
  before, during, and after rename

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Right` or `Arrow Up/Down` | moves focus between surface tabs |
| `Home/End` | moves focus to first/last surface tab |
| `Enter` or `Space` | activates focused surface tab |
| rename shortcut | requests rename when supported |
| reorder shortcut plus arrows | reorders focused surface when supported |
| context-menu key | opens surface-tab context actions when supported |

### Focus And Announcement

- focus entry: roving focus lands on the active surface or first enabled tab
- focus restoration: close or move returns focus to the nearest valid surface
  tab or documented fallback
- live-region behavior: rename, reorder, move, or close results should be
  announced when they materially affect current shell context
- GPUI-native accessibility mapping notes: GPUI must preserve rename context,
  move intent, and focus restoration across windows or shell surfaces

## 7. Layout

### Sizing

- tabs may compress or overflow according to shell policy
- add affordance remains reachable without hiding the active tab

### Composition

- parent expectations: `WorkspaceShell`, multi-surface workstation windows
- child expectations: surface tabs plus optional add affordance
- resizing rules: active surface remains identifiable under compression

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Tab Strip | shell border and spacing roles | workspace grouping |
| Surface Tab | selected, focus, text, and background roles | shell tab chrome |
| Add Affordance | action and focus roles | creation control |
| Reorder feedback | accent and border roles | insertion cue |

## 9. Svelte Notes

- expected substrate: `TabStrip`, `EditableLabel`, menu primitives, and shell
  action primitives
- wrapper strategy: move-to-window and rename flows remain host-owned intents

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::workstation::surface_tabs`
- implementation-only details: GPUI may realize move/rename with native menus
  and inline text editing, but shell semantics and restoration remain required

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] active-surface, rename, move, and close semantics match
- [ ] keyboard navigation and shell-context access match
- [ ] focus restoration after rename, close, or move matches

### Tier 2: Visual Parity

- [ ] shell-tab density and active-state hierarchy use comparable token roles

### Tier 3: Implementation Freedom

- [ ] overflow and cross-window move mechanics stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| cross-window drag visuals may differ | runtime drag/window systems differ | allowed | keep move meaning and focus restoration strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: multi-surface workstation windows, shell settings views
- future follow-up: connect richer workspace-layout persistence in later
  milestones

## Next Task

Use `SurfaceTabs` for top-level workspace-surface switching and keep panel
transfer semantics inside `DockRegion` and workspace orchestration.
