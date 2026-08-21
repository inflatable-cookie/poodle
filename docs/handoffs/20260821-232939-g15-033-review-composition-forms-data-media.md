---
title: g15.033 forms, data, and media specimen review worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-21
updated: 2026-08-21
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260821-232939-g15-033-review-composition-forms-data-media.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, specimens, forms, data, media]
---

## What This Thread Was Doing

Poodle is finishing the human specimen review between the mechanical catalogue
audit and primitive visual conformance. The first five screen-clear children
and every blocker they exposed are complete. `g15.033` is the sixth and final
child.

Execute `g15.033`: review exactly seven forms, data, and media pages in Svelte
and React, consume the landed headless GPUI evidence, keep good pages unchanged,
and repair only bounded specimen-teaching defects.

This is one worker handoff. You do not need the originating transcript or a
second prompt.

## Why It Matters

These seven pages mechanically screened clean, but that is not human judgment.
Forms and collection components must teach a real workflow, not just display a
pile of states; interactive choice and ordering controls must actually work;
and a media thumbnail must explain useful states without becoming an exhaustive
matrix. Closing this child gives all 56 screen-clear pages a real verdict,
completes `g15.011`, and unlocks the first primitive visual-fixture card.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `44a96ad865e6104c43ad06e280e44b1d1f486a98`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- **Posture:** `strict-ready`.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning state:** `g15.028`–`g15.032` and every routed repair are complete;
  `g15.033` is the single final screen-clear child. `g15.046` stays behind the
  resulting `g15.011` closeout.
- **Worker branch:** `t3code/g15-033-review-forms-data-media`
- **Worker worktree:** use the clean, registered, dedicated non-`main`
  worktree supplied by the launcher, regardless of its generated path or
  branch name.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from ignored `.agents.local.env`.
- **Active spec lane:** g15 human-centred specimen catalogue completion.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready card:**
  `docs/roadmaps/g15/033-review-composition-forms-data-media.md`
- **Allowed runway:** `g15.033` only.
- **Remaining budget:** one seven-page review/repair batch, one August log, one
  PR, then stop.
- **Dispatch topology:** final serial review child. The independent `g15.048`
  packaging lane may run elsewhere, but this worker must not touch it.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/roadmaps/g15/011-specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/027-screen-clear-human-review.md`,
  `docs/roadmaps/g15/specimen-plan-outline.md`, and
  `docs/roadmaps/g15/specimen-catalogue-audit.md`.
- **Component authority:** `docs/contracts/components/field-set.md`,
  `validation-summary.md`, `card-radio-group.md`, `list-container.md`,
  `order-by.md`, `selection-summary.md`, and `media-thumbnail.md`.
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

The open motion-learning and Longhorn conformance-lab triage notes are
unrelated and remain open. Do not pull them into this card. `effigy doctor`
also retains the repository's known scan baseline; the named selectors above
are the execution gate.

## Boundaries

This run owns exactly these seven pages:

- forms and validation: `FieldSet`, `ValidationSummary`;
- data and collections: `CardRadioGroup`, `ListContainer`, `OrderBy`,
  `SelectionSummary`;
- media: `MediaThumbnail`.

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

- the seven named specimen pages across Svelte, React, and GPUI;
- focused preview/specimen tests needed for changed teaching or interaction;
- those seven audit rows and mechanically affected totals;
- one August `g15.033` batch log;
- root `PAPERCUTS.md` only for newly encountered small execution friction.

Out of scope:

- component implementation, public props, contracts, tokens, shared CSS,
  Rust specs/render composition, or runtime semantics;
- any page outside the exact seven-page list;
- exhaustive prop matrices, a `Conformance` tab, a new shared fixture corpus,
  schema, codegen capability, or generated adapter;
- visual screenshot comparison, native-window execution, Jetstream,
  visual-conformance, packaging, release work, or the two open triage notes;
- roadmap/card status, generation front doors, and the dispatch ledger;
- merging the PR, closing `g15.011`, or starting `g15.046`.

Stop and report rather than repairing when a page exposes a component,
contract, public API, interaction, or native-runtime defect. Also stop if the
review needs a page outside this list, an executable cross-runtime specimen
extension, or windowed/native visual evidence.

## Important Context

- All seven rows currently read A/A/A `keep`. That is a screen-clear result,
  not a human verdict. Record a verdict for every page, including unchanged
  pages.
- Review documentation, not coverage matrices. `Examples` should lead with a
  realistic answer to “what is this for?” and stay concise. Sizes and densities
  remain in dedicated panes.
- `FieldSet` should teach grouped form meaning, legend/description use, and
  useful validation state without turning into a generic form showcase.
- `ValidationSummary` is valuable only when its errors and any navigation or
  dismissal affordances are understandable and actually wired. A component
  behaviour defect is a stop.
- `CardRadioGroup` must demonstrate real exclusive selection and make disabled
  or descriptive options useful rather than decorative.
- Judge `ListContainer`, `OrderBy`, and `SelectionSummary` as composable data
  primitives. Repeated lists, counts, empty states, or sorting directions earn
  space only when they explain a distinct consumer-facing idea.
- Exercise `OrderBy` through its actual selection/direction gesture and confirm
  repeatability. Do not accept captions as interaction evidence.
- `MediaThumbnail` should teach the normal thumbnail first. Loading, missing,
  fallback, selected, or overlay states belong only when the public component
  truly has them and the difference is useful.
- Svelte is the reference. React should match section order, copy, fixtures,
  and observable interactions. GPUI may use renderer-owned composition but
  must teach the same important evidence.
- The operator reviews only changed Svelte/React routes live after the PR is
  ready. Do not claim sign-off in the worker log.
- If no page needs repair, a docs-only audit/log PR is valid. Do not create
  churn to justify the run.
- Report after the initial seven-page pass with a compact
  `keep / repair / stop` inventory before broad edits.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, read the
card, parent method, working rules, shared outline, audit rows, component
contracts, and native probe evidence.

Start both web previews on explicit strict ports, inspect all seven routes at
ordinary and narrow widths, and exercise the gesture each interactive
component actually defines. Read the corresponding GPUI specimen source plus
the landed headless evidence. Write the seven-page verdict inventory before
editing; that keeps a screen-clear review from turning into another catalogue
rewrite.

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
   `git merge-base --is-ancestor 44a96ad865e6104c43ad06e280e44b1d1f486a98 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   `g15.011`, `g15.027`, `g15.033`, the audit, specimen plan, native probe,
   working rules, and the seven component contracts.
6. Use `effigy tasks` to confirm selectors. Do not run any windowed,
   native-visual, Jetstream, visual-conformance, packaging, or release path.

### While you work

- Review all seven pages before changing one; report the first-pass inventory.
- Keep good pages byte-untouched and repair only named teaching defects.
- Keep paired web structure, copy, fixtures, and interaction aligned. Change
  GPUI only where its page fails to teach the same important evidence.
- Exercise selection, ordering, removal, dismissal, and media state changes
  with the actual pointer/keyboard events defined by each component.
- Add focused evidence only when a changed specimen interaction could regress;
  do not build exhaustive specimen tests.
- Append one August `g15.033` batch log with all seven verdicts, changed routes,
  validation, and the pending operator checkpoint.
- Work in coherent batches and stop on any condition listed by the parent or
  child card.

### When the assigned runway is complete

1. Run the required final validation named in Current State. Finish with
   `git diff --check origin/main...HEAD`.
2. Ensure the audit contains a human-teaching verdict for all seven pages and
   the batch log separates unchanged pages from repaired pages.
3. List every changed Svelte and React route for operator live review. State
   that sign-off is pending; do not mark the card complete.
4. Confirm no component, contract, public API, specimen infrastructure,
   workflow, Jetstream, visual-conformance, packaging, or release file changed.
5. Push the worker branch and open one reviewable PR against current `main`.
   The handoff's planning base is the pre-handoff commit, not the commit that
   contains this file.
6. Link `g15.011`, `g15.027`, `g15.033`, the audit, batch log, changed routes,
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
- **Closeout refs:** `docs/roadmaps/g15/033-review-composition-forms-data-media.md`,
  the August batch log, `docs/roadmaps/g15/specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/011-specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/027-screen-clear-human-review.md`,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`, and
  `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns the seven-page review evidence, bounded specimen repairs, and
batch log. The orchestrator owns card/roadmap status, live operator review,
merge, `g15.011` closeout, and promotion of `g15.046`. Leave the lane open if
a changed page lacks operator sign-off or a routed stop condition remains.
