---
title: g15.028 screen-clear foundation controls and entry worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260820-143005-g15-028-review-foundation-controls-entry.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, specimens, web, gpui]
---

## What This Thread Was Doing

Poodle has finished the defect-led specimen curation and the headless native
construction probe. Fifty-six catalogue pages screened clear mechanically but
have not received a human teaching review. This worker owns the first exact
family: 14 foundation controls and entry pages.

Review all 14 before editing. Keep a good page unchanged. Repair only specimen
presentation defects that stop the page teaching normal use clearly across
Svelte, React, and GPUI.

## Why It Matters

Green tests and an A screening grade do not prove that a specimen is useful.
This lane closes that gap without turning the catalogue back into an exhaustive
conformance corpus. It is part of the last specimen-audit work required before
Poodle can build the primitive-first visual conformance lane and certify
v0.2.0.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `48e0c238c19244f21dcd6ce081f7c00ae89695c8`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that
  commit before this handoff was created.
- **Planning checkout:** clean, orchestrator-owned, and unavailable for worker
  edits.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** PR #50 / `g15.026` are closed;
  `g15.028` is ready and the generation front doors name it as the only current
  task.
- **Worker branch:** `t3code/g15-028-review-foundation-controls-entry`
- **Worker worktree:** launcher-provided clean, dedicated, registered
  non-`main` worktree.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from `.agents.local.env`.
- **Active spec lane:** g15 human-centred specimen catalogue completion.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g15/028-review-foundation-controls-entry.md`
- **Allowed runway:** `g15.028` only.
- **Remaining card budget:** one 14-page review, one batch log, one PR, then
  stop.
- **Dispatch topology:** serial. `g15.029`–`g15.033` follow only after this
  child is reviewed and merged.
- **Parallel safety check:** all six children edit the same catalogue audit and
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

This run owns exactly these 14 pages:

- actions and selection: `Checkbox`, `CollapseToggle`, `Radio`, `RadioGroup`,
  `SegmentedControl`, `Switch`, `ToggleGroup`;
- text and value entry: `CodeInput`, `ColorPicker`, `EditableLabel`,
  `NumberInput`, `Rating`, `Slider`, `ThemeSelect`.

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

Do not change component contracts, public APIs, component implementation,
shared CSS, generated catalogue data, specimen infrastructure, axis domains,
or application behaviour. Do not add a Conformance tab, exhaustive variant
matrix, shared fixture corpus, schema, or generated adapter. Size and density
matrices stay in their existing dedicated panes.

If useful teaching requires a component-semantic or public-API change, stop and
report the page and exact need. If an underlying component control is broken,
distinguish that from dead specimen wiring and stop on the component defect.

Work only in the selected worker worktree. Never edit, clean, reset, or stash
over the orchestrator's planning checkout. Do not edit
`docs/roadmaps/dispatch.md`, change roadmap/card status, or merge the PR.

## Important Context

- `g15.026` proved all 174 native routes construct and every advertised native
  axis pane opens. It did not judge copy, visual quality, arbitrary controls,
  or narrow layout.
- The current audit rows say `A / A / A`, `keep`, and `no named defect` for
  these pages. Treat that as a screening result, not a conclusion to preserve.
- Human-centred means concise and representative. The first example should
  answer what the component is and how it is normally used. Do not enumerate a
  prop cross-product.
- A useful loading, disabled, empty, error, or narrow example may stay. A state
  that merely makes the page longer should not.
- Svelte is the reference implementation. React should teach the same component
  using paired structure and copy. GPUI should teach the same important
  evidence within its renderer, without pretending headless construction is a
  visual review.
- The live operator checkpoint happens in the orchestrator thread after the PR
  is ready. Do not claim the card complete or operator-approved in the batch
  log. List every changed route so the orchestrator can open the paired pages.
- Record all 14 verdicts even when most pages remain unchanged. Do not create a
  second review table; update their existing audit rows and use one August batch
  log for detail.
- Report after the initial 14-page pass with a compact `keep / repair / stop`
  inventory before making broad edits. Report again when the PR and validation
  are ready.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, read the
parent and child cards, working rules, audit, and specimen plan.

Start both web previews, inspect the 14 routes at ordinary and narrow widths,
and compare their first examples and interactions side by side. Read the
corresponding GPUI specimen source plus the landed probe evidence. Write the
14-page verdict inventory before editing; this is the guard against turning a
screen-clear review into another specimen rewrite.

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
   `git merge-base --is-ancestor 48e0c238c19244f21dcd6ce081f7c00ae89695c8 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   `g15.011`, `g15.027`, `g15.028`, the audit, specimen plan, and working
   rules.
6. Use `effigy tasks` to confirm supported selectors. Do not run any windowed,
   native-visual, Jetstream, or release path.

### While you work

- Review all 14 pages before changing one; report the first-pass inventory.
- Keep good pages byte-untouched and repair only named teaching defects.
- Keep paired web structure and copy aligned. Change GPUI only where its page
  fails to teach the same important evidence.
- Add focused evidence only when a changed specimen interaction could regress;
  do not build exhaustive specimen tests.
- Append one August `g15.028` batch log with all 14 verdicts, changed routes,
  validation, and the pending operator checkpoint.
- Work in coherent chunks and commit meaningful results rather than model
  turns.
- Stop on any condition listed by the parent or child card.

### When the assigned runway is complete

1. Run every required selector named in Current State. Finish with
   `git diff --check origin/main...HEAD`.
2. Ensure the audit contains a human-teaching verdict for all 14 pages and the
   batch log separates unchanged pages from repaired pages.
3. List every changed Svelte and React route for operator live review. State
   explicitly that sign-off is pending; do not mark the card complete.
4. Confirm no component, contract, public API, generated catalogue,
   infrastructure, workflow, or Jetstream file changed.
5. Push the worker branch and open a reviewable PR against current `main`.
6. Link `g15.011`, `g15.027`, `g15.028`, the audit, batch log, changed routes,
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
- **Closeout refs:** `docs/roadmaps/g15/028-review-foundation-controls-entry.md`,
  the August batch log, `docs/roadmaps/g15/specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/011-specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`,
  `docs/roadmaps/g15/release-gap-register.md`, and
  `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns the 14-page review evidence, bounded specimen repairs, and the
batch log. The orchestrator owns card/roadmap status, live operator review,
merge, and promotion of `g15.029`. Leave the lane open if any changed page has
not received operator sign-off.
