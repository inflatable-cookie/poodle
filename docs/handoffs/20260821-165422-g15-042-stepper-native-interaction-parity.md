---
title: g15.042 Stepper native interaction parity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-21
updated: 2026-08-21
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260821-165422-g15-042-stepper-native-interaction-parity.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, stepper, gpui, interaction]
---

## What This Thread Was Doing

The g15 release audit found one concrete Stepper gap in the active native
cohort. Shared Rust composition already renders separate step-selection,
re-run, and collapse controls, but the GPUI preview adapter wires only collapse.
The specimen therefore advertises controls that do nothing.

Execute `g15.042`: connect the existing handler path, retain specimen state so
the result is visible, and prove selection, re-run, and collapse through the
mounted headless GPUI backend.

This is one bounded implementation handoff. You do not need the originating
transcript or a second prompt.

## Why It Matters

Poodle v0.2.0 is supposed to ship an honest active cohort. An inert native
Stepper cannot be certified merely because its node tree looks right. This
card closes the behavior gap without creating a GPUI-only Stepper or changing
the public contract.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `964e6c6f961e59b9eceba18f48ac670edcb79128`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- **Posture:** `strict-ready`.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts at the base:** the recompiled g15 runway, ready card
  `g15.042`, Stepper release-gap row, and runway-recompile log.
- **Worker branch:** `t3code/g15-042-stepper-native-interaction`
- **Worker worktree:** use the clean, registered, dedicated non-`main`
  worktree supplied by the launcher, regardless of generated path or branch
  name.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from ignored `.agents.local.env`.
- **Active lane:** g15 native interaction-gap closure.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready card:**
  `docs/roadmaps/g15/042-stepper-native-interaction-parity.md`
- **Allowed runway:** `g15.042` only.
- **Remaining budget:** one native interaction repair, focused evidence, one
  August batch log, and one PR; then stop.
- **Dispatch topology:** parallel with `g15.041` and `g15.044`.
- **Parallel safety:** this lane owns Stepper GPUI adapter/specimen and focused
  native evidence only. Do not edit Popover, Button, capture tooling, visual
  conformance, or another worker's audit/closeout surfaces.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/005-agent-local-paths.md`, and
  `docs/contracts/components/stepper.md`.
- **Gap evidence:** `docs/roadmaps/g15/release-gap-register.md` and
  `docs/roadmaps/g15/025-curate-collections-navigation-tail.md`.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** all GPUI validation is headless. Never run
  `*-windowed`, `test:native-visual`, Jetstream, visual-conformance, or release
  selectors.
- **Required validation:** focused Stepper Rust tests,
  `effigy regressions:native`, `effigy check:gpui`, `effigy docs:check`, and
  `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation and orchestrator review.
- **Merge authorisation:** absent. Push the PR and stop for review.

The open motion-learning and Longhorn conformance-lab triage notes are unrelated
to this lane. Leave them open and unchanged. Do not run `effigy doctor`; the
card already names its selectors.

## Boundaries

Please keep the run within the card's fixed seam:

- Add `on_change` and `on_rerun` builders to the existing preview/native
  Stepper adapter and reuse `poodle_render::StepperHandlers`.
- Give the GPUI specimen retained current-step state and a concise visible
  re-run receipt.
- Prove the real mounted controls. Do not call handler closures directly.
- Keep selection, re-run, and vertical collapse independent.
- Preserve disabled suppression, stable focus, arrow/Home/End movement, and
  pointer/keyboard activation parity.
- Close only the Stepper row in `release-gap-register.md`; add one August log.

Writable scope:

- `packages/gpui/preview/src/node_compat.rs`
- `packages/gpui/preview/src/specimens/stepper.rs`
- focused Stepper evidence under `packages/render/` and
  `packages/gpui/preview/tests/`
- `docs/roadmaps/g15/release-gap-register.md`
- one August g15.042 batch log
- `PAPERCUTS.md` only for newly discovered execution friction

Out of scope:

- Stepper public-contract or paired-web API changes;
- a second Stepper implementation or component-specific GPUI backend path;
- Popover/Button files owned by `g15.041`;
- offscreen capture, named visual fixtures, baselines, or comparison tooling;
- Jetstream, workflow, release, sibling-repository, roadmap-status, generation
  front-door, or dispatch-ledger edits;
- merging the PR.

Stop on any condition in the card. In particular, stop if the existing node
activation model cannot distinguish selection from re-run, correct focus needs
a Stepper-wide backend architecture, or the fix requires a public/web API
change.

## Important Context

- The native implementation pair has one authority: Stepper composition stays
  in `poodle-render`; GPUI only binds the existing node interactions.
- Selection and re-run are intentionally separate. Re-run may cost real time or
  money and must never silently select or navigate.
- Re-run appears only for a completed step when the handler exists. It emits
  that step's exact value once.
- A disabled step cannot select or emit. Arrow focus movement skips disabled
  steps without committing selection.
- Collapse is vertical-only and remains independent of the other callbacks.
- Keep Examples human-centred. A current-step label plus one short re-run
  receipt is enough; do not turn the specimen into an event transcript or
  exhaustive matrix.
- `packages/gpui/preview/tests/headless_regressions.rs` may be a shared merge
  hotspot. Prefer the narrowest owner-local test location. If the real mounted
  proof belongs there, keep the diff Stepper-only and rebase onto current
  `main` before handing off.
- `g15.041` is in flight but has no intended Stepper ownership. If an actual
  shared mutable file appears, stop and report the overlap instead of resolving
  it by widening scope.

Work in two meaningful chunks:

1. adapter wiring plus focused shared-render evidence;
2. retained specimen, mounted GPUI regression, release-gap/log closeout, and
   final headless validation.

Report after each chunk with changed files, validation actually run, remaining
work, and blockers.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, read the
Stepper contract, ready card, system shape, and gap row. Use `effigy tasks` to
confirm the named selectors, then inspect the current `StepperHandlers`, node
identities, GPUI adapter, specimen, and mounted-driver precedents.

Start by writing the smallest mounted failing evidence that demonstrates the
inert selection and re-run controls. Then bind the existing handlers and make
the specimen receipt visible without changing its teaching structure.

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
   `git merge-base --is-ancestor 964e6c6f961e59b9eceba18f48ac670edcb79128 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   `g15.042`, the Stepper contract, system shape, local-path contract, and the
   Stepper release-gap row.
6. Use `effigy tasks` to confirm selectors. Do not run `effigy doctor` or any
   windowed, native-visual, Jetstream, visual-conformance, or release path.

### While you work

- Keep commits aligned with the two meaningful chunks above.
- Use the existing handler and node vocabulary. Stop rather than adding a
  component-specific backend escape hatch.
- Drive mounted nodes through the headless GPUI backend; direct closure calls
  are not acceptance evidence.
- Test enabled selection, disabled suppression, permitted re-run, exact once
  emission, non-selection on re-run, collapse independence, and keyboard and
  pointer paths.
- Keep the specimen concise and update generated material only through its
  owning generator.
- Append a PAPERCUTS entry only for new small execution friction.
- Stop and report any card stop condition, shared-file collision, or validation
  result that changes the plan.

### When the assigned runway is complete

1. Run the complete headless validation list from the card.
2. Finish with `effigy docs:check` and
   `git diff --check origin/main...HEAD`.
3. Confirm the release-gap row cites the mounted evidence and the August log
   records actual commands/results without changing roadmap status.
4. Rebase onto current `main` if the in-flight parallel lanes have landed, then
   rerun the affected checks.
5. Push the worker branch and open one reviewable PR against current `main`.
   The planning base above predates the handoff commit; it is intentionally not
   the commit containing this file.
6. In the PR body, link g15.042, the Stepper contract, release-gap row, batch
   log, changed surfaces, and every validation result.
7. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect the PR independently and rerun the required
headless checks. Because the orchestrator and worker may share a GitHub
identity, the verdict may be a PR comment rather than formal approval.

If changes are requested, make only those changes on this branch, push again,
and report back. Merge requires explicit operator authorisation.

- **Requested changes:** none yet.
- **Closeout refs:**
  `docs/roadmaps/g15/042-stepper-native-interaction-parity.md`,
  `docs/roadmaps/g15/release-gap-register.md`, the g15.042 August log,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`, and
  `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns the Stepper implementation, focused evidence, release-gap row,
and batch log. The orchestrator owns review, merge, card/roadmap status, and
promotion of the next runway step. Leave the lane open if any mounted semantic
proof or required validation remains.
