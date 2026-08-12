# 12 — g13.034 HistoryCentre — Three Field Defects From Loophole (batch log)

Branch: `thread/g13-034-history-center-field-defects`
Date: 2026-08-12
Card: `docs/roadmaps/g13/batch-cards/034-history-center-field-defects.md`
Status: **COMPLETED** — the picker row's `disabled` now governs the `Select`
alone (R1): the actions menu lives on its own gates, so the auto-chosen
single fork stays check-outable and renamable. Defects 2 and 3 are one root
cause, fixed at the data (R2): a level whose shown fork's run is now on the
root spine is stale — the derivation never splices the duplicate run (the
not-yet-loaded row renders), and the machine drops the level's loaded data
once, keeps the anchor open, and re-requests through the existing
`loadContinuations` effect. Contract, tests and both runtimes updated; every
step-7 command exits 0.

## 0. Handover state (steps 1–2)

The previous attempt died on a provider stream closure after completing
step 2 only: the R2 repro test is landed in
`packages/core/test/history-center.test.ts` and failed on entry (54 pass /
1 fail in the core history-center file at handover). This card started at
step 3 with that exact state. The repro test is the only change that attempt
left in the working tree (`git status` clean otherwise, branch
`thread/g13-034-history-center-field-defects` on `b9090f4e`).

## 1. Core — `packages/core/src/history-center.ts`

- **Stale-level suppression in the derivation (R2).** `pushDisclosed` now
  computes the level's staleness before rendering: stale iff the shown fork
  (`pick ?? chosen`) has its first entry id in the **joined root pages** —
  a data fact, never an array-identity fact (a host that rebuilds its pages
  array each render cannot loop it). A stale level's picker renders with
  empty continuations and `pickedEntryId: null` and its run splice is
  replaced by the existing `not-yet-loaded` row — no new row kind. The empty
  picker is deliberate: it is exactly the row set the machine produces after
  the drop, so the list does not change shape at the reconcile boundary.
  `historyCenterVisibleRows` joins the root once and threads
  `rootEntries` through `pushRun`/`pushDisclosed` (nested runs included, so
  an inner level whose fork is on the spine is suppressed at any depth).
- **Machine reconcile (R2).** `historyCenterTransition` now runs
  `reconcileStaleLevels` before every open-state event except `CLOSE` and a
  closing `TOGGLE` (those drop the whole tree anyway and must not emit a
  re-request on the way out). A stale level keeps its `anchorEntryId`, drops
  `continuations`/`pick`/`chosen`/`runPages`/`inner` and emits
  `loadContinuations(entryId)` — the existing effect, no `SYNC`/`REFRESH`
  event. The drop makes the level non-stale (no shown fork), so later
  transitions emit nothing: **exactly one re-request, never per derivation**.
  The whole subtree drops with the level (its anchors live inside a run the
  host already made primary and would not be re-materialised by the
  re-requested run — keeping them would leave invisible levels that
  DISCLOSE would toggle shut). The re-requested continuations land through
  the ordinary `CONTINUATIONS_LOADED` flow, which re-filters the successor
  against the new root, so the loop condition cannot arise: the newly
  auto-chosen fork is the line just left, not the line now on the spine.
- **R1 — verified, with one machine fix.** `pickedEntryId` already falls back
  to `level.chosen?.entryId`, and the chosen/pick fork is always a member of
  the picker's filtered `continuations` (the machine sets `chosen`/`pick`
  from the `historyCenterForksAt` result, and the derivation filters with
  the same function), so `pickedContinuation` resolves the auto-chosen
  single fork. The row's `disabled` signal stays on the row — it is the
  `Select`'s gate, never the menu's; the defect was the renderer folding it
  into the menu, fixed in both runtimes. Enabling checkout on the single-fork
  row exposed a second half of the same defect: the machine's `CONFIRM` only
  committed a level with a `pick`, and a single-fork level stores its fork in
  `chosen` — so an enabled Checkout would have been a dead button. `confirm`
  now commits the **displayed fork** (`pick ?? chosen`): the auto-chosen
  single fork counts as picked, exactly as the ruling states, and checkout
  emits `checkoutContinuation(chosen.entryId)` and clears the disclosure.
  The `preferred` gate still disables the item for the current line;
  `AlreadyAtTarget` remains the race, not the normal path.

## 2. Svelte — `packages/svelte/components/src/HistoryCenter.svelte`

`pickerActions(picked, renameTarget, rowDisabled)` is now
`pickerActions(picked, renameTarget)`. Checkout's gates are exactly
`picked === undefined || picked.preferred || renamingBranchId !== null` —
the `rowDisabled` term is gone. Rename already lived on its own gates
(`renameTarget === null || renamingBranchId !== null`) and is untouched
beyond the signature. The `Select` keeps `disabled={row.disabled}`, as the
ruling requires. The call site drops the third argument.

## 3. React — `packages/react/components/src/HistoryCenter.tsx`

Exact mirror: the same signature change, the same checkout gates, the same
call-site update, the `Select` unchanged.

## 4. Tests

Core (`packages/core/test/history-center.test.ts`):

- The R2 repro test (landed at step 2) passes unchanged — 502 pass / 0 fail
  in the file, up from 54 pass / 1 fail at handover.
- New derivation test: a stale level renders `picker:c2@1::-:disabled` and
  `not-yet-loaded:c2@1:f1` — the same row set the machine produces after the
  drop, never the spliced run.
- New machine tests:
  - A stale level stays open at its anchor with its loaded data dropped —
    dropping is not a close (b028 R1) — and renders the not-yet-loaded row.
  - The re-request leaves exactly once: a second and third transition with
    the same pages emit no `loadContinuations`.
  - A level whose run is not on the spine is untouched — same `open`
    reference, no load.
  - A fed-back `RUN_LOADED` for a stale level is inert: the drop precedes
    the result, so the run pages are not re-added.
  - `CONFIRM` commits the auto-chosen single fork (`chosen`, never `pick`)
    and clears the disclosure — the machine half of R1.

Components (both runtimes, mirror tests):

- Single fork, non-preferred auto-chosen fork: the `Select` is disabled AND
  `Checkout` and `Rename` are enabled — the row signal never reaches the
  menu (R1) — and activating Checkout emits
  `onCheckoutContinuation("l1")` and clears the disclosure: the enabled
  item is real, not decorative.
- Single fork, preferred auto-chosen fork: `Checkout` stays disabled — the
  `picked.preferred` gate, not the row gate — while `Rename` stays enabled.
- Host supplies root pages containing an open level's run: no duplicate
  `data-row-entry` on any row (R2; the Svelte side uses the harness
  `rerender({ pages })` to swap the root under a live disclosure).

## 5. Contract — `docs/contracts/components/history-center.md`

- Anatomy: the picker row bullet now reads `forkCount >= 1` (b033 R3: the
  single fork gets the same picker row) — the `> 1` text was stale since
  b033 and this card's R1 is exactly about the single-fork row.
- Row model: the picker row gains its `disabled` field and the split — the
  signal governs the `Select` alone; the menu never inherits it. The
  not-yet-loaded row gains the stale-level role (never a spliced duplicate,
  no new row kind).
- Behavior machine: a "Stale levels (R2, g13-034)" paragraph — detection
  from the data, drop-but-stay-open, exactly-once re-request through the
  existing effect, no `SYNC`/`REFRESH` event. The transitions guard
  paragraph notes the reconcile pre-pass and its effect ordering.
- Context table: the `open` field documents the stale-level re-request.
- Events table: `onLoadContinuations` gains the stale-level trigger.
- Part-attribute table: `picker-select` and `picker-actions` document the
  split (Select disabled from the row signal; Checkout's own gates).

## 6. Validation (step 7)

| Command | Result |
|---|---|
| `effigy test:core` | 503 pass / 0 fail, exit 0 |
| `effigy test:components` | 1010 pass / 0 fail, exit 0 |
| `effigy test:parity` | exit 0 |
| `effigy check:svelte` | 0 errors, exit 0 |
| `effigy docs:lint` | exit 0 |
| `effigy docs:contract-drift` | exit 0 |
| `effigy docs:callback-drift` | exit 0 |
| `effigy svelte:surface-audit` | exit 0 |
| `effigy drift:recipes` | exit 0 |
| `git diff --check` | clean |

No baseline refreshed.

## 7. Follow-ups and papercuts

- No new papercuts were hit this card; PAPERCUTS.md is untouched.
- The nested stale case is deliberately unremarkable: an inner level whose
  fork is on the spine is suppressed at depth exactly like a root level,
  and when its container drops, the subtree drops with it (see §1).
