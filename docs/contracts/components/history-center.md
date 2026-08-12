# HistoryCenter

Status: active contract
Updated: 2026-08-11

## 1. Purpose

- Component name: `HistoryCenter`
- Layer: composite
- Summary: the history counterpart to `MessageCenter` — a compact titlebar-grade trigger cluster (undo / list / redo) plus a popover rendering the stitched history tree: the spine's entries at depth 0 and each fork branch's run (caption + entries) at its true position, with light git-graph lanes and checkpoint pins when the host supplies them
- Composes: `IconButton`, `Popover`, `Button`, `Icon`, `Spinner`, `EmptyState`
- In scope: undo/redo commands, popover open state, tree rendering (spine + fork runs with lanes), current-position marker, entry navigation, inline branch rename, checkpoint pins, transient rejection display, loading/failed status
- Out of scope: history storage, authority logic, protocol validation, checkpoint creation, undo/redo semantics, persistence, and any Longhorn knowledge

`HistoryCenter` is **authority-agnostic**. Data arrives through props; commands
leave through callbacks. It never validates protocol rules, never assumes the
supplied history is complete, and never decides what undo or redo means — the
host owns all of that. `MessageCenter` is the precedent: `items` in,
`onItemSelect`/`onRemove`/`onMarkAllRead` out.

There is no Longhorn dependency, and none is possible: the dependency runs
Longhorn → Poodle (`longhorn-poodle-svelte` imports Poodle, not the reverse).
`HistoryCenter` does not import Longhorn types, does not reference
`ForkHistorySession`, and does not compose `ForkHistoryPanel`. The Longhorn
bridge maps session state → props and receives commands → session calls. The
prop shapes below are structurally compatible with the authority's
`ForkEntryRecord`/`ForkBranchRecord` so that bridge is a plain mapping.

## 2. Anatomy

```text
HistoryCenter
├── Trigger cluster
│   ├── IconButton undo (icon `undo`; enabled from canUndo, busy)
│   ├── IconButton list (opens the popover)
│   └── IconButton redo (icon `redo`; enabled from canRedo, busy)
└── Popover
    └── Surface
        ├── Header: title + totals
        ├── Rejection notice (transient, dismissible)
        ├── Status row (loading spinner / failed message)
        ├── History list (the stitched tree)
        │   ├── Entry row
        │   │   ├── Lane gutter (ancestor columns + the row's own lane)
        │   │   ├── Checkpoint pin (when the entry is a checkpoint)
        │   │   ├── Label + meta (position, group)
        │   │   └── Current-position marker
        │   └── Caption row (a fork run's label, at the run's depth)
        │       ├── Name (or id) + run entry count + derived relative time + current badge
        │       └── Inline rename
        ├── Load-more row (entries)
        └── Load-more row (branches)
        └── EmptyState
```

The popover content is component-owned; `center`-style snippets are not used
here.

## 3. Props And Inputs

### Data Shapes

```ts
type HistoryEntryPosition = "past" | "current" | "future";

type HistoryEntry = {
  id: string;
  label: string;
  position: HistoryEntryPosition;
  checkpoint?: boolean;      // renders as a named pin
  groupId?: string | null;
  recordedAtMs?: number;     // authority-supplied; absent → render nothing.
                             // Never invented client-side (ruling D2).
};

type HistoryBranch = {
  id: string;
  name: string | null;       // auto-named by the authority; null → show id
  annotation?: string | null;
  headEntryId?: string;      // branch head; absent when the branch has no entries
  divergedAfterEntryId?: string; // entry on the current branch after which this
                             // branch diverged; absent at root. Carried for
                             // host/bridge use and structural compatibility
                             // with the authority's `divergence_entry_id` —
                             // the stitcher never attaches to it (see row
                             // model).
                             // No `recordedAtMs` here, deliberately: the agreed
                             // upstream shape puts `recorded_at` on entry
                             // metadata only, with no branch-level equivalent
                             // proposed. A run caption takes its relative time
                             // from its own run's most recent entry (ruling D2).
  entryCount?: number;
  current?: boolean;
  pinned?: boolean;
};
```

### Public Props

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `branches` | `HistoryBranch[] \| null` | `null` | Branch records in supplied order. `null` disables the tree: no rows render and every row event is inert. |
| `paths` | `Record<string, HistoryEntry[]> \| null` | `null` | Per-branch entry path (`branchId` → root-to-head entries). `null` when `branches` is `null`. |
| `totalEntries` | `number` | `0` | Total entry count; shown in the header. |
| `totalBranches` | `number` | `0` | Total branch count; shown in the header when branches are supplied. |
| `hasMoreEntries` | `boolean` | `false` | Shows the entries load-more action. |
| `hasMoreBranches` | `boolean` | `false` | Shows the branches load-more action. |
| `canUndo` | `boolean` | `false` | Enables the undo trigger. |
| `canRedo` | `boolean` | `false` | Enables the redo trigger. |
| `busy` | `boolean` | `false` | Disables both undo and redo while an authority operation runs. |
| `status` | `"idle" \| "loading" \| "failed"` | `"idle"` | Source status; loading shows a spinner row, failed shows `statusMessage`. |
| `statusMessage` | `string \| null` | `null` | Copy for the failed status row. |
| `rejection` | `string \| null` | `null` | Transient inline notice; never rendered silently, never sticky. |
| `maxBranchNameBytes` | `number` | `256` | Client-side affordance only — caps inline rename input length. The component enforces no protocol rule. |
| `open` | `boolean \| null` | `null` | Controlled open state; Svelte supports binding. |
| `defaultOpen` | `boolean` | `false` | Initial uncontrolled state. |
| `placement` | `OverlayPlacement` | `"bottom-end"` | Popover placement hint. |
| `undoLabel` | `string` | `"Undo"` | Undo trigger accessible name and tooltip. |
| `redoLabel` | `string` | `"Redo"` | Redo trigger accessible name and tooltip. |
| `listLabel` | `string` | `"History"` | List trigger accessible name, tooltip, and list region label. |
| `title` | `string` | `"History"` | Surface heading and default accessible label. |
| `emptyMessage` | `string` | `"No history entries yet."` | Empty-state copy. |
| `ariaLabel` | `string \| null` | `null` | Overrides the surface label. |
| `size` | `ControlSize \| null` | `null` | Explicit semantic size. |
| `sizeRole` | `SemanticControlSizeRole` | `"chrome"` | Inherited-size role. |
| `density` | `ControlDensity \| null` | `null` | Explicit density. |
| `onUndo` | `() => void` | `null` | Undo command. |
| `onRedo` | `() => void` | `null` | Redo command. |
| `onOpenChange` | `(open: boolean) => void` | `null` | Open-state request. |
| `onNavigateEntry` | `(branchId: string, entryId: string) => void` | `null` | Entry row activation — always the entry actually clicked, on the branch that owns its run. Replaces v1's `onSelectEntry`/`onCheckout` (ruling D1). |
| `onRenameBranch` | `(branchId: string, name: string) => void` | `null` | Committed inline branch rename. |
| `onLoadMoreEntries` | `(offset: number) => void` | `null` | Requests the next entries page; `offset` is the number of entry rows currently stitched. |
| `onLoadMoreBranches` | `(offset: number) => void` | `null` | Requests the next branches page; `offset` is the supplied branch count. |

### Command-Only Callbacks

Undo/redo/navigate/rename/load-more are **commands out** — the
component emits the callback on user activation and does nothing else. It
never invokes a callback speculatively: no auto-undo, no implicit checkout on
open, no load-more ahead of the user's click. The host owns what undo does and
whether a rejected command is retried.

### Controlled And Uncontrolled

`open` follows the `MessageCenter` pattern: `open = null` means uncontrolled
with `defaultOpen` seeding the first render; a non-null `open` is controlled
and updates flow back through `onOpenChange`. Everything else is caller-owned
data — the component keeps no second store of entries, branches, or status.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | — | Trigger cluster renders undo/list/redo; undo/redo disabled without `canUndo`/`canRedo` |
| busy | `busy` | Undo and redo disabled and visually inert |
| open | list trigger click / programmatic | Popover anchored to the trigger, focus moved into the surface |
| tree | `branches` supplied | The stitched tree renders: spine entries at depth 0, fork runs (caption + entries) at their true depth, lanes drawn from row lane metadata |
| linear | a single current branch | The tree is the spine only — one trunk lane, no captions, no elbows |
| forked | `branches` with more than one branch | Runs attach at their true position; elbows and pass-through lanes draw the fork graph |
| empty | `branches` null or zero stitched rows | No tree; empty state (or the status row when loading/failed) |
| loading | `status === "loading"` | Spinner status row |
| failed | `status === "failed"` | `statusMessage` status row |
| rejected | `rejection` non-null | Transient inline notice; dismissible |

### Component States

The tree is **presence-driven**: when `branches` is not supplied the machine
has no rows and the popover shows the empty state — absence is the signal,
matching `AppHeader`'s `center` region. There is no `mode` prop. A single
current branch degrades the tree to the spine alone (linear); checkpoint pins
render on checkpoint entries wherever they sit.

Behavior classification: `machine-backed`.

### Behavior Machine

Contract: `packages/core/src/history-center.ts`. The machine owns popover open
state, linear keyboard traversal over the stitched history tree, and transient
rejection display. Branch records and per-branch entry paths are part of
context, supplied by the caller on every transition; the tree is re-stitched
per transition. Undo/redo/load-more are plain button commands the adapter
forwards directly — they carry no machine state and are never invoked
speculatively.

v2 renders the **actual tree** (card `023`): every entry in the fork graph,
exactly once, at its true position, in topological order. A fork run's
content is its entries — branch names are captions, not rows pinned to a
divergence id (the v1 model, where divergence ids computed relative to the
current branch collapsed genuinely different forks onto one entry).

#### Context

| Field | Type | Initial | Controllable | Meaning |
|-------|------|---------|--------------|---------|
| `branches` | `HistoryBranch[] \| null` | `null` | no (host-supplied) | Branch records in supplied order. `null` disables the tree: the machine has no rows and every row event is inert. |
| `paths` | `Record<string, HistoryEntry[]> \| null` | `null` | no (host-supplied) | Per-branch entry path (`branchId` → root-to-head entries). `null` when `branches` is null. |
| `focusIndex` | `number` | `0` | no | Roving focus index over the stitched row list. |
| `rejection` | `string \| null` | `null` | no | Currently displayed rejection message. |

#### Row Model

`historyCenterRows(branches, paths)` is the **stitcher** — a pure, exported
function (ruling D5: it never fetches; it knows nothing of Longhorn, ports,
or paging). It returns every row of the history tree in topological order;
each row's `index` equals its position in the returned array and keyboard
traversal is linear in visual order. The spine is the branch marked `current`
(fallback: the first supplied branch); its path renders at depth 0. Each
other branch's run — a caption row plus its unique entries — attaches
immediately after the deepest entry its path shares with the already-stitched
tree, in supplied order (ruling D4: order is supplied, not invented; the
outer run of a fork-off-fork must precede the inner one, which is the
authority's natural listing order). Shared prefixes render once (dedupe by
`entryId`); a run never re-emits entries that belong to the spine or an outer
run. The authority's `divergedAfterEntryId` is carried on the record but
never used for attachment — attachment comes from the paths, which is exactly
what fixes the v1 collapse.

```text
entry e1                    ← spine, depth 0
entry e2                    ← last shared entry; the run attaches right after it
  caption feature/audio     ← run label, focusable for rename, never navigates
  entry a1                  ← run entry, depth 1
    caption fork-of-fork    ← inner run, depth 2, attaches to the outer run
    entry b1
  entry a2                  ← outer run continues past the inner run
entry e3                    ← spine continues
```

Rows:

- `{ kind: "entry"; index; branchId; entry; depth; lane }` — an entry of the
  tree, owned by the branch whose run it sits in (the spine's rows carry the
  current branch id). Activation emits `emitNavigateEntry(branchId, entryId)`
  with the clicked row's **own** branch and entry — never an ancestor or a
  divergence entry belonging to another branch.
- `{ kind: "caption"; index; branch; depth }` — a fork run's label, at the
  run's depth before its first entry. Focusable for rename; activation never
  navigates.

Depth saturates at `HISTORY_TREE_DEPTH_CAP` (3) — ruling D3: past depth 3 a
chain keeps rendering flat at depth 3 rather than indenting further. Only
indentation saturates: the row keeps its true `branchId` and lane structure,
so navigation is unaffected.

Lane metadata (`HistoryRowLane`, on every entry row) is sufficient for a
renderer to draw the git-graph lanes without re-deriving structure:
`branchId`, `parentBranchId` (the run this run attaches to; null for the
spine and root-attached runs), `start` (the run's first entry row — the
elbow, except depth 0 where the spine's run is the trunk), `continue` (the
lane passes through this row), `end` (the run's last entry row). A
single-entry run is both `start` and `end`. Card `024` draws the graph from
this; lane columns, crossing lines, and elbow geometry are derived there, not
here.

A branch with **no unique entries** — an empty path, or a path fully shared
with the already-stitched tree — is **omitted** (no caption, no entry rows):
attachment is defined by path prefix sharing, so an empty path has no
position, and attaching to the divergence id would reintroduce the v1
collapse bug. (This is the recorded empty-branch-head decision; a page for
the branch can arrive later and the run stitches in.)

`historyCenterRowCount(branches, paths)` returns the stitched row count.

#### States

| State | Description |
|-------|-------------|
| `closed` | Popover closed; trigger cluster interactive, undo/redo per `canUndo`/`canRedo`/`busy`. |
| `open` | Popover open; list navigable and rejection events live. |

#### Events

| Event | Payload | Source |
|-------|---------|--------|
| `TOGGLE` / `OPEN` / `CLOSE` | — | trigger / programmatic / dismiss |
| `FOCUS_MOVE` | `direction: "next" \| "prev" \| "first" \| "last"` | keyboard |
| `ACTIVATE_ROW` | `index?` (default: focused) | keyboard / pointer |
| `RENAME` | `branchId`, `name` | rename input commit |
| `SHOW_REJECTION` | `message` | adapter (rejection prop change) |
| `DISMISS_REJECTION` | — | notice dismiss button |

The v1 expansion events (`TOGGLE_BRANCHES`, `EXPAND_BRANCHES`,
`COLLAPSE_BRANCHES`) and `CHECKOUT` are retired (ruling D1); the tree always
renders fully and no fork needs expanding.

#### Transitions

| State | Event | Guard | Target | Actions / Effects |
|-------|-------|-------|--------|-------------------|
| closed | `TOGGLE` / `OPEN` | — | open | `emitOpenChange(true)` |
| open | `TOGGLE` / `CLOSE` | — | closed | `emitOpenChange(false)` |
| open | `OPEN` / closed | — | stay | — |
| open | `FOCUS_MOVE` | row count > 0 | open | move `focusIndex` (wrap; first/last land on boundaries); `focusRow(index)` |
| open | `ACTIVATE_ROW` | row exists at index | open | entry row → `emitNavigateEntry(branchId, entryId)` for the clicked row's own branch and entry; caption row → focus syncs, no effect |
| any | `RENAME` | — | stay | `emitRenameBranch(branchId, name)` |
| any | `SHOW_REJECTION` | message differs from displayed | stay | set `rejection` |
| any | `DISMISS_REJECTION` | rejection displayed | stay | clear `rejection` |
| closed | `FOCUS_MOVE` / `ACTIVATE_ROW` | — | stay | — |

Guards are pure predicates over context + payload. Out-of-range activation,
empty row lists (including `branches: null`), and closed-state row events are
all inert (stay with no effects). `emitNavigateEntry` always reports the
entry actually clicked — the clicked row carries its own `branchId` and
`entry`, so an ancestor or another branch's divergence entry can never be
reported.

#### Effects

| Effect | What It Does | Cleanup |
|--------|--------------|---------|
| `emitOpenChange { open }` | Adapter syncs the bindable `open` and calls `onOpenChange`. | None (host-owned). |
| `focusRow { index }` | Adapter moves roving focus to the row element and scrolls it into view. | Overridden by the next `focusRow`. |
| `emitNavigateEntry { branchId, entryId }` | Adapter calls the host's navigate handler with the clicked row's branch and entry. | None (host-owned). |
| `emitRenameBranch { branchId, name }` | Adapter calls `onRenameBranch(branchId, name)`. | None (host-owned). |

The v1 `emitSelectEntry` and `emitCheckout` effects are gone: `emitSelectEntry`
collapses into `emitNavigateEntry` (every entry row carries the branch that
owns its run) and `emitCheckout` is retired with the expansion model (ruling
D1).

Open/close focus management (focus the surface on open, restore trigger focus
on close, dismiss-on-outside, focus trap) is adapter-owned: the composed
`Popover` runs its own machine for anchor, dismissal, and initial focus, and
the surface applies `trapFocusKeydown` while open.

#### Part Attribute Output

| Part | Attribute | Value |
|------|-----------|-------|
| root | `data-scope` / `data-part` | `history-center` / `root` |
| trigger cluster | `data-part` | `trigger` |
| undo/redo trigger | `disabled` | `!canUndo \|\| busy` / `!canRedo \|\| busy` |
| list trigger | `aria-expanded` | `true` / `false` from open state |
| surface | `data-part` / `data-state` | `surface` / `open` \| `closed` |
| list | `data-part` / `aria-label` | `list` / `listLabel` |
| row | `data-row-index` / `data-depth` / `aria-level` | position in the stitched array / saturated depth (0-based) / `depth + 1` |
| entry row | `data-part` / `data-position` / `data-checkpoint` | `entry` / `past\|current\|future` / presence |
| entry lane cells | `data-lane` | `trunk` (depth 0) \| `elbow` (run's first entry) \| `single` (one-entry run) \| `continue` \| `end` \| `ancestor` (pass-through columns) |
| caption row | `data-part` / `data-current` / lane cell | `caption` / `true\|false` / the row's own lane column is an empty `data-lane="caption"` cell |
| rejection | `data-part` / `role` | `rejection` / `status` |

The v1 fork-indicator surface (`data-fork`, the expand/collapse affordance,
per-entry `aria-expanded`) and the branch-row surface are retired with the
expansion model (ruling D1). The lane gutter is decorative
(`aria-hidden`); the row's `data-lane` attributes carry the lane shape for
styling and tests, and depth reaches assistive tech through `aria-level`
(see §6).

#### Machinery Dependencies

Focus trap (`trapFocusKeydown`), roving row navigation (machine-owned
`focusIndex`), dismissable layer + anchor positioning + initial focus (via the
composed `Popover`), `createInstanceId` for surface ids.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onUndo` | undo trigger click while `canUndo && !busy` | — | Plain command; host decides what undo means. |
| `onRedo` | redo trigger click while `canRedo && !busy` | — | Plain command; host decides what redo means. |
| `onOpenChange` | open state actually changes | `boolean` | Never emitted speculatively. |
| `onNavigateEntry` | entry row activation (pointer or keyboard) | `branchId`, `entryId` | Always the entry actually clicked, on the branch that owns its run — never an ancestor or another branch's divergence entry. Captions never fire it. Replaces v1's `onSelectEntry`/`onCheckout` (ruling D1). |
| `onRenameBranch` | rename input commit (Enter or blur) | `branchId`, `name` | Escape cancels without emitting. |
| `onLoadMoreEntries` | load-more click | `offset` = stitched entry-row count | Only shown when `hasMoreEntries`. |
| `onLoadMoreBranches` | load-more click | `offset` = supplied `branches.length` | Only shown when `hasMoreBranches` and branches supplied. |

## 6. Accessibility

### Semantics

- Trigger cluster: three native `IconButton`s with distinct accessible names
  (`undoLabel`, `listLabel`, `redoLabel`); the list trigger carries
  `aria-expanded`.
- The popover surface is a labelled dialog (`ariaLabel ?? title`) and receives
  initial focus through `Popover`.
- The list is a labelled list region (`ul` with `listLabel`); every row is an
  `li`. Entry rows are native buttons; caption rows expose their rename
  button as the roving-focus target and never navigate.
- **Depth reaches assistive tech through `aria-level`** on every row
  (1-based, `depth + 1`), with `data-depth` carrying the 0-based saturated
  depth for styling and tests. A `tree` role was considered and rejected: the
  machine's traversal is linear over the flat stitched array in visual order,
  so native list semantics plus `aria-level` describe the rows more honestly
  than a tree whose children are not DOM-nested.
- The lane gutter is decorative (`aria-hidden`); the row content carries the
  meaning.
- The checkpoint pin is decorative (`aria-hidden`); the entry label carries
  the meaning.
- The rejection notice is `role="status"` (polite live region) — never silent,
  never announced as an alert.
- Loading and failed status rows are `role="status"`.
- No interactive element nests inside another interactive element.

### Keyboard

| Key | Behavior |
|-----|----------|
| ArrowDown / ArrowUp | Move roving focus to the next/previous row, wrapping at the ends. |
| Home / End | Move focus to the first/last row. |
| Enter / Space | Activate the focused row: entry → `onNavigateEntry(branchId, entryId)`; caption → focus syncs, nothing navigates (the caption's rename button keeps its native activation and opens inline rename). |
| Enter / Escape | In the rename input: commit (`onRenameBranch`) / cancel. After commit or cancel, focus returns to the caption's rename button. |
| Tab / Shift+Tab | Trapped within the open surface (wraps first↔last focusable). |
| Escape | Close through `Popover`; focus returns to the trigger. |

### Focus And Announcement

- focus entry: on open, `Popover` focuses the surface content (first focusable).
- focus exit: on close, `Popover` restores trigger focus.
- live-region behavior: rejection and status notices announce politely; the
  archive list itself is not a live region.
- GPUI-native accessibility mapping notes: see `020` (native parity).

## 7. Layout

### Sizing

- Trigger cluster: three icon-sized buttons in a row, gap 0.125rem; titlebar
  space is premium, nothing else is added.
- Surface: sized by the popover, `min(28rem, calc(100vw - 2rem))` to
  `min(38rem, calc(100vw - 2rem))`. The panel root sets no width of its own —
  the surface owns it and adds its own horizontal padding, so a width here
  would overflow the rounded edge.
- Text: the panel root sets `typography-label-size` as its base, so rows do
  not inherit the document default. The header title is `body-size`; entry
  meta steps down to `0.6875rem`.
- List: bounded `max-height: min(28rem, 60vh)`, scrolls internally with
  `overscroll-behavior: contain`. No virtualisation — paging only (Tree's
  virtual scroll is Svelte-only precedent and would break the native port).

### Lane Geometry

- Every row carries a lane gutter of one column per depth level plus the
  row's own lane column; the content inset grows one column per depth level
  and saturates with the row's depth cap (`HISTORY_TREE_DEPTH_CAP`).
- Vertical segments sit at column centres; the elbow into a run is a
  horizontal from the parent column centre into the run's own lane column,
  then the lane turns down. A single-entry run is the elbow plus a short
  stub; the spine's lane is the trunk; ancestor lanes pass through nested
  rows vertically.
- The gutter is decorative and `aria-hidden`; only lane colour and thickness
  are recipe-themable (§8) — column width is an internal metric.

### Composition

- parent expectations: placed in a titlebar/header cluster alongside other
  icon triggers; the trigger cluster is inline.
- child expectations: rows are full-width list rows; entry content and
  caption content carry the padding; the content inset grows with depth via
  the lane gutter.
- resizing rules: narrow viewports shrink the surface width to viewport minus
  gutter.

## 8. Token Usage

Semantic roles by default; recipes are the override surface (architecture
007). Public hooks:

| Part | Recipe Hook | Fallback Token |
|------|-------------|----------------|
| entry row | `--poodle-recipe-history-center-item-fill` | transparent |
| entry row (current) | `--poodle-recipe-history-center-current-fill` | accent 7% mix |
| entry hover | `--poodle-recipe-history-center-item-hover-fill` | surface 72% mix |
| checkpoint pin | `--poodle-recipe-history-center-checkpoint-fill` | `--poodle-color-accent-base` |
| position marker | `--poodle-recipe-history-center-current-marker` | `--poodle-color-accent-base` |
| lane | `--poodle-recipe-history-center-lane-color` | text-primary 25% mix |
| lane | `--poodle-recipe-history-center-lane-thickness` | `0.0625rem` |
| current badge fill | `--poodle-recipe-history-center-branch-current-badge-fill` | accent 16% mix |
| current badge text | `--poodle-recipe-history-center-branch-current-badge-text` | `--poodle-color-accent-base` |
| rejection border | `--poodle-recipe-history-center-rejection-border` | danger 45% mix |
| rejection fill | `--poodle-recipe-history-center-rejection-fill` | danger 10% mix |
| rejection text | `--poodle-recipe-history-center-rejection-text` | `--poodle-color-text-primary` |

The v1 fork-indicator and branch-row hooks are retired with the expansion
model (ruling D1); the current-badge hooks survive on caption rows.

Metric variables (widths, gaps, padding, font sizes) are internal and not part
of the recipe contract.

## 9. Svelte Notes

- expected substrate: `HistoryCenter.svelte` composing `IconButton`,
  `Popover`, `Button`, `Icon`, `Spinner`; the machine runs through a `send()`
  channel exactly like `Popover`/`MessageCenter` do.
- wrapper strategy: one `poodle-history-center-popover` root; the trigger
  cluster is the `Popover` trigger snippet; the surface is component-owned.
- implementation-only details: `open` is `$bindable`; the rejection prop diff
  is a `$effect` that sends `SHOW_REJECTION` on new messages; rows come from
  `historyCenterRows(branches, paths)` and the lane cells are `span`s derived
  from each row's `lane` metadata; the caption's relative time is derived from
  supplied `recordedAtMs` values only — there is no clock (ruling D2).
- known browser-specific deltas: none.

## 10. GPUI Notes

- Native parity is card `020` and is required by the runtime parity rule —
  this card delivers the web reference only.
- expected crate/module surface: `HistoryCenterSpec` plus the shared
  `poodle-render` node tree; open/expansion/rename/rejection state remains
  host-owned per `MessageCenter` precedent.
- theme access strategy: recipe hooks map to spec fields / token overrides.
- known GPUI-native deltas: documented in `020`.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [x] semantic inputs have the same meaning (fixed prop surface)
- [x] state transitions match (single headless machine)
- [x] event timing and payload meaning match
- [x] accessibility rules and keyboard behavior match
- [x] accessible name, role, state, and value exposure match
- [x] focus order and restoration behavior match when relevant

### Tier 2: Visual Parity

- [x] token roles match
- [x] spacing and sizing match within platform limits
- [x] overall proportions and hierarchy match

### Tier 3: Implementation Freedom

- [x] implementation-only differences are documented
- [x] no implementation detail leaks into the public contract

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Checkpoint creation is deferred | Ruling 6: checkpoints render as pins in v1; creating one is not in v1 | maintainer ruling | `020` parity card re-evaluates |
| Protocol constants not imported | `MAXIMUM_FORK_BRANCH_NAME_BYTES`/`MAXIMUM_FORK_PROJECTION_PAGE_SIZE` live in the Longhorn crate; the component enforces no protocol rule, `maxBranchNameBytes` is a client-side affordance only | maintainer ruling | bridge validates |
| Rejection dismissal is component-local | The fixed surface has no rejection-dismiss callback; the notice is transient inline UI | this contract | revisit if hosts need dismissal control |
| No virtualisation | Out of scope: paging only; Tree's virtual scroll is Svelte-only and would break the native port | card scope | `020` |
| Native runtimes | Out of scope: this card is the web reference; native parity is `020` | card scope | `020` |

## 13. Approval And Adoption Notes

- contract status: `implemented` (Svelte + React web reference; native pending `020`)
- approvers: Poodle core (card `019`)
- downstream adopters: Longhorn bridge (session → props mapping), consumer
  hosts that own undo/redo semantics
- future follow-up: `020` HistoryCenter native parity (required)
