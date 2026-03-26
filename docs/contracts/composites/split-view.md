# SplitView

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `SplitView`
- Layer: `composites`
- Summary: a resizable layout container that divides space between two pane
  regions with optional collapse toggles and drag-to-collapse behavior
- In scope: orientation, divider semantics, size ratios, collapsible panes,
  keyboard-resizable separators, collapse toggle buttons
- Out of scope: nested dock orchestration policy, persistence backend,
  app-specific pane content

## 2. Anatomy

```text
[Root]
  ├── [PrimaryPane]
  │     └── (slot: primary)
  ├── [Divider]
  │     ├── [ResizeHandle]
  │     └── [Toggles]        (optional)
  │           ├── [CollapseToggle: primary]    (optional)
  │           └── [CollapseToggle: secondary]  (optional)
  └── [SecondaryPane]
        └── (slot: secondary)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | flex container for pane layout | layout only |
| PrimaryPane | yes | first pane region | min sizes, overflow |
| Divider | yes | resize handle container and visual separator | layout only |
| ResizeHandle | yes | draggable/keyboard-resizable separator | (uses ResizeHandle primitive) |
| Toggles | no | overlay container for collapse toggle buttons | layout only |
| CollapseToggle | no | button to collapse/expand a pane | (uses CollapseToggle primitive) |
| SecondaryPane | yes | second pane region | min sizes, overflow |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | split axis |
| `ratio` | `number` | `0.5` | no | controlled primary split ratio (0.05 to 0.95) |
| `defaultRatio` | `number` | `0.5` | no | uncontrolled initial ratio |
| `minPrimarySize` | `number \| null` | `null` | no | minimum primary pane size in px |
| `minSecondarySize` | `number \| null` | `null` | no | minimum secondary pane size in px |
| `primaryCollapsed` | `boolean` | `false` | no | collapse state for primary pane |
| `secondaryCollapsed` | `boolean` | `false` | no | collapse state for secondary pane |
| `showCollapsePrimary` | `boolean` | `false` | no | show collapse toggle for primary pane |
| `showCollapseSecondary` | `boolean` | `false` | no | show collapse toggle for secondary pane |
| `ariaLabel` | `string \| null` | `null` | no | accessible name (defaults to "Split view") |
| `disabled` | `boolean` | `false` | no | disables resize and collapse interactions |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Slots

| Slot | Purpose |
|------|---------|
| `primary` | content for the primary (first) pane |
| `secondary` | content for the secondary (second) pane |

### Controlled And Uncontrolled

- controlled: `ratio` plus `ratioChange` event
- uncontrolled: `defaultRatio` (internal state tracks ratio)
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
| disabled | `disabled=true` | resize handle and collapse toggles non-interactive |

### Component States

Internal state: `uncontrolledRatio`, `dragMousePos` for resize tracking.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `ratioChange` | resize commits or streams | `{ ratio: number }` | host decides persistence cadence |
| `primaryCollapsedChange` | primary pane collapse state changes | `{ isCollapsed: boolean }` | fires on toggle click or drag-to-collapse |
| `secondaryCollapsedChange` | secondary pane collapse state changes | `{ isCollapsed: boolean }` | fires on toggle click or drag-to-collapse |

## 6. Drag-To-Collapse Behavior

During resize dragging, the split view supports automatic collapse:

- **Collapse threshold**: dragging below 2% collapses the primary pane;
  dragging above 98% collapses the secondary pane
- **On collapse**: ratio resets to 0.5, collapse event dispatched
- **On uncollapse**: dragging from a collapsed state uncollapses the pane
  (ratio set to 0.05 or 0.95 respectively) before normal resize resumes
- **Ratio clamping**: ratio is always clamped to [0.05, 0.95] range

## 7. Collapse Toggle Behavior

- Primary collapse toggle is shown only when `showCollapsePrimary=true` AND
  secondary is not collapsed
- Secondary collapse toggle is shown only when `showCollapseSecondary=true` AND
  primary is not collapsed
- Toggle direction adapts to orientation:
  - horizontal: left/right
  - vertical: up/down
- Toggle labels describe action: "Collapse primary" / "Expand primary"

## 8. Accessibility

### Semantics

- Role: group container with `aria-label` (defaults to "Split view")
- Divider contains `ResizeHandle` primitive with separator semantics
- CollapseToggle buttons have dynamic `aria-label` describing action

### Keyboard

| Key | Behavior |
|-----|----------|
| arrow keys on divider | adjusts ratio via `resizeStep` events |
| `Home` / `End` | optional jump to min/max positions |
| `Enter` / `Space` | on collapse toggle: toggles pane collapse |
| `Tab` | reaches divider, toggles, and pane content in logical order |

### Focus And Announcement

- focus entry: ResizeHandle becomes focusable when keyboard resizing is supported
- focus exit: divider focus clears while pane sizing remains updated
- live-region behavior: none; resize and collapse state conveyed through
  control semantics
- GPUI-native accessibility mapping notes: GPUI must expose resizable separators
  with orientation and value semantics, not just pointer-only drag handles

## 9. Layout

### Sizing

- root fills assigned parent space (100% width and height)
- flex direction: row for horizontal, column for vertical
- primary pane flex: `0 0 {ratio*100}%` (or `0 0 0` when collapsed, `1 1 0` when opposite collapsed)
- secondary pane flex: `1 1 0` (or `0 0 0` when collapsed)
- divider width: 0.5rem (horizontal) or height: 0.5rem (vertical)
- min-size constraints applied via inline style when not collapsed
- panes have `overflow: hidden`

### Composition

- composes: `ResizeHandle`, `CollapseToggle` primitives
- parent expectations: workspace shells, panel layouts, utility views
- child expectations: any content via primary/secondary slots
- resizing rules: child focus continuity should survive ratio changes and
  collapse/restore operations

## 10. Token Usage And Precise CSS

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-orientation` | root `<div>`, divider `<div>` | `"horizontal"`, `"vertical"` |
| `data-primary-collapsed` | root `<div>` | present when true |
| `data-secondary-collapsed` | root `<div>` | present when true |
| `data-disabled` | divider `<div>` | present when true |
| `data-has-toggles` | divider `<div>` | present when true |

### Root

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

### Pane

| Property | Value |
|----------|-------|
| min-width | `0` |
| min-height | `0` |

### Divider

| Property | Value |
|----------|-------|
| position | `relative` |
| display | `flex` |
| align-items | `center` |
| justify-content | `center` |
| flex-shrink | `0` |

#### Divider Horizontal

| Property | Value |
|----------|-------|
| width | `0.5rem` |
| height | `100%` |

#### Divider Vertical

| Property | Value |
|----------|-------|
| height | `0.5rem` |
| width | `100%` |

### Toggles

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

### Light Theme Overrides

None.

## 11. Svelte Notes

- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- uses `createEventDispatcher` for all events
- `bind:this={container}` on root for computing raw ratio from mouse position
- `rawRatio()` converts mouse position to ratio using container bounding rect
- primary/secondary collapse toggles use `CollapseToggle` from `@poodle/svelte-primitives`
- resize events handled via `ResizeHandle` `resizeStart`/`resizeMove`/`resizeStep` events
- pane visibility controlled via `{#if !isPrimaryCollapsed}` / `{#if !isSecondaryCollapsed}`

## 12. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::split_view`
- implementation-only details: GPUI may use native splitter support or custom
  layout code, but keyboard resizing, orientation semantics, and collapse state
  remain required

## 13. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] orientation, ratio, and collapse semantics match
- [ ] keyboard-resize behavior matches
- [ ] drag-to-collapse thresholds match (2% / 98%)
- [ ] collapse toggle visibility rules match

### Tier 2: Visual Parity

- [ ] divider emphasis and pane separation use comparable token roles
- [ ] collapse toggle placement matches

### Tier 3: Implementation Freedom

- [ ] drag physics and resize cadence stay internal

## 14. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact resize feel may differ | runtime event systems differ | allowed | keep keyboard parity and ratio meaning strict |

## 15. Specimen Definitions

### Group: Horizontal split

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Horizontal split | `orientation="horizontal"`, primary slot with "Primary pane", secondary slot with "Secondary pane" | Two side-by-side panes divided by a vertical divider; resizable horizontally |

### Group: Vertical split

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Vertical split | `orientation="vertical"`, primary slot with "Primary pane", secondary slot with "Secondary pane" | Two stacked panes divided by a horizontal divider; resizable vertically |

## 16. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: docked workspace shells, floating split inspectors,
  multi-pane utilities
- future follow-up: connect nested split orchestration and persistence later
