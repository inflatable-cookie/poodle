# 12 — g13.028 HistoryCentre v3 Flat List, Node-Owned Forks (Core) (batch log)

Branch: `thread/g13-028-history-center-v3-core`
Date: 2026-08-12
Card: `docs/roadmaps/g13/batch-cards/028-history-center-v3-core.md`
Status: **DELIVERED** — v3 records, flat visible-row derivation and machine in
`poodle-core`, all required tests green, contract updated. Rendering, the
picker, the nested list, specimens and the props surface are card `029`; the
expected `test:components` failure is recorded below, not chased.

## 1. Record types — `packages/core/src/history-center.ts`

Structural mirrors of the authority's shapes, verified against the Longhorn
prototype source (`~/Dev/projects/longhorn/crates/longhorn-history-tree/src/`):
`protocol/path.rs` (`ForkEntryRecord`, `ForkPathPageSnapshot`),
`protocol/continuation.rs` (`ForkContinuationRecord`), `projection/project.rs`
(ordering, `continuation_count` = `child_ids(entry).len()`, lineage pages
reversed newest-first). No Longhorn import, no manifest entry (R2).

- `HistoryEntry` — v2 mirror plus required `continuationCount`. The authority's
  doc line is quoted in the record comment: "How many entries continue from
  this one, this page's own next entry included. A fork count is one less; a
  run's last entry is always zero" — R4 verbatim. `recordedAtMs` stays
  optional; nothing invents a clock.
- `HistoryPathPage` — `entries`, `offset`, `rootContinuationCount`,
  `truncatedBefore`, `truncatedAfter` (the "both truncation flags"). No
  `branch_id`/`head_entry_id` — the card enumerates exactly these fields, and
  the authority's default-path page carries `branch_id: None` anyway.
- `HistoryContinuation` — `entryId`, `label`, `recordedAtMs`, `preferred`,
  `entryCount`, `branchId`, `branchName`, matching `ForkContinuationRecord`
  field for field. `entryId` is "the stable identity of the continuing entry"
  — the run's first entry, which doubles as the row-level fork identity (R1).

## 2. The visible-row derivation — `historyCenterVisibleRows(pages, open)`

Pure, exported, flat. One array of rows in display order; the renderer never
recurses (R1). `flattenVisibleTreeRows` (tree.ts) is the shape followed; the
`depth`-carrying flat list is what `poodle-render`'s native consumers already
get.

- **R3 — one reversal.** `historyCenterJoinPages` joins pages in fetch order
  and reverses once to display order (oldest entry first). Pages arrive
  newest-first; a later-fetched (older) page renders before the first page.
  Root and every nested run join through this same function — the paging trap
  (fetch-order join = history backwards) is impossible to express in a caller.
  Overlapping seams dedupe by `entryId`.
- **R4 — forks.** `historyCenterForkCount(continuationCount) = max(0, count -
  1)`. `continuationCount` includes the run's own next row, so a run's last
  entry reads 0 and one fork reads 1. The picker row's continuations are
  filtered by `historyCenterForksAt`: the child already on the list is the
  anchor's successor in the run, removed by id (never by position); when the
  successor is not on a loaded page, the preferred flag names the same record
  (the run follows preferred children — verified in `projection/project.rs`).
- **R1 condition.** Every row carries `depth`, `parentEntryId` and `forkId`
  as data. Spine rows: `parentEntryId: null`, `forkId: null` (the trunk is not
  a fork); run rows: the run's first entry is the `forkId`, and the first run
  row hangs off the anchor, each later row off its predecessor (graph truth —
  subsequent spine rows hang off their predecessor too). No depth cap; the
  five-deep chain test renders depth 5 with real identities.
- Row kinds: `entry` (with `forkCount` and `branchId` — the run's continuation
  branch, `null` on the spine), `picker` (only when the open entry's
  `forkCount > 1`; carries the filtered forks and the tentative pick),
  `not-yet-loaded` (an open entry whose run has not arrived — never a gap).
- `rootContinuationCount` is carried on the record (a root fork count is one
  less, per R4) but the derivation emits no root-level row for it — the card's
  row kinds don't include one; rendering it is `029`'s call.

## 3. The machine — disclosure tree, identity focus, host-op effects

Context: `{ pages, open, focusRow, rejection }`. `open` is a disclosure tree
of `HistoryCenterOpenFork` levels keyed by anchor entry id, nesting `inner`
levels for forks inside runs — multiple entries can be open at any level, the
same expansion-set shape as `flattenVisibleTreeRows`' `expandedValues`.

- `focusIndex` is gone; `focusRow` is a row identity (`HistoryCenterRowId` =
  kind + entry id). `FOCUS_MOVE` wraps by identity; every row-shaping
  transition re-clamps: keep the focused row when it still exists, else fall
  to the toggled anchor's entry row, else the first row. Test proves focus
  survives a disclosure toggle while the row's index shifts.
- `DISCLOSE { entryId }` toggles: closed fork at a visible entry with
  `forkCount >= 1` opens (emits `loadContinuations`) and attaches under the
  level whose run contains the entry (or the root); an open fork closes and
  drops its subtree (R5).
- `CONTINUATIONS_LOADED { entryId, continuations }` stores raw continuations;
  a single fork (`forkCount === 1`) auto-chooses and emits
  `loadContinuationRun(fromEntryId)`. `PICK_CONTINUATION { entryId }` sets the
  tentative pick (one at a time, only forks the picker offers — the own
  continuation is rejected); `CONFIRM` commits with
  `preferContinuation(entryId)` + `loadContinuationRun(fromEntryId)` (the
  picker's commit, per the execute.rs comment "the picker and the commit name
  the same thing").
- `RUN_LOADED { fromEntryId, pages }` appends pages to the level whose chosen
  fork matches `fromEntryId` — unambiguous when several runs are in flight.
  Stale responses for entries that are not open are inert.
- `CLOSE` (popover) drops the disclosure tree and focus (R5: nothing is cached
  across a close/reopen; the handoff says pages are not cached and not
  refreshed).
- Rejection: `SHOW_REJECTION { code }` with the two codes declared
  structurally — `AlreadyAtTarget` → "Already at the requested target"
  (verified in `navigation/error.rs`: the only stand-still case that is
  genuinely nothing to do), `UnknownEntry` → "Entry does not exist"
  (`navigation/error.rs`, `projection/error.rs`). The host's bridge maps
  protocol rejections onto these two.
- `RENAME { branchId, name }` → `emitRenameBranch` unchanged (R6); the opened
  region's run rows carry `branchId` so the renderer can target rename at the
  fork. `historyCenterKeydownEvent` unchanged.

## 4. Deletions — same commit as the index change

`historyCenterRows`, `historyCenterRowCount`, `HistoryRowLane`,
`HISTORY_TREE_DEPTH_CAP`, the v2 `HistoryCenterRow`, `HistoryBranch` (v2
surface: only caption rows used it), and the `paths` context field. All
removed from `packages/core/src/index.ts` in the same commit — no dead
re-export, the b023 link-time lesson. Verified with the step-7 index probe.

## 5. Contract — `docs/contracts/components/history-center.md`

Updated only the record types (§3 Data Shapes), the Behavior Machine (context,
row model with the ordering and fork rulings, states, events, transitions,
effects, rejection) and the Component States paragraph, per the writable
scope. Rendering sections — §2 Anatomy, Visual States, Part Attribute Output,
§5 Events (public callbacks), §9 Svelte Notes — are marked `029`-owned with a
pointer at the v3 shape. The public props table is left frozen (029 migrates
it); a callout says so. §1 Purpose still summarises v2 — 029 reconciles it.
`docs:lint` structure (numbered headings, sections) preserved.

## 6. Validation

| Command | Exit | Result |
|---------|------|--------|
| `effigy test:core` | 0 | 482 pass / 44 files, 0 fail (was 466) |
| `effigy docs:lint` | 0 | 171 component contracts |
| `effigy docs:contract-drift` | 0 | — |
| `bun -e 'import("./packages/core/src/index.ts").then(m=>console.log(Object.keys(m).length))'` | 0 | 322 exports; deleted symbols gone, index links |
| `git diff --check` | 0 | — |
| `git status --porcelain` | 0 | writable paths only |

`effigy test:components` is **expected to go red** (029's to fix): baseline
974 passed / 69 files / exit 0; after this card **30 failed / 944 passed
(974)**, 4 files — `packages/svelte/components/test/HistoryCenter.test.ts`,
`packages/react/components/test/HistoryCenter.test.tsx`, the React smoke
sweep's HistoryCenter row and the Svelte↔React anatomy-parity HistoryCenter
row — all exercising the retired v2 API (`branches`/`paths` props,
`historyCenterRows`-shaped rows). Recorded; not chased.

## 7. Notes for 029 and the authority

- A run page's `rootContinuationCount` reports the **history root's**
  continuation count, not the run anchor's (verified in `projection/project.rs`
  `project_lineage_page` — `child_ids(None)`). The mirror carries it; the
  derivation never reads it for a run. Do not build run-level affordances on
  it.
- A single fork off a run's last entry reads `forkCount 0` under R4 (the
  authority's "a run's last entry is always zero" holds only when the run's
  last entry has no children). Such a fork is graph-possible but not reachable
  through the disclosure model — recorded in PAPERCUTS for the authority.
