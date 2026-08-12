# 12 — g13.030 HistoryCentre — Persistent Fork Select (batch log)

Branch: `thread/g13-030-history-center-fork-select`
Date: 2026-08-12
Card: `docs/roadmaps/g13/batch-cards/030-history-center-persistent-fork-select.md`
Status: **COMPLETED** — persistent `Select` picker (R1), select/checkout split
(R2), checkout vocabulary (R2a), open-selects-current (R3), `Select` +
checkout `IconButton` in both runtimes (R4), tests, specimens, CSS and
contract updated; every step-8 command exits 0.

## 1. Base verification (step 1)

- Branch `thread/g13-030-history-center-fork-select` at `e2868b33` ("Correct
  b030: checkout swaps the root list, and is called checkout") — the ruling
  revision. ✓
- `effigy test:core` → 482 pass / 0 fail. ✓
- `effigy test:components` → 980 pass / 0 fail. ✓
- `effigy check:svelte` → 0 errors (449 files; the b028 core type errors the
  card's worker-rules mention were already gone at this revision). ✓
- `effigy docs:lint` → exit 0; `git diff --check` → clean. ✓

## 2. Core — `packages/core/src/history-center.ts`

- **R1 — the picker persists.** `pushDisclosed` emits the picker row whenever
  `forkCount > 1`, removing the `level.chosen === null` gate; the run below
  follows the displayed fork (`pick ?? chosen`), so the picker sits above the
  selected fork's entries and stays reachable after a choice. `forkCount ===
  1` still renders no picker. The picker row's `pickedEntryId` doc now says
  the select's value (R3 seeds it).
- **R2 — select previews, checkout swaps.** `PICK_CONTINUATION` sets the
  tentative pick, drops the loaded run when it belongs to another fork, and
  emits only `loadContinuationRun` — no host operation. `CONFIRM` (checkout)
  emits `checkoutContinuation(entryId)` alone and clears the anchor's
  disclosure state (`withoutLevel`); Poodle does not fabricate the new root —
  the host supplies the pages and the machine renders whatever root it is
  given (test proves a rerooted page list renders the fork as the root with
  no stale fork state). `runLoaded` matches the level's displayed fork.
- **R2a — checkout vocabulary.** Effect `preferContinuation` →
  `checkoutContinuation`. The record field `preferred` is untouched (the
  authority's name); only Longhorn's own name appears in Poodle's doc
  comments, as the mapping target.
- **R3 — open selects the current fork.** `CONTINUATIONS_LOADED` with
  `forkCount > 1` selects the preferred fork, else the first in supplied
  order, and emits `loadContinuationRun` for it. `forkCount === 1`
  auto-chooses as before.

## 3. Svelte — `packages/svelte/components/src/HistoryCenter.svelte`

- `Button` import out; `Select` in. Prop `onPreferContinuation` →
  `onCheckoutContinuation`; effect case renamed.
- Picker row: Poodle's `Select` (`value` = `row.pickedEntryId`, options = the
  forks, `variant="ghost"`, `size="xs"`, `ariaLabel="Continuations"`) plus a
  checkout `IconButton` (`icon="check"`, label/tooltip "Checkout") in a
  `picker-checkout` wrapper. `onValueChange` → `PICK_CONTINUATION`;
  `CONFIRM` on click. Checkout disabled while no pick is set or the pick is
  the current fork (`picked.preferred`) — R4.
- Both the trigger snippet and the option snippet render the fork label, its
  branch name and a `current-badge` ("Current") — the persistent select
  keeps the screenshot's information visible. No hand-rolled option list
  remains; `picker-option`/`preferred-badge`/`picker-confirm` parts are gone.
- Keyboard: `handleListKeydown` returns early for any key whose target is
  inside `[data-part="picker-select"]` — the Select owns its keys (arrows
  open the listbox instead of moving roving focus, Enter/Space pick), which
  the traversal test asserts.

## 4. React — `packages/react/components/src/HistoryCenter.tsx`

Exact mirror: same props, labels, parts, Select/IconButton composition,
trigger/option render props, keydown guard, `forkForValue` lookup helper.

## 5. CSS — `packages/core/src/styles/history-center.css`

Picker styles rebuilt for the Select: `.picker-options`, `.picker-option`
(+hover/focus/pressed) and `.picker-actions` deleted; `.picker-controls`
(flex row: select flexes, checkout fixed), `.picker-value` (trigger copy),
`.picker-checkout` and the Select option-content flex rule added; the
`preferred-badge` block became `current-badge` with recipe hooks renamed to
`--poodle-recipe-history-center-current-badge-fill`/`-text` (the
`--poodle-recipe-history-center-*` convention kept).

## 6. Tests

- Core (`packages/core/test/history-center.test.ts`): the picker render now
  carries the tentative pick; the old confirm flow tests replaced by:
  R3 auto-select (preferred and first-fallback), pick-shown-emits-nothing,
  pick-different-fork previews (no host operation), the R1 persistence
  flow (disclose → pick → run loads → picker still present with the new
  selection), checkout emits `checkoutContinuation` and clears the anchor's
  disclosure state, and rerooting after checkout renders the fork as the
  root list. 49 tests in the file, 487 core-wide.
- Component suites: `twoForkRuns` gains x1's run so R3's selection renders;
  the three picker tests rewritten against the Select (trigger copy
  assertions, role `option` in the open listbox, checkout enablement), plus
  a no-host-operation-on-select test in each suite; traversal updated for
  the run rows and extended with the Select-owns-its-keys assertion.
  Harness prop renamed `onCheckoutContinuation`. 982 component tests pass.
- Parity: `test:parity` 164 pass — both runtimes emit identical anatomy.

## 7. Specimens

Both `HistoryCenterSpecimen.svelte` / `.tsx`: the two-forks group (opens by
default) now captures the reported defect fixed — R3 selects the current
fork, the Select persists with the selection and its Current marker, and the
fork's run renders below it.

## 8. Contract — `docs/contracts/components/history-center.md`

Picker section, row-model persistence note, `onCheckoutContinuation` prop,
select/checkout split in the transitions and effects tables, part-attribute
table (`picker-select`, `picker-checkout`, `current-badge`; `picker-option`,
`preferred-badge`, `picker-confirm` retired), events, accessibility
(combobox/listbox semantics, Current marker), token table
(`current-badge-fill`/`-text` hooks) and the Svelte composition list.

## 9. Validation (step 8)

| Command | Result |
|---|---|
| `effigy test:core` | 487 pass / 0 fail, exit 0 |
| `effigy test:components` | 982 pass / 0 fail, exit 0 |
| `effigy test:parity` | 164 pass / 0 fail, exit 0 |
| `effigy check:svelte` | 0 errors, exit 0 |
| `effigy docs:lint` | exit 0 |
| `effigy docs:contract-drift` | exit 0 (130 checked, 34 skipped) |
| `effigy svelte:surface-audit` | exit 0 (164/164 coverage) |
| `git diff --check` | clean |

## 10. Papercuts hit

- The picker's `Select` portals its listbox to the document body (standard
  Select behaviour), so Tab from an open listbox is outside the Popover's
  focus trap — `trapFocusKeydown` only sees keys inside the surface. The
  Select's own keyboard (arrows/Enter/Escape) works, and roving traversal is
  unaffected; the escape is a Select-in-Popover composite concern, logged in
  PAPERCUTS.md.
