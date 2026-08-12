# 12 — g13.032 HistoryCentre — Rename The Selected Fork (batch log)

Branch: `thread/g13-032-history-center-multi-fork-rename`
Date: 2026-08-12
Card: `docs/roadmaps/g13/batch-cards/032-history-center-multi-fork-rename.md`
Status: **COMPLETED** — pencil `IconButton` between the `Select` and checkout
(R1), renaming whichever fork the `Select` currently shows through the
existing rename machinery (R2), input replaces the `Select` while renaming
with checkout disabled and focus returned to the pencil (R3), and the
"Current" badge deleted from both runtimes and its CSS (R4a). Specimens,
contract and tests updated; every step-7 command exits 0.

## 1. Base verification (step 1)

- Branch `thread/g13-032-history-center-multi-fork-rename` — the card's
  dispatch commits (`45b875a8` add, `352ebb67` R4a carve-out) sit on
  `c6590823` (g13-b030 merged). ✓
- `effigy test:core` → 487 pass / 0 fail. ✓
- `effigy test:components` → 982 pass / 0 fail. ✓
- `effigy check:svelte` → 0 errors (449 files, 4 pre-existing warnings). ✓
- `effigy docs:lint` → exit 0; `git diff --check` → clean. ✓

## 2. Svelte — `packages/svelte/components/src/HistoryCenter.svelte`

- Picker row order is now **Select, pencil, checkout** (R1). The pencil is an
  `IconButton` (`icon="edit"`, label `Rename <branch>`, tooltip "Rename
  branch") in a `picker-rename` wrapper between the `Select` and the
  `picker-checkout` wrapper; `variant="ghost"`, `size="xs"`, matching the
  checkout button (R4).
- The pencil renames the fork the `Select` currently shows: `renameTarget`
  derives from `row.pickedEntryId` (`branchId` + `branchName ?? branchId`),
  so a changed selection is a changed target. `startRename` seeds the shared
  `renameValue` with that fork's current name; `commitRename` sends the
  machine's `RENAME` and the existing `emitRenameBranch` effect calls
  `onRenameBranch(branchId, name)` — no second rename path (R2).
- While a rename is open the inline input takes the `Select`'s place (R3),
  bound to the same `renameInputElement`/`renameValue`/`handleRenameKeydown`
  machinery as the run-header input, and checkout is disabled
  (`renamingBranchId !== null` joins the existing `picked === undefined ||
  picked.preferred` rule). Commit or cancel restores the `Select`.
- `finishRename` focus restore now also targets the picker pencil. The
  run-header button and the pencil are both found by
  `[data-part][data-branch]`; the pencil's `IconButton` cannot carry data
  attributes (IconButton takes explicit props and forwards none), so the
  `picker-rename` wrapper carries `data-part`/`data-branch` and the button
  inside is the focus target. On commit or cancel, focus returns to the
  pencil (R3), exactly as it returns to the run-header button.
- The pencil sits inside `[data-part="picker-select"]`, so `handleListKeydown`
  treats its keys exactly like the checkout button's — the Select region owns
  them; Enter/Space activate the pencil natively. No keydown change needed.
- **Escape fix (shared machinery).** The required cancel behavior exposed a
  real bug in the existing rename path: the surface is portalled, so Svelte's
  delegated `keydown` listener and the dismiss layer's document listener both
  sit on `document`, and the dismiss layer ignores `preventDefault` — Escape
  in the rename input cancelled the rename *and* closed the popover. The
  single-fork path had the same quirk; its cancel test only checked that the
  input disappeared. `handleRenameKeydown` now calls
  `stopImmediatePropagation` (Enter and Escape): `stopPropagation` cannot stop
  a same-node document listener, `stopImmediatePropagation` can. React's
  synthetic system stops the native event before `document`, so React keeps
  `stopPropagation` — the asymmetry is the two event systems, not the
  behavior. Both runtimes now cancel a rename without closing the popover.
- The `current-badge` markup is gone from both the trigger snippet and the
  option snippet (R4a). `preferred` stays on the record; checkout still
  disables for the current fork.

## 3. React — `packages/react/components/src/HistoryCenter.tsx`

Exact mirror: `renameTarget` from `pickedContinuation`, pencil `IconButton`
in a `picker-rename` wrapper (order Select, pencil, checkout), input replaces
the `Select` while `renamingBranchId === renameTarget.branchId`, checkout
disabled while a rename is open, `current-badge` removed from the trigger and
option render props, and the focus-restore effect extended to the picker
pencil (wrapper carries the part/branch; the button inside receives focus).

## 4. CSS — `packages/core/src/styles/history-center.css`

- `.poodle-history-center__picker-rename` added (inline-flex, flex none —
  mirrors `picker-checkout`); the picker row's comments updated for the
  Select/pencil/checkout anatomy.
- `.poodle-history-center__picker-controls .poodle-history-center__rename-input`
  zeroes the run-header input's margins inside the controls row.
- `.poodle-history-center__current-badge` deleted along with the
  `--poodle-recipe-history-center-current-badge-fill`/`-text` hooks (R4a);
  the option-content comment no longer mentions a right-aligned marker.

## 5. Tests

- Component suites (Svelte + React): the two-fork picker test now asserts
  the pencil sits between the `Select` and checkout (`compareDocumentPosition`)
  and that no `current-badge` renders in the trigger or the open listbox;
  the trigger/option copy assertions drop "Current". The
  current-fork-checkout test keeps the disabled-checkout rule and asserts
  the badge is gone (R4a).
- New tests, both runtimes:
  - Renaming from the picker emits `onRenameBranch("b-x1", …)` — the
    selected fork's branch id (the input replaces the `Select` while open).
  - Changing the `Select` then renaming targets the newly selected fork
    (`b-l1`, not the preferred `b-x1`).
  - Checkout is disabled while a rename is open (after being enabled on the
    non-preferred pick).
  - Cancelling restores the `Select` and returns focus to the pencil.
- `forkCount === 1` is untouched: still the run-header rename, no picker
  (existing tests unchanged and green).

## 6. Specimens

Both `HistoryCenterSpecimen.svelte` / `.tsx`: the two-forks group now shows a
rename in progress. The capture never clicks, so each specimen drives its own
two interactions on mount — disclose the fork, then press the pencil — via a
small guarded effect (Svelte `onMount` + `tick`, React `useEffect` +
`requestAnimationFrame`, with one retry). The popover portals to the theme
root, so only the one open group's controls are matched.

## 7. Contract — `docs/contracts/components/history-center.md`

Anatomy (picker row: Select, pencil, checkout; rename replaces the Select
while open), `onRenameBranch` prop and events rows (picker included; the
picker's `branchId` is the fork the `Select` currently shows), machine note
(RENAME surfaces at both sites through one event), part-attribute table
(`picker-rename`, `picker-rename-input` added; `current-badge` retired;
`picker-checkout` disabled rule extended), accessibility (pencil semantics,
keyboard/focus rows), token table (`current-badge-fill`/`-text` retired;
`preferred` keeps its job through the disabled checkout button).

## 8. Validation (step 7)

| Command | Result |
|---|---|
| `effigy test:core` | 487 pass / 0 fail, exit 0 |
| `effigy test:components` | pass / 0 fail, exit 0 |
| `effigy test:parity` | exit 0 |
| `effigy check:svelte` | 0 errors, exit 0 |
| `effigy docs:lint` | exit 0 |
| `effigy docs:contract-drift` | exit 0 |
| `effigy svelte:surface-audit` | exit 0 |
| `git diff --check` | clean |

No baseline refreshed.

## 9. Follow-ups and papercuts

- **Follow-up (R4, deliberate): the single-fork run-header rename stays a raw
  `<button>`** while the picker's rename is an `IconButton` — the run header
  is a different region with its own styling, and changing it is an
  unrequested visual change. Recorded here as the card requires; a future
  card may align the run-header control with the `IconButton` convention.
- **Papercut (PAPERCUTS.md): `IconButton` forwards no data attributes** —
  explicit props only, no rest spread — so any composite that needs a
  `data-part`/`data-branch` marker on an `IconButton` must wrap it in a
  marker-carrying span and focus the button inside. The picker pencil is the
  first instance; a future `IconButton` rest-prop pass would remove the
  wrapper.
