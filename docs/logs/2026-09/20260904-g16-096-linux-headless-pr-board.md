# g16.096 — Linux Headless PR And Main Board

Status: implemented; PR #201 open — stopped at the card stop condition,
awaiting operator decision on a pre-existing `ci-web` failure
Date: 2026-09-04
Card: `docs/roadmaps/g16/096-linux-headless-pr-board.md`
Handoff: `docs/handoffs/20260904-132736-g16-096-linux-headless-pr-board.md`
Governing refs: `scripts/check-release-automation.ts`,
`tasks/effigy.tasks.toml` (`ci:web`, `ci:rust`), `.github/workflows/ci-web.yml`,
`.github/workflows/ci-rust.yml`, `AGENTS.md`
Branch: `feature/g16-096-linux-headless-pr-board`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-096-linux-headless-pr-board`
Base: `origin/main` at `f6e94718b849fbcbdac7da43d1b7f4477b60d1b3`; `main`
advanced to `c6d1a0ea2d70ae57cc5bd13838ee81c74077ce97` (g16.095 merge)
mid-lane — head refresh is reserved for the coordinator per the handoff
Worker PR: https://github.com/inflatable-cookie/poodle/pull/201

## Outcome

`ci-web.yml` and `ci-rust.yml` now run automatically on pull requests
targeting `main` and pushes to `main`, keeping `workflow_dispatch` and the
existing `concurrency` groups with `cancel-in-progress: true`. Both stay
`ubuntu-latest` only; no path filters, no macOS runner, no native selector,
no secret, no new job, no publish step. `ci-native.yml`, `ci-visual.yml`, and
`release.yml` are byte-identical to `main`.

`scripts/check-release-automation.ts` asserts trigger shape per workflow
instead of globally: `ci-web.yml` and `ci-rust.yml` must carry
`workflow_dispatch` plus `pull_request` and `push` restricted to `main` and
no other event; `ci-native.yml`, `ci-visual.yml`, and `release.yml` keep the
dispatch-only assertion (no `push`, `pull_request`, or `schedule`).

## Stop condition

The first `ci-web` PR run failed inside the gate, for a reason unrelated to
the trigger change, so this lane stops here per the card instead of
widening scope.

- Failing step: `Execute Effigy web gate` → `effigy ci:web` →
  `bunx vitest run` (job `web`, run
  https://github.com/inflatable-cookie/poodle/actions/runs/33873611405,
  12:36:18Z–12:40:38Z, 4m20s).
- Failure: 380 test files — 377 passed, 3 failed suites at load; all 3651
  tests passed. The three red files are the react-preview suites
  `catalogue-nav.test.tsx`, `g15-031-foundation-content-status.test.tsx`,
  and `g15-033-composition-forms-data-media.test.tsx`, each with
  `Error: Failed to resolve import "@inflatable-cookie/poodle-react" from
  "packages/react/preview/src/gallery/ComponentsSection.tsx". Does the file
  exist?` (same error against the gallery specimens).
- Classification: the open 2026-09-02 `PAPERCUTS.md` entry — on a cold
  tree `test:components` runs before `react:package` produces
  `packages/react/components/dist`, so react-preview cannot resolve the
  package's dist exports. A CI checkout is always cold, so the new
  automatic board hits this on every run. This lane's diff touches only
  the `on:` trigger blocks of the two workflows plus the checker; the job
  bodies are byte-identical to `main`, so the failure is pre-existing and
  out of lane (fixing the board order or the alias touches
  `tasks/effigy.tasks.toml` and package config, not an owned path).
  Second sighting is recorded in `PAPERCUTS.md`.
- `ci-rust` on the same head passed: run
  https://github.com/inflatable-cookie/poodle/actions/runs/33873611131,
  12:36:16Z–12:36:54Z (38s), `Execute Effigy Rust gate` green with
  `test result: ok` across the eight pure contract crate trees.

## Review oracle

| Invariant | Plant / probe | Result |
| --- | --- | --- |
| PR runs the web board | the card's own PR head | run 33873611405 appeared on the PR head and executed `effigy ci:web`; red on the pre-existing react-preview dist defect above |
| PR runs the Rust board | same PR | run 33873611131 appeared on the PR head, executed `effigy ci:rust`, all contract tests ok |
| Superseded heads cancel | push a second commit to the PR | concurrency groups preserved unchanged (`ci-web-${{ github.ref }}` / `ci-rust-${{ github.ref }}`, `cancel-in-progress: true`); not yet exercised by timing — the log-commit push landed after both runs completed |
| Only Linux | inspect both workflow files | `runs-on: ubuntu-latest` only; `macos` string count 0 in both files |
| Release stays manual | `git diff origin/main` at base | `ci-native.yml`, `ci-visual.yml`, `release.yml` unchanged; `workflow_dispatch` only |
| Automation checker agrees | pass on new shape; planted `push` on `release.yml`; planted `schedule` on `ci-web.yml` | pass on the new shape; both plants fail with the intended per-workflow messages |

## Validation

- `effigy check:release-automation`: pass on the new shape; the planted
  `push` on `release.yml` fails with `release.yml must not add an automatic
  trigger`; the planted `schedule` on `ci-web.yml` fails with `ci-web.yml
  must trigger on pull_request and push to main plus workflow_dispatch and
  nothing else`. Each plant was applied to a backup of the real file and
  the file restored afterward.
- `effigy docs:check`: pass (37s).
- `git diff --check origin/main...HEAD`: pass on the committed head
  (`d4ec5bde6`), checked after commit so the range is non-vacuous.
- PR runs are the executable proof; both URLs and durations are recorded
  under Stop condition above. No release, windowed, native-visual, or
  `*-windowed` selector was run. Local effigy was
  `v0.12.1+local.aafbd93`; CI uses `setup-effigy` 0.11.0 as pinned.

## Limits

- This worker has not merged, rebased PR #201, or dispatched any workflow.
- The lane stopped at the card stop condition; the automatic `ci-web` board
  cannot go green until the pre-existing react-preview dist-ordering defect
  is fixed (options are documented in the 2026-09-02 `PAPERCUTS.md` entry
  and the second-sighting entry added here).
- Head refresh against `main` at `c6d1a0ea` is reserved for the
  coordinator.

## Continuation

Operator decision needed: repair the pre-existing `ci-web` board order (or
alias) so the new automatic board can go green, or accept a red automatic
web signal. The trigger and checker work in PR #201 is complete and locally
validated; once the board defect lands, the same PR head can be re-run
without further edits.
