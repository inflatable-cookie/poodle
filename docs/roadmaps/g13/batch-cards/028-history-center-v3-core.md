# 028 HistoryCentre v3 — Flat List, Node-Owned Forks (Core)

Status: merged (`680f3c64` → `2a6d3af9`)
Milestone: side-quest (component architecture, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-028-history-center-v3-core`
Depends on: `g13-b024` merged (`3580399d`)
Governing refs: `docs/architecture/006-headless-core-and-machine-model.md`,
`docs/contracts/components/history-center.md`,
`docs/specs/062-headless-core-and-dual-layer-strategy.md`,
`packages/core/src/tree.ts` (the precedent this card follows)

## Goal

Replace the v2 stitcher with a flat list that owns forks on the entry. This
card delivers **core only** — record types, the visible-row derivation, and the
machine. Rendering, the picker, the nested list, specimens and the contract are
card `029`.

## Why v2 Goes

v2 stitches branch records plus per-branch paths into one topological row list.
`childrenOf` is a `Map<entryId, branchId[]>`, so N runs attach at one entry, and
`emitRun` writes them back to back at the same depth.

**Be accurate about the defect.** An earlier statement of it — including in this
card's first version and in the source handoff — said the two shapes produce
byte-identical rows. That is too strong, and the Longhorn thread corrected it in
`db9ac5c7`. Measured on merged main:

| Case | `cap:horns` / `H1` |
|---|---|
| Two forks at one entry | `depth=1`, `parentBranchId=main` |
| `horns` forks off `lead` | `depth=2`, `parentBranchId=lead` |

So the row data *does* separate them. What is true is narrower, and still
decides the redesign:

- Both cases land at the same position in the array.
- Only `depth` and `lane.parentBranchId` separate them — one indent step, plus a
  field indentation never shows.
- Past `HISTORY_TREE_DEPTH_CAP` the depth saturates and that step disappears
  too. Measured: a five-deep chain gives `f3` and `f4` both `depth=3`, leaving
  `parentBranchId` as the only difference.

The encoding is weak. The data is not wrong. The redesign does not rest on that
claim anyway — it rests on the fact that several forks at one node are ordinary,
and a tree cannot draw that unambiguously whoever computes the rows.

Two defects found on merged main come from the same attachment logic:

1. The stitcher needs the oldest entry first. Reversed input silently drops
   branches.
2. A branch whose path shares no prefix disappears through the same code path
   as a legitimate empty head. No error, no finding.

v3 computes no attachment, so both disappear. **Do not carry either behaviour
forward.** If v3 cannot place a row, that is a defect. Raise it; never omit.

## Fixed By Ruling (do not re-decide)

### R1 — Core flattens. The renderer never recurses.

Core derives **one flat array of visible rows**, each with a `depth` number.
Both runtimes render one loop. No `svelte:self`, no self-import, no recursive
React component.

This overrides the handoff's "literally the root list component". The evidence
is `packages/render/src/tree.rs`: `push_rows` walks the node tree and pushes
rows into one flat vector with a `depth` argument, and the doc comment at `:54`
says the renderer "knows the flattened visible order". **HistoryCentre needs
native parity and it is already queued.** GPUI and Jetstream consume a
`poodle-node` tree from `poodle-render`; a component that renders itself has no
native counterpart, and a flat row list with a depth number already has one.

`packages/core/src/tree.ts` `flattenVisibleTreeRows` is the in-repo precedent.
Follow its shape.

**Condition on this ruling.** Depth alone is not enough — that is exactly what
the measurement above shows. Every row must also carry **the entry it hangs off
and the fork it belongs to**, as identifiers, not as indentation. With the depth
cap gone and parent identity on every row, the v2 ambiguity cannot return in
any renderer.

The handoff's requirement still holds, and holds better: the renderer receives
a depth number and knows nothing about topology. Core knows it, which is core's
job.

### R2 — No Longhorn dependency. Data in, commands out.

The handoff describes `controller.loadContinuations`,
`controller.loadContinuationRun` and `controller.preferContinuation`. **Poodle
cannot call any of them.** The dependency runs Longhorn → Poodle, and
`history-center.md:20-24` states no Longhorn dependency is possible.

So Poodle re-declares the record shapes **structurally**, exactly as v1 and v2
already do for entries, and takes the three operations as caller-supplied
callbacks. The host wires them to its controller. This is the MessageCenter
pattern the component already follows.

Do not add `@inflatable-cookie/longhorn` to any manifest. Do not import a
Longhorn type. The handoff's "regenerate or bump `@inflatable-cookie/longhorn`
before starting" applies to Loophole, not here.

### R3 — Core owns display order, and reverses once.

Longhorn's path pages are **newest-first**: `entries[0]` is newest and `offset`
counts from the newest. The component displays **newest last**, which is what
v2 shipped and what the screenshot shows.

Core performs that reversal, so every level reverses identically and no caller
can get it wrong. A nested run is the same page type, so it reverses by the
same code.

`continuations` is **not** reversed. It is in stable graph order — a picker,
not a timeline.

**Page joins follow from this, and the worker must get it right.** `offset`
stays newest-first, so the page at a higher offset holds **older** entries.
After the reversal, a later-fetched page therefore renders **before** the first
page, not after it. Joining pages in fetch order is the obvious mistake and it
puts history backwards. Cover it with a test that joins two pages and asserts
the oldest entry is first.

### R4 — `forkCount = continuationCount - 1`.

`continuationCount` counts every continuation **including** the one that is the
next row. A run's last entry carries `0`. `rootContinuationCount` is the same
fact one level above the first entry.

The continuations page also returns the child already on the list. Filter it
out by id. Never assume its position.

### R5 — Core holds only what is open.

Core keeps the loaded continuations and the loaded run for **currently open**
entries, and drops each on close. This is not the deleted `paths` field, which
held every branch's path at all times. Nothing is cached across a close/reopen;
the handoff is explicit that these pages are not cached and not refreshed.

### R6 — Rename survives, in the opened region.

The handoff removes caption rows from the main list and says a fork's name
lives in the picker and the opened region. It does not remove rename. Keep
`emitRenameBranch` and `maxBranchNameBytes` as they are — a client-side
affordance that enforces no protocol rule — and surface rename in the opened
region. Do not invent a new rename path and do not silently drop the capability.

## Scope

### In scope — `packages/core/src/history-center.ts` and its tests

- **Record types**, structural mirrors of the authority's shapes: an entry
  record carrying `continuationCount` and optional `recordedAtMs`; a path page
  carrying entries, `offset`, `rootContinuationCount` and both truncation
  flags; a continuation record carrying `entryId`, `label`, `recordedAtMs`,
  `preferred`, `entryCount`, `branchId`, `branchName`.
- **The visible-row derivation** — a pure exported function over the root page
  plus open state, returning a flat array in display order with `depth`.
  Row kinds at minimum: an entry row, a picker row (only when `forkCount > 1`),
  and a not-yet-loaded row for an open entry whose run has not arrived.
- **The machine.** `historyCenterTransition` keeps popover open state, keyboard
  traversal and transient rejection display. Disclosure state is new. Replace
  `focusIndex` with the visible row identity, recomputed when a disclosure
  toggles — an index into a list that changes shape underneath it is the bug
  waiting to happen.
- **Effects** for the three host operations and for confirm, plus the existing
  navigate and rename effects.
- **Rejection handling** for `AlreadyAtTarget` and `UnknownEntry`.

### Delete

`historyCenterRows`, `historyCenterRowCount`, `HistoryCenterRow`,
`HistoryRowLane`, `HISTORY_TREE_DEPTH_CAP`, and the `paths` context field.
Remove each from `packages/core/src/index.ts` in the same commit — a package
index that re-exports a deleted symbol fails at link time, which `b023` already
cost us once.

### Out of scope — stop conditions if reached

- Any rendering: `packages/svelte`, `packages/react`, any CSS, any specimen.
  Card `029`.
- Native adapters. A later card, after `029`.
- Any Longhorn or Loophole file.
- Real fetching. Core receives pages; it never calls anything (R2).
- A depth cap. v2 had one and it hid nesting. Depth is a number the renderer
  uses; do not saturate it.

## Required Tests

- Two forks at one entry are **not** confusable with a fork off a fork. Assert
  the two cases produce different rows. This is the regression test for the
  defect v3 exists to fix.
- `continuationCount: 1` yields no fork affordance and no picker row.
- `continuationCount: 0` on a run's last entry yields no fork affordance.
- `forkCount > 1` yields a picker row; `forkCount === 1` yields none.
- The continuations page includes the child already on the list; the derivation
  filters it by id.
- Display order: a newest-first page renders oldest-first, at the root and in a
  nested run, by the same code.
- An open entry with no loaded run yields a not-yet-loaded row, never an empty
  gap and never a dropped entry.
- Closing an entry drops its loaded run from state (R5).
- Traversal survives a disclosure toggle: focus stays on the same row identity,
  not the same index.
- `recordedAtMs` absent yields no time. Nothing invents a clock.
- Two pages join with the older page first, and the oldest entry renders first
  (R3). This is the paging trap.
- Every row carries its parent entry id and its fork identity, at every depth,
  including past where a v2 depth cap would have saturated (R1 condition).
- Determinism: same input, same rows, same order.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `packages/core/src/tree.ts` first — `flattenVisibleTreeRows` is the
  shape to follow — then the current `history-center.ts`, then
  `docs/architecture/006-headless-core-and-machine-model.md`.
- Core stays pure: no DOM, no timers, no `Date.now()`, no fetching.
- `packages/core/src/index.ts` is writable **only** to remove deleted exports
  and add new ones.
- `effigy test:components` will go red once the machine changes. That is
  expected and is `029`'s to fix. Record the count; do not chase it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-028-history-center-v3-core`. Do not merge.

## Writable Paths

- `packages/core/src/history-center.ts`
- `packages/core/src/index.ts` (export list only)
- `packages/core/test/history-center.test.ts`
- `docs/contracts/components/history-center.md` — record types, row model and
  machine sections only. Mark any rendering section `029`-owned.
- `docs/logs/2026-08/<DD>-g13-028-history-center-v3-core.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy test:core`, `docs:lint`, `git diff --check`,
   `git status --porcelain`. Record exit states and the current
   `test:components` count.
2. Read `tree.ts`, the current machine, and the architecture doc.
3. Record types, then the visible-row derivation, then the machine.
4. Delete the v2 surface and its `index.ts` exports together.
5. Tests — every row above.
6. Contract: record types, row model, the machine, and the ordering ruling.
7. Validate:
   ```sh
   effigy test:core
   effigy docs:lint
   effigy docs:contract-drift
   bun -e 'import("./packages/core/src/index.ts").then(m=>console.log(Object.keys(m).length))'
   git diff --check
   git status --porcelain
   ```

## Acceptance Criteria

- [ ] The derivation is pure, exported, flat, and carries `depth`. No recursion
  in any renderer-facing shape.
- [ ] Two forks at one entry differ from a fork off a fork, proven by test, and
  the difference survives without any depth cap.
- [ ] Every row carries parent entry id and fork identity as data, not as depth.
- [ ] Pages join oldest-first after the reversal, proven by test.
- [ ] Nothing is ever silently dropped. An unplaceable row is impossible or
  loud.
- [ ] `historyCenterRows`, `historyCenterRowCount`, `HistoryCenterRow`,
  `HistoryRowLane`, `HISTORY_TREE_DEPTH_CAP` and `paths` are gone from the
  module **and** from `index.ts`, and the package index still links.
- [ ] No Longhorn import, no Longhorn manifest entry.
- [ ] No `Date.now()`; no depth cap.
- [ ] Traversal keyed on row identity, not index.
- [ ] All step-7 commands exit 0.
- [ ] Batch log records the expected `test:components` failure count.

## Stop Conditions

- A row cannot be placed from the supplied pages. Give the input and say what
  is missing — do not omit it.
- Rename cannot live in the opened region without a new protocol rule.
- The visible-row derivation needs data the host cannot supply without a
  Longhorn type reaching Poodle.

Stop with exact paths, commands, and the smallest unresolved question.
