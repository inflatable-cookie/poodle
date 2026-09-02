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
| Nested invalidation | restore root `Map.set` | `open.has("l1a")` true; inner `l1a` still held `n1x` |
| Sibling retention | same plant | `e0` and inner `l1b` still equal; not the failure |
| Effects | emit a second reload | expected `[delete n1, load l1a]` |
| Root delete | unchanged existing tests | still pass after repair |

Restored sources reran green.

## Validation

Focused, then required boards. Recorded after the repair commit.

## Limits

- HistoryCenter is not in the pinned `machines.json` corpus. The shared
  vector lives in the paired unit tests, not the conformance harness.
- Nucleus confidence evidence and g16 front doors stay orchestrator-owned.
- No windowed or release selector ran.
