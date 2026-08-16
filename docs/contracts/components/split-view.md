# SplitView

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `SplitView`
- Layer: `composites`
- Summary: a resizable layout container that divides space between two pane
  regions with an interactive divider, optional collapse toggles, drag-to-collapse
  behavior, rail-collapse mode for dock-style panes, fixed-size pane support,
  and keyboard-resizable separators
- In scope: orientation (horizontal/vertical), divider semantics, ratio-based
  and fixed-size pane allocation, collapsible panes with toggle buttons,
  drag-to-collapse thresholds, rail-collapse (collapse to a pinned pixel size
  with content mounted), keyboard-resizable separators via ResizeHandle,
  min-size constraints, size and density support
- Out of scope: nested dock orchestration policy, persistence backend,
  app-specific pane content

## 2. Anatomy

```text
[Root .split-view]  <div aria-label>
  ├── [PrimaryPane .split-view__pane--primary]  <div>
  │     └── (snippet: primary)
  ├── [Divider .split-view__divider]  <div>
  │     └── [ResizeHandle]  ResizeHandle primitive
  ├── [SecondaryPane .split-view__pane--secondary]  <div>
  │     └── (snippet: secondary)
  └── [Toggles .split-view__toggles]  <div> (optional)
        ├── [CollapseToggle: primary]  CollapseToggle primitive (optional)
        └── [CollapseToggle: secondary]  CollapseToggle primitive (optional)
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
| `primaryHidden` / `secondaryHidden` | `boolean` | `false` | no | pane takes zero space without being a collapse: no toggle, no collapsed data attribute — for panes that are absent, not user-collapsed |
| `secondaryCollapsed` | `boolean` | `false` | no | collapse state for secondary pane |
| `primaryCollapsedSize` | `number \| null` | `null` | no | rail-collapse: when primary is collapsed, pin it to this pixel size with content mounted instead of hiding it |
| `secondaryCollapsedSize` | `number \| null` | `null` | no | rail-collapse: when secondary is collapsed, pin it to this pixel size with content mounted instead of hiding it |
| `collapsePrimaryBelowSize` | `number \| null` | `null` | no | during divider drag, request primary collapse when its pixel size would drop below this value |
| `collapseSecondaryBelowSize` | `number \| null` | `null` | no | during divider drag, request secondary collapse when its pixel size would drop below this value |
| `showCollapsePrimary` | `boolean` | `false` | no | show collapse toggle for primary pane |
| `showCollapseSecondary` | `boolean` | `false` | no | show collapse toggle for secondary pane |
| `toggleVisibility` | `"always" \| "hover"` | `"always"` | no | when the collapse-toggle pill is visible; `"hover"` reveals it only on the seam (see Toggle reveal below) |
| `divider` | `boolean` | `false` | no | paint the visible divider line; off by default since pane borders read as the separator and the resize grab area is an overlay with no layout footprint |
| `ariaLabel` | `string \| null` | `null` | no | accessible name (defaults to "Split view") |
| `disabled` | `boolean` | `false` | no | disables resize and collapse interactions |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl" \| null` | `null` | no | explicit control size override |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Snippets

| Snippet | Purpose |
|------|---------|
| `primary` | content for the primary (first) pane |
| `secondary` | content for the secondary (second) pane |

### Controlled And Uncontrolled

- controlled: `ratio` plus `onRatioChange`
- uncontrolled: `defaultRatio` (internal state tracks ratio)
- fixed-size: `primarySize` or `secondarySize` override ratio-based allocation
- collapse states (`primaryCollapsed`, `secondaryCollapsed`) are externally
  owned; changes requested through callbacks

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| steady | default | both panes visible with divider between |
| resizing | pointer or keyboard resize active | divider focus/emphasis visible |
| primary-collapsed | `primaryCollapsed=true`, no `primaryCollapsedSize` | primary pane hidden (`flex: 0 0 0`), secondary fills space |
| secondary-collapsed | `secondaryCollapsed=true`, no `secondaryCollapsedSize` | secondary pane hidden (`flex: 0 0 0`), primary fills space |
| primary-railed | `primaryCollapsed=true` AND `primaryCollapsedSize` set | primary pane pinned to `flex: 0 0 {primaryCollapsedSize}px`, content stays mounted, secondary fills space |
| secondary-railed | `secondaryCollapsed=true` AND `secondaryCollapsedSize` set | secondary pane pinned to `flex: 0 0 {secondaryCollapsedSize}px`, content stays mounted, primary fills space |
| fixed-primary | `primarySize` is set | primary pane uses fixed pixel flex, secondary fills remaining space |
| fixed-secondary | `secondarySize` is set | secondary pane uses fixed pixel flex, primary fills remaining space |
| disabled | `disabled=true` | resize handle and collapse toggles non-interactive |
| toggles-hidden | `toggleVisibility="hover"`, pointer off the seam, no focus in the pill, neither pane collapsed | toggle pill at opacity 0 and non-interactive; still mounted and in the accessibility tree |
| toggles-revealed | `toggleVisibility="hover"` and any of: pointer over the divider, pointer over the pill, focus inside the pill, either pane collapsed | toggle pill at full opacity and interactive |

Toggle visibility: an open pane's collapse toggle renders while its sibling
is open, and a collapsed pane's expand toggle always renders. With both panes
collapsed both expand toggles stay — a collapse pair is never unrecoverable.
A fully collapsed pane anchors its toggles to the viewport edge: the pill
peeks inward, flat side flush with the edge, rather than hanging half out of
view. Hidden panes (`primaryHidden`/`secondaryHidden`) are absent, not
collapsed: they render no toggle and never pin the pill under hover-reveal.

Toggle reveal: with `toggleVisibility="hover"` the pill rests at opacity 0
and is revealed by the pointer entering the seam — the resize handle's grab
area or the pill itself — or by focus landing inside it. It is never
unmounted: the buttons stay in the DOM and the accessibility tree so `Tab`
reaches them, and `:focus-within` brings them back on screen when it does.
While hidden the pill takes no pointer events, so it cannot swallow clicks
aimed at pane content; the grab strip underneath is what triggers the reveal.
A collapsed pane overrides the mode and pins the pill visible: its expand
toggle is the only way back, and a collapsed pane leaves the seam on the
container edge with nothing left to hover toward.

### Component States

Internal state: `uncontrolledRatio`, `dragMousePos` for resize tracking.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|----------|--------------|---------|-------|
| `onRatioChange` | resize commits or streams | `number` | host decides persistence cadence |
| `onPrimaryCollapsedChange` | primary pane collapse state changes | `boolean` | runs on toggle click or drag-to-collapse |
| `onSecondaryCollapsedChange` | secondary pane collapse state changes | `boolean` | runs on toggle click or drag-to-collapse |

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
| arrow keys on divider | adjusts ratio via `ResizeHandle` step callbacks |
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
- divider width: `var(--poodle-resize-handle-thickness, 0.125rem)` (horizontal)
  or height (vertical) — the divider matches the handle's line, not its grab
  area; ResizeHandle overlays the grab area so the panes stay flush
- min-size constraints applied via inline style when not collapsed
- panes have `overflow: hidden`

### Composition

- composes: `ResizeHandle`, `CollapseToggle` primitives
- parent expectations: workspace shells, panel layouts, utility views
- child expectations: any content via primary/secondary snippets
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
| `data-divider` | root `<div>` | `"line"` when the `divider` prop is true; absent otherwise (zero-footprint seam) |
| `data-toggle-visibility` | root `<div>` | `"always"`, `"hover"` |

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
| primaryCollapsed (no collapsed size) | `0 0 0` | `1 1 0` |
| secondaryCollapsed (no collapsed size) | `1 1 0` | `0 0 0` |
| primaryCollapsed + primaryCollapsedSize | `0 0 {primaryCollapsedSize}px` | `1 1 0` |
| secondaryCollapsed + secondaryCollapsedSize | `1 1 0` | `0 0 {secondaryCollapsedSize}px` |
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
| width | `0` by default; `var(--poodle-resize-handle-thickness, 0.125rem)` with `data-divider="line"` |
| height | `100%` |

#### Divider Vertical (`[data-orientation="vertical"]`)

| Property | Value |
|----------|-------|
| height | `0` by default; `var(--poodle-resize-handle-thickness, 0.125rem)` with `data-divider="line"` |
| width | `100%` |

### Toggles (`.split-view__toggles`)

| Property | Value |
|----------|-------|
| position | `absolute` |
| z-index | `2` |
| `--poodle-split-toggles-reveal` | `1` |
| `--poodle-split-toggles-state-opacity` | `1` |
| opacity | `calc(var(--poodle-split-toggles-reveal) * var(--poodle-split-toggles-state-opacity))` |
| display | `flex` |
| align-items | `center` |
| justify-content | `center` |
| gap | `0.125rem` |
| padding | `0.125rem` |
| border-radius | `var(--poodle-radius-pill)` |
| background | `color-mix(in srgb, var(--poodle-color-background-panel) 92%, var(--poodle-color-background-elevated))` |
| box-shadow | `0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-default) 70%, transparent)` |

#### Toggles Horizontal

| Property | Value |
|----------|-------|
| flex-direction | `column` |
| top | `50%` |
| left | `var(--poodle-split-seam, 50%)` |
| transform | `translate(-50%, -50%)` |

#### Toggles Vertical

| Property | Value |
|----------|-------|
| flex-direction | `row` |
| top | `var(--poodle-split-seam, 50%)` |
| left | `50%` |
| transform | `translate(-50%, -50%)` |

The toggles are a direct child of the root, positioned along
`--poodle-split-seam` — a CSS length the component computes from the ratio
and collapse state — never against the divider box, which a collapsed or
hidden sibling pane can leave degenerate. Fully collapsed panes anchor the
pill to the viewport edge instead (flat side out; see Toggle visibility in
§4).

`z-index: 2` puts the pill above the divider (`z-index: 1`) and above the
resize handle's grab overlay nested inside it. Without it the handle's
`0.5rem` hit strip runs down the middle of the pill and swallows the clicks,
leaving the toggles reachable only by aiming either side of the seam line.

Reveal and disabled dim are separate axes that both land on `opacity`, so
they compose through the two custom properties above rather than overriding
one declaration — otherwise whichever selector is more specific silently
wins and a disabled hover-reveal split shows its dimmed pill permanently.

#### Toggles Hover Reveal (`[data-toggle-visibility="hover"]`)

| Selector | Declarations |
|----------|--------------|
| `[data-toggle-visibility="hover"] .split-view__toggles` | `--poodle-split-toggles-reveal: 0`; `pointer-events: none`; `transition: opacity 120ms ease` |
| …`.split-view__divider:hover ~ .split-view__toggles`, …`.split-view__divider:hover .split-view__toggles`, …`.split-view__toggles:hover`, …`.split-view__toggles:focus-within` | `--poodle-split-toggles-reveal: 1`; `pointer-events: auto` |
| `[data-toggle-visibility="hover"][data-primary-collapsed] .split-view__toggles`, `[data-toggle-visibility="hover"][data-secondary-collapsed] .split-view__toggles` | `--poodle-split-toggles-reveal: 1`; `pointer-events: auto` |
| `@media (prefers-reduced-motion: reduce)` | `transition: none` |

Two reveal selectors cover the two anatomies: the sibling combinator for the
toggles as a root sibling of the divider (Svelte), the descendant form for
the toggles nested inside it (React).

#### Toggles Disabled

| Selector | Declarations |
|----------|--------------|
| `.split-view[data-disabled] > .split-view__toggles` | `--poodle-split-toggles-state-opacity: var(--poodle-state-opacity-disabled)` |

### Composed Primitives

Token usage for `ResizeHandle` and `CollapseToggle` is defined in their
respective primitive contracts.

### Drag-To-Collapse Behavior (legacy, no collapsed size configured)

| Threshold | Action |
|-----------|--------|
| ratio < 0.02 during drag | collapses primary pane, resets ratio to 0.5 |
| ratio > 0.98 during drag | collapses secondary pane, resets ratio to 0.5 |
| drag starts while primary collapsed | uncollapses primary, sets ratio to 0.05 |
| drag starts while secondary collapsed | uncollapses secondary, sets ratio to 0.95 |
| ratio clamping | always clamped to [0.05, 0.95] range |

### Rail-Collapse Drag Behavior (`collapse*BelowSize` configured)

SplitView owns the drag lifecycle, so rail collapse and expand are resolved
from drag intent rather than from ratio stream heuristics:

| Condition | Action |
|-----------|--------|
| drag would size the pane below `collapse*BelowSize` px | requests collapse via the collapse callback; the last expanded ratio is preserved (no 0.5 reset) |
| dragging while railed, away from the collapsed edge, past `collapse*BelowSize + 8` px | requests expand via the collapse callback and resumes ratio tracking from the pointer position |
| dragging while railed, within the rail band | no ratio change is emitted |
| drag release while railed | pane stays railed |
| `onRatioChange` while railed | never emitted |

Rail collapse and legacy edge collapse are mutually exclusive per pane: when
`collapse*BelowSize` is set the 2% / 98% edge thresholds and their 0.5 ratio
reset do not apply to that pane.

### Collapse Toggle Visibility Rules

| Toggle | Shown When |
|--------|-----------|
| primary collapse | `showCollapsePrimary=true` AND secondary is not collapsed |
| secondary collapse | `showCollapseSecondary=true` AND primary is not collapsed |

These rules decide what is *rendered*. `toggleVisibility` is a separate,
purely presentational axis on top: it never removes a toggle from the DOM or
the accessibility tree, it only decides whether the rendered pill is painted.

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
- uses callback props for ratio and collapse-state change requests
- `bind:this={container}` on root for computing raw ratio from mouse position
- `rawRatio()` converts mouse position to ratio using container bounding rect
- primary/secondary collapse toggles use `CollapseToggle` from `@inflatable-cookie/poodle-svelte`
- resize callbacks handled via `ResizeHandle` `onResizeStart` / `onResizeMove` / `onResizeStep`
- pane content conditionally rendered: `{#if !primaryCollapsed}` / `{#if !secondaryCollapsed}`;
  with a rail-collapse size configured the content stays mounted while railed
- `SplitOrientation`, `CollapseDirection`, `ControlSize`, `SemanticControlSizeRole`,
  `ControlDensity` types imported from `@inflatable-cookie/poodle-svelte`
- `ResizeHandle` and `CollapseToggle` imported from `@inflatable-cookie/poodle-svelte`
- `data-primary-collapsed` and `data-secondary-collapsed` use `|| undefined` to
  omit the attribute when false
- `data-toggle-visibility` is always written (both values are meaningful), so
  it does not take the `|| undefined` treatment
- hover reveal is entirely CSS: no pointer listeners, no reactive state

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::split_view`
- spec struct: `SplitViewSpec` with orientation, ratio, collapse states,
  fixed sizes, min sizes, disabled, size, density
- `on_ratio_change(ratio)` streams during a divider drag, clamped to
  [0.05, 0.95]: the divider starts a gpui drag and the split root listens
  for its moves, whose bounds give the axis extent a ratio needs. The drag
  state lives in gpui, not the component, so mid-drag re-renders (each
  ratio emission causes one) do not drop the gesture. Hosts composing more
  than one resizable split give each a distinct `with_id`.
- keyboard resizing is not implemented (no focus/key routing yet); collapse
  state and orientation semantics are.
- `toggle_visibility` is honoured through the shared render tier
  (`poodle_render::split_view`): the toggle cluster rests at
  `opacity: 0` with a hover `StylePatch` restoring `1.0`. Opacity is
  paint-only in the backend, so the cluster still hit-tests while invisible —
  its own bounds are the reveal region.

## 10a. Jetstream Notes

- `SplitView::from_spec(spec, theme).primary(...).secondary(...)` then
  `.on_primary_collapse(...)` / `.on_secondary_collapse(...)`, forwarded to the
  composed `CollapseToggle`s.
- The divider forwards its drag as `.on_resize(phase, axis_delta)` — the
  composed `ResizeHandle`'s gesture verbatim: `Start`/`End` bracket it, `Move`
  carries the delta in logical px along the split axis.
- Known Delta: this is a pixel delta, not the contract's
  `onRatioChange(ratio)`. Converting needs the rendered axis extent, which
  the immediate-mode build never sees and the host (who laid the split out)
  already has — it applies `ratio += delta / extent` and clamps. Emitting a
  ratio from a guessed extent would be worse than an honest delta.
- `toggle_visibility` is honoured through the same shared render tier as GPUI;
  `jetstream-poodle` maps the hover `StylePatch`'s `opacity` onto the
  element's hover override.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] orientation, ratio, and collapse semantics match
- [ ] keyboard-resize behavior matches
- [ ] drag-to-collapse thresholds match (2% / 98%)
- [ ] rail-collapse semantics match (`*CollapsedSize`, `collapse*BelowSize`,
      preserved ratio, mounted content, no ratio emission while railed)
- [ ] collapse toggle visibility rules match
- [ ] `toggleVisibility` semantics match (hidden until the seam is hovered,
      never unmounted, collapsed panes pin the pill visible)
- [ ] fixed-size pane allocation matches
- [ ] ratio clamping to [0.05, 0.95] matches

### Tier 2: Visual Parity

- [ ] divider emphasis and pane separation use comparable token roles
- [ ] collapse toggle placement and direction match
- [ ] pane overflow behavior matches

### Tier 3: Implementation Freedom

- [ ] drag physics and resize cadence stay internal
- [ ] animation/transition approach may differ

## 11a. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| The zero-footprint seam (default off) is shared by the web targets; React still lacks the `divider` opt-in and the root `--poodle-split-seam` anchoring, and the natives still paint the divider line | the seam behavior was proven in the Svelte host first; React matches the default-off anatomy but not the opt-in or the collapsed-pane seam tracking | pending review | port the `divider` opt-in and seam anchoring to React, and the default-off to the natives, when one is next touched |
| The both-collapsed expand-toggle rule is web parity | the trap was proven against the Svelte host and the React mirror now matches; the natives hide both toggles when both panes are collapsed | pending review | port to the natives with the seam work |
| The natives' hover-reveal zone is the toggle cluster's own bounds, not the divider's grab strip — the pointer must reach the pill rather than the seam around it | the shared render tier has no absolute positioning for the cluster, so it sits inline beside the handle rather than overlaying it; there is no "hovering a sibling reveals me" primitive in the node vocabulary | pending review | widen when the node vocabulary gains group-hover |
| The natives have no `:focus-within` equivalent, so a keyboard-focused toggle is not revealed | GPUI has no accessibility/focus routing for the split, and Jetstream's focus does not feed the hover patch | pending review | revisit with native keyboard resize |

## 12. Specimen Definitions

### Group: Horizontal Split

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Horizontal split | `orientation="horizontal"`, primary snippet with "Primary pane", secondary snippet with "Secondary pane" | Two side-by-side panes divided by a vertical divider; resizable horizontally |

### Group: Vertical Split

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Vertical split | `orientation="vertical"`, primary snippet with "Primary pane", secondary snippet with "Secondary pane" | Two stacked panes divided by a horizontal divider; resizable vertically |

### Group: Toggle Visibility

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Hover-revealed toggles | `toggleVisibility="hover"`, `showCollapsePrimary`, `showCollapseSecondary` | No pill at rest; moving the pointer onto the seam fades it in, and it is clickable across its whole width rather than either side of the line |
