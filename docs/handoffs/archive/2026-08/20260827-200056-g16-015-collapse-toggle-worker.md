---
title: g16.015 CollapseToggle disclosure and mounted parity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260827-200056-g16-015-collapse-toggle-worker.md
base_commit: e63aa6dfb727143b5b9f61c7a68280f645b3b961
tags: [coordination, handoff, worker, pr, gpui, collapse-toggle]
---

## What This Thread Was Doing

Execute ready card `g16.015`. Align the standalone native CollapseToggle with
the existing Svelte/React disclosure-button authority, then prove pointer and
keyboard behavior through the real headless GPUI tree.

Start from this file. No copied transcript or second prompt is required.

## Why It Matters

CollapseToggle is a small foundation control used at panel and split-view
boundaries. The web runtimes and Rust spec agree, but the shared renderer
hardcodes the wrong default accessible label, omits expanded state, leaves
disabled controls focusable, and declares no structured focus ring. The
existing DockRegion mounted test drives a different internal collapse button,
so it does not prove this component.

This lane fixes that concrete seam without changing the public API, adding a
state machine, or opening the larger Select, EditableLabel, visual,
accessibility, or Jetstream programmes.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `e63aa6dfb727143b5b9f61c7a68280f645b3b961`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled the
  planning base before this handoff was created
- **Planning checkout:** clean at the planning base
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** resolved lane decision, ready
  `g16.015` card, and updated g16/front doors
- **Worker branch:** `t3code/g16-015-collapse-toggle-mounted-parity`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-015-collapse-toggle-mounted-parity`
- **Worktree creation command:** `git worktree add -b t3code/g16-015-collapse-toggle-mounted-parity /Users/tom/.t3/worktrees/poodle/g16-015-collapse-toggle-mounted-parity origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and do
  not create another worktree for a naming mismatch. If the current context is
  unusable, inspect the named worktree; only then use `.agents.local.env` and
  its required `AGENTS_WORKTREE_CONTAINER_DIR`. Ask the operator if that local
  path contract is absent; never use `/tmp`, `TMPDIR`, or a guessed path.
- **Active contract:** `docs/contracts/components/collapse-toggle.md`
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready card:**
  `docs/roadmaps/g16/015-collapse-toggle-disclosure-and-mounted-parity.md`
- **Source decision:**
  `docs/triage/20260827-195632-post-g16-014-native-lane-decision.md`
- **Allowed runway:** execute `g16.015` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial
- **Parallel safety check:** renderer semantics, GPUI mounted proof, generated
  ledger, and closeout surfaces are one shared mutable lane
- **Current ledger:** 43 mounted / 131 missing; only CollapseToggle may move to
  44 mounted / 130 missing; known-delta totals stay 115 / 60
- **Canonical refs:** `AGENTS.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`, the component contract, source
  decision, card, and ledger
- **Model capability profile:** capable coding model, medium reasoning; stop for
  frontier/orchestrator review if public API, composite ownership, or generic
  backend changes become necessary
- **Tool/runtime restrictions:** use repo-local Effigy selectors; everything is
  headless; never run `*-windowed`, native visual, Jetstream preview/QA,
  release, tag, publication, or workflow-mutation tasks
- **Required validation:** the card's focused Rust/web tests, native regression
  and specimen boards, relevant drift checks, ledger checks, `ci:rust`,
  `ci:native`, `ci:web`, `docs:check`, final `qa`, and diff check
- **PR base/head:** `main` <- worker branch
- **PR URL:** pending
- **Review state:** awaiting worker implementation and orchestrator review
- **Merge authorisation:** worker must not merge; explicit operator
  authorization follows orchestrator review

## Boundaries

- **In scope:** CollapseToggle effective/default labels, expanded state,
  enabled focus/tab/ring declaration, disabled focus and activation suppression,
  existing next-state callback and direction mapping, curated GPUI specimen
  preservation, one standalone mounted behavior proof, one ledger cell, and
  required closeout.
- **Out of scope:** public Svelte/React or Rust spec API changes, hidden state or
  a new machine, region ownership/ids/controls, generic node/backend changes,
  DockRegion/SplitView semantic rewrites, visual token repairs, GPUI visual
  comparison, broad native accessibility, Select, EditableLabel, NumberInput,
  Rating, overlays, Jetstream admission, releases, versions, workflows,
  downstreams, and siblings.
- Keep the callback prop-driven: accepted activation reports
  `!spec.is_collapsed`; the host rebuilds the spec. Repeated activation without
  rebuild may repeat the same next value, matching both web runtimes.
- Use `CollapseToggleSpec::effective_aria_label()` and the existing direction,
  size, density, and focus-token helpers. Do not duplicate policy in GPUI.
- The mounted test may assign one fixture id only for driver targeting. Do not
  invent a public identity contract.
- Do not turn the specimen into an exhaustive conformance matrix.
- This handoff represents one worker lane. If shared mutable scope or a hidden
  dependency outside the card appears, stop and report it through the operator.
- Work only in the selected clean worker worktree. Never edit, reset, clean, or
  stash the orchestrator checkout or another worktree.
- Do not merge the PR.

## Important Context

- **Planning lineage:** g16's evidence ledger reached 43 mounted / 131 missing
  after Accordion merged in PR #88. The orchestrator selected CollapseToggle
  because its web and Rust spec authorities are already coherent and its native
  defect is bounded and observable.
- **Why the card is ready:** existing node/backend channels already express
  Button role, expanded state, disabled state, tab index, structured focus
  rings, pointer/keyboard activation, and host rebuild. No architecture or
  public API choice remains.
- Expanded means label `Collapse`, expanded true, and the authored direction.
  Collapsed means label `Expand`, expanded false, and the opposite direction.
  An explicit label overrides both defaults.
- Enabled controls are sequential Button stops with the contract's focus colour
  and width and a `0.0625rem` offset. Disabled controls have no activation,
  focus handle, tab stop, or ring.
- The existing GPUI Directions specimen already has host-owned state and stable
  ids. Preserve its Examples, Sizes, and Densities presentation.
- The existing DockRegion mounted regression uses DockRegion's separate
  hand-built collapse affordance. It cannot be relabelled as standalone
  CollapseToggle evidence.
- `effigy doctor` is red on the planning base from generated-in-src, god-file,
  and stale-suppression scans already recorded in `PAPERCUTS.md`. Record the
  baseline; do not absorb cleanup.
- A planning-time `effigy graph index --json` reached a ready index but kept its
  process alive beyond the initial command window; that friction is recorded in
  `PAPERCUTS.md` and is outside this card.
- `effigy drift:roles` may require the deferred Jetstream sibling. Do not create
  a symlink or bring Jetstream into normal QA; run it only when the sibling is
  already available, otherwise record the known blocker.
- **Open tensions:** radius/token visual deltas and broad native accessibility
  remain real evidence gaps, but this behavior card leaves those cells
  unchanged.
- **Report after:** focused renderer semantics are green; then after the mounted
  proof and closeout are complete
- **Report to:** the operator, who relays progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, then run
the quick startup worktree-safety preflight below before broad repository reads.
Use the launcher-provided clean non-`main` worktree when present; do not compare
its generated name with these placeholders or create another worktree.

Start with focused renderer tests that expose the current wrong label, missing
expanded state, absent ring/tab declaration, and disabled focus leak. Repair
those semantics in one coherent renderer batch while preserving callback and
icon behavior. Then add the named mounted regression through production
rendering and real pointer/keyboard dispatch. Finish with the one-cell ledger
move and closeout.

At each natural pause, tell the operator what changed, what validation actually
ran, what remains, and whether anything needs a planning decision.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before
   broad repository reads, run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch and do not compare it with the named placeholders or
   create another worktree merely because they differ.
3. If the launcher supplied a dirty or `main` worktree, stop and report it. Only
   when the current context is otherwise unusable may you inspect the named
   worktree, then read `.agents.local.env` and require
   `AGENTS_WORKTREE_CONTAINER_DIR` for a unique manual fallback from
   `origin/main`. Ask the operator when absent. Never use `/tmp`, `TMPDIR`, or a
   guessed path; never clean, reset, stash-over, or discard another checkout.
4. From the selected worktree, run `git fetch origin`; confirm `HEAD` equals
   `origin/main`; confirm
   `git merge-base --is-ancestor e63aa6dfb727143b5b9f61c7a68280f645b3b961 HEAD`;
   and confirm this handoff and the ready card exist in `HEAD`.
5. Read `AGENTS.md`, the repo-local Northstar and Effigy skills, g16 README,
   card, component contract, source decision, ledger, paired web
   implementations/tests, Rust spec/renderer/tests, GPUI wrapper/specimen,
   mounted driver, and current Button/IconButton/Collapsible focus-ring
   patterns.
6. Run `effigy tasks` and `effigy doctor`. Record the known doctor baseline; do
   not widen into cleanup.

### While you work

- Execute only `g16.015` in its three meaningful batches. Keep commits aligned
  with coherent chunks, not model turns.
- Establish focused renderer behavior before adding mounted evidence.
- Use existing production renderer/backend channels and host-owned rebuilds.
- Keep the fixture id a test targeting aid, not a new identity claim.
- Preserve the curated specimen and public APIs.
- Report after each meaningful chunk with changed files, validation, remaining
  work, new risks, and blockers.
- Stop if contract intent becomes ambiguous, generic backend or composite
  redesign is required, scope expands, or validation changes the plan.

### When the assigned runway is complete

1. Run the card's required final validation, entirely headlessly:
   - focused `poodle-specs` and `poodle-render` CollapseToggle tests;
   - focused Svelte and React CollapseToggle tests;
   - the named mounted CollapseToggle regression;
   - `effigy regressions:native`, `effigy probe:gpui-specimens`, relevant
     handler/event/role and contract/spec drift checks, ledger test/check,
     `effigy ci:rust`, `effigy ci:native`, `effigy ci:web`,
     `effigy docs:check`, one final `effigy qa`, and
     `git diff --check origin/main...HEAD`;
   - Jetstream-coupled checks only when the deferred sibling already exists.
2. Mark the card complete and source decision resolved. Add one August
   execution log and leave g16 at an orchestrator evidence checkpoint. Do not
   compile or start `g16.016`.
3. Confirm only CollapseToggle's mounted cell moves and totals are 44 mounted /
   130 missing; known-delta totals remain 115 / 60.
4. Push the selected worker branch and open a reviewable PR against current
   `main`. The planning base above predates this handoff commit; it is not a
   self-referential hash.
5. In the PR body, link the card, contract, source decision, changed surfaces,
   semantic/focus repair, mounted evidence, exact ledger delta, validation, and
   non-claims.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator independently inspects PR metadata, commits, changed files,
semantic labels/expanded state, focus and disabled behavior, callback/direction
preservation, specimen scope, mounted production proof, ledger delta, closeout,
and checks. Because worker and orchestrator share one GitHub identity, the
orchestrator posts the canonical verdict as a PR comment rather than formal
self-approval. If changes are requested, make only those changes on this
branch, push, and report through the operator. The operator must explicitly
authorize any merge.

- **Requested changes:** none yet
- **Closeout refs:** the g16.015 card, source decision, one August execution
  log, g16 README, generation/front-door status, and parity ledger

