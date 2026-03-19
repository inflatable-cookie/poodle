# HostedSurface

Status: seed contract
Updated: 2026-03-17

## 1. Purpose

- Component name: `HostedSurface`
- Layer: `workstation`
- Summary: a generalized container for hosting external, embedded, or foreign
  content surfaces within a workstation shell
- In scope: container identity and title, focus and active state, embedded
  versus detached hosting relationship, bounded host states (ready, loading,
  unavailable, blocked, degraded), status messaging
- Out of scope: plugin discovery and installation, plugin lifecycle management,
  plugin routing and version control, product-specific editor workflows

## 2. Anatomy

```text
[Hosted Surface]
  ├── [Host Header]
  │     ├── [Surface Title]
  │     ├── [Status Indicator] (optional)
  │     └── [Host Actions] (optional, e.g., detach, reload, close)
  ├── [Content Viewport]
  │     └── [External Content] (slot)
  └── [Status Overlay] (conditional)
        └── [Status Message]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Hosted Surface | yes | root container | border, background, elevation |
| Host Header | yes | identity and status chrome | shell chrome, typography |
| Surface Title | yes | name of hosted content | typography roles |
| Status Indicator | no | visual state indicator | badge/status roles |
| Host Actions | no | detach, reload, close affordances | icon button roles |
| Content Viewport | yes | region for external content | surface background |
| Status Overlay | no | covers content when in non-ready state | overlay roles |
| Status Message | no | human-readable status explanation | typography, muted text |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `surfaceId` | `string` | — | yes | stable identity for the hosted surface |
| `title` | `string` | — | yes | display name |
| `state` | `HostedSurfaceState` | `"ready"` | no | current host state |
| `stateMessage` | `string \| null` | `null` | no | human-readable status for non-ready states |
| `hosting` | `"embedded" \| "detached"` | `"embedded"` | no | hosting relationship |
| `isActive` | `boolean` | `false` | no | currently focused/selected |
| `isDetachable` | `boolean` | `false` | no | shows detach affordance |
| `isReloadable` | `boolean` | `false` | no | shows reload affordance |
| `isClosable` | `boolean` | `false` | no | shows close affordance |
| `ariaLabel` | `string \| null` | `null` | no | accessible label |

### Types

```typescript
type HostedSurfaceState =
  | "ready"
  | "loading"
  | "unavailable"
  | "blocked"
  | "degraded";
```

### Controlled And Uncontrolled

- state is host-controlled
- the component renders appropriate status overlays based on state
- content viewport is a slot — host provides the actual foreign content

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"` | content viewport visible, no overlay |
| loading | `state="loading"` | loading indicator overlay on content |
| unavailable | `state="unavailable"` | status overlay with message, no content visible |
| blocked | `state="blocked"` | status overlay with block message |
| degraded | `state="degraded"` | content visible but with warning indicator |
| active | `isActive=true` | stronger border or focus ring |
| detached | `hosting="detached"` | visual indicator that surface is externally hosted |

## 5. Events

| Event | Payload | When |
|-------|---------|------|
| `requestDetach` | `{ surfaceId: string }` | user requests to detach to external window |
| `requestAttach` | `{ surfaceId: string }` | user requests to re-embed from external window |
| `requestReload` | `{ surfaceId: string }` | user requests content reload |
| `requestClose` | `{ surfaceId: string }` | user requests to close hosted surface |
| `focusChange` | `{ surfaceId: string, isFocused: boolean }` | hosted surface gains/loses focus |

## 6. Accessibility

### Semantics

- Role: `region` with accessible name from title
- Required attributes: `aria-label` or `aria-labelledby` from title
- Status changes should be announced via `aria-live` on the status overlay

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | enters header actions, then content viewport |
| `Escape` | returns focus to host panel (if applicable) |

### Focus And Announcement

- focus entry: when activated, focus moves to content viewport or first
  interactive element within
- state changes: non-ready states announce via live region
- detached surfaces: focus coordination between windows is host-managed

## 7. Layout

### Sizing

- fills its parent container (panel body or dedicated region)
- content viewport fills remaining space after header
- status overlay covers content viewport

### Composition

- parent expectations: PanelSurface body, DockRegion, or standalone region
- child expectations: foreign content provided via slot
- the host container handles chrome; the content is opaque to Pug

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Container | surface background, border roles | host boundary |
| Header | shell chrome, typography | identity row |
| Status indicator | badge/status color roles | state indicator |
| Content viewport | surface background | content area |
| Status overlay | overlay background, muted text | non-ready states |
| Active state | accent/focus border | active emphasis |

## 9. Svelte Notes

- content viewport uses a slot for embedded content
- for web-based external content, the slot may contain an iframe or web
  component
- detached hosting uses browser window coordination (same as WorkspaceWindow
  surface movement)
- state overlay uses existing overlay primitives

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::workstation::hosted_surface`
- content viewport maps to a GPUI view or element host
- for native plugin UIs, the viewport may host a platform view
- state management via `Model<HostedSurfaceState>`

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] state semantics and transitions match
- [ ] event payloads match
- [ ] focus behavior matches
- [ ] accessibility announcements match

### Tier 2: Visual Parity

- [ ] header, status, and overlay treatment use comparable tokens
- [ ] active state treatment matches

### Tier 3: Implementation Freedom

- [ ] content hosting mechanism stays renderer-specific
- [ ] detached window coordination stays renderer-specific

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| content hosting mechanism differs | iframe vs native view vs UiTree node | allowed | ensure state and focus semantics are equivalent |

## 13. Specimen Definitions

Specimen file: `HostedSurfaceSpecimen.svelte` (not yet created).

The specimen should demonstrate the following groups based on the contract's states and props:

### Group: Ready state (default)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Embedded ready | `surfaceId="demo"`, `title="Plugin Editor"`, `state="ready"`, `hosting="embedded"`, content slot with placeholder | Host header with title, content viewport visible, no overlay |

### Group: Host states

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Loading | `state="loading"`, `title="Plugin Editor"` | Loading indicator overlay covering content viewport |
| Unavailable | `state="unavailable"`, `stateMessage="Service unavailable"`, `title="Plugin Editor"` | Status overlay with message, content hidden |
| Blocked | `state="blocked"`, `stateMessage="Access denied"`, `title="Plugin Editor"` | Status overlay with block message |
| Degraded | `state="degraded"`, `stateMessage="Limited functionality"`, `title="Plugin Editor"` | Content visible with warning indicator in header |

### Group: Action affordances

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| All actions | `state="ready"`, `title="Plugin Editor"`, `isDetachable`, `isReloadable`, `isClosable` | Header shows detach, reload, and close action buttons |
| Active surface | `state="ready"`, `title="Plugin Editor"`, `isActive` | Stronger border or focus ring indicating active state |

### Group: Hosting modes

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Embedded | `hosting="embedded"`, `title="Plugin Editor"`, `state="ready"` | Standard embedded surface appearance |
| Detached | `hosting="detached"`, `title="Plugin Editor"`, `state="ready"` | Visual indicator that surface is externally hosted |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: Loophole Aura (plugin editor hosts), any workstation
  application embedding foreign content
- future follow-up: plugin lifecycle coordination, content sandboxing,
  communication channel between host and content

## Next Task

Implement HostedSurface in Svelte during `g11.011`.
