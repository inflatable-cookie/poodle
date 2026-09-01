# DockRegion

Status: active contract
Updated: 2026-09-01

## 1. Purpose

- Component name: `DockRegion`
- Layer: `composite` (implemented in `@inflatable-cookie/poodle-svelte`)
- Summary: a collapsible dock area that hosts panel tabs and one active panel
  body, or stacks multiple fixed panels, within a workstation layout
- In scope: edge placement, static/flexible sizing modes, collapse/expand with
  icon-strip and hidden postures, active panel selection via Tabs primitive,
  cross-region panel drag-and-drop with validation, an optional public
  external-drag extension seam, auto-compact icon-only mode when tabs overflow,
  click-to-expand from collapsed state
- Out of scope: persistence backend, DAW-specific panel contents, resize
  handle (handled externally by SplitView)

## 2. Anatomy

### Flexible mode (expanded)

```text
[Root Region]
  ├── [Strip]
  │     ├── [Tabs (variant="strip", horizontal)]
  │     └── [CollapseToggle]  (only when collapsible=true)
  └── [Body] (active panel content)
```

### Flexible mode (collapsed icon-strip, left/right edge)

```text
[Root Region]
  └── [Strip (vertical)]
        ├── [CollapseToggle]  (only when collapsible=true)
        └── [Tabs (variant="strip", vertical, icon-only)]
```

### Flexible mode (collapsed icon-strip, top/bottom edge)

```text
[Root Region]
  └── [Strip (horizontal)]
        ├── [Tabs (variant="strip", horizontal, compact icon-only)]
        └── [CollapseToggle]  (only when collapsible=true)
```

### Flexible mode (collapsed hidden)

```text
[Root Region]
  └── [CollapseToggle]  (only when collapsible=true)
```

### Static mode

```text
[Root Region]
  └── [Stack]
        ├── [Stack Item] (drag source, reorderable)
        ├── [Stack Item]
        └── ...
```

| Part | Required | Description |
|------|----------|-------------|
| Root Region | yes | `<section>` dock container with `aria-label` |
| Strip | flexible only | tab/collapse chrome area |
| Tabs | flexible only | Tabs primitive (variant="strip") |
| CollapseToggle | conditional | collapse/expand affordance; only rendered when `collapsible=true` |
| Body | flexible expanded only | active panel content snippet |
| Stack | static only | flex container for stacked panels |
| Stack Item | static only | panel wrapper, and the panel's drag source |
| Drop Zone | conditional | overlay shown during cross-region drag |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `edge` | `DockEdge` | `"left"` | no | dock placement: `"left" \| "right" \| "top" \| "bottom"` |
| `sizing` | `DockSizing` | `"flexible"` | no | `"static"` for fixed stacked panels, `"flexible"` for tabbed/collapsible |
| `collapsible` | `boolean` | `false` | no | when true, renders the CollapseToggle in all flexible-mode postures; when false, no collapse affordance is shown |
| `showCollapseToggle` | `boolean` | `true` | no | set false when a divider-level control (e.g. SplitView pills) already owns collapse, so the strip does not duplicate it; collapse state rendering is unaffected |
| `collapsed` | `boolean` | `false` | no | collapse state (flexible mode only); meaningful only when `collapsible=true` |
| `collapsedPosture` | `DockCollapsedPosture` | `"icon-strip"` | no | `"hidden"` or `"icon-strip"` |
| `emphasis` | `DockEmphasis` | `"standard"` | no | `"standard" \| "quiet" \| "strong"` |
| `items` | `PanelTabItem[]` | `[]` | no | panel definitions with value, label, icon, closable |
| `value` | `string \| null` | `null` | no | controlled active panel (flexible mode) |
| `size` | `ControlSize \| null` | `null` | no | explicit semantic size override for Tabs |
| `sizeRole` | `SemanticControlSizeRole` | `"chrome"` | no | semantic role used to resolve inherited size scale (default `"chrome"` not `"control"`) |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for Tabs |
| `ariaLabel` | `string \| null` | `null` | no | region accessible label |
| `canAcceptPanel` | `(panelId: string, sourceEdge: DockEdge) => boolean \| null` | `null` | no | cross-region drop validation |
| `crossWindowDragSource` | `CrossWindowDragSourceBridge \| undefined` | `undefined` | no | host preparation for a panel that may leave this window; forwarded to the tab strip as its `crossWindowSourceBridge`. Only an opaque receipt leaves the window |
| `crossWindowDropTarget` | `CrossWindowDragTargetBridge \| undefined` | `undefined` | no | incoming host projection and commit for this window. Valid only when the region owns its controller — a region that joined an ambient `DragDropProvider` throws, because the provider is the window and the bridge belongs there |
| `dragZoneId` | `string \| null` | `null` | no | exact drop-zone identity for the same-zone drop guard; defaults to `edge`. Hosts mapping several regions onto one edge must pass a per-region id, or cross-region drops read as same-zone and are ignored |

### Tab Pass-throughs

DockRegion composes Tabs at three call sites — the expanded horizontal strip,
the collapsed horizontal icon-strip, and the collapsed vertical icon-strip.
Five Tabs props are forwarded through DockRegion under the `tab` prefix that
`tabVariant` established:

| DockRegion prop | Forwarded Tabs prop | Default | Notes |
|-----------------|---------------------|---------|-------|
| `tabActiveEdge` | `activeEdge` | `"underline"` | the dock's active-tab edge; `"none"` draws no edge, `"outline"` draws the accent border around the active item |
| `tabActiveFill` | `activeFill` | `"tint"` | the active tab's selection fill |
| `tabBordered` | `bordered` | `false` | strip border around the tab list |
| `tabFullWidth` | `fullWidth` | `false` | stretch tabs across the strip |
| `tabReorderable` | `reorderable` | `true` | drag-to-reorder within the strip |

Every default is the value the dock passed before the pass-through existed, so
rendering is unchanged unless a host opts in. All five apply at all three call
sites, including the collapsed icon-strips.

**What DockRegion deliberately does not forward.** Tabs' remaining props stay
unexposed on purpose, so the next report can tell a deliberate line from an
oversight:

- `activationMode`, `historyKey`, `actions` — behaviour and slots with no
  requested dock use, and each is a surface to support forever once added.
- `showTooltips` — DockRegion derives it from its own `isCompact` measurement;
  the derivation is load-bearing: icon-only tabs are unreadable without
  tooltips.
- `collapseWhenOverflow`, `overflowStrategy`, `shed`, `collapseLabel` — the
  dock's overflow story is the `isCompact` compaction (labels hidden, icon-only
  tabs), which is on by default. Tabs' `collapseWhenOverflow` would collapse
  the whole strip into a dropdown menu on the same overflow, putting two
  overflow mechanisms on one strip. It reads like an omission and is not one.

These props are deliberately not in the §Public Props table, matching the
treatment of `tabVariant` and `showTabs`: they are `svelteOnly` entries in
`packages/svelte/preview/scripts/contract-prop-drift.ts`'s `BASELINE`, which
keeps them off the table until the shared spec surface carries them. This is a
tranche, not a permanent carve-out — when `g13.014` gives `DockRegionSpec` its
tab fields, the whole entry moves into the table together and the baseline
line is deleted.

### PanelTabItem

```ts
type PanelTabItem = {
  value: string;
  label: string;
  icon?: string | null;
  closable?: boolean;
};
```

All items should have `icon` set when used in flexible docks, as collapsed and
compact modes render icon-only tabs. **This is advisory, not load-bearing.** A
dock whose items have no icon does not compact at all — see Compact Mode. It
used to obey the "should" literally and render a row of empty squares.

### Controlled And Uncontrolled

- Active panel (`value`) is typically controlled by the parent shell via `onValueChange`
- Collapse state (`collapsed`) is externally owned via `onCollapsedChange`

### Snippets

| Snippet | When Used | Payload |
|---------|-----------|---------|
| `panel` | static mode | `PanelTabItem` |
| `children` | flexible expanded mode | `PanelTabItem \| null` |

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| expanded | `sizing="flexible"`, `collapsed=false` | horizontal strip tabs + body visible |
| collapsed icon-strip (side) | `collapsed=true`, `collapsedPosture="icon-strip"`, left/right edge | vertical icon-only tabs + collapse toggle at top |
| collapsed icon-strip (top/bottom) | `collapsed=true`, `collapsedPosture="icon-strip"`, top/bottom edge | horizontal icon-only tabs + collapse toggle, body hidden |
| collapsed hidden | `collapsed=true`, `collapsedPosture="hidden"` | only collapse toggle visible |
| static | `sizing="static"` | stacked panels, no tabs or collapse |
| compact | auto-detected | horizontal tabs collapse to icon-only when the strip overflows **and every item has an icon** |
| drag-over | cross-region drag enters | dashed accent border overlay |
| emphasis quiet | `emphasis="quiet"` | transparent border and background |
| emphasis strong | `emphasis="strong"` | accent-tinted border |

### Compact Mode

When horizontal tabs overflow their container, DockRegion automatically hides labels
and close buttons, showing icon-only tabs with bottom-positioned tooltips on hover.
Uses `ResizeObserver` with overflow detection and hysteresis to prevent oscillation.

**Compaction requires that every item carries an icon.** Falling back to
icon-only is only a strategy when there is an icon to fall back to: hiding the
label of a tab that has none leaves an empty `2.25rem` square with no way to
tell the panels apart, and no tooltip target a pointer would think to visit.
When any item lacks an icon the strip stays full-width and overflows — cramped
beats unreadable. Measured on the icon-less specimen: without the rule, three
labelled panels rendered as three identical 36×28 squares.

### Click-to-Expand

Clicking a tab in any collapsed state runs both `onValueChange` (to activate
the panel) and `onCollapsedChange(false)` (to expand the region).

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | active panel changes | `string` | tab click or keyboard |
| `onCollapsedChange` | region collapses or expands | `boolean` | toggle click or tab click when collapsed |
| `onClose` | closable tab dismissed | `string` | forwarded from Tabs |
| `onReorder` | tabs or stack items reordered | `string[]` | within-region reorder |
| `onPanelDrop` | panel dropped from another region | `{ panel: PanelDragData; targetEdge: DockEdge; index: number }` | cross-region transfer; `index` is the insert slot in the destination `items`. A drop on a tab or stack item lands at that item. A drop on the region body appends. |

`crossWindowDragSource` and `crossWindowDropTarget` are the host bridge, not
Poodle callbacks masquerading as app policy. Their field and method contract is
spec 069's Cross-Window Host Bridge section, which both web targets and shared
Rust implement identically. Poodle owns only the ordering: preparation runs on
the accepted pre-drag gesture and before activation, a source cannot advertise
or start a cross-window gesture until its own receipt is armed, the host's
terminal subscription is the sole authority on the outcome, and a projected
target is revalidated against this region's own `canAcceptPanel` before commit.

A host receipt commits through the target bridge and does **not** also call
`onPanelDrop`, matching the split the old external-target seam had.

### PanelDragData

```ts
type PanelDragData = {
  panelId: string;
  sourceEdge: DockEdge;
  /** The source region's `dragZoneId`, falling back to its edge. */
  sourceZone: string;
};
```

`sourceZone` is required. It was optional so a payload written by an older
build still parsed, and a receiver missing it fell back to `sourceEdge` — which
silently read two regions on one edge as the same zone. There is no wire to be
compatible with any more: the subject is minted and read by one controller, so
the fallback is gone rather than left as a quiet mis-resolution.

### Cross-Window Types

The bridge types are shared, not DockRegion's own: `CrossWindowDragSourceBridge`,
`CrossWindowDragTargetBridge`, `CrossWindowDragReceipt`,
`CrossWindowDragProjection`, and `CrossWindowDragCapabilities` are exported from
`@inflatable-cookie/poodle-core` and mirrored in
`poodle_headless::cross_window_drag`. Spec 069 is their single definition; this
contract does not restate them, because a second copy would be a second thing
to keep in step.

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

Panel movement runs on the shared drag substrate (architecture 011, spec 069).
There is no dock-specific DOM plumbing left: no `application/x-poodle-panel-drag`
MIME wire, and no `dockPanelDragSession` module global.

### Controller Scope

A DockRegion joins the nearest `DragDropProvider` when one is present, and
otherwise creates a private controller for its own reorder.

That scope is the whole cross-region rule. Two sibling regions resolve each
other's targets **only** when one controller holds both registrations, so a
consumer that wants cross-region transfer wraps its regions in one provider.
Two independently self-provided regions keep their own reorder and do not
discover each other. Nothing restores that link implicitly — the old global
session is deleted, and no MIME type, module singleton, or document registry
replaces it.

`crossWindowDropTarget` is therefore valid only on a region that owns its
controller. A region that joined a provider throws if given one: the provider
is the window, and a window bridge belongs there.

### Within-Region Reorder

- **Flexible mode**: delegated to the Tabs primitive's reorder, which runs on
  the same substrate. Pointer and Alt+Arrow reach one session.
- **Static mode**: each stack item is a drag source and a drop target on this
  region's controller. A drop whose subject came from this region's own zone is
  a reorder and reports `onReorder`.

### Cross-Region Transfer

- A stack item or tab dragged into another region on the same controller
  reports `onPanelDrop` with the complete `PanelDragData`, the destination
  edge, and the insert `index`.
- A drop on a destination tab or stack item lands *at* that item: the hovered
  half chooses before/after, matching same-strip land-at. A drop on the region
  body (no tab under the pointer) appends (`index === items.length`).
- Eligibility is `canAcceptPanel`, run during hover **and** again at commit,
  including when the pointer is over a destination tab. The same rule covers
  the region body and static stack items. The substrate carries the subject in
  the session, so the panel's identity is known at hover without a side
  channel — which is the entire reason the old global existed.
- A drop back onto the source zone is ineligible in flexible sizing, because
  same-strip reorder owns it. Zone identity is `dragZoneId` when set, else the
  edge.
- A tab or stack item beats the region it sits in: nested arbitration prefers
  the deepest target, and the region registers at a lower priority.
- Visual feedback: the hovered tab or stack item posts `data-drop-target`. The
  dashed region overlay still paints when the region itself holds the intent
  (body / empty strip). Drop-zone overlay is absolute-positioned and
  `pointer-events: none`.

### Cross-Window Transfer

A panel that leaves the window goes through the host bridge, never through a
local wire:

| Order | Poodle action | Host observation |
|------:|---------------|------------------|
| 1 | accepted pre-drag gesture on an enabled panel | `prepare(request, signal)` starts, before activation |
| 2 | a later gesture supersedes an unfinished preparation | the signal aborts; a late receipt is handed straight back through `cancel` |
| 3 | the receipt arms | only now may the source advertise or start a native cross-window gesture |
| 4 | the gesture becomes live | `start(receipt, transport, onTerminal)` installs the one authoritative terminal subscription |
| 5 | the receiving window projects | `subscribe` publishes a projection; this region re-runs `canAcceptPanel` against it |
| 6 | the drop lands | the projected target is revalidated, then `commit(request, signal)` runs — `onPanelDrop` does not |
| 7 | the host answers | that result ends the session. A native drag end, a pointer release, and `dropEffect` never manufacture a committed result |

A decline or failure at step 1 cancels only the transfer. The gesture continues
as an ordinary local drag.

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
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent)` |

#### `.dock-region[data-emphasis="quiet"]`

| Property | Value |
|----------|-------|
| `border-color` | `transparent` |
| `background` | `transparent` |

#### `.dock-region[data-emphasis="strong"]`

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--poodle-color-accent-base) 32%, var(--poodle-color-border-subtle))` |

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
| `box-shadow` | `inset 0 0 0 0.125rem var(--poodle-color-accent-base)` |
| `border-radius` | `var(--poodle-radius-control)` |

#### `.dock-region[data-sizing="flexible"]:not([data-collapsed])` (Expanded)

| Property | Value |
|----------|-------|
| `grid-template-rows` | `auto minmax(0, 1fr)` |

#### Expanded Edge-Aware Borders

When flexible and expanded, each edge gets a single border on its inner side:

| Edge | Border |
|------|--------|
| `left` | `border-right: 0.0625rem solid var(--poodle-color-border-subtle)` |
| `right` | `border-left: 0.0625rem solid var(--poodle-color-border-subtle)` |
| `top` | `border-bottom: 0.0625rem solid var(--poodle-color-border-subtle)` |
| `bottom` | `border-top: 0.0625rem solid var(--poodle-color-border-subtle)` |

#### `.dock-region__strip[data-orientation="horizontal"]`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `padding-right` | `0.5rem`; `0` when the root is `data-density="compact"` |
| `min-height` | `2.75rem`; `0` (hugs the tabs) when the root is `data-density="compact"` |
| `border-bottom` | `0.0625rem solid var(--poodle-color-border-subtle)` |

#### `.dock-region__tabs`

| Property | Value |
|----------|-------|
| `flex` | `1 1 0` |
| `min-width` | `0` |

#### `.dock-region__strip[data-orientation="horizontal"] :global(.poodle-tabs[data-variant="strip"] .poodle-tabs__list)`

| Property | Value |
|----------|-------|
| `border-bottom` | `0` |

#### `.dock-region__strip[data-compact] :global(.poodle-tabs__label)`, `.dock-region__strip[data-compact] :global(.poodle-tabs__close)`

| Property | Value |
|----------|-------|
| `display` | `none` |

#### `.dock-region__strip[data-compact] :global(.poodle-tabs__tab)`

| Property | Value |
|----------|-------|
| `padding` | `0 0.5rem` |
| `justify-content` | `center` |

#### `.dock-region__strip[data-compact] :global(.poodle-tabs__list)`

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

#### Collapsed Icon-Strip Edge-Aware Borders

When collapsed in icon-strip posture, top/bottom edges get a border on their inner side:

| Edge | Border |
|------|--------|
| `top` | `border-bottom: 0.0625rem solid var(--poodle-color-border-subtle)` |
| `bottom` | `border-top: 0.0625rem solid var(--poodle-color-border-subtle)` |

Left/right edges use the vertical strip's own `border-right` (or `border-left` for right edge) instead of a root-level border.

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
| `border-right` | `0.0625rem solid var(--poodle-color-border-subtle)` |

#### `.dock-region__strip[data-orientation="vertical"] :global(.poodle-tabs)`

| Property | Value |
|----------|-------|
| `flex` | `1 1 0` |
| `min-height` | `0` |
| `grid-template-columns` | `1fr !important` |

#### `.dock-region__strip[data-orientation="vertical"] :global(.poodle-tabs__list)`

| Property | Value |
|----------|-------|
| `border-right` | `0 !important` |

#### `.dock-region__strip[data-orientation="vertical"] :global(.poodle-tabs__item)`

| Property | Value |
|----------|-------|
| `margin-right` | `-0.0625rem !important` |

#### `.dock-region__strip[data-orientation="vertical"] > :global(.collapse-toggle)`

| Property | Value |
|----------|-------|
| `align-self` | `center` |
| `padding` | `var(--poodle-space-panel-y, 0.5rem) 0` |

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
| `padding` | `var(--poodle-space-panel-y, 0.5rem)` |

#### `.dock-region__drop-zone`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `inset` | `0` |
| `z-index` | `10` |
| `border` | `0.125rem dashed var(--poodle-color-accent-base)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent)` |
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

- Composes Tabs (`variant="strip"`) and CollapseToggle from `@inflatable-cookie/poodle-svelte`
- Does NOT use PanelTabs, PanelHeader, or PanelSurface
- CollapseToggle is only rendered when `collapsible=true`; in all four flexible
  postures (expanded, collapsed icon-strip left/right, collapsed icon-strip
  top/bottom, collapsed hidden) the toggle is conditionally gated
- `size`, `sizeRole` (default `"chrome"`), and `density` props are passed through
  to Tabs instances
- Compact mode uses `ResizeObserver` with `scrollWidth > clientWidth` detection
- Passes `showTooltips={isCompact}` to Tabs for horizontal icon-only tooltip support
- `use:observeStrip` Svelte action binds ResizeObserver to the tabs container
- DockRegion passes `poodle.dock-panel` to Tabs as its `dragSubjectKind` and
  maps each panel value onto an encoded subject id carrying panel id, source
  edge, and source zone. That encoding is substrate identity only: `value`,
  `onValueChange`, `onClose`, `onReorder`, and `onPanelDrop` all speak the
  consumer's own panel values
- `crossWindowDragSource` reaches the strip as its `crossWindowSourceBridge`;
  no selector, generated id, DOM event, MIME type, or host type is part of the
  contract

## 10a. Jetstream Notes

- `DockRegion::from_spec(spec, theme).content(...).on_tab_change(...).on_collapse_toggle(...)`.
- Panel drag is renderer-neutral: each tab registers a `poodle.dock-panel`
  drag source and the region registers the matching drop target, so a
  Jetstream-shaped runtime inherits the same construction. Whether a given
  runtime *delivers* the gesture is that runtime's own capability report.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [x] region naming and `aria-label`
- [x] collapse/expand semantics with `collapsedChange` event
- [x] tab activation with `valueChange` event
- [x] click-to-expand from collapsed state
- [ ] focus restoration on collapse
- [ ] `collapsible` prop gates CollapseToggle rendering (default false)
- [ ] edge-aware borders on expanded and collapsed icon-strip states

### Tier 2: Visual Parity

- [x] strip density and collapsed posture use token roles
- [x] emphasis variants (standard, quiet, strong)
- [x] compact icon-only mode with tooltips
- [x] drag-over drop zone overlay

### Tier 3: Implementation Freedom

- [x] cross-region panel movement through the shared drag substrate
- [x] asynchronous external preparation precedes the native payload-write window
- [x] unready external preparation cannot advertise an external payload
- [x] external eligibility drives the existing drop affordance
- [x] same-region reorder remains Poodle-owned when the external seam is enabled
- [x] compact mode detection via ResizeObserver
- [ ] static mode stack reorder animation

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Collapsed strip sizing differs by edge | side docks use vertical tabs, top/bottom keep horizontal | allowed | consistent behavior per edge type |
| `crossWindowDragSource` and `crossWindowDropTarget` are props on web and handler/controller seams on native | a host bridge is capability, not renderer-neutral data: leases, window geometry, and commit authority live outside Poodle. Native carries the same split — `DockRegionHandlers.cross_window_drag_source` per region, `DragDropController::set_cross_window_target_bridge` per window | accepted | do not copy either bridge into `DockRegionSpec` |
| A web DockRegion joins the nearest `DragDropProvider`; a GPUI one joins the window's `DragDropController` | the web has no ambient controller unless a consumer installs one, so a lone region self-provides. GPUI hosts already own one controller per provider | accepted | two sibling regions cross-drop only under one shared controller on either runtime |
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
| Flexible dock -- expanded (left edge) | `edge="left"`, `sizing="flexible"`, 3 items with icons (Explorer, Search, Source Control), `collapsed=false`, controlled `value` | Left dock with horizontal tab strip showing icon+label tabs, active panel body below with content; tabs closable and switchable |

### Flexible Dock -- Collapsed Icon-Strip (Left Edge)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Flexible dock -- collapsed icon-strip (left edge) | `edge="left"`, `sizing="flexible"`, 3 items with icons, `collapsed=true`, `collapsedPosture="icon-strip"` | Narrow vertical icon-only strip with collapse toggle; no panel body visible; tooltips on hover |

### Interactive Collapse Toggle

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Interactive collapse toggle | `edge="left"`, `sizing="flexible"`, 3 items with icons (Files, Outline, Debug), `collapsed` togglable, `collapsedPosture="icon-strip"`, collapse/expand events bound | Left dock with toggle between expanded (tab strip + body) and collapsed (icon-strip) states; main content area alongside |

### Bottom Edge Dock

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Bottom edge dock | `edge="bottom"`, `sizing="flexible"`, 3 items with icons (Terminal, Output, Problems), `collapsed` togglable, `collapsedPosture="icon-strip"` | Bottom dock with horizontal tabs, panel body expands upward; editor area above; collapses to thin horizontal strip |

### Cross-Region Drag-And-Drop

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Cross-region drag-and-drop | Two side-by-side DockRegions (left with 3 items: Explorer/Search/Source Control; right with 1 item: Outline), both `edge` appropriate, `canAcceptPanel` enabled, reorder and panelDrop events bound | Two docks in grid layout; tabs move between regions; drop zone overlay appears while a panel is over a region; panel counts update after transfer |

## 14. Approval And Adoption Notes

- contract status: `active contract`
- approvers: pending
- downstream adopters: workspace shells, IDE layouts, panel-based tools
- future follow-up: context menu support, animated collapse transitions,
  panel move validation rules
