---
title: Acowtancy released-Underlay boundary correction worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-25
updated: 2026-08-25
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260825-160317-g16-014-acowtancy-release-boundary-correction.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, acowtancy]
---

## What This Thread Was Doing

Acowtancy PRs 54 and 55 moved its declared web and Rust dependencies to Poodle
0.2.2 and Underlay `v0.9.2`. Post-merge review found that the Svelte toolchain
still compiles Underlay from the sibling checkout through committed aliases and
TypeScript paths. This worker removes that last application-source bypass.

## Why It Matters

The consumer rollout is meant to prove the released package in a clean clone.
A correct manifest and lock are not enough when Vite or TypeScript silently
substitutes `../underlay/ts/src` during the supported build.

## Current State

- **Worker repository:** `/Users/tom/Dev/projects/acowtancy`
- **Target base:** `3387275eaa5d756b03aaf541455befe961ea37cf`, verified as
  `origin/main` on 2026-08-25
- **Poodle planning repository:** `/Users/tom/Dev/projects/poodle`
- **Planning base:** `74ee709bd21260ed3153b4bb966a60426c542db5`, pushed to
  `origin/main` before this handoff commit
- **Planning checkout:** clean `main`, with `HEAD == origin/main` before this
  handoff
- **Worker mode:** activated by this orchestrator handoff
- **Worker branch:** launcher-generated non-`main`; suggested identity
  `t3code/g16-014-acowtancy-release-boundary`
- **Worker worktree:** use the launcher's clean registered worktree; none was
  created by the orchestrator
- **Roadmap:** `/Users/tom/Dev/projects/poodle/docs/roadmaps/g16/README.md`
- **Ready card:** `/Users/tom/Dev/projects/poodle/docs/roadmaps/g16/014-acowtancy-poodle-v022-adoption.md`
- **Allowed runway:** the `g16.014` post-merge correction only
- **Remaining budget:** one repository and one corrective PR
- **Parallel safety:** independent of Compli Me; stop if another Acowtancy lane
  owns the same Svelte/Vite/TypeScript configuration files
- **Canonical refs:** Poodle `g16.001`, `g16.014`, Acowtancy root/scoped
  `AGENTS.md`, working rules, and repo-local Effigy skill
- **Required validation:** `effigy workspace:js:prepare`,
  `effigy cream/validate`, `effigy dairy/validate`, `effigy froyo/check`,
  `effigy cattle-grid/validate`, `effigy validate`, `effigy qa`, and
  `git diff --check`; separate reproduced baselines
- **PR base/head:** Acowtancy `main` <- worker branch
- **PR URL:** pending
- **Review state:** awaiting orchestrator review
- **Merge authorisation:** worker must not merge

## Boundaries

- Remove active sibling Underlay aliases/path mappings from Cream and Dairy
  Svelte/Vite config and Froyo/Cattle Grid TypeScript config.
- Resolve application imports through installed
  `git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.2` only.
- Keep sibling mounts and scripts used solely for explicit conformance,
  template, or guardrail QA. They must not affect application compilation.
- Keep Poodle versions, Underlay manifests, Rust sources, product behavior, and
  deployment unchanged. Refresh the root Bun lock only if supported tooling
  requires it; reject unrelated churn.
- Do not edit Poodle or Underlay, add aliases/fallbacks, execute another
  Acowtancy card, or merge the PR.

## Important Context

- PR 54 merged web manifests and locks at `6e76b943`; PR 55 merged Farmyard's
  Rust tag at `03ef5736`.
- Known bypasses are in `apps/cream/svelte.config.js`,
  `apps/cream/vite.config.ts`, `apps/dairy/svelte.config.js`,
  `apps/dairy/vite.config.ts`, `packages/froyo/tsconfig.json`, and
  `packages/cattle-grid/tsconfig.json`. Re-inventory before editing.
- Acowtancy deliberately mounts sibling Underlay/Poodle for workspace QA. That
  tooling relationship stays; only application source substitution goes.
- Report after source convergence, then after validation. Stop if the released
  package lacks an export required by Acowtancy—the fix then belongs upstream.

## Suggested Next Move

Start with the worker preflight, then inspect the six known config files and the
effective package graph. Remove the bypasses as one coherent batch, validate
the four web consumers against the installed tag, and open the corrective PR.

## Completion Protocol

### Before you start

1. This file activates worker mode. Before broad reads, run
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. Use the current context only when it is a clean, registered, non-`main`
   launcher worktree. Record its real root/branch; generated names may differ.
3. If the launcher supplied `main`, a dirty checkout, or an unregistered path,
   stop and report it. Never clean or reset it. Manual fallback requires
   Acowtancy's `.agents.local.env` `AGENTS_WORKTREE_CONTAINER_DIR`; never guess
   a path or use `/tmp`.
4. Fetch origin and confirm selected `HEAD == origin/main == 3387275e`. Confirm
   this handoff exists at the absolute Poodle path and planning base `74ee709b`
   is an ancestor of Poodle `origin/main`.
5. Read the card, Acowtancy instructions/working rules, and its local Effigy
   skill. Run the cheap Effigy orientation from the selected worktree.

### While you work

- Keep the diff to application resolution config and a mechanically required
  root lock change. Inspect the complete diff before validation.
- Stop on a missing release export, unrelated lock churn, overlapping work, or
  any change that would require Underlay/Poodle/product decisions.
- Report changed files, validation, remaining work, and blockers to the
  operator after each meaningful chunk.

### When the assigned runway is complete

1. Run the required validation and `git diff --check`.
2. Record removed bypasses, resolved Underlay commit, Poodle identities, lock
   review, validation, actual worktree, and branch in the PR.
3. Push the worker branch and open one PR against Acowtancy `main`.
4. Link `g16.014`, report the PR URL, and do not merge or edit Poodle planning.

### Review and merge path

The Poodle orchestrator reviews the diff and checks independently. Shared
GitHub identity may require a canonical PR comment. Make only requested fixes;
merge needs separate operator authority.

### Handoff closeout

Leave the PR evidence honest. The orchestrator owns card, log, runway, and
next-task closeout after merge.
