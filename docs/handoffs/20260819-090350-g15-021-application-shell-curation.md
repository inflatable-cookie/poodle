---
title: g15.021 application-shell specimen curation worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-19
updated: 2026-08-19
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260819-090350-g15-021-application-shell-curation.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15]
---

## What This Thread Was Doing

The orchestrator closed `g15.020`, remeasured the next overloaded-Examples
family on current `main`, and compiled `g15.021` into one executable card.
Earlier caption and axis work has already shortened several application-shell
pages, so this is not a mechanical deletion pass. It curates the three pages
that remain long, converges the web and GPUI teaching structure, and repairs
the known decorative controls in DetailShell and PageHeader.

This is one bounded implementation lane. You should be able to start from this
file without the originating conversation.

## Why It Matters

Poodle's catalogue is product documentation, not a conformance matrix. These
shell pages currently mix useful real-world compositions with fixture IDs,
repeated prop cases, runtime-specific omissions, and controls that appear live
but do nothing. `g15.021` makes them teachable without deleting the regression
stories that protect DockRegion and HistoryCenter.

This is part of the specimen programme blocking the v0.2.0 release baseline.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `055641feebceeef91db0fc4678a01c8f498b04f9`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `055641feebceeef91db0fc4678a01c8f498b04f9` before this handoff was created
- **Planning checkout:** clean at the planning base
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:**
  `docs/roadmaps/g15/021-curate-application-shell.md` is ready, with current
  counts, exact teaching outlines, evidence, writable scope, and stop conditions
- **Worker branch:** `t3code/g15-021-application-shell-curation`
- **Worker worktree:** harness-managed; expected placeholder
  `/Users/tom/.t3/worktrees/poodle/g15-021-application-shell-curation`
- **Worktree creation command:** the launcher should supply the worktree. Only
  if fallback is required, read `.agents.local.env` as data, validate
  `AGENTS_WORKTREE_CONTAINER_DIR`, then run `git worktree add
  <validated-container>/poodle-g15-021-application-shell-curation -b
  t3code/g15-021-application-shell-curation origin/main`
- **Worker worktree policy:** use a clean, dedicated, non-`main` registered
  worktree supplied by the launcher even when its path or branch differs from
  these placeholders. Record the actual values and do not create another
  worktree. If the current context is unusable, use the named worktree when it
  matches; only then use the validated manual fallback. Never use `/tmp`,
  `TMPDIR`, or a guessed path.
- **Active spec lane:** `docs/roadmaps/g15/specimen-plan-outline.md`
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g15/021-curate-application-shell.md`
- **Allowed runway:** `g15.021` only
- **Remaining card budget:** one card, one PR, then stop
- **Dispatch topology:** serial single lane; do not absorb `g15.022`
- **Parallel safety check:** the card owns only seven named specimen pages in
  each active runtime, one focused parity test, one log, and narrowly scoped
  GPUI preview state if required
- **Canonical refs:** `AGENTS.md`, `docs/contracts/001-working-rules.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/roadmaps/g15/018-overloaded-examples-curation.md`,
  `docs/roadmaps/g15/specimen-catalogue-audit.md`, and
  `docs/roadmaps/g15/specimen-plan-outline.md`; the component contracts for
  ActionDiscoveryPanel, DetailSection, DetailSectionGroup, DetailShell,
  DockRegion, HistoryCenter, and PageHeader
- **Model capability profile:** capable coding model, medium reasoning
- **Tool/runtime restrictions:** headless only. Do not run windowed,
  native-visual, conformance, Jetstream, or release selectors.
- **Required validation:** focused `g15.021` regression;
  `effigy test:parity`; `effigy check:svelte`; `effigy react:build`;
  `effigy check:gpui`; `effigy docs:check`;
  `git diff --check origin/main...HEAD`
- **PR base/head:** `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting orchestrator review after the pushed PR
- **Merge authorisation:** absent; the worker must not merge

## Boundaries

Please keep this run inside `g15.021`.

- **In scope:** the seven named specimen pages across Svelte, React, and GPUI;
  their exact focused parity regression; one August batch log; and
  specimen-local GPUI host state only when a retained action needs it.
- **Out of scope:** components, component contracts, public APIs, shared
  specimen shells, catalogue navigation, generated scenes, exhaustive fixture
  tabs, `g15.022`, Jetstream, and release work.
- Preserve the target captions and story mapping in the card. A combined
  section may contain several examples only when they answer one teaching
  question and stay visually distinct.
- DetailSectionGroup is a verified no-op. Do not churn it unless current
  verification finds real drift.
- Do not invent architecture, widen the roadmap, or turn the renderer-neutral
  outline into imported data or a scene language.
- Work only in the selected worker worktree. Never edit the orchestrator's
  `main` checkout or clean/reset somebody else's dirty state.
- Do not merge the PR.

## Important Context

- **Planning lineage:** `g15.011` mechanically screened the full catalogue;
  `g15.015`–`g15.017`, `g15.019`, and `g15.034` repaired caption and axis
  structure; `g15.020` proved this family-child pattern. `g15.018` is the
  non-dispatchable parent for `020`–`025`.
- **Why this card is ready:** all seven pages were remeasured on current main.
  The card records exact current counts, target section order, the stories that
  may be combined, the controls that must become live, and the cases that may
  leave the catalogue only with named focused evidence.
- **Current shape:** ActionDiscoveryPanel, DetailSection, DetailSectionGroup,
  and DetailShell are already within the web budget. DockRegion is 9/8/4,
  HistoryCenter is 9/9/1, and PageHeader is 8/8/8 across
  Svelte/React/GPUI. GPUI differs on ActionDiscoveryPanel and DetailSection.
- **Human-facing rule:** captions describe what a reader learns. HistoryCenter's
  `linear`, `two-forks`, and similar fixture IDs must become the six plain-
  language questions in the card.
- **Interaction rule:** DetailSection, DetailShell, and PageHeader retain
  controls only when those controls produce visible specimen feedback. Do not
  replace dead UI with `console.log`.
- **DockRegion guard:** keep the iconless narrow fallback, g13.040 tab
  pass-throughs, interactive side/bottom collapse, cross-region transfer, and
  both static stack directions.
- **HistoryCenter guard:** map all nine existing fixture claims into the six
  named sections. Its portal behavior means several popovers cannot all be
  forced open at once; keep the examples interactable instead of manufacturing
  a misleading capture state.
- **Native evidence:** GPUI caption evidence remains structural until
  `g15.026`. This worker must converge the authored sections but must not build
  the native page probe.
- **Known baseline:** `effigy doctor` reports the recorded generated-in-src,
  god-file, stale-suppression, comment-ratio, and stale graph findings. Do not
  absorb them.
- **Report after:** first, paired Svelte/React curation plus the focused
  regression; second, GPUI convergence and the final card gate.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, read the
ready card and the seven component contracts. Start by writing the focused
test's exact page/caption table, then curate Svelte and React together. That
makes every deletion and combination visible before GPUI is brought to the
same outline.

Treat each page as documentation. Keep the default useful, keep grouped
examples coherent, and prove the named behavior behind any caption that claims
interaction.

## Completion Protocol

### Before you start

1. This file's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run only: `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root and branch. Do not compare them with the placeholders above or
   create another worktree merely because they differ.
3. Only if the current context is `main`, dirty, unregistered, or unusable
   should you inspect the named worktree. If that also cannot be used, read
   `.agents.local.env` as data and require an absolute, outside-repository
   `AGENTS_WORKTREE_CONTAINER_DIR`. Ask the operator if it is missing. Create a
   unique worktree and branch under that container from `origin/main`. Never
   use `/tmp`, `TMPDIR`, or a guessed path. Never clean, reset, stash over, or
   discard the original checkout. If the launcher itself supplied a dirty or
   `main` worktree, stop and report it rather than hiding the problem.
4. From the selected worktree, run `git fetch origin`; confirm `HEAD` equals
   `origin/main`; confirm `git merge-base --is-ancestor
   055641feebceeef91db0fc4678a01c8f498b04f9 HEAD`; and confirm this handoff
   exists in `HEAD`.
5. Read `AGENTS.md`, `docs/roadmaps/g15/README.md`,
   `docs/roadmaps/g15/021-curate-application-shell.md`, the parent and outline,
   and the seven component contracts.
6. Run `effigy tasks` and `effigy doctor`. Record the known doctor baseline;
   stop only if a new finding changes this card's plan.

### While you work

- Implement one coherent web tranche, then the GPUI convergence tranche.
- Keep Svelte and React section order and copy identical.
- Add behavior assertions where the caption makes a behavioral claim; caption
  equality alone is insufficient.
- Update the August batch log with the old-to-new story map, current counts,
  changed files, actual validation, and unresolved items.
- Report each meaningful chunk through the operator with changed files,
  validation run, remaining work, and blockers.
- Stop if component code, a contract change, shared preview infrastructure, or
  scope outside the seven pages becomes necessary.

### When the assigned runway is complete

1. Run the focused `g15.021` regression, `effigy test:parity`,
   `effigy check:svelte`, `effigy react:build`, `effigy check:gpui`,
   `effigy docs:check`, and `git diff --check origin/main...HEAD`.
2. Do not run windowed, native-visual, conformance, Jetstream, or release
   selectors.
3. Finish the batch log. Leave the live Svelte/React operator review as an
   explicit open PR checkpoint; do not claim it passed.
4. Push the selected worker branch and open a PR against current `main`.
5. The PR body must link this handoff, `g15.021`, `g15.018`, the specimen
   outline, changed surfaces, old-to-new story map, evidence, validation, live
   review checkpoint, and unresolved items.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect the PR independently against the card,
contracts, diff, and checks. Because the worker and orchestrator may share a
GitHub identity, the orchestrator's PR comment is the canonical review record
when formal self-approval is unavailable. Make only requested changes on this
branch and report the updated head through the operator.

Current review state: awaiting review. Requested changes: none yet. The
operator must explicitly authorise any merge after the code/check gate and the
live paired-preview checkpoint are satisfied.

- **Closeout refs:** `docs/roadmaps/g15/021-curate-application-shell.md`, the
  August batch log, `docs/roadmaps/g15/README.md`,
  `docs/roadmaps/generation-index.md`, `docs/roadmaps/README.md`, and
  `docs/roadmaps/dispatch.md`.

### Handoff closeout

Stop after one reviewable PR. Do not mark the card complete or advance the
runway; the orchestrator owns review, merge, roadmap currentness, and the
`g15.022` readiness decision.
