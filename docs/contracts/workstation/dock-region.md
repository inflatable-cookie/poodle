# DockRegion

Status: active contract
Updated: 2026-03-18

## 1. Purpose

- Component name: `DockRegion`
- Layer: `composite` (implemented in `@pug/svelte-workstation`)
- Summary: a collapsible dock area that hosts panel tabs and one active panel
  body, or stacks multiple fixed panels, within a workstation layout
- In scope: edge placement, static/flexible sizing modes, collapse/expand with
  icon-strip and hidden postures, active panel selection via Tabs primitive,
  cross-region panel drag-and-drop with validation, auto-compact icon-only
  mode when tabs overflow, click-to-expand from collapsed state
- Out of scope: persistence backend, DAW-specific panel contents, resize
  handle (handled externally by SplitView)

## 2. Anatomy

### Flexible mode (expanded)

```text
[Root Region]
  ├── [Strip]
  │     ├── [Tabs (variant="strip", horizontal)]
  │     └── [CollapseToggle]
  └── [Body] (active panel content)
```

### Flexible mode (collapsed icon-strip, left/right edge)

```text
[Root Region]
  └── [Strip (vertical)]
        ├── [CollapseToggle]
        └── [Tabs (variant="strip", vertical, icon-only)]
```

### Flexible mode (collapsed icon-strip, top/bottom edge)

```text
[Root Region]
  └── [Strip (horizontal)]
        ├── [Tabs (variant="strip", horizontal, compact icon-only)]
        └── [CollapseToggle]
```

### Flexible mode (collapsed hidden)

```text
[Root Region]
  └── [CollapseToggle]
```

### Static mode

```text
[Root Region]
  └── [Stack]
        ├── [Stack Item] (draggable, reorderable)
        ├── [Stack Item]
        └── ...
```

| Part | Required | Description |
|------|----------|-------------|
| Root Region | yes | `<section>` dock container with `aria-label` |
| Strip | flexible only | tab/collapse chrome area |
| Tabs | flexible only | Tabs primitive (variant="strip") |
| CollapseToggle | flexible only | collapse/expand affordance |
| Body | flexible expanded only | active panel content slot |
| Stack | static only | flex container for stacked panels |
| Stack Item | static only | draggable panel wrapper |
| Drop Zone | conditional | overlay shown during cross-region drag |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `edge` | `DockEdge` | `"left"` | no | dock placement: `"left" \| "right" \| "top" \| "bottom"` |
| `sizing` | `DockSizing` | `"flexible"` | no | `"static"` for fixed stacked panels, `"flexible"` for tabbed/collapsible |
| `isCollapsed` | `boolean` | `false` | no | collapse state (flexible mode only) |
| `collapsedPosture` | `DockCollapsedPosture` | `"icon-strip"` | no | `"hidden"` or `"icon-strip"` |
| `emphasis` | `DockEmphasis` | `"standard"` | no | `"standard" \| "quiet" \| "strong"` |
| `items` | `PanelTabItem[]` | `[]` | no | panel definitions with value, label, icon, isClosable |
| `value` | `string \| null` | `null` | no | controlled active panel (flexible mode) |
| `ariaLabel` | `string \| null` | `null` | no | region accessible label |
| `canAcceptPanel` | `(panelId: string, sourceEdge: DockEdge) => boolean \| null` | `null` | no | cross-region drop validation |

### PanelTabItem

```ts
type PanelTabItem = {
  value: string;
  label: string;
  icon?: string | null;
  isClosable?: boolean;
};
```

All items should have `icon` set when used in flexible docks, as collapsed/compact
modes render icon-only tabs.

### Controlled And Uncontrolled

- Active panel (`value`) is typically controlled by the parent shell
- Collapse state (`isCollapsed`) is externally owned via `collapsedChange` event

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| expanded | `sizing="flexible"`, `isCollapsed=false` | horizontal strip tabs + body visible |
| collapsed icon-strip (side) | `isCollapsed=true`, `collapsedPosture="icon-strip"`, left/right edge | vertical icon-only tabs + collapse toggle at top |
| collapsed icon-strip (top/bottom) | `isCollapsed=true`, `collapsedPosture="icon-strip"`, top/bottom edge | horizontal icon-only tabs + collapse toggle, body hidden |
| collapsed hidden | `isCollapsed=true`, `collapsedPosture="hidden"` | only collapse toggle visible |
| static | `sizing="static"` | stacked panels, no tabs or collapse |
| compact | auto-detected | horizontal tabs collapse to icon-only when strip overflows |
| drag-over | cross-region drag enters | dashed accent border overlay |
| emphasis quiet | `emphasis="quiet"` | transparent border and background |
| emphasis strong | `emphasis="strong"` | accent-tinted border |

### Compact Mode

When horizontal tabs overflow their container, DockRegion automatically hides labels
and close buttons, showing icon-only tabs with bottom-positioned tooltips on hover.
Uses `ResizeObserver` with overflow detection and hysteresis to prevent oscillation.

### Click-to-Expand

Clicking a tab in any collapsed state dispatches both `valueChange` (to activate
the panel) and `collapsedChange` with `isCollapsed: false` (to expand the region).

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | active panel changes | `{ value: string }` | tab click or keyboard |
| `collapsedChange` | region collapses or expands | `{ isCollapsed: boolean }` | toggle click or tab click when collapsed |
| `close` | closable tab dismissed | `{ value: string }` | forwarded from Tabs |
| `reorder` | tabs or stack items reordered | `{ items: string[] }` | within-region reorder |
| `panelDrop` | panel dropped from another region | `{ panel: PanelDragData; targetEdge: DockEdge }` | cross-region transfer |

### PanelDragData

```ts
type PanelDragData = {
  panelId: string;
  sourceEdge: DockEdge;
};
```

## 6. Accessibility

### Semantics

- Role: `<section>` with `aria-label` identifying the dock region
- Collapsed regions retain their accessible name and focusable controls
- Tab strip uses Tabs primitive ARIA (roving tabindex, `role="tablist"`)

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches collapse toggle, tab strip, and body in order |
| Arrow keys | navigate within tab strip (Left/Right horizontal, Up/Down vertical) |
| `Enter`/`Space` on collapse toggle | toggles collapsed state |
| `Enter`/`Space` on tab (collapsed) | activates tab and expands region |
| `Delete` on focused tab | closes tab if closable |
| `Alt+Arrow` | reorder tabs within strip |
| `Escape` | dismisses tooltip in compact/vertical mode |

### Focus

- Collapsing a region returns focus to the collapse toggle
- Tooltips appear on focus in compact and vertical modes (300ms delay)

## 7. Layout

### Sizing

- Flexible expanded: `grid-template-rows: auto minmax(0, 1fr)` (strip + body)
- Flexible collapsed (side): `width: fit-content` (narrow icon strip)
- Flexible collapsed (top/bottom): `height: fit-content` (thin horizontal strip)
- Static: flex container, items `flex: 1 1 0`
- DockRegion sets `height: 100%` to fill parent containers
- Resize is handled externally by SplitView/SplitDivider

### Stack Direction (static mode)

- Left/right edge: panels stack in a `row` (horizontally)
- Top/bottom edge: panels stack in a `column` (vertically)

### Composition

- Parent: `WorkspaceShell`, `SplitView`, or any flex/grid layout
- Children: Tabs primitive (strip variant), CollapseToggle primitive
- Does NOT use PanelHeader or PanelSurface internally

## 8. Drag-and-Drop

### Within-Region Reorder

- **Flexible mode**: delegated to Tabs primitive's built-in drag reorder
- **Static mode**: native HTML drag-and-drop on stack items with drop position indicators

### Cross-Region Transfer

- Uses custom MIME type `application/x-pug-panel-drag` on `dataTransfer`
- DockRegion augments Tabs' `dragstart` with panel identity and source edge
- Drop validation via `canAcceptPanel` callback (checked on `drop`, not `dragover`)
- Visual feedback: dashed accent-colored border overlay during drag-over
- Drop zone overlay: absolute-positioned, `pointer-events: none`

### Drag Data Flow

1. Tab `dragstart` fires in source Tabs (sets `text/plain` for internal reorder)
2. DockRegion's strip `dragstart` handler bubbles, adds `application/x-pug-panel-drag`
3. Target DockRegion's root `dragover` checks for custom type, shows overlay
4. Target DockRegion's root `drop` reads panel data, validates, dispatches `panelDrop`

## 9. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Region | `border-subtle`, `background-panel`, `radius-surface` | dock chrome |
| Emphasis quiet | transparent border and background | reduced visual weight |
| Emphasis strong | `accent-base` mixed into border | increased visual weight |
| Strip separator | `border-subtle` | horizontal strip bottom border |
| Drop zone | `accent-base` at 10% opacity | drag-over indicator |
| Compact tooltip | `background-elevated`, `border-default`, `elevation-overlay` | hover label |

## 10. Svelte Notes

- Composes Tabs (`variant="strip"`) and CollapseToggle from `@pug/svelte-primitives`
- Does NOT use PanelTabs, PanelHeader, or PanelSurface
- Compact mode uses `ResizeObserver` with `scrollWidth > clientWidth` detection
- Passes `showTooltips={isCompact}` to Tabs for horizontal icon-only tooltip support
- `use:observeStrip` Svelte action binds ResizeObserver to the tabs container

## 11. Parity Checklist

### Tier 1: Strict Parity

- [x] region naming and `aria-label`
- [x] collapse/expand semantics with `collapsedChange` event
- [x] tab activation with `valueChange` event
- [x] click-to-expand from collapsed state
- [ ] focus restoration on collapse

### Tier 2: Visual Parity

- [x] strip density and collapsed posture use token roles
- [x] emphasis variants (standard, quiet, strong)
- [x] compact icon-only mode with tooltips
- [x] drag-over drop zone overlay

### Tier 3: Implementation Freedom

- [x] cross-region drag-and-drop via native HTML Drag and Drop API
- [x] compact mode detection via ResizeObserver
- [ ] static mode stack reorder animation

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Collapsed strip sizing differs by edge | side docks use vertical tabs, top/bottom keep horizontal | allowed | consistent behavior per edge type |
| No context menu event | not implemented in current iteration | deferred | add in future workspace milestone |

## 13. Approval And Adoption Notes

- contract status: `active contract`
- approvers: pending
- downstream adopters: workspace shells, IDE layouts, panel-based tools
- future follow-up: context menu support, animated collapse transitions,
  panel move validation rules
