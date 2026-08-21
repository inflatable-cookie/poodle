---
title: g15.048 packed roster reachability worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-21
updated: 2026-08-21
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260821-234034-g15-048-packed-roster-reachability.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, packaging, svelte, react]
---

## What This Thread Was Doing

Poodle is closing the evidence needed for v0.2.0. The full Svelte and React
component rosters now exist in source with focused evidence, but the current
packed-install proof mounts only a small sample. `g15.048` must prove that all
175 public component names in each web runtime remain reachable after packing
and clean installation.

Execute `g15.048` as one bounded packaging-evidence lane. Keep the existing
representative mount tests for runtime machinery, add an exact full-roster
root-import proof, inspect the actual tarball contents and public subpaths, and
update the frozen roster's pack-install evidence mechanically.

This is one worker handoff. You do not need the originating transcript or a
second prompt.

## Why It Matters

Workspace imports can hide missing files, undeclared dependencies, broken
exports, and source-only paths. Longhorn and the other Poodle consumers need a
release artifact, not a source checkout that happens to work. This card gives
the release candidate an exact 175-name package boundary without pretending
that import success is component behaviour evidence.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `371e385e8a22d4bb877094890d1545c6029bc281`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- **Posture:** `strict-ready`.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning state:** late g15 web public-surface migrations are complete and
  `g15.048` is ready. `g15.050` remains behind accepted packed proof and the
  other named release prerequisites.
- **Worker branch:** `t3code/g15-048-packed-roster-reachability`
- **Worker worktree:** use the clean, registered, dedicated non-`main`
  worktree supplied by the launcher, regardless of its generated path or
  branch name.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from ignored `.agents.local.env`.
- **Active spec lane:** g15 release-package evidence.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready card:** `docs/roadmaps/g15/048-packed-roster-reachability.md`
- **Allowed runway:** `g15.048` only.
- **Remaining budget:** one packed-package proof batch, one August log, one PR,
  then stop.
- **Dispatch topology:** parallel with `g15.033`.
- **Parallel safety check:** `g15.033` owns seven specimen pages,
  `specimen-catalogue-audit.md`, and its own log. This lane owns
  `test/package-install/`, the release roster's pack-install column, package
  metadata only for a proved packed-boundary defect, and its own log. Do not
  edit specimen source or the specimen audit. `PAPERCUTS.md` is append-only.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  `docs/specs/022-packaging-versioning-and-release-channel-rules.md`,
  `docs/roadmaps/g15/release-baseline-roster.md`,
  `docs/roadmaps/g15/release-gap-register.md`, and
  `docs/roadmaps/g15/013-v020-release-certification.md`.
- **Existing proof:** `test/package-install/web-preview.ts` and its
  `test/package-install/fixture/` consumer tests.
- **Package roots:** `packages/core/package.json`,
  `packages/svelte/components/package.json`, and
  `packages/react/components/package.json`; component inventories come from
  the two package-root `src/index.ts` files and the frozen roster.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** headless package and web checks only. Never
  run release mutations, `*-windowed`, native-visual, Jetstream, or GPUI
  preview selectors. Do not edit `.github/workflows/`.
- **Required validation:** `effigy test:web-pack-install`, relevant
  Svelte/React package checks selected through Effigy, `effigy docs:check`, and
  `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation and orchestrator review.
- **Merge authorisation:** absent. Push the PR and stop for review.

The open motion-learning and Longhorn conformance-lab triage notes are
unrelated and remain open. Do not pull them into this card.

## Boundaries

In scope:

- pack core, Svelte, and React from the repository under test;
- install those tarballs into a clean consumer with no workspace aliases,
  sibling source imports, or undeclared dependencies;
- derive or check an exact 175-name component inventory for Svelte and React;
- import every inventory name from its installed package root and fail with the
  exact missing or extra name;
- retain a small representative mount set for styles/tokens, Svelte snippets,
  React render props, overlays, providers, composite state, and late g15 public
  APIs;
- inspect tarball contents for declared entry points, types, styles, generated
  tokens/icons, licences, READMEs, and manifests;
- prove the required core public subpaths and generated assets resolve from the
  installed tarball;
- update the release roster's pack-install summary and per-component column
  from the exact proof;
- add one August `g15.048` batch log.

Writable scope:

- `test/package-install/`;
- one generated or checkable roster import inventory under that test surface;
- `docs/roadmaps/g15/release-baseline-roster.md`;
- one August `g15.048` batch log;
- package metadata only where needed to fix a real defect reproduced from the
  clean tarball;
- root `PAPERCUTS.md` only for newly encountered small execution friction.

Out of scope:

- component implementation, component contracts, specimens, specimen audit,
  tokens, shared CSS, Rust/native runtimes, or application consumers;
- 175 fake prop fixtures, 175 mounts, or any claim that imports prove
  component behaviour;
- package version bumps, changelog/release-candidate work, publication, tags,
  release preparation/execution, or workflow edits;
- `g15.033`, `g15.046`, `g15.049`, `g15.050`, or certification closeout;
- merging the PR.

Stop and report when the frozen roster and either package root disagree about
the 175-name denominator, a public component is reachable only through source
or an undeclared dependency, or a fix would change public API rather than
repair package metadata. Do not silently redefine the roster.

## Important Context

- The release denominator is exactly 175 public Svelte component exports. The
  React mirror also currently has 175 public components. Helpers and public
  types outside the component list are checked as entry points, not counted as
  components.
- `test/package-install/web-preview.ts` already creates isolated tarballs and a
  clean consumer, rejects sibling-source/workspace resolution, and runs a small
  mounted proof. Extend that seam rather than creating a second packaging
  harness.
- The current roster column says “mounted proof” and shows 9 Svelte cases. This
  card changes the meaning to exact packed root-import reachability while
  preserving the representative mount count separately. Do not call 175
  components mounted when they were only imported.
- Prefer one mechanically derived/checkable inventory over a second hand-kept
  list. It must expose exact missing and extra names and remain tied to the
  frozen denominator.
- Inventory evaluation must occur against the installed tarballs in the clean
  consumer. A source-tree comparison alone does not satisfy the card.
- Static root imports are enough for roster reachability if the harness also
  checks the exact export-name set. Avoid rendering components that require
  props merely to prove the symbol exists.
- Tarball-content checks should use actual packed archives or their installed
  roots, not the source package directories.
- Keep the existing representative machinery tests meaningful. Include the
  late Popover/Button trigger boundary and other already-present late-g15
  package fixtures where relevant, but do not expand the sample mechanically.
- If package metadata is wrong, write a focused failing regression first, make
  the smallest metadata fix, and document package/public-entry-point impact
  under spec 022's release-note rule.
- Report after the initial inventory/tarball design and first red/green run,
  before broad package metadata changes.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, inspect the
existing pack-install runner and fixtures, both package-root indexes and
manifests, the core exports map, the frozen roster's counting method, and the
packaging spec.

Design the smallest exact proof first: one inventory authority/check, one
installed-root export comparison per framework, tarball-content assertions,
then the existing representative mounts. Run it red/green before updating the
roster or touching package metadata.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before any
   broad read, run only:
   - `git rev-parse --show-toplevel`
   - `git branch --show-current`
   - `git status --porcelain`
   - `git worktree list --porcelain`
2. If the current root is a registered, clean, non-`main` worktree, accept it
   as the launcher-provided worktree regardless of generated path or branch
   name. Record the actual values and do not create another worktree.
3. If the launcher supplied a dirty, `main`, or unregistered context, stop and
   report it. Do not clean or reset it. A manual fallback is allowed only after
   reading `.agents.local.env`, finding a valid
   `AGENTS_WORKTREE_CONTAINER_DIR`, and creating a unique worktree there from
   `origin/main`; ask the operator if the key is absent. Never use `/tmp`,
   `TMPDIR`, or a guessed path.
4. From the accepted worktree, run `git fetch origin`, confirm `HEAD` equals
   current `origin/main`, confirm
   `git merge-base --is-ancestor 371e385e8a22d4bb877094890d1545c6029bc281 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   `g15.048`, packaging spec 022, release roster/gap register/certification
   card, existing pack-install runner/fixtures, and the three package manifests.
6. Use `effigy tasks` to confirm selectors. Do not run release mutations,
   windowed/native-visual, Jetstream, or GPUI preview paths.

### While you work

- Extend the existing clean-consumer pack-install seam; do not build another
  harness beside it.
- Keep the exact inventory mechanically tied to package-root authority and the
  frozen 175-name denominator.
- Distinguish “imported” from “mounted” everywhere in code, evidence, and docs.
- Fail with exact missing/extra component names and exact missing tarball files
  or subpaths.
- Keep representative runtime mounts small and explain each retained category.
- Prove any metadata defect from a packed tarball before changing the manifest.
- Append one August `g15.048` batch log with denominator, inventory source,
  tarball checks, representative mount set, validation, and any defect/fix.
- Stop on every condition named by the card or this handoff.

### When the assigned runway is complete

1. Run the required final validation named in Current State. Finish with
   `git diff --check origin/main...HEAD`.
2. Confirm the clean installed roots expose exactly 175 Svelte and 175 React
   component names, and that the batch log separates import reachability from
   representative runtime mounts.
3. Confirm core public subpaths and every required tarball file were checked
   from packed/installed artifacts.
4. Recount the roster mechanically and ensure no `missing` pack-install cells
   remain for either web component roster.
5. Confirm no component, contract, specimen, workflow, native, release-version,
   tag, or publication file changed.
6. Push the worker branch and open one reviewable PR against current `main`.
   The handoff's planning base is the pre-handoff commit, not the commit that
   contains this file.
7. Link `g15.048`, spec 022, the roster/gap register, changed test surfaces,
   batch log, package-impact statement, and validation in the PR body.
8. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect the PR independently, reproduce the clean packed
proof, verify exact inventory accounting, and record the verdict on the PR.

If changes are requested, make only those changes on this branch, push again,
and report back. Merge requires explicit operator authorisation after review
and checks.

- **Requested changes:** none yet.
- **Closeout refs:** `docs/roadmaps/g15/048-packed-roster-reachability.md`,
  the August batch log, `docs/roadmaps/g15/release-baseline-roster.md`,
  `docs/roadmaps/g15/release-gap-register.md`, `docs/roadmaps/g15/README.md`,
  `docs/roadmaps/generation-index.md`, and `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns the packed proof, exact inventory evidence, bounded package
metadata fixes, roster pack-install update, and batch log. The orchestrator owns
card/roadmap status, review, merge, and promotion of the next release lane.
Leave the card open on a denominator disagreement or unresolved packed-boundary
defect.
