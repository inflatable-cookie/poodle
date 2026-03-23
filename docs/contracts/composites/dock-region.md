# DockRegion

Status: active contract
Updated: 2026-03-18

## 1. Purpose

- Component name: `DockRegion`
- Layer: `composite` (implemented in `@flint/svelte-workstation`)
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
- Resize is handled externally by SplitView

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

- Uses custom MIME type `application/x-flint-panel-drag` on `dataTransfer`
- DockRegion augments Tabs' `dragstart` with panel identity and source edge
- Drop validation via `canAcceptPanel` callback (checked on `drop`, not `dragover`)
- Visual feedback: dashed accent-colored border overlay during drag-over
- Drop zone overlay: absolute-positioned, `pointer-events: none`

### Drag Data Flow

1. Tab `dragstart` fires in source Tabs (sets `text/plain` for internal reorder)
2. DockRegion's strip `dragstart` handler bubbles, adds `application/x-flint-panel-drag`
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

### Token Usage — Exact CSS Values

#### `.dock-region` (Root)

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `grid` |
| `min-width` | `0` |
| `min-height` | `0` |
| `height` | `100%` |
| `border` | `0` |
| `background` | `color-mix(in srgb, var(--flint-color-background-panel) 94%, transparent)` |

#### `.dock-region[data-emphasis="quiet"]`

| Property | Value |
|----------|-------|
| `border-color` | `transparent` |
| `background` | `transparent` |

#### `.dock-region[data-emphasis="strong"]`

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--flint-color-accent-base) 32%, var(--flint-color-border-subtle))` |

#### `.dock-region[data-sizing="static"]`

| Property | Value |
|----------|-------|
| `grid-template-rows` | `1fr` |

#### `.dock-region__stack`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `min-width` | `0` |
| `min-height` | `0` |

#### `.dock-region__stack[data-direction="column"]`

| Property | Value |
|----------|-------|
| `flex-direction` | `column` |

#### `.dock-region__stack[data-direction="row"]`

| Property | Value |
|----------|-------|
| `flex-direction` | `row` |

#### `.dock-region__stack-item`

| Property | Value |
|----------|-------|
| `flex` | `1 1 0` |
| `min-width` | `0` |
| `min-height` | `0` |
| `cursor` | `grab` |

#### `.dock-region__stack-item[data-drag-source]`

| Property | Value |
|----------|-------|
| `opacity` | `0.4` |

#### `.dock-region__stack-item[data-drop-target]`

| Property | Value |
|----------|-------|
| `box-shadow` | `inset 0 0 0 0.125rem var(--flint-color-accent-base)` |
| `border-radius` | `var(--flint-radius-control)` |

#### `.dock-region[data-sizing="flexible"]:not([data-collapsed])` (Expanded)

| Property | Value |
|----------|-------|
| `grid-template-rows` | `auto minmax(0, 1fr)` |

#### `.dock-region__strip[data-orientation="horizontal"]`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--flint-space-inline-sm)` |
| `padding-right` | `0.5rem` |
| `border-bottom` | `0.0625rem solid var(--flint-color-border-subtle)` |

#### `.dock-region__tabs`

| Property | Value |
|----------|-------|
| `flex` | `1 1 0` |
| `min-width` | `0` |

#### `.dock-region__strip[data-orientation="horizontal"] :global(.flint-tabs[data-variant="strip"] .flint-tabs__list)`

| Property | Value |
|----------|-------|
| `border-bottom` | `0` |

#### `.dock-region__strip[data-compact] :global(.flint-tabs__label)`, `.dock-region__strip[data-compact] :global(.flint-tabs__close)`

| Property | Value |
|----------|-------|
| `display` | `none` |

#### `.dock-region__strip[data-compact] :global(.flint-tabs__tab)`

| Property | Value |
|----------|-------|
| `padding` | `0 0.5rem` |
| `justify-content` | `center` |

#### `.dock-region__strip[data-compact] :global(.flint-tabs__list)`

| Property | Value |
|----------|-------|
| `overflow` | `visible` |

#### `.dock-region__body`

| Property | Value |
|----------|-------|
| `min-height` | `0` |
| `overflow` | `auto` |

#### `.dock-region[data-collapsed][data-collapsed-posture="icon-strip"]`

| Property | Value |
|----------|-------|
| `grid-template-rows` | `1fr` |

#### `.dock-region[data-collapsed][data-collapsed-posture="icon-strip"][data-edge="left"]`, `[data-edge="right"]`

| Property | Value |
|----------|-------|
| `width` | `fit-content` |

#### `.dock-region[data-collapsed][data-collapsed-posture="icon-strip"][data-edge="top"]`, `[data-edge="bottom"]`

| Property | Value |
|----------|-------|
| `height` | `fit-content` |

#### `.dock-region__strip[data-orientation="vertical"]`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `align-items` | `stretch` |
| `gap` | `0` |
| `padding` | `0` |
| `border-right` | `0.0625rem solid var(--flint-color-border-subtle)` |

#### `.dock-region__strip[data-orientation="vertical"] :global(.flint-tabs)`

| Property | Value |
|----------|-------|
| `flex` | `1 1 0` |
| `min-height` | `0` |
| `grid-template-columns` | `1fr !important` |

#### `.dock-region__strip[data-orientation="vertical"] :global(.flint-tabs__list)`

| Property | Value |
|----------|-------|
| `border-right` | `0 !important` |

#### `.dock-region__strip[data-orientation="vertical"] :global(.flint-tabs__item)`

| Property | Value |
|----------|-------|
| `margin-right` | `-0.0625rem !important` |

#### `.dock-region__strip[data-orientation="vertical"] > :global(.collapse-toggle)`

| Property | Value |
|----------|-------|
| `align-self` | `center` |
| `padding` | `var(--flint-space-panel-y, 0.5rem) 0` |

#### `.dock-region[data-collapsed][data-collapsed-posture="hidden"]`

| Property | Value |
|----------|-------|
| `border-color` | `transparent` |
| `background` | `transparent` |

#### `.dock-region__edge-toggle`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `padding` | `var(--flint-space-panel-y, 0.5rem)` |

#### `.dock-region__drop-zone`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `inset` | `0` |
| `z-index` | `10` |
| `border` | `0.125rem dashed var(--flint-color-accent-base)` |
| `border-radius` | `var(--flint-radius-surface)` |
| `background` | `color-mix(in srgb, var(--flint-color-accent-base) 10%, transparent)` |
| `pointer-events` | `none` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-edge` | `.dock-region` root | dock placement: `left`, `right`, `top`, `bottom` |
| `data-sizing` | `.dock-region` root | sizing mode: `static`, `flexible` |
| `data-emphasis` | `.dock-region` root | emphasis variant: `standard`, `quiet`, `strong` |
| `data-collapsed` | `.dock-region` root | present when dock is collapsed |
| `data-collapsed-posture` | `.dock-region` root | collapsed display mode: `icon-strip`, `hidden` |
| `data-direction` | `.dock-region__stack` | stack flow direction: `column`, `row` |
| `data-orientation` | `.dock-region__strip` | strip orientation: `horizontal`, `vertical` |
| `data-compact` | `.dock-region__strip` | present when tabs auto-collapse to icon-only |
| `data-drag-source` | `.dock-region__stack-item` | marks item being dragged |
| `data-drop-target` | `.dock-region__stack-item` | marks item as drop target |

## 10. Svelte Notes

- Composes Tabs (`variant="strip"`) and CollapseToggle from `@flint/svelte-primitives`
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

## 13. Specimen Definitions

### Static Dock -- Horizontal (Top Edge)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Static dock -- horizontal (top edge) | `edge="top"`, `sizing="static"`, 3 items (Meter Strip, Transport, Mixer), reorderable | Horizontal stack of fixed panels along top edge, each panel labeled, separated by subtle borders; panels reorderable via drag |

### Static Dock -- Vertical (Left Edge)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Static dock -- vertical (left edge) | `edge="left"`, `sizing="static"`, 2 items (Toolbar, Inspector), reorderable | Vertical left-edge dock with panels stacked horizontally (row direction), each panel labeled and reorderable |

### Flexible Dock -- Expanded (Left Edge)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Flexible dock -- expanded (left edge) | `edge="left"`, `sizing="flexible"`, 3 items with icons (Explorer, Search, Source Control), `isCollapsed=false`, controlled `value` | Left dock with horizontal tab strip showing icon+label tabs, active panel body below with content; tabs closable and switchable |

### Flexible Dock -- Collapsed Icon-Strip (Left Edge)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Flexible dock -- collapsed icon-strip (left edge) | `edge="left"`, `sizing="flexible"`, 3 items with icons, `isCollapsed=true`, `collapsedPosture="icon-strip"` | Narrow vertical icon-only strip with collapse toggle; no panel body visible; tooltips on hover |

### Interactive Collapse Toggle

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Interactive collapse toggle | `edge="left"`, `sizing="flexible"`, 3 items with icons (Files, Outline, Debug), `isCollapsed` togglable, `collapsedPosture="icon-strip"`, collapse/expand events bound | Left dock with toggle between expanded (tab strip + body) and collapsed (icon-strip) states; main content area alongside |

### Bottom Edge Dock

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Bottom edge dock | `edge="bottom"`, `sizing="flexible"`, 3 items with icons (Terminal, Output, Problems), `isCollapsed` togglable, `collapsedPosture="icon-strip"` | Bottom dock with horizontal tabs, panel body expands upward; editor area above; collapses to thin horizontal strip |

### Cross-Region Drag-And-Drop

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Cross-region drag-and-drop | Two side-by-side DockRegions (left with 3 items: Explorer/Search/Source Control; right with 1 item: Outline), both `edge` appropriate, `canAcceptPanel` enabled, reorder and panelDrop events bound | Two docks in grid layout; tabs draggable between regions; drop zone overlay appears on drag-over; panel counts update after transfer |

## 14. Approval And Adoption Notes

- contract status: `active contract`
- approvers: pending
- downstream adopters: workspace shells, IDE layouts, panel-based tools
- future follow-up: context menu support, animated collapse transitions,
  panel move validation rules
