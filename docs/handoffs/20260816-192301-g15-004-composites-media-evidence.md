---
title: g15.004 composites and media evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-16
updated: 2026-08-16
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260816-192301-g15-004-composites-media-evidence.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

Poodle's g15 release runway is replacing broad parity claims with named,
component-local evidence. `g15.002` closed the first 29 paired Svelte/React
gaps. `g15.003` is now covering forms and overlays in another worktree. This
worker takes the disjoint composites and media tranche so both can advance in
parallel without mixing component ownership.

## Why It Matters

Poodle v0.2.0 needs dependable evidence for every public Svelte component,
with React kept paired to the same observable contract cases. The 35
components in this tranche are the largest remaining family and include many
of Underlay's most-used composites.

## Current State

Here is the state this worker inherits:

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `62f76ebd9633281c7c6870651dbb2ad12135a56f`
- **Pushed main verification:** local `HEAD` equalled `origin/main` at the planning base
- **Planning checkout:** clean before this handoff was written
- **Planning artifacts included at the base:** g15 roster, gap register, and turnkey `g15.004` card
- **Worker branch:** `t3code/g15-004-composites-media-evidence`
- **Worker worktree:** `/Users/tom/.t3/worktrees/poodle/g15-004-composites-media-evidence`
- **Worktree creation command:** `git fetch origin && git worktree add /Users/tom/.t3/worktrees/poodle/g15-004-composites-media-evidence -b t3code/g15-004-composites-media-evidence origin/main`
- **Active spec lane:** none; component contracts and working rules are canonical
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready cards, in order:** `docs/roadmaps/g15/004-svelte-focused-evidence-composites-media.md`
- **Allowed runway:** `g15.004` only
- **Remaining card budget:** one card, four meaningful batches
- **Canonical refs:** `docs/roadmaps/g15/release-baseline-roster.md`, `docs/roadmaps/g15/release-gap-register.md`; `docs/contracts/001-working-rules.md`, the 35 named component contracts
- **Model capability profile:** capable coding model, medium reasoning; stop rather than guess on public API ambiguity
- **Tool/runtime restrictions:** never run a `*-windowed`, `test:native-visual`, Jetstream, or `qa:jetstream` selector
- **Required validation:** per-batch focused component tests, then `effigy test:components`, `effigy check:svelte`, `effigy react:build`, `effigy docs:check`, and `git diff --check`
- **PR base/head:** `main` ← `t3code/g15-004-composites-media-evidence`
- **PR URL:** pending
- **Review state:** awaiting implementation and orchestrator review
- **Merge authorisation:** worker must not merge; the operator has retained merge authority

## Boundaries

Please keep this run inside `g15.004`:

- **In scope:** focused Svelte and React evidence for the card's 35 named
  composites/media components; bounded harness fixtures and contract-first
  fixes exactly as the card permits; its batch log and focused-evidence rows.
- **Out of scope:** `g15.003` components, native/GPUI/Jetstream work,
  specimens, new architecture, broad refactors, release mutation, or later
  cards.
- Work only in
  `/Users/tom/.t3/worktrees/poodle/g15-004-composites-media-evidence` on
  `t3code/g15-004-composites-media-evidence`. Do not edit the orchestrator's
  planning checkout or the existing `g15.003` worktree.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** `g15.001` measured the 175-component denominator;
  `g15.002` established the evidence threshold and paired-test precedent;
  `g15.004` is the measured composites/media tranche.
- **Why the card is ready:** the component list, four batches, evidence
  threshold, writable scope, validation, and stop conditions are explicit.
- **Decisions and preferences:** tests must assert load-bearing observable
  contract behaviour, not repeat the anatomy smoke. Svelte is the reference;
  React receives the same contract cases in the same batch.
- **Open tension:** `g15.003` is running concurrently and will edit
  `release-baseline-roster.md` and `release-gap-register.md`. Do not copy old
  summary totals over newer ones. If `003` lands first, rebase onto current
  `origin/main` before final validation and combine its additive totals with
  this tranche. If that merge is unclear, leave the branch clean and ask the
  orchestrator to reconcile it during review.
- **Report after:** each of batches A, B, C, and D from the card
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Read this handoff from the top, then read `AGENTS.md`, the g15 README, the
`g15.004` card, the roster/register, and each component contract before writing
its evidence. Verify the pushed base and worktree first. Start with batch A's
downstream-used data/list composites, then run the batch's narrow tests before
moving on.

## Completion Protocol

### Before you start

1. Confirm the worker worktree and branch match the paths above.
2. Run `git fetch origin`; confirm `git rev-parse HEAD` equals
   `git rev-parse origin/main`.
3. Confirm `git merge-base --is-ancestor 62f76ebd9633281c7c6870651dbb2ad12135a56f HEAD`
   succeeds and this handoff file exists in `HEAD`.
4. Read `AGENTS.md`, the g15 milestone, `g15.004`, the roster/register, and
   the relevant component contracts.
5. Use Effigy for orientation and validation. Record only commands actually
   run.

### While you work

- Execute batches A through D in order. Keep commits aligned with meaningful
  batches, not model turns.
- After each batch, report changed files, validation, remaining batches, new
  risks, and blockers through the operator.
- Stop if a contract is missing or ambiguous, the scope expands, a fix changes
  public behaviour without contract authority, or validation changes the plan.
- Do not invent a shared corpus, component authority, or new architecture.

### When the assigned runway is complete

1. Run `effigy test:components`, `effigy check:svelte`,
   `effigy react:build`, `effigy docs:check`, and `git diff --check`.
2. Update the focused-evidence roster/register rows and one August batch log
   exactly as `g15.004` requires. Do not change roadmap status or front doors.
3. Reconcile current `origin/main` before final validation if `g15.003` has
   landed; preserve both tranches' additive evidence totals.
4. Push `t3code/g15-004-composites-media-evidence` and open a reviewable PR
   against current `main`.
5. Link this handoff, `g15.004`, the changed surfaces, evidence, validation,
   and unresolved items in the PR body.
6. Report the PR URL to the operator. Do not merge.

### Review and merge path

The orchestrator will independently review the PR against the contracts,
card, diff, and checks. Because worker and orchestrator share a GitHub
identity, the orchestrator records its verdict in a PR comment rather than
self-approving. Requested changes are none yet. The operator retains explicit
merge authority.

- **Closeout refs:** `docs/roadmaps/g15/004-svelte-focused-evidence-composites-media.md`,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`,
  `docs/roadmaps/README.md`, `docs/roadmaps/dispatch.md`, the worker's August
  log, roster, and gap register

### Handoff closeout

Leave the card, log, roster, and gap register honest. If the work blocks, record
the blocker and stop. Do not advance `g15.005`; the orchestrator owns the next
ready-card decision after both parallel PRs are reviewed.
