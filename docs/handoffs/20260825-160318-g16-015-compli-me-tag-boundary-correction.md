---
title: Compli Me Underlay tag-boundary correction worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-25
updated: 2026-08-25
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260825-160318-g16-015-compli-me-tag-boundary-correction.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, compli-me]
---

## What This Thread Was Doing

Compli Me PRs 1 and 2 moved its graph to Poodle 0.2.2 and Underlay's `v0.9.2`
commit. Post-merge review found that the four JavaScript manifests name the
peeled commit directly, while every other consumer and Compli Me's Rust graph
declare the release tag. This worker restores that tag boundary without
changing the resolved code.

## Why It Matters

The rollout needs one recognizable release identity across consumers. Keeping
the lock at the immutable release commit is correct; keeping a one-off manifest
syntax because one install attempt failed makes future upgrades and audits less
predictable.

## Current State

- **Worker repository:** `/Users/tom/Dev/projects/compli-me`
- **Target base:** `e2744629612d9513c2a4207783f3f0872b139db5`, verified as
  `origin/main` on 2026-08-25
- **Poodle planning repository:** `/Users/tom/Dev/projects/poodle`
- **Planning base:** `74ee709bd21260ed3153b4bb966a60426c542db5`, pushed to
  `origin/main` before this handoff commit
- **Planning checkout:** clean `main`, with `HEAD == origin/main` before this
  handoff
- **Worker mode:** activated by this orchestrator handoff
- **Worker branch:** launcher-generated non-`main`; suggested identity
  `t3code/g16-015-compli-me-tag-boundary`
- **Worker worktree:** use the launcher's clean registered worktree; none was
  created by the orchestrator
- **Roadmap:** `/Users/tom/Dev/projects/poodle/docs/roadmaps/g16/README.md`
- **Ready card:** `/Users/tom/Dev/projects/poodle/docs/roadmaps/g16/015-compli-me-poodle-v022-adoption.md`
- **Allowed runway:** the `g16.015` post-merge correction only
- **Remaining budget:** one repository and one corrective PR
- **Parallel safety:** independent of Acowtancy; stop if another Compli Me lane
  owns the four package manifests or Bun locks
- **Canonical refs:** Poodle `g16.001`, `g16.015`, Compli Me root/scoped
  `AGENTS.md`, working rules, and repo-local Effigy skill
- **Required validation:** frozen/supported install plus
  `effigy admin/validate`, `effigy front/validate`, `effigy ui/validate`,
  `effigy api-client/validate`, `effigy validate`, `effigy qa`, and
  `git diff --check`; API/Rust validation is required only if its files move
- **PR base/head:** Compli Me `main` <- worker branch
- **PR URL:** pending
- **Review state:** awaiting orchestrator review
- **Merge authorisation:** worker must not merge

## Boundaries

- Change Admin, Front, UI, and API-client Underlay manifest refs from
  `#ddba26400f480638829917cf72eecc62be4b978d` to `#v0.9.2`.
- Refresh all affected Bun locks narrowly. They must continue resolving exact
  commit `ddba26400f480638829917cf72eecc62be4b978d` and registry Poodle 0.2.2.
- Do not change Rust manifests/locks, application code, Effigy configuration,
  product behavior, deployment, Poodle, or Underlay.
- If Bun still rejects the tag, capture the exact supported command, Bun
  version, and output; compare against current tag-consuming repositories and
  stop instead of restoring the commit syntax or widening scope.
- Do not add overrides, aliases, fallbacks, unrelated upgrades, or merge the PR.

## Important Context

- PR 1 merged public Poodle 0.2.2 at `2d38493e`; PR 2 merged Underlay release
  commit `ddba2640` at `e2744629`.
- Rust already uses `tag = "v0.9.2"`; only the four web manifests and their
  lock metadata are in scope.
- Acowtancy, Composer, Contact Patch, and Songsprout already declare the same
  `git+ssh` URL with `#v0.9.2`. A Compli-specific failure needs evidence, not a
  permanent silent exception.
- Explicit sibling mounts/scripts used for cross-repository QA remain outside
  package resolution and are not part of this correction.
- Report after the tag resolves and locks are reviewed, then after validation.

## Suggested Next Move

Start with the worker preflight, inventory the four manifest/lock pairs, and
record the supported Bun version. Change all four refs together, use the
repo-owned install path, prove the same release commit, then open the PR.

## Completion Protocol

### Before you start

1. This file activates worker mode. Before broad reads, run
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. Use the current context only when it is a clean, registered, non-`main`
   launcher worktree. Record its real root/branch; generated names may differ.
3. If the launcher supplied `main`, a dirty checkout, or an unregistered path,
   stop and report it. Never clean or reset it. Manual fallback requires Compli
   Me's `.agents.local.env` `AGENTS_WORKTREE_CONTAINER_DIR`; never guess a path
   or use `/tmp`.
4. Fetch origin and confirm selected `HEAD == origin/main == e2744629`. Confirm
   this handoff exists at the absolute Poodle path and planning base `74ee709b`
   is an ancestor of Poodle `origin/main`.
5. Read the card, Compli Me instructions/working rules, and its local Effigy
   skill. Run the cheap Effigy orientation from the selected worktree.

### While you work

- Keep the diff to four manifests and mechanically changed Bun locks. Inspect
  every lock hunk and reject unrelated resolution churn.
- Stop on a reproducible tag-resolution failure, overlapping work, or any need
  for source, Rust, deployment, or dependency-policy changes.
- Report changed files, validation, remaining work, and blockers to the
  operator after each meaningful chunk.

### When the assigned runway is complete

1. Run the required validation and `git diff --check`.
2. Record Bun version, supported install command, manifest refs, resolved
   Underlay/Poodle identities, lock review, validation, actual worktree, and
   branch in the PR.
3. Push the worker branch and open one PR against Compli Me `main`.
4. Link `g16.015`, report the PR URL, and do not merge or edit Poodle planning.

### Review and merge path

The Poodle orchestrator reviews the diff and checks independently. Shared
GitHub identity may require a canonical PR comment. Make only requested fixes;
merge needs separate operator authority.

### Handoff closeout

Leave the PR evidence honest. The orchestrator owns card, log, runway, and
next-task closeout after merge.
