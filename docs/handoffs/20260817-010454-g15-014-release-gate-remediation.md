---
title: g15.014 release-gate remediation worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-17
updated: 2026-08-17
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260817-010454-g15-014-release-gate-remediation.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, security]
---

## What This Thread Was Doing

Poodle's g15 runway is closing the honest v0.2.0 Svelte release baseline.
Focused web evidence is under repair in PR #29. This independent worker owns
the remaining release-gate advisory only: GHSA-2v37-7h3g-55p8 in `nanoid`,
reached through the React preview's Vite dependency.

This is one dependency-remediation card, not a component or release run.

## Why It Matters

`effigy qa` remains red while `bun audit` reports the high-severity advisory.
Poodle does not waive release gates. g15.014 must land before v0.2.0
certification can begin.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `cf25abe1a563dba12482e9f2f14955ccc05f4029`
- **Pushed main verification:** local `HEAD` equalled `origin/main` at
  `cf25abe1a563dba12482e9f2f14955ccc05f4029` before this handoff was written
- **Planning checkout:** clean before this handoff was written
- **Planning artifacts included at the base:** compact AGENTS instruction
  surface and g15 state through PR #28; this handoff's planning commit follows
  the recorded base
- **Worker branch:** `t3code/g15-014-release-gate-remediation`
- **Worker worktree:** `/Users/tom/.t3/worktrees/poodle/g15-014-release-gate-remediation`
- **Worktree creation command:** `git fetch origin && git worktree add /Users/tom/.t3/worktrees/poodle/g15-014-release-gate-remediation -b t3code/g15-014-release-gate-remediation origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and do
  not create another worktree. If the current context is unusable, inspect the
  named worktree, then read `.agents.local.env` and require
  `AGENTS_WORKTREE_CONTAINER_DIR` for any manual fallback. Never guess `/tmp`,
  `TMPDIR`, or a repository-adjacent path.
- **Active spec lane:** none; this is bounded release-gate remediation
- **Roadmap milestone:** `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/README.md`
- **Ready cards, in order:** `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/014-release-gate-remediation.md`
- **Allowed runway:** g15.014 only, batches A then B
- **Remaining card budget:** one card, two meaningful batches
- **Dispatch topology:** parallel with PR #29 repair
- **Parallel safety check:** g15.014 writes dependency manifests, lockfiles,
  version-pinned config, one unique log, and optional append-only papercuts.
  PR #29 writes component code/tests, one component contract, roster/register,
  and its own log. Do not enter those PR #29 surfaces.
- **Canonical refs:** `/Users/tom/Dev/projects/poodle/AGENTS.md`,
  `/Users/tom/Dev/projects/poodle/docs/contracts/001-working-rules.md`,
  `/Users/tom/Dev/projects/poodle/docs/contracts/005-agent-local-paths.md`,
  `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/release-gap-register.md`
- **Model capability profile:** capable coding model, medium reasoning; stop
  before any broad or breaking dependency migration
- **Tool/runtime restrictions:** no component, contract, specimen, public API,
  release mutation, native visual, windowed, or Jetstream work
- **Required validation:** `effigy audit:security`, `effigy qa`,
  `effigy react:build`, `effigy test:components`, `effigy docs:check`, and
  `git diff --check origin/main...HEAD`
- **PR base/head:** `main` ← `t3code/g15-014-release-gate-remediation`
- **PR URL:** pending
- **Review state:** awaiting implementation and orchestrator review
- **Merge authorisation:** worker must not merge; operator retains merge
  authority

## Boundaries

- **In scope:** identify the exact React preview → Vite → nanoid dependency
  path; make the smallest manifest/lockfile or version-pinned configuration
  change that removes GHSA-2v37-7h3g-55p8; record one August batch log; append
  a new papercut only if execution exposes distinct friction.
- **Out of scope:** advisory suppression, unrelated upgrades, component source,
  tests changed to accommodate behavior, component contracts, specimens,
  roster/register edits, public package API changes, PR #29 repairs, release
  preparation, tagging, publishing, and later g15 cards.
- Do not use `bun update` as an unbounded sweep. Inspect the dependency graph
  first and keep lockfile churn attributable to the chosen remediation.
- If the smallest supported fix requires a breaking Vite or toolchain change,
  alters preview behavior, or expands beyond the card, stop and report the
  exact choice to the operator.
- Do not edit roadmap status, generation front doors, or the dispatch ledger.
- Work only in the selected clean worker worktree. Never edit the orchestrator's
  `main` checkout or discard another checkout's dirty state.
- Do not merge the PR.

## Important Context

- **Planning lineage:** g15.001 recorded the advisory; g15.013 requires every
  `effigy qa` lane green and cannot waive it.
- **Current evidence:** on 2026-08-17, `effigy audit:security` failed on current
  main with one high vulnerability: `nanoid <3.3.18`, dependency path
  `workspace:@inflatable-cookie/poodle-react-preview › vite`.
- **Why this card is ready:** the defect, dependency path, writable scope,
  validation, acceptance, and stop conditions are explicit. It has no data or
  mutable-file dependency on PR #29.
- **Decision:** remediate; never suppress or bypass. Prefer the smallest
  supported dependency-chain change with no unrelated churn.
- **Known risk:** a broad package-manager update can rewrite unrelated lockfile
  entries. Measure the diff and stop if the change cannot stay bounded.
- **Baseline health:** Effigy doctor currently reports generated-in-src,
  god-file, and stale-suppression findings. They predate this lane and are not
  remediation scope.
- **Report after:** Batch A dependency-path finding and proposed exact change;
  Batch B implementation and final gates
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Read this handoff first, then run the worktree-safety preflight below. In the
selected worker worktree, inspect the React preview manifest and Bun lockfile,
prove the dependency path, and identify the narrow supported version change.
Report the Batch A plan before applying it if it crosses a major dependency or
toolchain boundary; otherwise continue through the bounded remediation.

## Completion Protocol

### Before you start

1. Read this handoff, then run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain` before broad repository reads.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch and do not compare it with the placeholders above or
   create another worktree merely because it differs.
3. If the launcher supplied a dirty or `main` worktree, stop and report it. If
   no launcher worktree exists, inspect the named worktree; only then read
   `.agents.local.env` as data and require an absolute
   `AGENTS_WORKTREE_CONTAINER_DIR` outside the repository for a manual
   fallback. Ask the operator if it is absent or invalid. Never use `/tmp`,
   `TMPDIR`, or a guessed path; never clean, reset, stash over, or discard an
   existing checkout.
4. From the selected worktree, run `git fetch origin`; confirm `HEAD` equals
   `origin/main`; confirm
   `git merge-base --is-ancestor cf25abe1a563dba12482e9f2f14955ccc05f4029 HEAD`
   succeeds; confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, the g15 milestone, g15.014, the release gap register,
   working rules, and agent-local-path contract.
6. Use Effigy for cheap orientation. Do not refresh the graph unless code
   navigation genuinely needs it.

### While you work

- Execute Batch A then Batch B. Keep commits aligned with those meaningful
  chunks, not model turns.
- Report the exact dependency path, chosen remediation, changed manifests or
  lockfile surfaces, validation run, remaining work, and blockers.
- Stop if the remediation requires unrelated upgrades, behavior/API changes,
  a gate bypass, or a release mutation.

### When the assigned runway is complete

1. Run `effigy audit:security`, `effigy qa`, `effigy react:build`,
   `effigy test:components`, `effigy docs:check`, and
   `git diff --check origin/main...HEAD`. Do not substitute a bare diff check.
2. Add one honest August log under `docs/logs/2026-08/`. Do not edit card or
   roadmap status, front doors, roster/register, or dispatch ledger.
3. Rebase or merge current `origin/main` before final validation if it moved.
   Resolve only additive log/PAPERCUTS overlap; stop on any unexpected shared
   mutable-file conflict.
4. Push the selected worker branch and open a reviewable PR against current
   `main`.
5. Link this handoff, g15.014, the milestone, exact dependency diff, audit
   evidence, validation, and unresolved items in the PR body.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator reviews the PR independently against the card, dependency
diff, lockfile churn, checks, and release-gate evidence. Because worker and
orchestrator share a GitHub identity, the canonical verdict is a PR comment.
Requested changes are none yet. The operator must explicitly authorise merge.

- **Closeout refs:** `docs/roadmaps/g15/014-release-gate-remediation.md`,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`,
  `docs/roadmaps/README.md`, `docs/roadmaps/dispatch.md`, and the worker's
  August log

### Handoff closeout

Leave the dependency diff and validation evidence honest. If the advisory
cannot be removed inside the bounded scope, record the exact blocker and stop.
Do not advance g15.013 or perform a release mutation.
