# Tree

Status: detailed contract
Updated: 2026-09-02

## 1. Purpose

- Component name: `Tree`
- Layer: `composites`
- Summary: hierarchical, collapsible disclosure list for file explorers, nested
  navigation, and outline views (VSCode-style file tree)
- In scope: recursive nodes, branch expand/collapse with a twisty, multi-node
  selection (single, toggle, and range), optional per-node leading icons,
  indentation guide lines, size and density scaling, disabled nodes, controlled
  and uncontrolled expansion, full WAI-ARIA `tree` keyboard model, focus-visible
  ring, checkbox / tri-state cascade selection, lazy / async child loading,
  inline rename, right-click context menu, drag-and-drop + keyboard reorder,
  and (Svelte only) virtual scrolling
- Out of scope: horizontal scrolling chrome, drag-and-drop **across** separate
  tree instances, and virtual scrolling in the Rust runtimes (no windowing
  primitive). These remain documented extension points.

## 2. Anatomy

```text
[Root <div role="tree">]
  └── [TreeItem <div role="treeitem">]*            (recursive)
        ├── [Row <div>]
        │     ├── [Indent <div>]*                  (depth cells, guide border)
        │     ├── [Twisty <div>]                   (chevron for branch, spacer for leaf)
        │     ├── [Icon]                           (optional, when showIcons)
        │     ├── [Label <span>]
        │     └── [EndLabel <span>]                (optional compact metadata)
        └── [Group <div role="group">]             (children, only when expanded)
              └── [TreeItem]*                       (recursion)
```

### Parts

| Part | Element | Required | Notes |
|------|---------|----------|-------|
| Root | `<div role="tree">` | yes | Class `tree`, `data-size`, `data-density`, `data-size-role`, `aria-label`, `aria-multiselectable="true"` |
| TreeItem | `<div role="treeitem">` | yes | Class `tree__item`, the focusable/interactive element: carries roving `tabindex`, pointer + keyboard handlers, `aria-level`, `aria-selected`, `aria-expanded` (branch only), `aria-disabled`, `data-branch`, `data-selected` |
| Row | `<div>` | yes | Class `tree__row`, presentational highlight surface (first child of the item; never focusable itself) |
| Indent | `<div>` | yes (per depth) | Class `tree__indent`, fixed-width cell; draws a vertical guide via left border when `showGuides` |
| Twisty | `<div>` | yes | Class `tree__twisty`; chevron-right collapsed / chevron-down expanded for branches, empty spacer for leaves |
| Checkbox | `Checkbox` | no | Class `tree__checkbox`; rendered before the icon when `showCheckboxes`. Tri-state (checked / unchecked / mixed) derived from descendants |
| Icon | `Icon` | no | Class `tree__icon`; rendered when `showIcons` and `node.icon` set; empty box reserves alignment when `showIcons` and no icon |
| Label | `<span>` | yes | Class `tree__label`, single-line, ellipsis-truncated; swapped for RenameInput when `editingValue` matches |
| EndLabel | `<span>` | when `endLabel` set | Class `tree__end-label`, compact trailing metadata aligned after the flexible label |
| RenameInput | `<input>` | when editing | Class `tree__rename`; inline text editor seeded with the node label (F2 / context-menu Rename) |
| LoadingRow | `<div role="treeitem">` | when lazy branch loading | A spinner + "Loading…" row shown under an expanded branch whose children are still loading (`loadingValues`) |
| Group | `<div role="group">` | when branch expanded | Class `tree__group`, contains child TreeItems |

### Cascade selection, lazy loading, virtual scroll

- **Cascade checkbox** (`showCheckboxes`): every node is checkable. The checkable
  atoms are leaves (and empty/lazy branches); a branch's state is derived —
  `checked` when all descendant atoms are checked, `unchecked` when none, else
  `mixed` (indeterminate). Toggling a node checks/unchecks all its atoms. Stored
  in `checkedValues` (atoms only); branch values are not stored.
- **Lazy loading** (`loadingValues` + `onLoadChildren`): expanding an empty
  branch (`isBranch` with no `children`) fires `onLoadChildren(value)`. While the
  value is in `loadingValues`, a LoadingRow renders under it. The host fetches,
  injects `children`, and clears the value from `loadingValues`.
- **Virtual scroll** (`virtualized`, **Svelte only**): the flattened visible
  rows render as a windowed flat list (valid WAI-ARIA via `aria-level`) inside a
  fixed-height scroll viewport with top/bottom offset spacers; only the visible
  window (+ overscan) is in the DOM. The Rust runtimes have no windowing
  primitive and render all visible rows.
- **Inline rename** (`editingValue` + `onRenameCommit`/`onRenameCancel`): when a
  node is being edited its label swaps for a text input seeded with the current
  label. F2 starts rename; Enter / blur commits; Escape cancels. Svelte uses an
  `<input>`; GPUI reuses the `EditableLabel` primitive; Jetstream renders an
  editor box and routes character input through the app loop.
- **Context menu** (`onContextMenu`): right-clicking a row fires
  `onContextMenu(value, x, y)`; the host opens its own `ContextMenu` at the
  pointer (items are app-specific). Jetstream polls the platform right-button and
  renders the menu as a positioned overlay routed by token.
- **Reorder** (`reorderable` + `onReorder`): rows are substrate drag sources; a drop fires
  `onReorder(from, to, position)` where `position` ∈ `before`/`after`/`inside`.
  The hovered row is the indicator anchor; the session's commit destination is
  the `{to, position}` the geometry resolved, and eligibility, announcements,
  and `onReorder` all use that destination — never a privately recomputed one.
  Hovering an expanded source does not land before its first child (own
  subtree). Y picks the band on the hovered row; `inside` on a folder appends as last
  child, including when that folder is the dragged node's parent or an
  immediate sibling. An `after` on the last visible
  descendant of an open parent then offers every ancestor that ends at that
  gap — including the dragged next sibling's whole row (the gap above it),
  that last descendant's whole row, and the last descendant itself when
  nothing follows it at the bottom of the tree. Vertical movement between
  those two rows does not change depth. The gap above an open folder is only before the
  folder — it does not indent, even if the pointer moves right. Nest from the
  folder row or from the gap below. `inside` appends as last child; the gap
  between an open folder header and its first child is before that child.
  Depth
  steps are two indent columns, so a root filename stays at root until the
  pointer moves clearly into the nested icon column. The drop line indents to
  the icon column at the chosen depth as the pointer moves, including when
  only X changes on the same row.
  Same-parent leaves land *at* the hovered row.
  Alt+↑/↓ moves the focused node among siblings through
  `requestKeyboardDrop` over the visible logical target catalogue; it does
  not call `onReorder` directly. Space/Enter remain selection/activation.
  The shared `reorder_nodes(nodes, from, to, position)` helper performs the
  move (no-op for self / missing / dropping into own subtree). An activated
  pointer drag does not deliver a trailing row click or selection change; a tap
  or pre-threshold release still selects. Svelte and
  React use the shared web drag substrate (source/focus owner on the
  `treeitem`, pointer/touch on the row handle, no HTML `DataTransfer`). GPUI
  uses `on_drag`/`on_drop`/`drag_over`; Jetstream tracks mouse down→up over
  rows.
- **Externally authoritative reorder** (`reorderable` + `reorderAuthority`, paired
  Svelte/React): the host projects an ordered moving set once at session start,
  inspects the resolved Tree destination, and synchronously accepts, rewrites,
  or refuses it before an accepted indicator is painted. The accepted
  destination drives indicator depth, announcement, drop-time revalidation,
  and the one sync/async commit callback. This is an adapter over the same
  Tree source, targets, outline geometry, keyboard route, auto-scroll, focus,
  and terminal lifecycle. It is not a second drag machine. `onReorder` remains
  the convenience path and is not fired for an authority-owned session.

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `nodes` | `TreeNode[]` | `[]` | yes | Root-level nodes |
| `selectedValues` | `string[]` | `[]` | no | Values of selected nodes (multi-select) |
| `expandedValues` | `string[] \| null` | `null` | no | Controlled set of expanded branch values |
| `defaultExpandedValues` | `string[]` | `[]` | no | Uncontrolled initial expansion (used when `expandedValues` is null) |
| `checkedValues` | `string[]` | `[]` | no | Checked checkable nodes (cascade). Branch state is derived, not stored |
| `loadingValues` | `string[]` | `[]` | no | Branches currently loading children (renders a loading row) |
| `editingValue` | `string \| null` | `null` | no | Value of the node in inline-rename mode (swaps label for a text input) |
| `ariaLabel` | `string \| null` | `null` | no | Accessible name for the tree region |
| `showGuides` | `boolean` | `true` | no | Render vertical indentation guide lines |
| `collapseTwistyWhenFlat` | `boolean` | `false` | no | Reclaim the twisty gutter while no node in the tree can expand; the spacer returns as soon as one can |
| `showIcons` | `boolean` | `true` | no | Reserve and render the leading icon slot |
| `showCheckboxes` | `boolean` | `false` | no | Render a leading cascade checkbox per row |
| `reorderable` | `boolean` | `false` | no | Allow drag-and-drop + Alt+↑/↓ row reordering |
| `reorderAuthority` | `TreeReorderAuthority \| null` | `null` | no | **Svelte/React.** External subject, eligibility, destination rewrite, and authoritative commit adapter. Mutually exclusive with `onReorder` |
| `virtualized` | `boolean` | `false` | no | **Svelte only.** Window the flattened visible rows in a scroll viewport |
| `virtualHeight` | `number` | `320` | no | **Svelte only.** Viewport height (px) when `virtualized` |
| `size` | `ControlSize \| null` | `null` | no | Absolute size override |
| `sizeRole` | `SemanticControlSizeRole` | `"chrome"` | no | Semantic size intent |
| `density` | `ControlDensity \| null` | `null` | no | Density override |

### Callbacks (added)

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onCheckedChange` | A cascade checkbox toggles | `string[]` | Full next checked set |
| `onLoadChildren` | An empty branch is expanded | `string` | Lazy-load request; host fetches + injects children |
| `onRenameCommit` | Inline rename commits (Enter / blur) | `(value, text)` | Host applies the new label |
| `onRenameCancel` | Inline rename cancels (Escape) | — | Host clears `editingValue` |
| `onContextMenu` | Right-click on a row | `(value, x, y)` | Host opens a `ContextMenu` at the pointer |
| `onReorder` | Drag-drop or Alt+↑/↓ | `(from, to, position)` | `position` is `before` / `after` / `inside`; host applies the move (`reorder_nodes`) |
| `onActivate` | Row double-click, or the keyboard activate intent | `string` | The node's value. Keyboard activation selects the row first, then fires; a disabled node fires nothing |

### Types: external drop authority

These types are owned by `@inflatable-cookie/poodle-core` and re-exported by
the paired component packages.

```ts
interface TreeReorderSubject {
  readonly sourceValue: string;
  readonly movingValues: readonly string[];
}

interface TreeReorderCandidate {
  readonly subject: TreeReorderSubject;
  readonly intent: DropIntent;
}

interface TreeReorderAuthority {
  projectMovingValues(
    sourceValue: string,
    selectedValues: readonly string[],
  ): readonly string[];
  canDrop(candidate: TreeReorderCandidate): DropEligibility;
  onDrop(
    candidate: TreeReorderCandidate,
  ): DragDropCommitResult | Promise<DragDropCommitResult>;
}

type TreeReorderProps =
  | {
      reorderAuthority?: null;
      onReorder?: (
        from: string,
        to: string,
        position: TreeDropPosition,
      ) => void;
    }
  | { reorderAuthority: TreeReorderAuthority; onReorder?: never };
```

`projectMovingValues` is pure and runs once per semantic session. Its ordered
result is non-empty, unique, and contains `sourceValue`; invalid output is
refused, never normalized. Every moving value must resolve in the current Tree.
The subject remains latched even if selection changes and is cleared on every
terminal or teardown. `canDrop` is synchronous. It may refuse or return an accepted
`DropEligibility` whose intent preserves the hovered target, indicator edge,
and operation while preserving or rewriting only `destination`. Poodle
validates the final destination against current nodes and every moving value.
That destination owns line depth, announcement, revalidation, and commit.
`onDrop` receives the revalidated candidate once and returns the real substrate
result; a pending Promise keeps the session in `dropping` and a late answer
cannot affect a newer session. The public props make `reorderAuthority` and
`onReorder` mutually exclusive. `reorderable` remains the explicit switch in
both branches; installing an authority does not implicitly enable dragging.

### Type: TreeNode

| Field | Type | Default | Required | Notes |
|-------|------|---------|----------|-------|
| `value` | `string` | — | yes | Stable unique key used for selection and expansion |
| `label` | `string` | — | yes | Visible row text |
| `endLabel` | `string \| null` | `null` | no | Compact metadata aligned to the row end |
| `icon` | `string \| null` | `null` | no | Leading icon name resolved through `Icon` |
| `children` | `TreeNode[]` | `[]` | no | Child nodes |
| `isBranch` | `boolean` | `false` | no | Force branch posture when `children` is empty (empty / lazy folder) |
| `isDisabled` | `boolean` | `false` | no | Disabled rows render inertly, suppress select and expand |
| `isMuted` | `boolean` | `false` | no | Reduces passive emphasis without changing interaction |

Branch rule: a node is a **branch** iff `isBranch || children.length > 0`. Branches
get a twisty and `aria-expanded`. All other nodes are **leaves** and render a
twisty-sized spacer for label alignment.

### Slots

None.

### Controlled And Uncontrolled

- Selection is controlled via `selectedValues` + `onSelectionChange`.
- Expansion supports both: controlled via `expandedValues` + `onExpandedChange`,
  or uncontrolled seeded by `defaultExpandedValues`. When `expandedValues` is
  non-null it wins; otherwise the component tracks expansion internally starting
  from `defaultExpandedValues`.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| leaf | Node has no children and `isBranch` false | No chevron; twisty-sized spacer keeps labels aligned |
| flat | `collapseTwistyWhenFlat` and no node anywhere is a branch | Twisty omitted entirely; labels align to the leading edge |
| collapsed | Branch not in expanded set | Chevron points right; children not rendered; `aria-expanded="false"` |
| expanded | Branch in expanded set | Chevron points down; child `group` rendered; `aria-expanded="true"` |
| selected | `value` in `selectedValues` | Row shows accent fill, inset accent ring, text primary; `aria-selected="true"`, `data-selected` |
| hover | Pointer over non-disabled row | Elevated background, text primary |
| focus-visible | Keyboard focus on row | Focus ring via `--poodle-border-width-focus` + `--poodle-color-accent-focusRing` |
| disabled | Node `isDisabled: true` | Reduced opacity, `cursor: not-allowed`, no select / expand, `aria-disabled="true"` |
| muted | Node `isMuted: true` | Reduced passive emphasis; hover, focus, selection, and all interaction remain active |

### Component States

State table suffices — selection and expansion are independent flat sets; there
is no transient async or open/close machine.

### Behavior Machine

Behavior classification: adapter-owned interaction (g11.004 sweep)

Keyboard tree navigation (expand/collapse, roving). Extraction debt: tree-navigation machine (largest remaining gap).

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onSelectionChange` | Selection set changes via click or keyboard | `string[]` | Full next selection set |
| `onExpandedChange` | A branch is expanded or collapsed | `string[]` | Full next expanded set |
| `onActivate` | Double-click or Enter on a node | `string` | Open intent; selection still updates separately |

## 6. Accessibility

### Semantics

- Role: Root `tree`; each node `treeitem`; child container `group`
- Required attributes: `aria-multiselectable="true"` on root; `aria-level`
  (1-based depth) and `aria-selected` on every `treeitem`; `aria-expanded` on
  branch items only; `aria-disabled` on disabled items
- Optional attributes: `aria-label` on root when context does not already label it
- Labeling rules: provide `ariaLabel` whenever the surrounding chrome does not
  name the tree

### Keyboard

Follows the WAI-ARIA `tree` (multi-select) pattern. Exactly one item is
tabbable at a time (roving tabindex); focus moves over the **visible** rows.
Implemented in all three targets: Svelte tracks focus in-component; the Rust
runtimes track it via `focused_value` on the spec (the host app owns + mutates
it) and render a focus ring on that node. Navigation uses the shared spec
helpers `visible_rows` / `next_visible` / `prev_visible` / `parent_of`.

| Key | Behavior |
|-----|----------|
| Down / Up | Move focus to next / previous visible item |
| Right | Collapsed branch: expand. Expanded branch: focus first child. Leaf: no-op |
| Left | Expanded branch: collapse. Otherwise: focus parent |
| Home / End | Focus first / last visible item |
| Enter | Select focused item (replace) and fire `onActivate` |
| Space | Toggle focused item in the selection set |
| F2 | Start inline rename of the focused item |
| Alt+Up / Alt+Down | One-keystroke sibling move through `requestKeyboardDrop` (when `reorderable`) |
| Shift+Down / Shift+Up | Move focus and extend the selection range |
| Ctrl/Cmd+Click, Ctrl/Cmd+Space | Toggle the item in the selection set |
| Shift+Click | Select the contiguous visible range from the anchor |

### Focus And Announcement

- focus entry: the first selected item, else the first item
- focus exit: native; roving tabindex preserves the last focused item
- live-region or announcement behavior: none; state is conveyed by ARIA on items
- GPUI-native accessibility mapping notes: **not exposed.** gpui 0.2.2 ships no
  public accessibility API (no role/level/selected/checked tree), so the GPUI
  Tree conveys state visually only. This is a runtime limitation, not a design
  choice — see Known Deltas. Svelte emits the full ARIA tree.

### The Twisty Gutter On A Flat Tree

A leaf renders a twisty-sized spacer so its label lines up with branch labels.
That is right whenever something in the tree can expand, and pointless when
nothing can: a Tree used to present a flat list gets an empty column down its
left, aligning labels with a chevron that will never appear.

`collapseTwistyWhenFlat` reclaims it, and the condition is **the whole tree, not
the node** — a single branch anywhere restores the spacer for every row, because
the moment one label needs the gutter they all need it to stay aligned.

It is opt-in rather than automatic. A tree whose nodes load asynchronously would
otherwise shift its rows sideways the first time a branch arrives, and a caller
that knows its data is flat is better placed to accept that than the component
is to guess.

## 7. Layout

### Sizing

- minimum size: `min-width: 0`; rows truncate labels with ellipsis
- maximum size: none; host owns scroll (wrap in `ScrollShell` when needed)
- overflow behavior: labels ellipsis-truncate; the tree itself does not scroll

### Composition

- parent expectations: sidebars, explorer panels, inspector rails
- child expectations: none (self-contained recursive rendering)
- resizing rules: rows stretch to full width so selection / hover spans the row;
  indentation is additive per depth level
- hierarchy guidance: use real nested `children`; never fake depth via labels

## 8. Token Usage

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` | Root | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` (or absent) |
| `data-density` | Root | `"compact"`, `"default"`, `"comfortable"` (or absent) |
| `data-size-role` | Root | `"chrome"`, `"control"`, `"prominent"` |
| `data-branch` | TreeItem | `"true"` when the node is a branch |
| `data-selected` | TreeItem | `"true"` when selected |
| `data-muted` | TreeItem | `"true"` when muted |

### CSS Custom Properties (Internal)

| Property | Default | Purpose |
|----------|---------|---------|
| `--poodle-tree-row-height` | `1.75rem` | Row min-height (size-driven) |
| `--poodle-tree-row-font` | `var(--poodle-typography-label-size)` | Row label font size (size-driven) |
| `--poodle-tree-twisty-size` | `calc(var(--poodle-tree-row-font) * 1.5)` | Twisty box width |
| `--poodle-tree-chevron-size` | `calc(var(--poodle-tree-row-font) * 0.85)` | Chevron glyph size |
| `--poodle-tree-indent` | `1rem` | Width of one depth indent cell (density-driven) |
| `--poodle-tree-row-gap` | `0.25rem` | Gap between twisty, icon, label (density-driven) |
| `--poodle-tree-row-pad-inline` | `0.375rem` | Row leading / trailing inline padding (density-driven) |
| `--poodle-tree-pad-block` | `var(--poodle-space-panel-y)` | The tree's own top / bottom inset |
| `--poodle-tree-pad-inline` | `0.25rem` | The tree's own leading / trailing inset |

### `.tree` (Root)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `min-width` | `0` |
| `padding` | `var(--poodle-tree-pad-block) var(--poodle-tree-pad-inline)` |

#### The Root Inset Is Addressable

Both axes of the root padding are their own variables rather than a literal and
a bare token read. The inset is right when the tree sits inside a bordered box
and wrong when the tree *is* the column: there the inline offset breaks
alignment with everything else in the column, and the block offset pushes the
first row away from whatever sits above it.

A consumer zeroes them per instance:

```css
.pane :global(.poodle-tree) {
  --poodle-tree-pad-block: 0;
  --poodle-tree-pad-inline: 0;
}
```

Reading `--poodle-space-panel-y` directly would have made the block axis look
overridable, but setting that token on the tree leaks into anything inside it
that reads the same token. The defaults are unchanged.

### Size Variants

Row font reuses the shared control font scale; row height is tree-specific (rows
are denser than `SidebarNav`). Density must never alter these.

| Size | Row Height | Row Font |
|------|-----------|----------|
| xs | `1.375rem` | `0.6875rem` |
| sm | `1.5rem` | `0.75rem` |
| md | `1.75rem` | `0.8125rem` |
| lg | `2rem` | `0.875rem` |
| xl | `2.25rem` | `0.9375rem` |

### Density Variants

Density touches only horizontal rhythm — indent, inter-element gap, and row inline
padding. It never changes row height (size-and-density contract).

| Density | Indent | Row Gap | Row Pad Inline |
|---------|--------|---------|----------------|
| compact | `0.75rem` | `0.1875rem` | `0.25rem` |
| default | `1rem` | `0.25rem` | `0.375rem` |
| comfortable | `1.25rem` | `0.375rem` | `0.5rem` |

### `.tree__item`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `min-width` | `0` |

### `.tree__row`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-tree-row-gap)` |
| `width` | `100%` |
| `min-width` | `0` |
| `min-height` | `var(--poodle-tree-row-height)` |
| `padding-inline` | `var(--poodle-tree-row-pad-inline)` |
| `border` | `0` |
| `border-radius` | `calc(var(--poodle-radius-control) - 0.125rem)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-tree-row-font)` |
| `font-weight` | `500` |
| `line-height` | `1.3` |
| `text-align` | `left` |
| `cursor` | `pointer` |
| `transition` | `color, background, box-shadow` via `--poodle-motion-duration-interaction` + `--poodle-motion-easing-standard` |

### `.tree__indent`

| Property | Value |
|----------|-------|
| `flex` | `0 0 auto` |
| `align-self` | `stretch` |
| `width` | `var(--poodle-tree-indent)` |
| `border-left` | `0.0625rem solid transparent` |

When `showGuides`, the indent cell's `border-left` becomes
`color-mix(in srgb, var(--poodle-color-border-subtle) 54%, transparent)`, drawing
the vertical ancestor guide lines.

### `.tree__twisty`

| Property | Value |
|----------|-------|
| `flex` | `0 0 auto` |
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `var(--poodle-tree-twisty-size)` |
| `font-size` | `var(--poodle-tree-chevron-size)` |
| `color` | `var(--poodle-color-text-secondary)` |

The chevron rotates from right (collapsed) to down (expanded). Svelte uses
`Icon name="chevron-right"` rotated 90°; Rust runtimes use the glyphs `▸` / `▾`.

### `.tree__icon`

| Property | Value |
|----------|-------|
| `flex` | `0 0 auto` |
| `display` | `inline-flex` |
| `color` | `var(--poodle-color-text-secondary)` |

### `.tree__label`

| Property | Value |
|----------|-------|
| `flex` | `1 1 auto` |
| `min-width` | `0` |
| `overflow` | `hidden` |
| `white-space` | `nowrap` |
| `text-overflow` | `ellipsis` |

### `.tree__row:hover` (non-disabled)

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 60%, transparent)` |

### `.tree__item[data-selected="true"] > .tree__row`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |
| `font-weight` | `600` |
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent)` |
| `box-shadow` | `inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-accent-base) 20%, transparent)` |

### `.tree__item:focus-visible > .tree__row`

Focus lives on the `treeitem`; the ring is painted on its presentational row child.

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `-0.0625rem` |

### `.tree__item[aria-disabled="true"] > .tree__row`

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |
| `cursor` | `not-allowed` |

### `.tree__checkbox` / `.tree__spinner`

| Part | Property | Value |
|------|----------|-------|
| `.tree__checkbox` | `flex` | `0 0 auto` — hosts a size-`xs` `Checkbox`, before the icon |
| `.tree__spinner` | `flex` / `width` | `0 0 auto` / `var(--poodle-tree-twisty-size)` — spinner in the LoadingRow |
| `.tree__label--muted` | `color` / `font-style` | `var(--poodle-color-text-secondary)` / `italic` — "Loading…" |

### `.tree__rename` (inline rename input)

| Property | Value |
|----------|-------|
| `flex` | `1 1 auto` |
| `padding` | `0 0.25rem` |
| `border` | `0.0625rem solid var(--poodle-color-accent-base)` |
| `border-radius` | `0.1875rem` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `font` | `inherit` (matches row) |
| `:focus-visible box-shadow` | `0 0 0 0.125rem color-mix(in srgb, var(--poodle-color-accent-focusRing) 50%, transparent)` |

### Drop indicator (`.tree__item[data-drop]`)

| Value | Rendering |
|-------|-----------|
| `before` | accent line (`0.125rem`, `var(--poodle-color-accent-base)`) at the row top, aligned to the icon column at the drop depth |
| `after` | accent line at the row bottom, aligned to the icon column at the drop depth |
| `inside` | row fill `color-mix(... accent-base 12%, transparent)` |

GPUI draws the equivalent (top/bottom accent line or inside fill) via an absolute
child; the position comes from pointer Y within the row during `on_drag_move`.

### Light Theme Overrides

None.

## 9. Svelte Notes

- expected substrate: a `Tree.svelte` whose body uses a single self-recursing
  snippet `renderNode(node, depth)` so nesting is internal — no public `TreeItem`
- wrapper strategy: `role="tree"` root with `aria-multiselectable`; roving
  tabindex over visible treeitems (`tabindex=0` on the active item, `-1`
  elsewhere); selection/keyboard handlers live on the `treeitem`, which
  `stopPropagation`s so nested items do not double-fire. Reorder registers the
  `treeitem` as the drag source with the row as its pointer handle, and the
  same `treeitem` as the nested drop target, so Space/Enter keep tree
  selection/activate, terminal focus returns to the `treeitem`, and
  ancestor/descendant rows can share a pointer. Geometry still comes from the
  row. The twisty is marked `data-poodle-no-drag` so expansion is not a drag
  source. Visible rows register as logical keyboard targets; Alt+↑/↓ calls
  `requestKeyboardDrop`. Tree omits `keyboardOrder` so ordinary Space/Enter
  pickup stays off.
- virtualized windows pin the active source row until the session ends; they
  do not page or unmount it mid-drag
- implementation-only details: expansion is uncontrolled-capable via internal
  `$state` seeded from `defaultExpandedValues`; controlled when `expandedValues`
  is non-null. Selection anchor is tracked for Shift range selection
- chevron is `Icon name="chevron-right"` rotated 90° on expand
- known browser-specific deltas: none
- `reorderAuthority` is read live for hover and drop-time revalidation while its
  projected moving set is session-latched. Removing the authority mid-session
  refuses the drop; it never falls through to `onReorder`.

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::tree` → `Tree`
  (+ `TreeSelectionUpdate`, `TreeReorderRequest`, `TreeContextRequest`,
  `TreeDragOver`)
- theme access strategy: `GpuiThemeProvider`, resolving the same semantic tokens
  as the Svelte CSS variable defaults
- interaction model: the component is a stateless `IntoElement`; the owning view
  holds tree state (selection/expansion/focus/anchor/editing/drop) and mutates it
  via `cx.listener` callbacks, then `cx.notify`. Mirrors `Tabs`/`Checkbox`.
- selection: multi-select is **component-owned** — `on_selection_change` emits
  the next `{values, anchor, focused}` from click (Ctrl/Cmd toggle, Shift range),
  Space (toggle), and Shift+Arrow (extend). Host round-trips `selection_anchor`.
- rename: reuses the `EditableLabel` primitive (controlled via `on_rename_change`
  → `editing_text`); F2 starts, Enter commits, Escape cancels
- context menu: right-click via `on_mouse_down(MouseButton::Right)` →
  `on_context_menu`; host renders `ContextMenu` at `anchor_point`
- reorder: the shared drag-and-drop substrate (architecture 011, spec 069).
  Every enabled row registers a `NodeDragSource` and a nested `NodeDropTarget`;
  the GPUI `DragDropController` owns capture, hit testing, deterministic
  deepest-target arbitration, cancellation, and exactly-once cleanup, and the
  band rule (`before` / `inside` / `after`) lives in
  `poodle_render::drag_drop`. Web also walks last-descendant ancestors from
  pointer X so a leftward move un-nests after an open parent. A row dropped onto itself is rejected. A disabled row is not a live destination even when an enabled hover remaps onto it. A
  row's subject kind is scoped to Tree, so a drag from another reorder surface
  sharing the controller is never eligible here. `on_drag_over` and
  `on_reorder` keep their `(dragged, over, DropEdge)` shape: the component
  still never sees a coordinate. The `drag_over` indicator is a top/bottom
  accent line or inside fill.
- Two native gaps are carried to the card that migrates Tree's keyboard route.
  Alt+Up/Down sibling reorder is reported through `on_key` and executed by the
  host rather than running through the shared semantic session the way the web
  route does. And there is no clear or terminal channel in `TreeHandlers`, so a
  host's `drag_value` / `drop_target_value` stay latched after a cancelled
  drag; adding one is a public API decision that card owns.
- chevron uses `▸` / `▾` glyphs; guides are left-bordered indent cells
- known GPUI-native deltas: no accessibility (runtime limit, §6 + Known Deltas);
  no virtual scrolling; transition timing is platform-owned. The paired web
  `reorderAuthority` is not projected onto native: local Node commits are
  synchronous, intent presentation does not carry a rewritten destination,
  and the Node subject has no durable multi-row session value. Native keeps
  `on_reorder`; closing those generic substrate gaps is separate work.

## 10a. Jetstream Notes

- `Tree::from_spec(spec, theme).on_select(...).on_toggle_expand(...).on_check(...)`,
  matching the GPUI target's handler names, each carrying the node's value.
- A row has three click targets and they are three events. The twisty and the
  checkbox sit *inside* the row, so each takes a handler of its own — inert when
  unwired — because clicks bubble to the nearest clickable ancestor and an
  unwired chevron would otherwise select the row it was expanding.
- `on_check` names the node rather than asserting a next state: the host holds
  the check state and the cascade rules, and a component that guessed would be
  wrong for any tree where checking a parent checks its children.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] branch rule (`isBranch || children`) matches
- [x] selection set semantics match (replace / toggle / range) — Svelte in-component,
      GPUI via `on_selection_change`; Jetstream click-select only (no modifiers in
      its `ClickEvent` — known runtime gap)
- [ ] expansion controlled + uncontrolled resolution matches
- [ ] `aria-level`, `aria-selected`, `aria-expanded`, `aria-disabled` exposure match
- [x] keyboard model matches (arrows, Home/End, Enter/Space) in all 3 targets;
      Shift/Ctrl range + toggle remain Svelte-only pending Phase 2+
- [ ] disabled nodes suppress select + expand
- [ ] event names and payloads match

### Tier 2: Visual Parity

- [ ] selected fill + inset ring + text weight match
- [ ] hover background + color match
- [ ] indentation per depth + guide lines match
- [ ] chevron collapsed / expanded states match
- [ ] icon slot reservation matches
- [ ] size variant scaling matches all 5 sizes
- [ ] density variant scaling matches all 3 densities (horizontal only)
- [ ] focus ring matches

### Tier 3: Implementation Freedom

- [ ] transition timing is platform-owned
- [ ] visible-row flattening / recursion internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Jetstream has no `onActivate` | activation is a double-click or Enter on the web; the runtime reports single clicks with no click count, and raises no key events | accepted, tracked | g12.017 |
| Jetstream has no rename, reorder or context-menu events | inline rename needs a text field and key handling, reorder needs drag, and the context menu needs a right-click — none of which this target raises yet | accepted, tracked | g12.017 |
| Jetstream has no `onLoadChildren` | lazy loading is driven by expansion, which the host already hears through `on_toggle_expand` | accepted (by design) | none |
| GPUI exposes no accessibility | gpui 0.2.2 has no public a11y API (no role/level/selected tree) | accepted (forced) | revisit when gpui ships accesskit support |
| Jetstream exposes no ARIA | immediate-mode runtime has no a11y tree | accepted (forced) | — |
| Virtual scroll is Svelte-only | GPUI/Jetstream have no row-windowing primitive; they render all visible rows | accepted | revisit if a Rust runtime gains windowing |
| Rust interaction is host-driven | GPUI is stateless + driven by the owning view; Jetstream renders from spec with the app loop owning state/events | accepted | inherent to the runtimes |
| Selected inset ring is Svelte-only | A layout-affecting border would jitter rows in the immediate/retained runtimes | accepted | — |

## 13. Approval And Adoption Notes

- contract status: `implemented`
- approvers: pending
- downstream adopters: Loophole explorer, Underlay outline views
- future follow-up: drag-reorder, inline rename, async-lazy children, checkbox
  cascade selection

## 14. Specimen Definitions

### File Explorer (default)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| File tree | Nested `src/` folder tree, `defaultExpandedValues=["src","src/components"]`, `selectedValues=["src/components/Button.svelte"]`, icons on, guides on | Expanded folders with down-chevrons, one selected file with accent fill, vertical guide lines, file/folder icons |

### Selection Modes

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Multi-select | Same tree with `selectedValues` holding two sibling files | Two rows show accent fill simultaneously |

### Guides Off / No Icons

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Plain | `showGuides=false`, `showIcons=false` | Indentation preserved, no guide lines, no icon slot, labels left-aligned to twisty |

### Disabled Node

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With disabled | One node `isDisabled: true` | Reduced opacity, non-interactive |

### Sizes And Densities

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Sizes | Same tree across xs–xl | Row height + font scale; indentation unchanged |
| Densities | Same tree across compact / default / comfortable | Indent, gap, inline padding scale; row height unchanged |
