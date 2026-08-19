---
title: g15.023 foundation entry, content and status curation worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-19
updated: 2026-08-19
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260819-201239-g15-023-foundation-entry-curation.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, specimens, foundation]
---

## What This Thread Was Doing

Poodle's specimen catalogue is being returned to its human-facing purpose.
Earlier conformance work overloaded many `Examples` views with variant, state,
and size matrices. The first three bounded families are complete. This worker
owns the next family: the 11 foundation entry, content, and status pages in
`g15.023`.

This is one implementation lane and one PR. Start from the exact card and its
parent method. Do not extend the run into the remaining catalogue.

## Why It Matters

Poodle v0.2.0 needs a complete Svelte roster that people can understand and
adopt. Exhaustive behavior evidence belongs in focused tests and the later
conformance lane, not in the first page a component user opens. These pages
must quickly show normal use and the distinctions that matter.

The active runtimes still need one teaching contract: Svelte and React stay
paired, while GPUI teaches the same ordered stories without claiming behavior
its renderer does not have.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `aa451297961be3fd98e3c038774af7f5151d9eed`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that
  commit before this handoff was created.
- **Planning checkout:** clean, orchestrator-owned, and unavailable for worker
  edits.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** `g15.023` is dispatch-ready;
  its exact targets, exceptions, writable files, evidence, and stop conditions
  are committed.
- **Worker branch:** `t3code/g15-023-foundation-entry-curation`
- **Worker worktree:** launcher-provided clean, dedicated, registered
  non-`main` worktree.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from `.agents.local.env`.
- **Active spec lane:** human-centred specimen catalogue audit.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g15/023-curate-foundation-entry-content.md`
- **Allowed runway:** `g15.023` only.
- **Remaining card budget:** one card, one batch log, one PR, then stop.
- **Dispatch topology:** one serial lane. `g15.024` and later cards are not
  included here.
- **Parallel safety check:** this lane owns only the exact specimen files and
  Meter contract section listed by the card. Stop if another active worker
  touches those files or a shared catalogue surface becomes necessary.
- **Canonical refs:** `docs/contracts/001-working-rules.md`,
  `docs/roadmaps/g15/018-overloaded-examples-curation.md`,
  `docs/roadmaps/g15/specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/specimen-plan-outline.md`, and the 11 component contracts
  for Card, DetailItem, DragNumberField, EmptyState, Eyebrow, Meter, RefSelect,
  Select, Skeleton, SplitButton, and TextInput.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** headless only. Never run `*-windowed`,
  `test:native-visual`, conformance, Jetstream, or release selectors.
- **Required validation:** focused g15.023 regression,
  `effigy test:parity`, `effigy catalogue:check`, `effigy check:svelte`,
  `effigy react:build`, `effigy check:gpui`, `effigy docs:check`, and
  `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation, then orchestrator review and the
  operator's live Svelte/React review.
- **Merge authorisation:** absent. Push the PR and stop for review.

## Boundaries

Keep this run inside the exact 11-page list in `g15.023`:

- `Card`
- `DetailItem`
- `DragNumberField`
- `EmptyState`
- `Eyebrow`
- `Meter`
- `RefSelect`
- `Select`
- `Skeleton`
- `SplitButton`
- `TextInput`

The card's writable-file list is exact. Paired web edits are limited to
Eyebrow, Meter, RefSelect, Select, SplitButton, and TextInput. Native edits are
limited to the nine named GPUI specimen files. Evidence is limited to the new
g15.023 parity regression and one August log. The only contract edit permitted
is Meter section 13: remove duplicated ring-size teaching from Examples and
make the dedicated Sizes pane use one ring representative per size.

Card, DetailItem, DragNumberField, EmptyState, and Skeleton are paired-web
verification pages, not editing targets. DragNumberField and EmptyState are
full no-ops. Card, DetailItem, and Skeleton need only native trimming to match
the accepted web outline.

Out of scope are component implementations, public props or types, every other
contract section, shared specimen shells, catalogue navigation, generated
scene infrastructure, shared audio specimen definitions, exhaustive or
`Conformance` tabs, Jetstream validation, and every page owned by another
card.

Do not change a component to make its specimen easier to curate. If curation
exposes a component, contract, or renderer defect outside the narrow Meter
wording correction, record it and stop for an orchestrator decision. Do not
remove the only evidence for contracted behavior without naming the resulting
coverage disposition.

Work only in the selected worker worktree. Never edit, clean, reset, or stash
over the orchestrator's planning checkout. Do not edit
`docs/roadmaps/dispatch.md` or change roadmap/card status. Do not merge the PR.

## Important Context

- `g15.018` owns the curation method: normally 3–6 useful sections, realistic
  default first, distinct forms rather than prop cross-products, named
  removals, and preserved contract coverage.
- `g15.023` already contains the remeasured Svelte, React, and GPUI counts and
  the exact final caption order for every page. Follow it rather than
  re-designing the family from scratch.
- DragNumberField's eight sections and Skeleton's seven are intentional
  exceptions. They teach distinct contracted behaviors or public presets, not
  axis matrices.
- Meter is the important structural ruling. Keep threshold, custom-range,
  ring-readout, and ring-tone teaching. Remove `Ring sizes` from Examples and
  show ring scaling once in the Sizes pane. GPUI supports rings and must gain
  the ring stories.
- RefSelect, Select, SplitButton, and TextInput may combine several component
  instances under one caption when they answer one reader question. Keep
  interactive examples live and visibly observable.
- Svelte and React structure and copy must match. GPUI must teach the same
  ordered intent; `g15.026` owns mounted native page probing, so this card uses
  deterministic source evidence rather than building another harness.
- The operator will review every changed Svelte and React page live before the
  card can close. Open the PR with that checkpoint explicitly pending.
- Jetstream remains program-deferred. Shared Rust changes are not authorised
  by this card.
- The current `effigy doctor` board has pre-existing repository scan findings.
  They are not part of this card. Use the named selectors and report only new
  failures caused by the branch.

Report after the initial re-measurement and first coherent changed page group,
then again when the full family and validation are ready for PR review. Name
what changed, what was removed and why, which checks actually ran, and what
remains.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, read the
card, parent, audit, specimen outline, working rules, and the 11 component
contracts.

Reconfirm the committed baseline without widening the page set. Then implement
one coherent batch: the small semantic pages first (Eyebrow, Card,
DetailItem, Skeleton), followed by Meter, then the interactive entry controls.
Add the focused parity regression early enough that it guards exact caption
order and paired-web equality while the work proceeds.

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
   as the launcher-provided worktree regardless of its generated path or branch
   name. Record the actual values and do not create another worktree.
3. If the launcher supplied a dirty, `main`, or unregistered context, stop and
   report it. Do not clean or reset it. A manual fallback is allowed only after
   reading `.agents.local.env`, finding a valid
   `AGENTS_WORKTREE_CONTAINER_DIR`, and creating a unique worktree there from
   `origin/main`; ask the operator if the key is absent. Never use `/tmp`,
   `TMPDIR`, or a guessed repository-adjacent path.
4. From the accepted worktree, run `git fetch origin`, confirm `HEAD` equals
   current `origin/main`, confirm
   `git merge-base --is-ancestor aa451297961be3fd98e3c038774af7f5151d9eed HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   `g15.018`, `g15.023`, the audit, the specimen outline, working rules, and
   these contracts completely: `card.md`, `detail-item.md`,
   `drag-number-field.md`, `empty-state.md`, `eyebrow.md`, `meter.md`,
   `ref-select.md`, `select.md`, `skeleton.md`, `split-button.md`, and
   `text-input.md`.
6. Use `effigy tasks` for the supported selector inventory. Do not use a
   windowed or Jetstream path.

### While you work

- Keep the exact target caption order and story mapping from `g15.023`.
- Keep Examples human-centred. Dedicated size and density panes retain their
  own one-representative-per-step matrices.
- Name every removed or combined example and its evidence disposition.
- Preserve visible host feedback for retained interactive controls.
- Keep Svelte and React structure/copy paired and GPUI teaching equivalent.
- Do not churn the five accepted paired-web pages.
- Work in coherent page groups and commit meaningful chunks rather than model
  turns.
- Append evidence to
  `docs/logs/2026-08/20260819-g15-023-foundation-entry-curation.md`.
- Report through the operator after the first coherent group and at PR-ready
  state.
- Stop on any condition listed by the card.

### When the assigned runway is complete

1. Run the focused regression, then every required selector named in Current
   State. Finish with `git diff --check origin/main...HEAD`.
2. Ensure the batch log records baseline/final counts, named removals and
   combinations, contract coverage decisions, changed files, exact validation
   outcomes, and every intentionally unchanged page.
3. Run the Svelte and React previews for the operator checkpoint, but leave the
   review item open until the operator actually inspects the six changed web
   pages.
4. Push the worker branch and open a reviewable PR against current `main`.
5. Link `g15.018`, `g15.023`, the batch log, changed surfaces, validation, and
   the pending live-review checkpoint in the PR body.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect the PR independently against the card, parent,
contracts, diff, checks, and batch log. Because the orchestrator and worker may
share a GitHub identity, the verdict may be recorded as a PR comment rather
than a formal approval.

If changes are requested, make only those changes on this branch, push again,
and report back. The operator's live Svelte/React review remains part of
acceptance. Merge requires explicit operator authorisation after both review
paths are satisfied.

- **Requested changes:** none yet.
- **Closeout refs:** `docs/roadmaps/g15/023-curate-foundation-entry-content.md`,
  the August batch log, `docs/roadmaps/g15/018-overloaded-examples-curation.md`,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`, and
  `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns implementation evidence and the batch log. The orchestrator
owns dispatch status, roadmap status, merge, and the next card. If the family
is blocked, leave those surfaces open and report the exact blocker instead of
making the card look complete.
