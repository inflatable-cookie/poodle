# DockRegion

Status: seed contract
Updated: 2026-03-17

## 1. Purpose

- Component name: `DockRegion`
- Layer: `workstation`
- Summary: a collapsible dock area that hosts panel tabs and one active panel
  body within a workstation shell
- In scope: edge placement, collapse/expand posture, active panel selection,
  panel-tab strip, empty-drop posture, active-panel emphasis, quieter inactive
  treatment, collapsed-tab posture with icon-only strip
- Out of scope: full drag/drop engine, persistence backend, DAW-specific panel
  contents

## 2. Anatomy

```text
[Root Region]
  ├── [Dock Strip]
  │     ├── [Panel Tabs]
  │     └── [Collapse Affordance]
  └── [Active Panel Body] (conditional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Region | yes | dock container | border, background, sizing |
| Dock Strip | yes | tab/collapse chrome | spacing, separator |
| Panel Tabs | yes | region panel selectors | selected, focus, tab roles |
| Collapse Affordance | no | collapse/expand control | icon, focus |
| Active Panel Body | no | visible active panel surface | surface, border |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `edge` | `"left" \| "right" \| "top" \| "bottom"` | none | yes | dock placement |
| `isCollapsed` | `boolean` | `false` | no | collapse posture |
| `tabsPlacement` | `"edge" \| "top"` | `"edge"` | no | strip placement |
| `items` | `Array<{ value: string; label: string; icon?: string }>` | none | yes | hosted panels |
| `value` | `string \| null` | `null` | no | controlled active panel |
| `emphasis` | `"standard" \| "quiet" \| "strong"` | `"standard"` | no | visual weight of active panel |
| `collapsedPosture` | `"hidden" \| "icon-strip"` | `"hidden"` | no | what to show when collapsed |
| `ariaLabel` | `string \| null` | `null` | no | region label |
| `onValueChange` | `(value: string) => void` | none | no | active-panel callback |
| `onCollapsedChange` | `(collapsed: boolean) => void` | none | no | collapse callback |
| `onRequestContextMenu` | `(value: string \| null) => void` | none | no | tab or list context-menu intent |

### Controlled And Uncontrolled

- controlled active panel recommended in shell use
- collapse posture is externally owned through `isCollapsed` plus
  `onCollapsedChange`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| expanded | default | strip plus active panel body visible |
| collapsed-hidden | `isCollapsed=true`, `collapsedPosture="hidden"` | strip remains, body hidden |
| collapsed-icon-strip | `isCollapsed=true`, `collapsedPosture="icon-strip"` | icon-only strip visible, body hidden |
| empty | no items | empty-drop or placeholder posture |
| active-standard | `emphasis="standard"` | one active panel visible, standard treatment |
| active-quiet | `emphasis="quiet"` | active panel visible, quieter chrome and border |
| active-strong | `emphasis="strong"` | active panel visible, stronger border and header emphasis |

### Component States

Expanded/collapsed state, active-panel state, and empty state are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | active panel changes | panel value | selection |
| `onCollapsedChange` | region collapses or expands | boolean | shell intent |
| `onRequestContextMenu` | context actions requested | panel value or `null` | optional |

## 6. Accessibility

### Semantics

- Role: named complementary region, group, or addressable panel area
- Required attributes: stable region label and active-panel semantics when a
  panel is visible
- Optional attributes: empty-drop descriptions and collapse-state descriptions
- Labeling rules: collapsed regions still need a discoverable accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches panel tabs, collapse affordance, and active panel body in order |
| panel-tab keys | follow `PanelTabs` semantics |
| collapse shortcut or `Enter`/`Space` on affordance | toggles collapsed state |

### Focus And Announcement

- focus entry: collapsed regions should still expose their strip and controls
  predictably
- focus restoration: collapsing a region with focused body content returns focus
  to the strip or collapse affordance
- live-region behavior: collapse and expand state changes should be announced
  through control semantics; active-panel changes should remain perceivable
- GPUI-native accessibility mapping notes: GPUI must expose dock regions as
  named shell areas with explicit collapse state and active-panel relationships

## 7. Layout

### Sizing

- dock width or height is edge-dependent and host-owned
- collapsed state keeps the strip reachable while removing body footprint

### Composition

- parent expectations: `WorkspaceShell` and `SplitView`
- child expectations: `PanelTabs`, `PanelHeader`, `PanelSurface`, placeholders
- resizing rules: strip thickness remains stable across collapse state

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Region | shell border, background, and sizing roles | dock chrome |
| Dock Strip | separator and spacing roles | tab/collapse grouping |
| Panel Tabs | panel-tab selected and focus roles | navigation |
| Active Panel Body | `PanelSurface` and border roles | visible panel |
| Empty posture | surface and subdued text roles | drop/empty cue |

## 9. Svelte Notes

- expected substrate: `PanelTabs`, `PanelSurface`, `ScrollShell`, and layout
  primitives
- wrapper strategy: drag/drop and context-menu wiring remain host-owned
  orchestration layered above the contract

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::workstation::dock_region`
- implementation-only details: GPUI may realize collapsed strips and body
  mounting natively, but region naming, collapse semantics, and active-panel
  mapping remain required

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] region naming, collapse semantics, and active-panel meaning match
- [ ] focus restoration on collapse matches
- [ ] tab/body order and reachability match

### Tier 2: Visual Parity

- [ ] strip density, body separation, and collapsed posture use comparable token roles

### Tier 3: Implementation Freedom

- [ ] drag/drop and mount strategy stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact collapsed-strip sizing may differ | runtime layout differs | allowed | keep reachability and collapse meaning strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: inspectors, browsers, history and utility docks
- future follow-up: connect richer drop targets and transfer orchestration in
  later workstation milestones

## Next Task

Use `DockRegion` for shell panel groups and keep the full panel move/reorder
engine in later orchestration milestones.
