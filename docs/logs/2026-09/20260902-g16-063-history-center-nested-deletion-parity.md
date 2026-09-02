# g16.063 — HistoryCenter Nested Deletion Parity

Status: implemented — awaiting orchestrator review
Date: 2026-09-02
Card: `docs/roadmaps/g16/063-history-center-nested-deletion-parity.md`
Handoff: `docs/handoffs/20260902-225241-g16-063-history-nested-delete.md`
Governing refs: `docs/contracts/components/history-center.md`,
`docs/roadmaps/g16/nucleus-gpui-parity-programme.md`
Branch: `fix/g16-063-history-nested-delete`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-063-history-nested-delete`
Planning base: `4ffa31345bc94f82c22d64d83e64b3af2613cfe3` (ancestor)
Live `origin/main` at dispatch: `a6a9d242a4473f2436e148b011c910f299ca6f36`
Worker PR: https://github.com/inflatable-cookie/poodle/pull/167

## Outcome

Deleting a nested continuation now invalidates that nested level in both
machines. TypeScript used a root `Map.set`; a nested delete left the inner
cache in place and added a ghost root key. Rust already called recursive
`replace_level` and had no nested vector. The pair now share one tree and
one effect order.

No public props, rejection codes, persistence, Nucleus, shells, or
renderers changed.

## What landed

- Core: `deleteContinuation` writes through `replaceLevel`.
- Headless: nested-delete proof only; production path already recursive.
- Tests: one shared tree (`e0` sibling root, `e1` → inner `l1a`/`l1b`,
  delete `n1` at `l1a`) in `history-center.test.ts` and
  `history_center.rs`.
- Contract: one sentence that replacement is recursive.

## Falsification

Counterexample committed, then TypeScript shown red on
`open.has("l1a") === true` before the repair. After repair, plants used a
clean index and `git checkout HEAD --`.

| Row | Plant | Result |
| --- | --- | --- |
| Nested invalidation | restore root `Map.set` | `open.has("l1a")` expected false, received true |
| Sibling retention | same plant | `e0` and inner `l1b` still equal; not the failure |
| Effects | emit `loadContinuations` twice for `l1a` | expected two effects, received three |
| Root delete | unchanged existing tests | still pass after repair |

Restored sources reran green.

## Validation

Focused, then required boards. Tree clean after restore.

- `bun test packages/core/test/history-center.test.ts` — 66 pass, 0 fail
- `cargo test --manifest-path packages/contracts/headless/Cargo.toml history_center`
  — 23 pass, 0 fail
- `effigy ci:web` — pass (~110s)
- `effigy ci:rust` (`test:contracts`) — pass (~22s)
- `effigy docs:check` — pass (~29s). First parallel attempt hit
  `task:poodle/core:build` lock against `ci:web`; rerun after that board
  finished.
- `git diff --check origin/main...HEAD` — clean
- `origin/main` still `a6a9d242a`; no sibling merge to rebase onto

Implementation commit: `1d2a400b620e83fa10e385000709fabbb62f469c`

## Limits

- HistoryCenter is not in the pinned `machines.json` corpus. The shared
  vector lives in the paired unit tests, not the conformance harness.
- Nucleus confidence evidence and g16 front doors stay orchestrator-owned.
- No windowed or release selector ran.
- Repo `effigy doctor` reports pre-existing scan errors (god-files,
  generated-in-src, stale-suppressions). Not this lane.
