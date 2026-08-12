# 033 HistoryCentre — One Fork Row, With Delete

Status: merged (`aa0350d2`)
Milestone: side-quest (component behaviour, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-033-history-center-unified-fork-row`
Depends on: `g13-b031` (`ef12a3bf`) and `g13-b032` (`321fc790`), both merged
Governing refs: `docs/contracts/components/history-center.md`,
`packages/svelte/components/src/Select.svelte`

## Goal

Four changes, all in the opened fork region. Three make the multi-fork row
better; the fourth makes the single-fork path **use that same row**, which
retires a second layout and a second rename affordance.

Target row, both fork counts:

```
[⑂ Name  branch/name  ▾] [✎] [🗑 opt-in] [✓]
```

## Fixed By Ruling (do not re-decide)

### R1 — Fork icon in the `Select` trigger.

Render `Icon name="git-branch"` before the fork name in the trigger, the same
icon the entry row already uses for its fork indicator
(`HistoryCenter.svelte:732`). The trigger currently shows name plus branch name
and no icon.

### R2 — `variant="default"`, not `ghost`.

The picker `Select` becomes the default bordered variant. `b031` gave ghost its
chevron back, so ghost is no longer mute, but a bordered control is the right
weight for a row that owns three buttons beside it.

### R3 — The single-fork path uses the same row, with a disabled `Select`.

This **supersedes `b030`'s R1**, which said `forkCount === 1` renders no picker.
It now renders the same picker row, with the `Select` disabled because there is
nothing to choose between.

Core currently emits the picker row only when `forkCount > 1`
(`pushDisclosed`). Emit it whenever the level is open and `forkCount >= 1`, and
carry enough on the row for the renderer to disable the control — do not make
the renderer infer it from `continuations.length`.

**The run header goes away.** Everything it carries must survive into the new
row or its immediate surroundings:

- the fork name — now the `Select`'s value
- the branch name — already in the trigger
- **the entry count and the relative time** — `HistoryCenter.svelte:698-699`
  renders `N entries · 20m ago`. Say where these land and keep them; losing
  them is a regression, not a simplification.

This also retires the raw-`<button>` rename that `b032`'s R4 deliberately left
alone. That was the right call then, because the two layouts were separate.
They are not any more, so the single `IconButton` pencil now serves both and
the inconsistency resolves itself.

### R4 — Delete is opt-in, and it is a command.

A delete `IconButton` sits **between the pencil and checkout**. It renders only
when the host supplies its callback — absent callback, absent button. No
`disabled` state standing in for "unsupported".

It emits a command for the **selected** fork, exactly as checkout does. Poodle
deletes nothing itself and does not guess at the resulting history; the host
runs it and supplies new pages.

Longhorn is building the operation now, so the callback will be honoured. Do
not block on it — the prop is opt-in and Poodle's side is complete without it.
`prune_to` is budget-driven retention and is **not** this operation; do not
map to it.

Destructive, so: no confirmation inside Poodle. Confirmation is the host's
call and Poodle does not own that policy. Say so in the contract.

### R5 — Everything v3 holds.

One loop over `historyCenterVisibleRows`. No `svelte:self`, no self-import.
`depth` drives padding only. No Longhorn import, no `fetch`. Svelte first,
React mirrors exactly.

## Scope

### In scope

- `packages/core/src/history-center.ts`: the `forkCount >= 1` gate and whatever
  the row must carry so the renderer can disable the `Select` (R3), plus the
  delete effect (R4).
- Both web components: icon, variant, unified row, delete button.
- Removing the run-header path in both runtimes once the row replaces it.
- `history-center.css`: the unified row; delete the run-header rules.
- Both test suites, both specimen files.
- Contract: the unified row, the disabled single-fork `Select`, delete and its
  opt-in rule, and the no-confirmation-in-Poodle statement.

### Out of scope — stop conditions if reached

- `Select`, `OrderBy`, `FilterBuilder` internals. `b031` settled those.
- Any Longhorn or Loophole file.
- Native adapters.
- Refreshing visual baselines.

## Required Tests

- The trigger renders the `git-branch` icon before the name.
- `forkCount === 1` renders the picker row with a **disabled** `Select`, the
  pencil, and checkout.
- The entry count and relative time still render for a single fork.
- Renaming works from the single-fork row — the same pencil, the same
  machinery, no run-header button anywhere.
- No delete button renders when the host supplies no callback.
- With the callback, delete emits for the **selected** fork, and emits nothing
  else.
- Delete is between pencil and checkout in both runtimes.
- Poodle shows no confirmation dialog of its own.
- Both runtimes render the same anatomy.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read the current picker row and the run-header path before writing. The run
  header is being replaced, not deleted blind — its content must land
  somewhere.
- **Run `effigy check:svelte`.** Not optional.
- Do not weaken a test to make it pass.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-033-history-center-unified-fork-row`. Do not
  merge.

## Writable Paths

- `packages/core/src/history-center.ts`
- `packages/core/test/history-center.test.ts`
- `packages/svelte/components/src/HistoryCenter.svelte`
- `packages/react/components/src/HistoryCenter.tsx`
- `packages/{svelte,react}/components/test/HistoryCenter.test.*`
- `packages/svelte/components/test/HistoryCenterHostHarness.svelte`
- `packages/core/src/styles/history-center.css`
- `packages/{svelte,react}/preview/src/**/HistoryCenterSpecimen.*`
- `docs/contracts/components/history-center.md`
- `docs/logs/2026-08/<DD>-g13-033-history-center-unified-fork-row.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy test:core`, `test:components`, `check:svelte`,
   `docs:lint`, `git diff --check`. All start green.
2. Read the picker row and the run-header path.
3. Core: the `forkCount >= 1` gate, the disable signal, the delete effect.
4. Svelte: icon, variant, unified row, delete button; remove the run header.
5. Mirror React exactly.
6. CSS and specimens — a specimen must show the single-fork row with its
   disabled `Select`, and one must show delete present.
7. Contract.
8. Validate:
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

- [ ] One row shape serves both fork counts; the run-header path is gone from
  both runtimes and the CSS.
- [ ] Single fork renders a disabled `Select`; entry count and time survive.
- [ ] Rename works from the single-fork row through the shared pencil.
- [ ] Delete renders only with its callback, sits between pencil and checkout,
  and emits for the selected fork.
- [ ] Fork icon in the trigger; `variant="default"`.
- [ ] All step-8 commands exit 0; no baseline refreshed.

## Stop Conditions

- The entry count and time have nowhere sensible to live in the unified row.
  Say where you tried; do not drop them.
- Disabling the `Select` for a single fork breaks keyboard traversal.
- The delete command cannot name the selected fork unambiguously.

Stop with exact paths, commands, and the smallest unresolved question.
