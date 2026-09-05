---
title: g15.015 specimen caption integrity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-18
updated: 2026-08-18
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260818-112051-g15-015-specimen-caption-integrity.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, specimens, svelte, type-gate]
---

## What This Thread Was Doing

Poodle's complete specimen audit found the catalogue's only D-grade defect:
all 52 authored captions on nine Svelte agent-surface pages render blank.
Those pages pass `title` and `description` to a preview helper that accepts only
`label`; Svelte silently drops the unknown props. The preview workspace was
never type-checked in CI, so the defect shipped behind a green gate.

This worker owns g15.015 only. Restore the 52 captions in Svelte, carry the
authored descriptions into the shared Svelte/React preview idiom, and make the
entire Svelte preview workspace pass a required Effigy type-check gate. Start
from this file without a copied transcript or second prompt.

## Why It Matters

Specimen pages are Poodle's human-facing component documentation. Authored
copy that disappears without a failure makes the catalogue actively
misleading, and a new CI selector that begins with hundreds of accepted errors
would be security theatre. This batch closes both the visible defect and the
exact gate hole that hid it.

Keep the repair narrow. The nine pages' examples and wording are already good;
g15.015 restores their presentation. Later cards own specimen idiom convergence
and content curation.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `1719785c7cbbbf12de705a547d19999d5b7ba148`
- **Pushed-main verification:** local `HEAD` and `origin/main` both equalled
  that planning base before this handoff was created
- **Planning checkout:** clean `main` before the planning edits; implementation
  edits are forbidden there
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker branch placeholder:** `t3code/g15-015-specimen-caption-integrity`
- **Worker worktree:** launcher-managed. No manual path is authorised; use the
  clean registered non-`main` worktree supplied by T3 Code
- **Worktree creation command:** none. If the launcher did not supply a usable
  worktree, stop and ask the operator; never guess a path or use `/tmp`
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready card:** `docs/roadmaps/g15/015-specimen-caption-integrity.md`
- **Allowed runway:** g15.015 only
- **Dispatch topology:** serial. g15.016 follows after this caption/gate repair;
  g15.017 may touch the same specimen estate and is not parallel-safe
- **Model capability profile:** capable coding model, medium reasoning; stop for
  orchestrator review on public API, contract, or architecture questions
- **Operator authority:** the operator authorised the in-scope
  `tasks/effigy.tasks.toml` change when advancing this card. Workflows remain
  forbidden
- **Measured preview baseline:** `svelte-check found 428 errors and 13 warnings
  in 25 files`. The 428 errors are 348 generated catalogue, 52 caption props,
  and 28 residual workspace diagnostics
- **Known doctor baseline:** `effigy doctor` reports pre-existing
  generated-in-src, god-file, stale-suppression, graph-staleness, and comment
  ratio findings. Record but do not absorb them
- **Tool/runtime restrictions:** never run a `*-windowed` selector,
  `test:native-visual`, `qa:jetstream`, or any Jetstream selector. This is a
  web-preview batch
- **PR base/head:** `main` <- selected worker branch
- **Review state:** awaiting worker delivery and live operator review
- **Merge authorisation:** none. Push a PR and stop for orchestrator review

## Boundaries

- Fix exactly these Svelte pages: `AgentMessage` (8 captions), `AgentPlan` (4),
  `AgentPlanRecord` (6), `AgentQuestion` (6), `AgentQuestionRecord` (6),
  `AgentSubagent` (6), `ChangedFiles` (7), `ToolCall` (4), and `ToolCallGroup`
  (5). Their files are under `packages/svelte/preview/src/specimens/`.
- Replace the invalid caption prop with the helper's canonical caption prop.
  Add optional `description` rendering to the Svelte and React
  `SpecimenGroup` helpers. Copy the existing Svelte descriptions into the nine
  React counterpart pages so the paired previews teach the same thing.
- Do not rewrite examples, captions, descriptions, fixtures, ordering, or
  interactions. Content curation belongs to g15.018 and its children.
- Close all 428 preview-workspace errors. The 28 outside the caption and
  generated-catalogue classes are presently: 13 in
  `scripts/build-recipe-inventory.ts`; 6 across `contract-role-drift.ts`,
  `contract-spec-drift.ts`, and `contract-value-domain-drift.ts`; 5 in
  `ListContainerSpecimen.svelte`; 2 in `SceneSpecimen.svelte`; and one each in
  `component-registry.ts`, `DialogSpecimen.svelte`, and
  `packages/core/src/licence.ts`.
- Residual fixes must be the smallest type-correct change that preserves
  behaviour. Bun script typing may be fixed through the preview tsconfig.
  Generated catalogue empties may use one honest typing boundary. Do not add
  blanket suppressions or exclude files from the gate.
- Add a named Effigy selector for the preview type-check and compose it into
  `check:svelte`, which already feeds `ci:web`. Do not duplicate the command
  directly in several aggregate selectors.
- Do not change public component APIs, component contracts, dependencies,
  release machinery, `.github/workflows/`, Rust renderers, GPUI, or Jetstream.
  `SpecimenGroup` is preview-local and is not package API.
- Work only in the selected worker worktree. Never edit, clean, reset, stash,
  or remove the orchestrator checkout or another worker's checkout.
- Do not edit `docs/roadmaps/dispatch.md`, card status, or generation status.
- Do not merge the PR.

## Important Context

- The complete finding and exact nine-page counts are in
  `docs/roadmaps/g15/specimen-catalogue-audit.md`, finding 1. The intended page
  vocabulary is in `docs/roadmaps/g15/specimen-plan-outline.md`.
- Current Svelte `SpecimenGroup` renders an `Eyebrow` from `label` and then its
  content. Preserve that structure; render `description` as quiet supporting
  copy with the same meaning and ordering in React.
- React already uses `label` correctly, so the React work is the optional
  description channel and paired copy, not a caption repair.
- The generated catalogue contributes 174 errors from the Svelte artifact and
  174 from the React artifact because empty collection literals infer
  `readonly never[]`. Fix the generator or generated-data boundary so a
  regeneration stays clean; do not hand-edit generated output without its
  authority.
- The residual 28 diagnostics are in the preview workspace because its tsconfig
  includes scripts and follows source aliases into core and React preview
  artifacts. The gate should continue to see that honest workspace.
- The new selector needs a mutation proof. Temporarily put `title=` back on one
  scoped Svelte page, show the selector fails, restore the file, and show it
  passes. Do not commit the mutation.
- Live review is acceptance evidence, not a worker self-approval. Start the
  Svelte and React previews once the complete batch is coherent, give the
  operator the nine routes, and pause for feedback. If review is deferred,
  leave it explicitly open in the PR.
- Use the repo-local Effigy skill for selector routing. Run checks by coherent
  batch, not after every caption.

## Suggested Next Move

Run the worker startup probe before broad reads. Then read `AGENTS.md`, the
repo-local Effigy skill, g15 README, g15.015, the audit finding, and the two
preview helpers. Confirm the 428-error baseline once in the worker worktree and
classify it against the counts above.

Implement in three batches: caption/helper alignment, generated/residual type
repair, then required gate wiring and evidence. Add focused tests for helper
caption/description rendering and paired copy where the existing preview test
shape supports it. After the full type selector and focused checks are green,
perform the fail/pass mutation proof and run the aggregate web gate once.

Then start the live Svelte and React previews and pause for the operator's
review. Apply bounded feedback, write one batch log, push the branch, open a PR,
and stop for orchestrator review.

## Completion Protocol

### Before starting

1. Run one read-only probe before broad reads: `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Accept the launcher-provided context only if it is a registered, clean,
   non-`main` worktree. The actual path and branch may differ from the
   placeholder. If it is main, dirty, unregistered, or otherwise unusable,
   stop; do not create or clean a replacement.
3. Fetch origin. Confirm `HEAD == origin/main`, confirm
   `1719785c7cbbbf12de705a547d19999d5b7ba148` is an ancestor of `HEAD`, and
   confirm this handoff exists in `HEAD`.
4. Read the sources named in Suggested Next Move plus
   `docs/contracts/001-working-rules.md` and
   `docs/architecture/003-component-docs-ia-and-implementation-substrates.md`.
5. Run `effigy tasks`; record `effigy doctor` as a baseline only. Reproduce the
   preview type-check count before editing.

### While working

- Work in the three coherent batches above. Keep the generated authority and
  output together in one commit-sized batch.
- Use `apply_patch` for hand edits. Preserve unrelated worktree state.
- Add one batch log under `docs/logs/2026-08/` with the baseline, fix classes,
  mutation proof, live-review state, validation, and any deviations.
- Record small solvable friction in `PAPERCUTS.md`; do not absorb it.
- Stop on any required public API, contract, dependency, workflow, release,
  native-runtime, or architecture change.

### Validate and hand off

1. Run the focused preview tests for `SpecimenGroup` and the scoped pages.
2. Run the new preview type selector and record zero errors.
3. Perform and restore the `title=` fail/pass mutation proof.
4. Run `effigy check:svelte`, `effigy react:build`,
   `effigy catalogue:check`, `effigy ci:web`, and `effigy docs:check`.
5. Run `git diff --check origin/main...HEAD` and confirm the worktree is clean
   after committing.
6. Complete the live Svelte/React operator checkpoint or name it as an open PR
   acceptance item.
7. Push the worker branch and open a PR against `main`. Report the PR URL,
   pushed SHA, exact selector results, mutation result, review state, and any
   remaining blocker. Do not merge.
