---
title: Underlay Reference Poodle 0.2.2 adoption worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-24
updated: 2026-08-24
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260824-231358-g16-013-underlay-reference-v022-adoption.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, underlay-reference]
---

## What This Thread Was Doing

Underlay now consumes public Poodle 0.2.2. This worker moves its canonical
reference estate—UI package, admin app, and front app—from Poodle 0.1.0 plus
committed sibling overrides to exact registry 0.2.2. This handoff is complete
without the originating transcript.

## Why It Matters

Underlay Reference is the canonical reusable application shape. Its adoption
proves the release through three linked packages without teaching applications
about Poodle or hiding registry drift behind the mounted sibling checkout.

## Current State

- **Worker repository:** `/Users/tom/Dev/projects/underlay-reference`
- **Target base:** `8885661e10813bb9d8a3f6782c87a840b26bd0be`
- **Poodle planning repository:** `/Users/tom/Dev/projects/poodle`
- **Planning base:** `a7293ecd43bb6715afea8c061c27223d39b0b1aa`, pushed to
  `origin/main` before this handoff commit
- **Planning checkout:** clean and synchronized before this planning batch
- **Worker mode:** activated by this orchestrator handoff
- **Worker branch:** launcher-generated non-`main` branch; suggested identity
  `t3code/g16-013-underlay-reference-v022`
- **Worker worktree:** use the launcher-provided registered worktree; no manual
  worktree has been pre-created
- **Roadmap card:**
  `/Users/tom/Dev/projects/poodle/docs/roadmaps/g16/013-underlay-reference-poodle-v022-adoption.md`
- **Allowed runway:** that one card only
- **Parallel topology:** runs beside Nucleus `011` and Soundcheck `012`; no
  shared mutable repository files
- **Required validation:** card validation through Underlay Reference's
  Effigy-owned headless/container flow
- **PR:** Underlay Reference `main` <- worker branch; pending
- **Review state:** awaiting orchestrator review; worker must not merge

## Boundaries

- Update only the three Poodle-consuming reference package manifests, their
  locks, and bounded 0.2.2 compatibility fallout.
- Keep local Underlay and reference-package links intact. Remove only Poodle
  source overrides; prove registry resolution despite the sibling Poodle mount.
- Do not edit Underlay or Poodle, change template/public APIs, invent app
  exceptions, publish packages, or restructure the reference workspace.
- Use the clean launcher-provided Underlay Reference worktree. Do not edit main
  checkouts and do not merge the PR.

## Important Context

- `acme-admin`, `acme-front`, and `acme-ui` declare Poodle 0.1.0 ranges and
  committed `../../poodle` overrides. Four package-local Bun locks exist.
- Local Underlay `file:` dependencies are intentional and remain. Underlay PR 5
  already proves its adapter against registry Poodle 0.2.2.
- The repository treats its live stack as Effigy/container-owned. Follow that
  flow; a host-side raw install is not evidence that the runtime changed.
- Report after manifest/lock convergence and after full validation.

## Suggested Next Move

Read `g16.013`, inventory the three manifests and four locks, then use the
supported Effigy flow to pin exact 0.2.2 and remove only Poodle overrides.
Prove the effective package source before compatibility work. Stop if the
mounted Poodle checkout still controls resolution.

## Completion Protocol

### Before you start

1. This file's metadata activates worker mode. Before broad reads, run
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. Use the current context only when it is a clean, registered, non-`main`
   Underlay Reference worktree supplied by the launcher. Record the actual
   root/branch; do not create another because its generated name differs.
3. If the launcher supplied `main`, a dirty checkout, or an unregistered path,
   stop and report it. Never clean it or guess `/tmp`; manual fallback requires
   the repository's operator-selected `AGENTS_WORKTREE_CONTAINER_DIR`.
4. Run `git fetch origin`; confirm target `HEAD == origin/main == 8885661e`.
   Confirm this handoff exists at the absolute Poodle path and planning base
   `a7293ecd` is an ancestor of Poodle `origin/main`.
5. Read Underlay Reference `AGENTS.md`, reference notes, `g16.013`, and the
   governing Poodle/Underlay refs. Run `effigy tasks` as orientation.

### While you work

- Use Effigy-owned install/validation paths and keep source changes bounded.
- Stop on template/API scope, sibling Poodle resolution, unrelated lock churn,
  container ambiguity that changes evidence, or a Poodle release defect.

### When the assigned runway is complete

1. Inspect all manifests and locks; run `effigy validate`, `effigy qa`,
   `effigy qa:docs`, any missing package-local checks, and `git diff --check`.
2. Push the worker branch and open a reviewable PR against Underlay Reference
   `main`.
3. Link `g16.013`; record registry resolution, retained Underlay sources,
   changed files, compatibility edits, lock review, validation, and unresolved
   items.
4. Report the PR URL and evidence to the operator. Do not merge or edit Poodle
   planning state.

### Review and merge path

The Poodle orchestrator will review independently. Shared GitHub identity may
require a canonical PR comment instead of formal approval. Make only requested
changes. Merge requires separate operator authority.
