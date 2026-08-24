---
title: Soundcheck Library Poodle 0.2.2 follow-up worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-24
updated: 2026-08-24
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260824-205429-g16-010-soundcheck-library-v022-follow-up.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, soundcheck-library]
---

## What This Thread Was Doing

Soundcheck Library already moved off sibling Poodle overrides and adopted
public 0.2.1. This follow-up advances its exact development dependency, two
published peer lines, and both lockfiles to Poodle 0.2.2.

## Why It Matters

Soundcheck itself depends on both Longhorn and Soundcheck Library. Closing this
independent foundation lane now leaves the later app migration waiting only on
Longhorn, while preserving honest peer and lock evidence for library consumers.

## Current State

- **Worker repository:** `/Users/tom/Dev/projects/soundcheck-library`
- **Target base:** `a720f22f5bb08ae465ba3dd46873855fec9b7c72`
- **Poodle planning repository:** `/Users/tom/Dev/projects/poodle`
- **Planning base:** `3434a32d1a3e2e94ea67f225f2739c4f9f6355ef`, pushed to `origin/main`
- **Worker mode:** activated by this orchestrator handoff
- **Worker branch:** launcher-generated non-`main`; suggested identity
  `t3code/g16-010-soundcheck-library-v022`
- **Worker worktree:** launcher-managed; no manual worktree pre-created
- **Roadmap card:**
  `/Users/tom/Dev/projects/poodle/docs/roadmaps/g16/010-soundcheck-library-poodle-v022-follow-up.md`
- **Allowed runway:** g16.010 only
- **Parallel topology:** independent of Longhorn and Underlay
- **Required validation:** `npm ci`, `npm test`, `npm run check`, focused
  package compilation as needed, dual-lock review, `git diff --check`
- **PR:** Soundcheck Library `main` <- worker branch; pending; do not merge

## Boundaries

- Change the root exact Poodle Svelte pin to 0.2.2, both peers to `^0.2.2`, and
  only the corresponding Bun/npm lock entries.
- Preserve platform lock constraints restored during the 0.2.1 PR review.
- Do not publish/version the library, expand its component API, alter Rust
  behavior, add wrappers/aliases, edit Poodle, or touch Soundcheck itself.
- Do not disguise the target base's pre-existing 18 type errors and one failing
  test as adoption regressions or silently fix them out of scope.

## Important Context

- PR 5 established the current 0.2.1 registry state. Review had to restore ten
  unrelated platform `libc` constraints removed by npm lock regeneration.
- The accepted baseline was 179 passing tests, 18 type errors, and one failing
  test. Reproduce that base when classification is necessary; this card must
  demonstrate no new Poodle-caused failure.
- Poodle core/Svelte 0.2.2 are public npm `latest`; libraries advertise
  `^0.2.2`, while the development root stays exact.
- Report after lock review and again after baseline-comparative validation.

## Suggested Next Move

Read g16.010, inspect the existing npm and Bun lock entries, then change all
three manifest declarations together. Regenerate locks with the narrowest
supported command and review them before running tests. If platform constraints
drop or unrelated packages move, restore the unaffected entries rather than
accepting lock churn.

## Completion Protocol

### Before you start

1. Run the quick worker preflight: repository root, branch,
   `git status --porcelain`, and registered worktrees.
2. Use only a clean, registered, non-`main` Soundcheck Library worktree supplied
   by the launcher. Record its actual generated path/branch.
3. If the launcher context is dirty, `main`, or unregistered, stop and tell the
   operator. Do not clean or silently create another worktree. Any manual
   fallback needs an operator-selected target-repo worktree container.
4. Fetch and confirm `HEAD == origin/main == a720f22`. Confirm the absolute
   Poodle handoff exists and planning base `3434a32d` is an ancestor of Poodle
   `origin/main`; the handoff is not expected in Soundcheck Library `HEAD`.
5. Read g16.010, the package READMEs, root README, and `PAPERCUTS.md`. The repo
   currently has no root `AGENTS.md`; do not invent one in this lane.

### While you work

- Keep the manifest and dual-lock migration one coherent batch.
- Classify validation against the target-base baseline before changing code.
- Stop on release-policy decisions, unrelated lock churn, lost platform
  constraints, or a new Poodle defect.

### When the assigned runway is complete

1. Run all g16.010 validation, inspect both locks, and run `git diff --check`.
2. Push the worker branch and open a PR against Soundcheck Library `main`.
   Record exact resolved versions, baseline comparison, changed files, and
   validation in the PR body.
3. Report the PR URL to the operator. Do not merge or edit Poodle planning docs.

### Review and merge path

The Poodle orchestrator will independently review the PR and record a verdict.
Make requested fixes only on this worker branch. Merge requires separate
operator authority.
