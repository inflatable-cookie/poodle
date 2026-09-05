---
title: g15.032 navigation and overlays specimen review worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-21
updated: 2026-08-21
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260821-111649-g15-032-review-composition-navigation-overlays.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, specimens, navigation, overlays]
---

## What This Thread Was Doing

Poodle is finishing the human specimen review between the mechanical catalogue
audit and visual conformance. Four screen-clear children are complete. PR #57
closed `g15.031`, including operator acceptance of the repaired ErrorBoundary
specimen, so the next serial child is ready.

Execute `g15.032`: review exactly ten navigation/overlay pages in Svelte and
React, consume the landed headless GPUI evidence, keep good pages unchanged,
and repair only bounded specimen-teaching defects.

This is one worker handoff. You do not need the originating transcript or a
second prompt.

## Why It Matters

An A screening grade means the mechanical pass found no named defect. It does
not prove that a person can understand normal use, discover the correct
gesture, or see equivalent teaching across active runtimes. Menus, popovers,
hover surfaces, and disclosure are particularly easy to make attractive but
misleading. These ten verdicts must be honest before Poodle starts
primitive-first visual conformance and v0.2.0 certification.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `64ed9edbeb1189b47af539a792cdce34e86335b9`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- **Posture:** `strict-ready`.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning state:** `g15.028`–`g15.031` are complete; `g15.032` is the single
  next ready child. `g15.033` waits behind it.
- **Worker branch:** `t3code/g15-032-review-navigation-overlays`
- **Worker worktree:** use the clean, registered, dedicated non-`main`
  worktree supplied by the launcher, regardless of its generated path or
  branch name.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from ignored `.agents.local.env`.
- **Active spec lane:** g15 human-centred specimen catalogue completion.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready card:**
  `docs/roadmaps/g15/032-review-composition-navigation-overlays.md`
- **Allowed runway:** `g15.032` only.
- **Remaining budget:** one ten-page review/repair batch, one August log, one
  PR, then stop.
- **Dispatch topology:** serial. `g15.033` shares the audit surface and stays
  paused until this child is reviewed, operator-approved, merged, and closed.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/roadmaps/g15/027-screen-clear-human-review.md`,
  `docs/roadmaps/g15/specimen-plan-outline.md`, and
  `docs/roadmaps/g15/specimen-catalogue-audit.md`.
- **Component authority:** the ten matching files under
  `docs/contracts/components/`.
- **Native evidence:** `docs/roadmaps/g15/026-native-specimen-probe.md` and
  `docs/logs/2026-08/20260820-g15-026-native-specimen-probe.md`.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** live Svelte/React previews are allowed; GPUI
  stays headless. Never run `*-windowed`, `test:native-visual`, Jetstream,
  visual-conformance, or release selectors.
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
doctor` also retains the repository's known scan baseline; the card's named
selectors are the execution gate.

## Boundaries

This run owns exactly these ten pages:

- navigation: `Breadcrumbs`, `NavigationMenu`, `Pagination`,
  `PaginationSummary`;
- overlays and disclosure: `Collapsible`, `ContextMenu`, `DebugDialog`,
  `HoverCard`, `Menubar`, `Popover`.

For every page:

- inspect the live Svelte and React route and consume `g15.026`'s headless
  GPUI construction/axis evidence;
- judge the default example, representative variants, useful states, correct
  interaction gesture, captions, narrow behaviour, and cross-runtime teaching;
- record one short human-teaching verdict in the existing audit row;
- leave passing specimen source untouched;
- repair only specimen composition, copy, specimen-owned state, or
  specimen-owned interaction wiring where the page fails.

Writable scope:

- the ten named specimen pages across Svelte, React, and GPUI;
- focused preview/specimen tests needed for changed teaching or interaction;
- those ten audit rows and mechanically affected totals;
- one August `g15.032` batch log;
- root `PAPERCUTS.md` only for newly encountered small execution friction.

Out of scope:

- component implementation, public props, contracts, tokens, shared CSS,
  Rust specs/render composition, or runtime semantics;
- any page outside the exact ten-page list;
- exhaustive prop matrices, a `Conformance` tab, a new shared fixture corpus,
  schema, codegen capability, or generated adapter;
- visual screenshot comparison, native-window execution, Jetstream,
  visual-conformance, release work, or the open motion-learning triage note;
- roadmap/card status, generation front doors, and the dispatch ledger;
- merging the PR or starting `g15.033`.

Stop and report rather than repairing when a page exposes a component,
contract, public API, interaction, or native-runtime defect. Also stop if the
review needs a page outside this list, an executable cross-runtime specimen
extension, or windowed/native visual evidence.

## Important Context

- All ten rows currently read A/A/A `keep`. That is a screen-clear result, not
  a human verdict. Record a verdict for every page, including unchanged pages.
- Review documentation, not coverage matrices. `Examples` should lead with a
  realistic answer to “what is this for?” and stay concise. Sizes and densities
  remain in dedicated panes.
- `ContextMenu` must be exercised with a real context-menu gesture. The audit's
  ordinary-click no-op is expected and must not be reported as dead wiring.
- `HoverCard` must be exercised by hover and keyboard focus where the contract
  requires it. Ordinary clicks are not evidence either way.
- Overlay review includes opening, choosing meaningful actions, Escape/outside
  dismissal, focus return, and repeatability where those behaviours are
  specimen-visible. A broken component semantic is a stop, not specimen scope.
- `NavigationMenu` currently teaches several active-edge/fill treatments.
  Judge whether each section adds real vocabulary; do not preserve or remove
  them merely because the mechanical count passed.
- Pagination's paired web pages and GPUI page are not section-for-section
  identical: GPUI also shows standalone and terminal-page states. Decide
  whether those are useful renderer-owned evidence or redundant drift against
  the same human-teaching rubric.
- A `DebugDialog` null-value example intentionally renders no dialog trigger.
  Judge whether its caption teaches that behaviour rather than treating
  absence alone as breakage.
- Svelte is the reference. React should match section order, copy, fixtures,
  and observable interactions. GPUI may use renderer-owned composition but
  must teach the same important evidence.
- The operator reviews only changed Svelte/React routes live after the PR is
  ready. Do not claim sign-off in the worker log.
- If no page needs repair, a docs-only audit/log PR is valid. Do not create
  churn to justify the run.
- Report after the initial ten-page pass with a compact
  `keep / repair / stop` inventory before broad edits.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, read the
card, parent method, working rules, shared outline, audit rows, component
contracts, and native probe evidence.

Start both web previews on explicit strict ports, inspect all ten routes at
ordinary and narrow widths, and exercise the gesture each component actually
defines. Read the corresponding GPUI specimen source plus the landed headless
evidence. Write the ten-page verdict inventory before editing; that keeps a
screen-clear review from turning into another catalogue rewrite.

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
   `git merge-base --is-ancestor 64ed9edbeb1189b47af539a792cdce34e86335b9 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   `g15.011`, `g15.027`, `g15.032`, the audit, specimen plan, native probe,
   working rules, and the ten component contracts.
6. Use `effigy tasks` to confirm selectors. Do not run any windowed,
   native-visual, Jetstream, visual-conformance, or release path.

### While you work

- Review all ten pages before changing one; report the first-pass inventory.
- Keep good pages byte-untouched and repair only named teaching defects.
- Keep paired web structure, copy, fixtures, and interaction aligned. Change
  GPUI only where its page fails to teach the same important evidence.
- Use right-click for ContextMenu and hover/focus for HoverCard. Exercise
  overlay dismissal with the actual pointer/keyboard events rather than
  click-only substitutes where the contract says otherwise.
- Add focused evidence only when a changed specimen interaction could regress;
  do not build exhaustive specimen tests.
- Append one August `g15.032` batch log with all ten verdicts, changed routes,
  validation, and the pending operator checkpoint.
- Work in coherent batches and stop on any condition listed by the parent or
  child card.

### When the assigned runway is complete

1. Run the required final validation named in Current State. Finish with
   `git diff --check origin/main...HEAD`.
2. Ensure the audit contains a human-teaching verdict for all ten pages and
   the batch log separates unchanged pages from repaired pages.
3. List every changed Svelte and React route for operator live review. State
   that sign-off is pending; do not mark the card complete.
4. Confirm no component, contract, public API, specimen infrastructure,
   workflow, Jetstream, visual-conformance, or release file changed.
5. Push the worker branch and open one reviewable PR against current `main`.
   The handoff's planning base is the pre-handoff commit, not the commit that
   contains this file.
6. Link `g15.011`, `g15.027`, `g15.032`, the audit, batch log, changed routes,
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
- **Closeout refs:** `docs/roadmaps/g15/032-review-composition-navigation-overlays.md`,
  the August batch log, `docs/roadmaps/g15/specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/011-specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/027-screen-clear-human-review.md`,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`, and
  `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns the ten-page review evidence, bounded specimen repairs, and
batch log. The orchestrator owns card/roadmap status, live operator review,
merge, and promotion of `g15.033`. Leave the lane open if a changed page lacks
operator sign-off or a routed stop condition remains.
