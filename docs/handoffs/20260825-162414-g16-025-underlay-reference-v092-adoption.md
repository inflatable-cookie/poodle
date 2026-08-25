---
title: Underlay Reference 0.9.2 adoption worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-25
updated: 2026-08-25
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260825-162414-g16-025-underlay-reference-v092-adoption.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, underlay-reference]
---

## What This Thread Was Doing

Underlay Reference already consumes exact public Poodle 0.2.2, but its four web
packages and Rust API still resolve Underlay from the sibling checkout. This
worker moves that entire active graph to released tag `v0.9.2` without reopening
the completed Poodle migration.

## Why It Matters

The reference app is the clean template proof for Underlay consumers. Its
validation only counts when the installed Git tag—not the adjacent source
tree—supplies every active web package and Rust crate.

## Current State

- **Worker repository:** `/Users/tom/Dev/projects/underlay-reference`
- **Target base:** `f5ea7d72eee278e8838ba16f8f43eb2b662406d0`, verified as
  current `origin/main` on 2026-08-25
- **Poodle planning repository/base:** `/Users/tom/Dev/projects/poodle` at
  pushed `3a9b69b471259539b5c13585ae06e42cf5fa2d63`
- **Planning checkout:** clean `main`, with `HEAD == origin/main`
- **Worker branch:** launcher-generated non-`main`; suggested
  `t3code/g16-025-underlay-reference-v092`
- **Worker worktree:** use a fresh clean launcher worktree
- **Ready card:** `/Users/tom/Dev/projects/poodle/docs/roadmaps/g16/025-underlay-reference-v092-adoption.md`
- **Allowed runway:** `g16.025` only; one repository and one PR
- **Parallel safety:** no open PR exists. A stale dirty worktree remains at
  `/Users/tom/.t3/worktrees/underlay-reference/t3code-4e9fc1d9` on a gone
  branch. Preserve it; never use, clean, reset, or delete it. Stop if the
  operator says it is active.
- **Canonical refs:** Poodle `g16.001`, `g16.013`, `g16.025`; Underlay Reference
  root/scoped `AGENTS.md`, reference notes, template/adapter contracts, and
  repo-local Effigy skill
- **Required validation:** supported prepare/install, package-local checks
  exposed by `effigy tasks`, `effigy validate`, `effigy qa`,
  `effigy qa:docs`, and `git diff --check`; reproduce the known routing
  baseline separately and do not launch the live stack
- **PR base/head:** Underlay Reference `main` <- worker branch
- **PR URL:** pending; review awaiting orchestrator; worker must not merge

## Boundaries

- Move Admin, Front, UI, and client to
  `git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.2`.
- Move every active API Underlay crate to the same Git repository with
  `tag = "v0.9.2"`, preserving features. Include any direct non-workspace path.
- Regenerate all four Bun locks and Rust locks narrowly; preserve exact registry
  Poodle 0.2.2 and resolve Underlay commit `ddba2640` everywhere.
- Remove active sibling Underlay aliases, paths, or generator imports that can
  bypass the installed package. Explicit cross-repository QA mounts/scripts may
  remain if they do not affect application resolution.
- Do not edit Poodle/Underlay, templates/public APIs, app structure, validation
  policy, the stale worktree, or live stack. Do not add exceptions or shims.

## Important Context

- `g16.013` already proved public Poodle 0.2.2; this card preserves that result.
- Current web manifests use `file:../../underlay`; API workspace crates and
  direct `underlay-config` use sibling paths.
- The stale dirty worktree contains broad deletions and old adoption changes.
  It is not evidence and not cleanup scope; the new worker starts from current
  `origin/main` in a separate worktree.
- The previous Poodle adoption exposed a pre-existing Effigy test-routing
  baseline. Reproduce it honestly rather than weakening checks.
- Report after complete source/lock convergence, then after validation.

## Suggested Next Move

Run the worker preflight, select only the fresh launcher worktree, and read
`g16.025` plus Underlay Reference authority. Inventory all manifests, locks,
aliases, and generator imports before changing the graph as one coherent batch.

## Completion Protocol

### Before you start

1. This handoff activates worker mode. Run the four-command
   root/branch/status/worktree preflight before broad reads.
2. Use only a clean registered non-`main` launcher worktree. Do not use or
   clean the stale dirty worktree. If the launcher context is unusable, stop.
   Manual fallback requires Underlay Reference's `.agents.local.env`
   `AGENTS_WORKTREE_CONTAINER_DIR`; never guess or use `/tmp`.
3. Fetch origin; confirm `HEAD == origin/main == f5ea7d72`. Confirm this
   handoff exists and planning base `3a9b69b4` is an ancestor of Poodle
   `origin/main`.
4. Recheck worktrees/open PRs, then read instructions, card, contracts, and the
   local Effigy skill. Stop if another active lane owns manifests or locks.

### While you work

Keep web/Rust sources aligned to one tag and inspect every lock hunk. Report
meaningful chunks. Stop on missing tag exports/features, duplicate/local
Poodle, unrelated churn, app exceptions, or visible proof.

### When the assigned runway is complete

Run all required checks and `git diff --check`. Record exact tag/commit,
preserved Poodle identities, complete manifest/lock/source-bypass review,
compatibility edits, worktree, and branch in one PR against current `main`.
Link `g16.025`, report the URL, and do not merge.

### Review and merge path

The Poodle orchestrator reviews independently; shared identity may require a
canonical PR comment. Merge needs separate operator authority.

### Handoff closeout

Leave evidence honest. The orchestrator owns planning closeout after merge.
