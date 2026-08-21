---
title: g15.044 GPUI offscreen capture feasibility worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-21
updated: 2026-08-21
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260821-165423-g15-044-gpui-offscreen-capture-feasibility.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, gpui, offscreen, research]
---

## What This Thread Was Doing

Poodle needs native pixel evidence that never opens a desktop window or steals
focus. The current GPUI 0.2.2 test platform gives deterministic construction,
geometry, and interaction evidence but no raster readback. The retained visual
gate opens a real window and uses macOS screen capture, so it is not an
acceptable local path.

Execute `g15.044`: run a bounded feasibility proof against an exact upstream
GPUI revision, attempt to render a real Poodle Button to pixels offscreen, and
return a measured go/no-go plus migration cost. Do not adopt the dependency or
build the full conformance lab.

This is one proof-oriented worker handoff. You do not need the originating
transcript or a second prompt.

## Why It Matters

The first primitive visual-comparison batch cannot include genuine GPUI output
until Poodle can read native pixels without taking over the operator's machine.
This card is the decision boundary: it either proves a safe platform seam and
unblocks adoption, or records precisely why the native visual lane remains
blocked. It prevents a third broad conformance architecture from being built on
an unproved renderer assumption.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `964e6c6f961e59b9eceba18f48ac670edcb79128`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- **Posture:** `strict-ready` research/proof lane.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts at the base:** recompiled parent `g15.012`, ready card
  `g15.044`, release-gap row, Longhorn-lab triage note, and runway-recompile
  log.
- **Worker branch:** `t3code/g15-044-gpui-offscreen-feasibility`
- **Worker worktree:** use the clean, registered, dedicated non-`main`
  worktree supplied by the launcher, regardless of generated path or branch
  name.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from ignored `.agents.local.env`.
- **Active lane:** g15 primitive visual-conformance capture feasibility.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Parent/card:** `docs/roadmaps/g15/012-visual-conformance-lane.md` and
  `docs/roadmaps/g15/044-gpui-offscreen-capture-feasibility.md`
- **Allowed runway:** `g15.044` only. Do not start `g15.045`.
- **Remaining budget:** one isolated proof, one promoted research decision,
  parent/card evidence updates, one August batch log, and one PR; then stop.
- **Dispatch topology:** parallel with `g15.041` and `g15.042`.
- **Parallel safety:** this lane changes no production package and owns only
  bounded research/proof documentation. Do not edit Stepper, Popover, Button
  production code, the specimen audit, or another worker's closeout surfaces.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/005-agent-local-paths.md`,
  `docs/roadmaps/g14/conformance-estate.md`, and
  `docs/roadmaps/g14/022-generation-closeout.md`.
- **Planning context:**
  `docs/triage/20260821-165500-longhorn-conformance-lab.md` and
  `docs/logs/2026-08/20260821-g15-release-runway-recompile.md`.
- **Sibling evidence:** inspect
  `/Users/tom/Dev/projects/longhorn/docs/contracts/022-agent-app-control.md`
  read-only. Never modify the Longhorn checkout.
- **Model capability profile:** frontier coding/research model, high reasoning.
- **Tool/runtime restrictions:** no visible `NSWindow`, screen capture, focus,
  pointer movement, accessibility permission, or Screen Recording permission.
  Never run `*-windowed`, `test:native-visual`, Jetstream, release, or workflow
  selectors.
- **Required validation:** exact isolated proof command recorded in the log,
  `effigy docs:check`, and `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting feasibility evidence and orchestrator/operator
  decision on the verdict.
- **Merge authorisation:** absent. Push the PR and stop for review.

The motion-learning triage note is unrelated and remains open. Do not run
`effigy doctor`; the card's evidence route is explicit.

## Boundaries

Please keep this as a feasibility proof:

- Confirm from source that pinned GPUI 0.2.2 lacks raster readback and identify
  the current windowed screen-capture path.
- Resolve one exact immutable upstream GPUI version or commit containing the
  candidate offscreen API. Do not base the verdict only on a mutable local
  checkout.
- In an isolated throwaway harness, render at least one real Poodle Button
  scene to RGBA/PNG without a visible window or desktop capture.
- Record toolchain, Metal/device needs, fonts/theme, viewport/scale control,
  startup cost, and repeated-output stability.
- Measure the production migration surface without committing it: dependency
  and lock changes plus compile failures across the GPUI adapter, node backend,
  preview, and headless tests.
- Inspect Longhorn contract 022 read-only and preserve its boundary: it can
  control and capture webviews, not genuine native GPUI output.
- Produce one promoted capture-platform decision and update parent evidence.

Writable scope:

- one bounded research note under `docs/research/`
- `docs/roadmaps/g15/012-visual-conformance-lane.md` and
  `docs/roadmaps/g15/044-gpui-offscreen-capture-feasibility.md` for evidence,
  verdict, and honest continuation state
- `docs/roadmaps/g15/release-gap-register.md` only if the card's measured
  verdict changes that row's evidence
- one August g15.044 batch log
- an optional retained proof fixture only if it is fully dependency-isolated
  and cannot enter package, workspace, QA, or release graphs
- `PAPERCUTS.md` only for newly discovered execution friction

Out of scope:

- changing any production GPUI manifest, lockfile, adapter, backend, preview,
  public API, package graph, baseline, workflow, or release artifact;
- building `g15.045`, named fixtures, the comparator, or a full Tauri/Longhorn
  conformance app;
- wrapping the current windowed gate, using `screencapture`, or asking for
  desktop permissions;
- copying mutable upstream code into Poodle without an immutable source and
  licence check;
- changing the Longhorn repository or treating its control plane as Poodle
  component authority;
- Stepper/Popover/Button implementation, Jetstream, roadmap front doors,
  dispatch ledger, or merging the PR.

Stop on any condition in the card. In particular, stop with a `no-go` if the
proof needs a visible window, private platform API, screen capture/permissions,
or an unbounded renderer/backend redesign. Also stop if the proof starts
growing shared fixture or comparison semantics.

## Important Context

- Headless layout/interaction and headless pixels are different claims. GPUI
  0.2.2 already provides the first; this card must prove or reject the second.
- Local source reconnaissance found newer GPUI code with offscreen-window and
  image-readback APIs. Treat that only as a lead. Name an upstream immutable
  revision and reproduce it independently.
- A successful proof must use a real Poodle primitive path, not a plain GPUI
  rectangle. Keep the scene as small as possible so the experiment measures
  capture feasibility rather than fixture design.
- The proof should control viewport and scale and run repeatedly. Record hashes
  or a similarly concrete stability measure; do not assert determinism from one
  image.
- Migration costing may use a disposable copy or temporary harness. Temporary
  proof directories should come from `mktemp -d`; do not create Git worktrees
  there. Never write dependency experiments into the worker checkout and then
  try to clean them away.
- Do not alter the production pin even on a `go`. The next card, `g15.045`, is
  the only adoption authority and requires operator review of this verdict.
- Longhorn/Tauri remains useful later as the unfocused control plane for
  Svelte and React webviews and perhaps a GPUI sidecar. It cannot solve native
  pixels by itself and is not a v0.2.0 prerequisite.
- The rejected g14 executable corpus remains rejected. This proof must not
  invent a portable component language, completion gate, or universal scene.
- If a retained proof fixture would affect package/workspace discovery or
  broaden validation, leave only the reproducible command and research evidence
  instead.

Work in three meaningful chunks:

1. current-pin and upstream-source evidence with an exact candidate revision;
2. isolated real-Poodle pixel proof plus repeatability and migration costing;
3. promoted go/no-go decision, parent/card/log updates, and docs validation.

Report after each chunk with evidence gathered, commands actually run, current
verdict, remaining work, and blockers.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, read the
parent, card, conformance-estate ledger, runway-recompile log, Longhorn-lab note,
and Longhorn contract 022. Use exact source searches to establish the 0.2.2
limit and current capture path.

Then identify one immutable upstream GPUI candidate and build the smallest
disposable harness that can render a real Poodle Button offscreen. Do not touch
the production dependency graph while proving it.

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
   `g15.012`, `g15.044`, the conformance-estate ledger, system shape,
   working/local-path contracts, the runway-recompile log, the Longhorn-lab
   note, and Longhorn contract 022 read-only.
6. Use `effigy tasks` only if selector discovery is needed for the final docs
   check. Do not run `effigy doctor` or any windowed, native-visual, Jetstream,
   release, or workflow path.

### While you work

- Keep all dependency and migration experiments outside the worker checkout.
- Record exact upstream repository URLs, immutable revisions, commands,
  toolchain versions, and relevant source paths.
- Prove the absence of a visible window and desktop capture by construction;
  do not test that boundary by briefly opening a window.
- Use one real Poodle Button and retain the produced PNG as evidence only when
  the card's isolated-fixture rule permits it.
- Run repeated captures under the same inputs and record concrete stability
  evidence. Separate stable rendering from environmental font/GPU caveats.
- Cost production adoption from disposable manifest/lock and compile evidence;
  do not commit those changes.
- State `go` or `no-go` plainly. A conditional verdict must name the exact
  unresolved measurement and stops the lane.
- Append a PAPERCUTS entry only for new small execution friction.
- Stop and report any card stop condition, scope expansion, sibling write, or
  validation result that changes the plan.

### When the assigned runway is complete

1. Record the exact isolated proof command and result in the g15.044 log.
2. Ensure the research note contains current-pin evidence, immutable candidate,
   real-Poodle result, repeatability, environment needs, migration cost, and the
   go/no-go decision.
3. Update parent/card continuation honestly. Do not mark or start g15.045; the
   orchestrator and operator decide that after review.
4. Run `effigy docs:check` and
   `git diff --check origin/main...HEAD`.
5. Confirm no production manifest, lockfile, package, source, baseline,
   workflow, release artifact, or Longhorn file changed.
6. Rebase onto current `main` if parallel lanes landed, then rerun the docs and
   diff checks.
7. Push the worker branch and open one reviewable PR against current `main`.
   The planning base above predates the handoff commit; it is intentionally not
   the commit containing this file.
8. In the PR body, link g15.012, g15.044, the research note, batch log,
   conformance-estate evidence, Longhorn boundary, exact commands, verdict, and
   unresolved risks.
9. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will verify the evidence independently and bring the go/no-go
decision to the operator. A `go` only makes `g15.045` eligible for planning;
it does not authorise adoption in this branch. Because the orchestrator and
worker may share a GitHub identity, the verdict may be a PR comment rather than
formal approval.

If changes are requested, make only those changes on this branch, push again,
and report back. Merge requires explicit operator authorisation.

- **Requested changes:** none yet.
- **Closeout refs:**
  `docs/roadmaps/g15/044-gpui-offscreen-capture-feasibility.md`,
  `docs/roadmaps/g15/012-visual-conformance-lane.md`, the g15.044 research note
  and August log, `docs/roadmaps/g15/release-gap-register.md` when changed,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`, and
  `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns the bounded proof, promoted research evidence, and batch log.
The orchestrator owns the verdict review, merge, card/roadmap status, and any
promotion of `g15.045`. Leave the lane open if the result is conditional,
non-reproducible, or missing an immutable upstream reference.
