# WorkspaceLayout

Status: seed contract
Updated: 2026-03-17

## 1. Purpose

- Component name: `WorkspaceLayout`
- Layer: `workstation`
- Summary: the region grammar and layout snapshot system for workstation shells,
  defining canonical region keys, snapshot shape, and region visibility rules
- In scope: region key vocabulary, layout snapshot structure, collapse and
  visibility state, split ratios, strip region state, center region state
- Out of scope: persistence backend, region content assignment, drag-drop
  reflow policy, app-specific default layouts

## 2. Anatomy

```text
[Workspace Layout]
  ├── [Top Strip Region] (optional)
  ├── [Main Body]
  │     ├── [Left Strip Region] (optional)
  │     ├── [Left Dock Region] (optional)
  │     ├── [Center Area]
  │     │     ├── [Center Top] (primary)
  │     │     └── [Center Bottom] (optional, split)
  │     ├── [Right Dock Region] (optional)
  │     └── [Right Strip Region] (optional)
  ├── [Bottom Dock Region] (optional)
  └── [Bottom Strip Region] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Top Strip | no | horizontal strip rail at top edge | strip roles |
| Bottom Strip | no | horizontal strip rail at bottom edge | strip roles |
| Left Strip | no | vertical strip rail at left edge | strip roles |
| Right Strip | no | vertical strip rail at right edge | strip roles |
| Left Dock | no | collapsible dock at left edge | dock roles |
| Right Dock | no | collapsible dock at right edge | dock roles |
| Top Dock | no | collapsible dock at top edge (below top strip) | dock roles |
| Bottom Dock | no | collapsible dock at bottom edge (above bottom strip) | dock roles |
| Center Top | yes | primary center work area | surface roles |
| Center Bottom | no | secondary center area when split | surface roles |

## 3. Props And Inputs

### Region Keys

The canonical region key vocabulary for workstation layouts:

| Key | Type | Position | Orientation |
|-----|------|----------|-------------|
| `topStrip` | strip | top edge | horizontal |
| `bottomStrip` | strip | bottom edge | horizontal |
| `leftStrip` | strip | left edge | vertical |
| `rightStrip` | strip | right edge | vertical |
| `left` | dock | left side | vertical |
| `right` | dock | right side | vertical |
| `top` | dock | top area (below top strip) | horizontal |
| `bottom` | dock | bottom area (above bottom strip) | horizontal |
| `centerTop` | center | primary center | — |
| `centerBottom` | center | secondary center (when split) | — |

### Layout Snapshot

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
    primary: number;
    secondary: number;
  };
};

type StripRegionSnapshot = {
  isCollapsed: boolean;
  activeItem: string | null;
};

type DockRegionSnapshot = {
  edge: DockEdge;
  isCollapsed: boolean;
  activePanel: string | null;
  order: string[];
  tabsPlacement: "edge" | "top";
};

type CenterRegionSnapshot = {
  activeSurface: string | null;
};
```

### Controlled And Uncontrolled

- layout snapshot is host-controlled
- components consume snapshot fields and emit change events
- the layout system does not self-mutate

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| minimal | only centerTop populated | simple single-pane layout |
| standard | docks and center populated | typical workstation layout |
| full | all regions populated including strips | dense workstation layout |
| center-split | centerBottom present | center area divides vertically |

### Region Visibility

- a region is visible when its snapshot entry exists and is not collapsed
- collapsed regions show their collapse affordance but hide their content
- absent region keys mean the region is not rendered at all

## 5. Events

| Event | Payload | When |
|-------|---------|------|
| `layoutChange` | `WorkspaceLayoutSnapshot` | any region state changes |
| `splitRatioChange` | `{ axis: "primary" \| "secondary", ratio: number }` | user adjusts a split |
| `regionCollapse` | `{ region: string, isCollapsed: boolean }` | region collapse/expand |

## 6. Accessibility

### Semantics

- each visible region should be a named landmark or region
- region names should be stable and meaningful (e.g., "left dock", "center
  editor")

### Keyboard

| Key | Behavior |
|-----|----------|
| focus navigation | moves between regions in logical order |
| region shortcuts | host-provided shortcuts for jumping to specific regions |

### Focus And Announcement

- collapsing a region moves focus to the collapse affordance or next visible
  region
- expanding a region moves focus into the newly visible content

## 7. Layout

### Sizing

- strips have fixed or compact sizing based on content
- docks have user-adjustable sizing via resize handles
- center area fills remaining space
- split ratios control the division of shared axes

### Composition

- parent expectations: WorkspaceWindow or WorkspaceShell
- child expectations: StripRail, DockRegion, SplitView, ResizeHandle
- the layout grammar defines region placement; individual region components
  define their own internal behavior

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Region borders | separator and border roles | region boundaries |
| Split dividers | divider roles | resize affordance |
| Background | surface background roles | region fill |

## 9. Svelte Notes

- the layout snapshot shape should be importable from the workstation package
- WorkspaceShell should accept the expanded snapshot to configure region
  visibility
- version field enables migration from v1 to v2 snapshot format

## 10. GPUI Notes

- snapshot type maps to a Rust struct
- region keys map to named layout slots in GPUI's layout system
- the same snapshot semantics apply regardless of GPUI layout mechanism

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] region key vocabulary matches
- [ ] snapshot shape matches
- [ ] collapse and visibility semantics match
- [ ] event payloads match

### Tier 2: Visual Parity

- [ ] region placement and proportions match
- [ ] collapse transitions use comparable timing

### Tier 3: Implementation Freedom

- [ ] layout mechanism internals stay renderer-specific
- [ ] snapshot serialization format can differ

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none anticipated | — | — | — |

## 13. Specimen Definitions

No specimen file exists. `WorkspaceLayoutSpecimen.svelte` needs to be created.

WorkspaceLayout is primarily a data structure and region grammar contract rather
than a visual component. A specimen should demonstrate the layout snapshot system
by showing how different snapshot configurations produce different region
arrangements:

### Group: Minimal layout

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Center only | Snapshot with only `centerTop` populated | Simple single-pane layout with no docks or strips |

### Group: Standard layout

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With docks | Snapshot with `leftStrip`, `left` dock, `centerTop`, `right` dock | Typical workstation layout with left strip, left dock, center area, and right dock |

### Group: Full layout

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| All regions | Snapshot with all strip, dock, and center regions populated | Dense workstation layout with all four strips, docks, and center area |

### Group: Center split

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Split center | Snapshot with `centerTop` and `centerBottom` populated | Center area divided vertically between primary and secondary regions |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: any workstation application using WorkspaceShell
- future follow-up: layout persistence, layout presets, animated region
  transitions

## Next Task

Use this contract as the basis for expanding the TypeScript types in
`packages/svelte/workstation/src/types.ts` and updating WorkspaceShell to
accept the expanded snapshot.
