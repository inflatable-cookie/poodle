# WorkspaceShell

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `WorkspaceShell`
- Layer: `workstation`
- Summary: the top-level workstation content shell that coordinates headers,
  surface tabs, dock regions, central work area, and shell utility overlays
- In scope: shell region structure, active surface posture, dock and central
  region expectations, shell-level disabled/loading posture
- Out of scope: app-specific workspace commands, persistence backend, DAW
  surface contents

## 2. Anatomy

```text
[Root Shell]
  ├── [App Header] (optional)
  ├── [Project Header] (optional)
  ├── [Surface Tabs] (optional)
  ├── [Workspace Body]
  │     ├── [Left Dock] (optional)
  │     ├── [Center Work Area]
  │     └── [Right/Bottom Dock] (optional)
  └── [Utility Overlays Host] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Shell | yes | workstation shell container | background, spacing |
| App Header | no | global shell header | shell chrome |
| Project Header | no | project/workspace context row | shell chrome |
| Surface Tabs | no | surface navigation row | border, tab roles |
| Workspace Body | yes | main layout region | split, dock, spacing |
| Utility Overlays Host | no | command palette, dialogs, menus | overlay roles |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `isDisabled` | `boolean` | `false` | no | shell interaction suppression posture |
| `activeSurfaceLabel` | `string \| null` | `null` | no | optional current-surface summary |
| `ariaLabel` | `string \| null` | `null` | no | optional shell label |

### Controlled And Uncontrolled

- declarative shell composition
- active surface, region layout, and overlay state remain host-owned

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| standard | default | shell fully interactive |
| disabled | `isDisabled=true` | visible blocked/degraded shell posture |
| multi-surface | surface tabs present | surface-navigation row visible |
| utility-open | overlay host contains modal or utility surface | shell reflects overlay layering |

### Component States

State table is sufficient for the baseline shell.

## 5. Events

No component-owned events beyond child shell-region behavior.

## 6. Accessibility

### Semantics

- Role: main workstation region or named root shell container
- Required attributes: stable shell or current-workspace naming when the shell
  is an addressable destination
- Optional attributes: active-surface descriptions
- Labeling rules: shell-level disabled posture must not silently make focused
  content unreachable without a clear fallback

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves through headers, surface tabs, docks, center content, and utility surfaces in logical order |
| shell shortcuts | remain host-owned but must respect focused text-entry and modal rules |

### Focus And Announcement

- focus entry: shell should provide sensible first reachable controls in headers
  or active regions
- focus restoration: closing shell-level overlays returns focus to the invoking
  shell control or a reasonable fallback region
- live-region behavior: shell itself does not announce by default; child
  overlays, status indicators, and banners own announcements
- GPUI-native accessibility mapping notes: GPUI must preserve named-region
  hierarchy across shell areas and maintain deterministic focus paths in dense
  keyboard-driven layouts

## 7. Layout

### Sizing

- shell fills the window or assigned root region
- workspace body owns the primary flexible space

### Composition

- parent expectations: top-level app window content
- child expectations: `AppHeader`, `ProjectHeader`, `SurfaceTabs`,
  `SplitView`, `DockRegion`, `CommandPaletteShell`, and overlay primitives
- resizing rules: headers remain non-scrolling; body regions own scroll and
  split behavior

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Shell | background and spacing roles | root shell |
| Header Rows | shell chrome roles | top identity/context |
| Surface Tabs | tab-strip and border roles | multi-surface nav |
| Workspace Body | split/dock sizing and surface roles | main layout |
| Utility Overlays Host | overlay, elevation, and motion roles | shell utilities |

## 9. Svelte Notes

- expected substrate: shell-level `Stack`, `SplitView`, `DockRegion`, and
  overlay primitives
- wrapper strategy: app-specific workspace command wiring remains external

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::workstation::workspace_shell`
- implementation-only details: GPUI may integrate more directly with native
  window views, but named-region hierarchy, focus continuity, and shell layering
  remain required

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] shell-region hierarchy and active-surface posture match
- [ ] disabled-shell meaning and focus fallback behavior match
- [ ] overlay restoration within the shell matches

### Tier 2: Visual Parity

- [ ] shell hierarchy and dock/header proportions use comparable token roles

### Tier 3: Implementation Freedom

- [ ] root window integration and orchestration mechanics stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact shell chrome packing may differ | runtime shell integration differs | allowed | keep hierarchy and focus rules strict |

## 13. Specimen Definitions

Specimen reference: `WorkspaceShellSpecimen.svelte`.

### Group: Default shell

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default shell | Slots populated: `appHeader` ("App Header"), `projectHeader` ("Project Header"), `surfaceTabs` ("Surface Tabs"), default slot ("Center content"), `statusBar` ("Status Bar") | Full shell layout with all header rows, center content area, and status bar; each slot shown as a labeled dashed-border placeholder |

### Group: Loading state

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Loading | `state="loading"`, `stateTitle="Loading workspace"`, `stateMessage="Please wait..."` | Shell in loading posture with title and message displayed; no slot content visible |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: Aura-like workspace windows, Spark-like GPUI roots,
  future desktop tools
- future follow-up: connect deeper persistence and multi-window orchestration in
  later milestones

## Next Task

Use `WorkspaceShell` as the shell composition boundary and keep app-specific
workspace logic above the contract layer.
