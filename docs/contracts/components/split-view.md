# SplitView

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `SplitView`
- Layer: `composites`
- Summary: a resizable layout container that divides space between two pane
  regions with an interactive divider, optional collapse toggles, drag-to-collapse
  behavior, fixed-size pane support, and keyboard-resizable separators
- In scope: orientation (horizontal/vertical), divider semantics, ratio-based
  and fixed-size pane allocation, collapsible panes with toggle buttons,
  drag-to-collapse thresholds, keyboard-resizable separators via ResizeHandle,
  min-size constraints, size and density support
- Out of scope: nested dock orchestration policy, persistence backend,
  app-specific pane content

## 2. Anatomy

```text
[Root .split-view]  <div aria-label>
  ├── [PrimaryPane .split-view__pane--primary]  <div>
  │     └── (slot: primary)
  ├── [Divider .split-view__divider]  <div>
  │     ├── [ResizeHandle]  ResizeHandle primitive
  │     └── [Toggles .split-view__toggles]  <div> (optional)
  │           ├── [CollapseToggle: primary]  CollapseToggle primitive (optional)
  │           └── [CollapseToggle: secondary]  CollapseToggle primitive (optional)
  └── [SecondaryPane .split-view__pane--secondary]  <div>
        └── (slot: secondary)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | flex container for pane layout | layout only |
| PrimaryPane | yes | first pane region | min sizes, overflow |
| Divider | yes | resize handle container and visual separator | layout only |
| ResizeHandle | yes | draggable/keyboard-resizable separator | delegates to ResizeHandle primitive |
| Toggles | no | overlay container for collapse toggle buttons | layout only |
| CollapseToggle | no | button to collapse/expand a pane | delegates to CollapseToggle primitive |
| SecondaryPane | yes | second pane region | min sizes, overflow |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | split axis |
| `ratio` | `number` | `0.5` | no | controlled primary split ratio (clamped to 0.05–0.95) |
| `defaultRatio` | `number` | `0.5` | no | uncontrolled initial ratio |
| `minPrimarySize` | `number \| null` | `null` | no | minimum primary pane size in px |
| `minSecondarySize` | `number \| null` | `null` | no | minimum secondary pane size in px |
| `primarySize` | `number \| null` | `null` | no | fixed primary pane size in px; when set, primary uses fixed flex and secondary fills remaining space |
| `secondarySize` | `number \| null` | `null` | no | fixed secondary pane size in px; when set, secondary uses fixed flex and primary fills remaining space |
| `primaryCollapsed` | `boolean` | `false` | no | collapse state for primary pane |
| `secondaryCollapsed` | `boolean` | `false` | no | collapse state for secondary pane |
| `showCollapsePrimary` | `boolean` | `false` | no | show collapse toggle for primary pane |
| `showCollapseSecondary` | `boolean` | `false` | no | show collapse toggle for secondary pane |
| `ariaLabel` | `string \| null` | `null` | no | accessible name (defaults to "Split view") |
| `disabled` | `boolean` | `false` | no | disables resize and collapse interactions |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl" \| null` | `null` | no | explicit control size override |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Slots

| Slot | Purpose |
|------|---------|
| `primary` | content for the primary (first) pane |
| `secondary` | content for the secondary (second) pane |

### Controlled And Uncontrolled

- controlled: `ratio` plus `ratioChange` event
- uncontrolled: `defaultRatio` (internal state tracks ratio)
- fixed-size: `primarySize` or `secondarySize` override ratio-based allocation
- collapse states (`primaryCollapsed`, `secondaryCollapsed`) are externally
  owned; changes dispatched via events

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| steady | default | both panes visible with divider between |
| resizing | pointer or keyboard resize active | divider focus/emphasis visible |
| primary-collapsed | `primaryCollapsed=true` | primary pane hidden (`flex: 0 0 0`), secondary fills space |
| secondary-collapsed | `secondaryCollapsed=true` | secondary pane hidden (`flex: 0 0 0`), primary fills space |
| fixed-primary | `primarySize` is set | primary pane uses fixed pixel flex, secondary fills remaining space |
| fixed-secondary | `secondarySize` is set | secondary pane uses fixed pixel flex, primary fills remaining space |
| disabled | `disabled=true` | resize handle and collapse toggles non-interactive |

### Component States

Internal state: `uncontrolledRatio`, `dragMousePos` for resize tracking.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `ratioChange` | resize commits or streams | `{ ratio: number }` | host decides persistence cadence |
| `primaryCollapsedChange` | primary pane collapse state changes | `{ isCollapsed: boolean }` | fires on toggle click or drag-to-collapse |
| `secondaryCollapsedChange` | secondary pane collapse state changes | `{ isCollapsed: boolean }` | fires on toggle click or drag-to-collapse |

## 6. Accessibility

### Semantics

- Root: `<div>` with `aria-label` (defaults to "Split view")
- ResizeHandle: separator semantics with orientation and aria-label="Resize",
  delegated to ResizeHandle primitive
- CollapseToggle buttons: dynamic `aria-label` describing action
  ("Collapse primary" / "Expand primary", "Collapse secondary" / "Expand secondary")

### Keyboard

| Key | Behavior |
|-----|----------|
| arrow keys on divider | adjusts ratio via `resizeStep` events |
| `Home` / `End` | optional jump to min/max positions (delegated to ResizeHandle) |
| `Enter` / `Space` | on collapse toggle: toggles pane collapse |
| `Tab` | reaches divider, toggles, and pane content in logical order |

### Focus And Announcement

- focus entry: ResizeHandle becomes focusable when keyboard resizing is
  supported
- focus exit: divider focus clears while pane sizing remains updated
- live-region behavior: none; resize and collapse state conveyed through
  control semantics
- GPUI-native accessibility mapping notes: GPUI must expose resizable
  separators with orientation and value semantics, not just pointer-only
  drag handles

## 7. Layout

### Sizing

- root fills assigned parent space (100% width and height)
- flex direction: row for horizontal, column for vertical
- primary pane flex: `0 0 {ratio*100}%` (ratio-based), `0 0 {primarySize}px`
  (fixed), `0 0 0` (collapsed), `1 1 0` (opposite collapsed or opposite fixed)
- secondary pane flex: `1 1 0` (default), `0 0 {secondarySize}px` (fixed),
  `0 0 0` (collapsed)
- divider width: 0.5rem (horizontal) or height: 0.5rem (vertical)
- min-size constraints applied via inline style when not collapsed
- panes have `overflow: hidden`

### Composition

- composes: `ResizeHandle`, `CollapseToggle` primitives
- parent expectations: workspace shells, panel layouts, utility views
- child expectations: any content via primary/secondary slots
- resizing rules: child focus continuity should survive ratio changes and
  collapse/restore operations

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-orientation` | root `<div>`, divider `<div>` | `"horizontal"`, `"vertical"` |
| `data-primary-collapsed` | root `<div>` | present when true |
| `data-secondary-collapsed` | root `<div>` | present when true |
| `data-disabled` | divider `<div>` | present when true |
| `data-has-toggles` | divider `<div>` | present when true |
| `data-size` | root `<div>` | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` |
| `data-density` | root `<div>` | `"compact"`, `"default"`, `"comfortable"` |

### Root (`.split-view`)

| Property | Value |
|----------|-------|
| display | `flex` |
| min-height | `0` |
| min-width | `0` |
| height | `100%` |
| width | `100%` |

#### Root Vertical (`[data-orientation="vertical"]`)

| Property | Value |
|----------|-------|
| flex-direction | `column` |

### Pane (`.split-view__pane`)

| Property | Value |
|----------|-------|
| min-width | `0` |
| min-height | `0` |

Pane `flex` and `overflow` are applied via inline style:
- `overflow: hidden` always
- `flex` computed from ratio, fixed size, and collapse state (see Layout section)
- `min-width` or `min-height` applied inline when `minPrimarySize`/`minSecondarySize`
  is set and pane is not collapsed

### Pane Flex Computation

| Condition | Primary Flex | Secondary Flex |
|-----------|-------------|---------------|
| default (ratio-based) | `0 0 {ratio*100}%` | `1 1 0` |
| primaryCollapsed | `0 0 0` | `1 1 0` |
| secondaryCollapsed | `1 1 0` | `0 0 0` |
| primarySize set | `0 0 {primarySize}px` | `1 1 0` |
| secondarySize set | `1 1 0` | `0 0 {secondarySize}px` |

### Divider (`.split-view__divider`)

| Property | Value |
|----------|-------|
| position | `relative` |
| display | `flex` |
| align-items | `center` |
| justify-content | `center` |
| flex-shrink | `0` |

#### Divider Horizontal (`[data-orientation="horizontal"]`)

| Property | Value |
|----------|-------|
| width | `0.5rem` |
| height | `100%` |

#### Divider Vertical (`[data-orientation="vertical"]`)

| Property | Value |
|----------|-------|
| height | `0.5rem` |
| width | `100%` |

### Toggles (`.split-view__toggles`)

| Property | Value |
|----------|-------|
| position | `absolute` |
| z-index | `1` |
| display | `flex` |
| align-items | `center` |
| gap | `0.25rem` |
| pointer-events | `none` |
| children `*` pointer-events | `auto` |

#### Toggles Horizontal

| Property | Value |
|----------|-------|
| flex-direction | `column` |
| top | `50%` |
| left | `50%` |
| transform | `translate(-50%, -50%)` |

#### Toggles Vertical

| Property | Value |
|----------|-------|
| flex-direction | `row` |
| top | `50%` |
| left | `50%` |
| transform | `translate(-50%, -50%)` |

### Composed Primitives

Token usage for `ResizeHandle` and `CollapseToggle` is defined in their
respective primitive contracts.

### Drag-To-Collapse Behavior

| Threshold | Action |
|-----------|--------|
| ratio < 0.02 during drag | collapses primary pane, resets ratio to 0.5 |
| ratio > 0.98 during drag | collapses secondary pane, resets ratio to 0.5 |
| drag starts while primary collapsed | uncollapses primary, sets ratio to 0.05 |
| drag starts while secondary collapsed | uncollapses secondary, sets ratio to 0.95 |
| ratio clamping | always clamped to [0.05, 0.95] range |

### Collapse Toggle Visibility Rules

| Toggle | Shown When |
|--------|-----------|
| primary collapse | `showCollapsePrimary=true` AND secondary is not collapsed |
| secondary collapse | `showCollapseSecondary=true` AND primary is not collapsed |

### Toggle Direction By Orientation

| Orientation | Primary Toggle Direction | Secondary Toggle Direction |
|-------------|------------------------|--------------------------|
| horizontal | `left` | `right` |
| vertical | `up` | `down` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- `data-size` attribute on root reflects resolved size via `resolveSemanticControlSize`
- `data-density` attribute on root reflects resolved density
- uses `createEventDispatcher` for all events
- `bind:this={container}` on root for computing raw ratio from mouse position
- `rawRatio()` converts mouse position to ratio using container bounding rect
- primary/secondary collapse toggles use `CollapseToggle` from `@poodle/svelte`
- resize events handled via `ResizeHandle` `resizeStart`/`resizeMove`/`resizeStep` events
- pane content conditionally rendered: `{#if !primaryCollapsed}` / `{#if !secondaryCollapsed}`
- `SplitOrientation`, `CollapseDirection`, `ControlSize`, `SemanticControlSizeRole`,
  `ControlDensity` types imported from `@poodle/svelte`
- `ResizeHandle` and `CollapseToggle` imported from `@poodle/svelte`
- `data-primary-collapsed` and `data-secondary-collapsed` use `|| undefined` to
  omit the attribute when false

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::split_view`
- spec struct: `SplitViewSpec` with orientation, ratio, collapse states,
  fixed sizes, min sizes, disabled, size, density
- GPUI may use native splitter support or custom layout code, but keyboard
  resizing, orientation semantics, and collapse state remain required

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] orientation, ratio, and collapse semantics match
- [ ] keyboard-resize behavior matches
- [ ] drag-to-collapse thresholds match (2% / 98%)
- [ ] collapse toggle visibility rules match
- [ ] fixed-size pane allocation matches
- [ ] ratio clamping to [0.05, 0.95] matches

### Tier 2: Visual Parity

- [ ] divider emphasis and pane separation use comparable token roles
- [ ] collapse toggle placement and direction match
- [ ] pane overflow behavior matches

### Tier 3: Implementation Freedom

- [ ] drag physics and resize cadence stay internal
- [ ] animation/transition approach may differ

## 12. Specimen Definitions

### Group: Horizontal Split

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Horizontal split | `orientation="horizontal"`, primary slot with "Primary pane", secondary slot with "Secondary pane" | Two side-by-side panes divided by a vertical divider; resizable horizontally |

### Group: Vertical Split

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Vertical split | `orientation="vertical"`, primary slot with "Primary pane", secondary slot with "Secondary pane" | Two stacked panes divided by a horizontal divider; resizable vertically |
