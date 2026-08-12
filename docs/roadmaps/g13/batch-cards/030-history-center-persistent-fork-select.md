# 030 HistoryCentre — Persistent Fork Select

Status: ready
Milestone: side-quest (component behaviour, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-030-history-center-fork-select`
Depends on: `g13-b029` merged (`2a6d3af9`)
Governing refs: `docs/contracts/components/history-center.md`,
`packages/svelte/components/src/Select.svelte`

## Goal

The fork picker must be a **persistent `Select`**, and the selected fork's
entries must render **under it**. Today the picker disappears the moment you
choose, and it is a hand-rolled button list rather than the `Select` component.

## The Defect

Reported from a live screenshot: *"when I open a forked node, I only see the
forks themselves, and once I've selected one there's no way back to that
selector without closing the parent node and reopening it."*

Two causes, one in each layer. Both are confirmed in the merged code.

**Core drops the picker after a choice.**
`packages/core/src/history-center.ts`, `pushDisclosed`:

```ts
if (level.chosen === null) {
  if (historyCenterForkCount(entry.continuationCount) > 1) {
    rows.push({ kind: "picker", … });
```

The picker row exists only while nothing is chosen. After `CONFIRM` sets
`chosen`, the row is gone and only the run renders. That is the "no way back".

**The renderer hand-rolls the picker.**
`HistoryCenter.svelte` renders `row.continuations` as a `<button>` list with a
`Preferred` badge and a `Button` labelled "Choose". The v3 handoff asked for a
select and a confirm `IconButton`. Poodle ships a `Select`; the component
should use it.

## Fixed By Ruling (do not re-decide)

### R1 — The picker persists while the fork is open.

Emit the picker row whenever the level is open and `forkCount > 1`, whatever
`chosen` holds. Remove the `level.chosen === null` gate. The picker shows the
current selection and stays reachable, so a second fork is one interaction
away, not a close-and-reopen.

`forkCount === 1` still renders no picker. One continuation is nothing to
choose between — that rule is unchanged.

### R2 — Selection previews. Confirm commits.

These are two different actions and the current code conflates them.

- `PICK_CONTINUATION` sets the selection **and** shows that fork's entries
  below the select. It loads the run if it is not loaded. It applies nothing
  and emits no `preferContinuation`.
- `CONFIRM` emits `preferContinuation` for the selected fork. It does not
  change what is displayed.

Today the run only appears after `CONFIRM`, which is why choosing feels modal.
The handoff is explicit: the nested list shows "the entries of the selected
fork".

### R3 — Open selects the preferred fork.

On disclosing an entry with `forkCount > 1`, once continuations arrive, select
the one with `preferred: true` and show its run. The operator sees the current
future immediately, and the select shows what they are looking at.

If no continuation is preferred, select the first in supplied order. Do not
leave the region empty.

### R4 — Use `Select` and `IconButton`.

Replace the hand-rolled option list with `Select`, and the "Choose" `Button`
with a confirm `IconButton` beside it. `Select` takes `value`, `options`,
`size`, `density` and `variant` — read
`packages/svelte/components/src/Select.svelte` for the surface.

Keep the branch name and the `preferred` marker visible in the option labels;
losing them would lose information the screenshot shows today.

Confirm stays disabled when the selected fork is already `preferred`. That is
what keeps `AlreadyAtTarget` a race rather than a normal path.

### R5 — Everything else v3 holds.

One loop over `historyCenterVisibleRows`. No `svelte:self`, no self-import, no
recursive component. `depth` drives padding only. No Longhorn import, no
`fetch`. Svelte first, React mirrors exactly.

## Scope

### In scope

- `packages/core/src/history-center.ts`: the `pushDisclosed` gate (R1), and the
  `PICK_CONTINUATION` / `CONFIRM` split (R2), and the open-selects-preferred
  behaviour (R3).
- Both web components: `Select` + confirm `IconButton` (R4).
- `history-center.css`: picker-option styles give way to whatever `Select`
  needs; keep the recipe-hook convention.
- Both test suites and both specimen files.
- Contract: the picker section, the row model note on persistence, and the
  select/confirm split.

### Out of scope — stop conditions if reached

- The flat row model, `parentEntryId`/`forkId`, or the depth handling. v3
  settled those and they are not the defect.
- `precedingContinuationCount` semantics.
- Native adapters.
- Any Longhorn or Loophole file.
- Refreshing visual baselines.

## Required Tests

- The picker row survives a choice: disclose, pick, and the picker is still
  present with the new selection shown.
- Picking a different fork swaps the entries below without closing anything.
- `PICK_CONTINUATION` emits no `preferContinuation`.
- `CONFIRM` emits `preferContinuation` for the selected fork and does not
  change the displayed run.
- Disclosing selects the preferred fork and shows its run (R3), and falls back
  to the first when none is preferred.
- `forkCount === 1` still renders no picker.
- Confirm is disabled when the selection is already `preferred`.
- Both runtimes render the same anatomy (the parity suite covers this).

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `packages/core/src/history-center.ts` and `Select.svelte` before
  writing.
- **Run `effigy check:svelte`.** It is in the validation list below and it is
  not optional. It found 17 type errors in this component's core that
  `test:core` could not see, because an earlier card left it out.
- Do not weaken a test to make it pass.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-030-history-center-fork-select`. Do not merge.

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
- `docs/logs/2026-08/<DD>-g13-030-history-center-fork-select.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy test:core`, `test:components`, `check:svelte`,
   `docs:lint`, `git diff --check`. Record exit states. All start green.
2. Read the core module and `Select.svelte`.
3. Core: R1, then R2, then R3, with tests for each.
4. Svelte: `Select` + confirm `IconButton`.
5. Mirror React exactly.
6. CSS and specimens — a specimen must show a chosen fork **with the select
   still visible**, which is the reported defect.
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

- [ ] The picker persists after a choice, proven by test.
- [ ] Picking previews; confirm commits. Proven by two separate tests on the
  effects each emits.
- [ ] Disclosing selects the preferred fork and shows its run.
- [ ] The picker is a `Select` with a confirm `IconButton`; no hand-rolled
  option list remains.
- [ ] Branch name and preferred marker still visible.
- [ ] A specimen shows a chosen fork with the select still present.
- [ ] All step-8 commands exit 0, `check:svelte` included.

## Stop Conditions

- `Select` cannot carry the option content this needs. Say what is missing.
- Making the picker persistent breaks keyboard traversal or focus.
- Preview-on-pick needs a core structure v3 does not have.

Stop with exact paths, commands, and the smallest unresolved question.
