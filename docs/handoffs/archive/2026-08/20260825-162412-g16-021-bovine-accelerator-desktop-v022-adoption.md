---
title: Bovine Accelerator Desktop Poodle 0.2.2 adoption worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-25
updated: 2026-08-25
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260825-162412-g16-021-bovine-accelerator-desktop-v022-adoption.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, bovine-accelerator-desktop]
---

## What This Thread Was Doing

Bovine Accelerator Desktop still consumes Poodle core/Svelte through sibling
paths and overrides. This worker moves it to exact registry 0.2.2 and proves
that the packed icon builder still supports the application.

## Why It Matters

This is the strongest source-independence check in the Longhorn product wave:
the app uses a Poodle build utility as well as runtime components. A passing
result proves the package, not merely its component imports.

## Current State

- **Worker repository:** `/Users/tom/Dev/projects/bovine-accelerator-desktop`
- **Target base:** `ac7487fd82e9792b14f1c499f4342182914501da`, refreshed and
  verified as current `origin/main` on 2026-08-25
- **Poodle planning repository/base:** `/Users/tom/Dev/projects/poodle` at
  pushed `3a9b69b471259539b5c13585ae06e42cf5fa2d63`
- **Planning checkout:** clean `main`, with `HEAD == origin/main`
- **Worker branch:** launcher-generated non-`main`; suggested
  `t3code/g16-021-bovine-desktop-v022`
- **Worker worktree:** use a fresh isolated launcher worktree
- **Ready card:** `/Users/tom/Dev/projects/poodle/docs/roadmaps/g15/075-bovine-accelerator-desktop-poodle-v022-adoption.md`
- **Allowed runway:** `g15.075` only; one repository and one PR
- **Parallel safety:** two existing product worktrees are registered but no PR
  is open. Preserve them and stop if either now owns `package.json`, `bun.lock`,
  or icon-generation files.
- **Canonical refs:** Poodle `g15.055`, `g15.065`, `g15.075`; Bovine Desktop
  root/scoped `AGENTS.md`, working rules, dependency-release checks, and local
  Effigy skill
- **Required validation:** supported dependency preparation,
  `effigy check:dependencies:release`,
  `effigy check:dependencies:release:source-independent`,
  `effigy check:frontend`, `effigy test:desktop`, `effigy qa`, and
  `git diff --check`; avoid visible proof selectors
- **PR base/head:** Bovine Desktop `main` <- worker branch
- **PR URL:** pending; review awaiting orchestrator; worker must not merge

## Boundaries

- Replace core/Svelte sibling dependencies and Poodle overrides with exact
  registry `0.2.2`; retain Longhorn local dependency/override.
- Regenerate `bun.lock` narrowly and keep the published icon-builder command
  working from installed Poodle core.
- Do not alter existing worktrees, Rust domain behavior, Tauri commands,
  content, Longhorn/Poodle, release automation, or visible apps.
- Reject aliases, shims, unrelated upgrades, and source-only fallback paths.

## Important Context

- The roadmap's old base was stale; this handoff and card now use `ac7487fd`.
- The root checkout is clean, but the lane still requires its own worker because
  other product worktrees exist and the operator checkout is not worker scope.
- A missing packed icon-builder file is a stop condition and possible Poodle
  release defect, not permission to relink the sibling source.
- Report after dependency/source-independent proof, then after full validation.

## Suggested Next Move

Run the worker preflight, recheck the two existing product worktrees, then read
the card and Bovine authority. Use the supported prepare path, review the whole
lock diff, and run the release/source-independence checks before compile fixes.

## Completion Protocol

### Before you start

1. This handoff activates worker mode. Run the four-command
   root/branch/status/worktree preflight before broad reads.
2. Use only a clean registered non-`main` launcher worktree. Never reuse or
   clean the operator/product worktrees. If the launcher context is unusable,
   stop. Manual fallback requires Bovine Desktop's `.agents.local.env`
   `AGENTS_WORKTREE_CONTAINER_DIR`; never guess or use `/tmp`.
3. Fetch origin; confirm `HEAD == origin/main == ac7487fd`. Confirm this
   handoff exists and planning base `3a9b69b4` is an ancestor of Poodle
   `origin/main`.
4. Recheck worktrees/open PRs; read the card, repo instructions, and local
   Effigy skill. Stop on manifest/lock overlap.

### While you work

Keep the change to one dependency/lock batch plus bounded app-owned fallout.
Inspect all lock churn. Report meaningful chunks and stop on missing published
assets, shared scope, unrelated updates, or product/API decisions.

### When the assigned runway is complete

Run the card validation and `git diff --check`. Record registry/integrity,
source-independent and icon evidence, peer convergence, lock review, worktree,
and branch in one PR against current `main`. Link `g15.075`, report, do not merge.

### Review and merge path

The Poodle orchestrator independently reviews the diff and checks; shared
identity may require a canonical PR comment. Merge needs operator authority.

### Handoff closeout

Leave evidence honest. The orchestrator owns planning closeout after merge.
