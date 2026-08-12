# 032 HistoryCentre — Rename The Selected Fork

Status: ready
Milestone: side-quest (component behaviour, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-032-history-center-multi-fork-rename`
Depends on: `g13-b030` merged (`c6590823`). Runs alongside `g13-b031`, which
owns `Select`, `OrderBy` and `FilterBuilder` and must not touch HistoryCentre —
these two do not overlap.
Governing refs: `docs/contracts/components/history-center.md`

## Goal

The multi-fork picker cannot rename a fork. The single-fork path can. Add the
same pencil affordance to the picker row, renaming **the fork currently
selected in the `Select`**.

## The Gap

`b028`'s R6 kept rename and put it in the opened region, and the single-fork
path has it: `HistoryCenter.svelte` renders a run header with the branch name,
its entry count, and a pencil button (`data-part="run-header-rename"`,
`Icon name="edit"`) that swaps in an inline input.

The multi-fork path renders a `Select` and a checkout `IconButton`, and no
rename at all. So a fork's name is editable only when it is the sole
continuation — exactly backwards, since naming matters more when there are
several to tell apart.

## Fixed By Ruling (do not re-decide)

### R1 — Pencil sits left of checkout, and renames the selection.

Picker row order: **`Select`, pencil, checkout**. The pencil renames the branch
of whichever fork the `Select` currently shows — not the preferred one, not the
anchor's own branch.

### R2 — Reuse the rename machinery. Do not build a second one.

`startRename`, `commitRename`, `finishRename`, the `RENAME` event and the
`emitRenameBranch` effect all exist and work. The picker path uses them
unchanged. `maxBranchNameBytes` stays a client-side affordance that enforces no
protocol rule.

Adding a parallel rename path is the drift this project exists to stop.

### R3 — The input replaces the `Select` while renaming.

While a rename is open, the inline input takes the `Select`'s place, seeded
with the selected fork's current name. Checkout is disabled during the rename —
committing a name and switching the root list are different intentions and
should not be one click apart.

On commit or cancel the `Select` returns and focus goes back to the pencil.
`finishRename` already restores focus this way for the single-fork path; follow
it.

### R4 — Match the checkout button's component.

The new pencil is an `IconButton`, like the checkout button beside it.

Leave the single-fork run-header rename as the raw `<button>` it is today. It
is a different region with its own styling, the user is content with it, and
changing it is an unrequested visual change. Record the inconsistency in the
batch log as a follow-up rather than fixing it here.

### R5 — Everything v3 holds.

One loop over `historyCenterVisibleRows`. No `svelte:self`, no self-import.
`depth` drives padding only. No Longhorn import, no `fetch`. Svelte first, React
mirrors exactly.

## Scope

### In scope

- The picker row in both web runtimes: pencil `IconButton` between the `Select`
  and checkout, plus the inline rename state.
- `history-center.css` for the new control and the rename-in-picker layout.
- Both test suites.
- Both specimen files — the two-forks group should show a rename in progress,
  since that is the state this card adds.
- Contract: the picker section gains the rename affordance and its rule.

### Out of scope — stop conditions if reached

- `Select`, `OrderBy`, `FilterBuilder`. `b031` owns those; touching them will
  conflict.
- The single-fork run-header rename (R4).
- `packages/core/src/history-center.ts` — the machine already carries `RENAME`
  and `emitRenameBranch`. If the picker genuinely needs a core change, stop and
  say what is missing.
- Native adapters, any Longhorn or Loophole file, refreshing baselines.

## Required Tests

- The picker renders a pencil between the `Select` and checkout when
  `forkCount > 1`.
- Renaming from the picker emits `onRenameBranch` with **the selected fork's**
  branch id, not the anchor's and not the preferred one.
- Changing the `Select` and then renaming targets the newly selected fork.
- Checkout is disabled while a rename is open.
- Cancelling restores the `Select` and returns focus to the pencil.
- `forkCount === 1` is unchanged — still the run-header rename, no picker.
- Both runtimes render the same anatomy.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read the existing rename path in `HistoryCenter.svelte` first —
  `startRename`, `commitRename`, `finishRename`, and the
  `data-part="run-header-rename"` markup. Reuse it.
- **Run `effigy check:svelte`.** Not optional.
- Do not weaken a test to make it pass.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-032-history-center-multi-fork-rename`. Do not
  merge.

## Writable Paths

- `packages/svelte/components/src/HistoryCenter.svelte`
- `packages/react/components/src/HistoryCenter.tsx`
- `packages/{svelte,react}/components/test/HistoryCenter.test.*`
- `packages/svelte/components/test/HistoryCenterHostHarness.svelte`
- `packages/core/src/styles/history-center.css`
- `packages/{svelte,react}/preview/src/**/HistoryCenterSpecimen.*`
- `docs/contracts/components/history-center.md`
- `docs/logs/2026-08/<DD>-g13-032-history-center-multi-fork-rename.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy test:core`, `test:components`, `check:svelte`,
   `docs:lint`, `git diff --check`. All start green.
2. Read the existing rename path.
3. Svelte: pencil `IconButton`, rename-replaces-`Select`, focus return.
4. Mirror React exactly.
5. CSS and specimens.
6. Contract.
7. Validate:
   ```sh
   effigy test:core
   effigy test:components
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   effigy docs:contract-drift
   effigy svelte:surface-audit
   git diff --check
   ```

## Acceptance Criteria

- [ ] Pencil sits between the `Select` and checkout, and renames the selected
  fork — proven by a test that changes the selection first.
- [ ] The existing rename machinery is reused; no second rename path exists.
- [ ] Checkout is disabled during a rename; cancel restores the `Select` and
  the focus.
- [ ] The single-fork path is unchanged.
- [ ] A specimen shows a rename in progress in the two-forks group.
- [ ] All step-7 commands exit 0; no baseline refreshed.

## Stop Conditions

- The selected fork's branch id is not reachable where the picker renders.
- Reuse of the rename machinery forces a core change.
- The rename input cannot replace the `Select` without breaking keyboard
  traversal.

Stop with exact paths, commands, and the smallest unresolved question.
