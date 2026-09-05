---
title: g16.096 Linux headless PR and main board worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-04
updated: 2026-09-04
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260904-132736-g16-096-linux-headless-pr-board.md
base_required: pushed-main
tags: [coordination, handoff, worker, g16, g16.096]
---

## What This Thread Was Doing

The coordinator is consuming the Chatterbox-promoted dispatch manifest. This
handoff dispatches the approved g16.096 workflow lane. No transcript or second
prompt is part of the authority chain.

## Why It Matters

The cheap Linux web and Rust boards are currently manual-only, so merges can
land without an automatic validation signal. This bounded lane enables PR and
main triggers while preserving manual native, visual, and release lanes.

## Current State

- **Repository:** `inflatable-cookie/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `3fe9c052767e87fe22b53369e541606e3d1e434d`
- **Pushed main verification:** `HEAD == origin/main`; promoted commit is an ancestor
- **Planning checkout:** clean
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Chatterbox dispatch manifest and
  g16.096 card
- **Worker branch:** `feature/g16-096-linux-headless-pr-board`
- **Worker worktree:** Paseo-managed worktree; use the launcher-provided path
- **Worktree creation command:** `paseo workspace create --isolation worktree --mode branch-off --new-branch feature/g16-096-linux-headless-pr-board --base origin/main`
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required.
- **Required sibling worktree links:** `none`
- **Active spec lane:** `docs/roadmaps/g16/096-linux-headless-pr-board.md`
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:** `g16.096`
- **Allowed runway:** implement g16.096 only
- **Remaining card budget:** one bounded card
- **Dispatch topology:** concurrent with g16.095; no serial edge
- **Parallel safety check:** disjoint owned paths; shared closeout is reserved
  for the coordinator
- **Surfaces this lane owns:** `.github/workflows/ci-web.yml`, `.github/workflows/ci-rust.yml`, the per-workflow trigger assertions in `scripts/check-release-automation.ts`, one execution log under `docs/logs/2026-09/`, and append-only `PAPERCUTS.md`
- **Integration ownership:** coordinator owns `docs/roadmaps/g16/README.md`, `docs/roadmaps/generation-index.md`, and `docs/roadmaps/dispatch.md`
- **Merge ordering:** same-repository PRs merge one at a time; the coordinator
  refreshes this head against current `main` if g16.095 merges first
- **Canonical refs:** `scripts/check-release-automation.ts`, `tasks/effigy.tasks.toml`; `AGENTS.md`
- **Review oracle:** the Review Oracle table in g16.096
- **Model capability profile:** capable coding model, medium reasoning
- **Worker provider/model identity:** `omp/opencode-go/deepseek-v4-flash`
- **Frontier-worker justification:** none
- **Tool/runtime restrictions:** workflow-edit authority is the operator's
  explicit 2026-09-02 approval recorded in the card; no macOS, native, visual,
  release, secret, publish, or new-job changes
- **Required validation:** `effigy check:release-automation`, `effigy docs:check`, `git diff --check origin/main...HEAD`; the PR's own `ci:web` and `ci:rust` runs are executable proof
- **PR base/head:** current pushed `main` / this worker branch
- **PR URL:** pending
- **Review state:** awaiting independent exact-head review after PR creation
- **Merge path:** orchestrator after accepted review of the current head and passing required checks

## Boundaries

- **In scope:** the two approved Linux workflow trigger changes, per-workflow
  release-automation assertions, execution log, and append-only papercuts.
- **Out of scope:** `ci-native.yml`, `ci-visual.yml`, `release.yml`, macOS
  runners, secrets, new jobs, publish steps, release mutations, and coordinator
  closeout files.
- **Outcome shape:** a complete workflow and checker change with green PR
  runs recorded in the log. Do not repair unrelated workflow or product
  failures in this lane.
- Preserve existing concurrency groups and fail-closed trigger assertions.
- Work only in the clean worker worktree selected by `Completion Protocol`.
- Do not merge the PR.

## Important Context

- **Planning lineage:** g16 maintains bounded parity and validation evidence;
  this lane restores a cheap automatic signal without making native or release
  automation implicit.
- **Why this card is ready:** no prerequisites; explicit operator workflow
  authority, owned paths, acceptance, counterexamples, validation, and
  escalation owner are recorded in the card and dispatch manifest.
- **Decisions and preferences:** web and Rust PR/main triggers are approved;
  macOS, native, visual, and release remain manual.
- **Open tensions:** GitHub permissions or allowance failures are operator
  owned; unrelated failing jobs stop this lane rather than widening scope.
- **Report after:** trigger/checker edits and local validation, then both PR
  workflow URLs, final evidence, and PR.
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then read
`AGENTS.md`, the g16 front door, the assigned card, and its canonical refs.
Start with the planted trigger counterexamples before finalizing assertions.

## Completion Protocol

Follow the shared worker completion protocol in the Northstar orchestrator
contract. In particular, verify the launcher worktree before broad reads,
confirm the tracked handoff at the pushed base, work only in the owned paths,
falsify every Review Oracle row, push the branch, and open a reviewable PR.
The coordinator owns review dispatch, merge, and the reserved closeout.

- **Closeout refs:** `docs/roadmaps/g16/096-linux-headless-pr-board.md`, the
  execution log, `docs/roadmaps/g16/README.md`,
  `docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`
