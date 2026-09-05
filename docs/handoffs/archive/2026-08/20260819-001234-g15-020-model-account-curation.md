---
title: g15.020 model-connection and account-lifecycle specimen curation handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-19
updated: 2026-08-19
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260819-001234-g15-020-model-account-curation.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, specimens, curation]
---

## What This Thread Was Doing

Poodle is curating the catalogue back into human-facing documentation after
the conformance experiments overloaded many `Examples` views. This run owns
the model-connection and account-lifecycle family: eight fixed pages, six real
curation targets, and two licence pages that are already good and must not be
churned.

This is one bounded implementation thread. Start from this file without a
copied transcript or second prompt.

## Why It Matters

These pages are among Poodle's most complex composites, and several currently
show 8–13 separate examples before a reader can understand normal use. Poodle
v0.2.0 needs concise teaching pages across Svelte, React, and GPUI without
turning the catalogue into another exhaustive test corpus.

The curation must keep contract-critical stories visible, especially the five
licence usability states, configuration-free connection setup, and
ModelPicker's provider-specific axes. Exhaustive cases belong in focused
tests, not as dozens of catalogue surfaces.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `55f7bbe3555b0ed24d9f85ac76d07e3f6c94d504`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that
  commit before this handoff was created
- **Planning checkout:** clean `main`; implementation edits are forbidden there
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts at the base:** ready `g15.020`, remeasured eight-page
  baseline, exact target outlines, evidence and stop conditions
- **Worker branch placeholder:** `t3code/g15-020-model-account-curation`
- **Worker worktree:** launcher-managed. If a manual fallback is required, use
  `${AGENTS_WORKTREE_CONTAINER_DIR}/poodle-g15-020-model-account-curation`
- **Manual fallback command:** after validating the ignored local-path file,
  `git worktree add -b t3code/g15-020-model-account-curation
  "$AGENTS_WORKTREE_CONTAINER_DIR/poodle-g15-020-model-account-curation"
  origin/main`
- **Active spec lane:** human-centred specimen catalogue completion
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready card:** `docs/roadmaps/g15/020-curate-model-connection-licence.md`
- **Allowed runway:** `g15.020` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial for this worker. Do not absorb `g15.021` or the
  native probe
- **Parallel safety:** the eight owned pages are exclusive to this card, but
  parity evidence is shared; stop if another worker is changing the same files
- **Canonical refs:** `docs/contracts/001-working-rules.md`,
  `docs/roadmaps/g15/018-overloaded-examples-curation.md`,
  `docs/roadmaps/g15/specimen-plan-outline.md`,
  `docs/roadmaps/g15/specimen-catalogue-audit.md`, and the eight component
  contracts named by the card
- **Model capability profile:** capable coding model, medium reasoning
- **Known doctor baseline:** generated-in-src, god-file, stale-suppression,
  stale-graph, and comment-ratio findings. Record them; do not absorb them
- **Required validation:** the exact headless board in `g15.020`
- **PR base/head:** `main` <- selected worker branch
- **PR URL:** pending
- **Review state:** awaiting implementation and later live operator review
- **Merge authorisation:** none. Push one PR and stop

## Boundaries

- **In scope:** the eight pages named by `g15.020` across Svelte, React, and
  GPUI; exact caption/budget evidence; one August batch log.
- **Out of scope:** component behavior, props, contracts, shared specimen
  shells, catalogue navigation, generated scene infrastructure, pages owned by
  another curation child, the native page probe, visual conformance, and
  release work.
- LicenceActivation and LicenceSeats are verified no-op pages at five and six
  sections. Do not edit them unless the recheck finds actual cross-runtime
  drift; record the verification in the log.
- Use the card's target teaching outlines. Several instances may share one
  surface only when they answer the same question and remain visually distinct.
  Do not replace many captions with one unstructured slab.
- Keep Svelte and React captions/copy verbatim and in the same order. GPUI may
  shorten copy but must teach the same ordered sections.
- Do not invent architecture, change a public API, weaken focused component
  evidence, or silently drop the only proof of a contracted behavior.
- Work only in the selected clean worker worktree. Never clean, reset, or
  discard another checkout and never edit the orchestrator's `main` checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** `g15.011` measured the whole catalogue, `g15.018`
  split 53 overloaded pages into six family children, and `g15.034` made axis
  tabs truthful. This card is the first family curation child.
- **Current measurement:** LicenceActivation 5/5/5 and LicenceSeats 6/6/6 are
  already bounded. The other six pages carry 8–13 web captions; GPUI carries
  8–12 and disagrees with the web set on four pages.
- **Operator preference:** catalogue examples should show what is available
  and how to use it. They must not become exhaustive variant matrices. Each
  example surface needs a clear boundary; dense unrelated piles are not a
  valid reduction.
- **Contract coverage:** LicenceStatus retains one surface per usability state.
  ModelConnectionSetup proves that a route needing no credentials skips the
  configuration step. ModelPicker consolidates its required stories across a
  few representative fixtures and focused tests rather than deleting them.
- **Native boundary:** GPUI is structurally checked until `g15.026` supplies a
  headless page probe. Do not build that probe here.
- **Evidence expectation:** add an exact eight-page regression for ordered
  captions and the 3–6 budget. Record every removed caption and where its
  behavior remains covered.
- **Report after:** (1) paired web curation plus exact caption evidence, then
  (2) GPUI convergence, batch log, and final headless board.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Read this handoff first. Before broad repository reads, run the quick worktree
preflight below. If the launcher already placed you in a clean, registered,
dedicated non-`main` worktree, use it regardless of its generated path or
branch name and do not create another one.

Then read `AGENTS.md`, the repo-local Effigy skill, `g15.020`, its parent, the
specimen outline, and the eight component contracts. Recount all three runtime
pages at the worker base. Start with LicenceStatus and one model page in both
web runtimes, establish the curation idiom and exact regression, then apply the
same bounded method to the remaining pages. Do not touch the two no-op pages
just to make the diff look symmetrical.

## Completion Protocol

### Before you start

1. This file's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before
   broad reads run only: `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and branch
   is not `main`, accept it as the launcher-provided worktree. Record the actual
   root/branch. Do not compare them with the placeholders or create another
   worktree merely because they differ.
3. If the launcher supplied a dirty or `main` worktree, stop and report it. Do
   not silently create a second worktree behind the launcher. Only when the
   current context is otherwise not launcher-owned may you inspect the named
   fallback, then read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique worktree beneath it.
   Ask the operator if the key is absent. Never use `/tmp`, `TMPDIR`, or a
   guessed repository-adjacent path.
4. From the selected worktree fetch `origin`, confirm `HEAD == origin/main`,
   confirm `git merge-base --is-ancestor
   55f7bbe3555b0ed24d9f85ac76d07e3f6c94d504 HEAD`, and confirm this handoff
   exists in `HEAD`.
5. Read the ready card and canonical refs named above. Run `effigy tasks` and
   `effigy doctor`; treat the recorded doctor findings as baseline, not scope.
6. Check for an overlapping worker on the eight specimens or shared parity
   test. Stop if one exists.

### While you work

- Execute only `g15.020`. Keep commits aligned with the web/evidence and
  GPUI/log chunks rather than arbitrary model turns.
- Use `apply_patch` for edits. Keep component implementations and contracts
  untouched.
- After each chunk, report changed files, exact validation run, remaining
  work, risks, and blockers through the operator.
- Stop on missing coverage, unclear consolidation, a need for component or
  contract changes, runtime-specific section order, or scope growth.
- Record small solvable execution friction in `PAPERCUTS.md`; do not absorb it.

### When the assigned runway is complete

1. Run the card's focused curation regression, then `effigy test:parity`,
   `effigy check:svelte`, `effigy react:build`, `effigy check:gpui`,
   `effigy docs:check`, and `git diff --check origin/main...HEAD`.
2. Never run windowed, native-visual, conformance, Jetstream, or release
   selectors.
3. Write `docs/logs/2026-08/20260819-g15-020-model-account-curation.md` with
   before/after counts, final ordered captions, every removal and coverage
   disposition, commands/results, the two no-op verifications, and unresolved
   findings. Do not change roadmap or dispatch status.
4. Start the paired Svelte and React previews on strict non-conflicting ports.
   Give the operator the six changed routes: `licence-status`,
   `model-catalogue-editor`, `model-connection-card`,
   `model-connection-picker`, `model-connection-setup`, and `model-picker`.
   Live operator acceptance remains open until the orchestrator records it.
5. Commit meaningful batches, push the selected branch, and open one reviewable
   PR against current `main`. Link the card, parent, outline, changed pages,
   exact evidence, validation, batch log, preview routes, and unresolved items.
6. Return the PR URL, head SHA, exact evidence, preview URLs/routes, and any
   deviation. Do not merge.

### Review and merge path

The orchestrator will independently inspect the PR metadata, commits, diff,
checks, final caption sets, coverage map, and live operator checkpoint. Because
worker and orchestrator may share a GitHub identity, the canonical verdict may
be a PR comment rather than formal self-approval.

If changes are requested, change only this branch and report back through the
operator. Merge requires explicit operator authorisation after the gate passes.

- **Closeout refs:** `g15.020`, its August batch log, `g15`/root roadmap
  currentness, generation index, specimen audit, dispatch ledger, and the next
  readiness decision for `g15.021`.

### Handoff closeout

Leave the worker branch and PR honest. The orchestrator—not this worker—marks
the card complete, updates the dispatch ledger/front doors, and selects the
next lane after merge. If blocked, record the blocker and stop.
