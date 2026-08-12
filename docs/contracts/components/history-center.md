# HistoryCenter

Status: active contract
Updated: 2026-08-12

## 1. Purpose

- Component name: `HistoryCenter`
- Layer: composite
- Summary: the history counterpart to `MessageCenter` — a compact titlebar-grade trigger cluster (undo / list / redo) plus a popover rendering the flat history list: the spine's entries and each open fork's run at its true depth, with node-owned fork disclosure (a fork icon, a counter badge, a persistent `Select` picker for several forks, and an opened region carrying the run's name and inline rename)
- Composes: `IconButton`, `Popover`, `Select`, `Icon`, `Spinner`, `EmptyState`
- In scope: undo/redo commands, popover open state, flat list rendering (spine + open fork runs, one loop over the core's visible rows), current-position marker, entry navigation, fork disclosure and picker, inline branch rename (opened region and picker), checkpoint pins, transient rejection display, loading/failed status
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

v3 (card `029`) renders the flat row list from `historyCenterVisibleRows`:
entry rows with a `depth` number, picker rows and the not-yet-loaded row, all
in one loop (ruling R1). Captions are gone from the main list — a fork's name
lives in the picker and the opened region (ruling R6).

```text
HistoryCenter
├── Trigger cluster
│   ├── IconButton undo (icon `undo`; enabled from canUndo, busy)
│   ├── Chevron glyph (a bare button, not an IconButton; opens the popover)
│   └── IconButton redo (icon `redo`; enabled from canRedo, busy)
└── Popover
    └── Surface
        ├── Header: title + visible entry count
        ├── Rejection notice (transient, dismissible)
        ├── Status row (loading spinner / failed message)
        ├── History list (the flat visible rows, one loop)
        │   ├── Entry row (depth inset; row button + fork disclosure)
        │   │   ├── Entry button: checkpoint pin or position marker, label,
        │   │   │   group meta; navigates
        │   │   └── Fork disclosure (forkCount > 0): fork icon, counter badge
        │   │       (forkCount > 1), chevron; toggles the fork open/closed
        │   ├── Picker row (an open fork with forkCount >= 1): a persistent
        │   │   `Select` (fork label + branch name) with a rename pencil and a
        │   │   checkout `IconButton` beside it — Select, pencil, checkout (R1);
        │   │   the pencil renames whichever fork the `Select` currently shows, the
        │   │   inline input taking the `Select`'s place while a rename is open
        │   │   (R3); the selected fork's run renders below the select
        │   └── Not-yet-loaded row (an open fork whose run has not arrived):
        │       spinner + "Loading…"
        └── EmptyState
```

The popover content is component-owned; `center`-style snippets are not used
here.

## 3. Props And Inputs

### Data Shapes

Card `028` re-declares the record shapes **structurally** (ruling R2): Poodle
never imports a Longhorn type and no manifest gains one — the dependency runs
Longhorn → Poodle. The three continuation operations
(`loadContinuations`, `loadContinuationRun`, `checkoutContinuation`) are
caller-supplied callbacks; the machine emits them as effects. Checkout is
Poodle's own word (R2a): the host maps `onCheckoutContinuation` onto
Longhorn's `preferContinuation`. The records
mirror the authority's `ForkEntryRecord`, `ForkPathPageSnapshot` and
`ForkContinuationRecord` (camelCase, optional fields optional).

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
  continuationCount: number; // how many entries continue from this one, the
                             // run's own next row included. A fork count is
                             // one less (R4); a run's last entry is always 0.
};

// One bounded page of a path — the root path or a continuation run — newest
// first (R3). offset counts from the newest entry, so the page at a higher
// offset holds older entries.
type HistoryPathPage = {
  entries: HistoryEntry[];
  offset: number;
  precedingContinuationCount: number; // continuations at the position directly
                                 // above this page's first entry, that entry
                                 // included: the history root on a default or
                                 // branch path, the anchor entry on a
                                 // continuation run. A fork count is one less.
                                 // Carried for the host and renderer; the
                                 // derivation emits no row for it.
  truncatedBefore: boolean;      // newer records precede this page
  truncatedAfter: boolean;       // older records follow this page
};

// One continuation at an anchor: the operator's fork. The continuations page
// returns every child of the anchor, including the child already rendered on
// the list — the derivation filters that one out by id and never assumes its
// position (R4).
type HistoryContinuation = {
  entryId: string;         // stable identity of the continuing entry — the
                           // run's first entry
  label: string;
  recordedAtMs?: number;   // never invented client-side
  preferred: boolean;      // whether a redo from the anchor takes this
                           // continuation
  entryCount: number;      // entries in the run starting here, this one included
  branchId: string;        // branch a consumer lands on by taking this continuation
  branchName: string | null;
};
```

### Public Props

The v3 surface: path pages in, three host-operation callbacks out. `pages`
replaces v2's `branches`/`paths`; the load-more and totals props are gone (the
header count is derived from the visible rows). The three continuation
operations are caller-supplied callbacks (ruling R2); results arrive back
through the two result props, diffed by reference — a host must keep a result
reference stable until a new result replaces it (the same rule v2's
`rejection` prop followed).

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `pages` | `HistoryPathPage[] \| null` | `null` | Root path pages in fetch order (newest page first). `null` disables the list: no rows render and every row event is inert. |
| `canUndo` | `boolean` | `false` | Enables the undo trigger. |
| `canRedo` | `boolean` | `false` | Enables the redo trigger. |
| `busy` | `boolean` | `false` | Disables both undo and redo while an authority operation runs. |
| `status` | `"idle" \| "loading" \| "failed"` | `"idle"` | Source status; loading shows a spinner row, failed shows `statusMessage`. |
| `statusMessage` | `string \| null` | `null` | Copy for the failed status row. |
| `rejection` | `HistoryCenterRejectionCode \| null` | `null` | A rejection code the host's bridge mapped from the protocol (`AlreadyAtTarget` \| `UnknownEntry`); the component owns the display copy. `null` clears the notice. |
| `continuationsResult` | `{ entryId: string; continuations: HistoryContinuation[] } \| null` | `null` | Host op 1 result: the continuations at an anchor, fed back after `onLoadContinuations`. A new non-null reference dispatches `CONTINUATIONS_LOADED`. |
| `runResult` | `{ fromEntryId: string; pages: HistoryPathPage[] } \| null` | `null` | Host op 2 result: a continuation run's pages in fetch order, fed back after `onLoadContinuationRun`. A new non-null reference dispatches `RUN_LOADED`. |
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
| `onNavigateEntry` | `(branchId: string \| null, entryId: string) => void` | `null` | Entry row activation — always the entry actually clicked, on the branch that owns its run. `null` branch on the spine: the host knows its own branch. Replaces v1's `onSelectEntry`/`onCheckout` (ruling D1). |
| `onRenameBranch` | `(branchId: string, name: string) => void` | `null` | Committed inline branch rename (opened region or picker). |
| `onLoadContinuations` | `(entryId: string) => void` | `null` | Host op 1: load the continuations at the anchor entry. |
| `onLoadContinuationRun` | `(fromEntryId: string) => void` | `null` | Host op 2: load the run starting at the fork's first entry. |
| `onCheckoutContinuation` | `(entryId: string) => void` | `null` | Host op 3: checkout the picked continuation — the selected fork becomes the primary history. The host maps the callback onto its own prefer operation (R2a); Longhorn names are never Poodle's. |
| `onDeleteContinuation` | `(entryId: string) => void` | `null` | Host op 4: delete the picked continuation. Opt-in — absent callback, absent menu item, never a disabled stand-in for "unsupported". Poodle deletes nothing itself and does not guess at the resulting history; the host runs the operation and supplies new pages. Not `prune_to`, which is budget-driven retention. |

### Command-Only Callbacks

Undo/redo/navigate/rename and the four host operations are **commands out** —
the component emits the callback on user activation and does nothing else. It
never invokes a callback speculatively: no auto-undo, no implicit checkout on
open, no continuations load ahead of the user's disclosure click. The host
owns what undo does and whether a rejected command is retried.

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
| list | `pages` supplied | The flat list renders: spine entries at depth 0, open fork runs at their true depth, one loop (R1) |
| disclosed | a fork disclosure toggled | `forkCount === 1` → the run's not-yet-loaded row, then the run when its pages arrive; `forkCount > 1` → the picker row (R3 selects the current fork and shows its run under the select; the picker persists) |
| empty | `pages` null or zero visible rows | No list; empty state (or the status row when loading/failed) |
| loading | `status === "loading"` | Spinner status row |
| failed | `status === "failed"` | `statusMessage` status row |
| rejected | `rejection` non-null | Transient inline notice; dismissible |

### Component States

The list is **presence-driven**: when `pages` is not supplied the machine has
no rows and the popover shows the empty state — absence is the signal,
matching `AppHeader`'s `center` region. There is no `mode` prop. The root
path alone is the linear spine; forks appear on entries as disclosure
affordances (a picker when `forkCount > 1`, the run when one fork is open).

Behavior classification: `machine-backed`.

### Behavior Machine

Contract: `packages/core/src/history-center.ts`. Card `028` replaces the v2
stitcher with a **flat list that owns forks on the entry** (ruling R1): core
derives one flat array of visible rows, each with a `depth` number plus the
entry it hangs off and the fork it belongs to as identifiers — no renderer
recurses, no `svelte:self`, no self-import, no recursive React component.
`flattenVisibleTreeRows` in `packages/core/src/tree.ts` is the in-repo
precedent; the flat `depth`-carrying row list already has a native counterpart
(`packages/render/src/tree.rs`), which is what the GPUI/Jetstream port
consumes. Depth alone is not enough — every row also carries its parent entry
id and its fork identity as data, so the v2 ambiguity cannot return in any
renderer (R1 condition).

The machine owns popover open state, roving focus over the visible rows keyed
by **row identity** (never index — a disclosure toggle changes the list shape
underneath an index), the disclosure tree, and transient rejection display.
Path pages and the open-forks state are part of context, supplied by the
caller on every transition; rows are re-derived per transition. Undo/redo are
plain button commands the adapter forwards directly — they carry no machine
state and are never invoked speculatively.

The v2 stitcher surface is deleted with this card: `historyCenterRows`,
`historyCenterRowCount`, the v2 `HistoryCenterRow`, `HistoryRowLane`,
`HISTORY_TREE_DEPTH_CAP`, `HistoryBranch`, and the `paths` context field.
There is **no depth cap** (v2 had one and it hid nesting): depth is a number
the renderer uses and is never saturated. `emitRenameBranch` and
`maxBranchNameBytes` survive unchanged (ruling R6: rename is a client-side
affordance that enforces no protocol rule, surfaced in the opened region and
the picker (R1) — both through the same machinery, no second rename path, no
silently dropped capability).

The three continuation operations (`loadContinuations`,
`loadContinuationRun`, `checkoutContinuation`) arrive as caller-supplied
callbacks and leave as effects (ruling R2) — Poodle cannot call the host's
controller, so the host wires the callbacks to it, exactly like `MessageCenter`.

**Stale levels (R2, g13-034).** An open level caches `continuations`, `pick`,
`chosen` and `runPages` from when it loaded. When the host navigates into the
fork — the new root pages already contain the run — that cache is stale: the
derivation never splices a run that duplicates a spine entry, and the level
renders the existing `not-yet-loaded` row instead (no new row kind). Before
every open-state event except `CLOSE` and a closing `TOGGLE`, the machine
reconciles the disclosure tree against the current root pages: a level whose
shown fork's first entry id appears in the joined root pages drops its loaded
data (continuations, pick, chosen, run pages and the inner subtree) but stays
**open** at its anchor — disclosure is UI state and persists (b028 R1); this
is not a close — and re-requests through the existing `loadContinuations`
effect, so the picker re-reads its continuations and offers the line just
left. The check is data-based, never array-identity based: a host that
rebuilds its pages array each render cannot loop it, and once dropped the
level has no shown fork, so exactly one re-request leaves, never one per
derivation. The re-requested data lands through the ordinary
`CONTINUATIONS_LOADED` flow — no `SYNC` / `REFRESH` event exists or is added.

#### Context

| Field | Type | Initial | Controllable | Meaning |
|-------|------|---------|--------------|---------|
| `pages` | `HistoryPathPage[] \| null` | `null` | no (host-supplied) | Root path pages in fetch order (newest page first). `null` disables the list: the machine has no rows and every row event is inert. |
| `open` | `Map<string, HistoryCenterOpenFork> \| null` | `null` | no (machine-owned) | The disclosure tree: open forks keyed by anchor entry id, with `inner` levels for forks open inside runs. Holds only what is open (R5); dropped on close — nothing is cached across a close/reopen and pages are not cached or refreshed. A level whose shown fork's run is now on the root spine is **stale**: it keeps its anchor open, drops its loaded data and re-requests its continuations (R2, g13-034). |
| `focusRow` | `HistoryCenterRowId \| null` | `null` | no | Roving focus identity over the visible rows. |
| `rejection` | `string \| null` | `null` | no | Currently displayed rejection message. |

#### Row Model

`historyCenterVisibleRows(pages, open)` is the **visible-row derivation** — a
pure, exported function (ruling D5: it never fetches; it knows nothing of
Longhorn, ports, or paging). It returns one flat array of rows in **display
order** (oldest entry first, newest last), each with a `depth` number. The
renderer receives the depth number and knows nothing about topology — core
knows it, which is core's job (R1).

**Ordering (ruling R3).** Path pages — the root and each nested run — arrive
**newest-first**: `entries[0]` is newest and `offset` counts from the newest,
so a page at a higher offset holds older entries. Display is **newest last**.
Core performs that reversal exactly once, in `historyCenterJoinPages`, and
every level — the root and each nested run — reverses by the same code.
Joining pages in fetch order is the paging trap and puts history backwards:
after the reversal, the later-fetched (older) page renders **before** the
first page. Overlapping page seams dedupe by `entryId`. `continuations` is
**not** reversed — it is stable graph order, a picker, not a timeline.

**Forks (ruling R4).** `forkCount = max(0, continuationCount - 1)`:
`continuationCount` counts every continuation **including** the one that is
the next row, and `precedingContinuationCount` is the same fact one level above
the page's first entry.

The saturation matters. A run's terminal entry has **no children at all**, so
its `continuationCount` is `0` and the unsaturated form would yield `-1`. The
authority guarantees this: recording always installs the new entry as its
parent's preferred continuation, so a node with children always has a preferred
one, and the run walk stops only at a childless node. Longhorn added the guard
for it in `777de887` after finding two fixtures that violated the invariant. The continuations page also returns the
child already on the list; the derivation filters it out **by id** — the
anchor's successor in the run, which is also the preferred continuation when
the successor is not on a loaded page — and never assumes its position.

```text
entry e1                    ← spine, depth 0, parent -, fork -
entry e2                    ← depth 0, hangs off e1
  picker                    ← e2 has forkCount > 1: its forks, own continuation filtered
  entry f1a                 ← opened fork run, depth 1, parent e2, fork f1a
  entry f1b                 ← depth 1, parent f1a, fork f1a
entry e3                    ← spine continues
```

Rows (every row carries `depth`, `parentEntryId` and `forkId` as data, not as
indentation — the R1 condition: two forks at one entry are never confusable
with a fork off a fork, at any depth, with no cap):

- `{ kind: "entry"; entry; depth; parentEntryId; forkId; branchId; forkCount }`
  — one entry of the list. Spine rows carry `parentEntryId: null` and
  `forkId: null` (the trunk is not a fork); run rows carry their anchor and
  run identity, where `forkId` is the run's first entry (the continuation's
  stable identity). `branchId` is the run's continuation branch, `null` on the
  spine (the host knows its own branch). Activation emits
  `emitNavigateEntry(branchId, entryId)` with the clicked row's **own** branch
  and entry — never an ancestor or another branch's divergence entry.
- `{ kind: "picker"; anchorEntryId; depth; parentEntryId; forkId; continuations; pickedEntryId; disabled }`
  — the forks at the anchor, the child already on the list filtered out, and
  the select's value (the tentative pick, else the auto-chosen single fork;
  null only while the level's continuations are in flight). **The picker
  persists for as long as the level is open (R1)**: it is emitted whenever
  the open entry's `forkCount >= 1`, whatever `chosen` holds — the current
  selection stays visible and a second fork is one interaction away, never a
  close-and-reopen. `disabled` is true when `forkCount <= 1` and **governs
  the `Select` alone** (R1, g13-034): with one fork there is nothing to
  choose between, while the auto-chosen single fork still counts as picked
  for the actions menu — the menu never inherits the row's `disabled`.
  Picker rows never navigate.
- `{ kind: "not-yet-loaded"; anchorEntryId; depth; parentEntryId; forkId; branchId }`
  — an open entry whose run has not arrived. Never an empty gap, never a
  dropped entry. It is also the row a **stale** level renders (R2, g13-034):
  a level whose shown fork's first entry now sits on the root spine never
  splices its cached run — that would duplicate spine entries — and shows
  the not-yet-loaded row until the machine drops the level's data and
  re-requests its continuations. No new row kind is invented.

Rows are keyed by identity (`HistoryCenterRowId`: kind + entry id) for roving
focus; traversal is linear over the flat array in visual order and survives a
disclosure toggle because the focus is the row, not an index.

#### States

| State | Description |
|-------|-------------|
| `closed` | Popover closed; trigger cluster interactive, undo/redo per `canUndo`/`canRedo`/`busy`. |
| `open` | Popover open; list navigable, disclosure and rejection events live. |

#### Events

| Event | Payload | Source |
|-------|---------|--------|
| `TOGGLE` / `OPEN` / `CLOSE` | — | trigger / programmatic / dismiss |
| `FOCUS_MOVE` | `direction: "next" \| "prev" \| "first" \| "last"` | keyboard |
| `ACTIVATE_ROW` | `row?` (default: focused row identity) | keyboard / pointer |
| `DISCLOSE` | `entryId` | fork disclosure affordance (toggles open/closed) |
| `CONTINUATIONS_LOADED` | `entryId`, `continuations` | adapter (host callback result) |
| `PICK_CONTINUATION` | `entryId` (the fork's first entry) | picker selection |
| `CONFIRM` | — | picker confirm |
| `RUN_LOADED` | `fromEntryId`, `pages` | adapter (host callback result) |
| `RENAME` | `branchId`, `name` | rename input commit |
| `SHOW_REJECTION` | `code: "AlreadyAtTarget" \| "UnknownEntry"` | adapter (rejection prop change) |
| `DISMISS_REJECTION` | — | notice dismiss button |

The v1 expansion events (`TOGGLE_BRANCHES`, `EXPAND_BRANCHES`,
`COLLAPSE_BRANCHES`) and `CHECKOUT` are retired (ruling D1); the v2 events
were index-based (`ACTIVATE_ROW` carried `index`), replaced here by row
identity.

#### Transitions

| State | Event | Guard | Target | Actions / Effects |
|-------|-------|-------|--------|-------------------|
| closed | `TOGGLE` / `OPEN` | — | open | `emitOpenChange(true)` |
| open | `TOGGLE` / `CLOSE` | — | closed | `emitOpenChange(false)`; drop the disclosure tree and focus (R5) |
| open | `OPEN` / closed | — | stay | — |
| open | `FOCUS_MOVE` | row count > 0 | open | move `focusRow` by identity (wrap; first/last land on boundaries); `focusRow(row)` |
| open | `ACTIVATE_ROW` | row exists | open | entry row → `emitNavigateEntry(branchId, entryId)` for the clicked row's own branch and entry; picker / not-yet-loaded row → focus syncs, no effect |
| open | `DISCLOSE` | entry visible with `forkCount >= 1` | open | closed fork at the entry opens: add its level, `loadContinuations(entryId)`; open fork closes: drop its subtree (R5), clamp focus |
| open | `CONTINUATIONS_LOADED` | entry open | open | store continuations; a single fork (`forkCount === 1`) auto-chooses it and emits `loadContinuationRun(fromEntryId)`; more than one fork selects the current one — `preferred`, else first in supplied order (R3) — and emits `loadContinuationRun` for it |
| open | `PICK_CONTINUATION` | fork offered by a picker | open | set the tentative pick (one at a time); when the loaded run belongs to another fork, drop it and emit `loadContinuationRun(fromEntryId)` — the pick previews (R2) and emits no host operation |
| open | `CONFIRM` | displayed fork set (pick, else the auto-chosen single fork — it counts as picked, R1 g13-034) | open | `checkoutContinuation(entryId)` — the picker's commit (R2): the selected fork becomes primary. Poodle does not build the new root: it clears the disclosure state for the anchor and renders whatever root pages the host supplies afterwards |
| open | `RUN_LOADED` | level's displayed fork (pick, else chosen) is `fromEntryId` | open | append run pages (fetch order); the run renders through the same join |
| any | `RENAME` | — | stay | `emitRenameBranch(branchId, name)` |
| any | `SHOW_REJECTION` | mapped message differs from displayed | stay | set `rejection` |
| any | `DISMISS_REJECTION` | rejection displayed | stay | clear `rejection` |
| closed | row / disclosure events | — | stay | — |

Guards are pure predicates over context + payload. Out-of-list activation,
empty row lists (including `pages: null`), entries with `forkCount < 1`,
stale responses for entries that are not open, and closed-state row events are
all inert (stay with no effects). Focus is re-clamped after any disclosure
toggle: the focused row stays focused when it still exists, else focus falls
to the toggled anchor's entry row, else the first row. `emitNavigateEntry`
always reports the entry actually clicked — the clicked row carries its own
`branchId` and `entry`, so an ancestor or another branch's divergence entry
can never be reported.

Every open-state transition except `CLOSE` and a closing `TOGGLE` reconciles
stale levels first (R2, g13-034 — see above): the stale level's
`loadContinuations` leaves before the event's own effects, and the event then
runs against the reconciled tree.

#### Effects

| Effect | What It Does | Cleanup |
|--------|--------------|---------|
| `emitOpenChange { open }` | Adapter syncs the bindable `open` and calls `onOpenChange`. | None (host-owned). |
| `focusRow { row }` | Adapter moves roving focus to the row element identified by `row` and scrolls it into view. | Overridden by the next `focusRow`. |
| `emitNavigateEntry { branchId, entryId }` | Adapter calls the host's navigate handler with the clicked row's branch (null on the spine — the host's own branch) and entry. | None (host-owned). |
| `emitRenameBranch { branchId, name }` | Adapter calls `onRenameBranch(branchId, name)`. | None (host-owned). |
| `loadContinuations { entryId }` | Host op 1: the adapter calls the host's continuation loader for the anchor. | None (host-owned). |
| `loadContinuationRun { fromEntryId }` | Host op 2: the adapter calls the host's run loader starting at the fork's first entry. | None (host-owned). |
| `checkoutContinuation { entryId }` | Host op 3: the adapter calls the host's checkout handler for the picked fork — the picker's commit. The host maps it onto its own prefer operation (R2a); Poodle never names the host's operation. | None (host-owned). |
| `deleteContinuation { entryId }` | Host op 4: the adapter calls `onDeleteContinuation(entryId)` for the picked fork, once the operator has confirmed. Poodle does not touch its own pages — the anchor's disclosure state and the rows stand until the host supplies new pages. | None (host-owned). |

The v1 `emitSelectEntry` and `emitCheckout` effects are gone; the v2 index
`focusRow { index }` is replaced by identity `focusRow { row }`. Effects for
the four host operations leave only on user activation or a matching loaded
event — never speculatively. `PICK_CONTINUATION` emits at most
`loadContinuationRun` (the preview) and never a host operation; `CONFIRM`
emits `checkoutContinuation` alone — the fork's run was already previewed by
the pick (R2).

**Rejection handling.** The machine owns display copy for the two rejections
it can show, declared structurally (R2): `AlreadyAtTarget` →
`"Already at the requested target"` (the picked fork is already the preferred
future), `UnknownEntry` → `"Entry does not exist"` (a continuation or run
anchor no longer exists). The host's bridge maps protocol rejections onto
these two codes; `SHOW_REJECTION` stores the mapped message.

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
| list trigger | `data-part` / `data-size` / `aria-expanded` / `aria-label` | `list-trigger` / the resolved control size / `true` \| `false` from open state / `listLabel`. A bare `chevron-down` glyph in a plain `button`, not an `IconButton`: undo and redo carry the cluster's weight and the disclosure reads narrower between them. It keeps its accessible name, its focus ring and a full-height hit area. |
| surface | `data-part` / `data-state` | `surface` / `open` \| `closed` |
| list | `data-part` / `aria-label` | `list` / `listLabel` |
| row (all kinds) | `data-part` / `data-row-kind` / `data-row-entry` / `data-depth` / `aria-level` | the row's kind (`entry` \| `picker` \| `not-yet-loaded`) / kind / entry id (the anchor's for non-entry rows) / depth (0-based) / `depth + 1` |
| entry row | `data-position` / `data-checkpoint` / `data-fork-count` / `data-parent-entry` / `data-fork-id` | `past\|current\|future` / presence when checkpoint / `forkCount` (R3) / parent entry id (`null` on the spine) / run's first entry id (`null` on the spine) |
| entry button | `data-open` | `true` when a fork is open at the row's entry |
| fork disclosure | `data-part` / `data-open` / `aria-expanded` / `aria-label` | `fork-disclosure` / `true` when open / open state / `Show\|Hide N continuation(s)` — rendered only when `forkCount > 0` |
| fork badge | `data-part` | `fork-badge` — rendered only when `forkCount > 1`, reads `forkCount` |
| picker | `data-part` / `data-anchor` / roving tabindex | `picker` / the anchor entry id — rendered whenever the open entry's `forkCount >= 1`, persisting across a choice (R1) |
| picker select | `data-part` (wrapper) | `picker-select` — Poodle's `Select`, its value the tentative pick, options the forks; `PICK_CONTINUATION` on change. The trigger and the option rows carry the fork label and its branch name. Disabled from the row's `disabled` signal alone (`forkCount <= 1` — nothing to choose between; R1, g13-034). Replaced by the rename input while a rename is open (R3) |
| picker actions | `data-part` (wrapper) | `picker-actions` — an ellipsis `Menu` after the `Select`, holding `Rename`, `Checkout` and `Delete`. Every item acts on whichever fork the `Select` currently shows. `Delete` appears only when the host supplies its callback. `Checkout` is disabled when no fork is picked, when the picked fork is already the current one, or while a rename is open — it never inherits the row's `disabled` signal, so the auto-chosen single fork still counts as picked (R1, g13-034). Focus returns to the menu trigger after a rename commits or cancels. |
| picker rename input | `data-part` | `picker-rename-input` — the inline input that takes the `Select`'s place while a rename is open, seeded with the selected fork's current name (R3) |
| not-yet-loaded | `data-part` / `data-anchor` / roving tabindex | `not-yet-loaded` / the anchor entry id |
| rejection | `data-part` / `role` | `rejection` / `status` |

The v2 lane surface (`data-lane`, the lane gutter) and the caption row
(`data-part="caption"`, `data-current`, branch-current badge) are retired with
the v2 renderer. Depth reaches assistive tech through `aria-level` (see §6);
`data-depth` carries the raw depth for styling and tests — never clamped.

#### Machinery Dependencies

Focus trap (`trapFocusKeydown`), roving row navigation (machine-owned
`focusRow` identity), dismissable layer + anchor positioning + initial focus (via the
composed `Popover`), `createInstanceId` for surface ids.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onUndo` | undo trigger click while `canUndo && !busy` | — | Plain command; host decides what undo means. |
| `onRedo` | redo trigger click while `canRedo && !busy` | — | Plain command; host decides what redo means. |
| `onOpenChange` | open state actually changes | `boolean` | Never emitted speculatively. |
| `onNavigateEntry` | entry row activation (pointer or keyboard) | `branchId`, `entryId` | Always the entry actually clicked, on the branch that owns its run — never an ancestor or another branch's divergence entry. `branchId` is `null` on the spine. Picker and not-yet-loaded rows never fire it. Replaces v1's `onSelectEntry`/`onCheckout` (ruling D1). |
| `onRenameBranch` | rename input commit (Enter or blur) in the opened region or the picker | `branchId`, `name` | In the picker, `branchId` is the fork the `Select` currently shows (R1) — never the anchor's own branch or the preferred fork. Escape cancels without emitting. |
| `onLoadContinuations` | fork disclosure opens an entry with `forkCount >= 1`; an open level becomes stale — its shown fork's run now sits on the root spine, so the machine re-requests the continuations (R2, g13-034) | `entryId` | Host op 1: the host loads the continuations at the anchor and feeds the result back as `continuationsResult`. |
| `onLoadContinuationRun` | a single fork is auto-chosen (R3), or a fork is selected in the picker whose run is not loaded (R2) | `fromEntryId` | Host op 2: the host loads the run starting at the fork's first entry and feeds the result back as `runResult`. |
| `onDeleteContinuation` | the operator confirms the delete dialog | `entryId` | Host op 4: delete the fork the `Select` shows. Emitted once, on confirmation only — picking the menu item emits nothing. Cancelling or dismissing the dialog emits nothing at all. |
| `onCheckoutContinuation` | the picker's checkout is confirmed on a fork that is not the current one | `entryId` | Host op 3: checkout the picked fork — it becomes the primary history. The host maps the callback onto its own prefer operation (R2a). Emitted alone (the run was previewed by the pick, R2) and never with navigation. The document does not move forward: Poodle clears the anchor's disclosure state and renders whatever root pages the host supplies afterwards. |

The v2 `onLoadMoreEntries`/`onLoadMoreBranches` callbacks are retired with the
v2 paging surface; the machine's `RENAME` event surfaces rename in the opened
region (R6) and the picker (R1) — one event, both sites.

## 6. Accessibility

### Semantics

- Trigger cluster: three native `IconButton`s with distinct accessible names
  (`undoLabel`, `listLabel`, `redoLabel`); the list trigger carries
  `aria-expanded`.
- The popover surface is a labelled dialog (`ariaLabel ?? title`) and receives
  initial focus through `Popover`.
- The list is a labelled list region (`ul` with `listLabel`); every row is an
  `li`. Entry rows are native buttons; the fork disclosure button is its
  row's secondary control — a sibling, never nested inside the entry button
  (no interactive element nests inside another interactive element).
- **Depth reaches assistive tech through `aria-level`** on every row
  (1-based, `depth + 1`), with `data-depth` carrying the raw 0-based depth for
  styling and tests. A `tree` role was considered and rejected: the machine's
  traversal is linear over the flat visible array in visual order, so native
  list semantics plus `aria-level` describe the rows more honestly than a tree
  whose children are not DOM-nested. Depth is never clamped.
- The fork disclosure button carries `aria-expanded` and an accessible label
  naming the fork count (`Show 2 continuations` / `Hide 2 continuations`);
  the counter badge is decorative (`aria-hidden` not required — it is inside
  the button's label scope and reads as part of the accessible name).
- The picker row is Poodle's `Select` (a labelled combobox; its listbox
  options carry `aria-selected` for the selection) followed by one ellipsis
  `Menu`. The trigger and every option carry the fork label, its branch name,
  its entry count and its derived relative time. The menu holds `Rename`,
  `Checkout` and `Delete`, each acting on whichever fork the `Select` shows.
  While a rename is open the inline input takes the `Select`'s place (R3).
  Checkout is disabled until a fork that is not the current one is selected
  (R4: `AlreadyAtTarget` stays a race, not a normal path) and while a rename
  is open (R3).
- **`Delete` confirms inside Poodle.** Picking the item opens an
  `AlertDialog` naming the fork; the command leaves only on confirmation.
  This reverses b033's R4, which made confirmation the host's policy: every
  host would have had to build the same dialog, and one that forgot would
  ship a menu item that destroys history on a single click. A host that wants
  its own additional confirmation still can — it owns the operation — but it
  is no longer the only thing standing between a click and lost work. One
  dialog serves the whole component; it renders outside the `Popover` and the
  dismiss-layer ancestry (b031) keeps the history list open behind it. Checkout is not the only way to activate a fork — picking any
  entry inside the fork's rows navigates to that point; checkout exists to
  make a fork primary **without** moving HEAD.
- The not-yet-loaded row is a non-interactive roving-focus stop with a
  spinner; the loading status is `role="status"`.
- The checkpoint pin and position marker are decorative; the entry label
  carries the meaning.
- The rejection notice is `role="status"` (polite live region) — never silent,
  never announced as an alert.
- Loading and failed status rows are `role="status"`.

### Keyboard

| Key | Behavior |
|-----|----------|
| ArrowDown / ArrowUp | Move roving focus to the next/previous visible row, wrapping at the ends. Inside the picker's `Select`, the keys belong to the select (arrows open the listbox and move its highlight) — the machine never maps them. |
| Home / End | Move focus to the first/last row. Inside the picker's `Select`, the keys belong to the select. |
| Enter / Space | Activate the focused row: entry → `onNavigateEntry(branchId, entryId)`; picker / not-yet-loaded → focus syncs, nothing navigates. On the fork disclosure, the picker's select trigger, or the picker's actions menu, the key activates that control natively (disclosure toggles the fork, the select opens/picks, the menu opens) — never row navigation. |
| Enter / Escape | In the rename input: commit (`onRenameBranch`) / cancel. After commit or cancel, focus returns to the picker's actions-menu trigger, which is what opened the rename. |
| Tab / Shift+Tab | Trapped within the open surface (wraps first↔last focusable). Within a focused row, Tab moves entry button → fork disclosure in visual order; the picker's select trigger and its actions-menu trigger are tabbable from the picker row. |
| Escape | Close through `Popover`; focus returns to the trigger. Inside the picker's `Select`, Escape closes its listbox first. |

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

### Depth Inset

- Every row's left padding grows one inset step per depth level via the
  row-scoped `--poodle-history-center-depth` custom property
  (`padding-left: calc(var(--poodle-history-center-inset-step) * var(--poodle-history-center-depth))`).
  Depth drives padding and nothing else (R1); there is **no depth cap** — v2's
  `HISTORY_TREE_DEPTH_CAP` is gone and must not return in CSS. The inset step
  is an internal metric, not recipe-themable.

### Composition

- parent expectations: placed in a titlebar/header cluster alongside other
  icon triggers; the trigger cluster is inline.
- child expectations: rows are full-width list rows; entry content, the run
  header and the picker carry the padding; the content inset grows with depth
  via the row padding.
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
| fork icon | `--poodle-recipe-history-center-fork-color` | text-secondary |
| fork badge fill | `--poodle-recipe-history-center-fork-badge-fill` | accent 16% mix |
| fork badge text | `--poodle-recipe-history-center-fork-badge-text` | `--poodle-color-accent-base` |
| rejection border | `--poodle-recipe-history-center-rejection-border` | danger 45% mix |
| rejection fill | `--poodle-recipe-history-center-rejection-fill` | danger 10% mix |
| rejection text | `--poodle-recipe-history-center-rejection-text` | `--poodle-color-text-primary` |

The v2 lane hooks (`lane-color`, `lane-thickness`), the caption-row hooks and
the branch-current-badge hooks are retired with the v2 renderer; the fork
badge takes over the badge role (R3, picker). The `current-badge` hooks from
g13-030 are retired with the badge itself (R4a): `preferred` stays on the
record and keeps its job — checkout is disabled when the selected fork is
already the current one. Metric variables (widths, gaps, padding, the depth
inset step, font sizes) are internal and not part of the recipe contract.

## 9. Svelte Notes

- expected substrate: `HistoryCenter.svelte` composing `IconButton`,
  `Popover`, `Select`, `Icon`, `Spinner`; the machine runs through a `send()`
  channel exactly like `Popover`/`MessageCenter` do.
- wrapper strategy: one `poodle-history-center-popover` root; the trigger
  cluster is the `Popover` trigger snippet; the surface is component-owned.
- implementation-only details: `open` is `$bindable`; the rejection prop diff
  and the two result feeds are `$effect`s that dispatch
  `SHOW_REJECTION` / `CONTINUATIONS_LOADED` / `RUN_LOADED` on new values
  (diffed by reference); rows come from `historyCenterVisibleRows(pages,
  openForks)` in ONE `{#each}` keyed by row identity — no `svelte:self`, no
  recursion (R1); the picker's relative time is derived from supplied
  `recordedAtMs` values only — there is no clock (ruling D2); rename is
  surfaced from the picker's actions menu.
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
