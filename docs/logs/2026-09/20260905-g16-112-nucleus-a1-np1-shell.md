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
- `bun scripts/parity-evidence-ledger.ts --write`: regenerated the lane-owned
  evidence ledger; validation passed with 176 component rows.
- `effigy docs:check`: documentation lint/audits and ledger validation passed.
- `effigy check:parity-evidence-ledger`: passed after regenerating the ledger;
  NP-1 now records five mounted rows and GPUI accessibility is `7 mounted /
  168 manual`.
- Hosted `web` run `33965002026` was the pre-clarification failure at the stale
  ledger step; a new exact-head hosted run is required after push.
- Windowed selectors: not run.
