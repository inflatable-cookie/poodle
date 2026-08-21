---
title: g15.031 foundation content and status specimen review worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-21
updated: 2026-08-21
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260821-100723-g15-031-review-foundation-content-status.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, specimens, content, status]
---

## What This Thread Was Doing

Poodle is finishing the human specimen review that sits between the mechanical
catalogue audit and visual conformance. The first three screen-clear children
are complete, and PR #56 closed the native ResizeHandle blocker returned by
the last one.

Execute `g15.031`: review exactly nine foundation content/status pages in
Svelte and React, consume the existing headless GPUI evidence, keep good pages
unchanged, and repair only bounded specimen-teaching defects.

This is one worker handoff. You do not need the originating transcript or a
second prompt.

## Why It Matters

An A screening grade means no mechanical defect was named. It does not prove
that a person can understand normal use, see useful states, or find equivalent
teaching across the active runtimes. These nine verdicts are part of the full
175-component Svelte release denominator and must be honest before Poodle
starts primitive-first visual conformance and v0.2.0 certification.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `055d0f0c67deea617c61e13af87ac44c7d589e8f`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- **Posture:** `strict-ready`.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning state:** `g15.028`–`g15.030` and routed `g15.040` are complete;
  `g15.031` is the single next ready child. `g15.032` and `g15.033` wait behind
  it.
- **Worker branch:** `t3code/g15-031-review-foundation-content-status`
- **Worker worktree:** use the clean, registered, dedicated non-`main`
  worktree supplied by the launcher, regardless of its generated path or
  branch name.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from ignored `.agents.local.env`.
- **Active spec lane:** g15 human-centred specimen catalogue completion.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready card:**
  `docs/roadmaps/g15/031-review-foundation-content-status.md`
- **Allowed runway:** `g15.031` only.
- **Remaining budget:** one nine-page review/repair batch, one August log, one
  PR, then stop.
- **Dispatch topology:** serial. The remaining children share
  `specimen-catalogue-audit.md` and require operator live review.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/roadmaps/g15/027-screen-clear-human-review.md`,
  `docs/roadmaps/g15/specimen-plan-outline.md`, and
  `docs/roadmaps/g15/specimen-catalogue-audit.md`.
- **Native evidence:** `docs/roadmaps/g15/026-native-specimen-probe.md` and
  `docs/logs/2026-08/20260820-g15-026-native-specimen-probe.md`.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** live Svelte/React previews are allowed; GPUI
  stays headless. Never run `*-windowed`, `test:native-visual`, Jetstream, or
  release selectors.
- **Required validation:** focused preview/component evidence for changed
  pages, `effigy catalogue:check`, `effigy check:svelte`,
  `effigy react:build`, `effigy docs:check`, and
  `git diff --check origin/main...HEAD`. If GPUI specimen code changes, also
  run `effigy check:gpui` and `effigy regressions:native`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation, orchestrator review, and operator
  live sign-off on every changed Svelte and React route.
- **Merge authorisation:** absent. Push the PR and stop for review.

The open `docs/triage/20260820-205249-transitions-dev-motion-learning.md`
note is unrelated and remains open. Do not pull it into this card. `effigy
doctor` also retains the repository's known generated-in-source and structural
baseline; the card's named selectors are the execution gate.

## Boundaries

This run owns exactly these nine pages:

- content and identity: `Code`, `EmbedPreview`, `IconProvider`, `Pill`;
- status and progress: `ErrorBoundary`, `PageLoading`, `Progress`, `Spinner`,
  `StateTile`.

For every page:

- inspect the live Svelte and React route and consume `g15.026`'s headless
  GPUI construction/axis evidence;
- judge the default example, representative variants, useful states,
  interaction, captions, narrow behaviour, and cross-runtime teaching;
- record one short human-teaching verdict in the existing audit row;
- leave passing specimen source untouched;
- repair only specimen composition, copy, specimen-owned state, or
  specimen-owned interaction wiring where the page fails.

Writable scope:

- the nine named specimen pages across Svelte, React, and GPUI;
- focused preview/specimen tests needed for changed teaching or interaction;
- those nine audit rows and mechanically affected totals;
- one August `g15.031` batch log;
- root `PAPERCUTS.md` only for newly encountered small execution friction.

Pill and Spinner use retained generated specimen outputs. If either needs a
repair, edit its authored entry in
`packages/codegen/fixtures/specimens-model.json`, regenerate with the supported
`effigy ir:build` path, and keep the diff limited to the owned scenes and
derived outputs. Never edit `src/generated/**` by hand. Do not change the
schema, code generator, scene vocabulary, or use this as a reason to expand the
rejected cross-runtime specimen mechanism.

Out of scope:

- component implementation, public props, contracts, tokens, shared CSS,
  Rust specs/render composition, or runtime semantics;
- any page outside the exact nine-page list;
- exhaustive prop matrices, a `Conformance` tab, a new shared fixture corpus,
  schema, codegen capability, or generated adapter;
- visual screenshot comparison, native-window execution, Jetstream, release
  work, or the open motion-learning triage note;
- roadmap/card status, generation front doors, and the dispatch ledger;
- merging the PR or starting `g15.032`.

Stop and report rather than repairing when a page exposes a component,
contract, public API, interaction, or native-runtime defect. Also stop if the
review needs a page outside this list, an executable cross-runtime specimen
extension, or windowed/native visual evidence.

## Important Context

- All nine rows currently read A/A/A `keep`. That is a screen-clear result,
  not a human verdict. Record a verdict for every page, including unchanged
  pages.
- Review documentation, not coverage matrices. `Examples` should lead with a
  realistic answer to “what is this for?” and stay around three to six useful
  sections where the component warrants them. Sizes and densities remain in
  dedicated panes.
- Code's copy action writes to the clipboard and may not produce a DOM change;
  prove the action is genuinely wired rather than treating unchanged markup as
  failure.
- ErrorBoundary's current “Throw again” action can be invoked while the
  boundary is already in its error state. Judge whether the page teaches
  recovery and failure clearly; do not infer dead wiring from unchanged error
  markup.
- Status pages should explain the difference between progress, indeterminate
  waiting, full-page loading, and persistent state without repeating the same
  spinner in several long sections.
- IconProvider should teach the provider boundary rather than become an icon
  catalogue. Pill should show its important appearance/tone vocabulary without
  reconstructing every size/density combination in `Examples`.
- Svelte is the reference. React should match section order, copy, and fixture
  meaning. GPUI may use renderer-owned composition but must teach the same
  important evidence.
- The operator reviews only changed Svelte/React routes live after the PR is
  ready. Do not claim sign-off in the worker log.
- If no page needs repair, a docs-only audit/log PR is valid. Do not create
  churn to justify the run.
- Report after the initial nine-page pass with a compact
  `keep / repair / stop` inventory before broad edits.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, read the
card, parent method, working rules, shared outline, audit rows, and native probe
evidence.

Start both web previews on explicit strict ports, inspect the nine routes at
ordinary and narrow widths, and exercise meaningful controls. Read the
corresponding GPUI specimen source plus the landed headless evidence. Write the
nine-page verdict inventory before editing; that prevents a screen-clear review
from turning into another catalogue rewrite.

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
   `git merge-base --is-ancestor 055d0f0c67deea617c61e13af87ac44c7d589e8f HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   `g15.011`, `g15.027`, `g15.031`, the audit, specimen plan, native probe,
   and working rules.
6. Use `effigy tasks` to confirm selectors. Do not run any windowed,
   native-visual, Jetstream, or release path.

### While you work

- Review all nine pages before changing one; report the first-pass inventory.
- Keep good pages byte-untouched and repair only named teaching defects.
- Keep paired web structure and copy aligned. Change GPUI only where its page
  fails to teach the same important evidence.
- Add focused evidence only when a changed specimen interaction could regress;
  do not build exhaustive specimen tests.
- Append one August `g15.031` batch log with all nine verdicts, changed routes,
  validation, and the pending operator checkpoint.
- Work in coherent batches and stop on any condition listed by the parent or
  child card.

### When the assigned runway is complete

1. Run the required final validation named in Current State. Finish with
   `git diff --check origin/main...HEAD`.
2. Ensure the audit contains a human-teaching verdict for all nine pages and
   the batch log separates unchanged pages from repaired pages.
3. List every changed Svelte and React route for operator live review. State
   that sign-off is pending; do not mark the card complete.
4. Confirm no component, contract, public API, specimen infrastructure,
   workflow, Jetstream, or release file changed. If Pill/Spinner changed,
   distinguish the authored fixture from regenerated outputs and prove
   `effigy ir:check` passes.
5. Push the worker branch and open one reviewable PR against current `main`.
   The handoff's planning base is the pre-handoff commit, not the commit that
   contains this file.
6. Link `g15.011`, `g15.027`, `g15.031`, the audit, batch log, changed routes,
   and validation in the PR body.
7. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect the PR independently, run the required checks,
and open every changed Svelte and React route for operator review. Because the
orchestrator and worker may share a GitHub identity, the verdict may be a PR
comment rather than formal approval.

If changes are requested, make only those changes on this branch, push again,
and report back. Merge requires explicit operator authorisation after code
review, checks, and live operator sign-off.

- **Requested changes:** none yet.
- **Closeout refs:** `docs/roadmaps/g15/031-review-foundation-content-status.md`,
  the August batch log, `docs/roadmaps/g15/specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/011-specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/027-screen-clear-human-review.md`,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`, and
  `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns the nine-page review evidence, bounded specimen repairs, and
batch log. The orchestrator owns card/roadmap status, live operator review,
merge, and promotion of `g15.032`. Leave the lane open if a changed page lacks
operator sign-off or a routed stop condition remains.
