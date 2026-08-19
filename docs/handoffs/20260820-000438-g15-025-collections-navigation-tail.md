---
title: g15.025 collections, navigation and long-tail specimen curation worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260820-000438-g15-025-collections-navigation-tail.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, specimens, collections, navigation]
---

## What This Thread Was Doing

Poodle's overloaded `Examples` pages are being returned to a human-facing
teaching role. Five bounded curation families are complete. This worker owns
the final family: ten collection, navigation, overlay, layout, media, and
date-time pages in `g15.025`.

This is one implementation lane and one PR. The card has been remeasured after
its prerequisites and now fixes the exact 2–6-group teaching outline. Do not
extend the run into the remaining catalogue audit or release work.

## Why It Matters

This closes the defect-led half of the specimen curation programme. The pages
must help a reader understand normal use without opening on prop matrices or
nineteen unrelated surfaces. Contract-required cases still need a deliberate
home, and controls presented as interactive must remain live.

Svelte and React need one teaching surface. GPUI must teach the same ordered
intent without claiming web-only mechanics or papering over known component
parity gaps.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `73b285a1a0de7cfd46625c4c7611f1964b47f566`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that
  commit before this handoff was created.
- **Planning checkout:** clean, orchestrator-owned, and unavailable for worker
  edits.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** `g15.025` is ready with a
  remeasured baseline, exact teaching outline, writable scope, validation,
  evidence rules, and stop conditions.
- **Worker branch:** `t3code/curate-collections-navigation-tail`
- **Worker worktree:** launcher-provided clean, dedicated, registered
  non-`main` worktree.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from `.agents.local.env`.
- **Active spec lane:** human-centred specimen catalogue curation.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g15/025-curate-collections-navigation-tail.md`
- **Allowed runway:** `g15.025` only.
- **Remaining card budget:** one card, one batch log, one PR, then stop.
- **Dispatch topology:** one serial lane. `g15.026`, `g15.028`–`g15.033`,
  and release certification are not included.
- **Parallel safety check:** this lane owns only the ten named Svelte, React,
  and GPUI specimen files, focused evidence, and one batch log. Stop if another
  active worker touches them.
- **Canonical refs:** `docs/contracts/001-working-rules.md`,
  `docs/roadmaps/g15/018-overloaded-examples-curation.md`,
  `docs/roadmaps/g15/specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/specimen-plan-outline.md`, and the component contracts
  for Accordion, Dialog, FilterBuilder, ListCard, ListCardCounter,
  MediaPreview, SplitView, Stepper, TimeAgo, and Tree.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** headless only. Never run `*-windowed`,
  native-visual, conformance, Jetstream, or release selectors.
- **Required validation:** focused ten-page outline/caption regression,
  `effigy check:svelte`, `effigy react:build`, `effigy check:gpui`,
  `effigy catalogue:check`, `effigy docs:check`, and
  `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation, orchestrator review, and the
  operator's paired Svelte/React live review.
- **Merge authorisation:** absent. Push the PR and stop for review.

## Boundaries

Keep this run inside these ten pages:

- `Accordion`
- `Dialog`
- `FilterBuilder`
- `ListCard`
- `ListCardCounter`
- `MediaPreview`
- `SplitView`
- `Stepper`
- `TimeAgo`
- `Tree`

The card's writable scope is exact. Out of scope are component
implementations, contracts, public props/types, tokens, shared specimen
infrastructure, catalogue generation/navigation, exhaustive or conformance
tabs, every page owned by another card, Jetstream, and release surfaces.

Do not repair SplitView's known React `divider`/seam or native
both-collapsed parity debt in this lane. Do not turn Tree runtime behaviour
into component work. If the teaching outline exposes a real implementation or
contract defect, stop and report it for an orchestrator decision.

Work only in the selected worker worktree. Never edit, clean, reset, or stash
over the orchestrator's planning checkout. Do not edit
`docs/roadmaps/dispatch.md` or change roadmap/card status. Do not merge the
PR.

## Important Context

- `g15.018` owns the method: 2–6 useful sections for this already-bounded
  tail, realistic normal use first, reader tasks rather than prop
  cross-products, named removals, and preserved contract coverage.
- `g15.025` contains the exact ordered outline for all ten pages. Follow it
  rather than redesigning the family from scratch.
- The post-prerequisite web baseline is: Accordion 2/2, Dialog 9/9,
  FilterBuilder 7/7, ListCard 19/19, ListCardCounter 2/2, MediaPreview 3/3,
  SplitView 7/7, Stepper 8/8, TimeAgo 7/7, and Tree 8/7.
- Accordion, ListCardCounter, and MediaPreview are already concise. Remeasure
  and preserve them unless a concrete defect appears; keeping a good page is a
  valid disposition.
- ListCard is the heavy edit. The target six groups retain every contract
  story while dropping redundant inherited-counter, wrapped-context-menu, and
  separate highlighted/active sections.
- Dialog's first group must use Dialog's own `role="alertdialog"` case, not
  the separate AlertDialog component. Every dialog trigger stays live.
- Tree currently has paired-web drift: Svelte has a flat-tree example React
  lacks, and neither page gives the contract's disabled-node case its own
  honest teaching group. Resolve that inside specimen content only.
- Dedicated size and density panes are already present where supported. Do not
  move axis matrices back into `Examples`.
- GPUI needs a fresh count in the batch log. It teaches the same ordered
  intent, but may represent host-driven interactions as truthful rendered
  postures.
- The operator will inspect every changed Svelte and React page live before
  the card can close. Leave that checkpoint pending in the PR.
- Jetstream remains deferred. Shared Rust component or public surface changes
  are not authorised.

Report after the first coherent half — Dialog, FilterBuilder, and ListCard —
then again when the remaining pages and validation are ready for PR review.
Name changed files, combined/removed stories, checks actually run, remaining
work, and blockers.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, read
`AGENTS.md`, the Effigy skill, `g15.018`, `g15.025`, the audit, specimen
outline, working rules, and all ten component contracts.

Reconfirm the committed counts without changing the page set. Add the focused
regression early enough to pin exact caption order, 2–6-group budgets, and
paired-web equality. Implement Dialog, FilterBuilder, and ListCard as the first
coherent batch. Then take SplitView, Stepper, TimeAgo, and Tree while verifying
that Accordion, ListCardCounter, and MediaPreview remain sound.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before
   any broad read, run only:
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
   `git merge-base --is-ancestor 73b285a1a0de7cfd46625c4c7611f1964b47f566 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   `g15.018`, `g15.025`, the audit, specimen outline, working rules, and
   these contracts completely: `accordion.md`, `dialog.md`,
   `filter-builder.md`, `list-card.md`, `list-card-counter.md`,
   `media-preview.md`, `split-view.md`, `stepper.md`, `time-ago.md`,
   and `tree.md`.
6. Use `effigy tasks` for the supported selector inventory. Do not use a
   windowed, native-visual, conformance, or Jetstream path.

### While you work

- Keep the exact target caption order and story mapping from `g15.025`.
- Keep `Examples` human-centred while retaining every contract-required case
  inside the grouped sections or through named focused evidence.
- Preserve interactive host feedback; do not turn live examples into pictures.
- Keep Svelte and React structure/copy paired and GPUI teaching equivalent.
- Add focused assertions for exact order, budgets, paired equality, and
  representative live behaviour.
- Append evidence to one new August `g15.025` batch log.
- Work in coherent page groups and commit meaningful chunks rather than model
  turns.
- Report through the operator after the first three changed pages and at
  PR-ready state.
- Stop on any condition listed by the card.

### When the assigned runway is complete

1. Run the focused regression, then every required selector named in Current
   State. Finish with `git diff --check origin/main...HEAD`.
2. Ensure the batch log records baseline/final counts, exact caption order,
   named removals/combinations, preserved-page dispositions, contract coverage,
   GPUI recount, changed files, and exact validation outcomes.
3. Start the Svelte and React previews for the operator checkpoint, but leave
   review pending until the operator actually inspects the changed pages.
4. Push the worker branch and open a reviewable PR against current `main`.
5. Link `g15.018`, `g15.025`, the batch log, changed surfaces, validation,
   and pending live review in the PR body.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect the PR independently against the card, parent,
contracts, diff, checks, and batch log. Because the orchestrator and worker may
share a GitHub identity, the verdict may be recorded as a PR comment rather
than a formal approval.

If changes are requested, make only those changes on this branch, push again,
and report back. The operator's paired live review remains part of acceptance.
Merge requires explicit operator authorisation after both review paths pass.

- **Requested changes:** none yet.
- **Closeout refs:** `docs/roadmaps/g15/025-curate-collections-navigation-tail.md`,
  the August batch log, `docs/roadmaps/g15/018-overloaded-examples-curation.md`,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`,
  `docs/roadmaps/g15/release-gap-register.md`, and
  `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns implementation evidence and the batch log. The orchestrator
owns dispatch status, roadmap status, merge, and the next card. If the family
is blocked, leave those surfaces open and report the exact blocker instead of
making the handoff look complete.
