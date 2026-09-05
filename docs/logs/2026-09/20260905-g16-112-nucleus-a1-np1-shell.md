# g16.112 — Nucleus A1 NP-1 Shell execution log

## Authority

- Card: `docs/roadmaps/g16/112-nucleus-a1-np1-shell.md`
- Dispatch: `docs/roadmaps/dispatch.md`, revision 17
- Base verified: `3dbabac3990fb5f3856305b7c8f971039b0a81be`
- Worker handoff: `docs/handoffs/20260905-g16-112-nucleus-a1-np1-shell.md`

## Scope

Seven shell rows: Icon, Text, Surface, Button, IconButton, AppHeader, and
SplitView. No component, contract, Svelte, or backend behaviour changes.

## Work

- Added shared A1 scenarios for all seven NP-1 rows.
- Added committed Svelte snapshots from `effigy test:nucleus-a11y`.
- Added native A1 proof entry points to the existing headless receipt harness.
- Recorded the cold-worktree core CSS build friction in `PAPERCUTS.md`.

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
- `effigy check:parity-evidence-ledger`: blocked on the reserved coordinator
  ledger markdown closeout and the expected summary update for these new
  mounted rows; no reserved closeout surface was edited in this worker PR.
- Windowed selectors: not run.
