---
title: g16.005 Slider axis, keyboard, and mounted parity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260826-173954-g16-005-slider-axis-keyboard-mounted-parity-worker.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, parity, slider, web, gpui]
---

## What This Thread Was Doing

The orchestrator re-measured the post-`g16.004` ledger and inspected the next
foundation controls rather than extending the previous selection batch by
habit. Tabs was considered first and stopped on an unresolved native
drag-payload lifecycle. Slider is ready: its shared value machines and GPUI
scrub backend exist, but the native renderer drops commit on the active scrub
path, lacks keyboard and slider-value intent, and ignores orientation for
layout and pointer normalization.

Those decisions are promoted into the Slider contract and compiled as one
bounded implementation card. Start from this file; no copied transcript or
second prompt is required.

## Why It Matters

Slider is a foundation primitive used directly and inside dense audio and
editing controls. Closing its axis, callback, keyboard, focus, and value-intent
seams gives consumers dependable behavior and proves a reusable normalized
scrub boundary without reviving a generated conformance plane.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `09801720af14d8dc86691a4d9162264f5dd7f36e`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled the
  planning base before this handoff was created
- **Planning checkout:** clean at the recorded base
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** promoted Slider contract,
  `docs/roadmaps/g16/005-slider-axis-keyboard-and-mounted-parity.md`, open Tabs
  lifecycle triage, and updated g16/front-door runway
- **Worker branch:** `t3code/g16-005-slider-axis-keyboard-mounted-parity`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-005-slider-axis-keyboard-mounted-parity`
- **Worktree creation command:** `git worktree add -b t3code/g16-005-slider-axis-keyboard-mounted-parity /Users/tom/.t3/worktrees/poodle/g16-005-slider-axis-keyboard-mounted-parity origin/main`
- **Worker worktree policy:** use a clean launcher-provided non-`main`
  registered worktree first, whatever its generated name. Only fall back to
  the named path and then `.agents.local.env` when the current context is
  unusable. Never create a second worktree merely because names differ.
- **Active contract:** `docs/contracts/components/slider.md`; axis, arrow,
  callback, node-intent, and Jetstream-deferral decisions are promoted
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready card:**
  `docs/roadmaps/g16/005-slider-axis-keyboard-and-mounted-parity.md`
- **Allowed runway:** execute `g16.005` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial; node/backend axis, web proof, shared Rust
  repair, mounted regression, and ledger close share one semantic seam
- **Parallel safety check:** the chunks overlap Slider machines/tests, node
  vocabulary, GPUI backend input, the mounted regression file, and ledger;
  do not split them across worktrees
- **Canonical refs:** `AGENTS.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`, and
  `docs/contracts/components/slider.md`
- **Model capability profile:** capable coding model, high reasoning; stop on
  any card stop condition instead of widening the substrate
- **Tool/runtime restrictions:** use the repo-local Effigy skill and selectors;
  everything stays headless; never run `*-windowed`, native visual, Jetstream
  preview/QA, release, tag, or publication tasks
- **Required validation:** focused TS/Rust machine, Svelte, React, node,
  renderer, GPUI backend and changed-caller tests; retained RangeSlider scrub
  proof; `effigy regressions:native`; `effigy probe:gpui-specimens`;
  `effigy test:parity-evidence-ledger`;
  `effigy check:parity-evidence-ledger`; `effigy ci:native`;
  `effigy ci:web`; `effigy docs:check`; one final `effigy qa`; and
  `git diff --check origin/main...HEAD`
- **Known orientation finding:** `effigy doctor` is already red on the planning
  base from the open generated-in-src, oversized-file, and stale/broad
  suppression scans recorded in `PAPERCUTS.md`; report the baseline without
  absorbing unrelated cleanup
- **Planning validation:** `effigy docs:check` green on the planning base;
  existing Svelte build warnings remain non-failing baseline output
- **PR base/head:** `main` <- worker branch
- **PR URL:** pending
- **Review state:** awaiting worker implementation and orchestrator review
- **Merge authorisation:** worker must not merge; explicit operator authority
  is required after orchestrator review

## Boundaries

Keep the run inside Slider semantic and mounted parity.

- **In scope:** existing Slider machine proof; focused Svelte/React behavior
  tests; explicit node scrub axis; axis-aware GPUI capture; node value-text
  intent; at most symmetric percentage-height vocabulary; shared Rust handler,
  keyboard, focus, accessibility-intent and orientation repair; smallest
  RangeSlider axis retention adaptation; mechanical callers; mounted GPUI
  proof; exact ledger regeneration; one log.
- **Out of scope:** Tabs, other component semantics, a generic gesture/layout
  architecture, page-key standardization, specimen redesign, visual fixtures
  or thresholds, accessibility promotion, Jetstream backend admission,
  workflows, versions, releases, and downstream repositories.
- Keep `slider_transition` and `slider_control_transition` as the only value
  authority. Components and hosts must not reproduce snap/clamp math.
- Pointer value is an axis-normalized position. Do not restore the stale
  fixed-track delta fallback or expose raw layout coordinates to components.
- Every node carrying `on_scrub` sets its axis explicitly. Horizontal is
  left-to-right; vertical is bottom-to-top.
- All arrow keys retain the promoted mapping in both orientations. Page keys
  remain browser-owned and outside strict parity.
- Node role/value/bounds/value-text assertions are node-level intent, not proof
  of broad native assistive-technology behavior. Keep the ledger manual.
- Rename `SliderHandlers` fields without aliases. Pre-1.0 code does not retain
  compatibility shims.
- Deferred Jetstream call sites may change only enough to compile against the
  shared-render signature. Do not run or claim its backend.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or clean/reset an unrelated checkout.
- Do not merge the PR.

## Important Context

- **Planning lineage:** `g16.001` measured 174 portable native components and
  29 mounted GPUI cells. `g16.002`–`g16.004` moved five selection controls to
  34 mounted / 140 missing. This card moves only Slider to 35 / 139.
- **Why the card is ready:** the current TS and Rust headless machines agree;
  Svelte and React already share the core; GPUI already has captured normalized
  scrub input proven by RangeSlider; node keys, role, value bounds, focus
  treatment, and mounted driver patterns exist. The missing axis and value-text
  fields are small additive vocabulary explicitly authorized by the card.
- **Current defect:** when `on_change` exists, shared Rust installs
  `on_scrub` but forwards only change effects, so `on_value_commit` never fires
  on release. Keyboard handling and Slider role/value/bounds/orientation are
  absent. The backend always normalizes X, and renderer geometry is horizontal
  even when `SliderSpec::orientation` is vertical.
- **Decisions and preferences:** specimens remain human-centred; public web
  props stay unchanged; standard web retains native range behavior; custom and
  native controls share normalized position semantics; callback fields align
  with RangeSlider; Jetstream stays deferred.
- **Open tension:** native vertical geometry may need `NodeStyle.height_pct`.
  The card authorizes only that symmetric channel. Stop if faithful layout
  needs a broader node/layout rewrite.
- **Tabs checkpoint:**
  `docs/triage/20260826-173329-tabs-native-drag-lifecycle.md` is not part of
  this implementation. Do not solve it opportunistically.
- **Report after:** first the machine/web and node/backend axis batch, then the
  shared Rust/mounted/ledger batch; report earlier on any stop condition
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Read this handoff and run the worktree preflight below before broad reads. Then
read the promoted Slider contract and card, existing TS/Rust machines, both web
components/tests, shared Slider and RangeSlider renderers, GPUI scrub backend,
Slider specimen facade, and the retained RangeSlider mounted regression.

Implement in two meaningful chunks. First lock machine/web cases and add the
explicit axis/value-text substrate with focused backend tests. Then repair the
shared Rust control, migrate callers, drive the mounted GPUI path, regenerate
the ledger, and close the docs. Do not turn the symmetric node additions into
a general event or layout redesign.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before any
   broad repository read, run:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root and branch. Do not compare generated names with this handoff or
   create another worktree because they differ.
3. Only if the current context is `main`, dirty, unregistered, or unusable,
   inspect the named worktree. If that cannot be used, read
   `.agents.local.env`, require the absolute `AGENTS_WORKTREE_CONTAINER_DIR`,
   and create a unique worktree/branch there from `origin/main`. Ask the
   operator if the key is absent. Never use `/tmp`, `TMPDIR`, a repository
   child, or a guessed path. Never clean, reset, stash, or discard the original
   checkout. If the launcher itself supplied a dirty or `main` worktree, stop
   and report it instead of silently creating another.
4. From the selected worktree, run `git fetch origin`; confirm `HEAD` equals
   `origin/main`; confirm
   `git merge-base --is-ancestor 09801720af14d8dc86691a4d9162264f5dd7f36e HEAD`;
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `docs/roadmaps/g16/README.md`, the assigned card, Slider
   contract, and canonical architecture/working-rule refs.
6. Read `.agents/skills/effigy/SKILL.md`, then use `effigy tasks` and
   `effigy doctor` for orientation. Record the known doctor baseline without
   widening into unrelated cleanup.

### While you work

- Execute only `g16.005`. Keep commits aligned with the two meaningful chunks,
  not model turns.
- Add focused direct machine/handler tests, but use mounted backend input and
  host rebuilds for the ledger claim.
- Preserve the existing RangeSlider mounted scrub regression after making its
  axis explicit. Do not change its evidence status.
- After each chunk, report changed files, validation actually run, remaining
  acceptance, defects, and blockers through the operator.
- Stop on any card stop condition. Do not invent generic substrate, alter
  unrelated public APIs, absorb Tabs, or admit Jetstream.

### When the assigned runway is complete

1. Run the full validation named in `Current State`, entirely headlessly.
2. Mark the card complete, regenerate the ledger through its source, add the
   August execution log, and leave g16's next task as an orchestrator review
   checkpoint. Do not compile or implement another card.
3. Run `git diff --check origin/main...HEAD` and confirm the worktree is clean
   after committing.
4. Push the worker branch and open a reviewable PR against current `main`.
5. The PR body must link the g16 milestone/card and promoted Slider contract;
   name focused and mounted tests; report node vocabulary and handler changes;
   show horizontal/vertical, pointer/keyboard, disabled and rebuild evidence;
   give ledger before/after counts; list validation; and preserve unresolved
   accessibility/visual gaps.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect PR metadata, commits, changed files, diff, tests,
API migration, ledger lineage, and checks independently. Because worker and
orchestrator share the GitHub identity, the orchestrator will post the
canonical verdict as a PR comment rather than formal self-approval. Requested
changes are currently none. The operator must explicitly authorise merge after
a green review.

- **Closeout refs:** assigned card, g16 README/front doors, generated parity
  ledger, one August log, and the single next-task checkpoint

### Handoff closeout

Before calling the runway complete, leave the card, log, ledger, roadmap, and
next-task state honest. If blocked, record the exact blocker and stop rather
than making the handoff appear complete.
