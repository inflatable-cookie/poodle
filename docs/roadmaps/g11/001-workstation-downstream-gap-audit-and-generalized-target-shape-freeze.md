# g11.001 — Workstation Downstream Gap Audit And Generalized Target-Shape Freeze

Status: complete
Owner: Pug Core
Updated: 2026-03-17
Depends on: g10.016
Primary repos: `pug`, `loophole`

## Goals

- [x] audit real downstream workstation pressure and convert it into a Pug-safe
  generalized target shape
- [x] separate substrate work that belongs in Pug from product semantics that
  must stay downstream
- [x] produce the execution-ready intake for the rest of `g11`

## Current Workstation Surface

### Existing Components (12)

| Component | Contract | Description |
|-----------|----------|-------------|
| WorkspaceShell | ✓ | Top-level shell composition with headers, tabs, body, overlays |
| SplitView | ✓ | Resizable two-pane divider with orientation, ratio, collapse |
| DockRegion | ✓ | Collapsible dock with panel tabs and active panel body |
| PanelSurface | ✓ | Panel container with header, body, scroll, active/elevated states |
| PanelHeader | ✓ | Panel chrome row with title, tabs, actions |
| SurfaceTabs | ✓ | Multi-surface navigation strip with rename/move/close/add |
| PanelTabs | ✓ | Dock panel selection strip with icons and context menu |
| AppHeader | ✓ | Global shell header with identity, actions, utility regions |
| ProjectHeader | ✓ | Project context row with title, subtitle, dirty state |
| ShellStatusBar | ✓ | Footer status bar with leading and trailing content |
| CommandPalette | ✓ | Modal command-discovery surface with grouped results |
| ActionDiscoveryPanel | ✓ | Non-modal action discovery alternative |

### Existing Type Surface

- `WorkspaceLayoutSnapshot` — covers `activeSurface`, `surfaceOrder`,
  `primarySplitRatio`, `secondarySplitRatio`, `leftDock`, `rightDock`
- `DockRegionSnapshot` — covers `edge`, `isCollapsed`, `activePanel`, `order`
- `DockEdge` — `"left" | "right" | "top" | "bottom"`
- `SplitOrientation` — `"horizontal" | "vertical"`
- `SurfaceTabItem` — `value`, `label`, `isClosable`
- `PanelTabItem` — `value`, `label`, `icon`, `isClosable`

## Gap Audit Against Downstream Needs

### 1. Window Hosts and Surface Ownership

**Current state:** WorkspaceShell is a single-window composition boundary.
`activeSurfaceLabel` is a display string, not a surface identity model.
`WorkspaceLayoutSnapshot` tracks `activeSurface` and `surfaceOrder` but has no
concept of window identity, multi-window ownership, or surface movement between
windows.

**Downstream pressure:** Multi-window workstation applications need surfaces to
have stable identity, belong to a specific window, and move between windows.
Loophole Aura needs this for detachable editor surfaces and multi-monitor
layouts.

**Gap classification: Core Pug substrate.** The window host model, surface
identity, and surface-to-window ownership belong in Pug as generalized
workstation semantics. Product-specific window policies (which surfaces open on
launch, default window positions, etc.) stay downstream.

### 2. Region Grammar and Layout Snapshots

**Current state:** `WorkspaceLayoutSnapshot` supports a fixed two-split layout
with left and right docks. No strip regions, no top/bottom docks, no
center-split semantics. The snapshot cannot express:
- strip rails at any edge
- top or bottom dock regions
- a center area divided into `centerTop` and `centerBottom`
- per-region visibility independent of collapse state

**Downstream pressure:** Real workstation shells need more than two docks and
two splits. Loophole Aura uses left strip (activity bar), bottom strip (status
affordances), left dock, right dock, bottom dock, and a center area that can
split vertically for timeline/editor layouts.

**Gap classification: Core Pug substrate.** The region grammar and expanded
snapshot belong in Pug. The specific assignment of downstream content to regions
stays downstream.

### 3. Strip Rails

**Current state:** No strip rail components exist. The concept of a compact
icon-first activity bar or edge rail is not in the workstation surface.

**Downstream pressure:** Strip rails are a standard workstation pattern (VS
Code activity bar, DAW tool strips). Loophole Aura uses a left strip for
view/panel switching and a bottom strip for workspace-level status affordances.

**Gap classification: Core Pug substrate.** Strip rails for all four edges
with icon-first and mixed-content modes, active/idle/compact/collapsed
variants. Product-specific strip item semantics (which icons, what they
activate) stay downstream.

### 4. Resize/Collapse Behavior

**Current state:** SplitView provides resize via drag and keyboard (arrow
keys, Home/End) and supports `isPrimaryCollapsed`/`isSecondaryCollapsed`.
However:
- no standalone resize handle primitive
- no standalone split divider component
- no standalone collapse/expand affordance
- collapse is binary (collapsed or not), no animated or progressive collapse
- no programmatic resize events separate from user-initiated resize

**Downstream pressure:** Workstation shells need resize and collapse as
reusable interaction primitives, not just embedded in SplitView. Dock regions,
strip rails, and arbitrary layout regions all need resize and collapse without
reimplementing the interaction model.

**Gap classification: Core Pug substrate.** Resize handles, split dividers,
and collapse affordances as standalone interaction primitives. Animation and
progressive collapse details are renderer-specific implementation concerns.

### 5. Dock Semantics

**Current state:** DockRegion supports:
- edge placement (left/right/top/bottom)
- collapse/expand
- tab placement (edge or top)
- active panel selection
- panel tab reorder, close, and context menu

Missing:
- no explicit active-panel emphasis beyond tab highlighting
- no quieter inactive-panel visual treatment
- no collapsed-tab posture (showing icons-only in collapsed state)
- no dock-local status without app-specific logic

**Downstream pressure:** Real workstation docks need stronger visual hierarchy
between active and inactive panels, and a useful collapsed posture that shows
which panels are available without fully expanding the dock.

**Gap classification: Core Pug substrate.** Active-panel emphasis, quieter
inactive treatment, and collapsed-tab posture belong in Pug. Dock-specific
business logic (auto-collapse on focus change, panel pinning policy) stays
downstream.

### 6. Panel Variants

**Current state:** PanelSurface has `isActive`, `isElevated`, `bodyPadding`,
and `scrollMode`. No variant system for distinguishing utility/support panels
from focused/detail panels. All panels share the same visual weight.

**Downstream pressure:** Workstation applications need visually quieter
utility panels (file browsers, logs, output) and visually stronger focused
panels (editors, inspectors, detail views). Without variants, downstream apps
add one-off CSS overrides to differentiate panel importance.

**Gap classification: Core Pug substrate.** A generalized panel variant system
(utility/standard/focused) with explicit header, body, border, and background
deltas. Product-specific panel assignments stay downstream.

### 7. Hosted External Surfaces

**Current state:** No hosted external surface model exists. Plugin UIs,
embedded web views, or foreign content containers require fully custom
downstream implementations.

**Downstream pressure:** Loophole Aura needs to host plugin editor UIs
(both embedded and detachable) with consistent title, focus, and status
treatment. Other workstation applications have similar needs for embedded
foreign content.

**Gap classification: Core Pug substrate.** A generalized hosted-surface
container with identity, focus, active state, and bounded status (ready,
loading, unavailable, blocked, degraded). Plugin workflow semantics (discovery,
installation, routing, lifecycle management) stay downstream.

## Generalized Target Shape for g11

### New Components

| Component | Contract File | Description |
|-----------|--------------|-------------|
| WorkspaceWindow | `workspace-window.md` | Window host with surface ownership, ordering, focus |
| StripRail | `strip-rail.md` | Four-edge strip with icon-first/mixed modes, active/idle/compact/collapsed |
| ResizeHandle | `resize-handle.md` | Standalone resize interaction primitive |
| SplitDivider | `split-divider.md` | Standalone split divider with hit target and visual affordance |
| CollapseAffordance | `collapse-affordance.md` | Standalone collapse/expand trigger |
| HostedSurface | `hosted-surface.md` | Generalized container for foreign/external content |

### Existing Components to Deepen

| Component | Contract Update | Changes |
|-----------|----------------|---------|
| DockRegion | `dock-region.md` | Active-panel emphasis, quieter inactive treatment, collapsed-tab posture |
| SurfaceTabs | `surface-tabs.md` | Window-aware identity, surface movement between windows |
| PanelTabs | `panel-tabs.md` | Window-aware context for dock-local panel switching |
| PanelSurface | `panel-surface.md` | Variant system: utility, standard, focused |

### Snapshot Expansion

`WorkspaceLayoutSnapshot` expands to cover:

```typescript
type WorkspaceLayoutSnapshot = {
  version: 2;
  activeSurface: string;
  surfaceOrder: string[];
  regions: {
    topStrip?: StripRegionSnapshot;
    bottomStrip?: StripRegionSnapshot;
    leftStrip?: StripRegionSnapshot;
    rightStrip?: StripRegionSnapshot;
    left?: DockRegionSnapshot;
    right?: DockRegionSnapshot;
    top?: DockRegionSnapshot;
    bottom?: DockRegionSnapshot;
    centerTop?: CenterRegionSnapshot;
    centerBottom?: CenterRegionSnapshot;
  };
  splitRatios: {
    primary: number;    // left-right split
    secondary: number;  // center top-bottom split
  };
};

type StripRegionSnapshot = {
  isCollapsed: boolean;
  activeItem: string | null;
};

type CenterRegionSnapshot = {
  activeSurface: string | null;
};
```

## Explicit Non-Goals (Keep Out of Pug)

The following must remain downstream and product-specific:

- **DAW semantics:** transport, timeline, browser, mixer, inspector, automation,
  clip editor, device chain, plugin workflow — all Loophole-specific
- **Window management policy:** which surfaces open on launch, default window
  positions, window persistence strategy — all product-specific
- **Panel assignment policy:** which panels belong in which docks by default,
  panel pinning rules, auto-collapse behavior — all product-specific
- **Strip item semantics:** which icons appear in strips, what they activate,
  strip item badges and indicators — all product-specific
- **Plugin lifecycle:** discovery, installation, activation, routing, version
  management — all product-specific
- **Command routing:** which commands are available, keyboard shortcut
  assignments, command categorization — all product-specific
- **Project identity:** project names, file paths, save/load behavior, dirty
  tracking logic — all product-specific (ProjectHeader's `isDirty` is a
  display concern, not a project management concern)

## Execution Checklist

- [x] audit the current Pug workstation surface against downstream needs:
  - [x] window hosts and surface ownership
  - [x] region grammar and layout snapshots
  - [x] strip rails
  - [x] resize/collapse behavior
  - [x] dock semantics
  - [x] panel variants
  - [x] hosted external surfaces
- [x] review real downstream evidence from Loophole Aura's current-versus-legacy
  shell audit and Pug gap inventory
- [x] classify each discovered gap as:
  - [x] belongs in core Pug workstation substrate
  - [x] belongs in renderer-specific implementation only
  - [x] stays downstream and product-specific
- [x] freeze a generalized target shape for `g11`
- [x] record explicit non-goals so Pug does not drift into app-specific shell
  design

## Acceptance Criteria

- [x] `g11` has one explicit downstream-driven target shape instead of an
  informal list of component wishes
- [x] every major workstation gap is classified as core, renderer-local, or
  downstream-only
- [x] at least one explicit "keep this out of Pug" section is recorded
- [x] the next milestones can proceed without reopening scope

## Next Task

Open `g11.002` and define the workspace window host and surface-ownership
contract.
