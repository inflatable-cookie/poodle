# g16.112 — Nucleus A1 NP-1 Shell execution log

## Authority

- Card: `docs/roadmaps/g16/112-nucleus-a1-np1-shell.md`
- Dispatch: `docs/roadmaps/dispatch.md`, revision 17
- Base verified: `3dbabac3990fb5f3856305b7c8f971039b0a81be`; rebased onto current
  `origin/main` `7c1837f0fa2fede8fbd476b3362d88dd112290d7`.
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
- `effigy docs:check`: passed.
- `effigy ci:web`: web/package/component checks passed; it stops at the
  coordinator-owned ledger boundary because the reserved markdown lacks the
  four new mounted cells and summary update.
- `effigy check:parity-evidence-ledger`: same reserved boundary (rows 1/3/4/5
  and GPUI mounted/manual summary drift); no reserved closeout surface changed.
- Hosted `web` run `33965002026` failed at that same ledger step; Svelte checks
  passed. Coordinator must regenerate the reserved ledger at this exact head.
- Windowed selectors: not run.
