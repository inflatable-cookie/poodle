# 023 HistoryCenter v2 — Tree Stitcher And Machine

Status: ready
Milestone: side-quest (component architecture, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-023-history-center-tree-core`
Depends on: `g13-b019` merged (`1670ae54`)
Governing refs: `docs/architecture/006-headless-core-and-machine-model.md`,
`docs/contracts/components/history-center.md`,
`docs/specs/062-headless-core-and-dual-layer-strategy.md`

## Goal

Replace HistoryCenter's fork-expander model with a unified history tree, in
`poodle-core` only. This card delivers the **stitcher and the machine**. Web
rendering, lane styling, and specimens are card `024`.

Splitting here is deliberate: the stitcher is pure, fully testable without a
DOM, and is the piece hosts must not reimplement. It is also the part most
likely to be got wrong.

## Why v2 Exists — read this before designing

v1 rendered branch **name** rows pinned to a divergence id. Longhorn computes
divergence *relative to the current branch*
(`prototypes/history-tree/src/projection.rs:108`, "Returns divergence node
relative to the current branch, or root"), so several genuinely different forks
project onto one visible entry and the UI reads as "6 branches off the root
edit" — describing nothing.

v2 renders the actual tree: every entry in the fork graph, exactly once, at its
true position. A fork run's **content** is its description; branch names are
captions, not the payload.

## Fixed By Ruling (do not re-decide)

- **D1 — Clean break. No deprecation shim.** The handoff allows one "if cheap";
  it is not cheap. Rows, events, effects, and callbacks all change together,
  and a shim would mean keeping v1's row-index math alive beside the tree.
  HistoryCenter merged on 2026-08-11 (`1670ae54`), has never appeared in a
  published release (the registry's 0.1.0 predates it), and has exactly one
  consumer, which will rebind. Delete `TOGGLE_BRANCHES`, `EXPAND_BRANCHES`,
  `COLLAPSE_BRANCHES`, `expandedBranchIds`, `isForkPoint`, and
  `HistoryEntry.branchCount`.
- **D2 — No client-side clocks.** Verified: the entire Longhorn history-tree
  domain has no recorded-at field — `ForkBranchProjection` carries
  `branch_id`, `head_entry_id`, `divergence_entry_id`, `name`, `annotation`,
  `pinned`, `current`, and the only `Instant` in the crate is in a benchmark
  binary. Model `recordedAtMs?: number` on the record types so the data can
  arrive later, render nothing when absent, and **never** call `Date.now()`.
- **D3 — Visual depth cap is 3.** Past depth 3, continue rendering flat at
  depth 3 rather than indenting further. The row still carries its true
  `branchId`, so navigation is unaffected — only indentation saturates.
- **D4 — Order is supplied, not invented.** The stitcher emits topological
  order. Whether the host displays oldest-first or newest-first is the host's
  existing choice and this card does not change it: the current component
  renders `entries` in supplied order and the contract fixes no direction.
- **D5 — The stitcher never fetches.** It is a pure function over data the
  caller supplies. It must not know about Longhorn, ports, or paging
  mechanics.

## Scope

### In scope — `packages/core/src/history-center.ts` and its tests

- **Record types.** Extend `HistoryBranch` with `headEntryId`,
  `divergedAfterEntryId`, and optional `recordedAtMs`. Add `recordedAtMs?` to
  `HistoryEntry`. Remove `branchCount`.
- **The stitcher**, a pure exported function taking branch records plus a
  per-branch entry path (`branchId -> HistoryEntry[]`) and returning ordered
  rows. Paths share ancestor prefixes: dedupe by `entryId`, and attach each
  branch's unique suffix at its last shared entry.
- **Row model:**
  - `{ kind: "entry", index, branchId, entry, depth, lane }`
  - `{ kind: "caption", index, branch, depth }`
  Every entry row carries the branch that **owns its run** — the spine's rows
  carry the current branch id.
- **Lane metadata** sufficient for a renderer to draw a git-graph lane without
  re-deriving structure: at minimum whether the row continues a lane, starts a
  run (elbow), or ends one. Card `024` draws it; do not style anything here.
- **Machine rewrite.** `ACTIVATE_ROW` on an entry emits
  `emitNavigateEntry(branchId, entryId)` — always the entry actually clicked,
  never an ancestor or divergence entry belonging to another branch. Captions
  are focusable for rename but do not navigate. Keyboard traversal stays linear
  in visual order across spine and runs.
- **Retire** the three expansion events, `expandedBranchIds`, and
  `emitCheckout`. `emitSelectEntry` collapses into `emitNavigateEntry`.
- **Rejection** state and `SHOW_REJECTION` / `DISMISS_REJECTION` unchanged.
- **Rename** unchanged — `emitRenameBranch`, and `maxBranchNameBytes` stays a
  client-side affordance that enforces no protocol rule.

### Out of scope — stop conditions if reached

- Any rendering: `packages/svelte`, `packages/react`, any CSS, any specimen.
  That is card `024`.
- Native adapters. A separate follow-up card, after `024`.
- Any Longhorn or Loophole file. Poodle does not depend on Longhorn — the
  direction is Longhorn → Poodle.
- Fetching, paging strategy, or a controller. D5.
- Importing protocol constants. `MAXIMUM_FORK_BRANCH_NAME_BYTES` stays in the
  Longhorn crate; the existing "Known Deltas" ruling holds.
- Inventing timestamps. D2.

## Required Stitcher Tests

Each is a named test:

- Linear only — no branches; output equals the spine, all depth 0.
- One fork mid-spine — the run attaches at the last shared entry, not at the
  divergence id.
- Fork off a fork — depth 2, and the inner run attaches to the outer run.
- Depth cap — a chain deeper than 3 saturates at depth 3 and keeps true
  `branchId`s (D3).
- Many shallow forks — the "6 branches" field case. Each fork is a distinct
  run; none collapse onto one entry. **This is the regression test for the bug
  v2 exists to fix.**
- Empty branch head — a branch with no entries yields a caption and no entry
  rows, or is omitted; assert whichever you implement and say which in the
  contract.
- Page-boundary split — a branch path supplied in two pages stitches into one
  run without duplicate or dropped entries.
- Shared prefix dedupe — an entry present in three paths appears exactly once.
- Determinism — same input, same output, including row order and indices.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `docs/architecture/006-headless-core-and-machine-model.md` and the
  existing `packages/core/src/history-center.ts` in full first. Match the
  file's existing doc-comment convention, which cites the contract section each
  export serves.
- The machine stays pure: no DOM, no timers, no `Date.now()`, no fetching.
- `index` on every row must equal its position in the returned array. v1's
  keyboard math depends on that and card `024` will too.
- Do not edit the contract's "Known Deltas" rulings.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-023-history-center-tree-core`. Do not merge.

## Writable Paths

- `packages/core/src/history-center.ts`
- `packages/core/test/history-center.test.ts`
- `docs/contracts/components/history-center.md` — the "Behavior Machine",
  row-model, and record-type sections only. Leave the rendering sections to
  `024`; note at the top of any section you cannot complete that `024` owns it.
- `docs/logs/2026-08/<DD>-g13-023-history-center-tree-core.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

Any other changed path is a scope failure. In particular, do not touch
`HistoryCenter.svelte` or `HistoryCenter.tsx` — they will not compile against
the new machine, and that is expected. Card `024` fixes them.

## Steps

1. Baseline: `effigy test:core`, `effigy docs:lint`, `git diff --check`,
   `git status --porcelain`. Record exit states. Note that `test:components`
   will go red once the machine changes — that is expected and is `024`'s to
   fix. Record it; do not chase it.
2. Read the machine-model architecture doc, the current machine, and the
   contract's "Behavior Machine" section.
3. Record types, then the stitcher, then the machine rewrite.
4. Tests — every row in "Required Stitcher Tests", plus machine transition
   tests for navigate, rename, rejection, and keyboard traversal across a
   spine-and-run list.
5. Contract: record types, row model, the depth cap and its rationale, the
   empty-branch-head decision, and the removal of the expansion events.
6. Validate:
   ```sh
   effigy test:core
   effigy docs:lint
   effigy docs:contract-drift
   git diff --check
   git status --porcelain
   ```

## Acceptance Criteria

- [ ] The stitcher is pure, exported, and has every test listed above,
  including the many-shallow-forks regression test.
- [ ] Entry rows carry the branch owning their run; `emitNavigateEntry` always
  reports the clicked entry, never an ancestor or another branch's divergence
  entry.
- [ ] `TOGGLE_BRANCHES`, `EXPAND_BRANCHES`, `COLLAPSE_BRANCHES`,
  `expandedBranchIds`, `isForkPoint`, `branchCount`, and `emitCheckout` are
  gone.
- [ ] No `Date.now()` and no clock anywhere; `recordedAtMs` is modelled and
  optional.
- [ ] Depth saturates at 3 while `branchId` stays true.
- [ ] Contract records the row model, the cap, and the empty-head decision.
- [ ] `effigy test:core`, `docs:lint`, `docs:contract-drift`, `git diff --check`
  all exit 0.
- [ ] Batch log records commands, exit states, and explicitly notes the
  expected `test:components` failure with the count.

## Stop Conditions

- The stitcher cannot produce a stable order from the supplied data — say what
  is missing from the record types.
- Attaching a run at "the last shared entry" is ambiguous for some input. Give
  the input and the two candidate outputs.
- Retiring `emitCheckout` appears to lose a capability rather than rename one.

Stop with exact paths, commands, and the smallest unresolved question.
