# g16.112 — Nucleus A1 NP-1 Shell execution log

## Authority

- Card: `docs/roadmaps/g16/112-nucleus-a1-np1-shell.md`
- Dispatch: `docs/roadmaps/dispatch.md`, revision 17
- Base verified: `3dbabac3990fb5f3856305b7c8f971039b0a81be`; rebased onto current
  `origin/main` `ef9049f158863ec181dee46123a8e59c0c957091` after PR #219 became
  conflicting.
- Worker handoff: `docs/handoffs/20260905-g16-112-nucleus-a1-np1-shell.md`

## Scope

Seven shell rows: Icon, Text, Surface, Button, IconButton, AppHeader, and
SplitView. No component, contract, Svelte, or backend behaviour changes.

## Work

- Added shared A1 scenarios for all seven NP-1 rows.
- Added committed Svelte snapshots from `effigy test:nucleus-a11y`.
- Added native A1 proof entry points to the existing headless receipt harness.
- Recorded the cold-worktree core CSS build friction in `PAPERCUTS.md`.
- Native proofs deserialize each row's shared scenario props and fixture text.
- Text uses a declared `status` probe fixture because production Text is
  intentionally non-semantic; both runtimes compose the same probe.

## Checks

- `effigy test:nucleus-a11y` with snapshot generation: passed, 11 tests.
- `effigy regressions:native`: passed, 212 passed and 1 existing ignored.
- Five validated A1 receipts were emitted for Icon, Text, Surface, Button, and
  IconButton. AppHeader diverged on the unavailable native `banner` role;
  SplitView diverged on separator value and collapse-toggle name. Both diffs
  are stored under `nucleus-parity-receipts/a1-divergences/np1/`.
- `git diff --check`: passed.
- `effigy test:nucleus-parity-receipts`: passed, 11 tests.
- `bun scripts/parity-evidence-ledger.ts --write`: regenerated the lane-owned
  evidence ledger; validation passed with 176 component rows.
- `effigy docs:check`: documentation lint/audits and ledger validation passed.
- `effigy check:parity-evidence-ledger`: passed after regenerating the ledger;
- NP-1 now records five mounted rows and GPUI accessibility is `9 mounted /
  166 manual` at the rebased head. The manifest and all receipts are repinned
  to source commit `7d88454df565030360ef868528b50c50b63b0eba`; the lock digest
  remains `c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`.
- `effigy test:nucleus-a11y`: passed, 16 tests.
- Focused native NP-1 proof selector: passed, 7 tests.
- Full `effigy regressions:native` remains blocked by two unrelated current-main
  failures (`model_picker_selection_and_identity_rebuild_through_mounted_input`
  and `select_two_instances_search_pointer_and_dismiss_through_mounted_rebuilds`);
  the run reached 211 passed, 2 failed, 5 ignored.
- Ledger, receipt contract, Svelte, native NP-1, docs, and diff checks are
  rerun after the rebase; hosted `ci-web` and `ci-rust` must pass on the pushed
  exact head below.
- Hosted `web` run `33965002026` was the pre-clarification failure at the stale
  ledger step; a new exact-head hosted run is required after push.
- Ledger ownership clarification: `docs/roadmaps/g16/parity-evidence-ledger.md`
  is lane-owned generated evidence. Only `docs/roadmaps/g16/README.md` and
  `docs/roadmaps/generation-index.md` remain coordinator-reserved.
- Windowed selectors: not run.
