# WorkspaceWindow

Status: seed contract
Updated: 2026-03-17

## 1. Purpose

- Component name: `WorkspaceWindow`
- Layer: `workstation`
- Summary: a generalized workspace window host that owns a set of surfaces,
  tracks the active surface, and supports surface movement between windows
- In scope: window identity, surface-to-window ownership, active surface
  tracking, surface ordering, surface movement between windows, window-local
  focus posture
- Out of scope: native window chrome, window management policy (positions,
  sizes, defaults), app-specific toolbar behavior, project identity, product
  menus and commands, persistence backend

## 2. Anatomy

```text
[WorkspaceWindow]
  ├── [Window Identity]
  ├── [Surface Registry]
  │     ├── [Surface Entry] (1..n)
  │     │     ├── surface id
  │     │     ├── surface label
  │     │     └── surface state (active | inactive | detached)
  │     └── [Surface Ordering]
  └── [Shell Host]
        └── WorkspaceShell (existing)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Window Identity | yes | stable window id for multi-window coordination | — |
| Surface Registry | yes | ordered collection of surfaces owned by this window | — |
| Surface Entry | yes (1+) | individual surface with id, label, and state | — |
| Shell Host | yes | delegates to WorkspaceShell for layout and rendering | shell roles |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `windowId` | `string` | — | yes | stable identifier for this window |
| `surfaces` | `WindowSurface[]` | `[]` | no | ordered list of surfaces owned by this window |
| `activeSurfaceId` | `string \| null` | `null` | no | which surface is currently active |
| `ariaLabel` | `string \| null` | `null` | no | window-level label |

### Types

```typescript
type WindowSurface = {
  id: string;
  label: string;
  isClosable?: boolean;
};
```

### Controlled And Uncontrolled

- surface list and active surface are host-controlled
- window does not manage its own surface state internally

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| single-surface | one surface in registry | surface tabs may be hidden |
| multi-surface | multiple surfaces | surface tabs visible |
| no-surfaces | empty registry | empty window posture |
| focused | window has OS focus | active-window visual treatment |
| unfocused | window loses OS focus | quieter visual treatment |

### Component States

| State | Description |
|-------|-------------|
| ready | window has at least one surface and is interactive |
| empty | window has no surfaces, shows empty posture |

## 5. Events

| Event | Payload | When |
|-------|---------|------|
| `activeSurfaceChange` | `{ surfaceId: string }` | user switches active surface |
| `surfaceClose` | `{ surfaceId: string }` | user requests to close a surface |
| `surfaceReorder` | `{ order: string[] }` | user reorders surface tabs |
| `surfaceDetach` | `{ surfaceId: string }` | user detaches surface to new window |
| `surfaceReceive` | `{ surfaceId: string, fromWindowId: string }` | surface arrives from another window |

## 6. Accessibility

### Semantics

- Role: application window or named region
- Required attributes: stable window identity for multi-window screen reader
  navigation
- Labeling rules: each window should be distinguishable by label or active
  surface name

### Keyboard

| Key | Behavior |
|-----|----------|
| `Ctrl+Tab` / `Ctrl+Shift+Tab` | cycle surfaces within window (host-provided) |
| standard tab/focus | delegates to WorkspaceShell focus behavior |

### Focus And Announcement

- focus entry: active surface content receives focus when window gains focus
- surface switch: moving to a new surface announces the surface label
- GPUI-native: maps to GPUI window with focus tracking per surface

## 7. Layout

### Sizing

- window fills its OS-level bounds or assigned root container
- delegates interior layout to WorkspaceShell

### Composition

- parent expectations: OS window or application root
- child expectations: WorkspaceShell, which handles headers, docks, and body
- the window host is a coordination layer above WorkspaceShell, not a
  replacement for it

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Window container | shell background | root window surface |
| Focused/unfocused | elevation or border roles | window emphasis |

## 9. Svelte Notes

- expected substrate: a root-level component that wraps WorkspaceShell
- multi-window in browser context may use detached browser windows or
  dedicated root mounts
- surface movement uses event-based coordination between window instances
- the component does not manage browser window creation — that is host-level
  responsibility

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::workstation::workspace_window`
- maps naturally to GPUI's native window model
- surface-to-window ownership can leverage GPUI's `Model<T>` for shared state
- window focus integrates with OS-level focus events

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] surface identity and ordering semantics match
- [ ] active surface tracking matches
- [ ] surface movement events match
- [ ] window-to-surface ownership model matches

### Tier 2: Visual Parity

- [ ] focused/unfocused window treatment uses comparable tokens
- [ ] surface tab integration matches

### Tier 3: Implementation Freedom

- [ ] native window integration mechanics stay internal
- [ ] multi-window coordination mechanism stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| browser multi-window vs native multi-window | platform constraints | allowed | ensure event model is equivalent |

## 13. Specimen Definitions

Specimen reference: `WorkspaceWindowSpecimen.svelte`.

### Group: With surfaces

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With surfaces | `windowId="main"`, `surfaces=[{id:"editor", label:"Editor"}, {id:"preview", label:"Preview", isClosable:true}]`, `activeSurfaceId="editor"` | Window with surface tabs showing "Editor" (active) and "Preview" (closable); content area below with descriptive text |

### Group: Empty window

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Empty window | `windowId="secondary"`, `surfaces=[]`, `activeSurfaceId=null` | Empty window posture with no surface tabs; muted italic "No surfaces" placeholder in content area |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: Loophole Aura (Svelte), Loophole Spark (GPUI), future
  multi-window workstation applications
- future follow-up: window layout persistence, window arrangement policies

## Next Task

Use this contract as the basis for the `g11.002` milestone and implement in
`g11.010`.
