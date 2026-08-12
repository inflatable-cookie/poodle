# 11 — g13.023 HistoryCenter v2 Tree Stitcher And Machine (batch log)

Branch: `thread/g13-023-history-center-tree-core`
Date: 2026-08-11
Card: `docs/roadmaps/g13/batch-cards/023-history-center-tree-core.md`
Status: **DELIVERED** — stitcher + machine rewritten in `poodle-core`, all
required tests green, contract updated. Rendering, lane styling, and
specimens are card `024`; the expected `test:components` failure is recorded
(§5).

## 1. Record types — `packages/core/src/history-center.ts`

- `HistoryEntry`: removed `branchCount` (D1); added optional `recordedAtMs`
  (D2 — modelled so the data can arrive later; nothing renders or invents it,
  and there is no `Date.now()` anywhere in the module).
- `HistoryBranch`: added `headEntryId?`, `divergedAfterEntryId?`, and
  optional `recordedAtMs`; kept `id`, `name`, `annotation`, `entryCount`,
  `current`, `pinned`. The two ids are optional to stay structurally
  compatible with the authority's `ForkBranchProjection`
  (`head_entry_id: Option<HistoryEntryId>`,
  `divergence_entry_id: Option<HistoryEntryId>` — verified read-only in
  `~/Dev/projects/longhorn/prototypes/history-tree/src/projection.rs:79-112`).

## 2. The stitcher — `historyCenterRows(branches, paths)`

Pure, exported, DOM-free, fetch-free (D5). Spine = the `current` branch
(fallback: first supplied branch); its path renders at depth 0. Every other
branch attaches its run — caption row + unique entries — immediately after
the **deepest entry its path shares with the already-stitched tree**, in
supplied order (D4). Dedupe by `entryId`; a run never re-emits shared
entries. Attach points are computed from the paths alone:
`divergedAfterEntryId` is carried on the record but never used for
attachment — that divergence-id pinning is the v1 bug this card removes
(Longhorn computes the divergence relative to the current branch, so
fork-off-fork runs inherit a coarse divergence id; the stitcher's
already-stitched-set rule nests them at their true position instead).

- Row model: `{ kind: "entry"; index; branchId; entry; depth; lane }` and
  `{ kind: "caption"; index; branch; depth }`. `index` always equals the
  row's position in the returned array. Entry rows carry the branch owning
  their run; spine rows carry the current branch id.
- Lane metadata (`HistoryRowLane`): `branchId`, `parentBranchId` (null for
  spine/root-attached), `start`, `continue`, `end` — the structural minimum
  for a renderer to draw lanes without re-deriving (card `024` draws).
- Depth cap (D3): `HISTORY_TREE_DEPTH_CAP = 3`; deeper runs render flat at 3,
  keeping true `branchId` and lane structure.
- Empty branch head: **omitted entirely** (no caption, no rows) — attachment
  is defined by path prefix sharing, so an empty path has no position, and
  divergence-id attachment would reintroduce the v1 collapse. Asserted in a
  test and recorded in the contract.
- Root-attached runs (path shares nothing with the placed structure, e.g. a
  spine page that starts below the branch's divergence) attach before the
  spine at depth 0 — the authority's "or root" case.
- Order-dependence is deliberate and documented (D4): fork-off-fork runs must
  be supplied after the run they attach to (the authority's natural listing
  order); same input always yields the same output (determinism test).

## 3. Machine rewrite

- Retired: `TOGGLE_BRANCHES`, `EXPAND_BRANCHES`, `COLLAPSE_BRANCHES`,
  `CHECKOUT`, `expandedBranchIds`, `isForkPoint`, `emitCheckout`,
  `emitSelectEntry`, and the `HistoryCenterRow` "branch" row kind (D1).
- `ACTIVATE_ROW` on an entry emits `emitNavigateEntry(branchId, entryId)` —
  always the clicked row's own branch and entry, never an ancestor or
  another branch's divergence entry. Captions are focusable (in the roving
  traversal) for rename but never navigate: activation syncs focus and emits
  nothing.
- Keyboard traversal stays linear in visual order across spine and runs
  (captions included); `FOCUS_MOVE` wraps, `first`/`last` land on
  boundaries; `historyCenterKeydownEvent` unchanged.
- Context: `{ branches, paths, focusIndex, rejection }` —
  `branches: null`/`paths: null` → no rows, every row event inert.
- Rejection (`SHOW_REJECTION`/`DISMISS_REJECTION`) and rename
  (`RENAME` → `emitRenameBranch`, `maxBranchNameBytes` stays a client-side
  affordance) unchanged.

## 4. Contract — `docs/contracts/components/history-center.md`

Updated the record types (§3 Data Shapes), the Behavior Machine section
(§4: context, row model with the depth cap and its rationale, the
empty-branch-head decision, the retired expansion events, the new
`emitNavigateEntry` transitions/effects, part-attribute removal of the fork
indicator). Added `024`-owned notes at the top of §3 (Public Props /
callback rebind), §4 States (visual/component states are v1 rendering), and
§5 Events (`onSelectEntry`/`onCheckout` retire into a single navigate
callback) — those sections' rebind is `024`'s. "Known Deltas" untouched.

## 5. Validation

| Command | Exit | Notes |
|---|---|---|
| `git status --porcelain` (baseline) | — | clean at HEAD |
| `git diff --check` (baseline) | 0 | clean |
| `effigy test:core` (baseline, after `bun install`) | 0 | 463 tests / 0 fail. First run at HEAD failed 1 test / 1 error: `Cannot find package 'marked'` in `packages/core/test/markdown-blocks.test.ts` — `marked` (declared in `packages/core/package.json`) was not installed in this worktree; `bun install` (234 packages, exit 0) fixed the environment, not code. |
| `effigy docs:lint` (baseline) | 0 | 171 component contracts |
| `effigy test:components` (baseline) | 0 | 898 passed at HEAD |
| `effigy test:core` (post-change) | 0 | **466 tests / 0 fail** (was 463; +3 net: stitcher suite replaces the v1 row tests, machine tests trimmed) |
| `effigy docs:lint` (post-change) | 0 | 171 contracts |
| `effigy docs:contract-drift` (post-change) | 0 | Public Props table untouched (024-owned), so drift is clean |
| `effigy test:components` (post-change) | 1 | **expected red — recorded, not chased.** See below. |
| `git diff --check` | 0 | clean |
| `git status --porcelain` | — | only the writable paths (§6) |

`test:components` failure (expected, `024`'s to fix): **8 failed / 890 passed
(898)** — exactly the two HistoryCenter suites, 4 in
`packages/svelte/components/test/HistoryCenter.test.ts` and 4 in
`packages/react/components/test/HistoryCenter.test.tsx`, all exercising the
retired v1 API ("expands fork points into branch rows and emits checkout",
"selects an entry", inline-rename wiring, arrow-key navigation against
`expandedBranchIds`). Recorded; not chased.

Secondary hazard, recorded for `024`: `packages/core/src/index.ts` (not in
this card's writable set) still re-exports `isForkPoint` from
`history-center.ts`. Vitest is lenient here, so the component gate shows only
the HistoryCenter suites red — but a plain bun/Node ESM import of
`@inflatable-cookie/poodle-core` validates re-export bindings at link time
(probe-verified: `export 'missing' not found`), so any consumer importing the
package index would crash until `024` drops the stale re-export (and adds the
new machine surface it needs). Recorded in PAPERCUTS.md (§7).

## 6. Changed paths (writable set only)

```
 docs/contracts/components/history-center.md               | record types + Behavior Machine + 024-owned notes
 docs/logs/2026-08/11-g13-023-history-center-tree-core.md  | this log
 packages/core/src/history-center.ts                       | v2 records, stitcher, machine
 packages/core/test/history-center.test.ts                 | v2 tests (9 stitcher + machine transitions)
 PAPERCUTS.md                                              | index.ts dangling re-export entry (§7)
```

No `HistoryCenter.svelte` / `HistoryCenter.tsx` / `index.ts` / roadmap /
card-status changes; no `git add -A`.

## 7. Notes

- Papercut (new, in PAPERCUTS.md): `packages/core/src/index.ts` hard-lists
  every `history-center` export, so a card deleting a core export (D1) must
  leave a dangling re-export that fails the whole package at module load
  until the follow-up card fixes `index.ts` — and the follow-up card's
  writable set must include `index.ts` or the same wall recurs. Consider
  making `index.ts` re-export blocks (`export { ... } from "./module"`)
  writable on any card that deletes a core export, or switching those blocks
  to wildcard re-exports so deletions stop breaking the package index.
- Longhorn ground truth used (read-only): `projection.rs:108-112`
  (`divergence_entry_id` doc) and `projection.rs:238-248` (divergence =
  last-shared entry computed relative to the current branch's lineage —
  `shared.checked_sub(1)`), confirming the v1 collapse mechanism and that the
  stitcher must not attach to divergence ids.
- Pre-existing `tsc`-only failures in `packages/core` (e.g. `icons/index.ts`)
  are untouched; no step-8 command runs bare `tsc`.
