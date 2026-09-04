# g16.098 — Cold-Checkout Web Board Repair

Status: complete — awaiting orchestrator review
Date: 2026-09-04
Card: `docs/roadmaps/g16/098-cold-checkout-web-board-repair.md`
Handoff: `docs/handoffs/20260904-150000-g16-098-cold-checkout-web-board-repair.md`
Governing refs: `vitest.config.ts`, `tasks/effigy.tasks.toml` (`ci:web`),
`docs/architecture/014-compiled-web-package-distribution.md`
Branch: `feature/g16-098-cold-checkout-web-board-repair`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-098-cold-checkout-web-board-repair`
Base: `origin/main` at `7022534e7` after rebase over merged g16.095
Planning base at dispatch: `3dbd1cabd1ca504d9744e948a0b4079b0b865eea`

## Outcome

`react-preview` resolves `@inflatable-cookie/poodle-react` through the same
workspace alias every other vitest project already had. `ci:web` now builds
`svelte:package` and `react:package` before `test:components`, matching
`docs:check`, and still runs `test:web-pack-install` after those builds.

A detached-worktree proof parks no shell `dist/`, runs the three previously
failing suites, and replants the missing alias. No workflow, package export,
component, release, windowed, or native-visual change.

## Reproduction

On current `main` before the alias, parking both shell `dist/` trees and
running the three `react-preview` suites failed at transform:

```text
FAIL  react-preview  packages/react/preview/test/catalogue-nav.test.tsx
Error: Failed to resolve import "@inflatable-cookie/poodle-react" from
"packages/react/preview/src/gallery/ComponentsSection.tsx". Does the file exist?

FAIL  react-preview  packages/react/preview/test/g15-031-foundation-content-status.test.tsx
Error: Failed to resolve import "@inflatable-cookie/poodle-react" from
"packages/react/preview/src/gallery/specimens/ErrorBoundarySpecimen.tsx".

FAIL  react-preview  packages/react/preview/test/g15-033-composition-forms-data-media.test.tsx
Error: Failed to resolve import "@inflatable-cookie/poodle-react" from
"packages/react/preview/src/gallery/specimens/FieldSetSpecimen.tsx".
```

The committed proof reproduced the same miss: order test
`svelte:package` index 9 vs `test:components` index 8, and no
`resolve: { alias: workspaceAliases }` on `react-preview`. The planted
alias removal already failed with `Failed to resolve import`.

## Review oracle

| Invariant | Plant | Result |
| --- | --- | --- |
| Cold checkout passes | delete both shell `dist/` trees; run the three `react-preview` suites | pass after the alias |
| Alias is real | strip `react-preview`'s `workspaceAliases` | `Failed to resolve import "@inflatable-cookie/poodle-react"` on all three files |
| Order is safe | `effigy ci:web` from a fresh detached worktree with `bun install` | green; 381 files / 3658 tests after rebase |
| No behaviour change | warm `effigy ci:web` | same 381 / 3658 and pack-install 11 / 22 |

## Validation

- Pre-fix parked-dist run of the three suites: 3 failed / 0 tests, exact
  `Failed to resolve import` text above.
- Proof commit first (`2bcf954c3`): 1 pass / 2 fail.
- After the alias and `ci:web` reorder: `bun test scripts/web-distribution/cold-checkout-react-preview.test.ts` — 3 pass / 0 fail.
- Warm `effigy ci:web` on the worker tree: 380 files / 3658 tests then, after
  rebase onto merged g16.095, 381 files / 3658 tests (the extra file is
  `react-prop-drift.test.ts`); pack-install 11 / 22; proof 3 pass; EXIT 0.
- Cold detached worktree at `5fbe0fc18` then rebased `9441d7b8e`: first
  attempt passed test:components 380 / 3658 and pack-install, then
  `gate:clean` lost the shared `/tmp` snapshot (known papercut). Retry with
  isolated `TMPDIR` EXIT 0. Post-rebase warm board EXIT 0.
- `effigy docs:check`: pass before and after rebase (now includes
  `docs:react-prop-drift`).
- `git diff --check origin/main...HEAD`: pass.

Bun's default 5s `afterAll` budget was too short for `git worktree remove`
on the proof checkout; the hook now has 60s. `FORCE_COLOR` was unset for
selectors.

No `release prepare/execute/simulate`, tag, publish, workflow dispatch,
windowed selector, or native-visual selector was run.

## Limits

- This worker has not merged, rebased PR #201 (`g16.096`), tagged, or started
  `g16.097`. Those stay with the orchestrator after merge.
- `docs:check` / `ci:web` still share `os.tmpdir()` for `gate-tree-guard`.
  Isolated `TMPDIR` is a validation workaround, not a harness fix.

## Continuation

Orchestrator exact-head review remains. After merge, rebase PR #201 and
start `g16.097` re-certification from the new `main` tip.
