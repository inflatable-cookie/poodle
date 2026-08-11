# HistoryCenter

Status: active contract
Updated: 2026-08-11

## 1. Purpose

- Component name: `HistoryCenter`
- Layer: composite
- Summary: the history counterpart to `MessageCenter` — a compact titlebar-grade trigger cluster (undo / list / redo) plus a popover listing history entries, with fork branches and checkpoint pins when the host supplies them
- Composes: `IconButton`, `Popover`, `Button`, `Icon`, `Spinner`, `EmptyState`
- In scope: undo/redo commands, popover open state, paged entry list, current-position marker, entry selection, fork-point expansion, branch checkout, inline branch rename, checkpoint pins, transient rejection display, loading/failed status
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
        ├── History list
        │   ├── Entry row
        │   │   ├── Checkpoint pin (when branches supplied)
        │   │   ├── Label + meta (position, group)
        │   │   ├── Current-position marker
        │   │   └── Fork indicator (when branchCount > 1 and branches supplied)
        │   ├── Branch row (under an expanded fork)
        │   │   ├── Name (or id) + entry count + current badge
        │   │   └── Inline rename
        │   ├── Load-more row (entries)
        │   └── Load-more row (branches)
        └── EmptyState
```

The popover content is component-owned; `center`-style snippets are not used
here.

## 3. Props And Inputs

> Card `023` note: the **record types** below are updated by this card
> (v2 tree). The **Public Props table and callback surface** — including
> `onSelectEntry`/`onCheckout` being rebound as a single navigate callback —
> are the rendering layer's rebind and are owned by card `024`.

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
  recordedAtMs?: number;     // authority-supplied; never invented client-side (ruling D2)
  entryCount?: number;
  current?: boolean;
  pinned?: boolean;
};
```

### Public Props

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `entries` | `HistoryEntry[]` | `[]` | Caller-owned page of history entries. |
| `totalEntries` | `number` | `0` | Total entry count; shown in the header. |
| `hasMoreEntries` | `boolean` | `false` | Shows the entries load-more action. |
| `branches` | `HistoryBranch[] \| null` | `null` | Fork branch rows. `null` hides **all** branch and checkpoint UI. |
| `totalBranches` | `number` | `0` | Total branch count; shown in the header when branches are supplied. |
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
| `onSelectEntry` | `(id: string) => void` | `null` | Entry row activation. |
| `onCheckout` | `(branchId: string, entryId: string) => void` | `null` | Branch row activation; carries the fork context. |
| `onRenameBranch` | `(branchId: string, name: string) => void` | `null` | Committed inline branch rename. |
| `onLoadMoreEntries` | `(offset: number) => void` | `null` | Requests the next entries page; `offset` is the supplied count. |
| `onLoadMoreBranches` | `(offset: number) => void` | `null` | Requests the next branches page; `offset` is the supplied count. |

### Command-Only Callbacks

Undo/redo/checkout/rename/load-more/select are **commands out** — the
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

> Card `023` note: the **visual and component states below describe the v1
> rendering** (fork-point expansion, presence-driven plain list). Card `024`
> owns their rebind to the v2 tree; the Behavior Machine subsection is the
> authoritative machine contract and is current.

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | — | Trigger cluster renders undo/list/redo; undo/redo disabled without `canUndo`/`canRedo` |
| busy | `busy` | Undo and redo disabled and visually inert |
| open | list trigger click / programmatic | Popover anchored to the trigger, focus moved into the surface |
| linear | `branches` not supplied | Plain list: no fork indicators, no branch rows, no checkpoint pins |
| forked | `branches` supplied | Fork points expandable; checkpoint pins render; branch rows under expanded forks |
| loading | `status === "loading"` | Spinner status row |
| failed | `status === "failed"` | `statusMessage` status row |
| rejected | `rejection` non-null | Transient inline notice; dismissible |

### Component States

The open/linear-vs-forked split is deliberate: **linear degradation is
presence-driven** — when `branches` is not supplied, all branch and checkpoint
UI is hidden and the popover is a plain list. There is no `mode` prop; absence
is the signal, matching `AppHeader`'s `center` region.

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
| entry row | `data-part` / `data-position` / `data-checkpoint` | `entry` / `past\|current\|future` / presence |
| caption row | `data-part` / `data-current` | `caption` / `true\|false` |
| rejection | `data-part` / `role` | `rejection` / `status` |

The v1 fork-indicator surface (`data-fork`, the expand/collapse affordance,
per-entry `aria-expanded`) is retired with the expansion model. The full tree
part surface — lane rendering, depth and branch exposure, caption placement —
is card `024`'s to finalize.

#### Machinery Dependencies

Focus trap (`trapFocusKeydown`), roving row navigation (machine-owned
`focusIndex`), dismissable layer + anchor positioning + initial focus (via the
composed `Popover`), `createInstanceId` for surface ids.

## 5. Events

> Card `023` note: `onSelectEntry` and `onCheckout` are retired with the v1
> machine (see §4 Effects). Card `024` rebinds them as a single navigate
> callback carrying `(branchId, entryId)` per `emitNavigateEntry`. The
> remaining callback rows below are current.

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onUndo` | undo trigger click while `canUndo && !busy` | — | Plain command; host decides what undo means. |
| `onRedo` | redo trigger click while `canRedo && !busy` | — | Plain command; host decides what redo means. |
| `onOpenChange` | open state actually changes | `boolean` | Never emitted speculatively. |
| `onSelectEntry` | entry row activation | `id` | Covers past, current, and future entries. |
| `onCheckout` | branch row activation | `branchId`, `entryId` | `entryId` anchors the fork context. |
| `onRenameBranch` | rename input commit (Enter or blur) | `branchId`, `name` | Escape cancels without emitting. |
| `onLoadMoreEntries` | load-more click | `offset` = supplied `entries.length` | Only shown when `hasMoreEntries`. |
| `onLoadMoreBranches` | load-more click | `offset` = supplied `branches.length` | Only shown when `hasMoreBranches` and branches supplied. |

## 6. Accessibility

### Semantics

- Trigger cluster: three native `IconButton`s with distinct accessible names
  (`undoLabel`, `listLabel`, `redoLabel`); the list trigger carries
  `aria-expanded`.
- The popover surface is a labelled dialog (`ariaLabel ?? title`) and receives
  initial focus through `Popover`.
- The list is a labelled list region; entry and branch rows are native buttons.
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
| Enter / Space | Activate the focused row: entry → `onSelectEntry`, branch → `onCheckout`. |
| Enter / Escape | In the rename input: commit (`onRenameBranch`) / cancel. |
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

### Composition

- parent expectations: placed in a titlebar/header cluster alongside other
  icon triggers; the trigger cluster is inline.
- child expectations: rows are full-width buttons; branch rows nest under
  their fork entry with an indent.
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
| fork indicator fill | `--poodle-recipe-history-center-fork-indicator-fill` | accent 12% mix |
| fork indicator hover | `--poodle-recipe-history-center-fork-indicator-hover-fill` | accent 22% mix |
| fork indicator glyph | `--poodle-recipe-history-center-fork-indicator-text` | `--poodle-color-accent-base` |
| branch row | `--poodle-recipe-history-center-branch-fill` | transparent |
| branch row (current) | `--poodle-recipe-history-center-branch-current-fill` | accent 10% mix |
| branch hover | `--poodle-recipe-history-center-branch-hover-fill` | surface 72% mix |
| current badge fill | `--poodle-recipe-history-center-branch-current-badge-fill` | accent 16% mix |
| current badge text | `--poodle-recipe-history-center-branch-current-badge-text` | `--poodle-color-accent-base` |
| rejection border | `--poodle-recipe-history-center-rejection-border` | danger 45% mix |
| rejection fill | `--poodle-recipe-history-center-rejection-fill` | danger 10% mix |
| rejection text | `--poodle-recipe-history-center-rejection-text` | `--poodle-color-text-primary` |

Metric variables (widths, gaps, padding, font sizes) are internal and not part
of the recipe contract.

## 9. Svelte Notes

- expected substrate: `HistoryCenter.svelte` composing `IconButton`,
  `Popover`, `Button`, `Icon`, `Spinner`; the machine runs through a `send()`
  channel exactly like `Popover`/`MessageCenter` do.
- wrapper strategy: one `poodle-history-center-popover` root; the trigger
  cluster is the `Popover` trigger snippet; the surface is component-owned.
- implementation-only details: `open` is `$bindable`; the rejection prop diff
  is a `$effect` that sends `SHOW_REJECTION` on new messages.
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
