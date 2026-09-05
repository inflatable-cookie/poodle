---
title: g15.029 screen-clear foundation date and time worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260820-193238-g15-029-review-foundation-date-time.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, specimens, web, gpui]
---

## What This Thread Was Doing

Poodle is completing the human review of 56 catalogue pages that passed the
mechanical specimen screening. `g15.028` reviewed the first family and its
SegmentedControl stop condition is now closed. This worker owns the next exact
family: seven foundation date and time pages.

Review all seven before editing. Keep a good page unchanged. Repair only
bounded specimen defects that stop the page teaching normal use clearly across
Svelte, React, and GPUI.

## Why It Matters

These pages passed structural checks, but that does not prove they are useful
documentation. Date and time components are especially easy to turn into
fixture grids or ambiguous state demonstrations. This lane applies human
judgment without reopening component semantics or building another conformance
corpus. It is part of the final specimen work before Poodle's primitive visual
conformance lane and v0.2.0 certification.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `0ba51ca94e1eb02428d8be6bf4b3789d12e42cdd`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that
  commit before this handoff was created.
- **Planning checkout:** clean, orchestrator-owned, and unavailable for worker
  edits.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** PR #52 and `g15.038` are closed;
  `g15.029` is ready and the generation front doors name it as the only current
  task.
- **Worker branch:** `t3code/g15-029-review-foundation-date-time`
- **Worker worktree:** launcher-provided clean, dedicated, registered
  non-`main` worktree.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from `.agents.local.env`.
- **Active spec lane:** g15 human-centred specimen catalogue completion.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g15/029-review-foundation-date-time.md`
- **Allowed runway:** `g15.029` only.
- **Remaining card budget:** one seven-page review, one batch log, one PR, then
  stop.
- **Dispatch topology:** serial. `g15.030`–`g15.033` follow only after this
  child is reviewed and merged.
- **Parallel safety check:** the children edit the same catalogue audit and
  require operator live review. Do not start or absorb another child.
- **Canonical refs:** `docs/contracts/001-working-rules.md`,
  `docs/roadmaps/g15/011-specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/027-screen-clear-human-review.md`,
  `docs/roadmaps/g15/specimen-catalogue-audit.md`, and
  `docs/roadmaps/g15/specimen-plan-outline.md`.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** web previews are allowed; GPUI stays headless.
  Never run `*-windowed`, `test:native-visual`, Jetstream, or release selectors.
- **Required validation:** focused preview/component tests for changed pages,
  `effigy check:svelte-preview`, `effigy react:build`, relevant headless GPUI
  checks when native specimens change, `effigy probe:gpui-specimens`,
  `effigy catalogue:check`, `effigy docs:check`, and
  `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation, orchestrator review, and operator
  live sign-off on every changed Svelte and React page.
- **Merge authorisation:** absent. Push the PR and stop for review.

## Boundaries

This run owns exactly these seven pages:

- `Calendar`
- `DatePicker`
- `DateRangePicker`
- `DateTimePicker`
- `DateTimeRangePicker`
- `DateTimeZonePicker`
- `DurationInput`

For each page:

- inspect the live Svelte and React examples and consume `g15.026`'s headless
  GPUI construction/axis evidence;
- judge the first example, representative variants, interaction usefulness,
  captions, narrow behaviour, and cross-runtime teaching agreement against the
  parent rubric;
- record one short human-teaching verdict in the existing audit row;
- leave passing specimen source untouched;
- when a page fails, repair only its specimen composition, copy, state, or
  specimen-owned interaction wiring across the active runtimes that need it.

Do not change `TimeAgo`, `TimeInput`, or `TimeZoneSelect`; they are date/time
pages but belong to earlier defect-led curation, not this exact child. Do not
change component contracts, public APIs, component implementation, shared CSS,
date math, parsing, formatting, generated catalogue data, specimen
infrastructure, axis domains, or application behaviour. Do not add a
Conformance tab, exhaustive variant matrix, shared fixture corpus, schema, or
generated adapter. Size and density matrices stay in their dedicated panes.

If useful teaching requires a component-semantic or public-API change, stop and
report the page and exact need. If a picker, calendar gesture, range transition,
time-zone choice, or duration control is broken beneath the specimen, separate
that from dead specimen wiring and stop on the component defect.

Work only in the selected worker worktree. Never edit, clean, reset, or stash
over the orchestrator's planning checkout. Do not edit
`docs/roadmaps/dispatch.md`, change roadmap/card status, or merge the PR.

## Important Context

- `g15.026` proved all 174 native routes construct and every advertised native
  axis pane opens. It did not judge copy, visual quality, arbitrary controls,
  or narrow layout.
- The seven current audit rows say `A / A / A`, `keep`, and `no named defect`.
  Treat that as a screening result, not a conclusion.
- Human-centred means concise and representative. The first example should
  answer what the component is and how it is normally used. Do not enumerate a
  prop cross-product.
- Date fixtures should make the demonstrated state obvious and stable. Do not
  rewrite component date logic or widen this into deterministic-test cleanup;
  report an underlying defect if one appears.
- A useful loading, disabled, empty, error, narrow, or time-zone edge may stay.
  A state that merely lengthens the page should not.
- Svelte is the reference implementation. React should teach the same component
  using paired structure and copy. GPUI should teach the same important
  evidence within its renderer, without treating headless construction as a
  visual review.
- The live operator checkpoint happens in the orchestrator thread after the PR
  is ready. Do not claim the card complete or operator-approved in the batch
  log. List every changed web route so the orchestrator can open the pair.
- Record all seven verdicts even when most pages remain unchanged. Update the
  existing audit rows and use one August batch log for detail; do not create a
  second review table in the audit.
- Report after the initial seven-page pass with a compact `keep / repair / stop`
  inventory before making broad edits. Report again when the PR and validation
  are ready.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, read the
parent and child cards, working rules, audit, and specimen plan.

Start both web previews and inspect the seven routes side by side at ordinary
and narrow widths. Exercise the primary interaction rather than judging static
appearance alone. Read the corresponding GPUI specimen source plus the landed
probe evidence. Write the seven-page verdict inventory before editing; this is
the guard against turning the review into a date/time specimen rewrite.

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
   `git merge-base --is-ancestor 0ba51ca94e1eb02428d8be6bf4b3789d12e42cdd HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   `g15.011`, `g15.027`, `g15.029`, the audit, specimen plan, and working
   rules.
6. Use `effigy tasks` to confirm supported selectors. Do not run any windowed,
   native-visual, Jetstream, or release path.

### While you work

- Review all seven pages before changing one; report the first-pass inventory.
- Keep good pages byte-untouched and repair only named teaching defects.
- Keep paired web structure and copy aligned. Change GPUI only where its page
  fails to teach the same important evidence.
- Add focused evidence only when a changed specimen interaction could regress;
  do not build exhaustive specimen tests.
- Append one August `g15.029` batch log with all seven verdicts, changed routes,
  validation, and the pending operator checkpoint.
- Work in coherent chunks and commit meaningful results rather than model
  turns.
- Stop on any condition listed by the parent or child card.

### When the assigned runway is complete

1. Run every required selector named in Current State. Finish with
   `git diff --check origin/main...HEAD`.
2. Ensure the audit contains a human-teaching verdict for all seven pages and
   the batch log separates unchanged pages from repaired pages.
3. List every changed Svelte and React route for operator live review. State
   explicitly that sign-off is pending; do not mark the card complete.
4. Confirm no component, contract, public API, shared CSS, date math, generated
   catalogue, infrastructure, workflow, or Jetstream file changed.
5. Push the worker branch and open a reviewable PR against current `main`.
6. Link `g15.011`, `g15.027`, `g15.029`, the audit, batch log, changed routes,
   and validation in the PR body.
7. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect the PR independently, run the required checks,
and open every changed Svelte and React route for operator review. Because the
orchestrator and worker may share a GitHub identity, the verdict may be a PR
comment rather than a formal approval.

If changes are requested, make only those changes on this branch, push again,
and report back. Merge requires explicit operator authorisation after code
review, checks, and live operator sign-off.

- **Requested changes:** none yet.
- **Closeout refs:** `docs/roadmaps/g15/029-review-foundation-date-time.md`,
  the August batch log, `docs/roadmaps/g15/specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/011-specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`,
  `docs/roadmaps/g15/release-gap-register.md`, and
  `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns the seven-page review evidence, bounded specimen repairs, and
the batch log. The orchestrator owns card/roadmap status, live operator review,
merge, and promotion of `g15.030`. Leave the lane open if any changed web page
has not received operator sign-off.
