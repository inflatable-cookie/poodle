# g16.096 — Linux Headless PR And Main Board

Status: complete — merged in PR #201 at `372aa5253`. Boundary widened 2026-09-04: the checkout step
must fetch `origin/main` so the pack-install scope classifier has its base
Type: workflow automation — explicit operator approval recorded
Opened: 2026-09-04
Depends on: none; independent of every other ready lane
Governing refs: `../../../AGENTS.md` (workflow edits require explicit operator approval), `tasks/effigy.tasks.toml` (`ci:web`, `ci:rust`), `scripts/check-release-automation.ts`, `.github/workflows/ci-web.yml`, `.github/workflows/ci-rust.yml`
Operator decision: 2026-09-02 — "Plan web + Rust": authorize a bounded workflow card for automatic `ci:web` and `ci:rust` on pull requests and `main`; keep macOS, native, visual, and release manual
Dispatch manifest: `../dispatch.md`

## Goal

Give `main` an automatic validation signal. Every workflow under
`.github/workflows/` is `workflow_dispatch` only, by a documented decision
after macOS lanes exhausted the organisation's Actions allowance. `effigy qa`
is the release gate but runs only inside a manual `release.yml` dispatch. With
dozens of merges per day, nothing validates `main` after a merge. `ci-web.yml`
and `ci-rust.yml` already run on `ubuntu-latest` only; they are the cheap
lanes the operator approved.

## Fixed Boundary

- Edit exactly two files: `.github/workflows/ci-web.yml` and
  `.github/workflows/ci-rust.yml`. Add `pull_request` (targets `main`) and
  `push` (branches `main`) triggers. Keep `workflow_dispatch`. Keep the
  existing `concurrency` groups with `cancel-in-progress: true` so superseded
  PR heads do not queue.
- Do not add path filters unless a filter is needed to keep the run under the
  documented allowance; if one is added, document the exact rule in the
  workflow header comment and confirm `ci:web` still runs for docs-only PRs
  that touch `docs/contracts/` or `packages/svelte/preview/` (docs gates live
  in `ci:web`).
- Replace the "Manual dispatch only" header comment in both files with the
  new trigger rationale and the 2026-09-02 approval.
- `ci-native.yml`, `ci-visual.yml`, and `release.yml` stay dispatch-only.
  Do not touch them.
- `scripts/check-release-automation.ts:51-55` currently asserts every active
  workflow has `workflow_dispatch` and no `push`, `pull_request`, or
  `schedule` trigger. Make that assertion per workflow: `ci-web.yml` and
  `ci-rust.yml` must carry `workflow_dispatch` plus `pull_request` and
  `push` to `main` and nothing else; `ci-native.yml`, `ci-visual.yml`, and
  `release.yml` keep the dispatch-only assertion. Prove both directions with
  a planted trigger on `release.yml` (must fail) and a planted `schedule` on
  `ci-web.yml` (must fail).
- Checkout must expose `origin/main`. On a `pull_request` run,
  `actions/checkout` fetches only the merge ref, so
  `test/package-install/web-preview.ts:167` (`git merge-base HEAD
  origin/main`) exits 128 and `test:web-pack-install` fails (run
  `33881115094`). In both workflows set `fetch-depth: 0` on the checkout
  step, or add one explicit `git fetch --no-tags origin
  main:refs/remotes/origin/main` step immediately after checkout. Document the
  reason in the header comment. Do not change `web-preview.ts`; a clearer
  failure message for a missing base ref is a papercut, not this card.
- No macOS runner, no native selector, no secret, no new job, no publish step.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| PR runs the web board | open the card's own PR | a `ci-web` run appears on the PR head and executes `effigy ci:web` |
| PR runs the Rust board | same PR | a `ci-rust` run appears on the PR head and executes `effigy ci:rust` |
| Superseded heads cancel | push a second commit to the PR | the first run is cancelled by the concurrency group |
| Only Linux | inspect both workflow files | `runs-on: ubuntu-latest` only; no `macos` string |
| Release stays manual | inspect `release.yml`, `ci-native.yml`, `ci-visual.yml` | unchanged; `workflow_dispatch` only |
| Base ref is present on PR runs | remove the fetch again | `test:web-pack-install` exits 128 at `web-preview.ts:167` |
| Automation checker agrees | run `effigy check:release-automation` | pass on the new shape; planted `push` on `release.yml` and planted `schedule` on `ci-web.yml` each fail |

## Validation

Run `effigy check:release-automation`, `effigy docs:check`, and `git diff
--check origin/main...HEAD` locally. The workflow itself cannot run locally;
the card's PR is its proof. Record the run URLs and durations for both jobs in
the execution log. Never run release, windowed, or native-visual selectors.

## Owned Paths

`.github/workflows/ci-web.yml`, `.github/workflows/ci-rust.yml`,
`scripts/check-release-automation.ts` (per-workflow trigger assertions),
this card's execution log under `docs/logs/2026-09/`, and root `PAPERCUTS.md`
(append only).

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`.

## Accepted Exception (operator, 2026-09-04)

PR #201's own `ci-web` run fails at `test:web-pack-install` because the
g16.094 ordinary scope guard rejects any diff under `.github/workflows/`.
That is the guard working as designed: workflow mutation is never an
ordinary change and needs an operator-approved lane. The operator accepted
the red PR-head `ci-web` check for this PR only. Merge proceeds on the
accepted exact-head review plus the green `ci-rust` run; the first push-main
run (no changed paths against `origin/main`) is the board's green proof and
is recorded in the execution log. Ordinary PRs are unaffected. The guard's
message should name this case plainly; that is a papercut, not this card.

## Closeout

PR #201 merged after accepted exact-head review at `78e977975b` and green
`ci-rust` run `33884905792`. The accepted red PR-head `ci-web` run was the
documented workflow-scope exception. Post-merge push-main `ci-web` run
`33893736159` passed in 5m45s, including `test:components` (381 files / 3681
tests) and pack-install. This is the board's green proof.

## Stop Conditions

Stop and report when: the first PR run fails for a reason unrelated to the
trigger change (report the failing step; do not repair unrelated code in this
lane); the run exceeds twenty minutes of runner time (report and return the
budget question); GitHub refuses the workflow (permissions, allowance) — that
is operator-owned. Escalation owner: operator, via Chatterbox.

## Continuation

Once green on `main`, the board's history is the evidence that later cards
cite. A scheduled full `effigy qa` on `main` (native lanes on a self-hosted or
macOS runner) is a separate decision and is not authorized here.
