---
title: g15.038 SegmentedControl native option parity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260820-162916-g15-038-segmented-control-native-option-parity.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, segmented-control, rust, gpui]
---

## What This Thread Was Doing

The first screen-clear specimen review found a real active-runtime contract
gap rather than a specimen-only defect. Svelte and React SegmentedControl
options support icons, icon-only presentation, accessible label fallback, and
titles. Shared Rust still reuses the broad `ChoiceOption` type and cannot
represent that surface, so GPUI cannot teach the contracted icon-only example.

This worker owns the bounded parity repair. Introduce the dedicated Rust option
shape, migrate Poodle's callers cleanly, render the contracted presentation,
add the native specimen and evidence, then close the recorded blocker.

## Why It Matters

Poodle's active completion cohort cannot call a primitive complete while one
runtime cannot express a contracted option shape. SegmentedControl will also be
part of the primitive-first visual conformance work, so carrying this gap
forward would make that evidence knowingly incomplete. Closing it now keeps the
release runway honest without reopening a universal component representation.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `b5e8fd62def6706ad25b7892f7ce2bb3161267ff`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that
  commit before this handoff was created.
- **Planning checkout:** clean, orchestrator-owned, and unavailable for worker
  edits.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** PR #51 / `g15.028` are closed;
  `g15.038` is ready; the generation front doors and release register route the
  SegmentedControl blocker here.
- **Worker branch:** `t3code/g15-038-segmented-control-native-option-parity`
- **Worker worktree:** launcher-provided clean, dedicated, registered
  non-`main` worktree.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from `.agents.local.env`.
- **Active spec lane:** active-cohort primitive contract parity before visual
  conformance.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g15/038-segmented-control-native-option-parity.md`
- **Allowed runway:** `g15.038` only.
- **Remaining card budget:** one clean Rust API migration, renderer/backend
  projection, GPUI specimen, evidence, one batch log, one PR, then stop.
- **Dispatch topology:** serial. `g15.029`–`g15.033` resume only after this PR
  is reviewed and merged.
- **Parallel safety check:** this card and the remaining review children share
  `specimen-catalogue-audit.md`; do not absorb or start another child.
- **Canonical refs:** `docs/contracts/001-working-rules.md`,
  `docs/contracts/components/segmented-control.md`,
  `docs/roadmaps/g15/specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/release-gap-register.md`, and
  `docs/roadmaps/g15/038-segmented-control-native-option-parity.md`.
- **Model capability profile:** frontier coding model, high reasoning; this is
  a public pre-1.0 Rust API migration plus shared renderer/backend work.
- **Tool/runtime restrictions:** headless only. Never run `*-windowed`,
  `test:native-visual`, Jetstream, or release selectors.
- **Required validation:** focused specs/render/backend/specimen tests,
  `effigy ci:rust`, `effigy check:gpui`, `effigy regressions:native`,
  `effigy probe:gpui-specimens`, `effigy docs:check`, final headless
  `effigy qa`, and `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation and orchestrator review. No web
  specimen changes are expected, so no paired live-web checkpoint is planned.
- **Merge authorisation:** absent. Push the PR and stop for review.

## Boundaries

Implement only the ready card. The operator explicitly approved a clean
pre-1.0 break: replace SegmentedControl's use of `ChoiceOption` with a dedicated
public `SegmentedControlOption` and migrate all in-repo callers. Do not add an
alias, `From<ChoiceOption>` conversion, overloaded constructor, deprecated twin,
silent fallback, or any other compatibility surface.

The dedicated Rust option must carry every portable contract field: value,
label, optional icon name, icon-only state, disabled state, optional accessible
label, and optional title. Keep `ChoiceOption` unchanged for Select,
RadioGroup, CardRadioGroup, and every other family.

Shared rendering must:

- render an optional named icon before visible label text;
- hide label text only when icon-only is true and an icon exists;
- preserve the label when icon-only is requested without an icon;
- use explicit `aria_label` first, then required label, as the accessible name;
- use explicit title first, then the required label for an icon-only tooltip;
- match supporting-visual icon sizing, icon/text gap, and compact square
  geometry when `equal_width=false`.

The current node vocabulary has named icons but no obvious generic tooltip
declaration. Add only the smallest reusable optional node/backend field needed
to project the contracted tooltip in GPUI. If GPUI cannot project that field
without a component-specific overlay or a new general overlay architecture,
stop and report the exact limitation; do not invent that architecture inside
this card.

Add the contract's Effects/Instruments icon-only case to the GPUI specimen and
keep selection live. Add focused evidence that would fail if icon content,
visible-label suppression, accessible fallback, square geometry, tooltip
projection, or activation regresses.

Do not change Svelte or React behavior, public web APIs, shared CSS, other
choice-family APIs, generated catalogue data, visual-conformance fixtures, or
release automation. Do not edit `docs/roadmaps/dispatch.md`, change card or
front-door status, or merge the PR. Work only in the selected worker worktree;
never edit, clean, reset, or stash over the orchestrator's planning checkout.

## Important Context

- Svelte is the reference implementation and the component contract already
  defines the target. This is a port, not a product/API design exercise.
- `SegmentedControlSpec` currently exposes `Vec<ChoiceOption>` from
  `packages/contracts/components/src/segmented_control.rs`; `ChoiceOption`
  itself lives in `types.rs` and is used across unrelated choice families.
- `poodle-render::segmented_control` currently creates one text-labelled button
  node per option. Reuse the existing named `Node::icon` path and the same
  supporting-visual sizing principles used by Tabs/Button; do not add another
  icon registry.
- `docs/contracts/components/segmented-control.md` still records the native
  icon omission as a provisional delta. Remove that execution-era delta only
  when the implementation and evidence close it.
- The audit currently reads GPUI B / `contract/runtime-blocker`, with keep 55
  and blocker 1. Successful closeout restores GPUI A / `keep` and reconciles
  all affected grade/disposition totals.
- The release gap register already names `g15.038`. Update the row to closed;
  do not rewrite unrelated gaps such as UiPresentationProvider or Stepper.
- A Rust root export change is public. The batch log and PR must call the type
  migration breaking, pre-1.0, and explicitly operator-approved.
- Report after the dedicated type migration compiles across all in-repo callers
  and again when the complete PR/evidence is ready. Stop sooner if the tooltip
  boundary or another card stop condition fires.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before any broad repository reads. Once the worktree is accepted, read
the card, working rules, SegmentedControl contract, current Rust spec, renderer,
GPUI specimen, and their focused evidence.

Inventory every `SegmentedControlSpec` construction before editing. Make the
dedicated option type and migrate those callers as one coherent compile-first
chunk. Then implement renderer and tooltip projection, add the GPUI specimen,
and finish with focused tests before the broader headless board.

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
   `git merge-base --is-ancestor b5e8fd62def6706ad25b7892f7ce2bb3161267ff HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   `g15.038`, the working rules, SegmentedControl contract, audit, and release
   gap register.
6. Use `effigy tasks` to confirm supported selectors. Do not run any windowed,
   native-visual, Jetstream, or release path.

### While you work

- Keep the option-type migration clean and complete; compile after migrating
  all call sites before layering on renderer behavior.
- Keep each test owner-local and capable of detecting the old omission. A
  construction-only probe is not icon/tooltip evidence.
- Preserve existing selected, disabled, size, density, focus, and activation
  behavior while adding icon presentation.
- Append one August `g15.038` batch log with migration classification,
  implementation/evidence details, audit/register reconciliation, validation,
  and unresolved items.
- Work in meaningful chunks and commit coherent results rather than model
  turns.
- Stop on any condition named by the card or this handoff.

### When the assigned runway is complete

1. Run every required selector named in Current State. Finish with
   `git diff --check origin/main...HEAD`.
2. Confirm no SegmentedControl call site still uses `ChoiceOption`, no
   compatibility surface exists, and other choice APIs are unchanged.
3. Confirm the audit totals reconcile, the release gap is closed, and the
   provisional native icon delta is removed only after evidence passes.
4. Push the worker branch and open a reviewable PR against current `main`.
5. Link `g15.038`, the SegmentedControl contract, audit/register rows, batch
   log, changed surfaces, breaking-migration approval, and validation in the PR
   body.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect the PR independently against the contract, card,
diff, tests, and checks. Because the orchestrator and worker may share a GitHub
identity, the verdict may be a PR comment rather than a formal approval.

If changes are requested, make only those changes on this branch, push again,
and report back. Merge requires explicit operator authorisation after code
review and checks.

- **Requested changes:** none yet.
- **Closeout refs:**
  `docs/roadmaps/g15/038-segmented-control-native-option-parity.md`, the August
  batch log, `docs/contracts/components/segmented-control.md`,
  `docs/roadmaps/g15/specimen-catalogue-audit.md`,
  `docs/roadmaps/g15/release-gap-register.md`, `docs/roadmaps/g15/README.md`,
  `docs/roadmaps/generation-index.md`, and `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns the dedicated Rust option migration, shared/native behavior,
GPUI teaching page, focused evidence, audit/register closeout, and batch log.
The orchestrator owns card/front-door status, merge, and promotion of
`g15.029`. Leave the lane open if any contracted behavior or validation remains
unproved.
