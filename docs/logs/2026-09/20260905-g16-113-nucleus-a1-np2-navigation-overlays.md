# g16.113 — Nucleus A1 NP-2 Navigation And Overlays

Status: complete — one validated A1 receipt and four executed divergences; PR
open for independent exact-head review
Date: 2026-09-05
Card: `docs/roadmaps/g16/113-nucleus-a1-np2-navigation-overlays.md`
Dispatch: `docs/roadmaps/dispatch.md` revision 17
Base: `3dbabac3990fb5f3856305b7c8f971039b0a81be` (`origin/main`)
Source pin: `2fa5b2a1047b61c2a08a0f650e26a3ef7fbb1a06`
Worker branch: `worker/g16-113-nucleus-np2`

## Outcome

NP-2 now has shared scenarios and executed paired A1 evidence for
SegmentedControl, Menu, Dialog, Popover, and EditableLabel. EditableLabel
matches and has a validated A1 receipt. The other four rows have committed
GPUI snapshots and exact recorded diffs under the A1 divergence store. Select
is explicitly skipped and remains owned by g16.117.

Two missing native accessible-name projections were repaired with the
card-permitted one-line renderer fixes: SegmentedControl options and Menu
items now project the labels already present in their specs. Dialog and
Popover retain honest structural divergences; Menu retains the default-open
focus difference; SegmentedControl retains the native roving-focus difference.
No contract, Svelte, or backend behaviour change was made.

## Evidence

- Shared scenario files and Svelte snapshots cover the five NP-2 rows.
- `editablelabel--nucleus-navigation-editable-label--a1.json` is validated;
  its GPUI snapshot is `test/nucleus-a11y/snapshots/editable-label.gpui.json`.
- Divergences are stored under
  `docs/roadmaps/g16/nucleus-parity-receipts/a1-divergences/{segmented-control,menu,dialog,popover}/`.
- The complete receipt cohort was repinned to the source commit because the
  receipt validator pins the mounted preview/runtime source surface.

## Validation

- `effigy test:nucleus-a11y`: pass, 9 tests.
- Headless native regression board: pass, 206 passed, 5 ignored, 0 failed.
- No windowed, native-visual, or capture selector was run.
- Final checks passed after the closeout evidence commit: receipt ledger,
  docs check, diff check, and the focused A1 Svelte/native selectors.

## Closeout

Reserved coordinator surfaces remain untouched: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/g16/generation-index.md`, and `docs/roadmaps/dispatch.md`.
