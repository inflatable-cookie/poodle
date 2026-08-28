---
title: g16.019 Select mounted overlay parity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260828-181625-g16-019-select-overlay-worker.md
base_required: pushed-main
base_commit: b0703fc5d1aae065913ffaf7dc0ed8c4c900f4cd
tags: [coordination, handoff, worker, pr, select, gpui, overlay]
---

## What This Thread Was Doing

Execute ready card `g16.019`. Finish Select's real mounted GPUI behavior on the
semantic machine and required instance-scope surface merged in PR #93.

The work has three connected parts: make the native search row genuinely
editable, route pointer/keyboard/focus/dismissal through the shared Select
transition and host rebuilds, and repair the deferred-overlay pointer seam that
currently forces Pagination's Select proof through a test-only keyboard
workaround.

Start from this file. No copied transcript or second prompt is required.

## Why It Matters

Select is a foundation for many Poodle composites. Its state and interfaces are
now coherent across TypeScript and Rust, but GPUI still draws searchable input
as static text and cannot prove real pointer selection inside the deferred
overlay. Leaving that split would make the new semantic substrate theoretical
and keep every Select consumer on an incomplete native interaction base.

This card closes the production path and moves exactly Select from 46 to
47 mounted components. It does not claim that Pagination or another composite
gains new parity merely because it consumes Select.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `b0703fc5d1aae065913ffaf7dc0ed8c4c900f4cd`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled
  the planning base before this handoff was created
- **Planning checkout:** clean at the planning base
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** merged g16.018 closeout,
  recompiled g16.019 with cleared entry gate, updated g16/front doors, and the
  resolved two-card Select decision
- **Worker branch:** `t3code/g16-019-select-mounted-overlay`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-019-select-mounted-overlay`
- **Worktree creation command:** `git worktree add -b t3code/g16-019-select-mounted-overlay /Users/tom/.t3/worktrees/poodle/g16-019-select-mounted-overlay origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and do
  not create another worktree for a naming mismatch. If the current context is
  unusable, inspect the named worktree; only then read `.agents.local.env` and
  require `AGENTS_WORKTREE_CONTAINER_DIR`. Ask the operator if that local path
  contract is absent; never use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** `docs/contracts/components/select.md`
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g16/019-select-mounted-overlay-parity.md`
- **Source decision:**
  `docs/triage/20260828-085200-post-g16-017-native-lane-decision.md`
- **Landed substrate evidence:**
  `docs/logs/2026-08/20260828-g16-018-select-semantic-machine-and-interface-convergence.md`
- **Allowed runway:** execute `g16.019` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial
- **Parallel safety check:** Select rendering, the GPUI layer/hit-test seam,
  mounted regression, Pagination workaround, specimen state, and one ledger
  cell share mutable native surfaces. Do not run a parallel worker in this lane.
- **Current ledger:** 46 mounted / 128 missing and 115 known-delta present /
  60 not-applicable
- **Canonical refs:** `AGENTS.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/003-component-docs-ia-and-implementation-substrates.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`, the Select contract, source decision,
  g16.018 execution log, g16.019 card, and parity evidence ledger
- **Model capability profile:** frontier-capable coding model with high
  reasoning. This card touches native text/focus and a reusable overlay
  hit-testing seam. Stop if the bounded repair cannot stay within the card.
- **Tool/runtime restrictions:** use repo-local Effigy selectors; everything is
  headless; never run `*-windowed`, native visual, Jetstream preview/QA,
  release, tag, publication, or workflow-mutation tasks
- **Required validation:** focused Select/web/Rust/backend tests, named mounted
  Select and Pagination regressions, native specimen/regression boards,
  relevant drift and ledger checks, `ci:rust`, `ci:native`, `ci:web`,
  `docs:check`, one final `qa`, and diff check
- **PR base/head:** `main` <- worker branch
- **PR URL:** pending
- **Review state:** awaiting worker implementation and orchestrator review
- **Merge authorisation:** worker must not merge; explicit operator
  authorization follows orchestrator review

## Boundaries

- **In scope:** real native searchable editing through existing Node channels;
  Select query/highlight/value/open transitions through the landed machine;
  trigger/editor focus and keyboard behavior; option hover/activation; clear;
  inside/outside dismissal; a focused deferred-overlay pointer reproducer and
  the smallest reusable backend repair; removal of Pagination's Select
  workaround; curated GPUI specimen state; one named mounted Select regression;
  one Select ledger-cell move; and required closeout.
- **Out of scope:** a new Select machine or public web API, behavioral closure
  for composed components, generic input vocabulary, broad overlay redesign,
  menu/popover/dialog migration, broad accessibility or visual comparison,
  Jetstream admission, NumberInput, EditableLabel, audio, motion, Longhorn,
  releases, versions, workflows, downstreams, and sibling repositories.
- Keep `SelectSpec` host-owned. A production event emits one complete
  `SelectTransitionResult`; the host applies it and rebuilds. Do not add hidden
  renderer state.
- Keep the required non-empty `SelectHandlers::new(instance_scope)` surface and
  the composed required scopes exactly as landed. No `Default`, optional ids,
  fallback identity, aliases, or shims.
- Build the compact search editor from existing Node input/caret/edit/submit/
  cancel/focus channels. Do not nest a second fully styled TextInput shell.
- Searchable focus stays on the editor while highlight moves. Non-searchable
  focus stays on the trigger. Options are pointer targets, not separate tab
  stops.
- Reproduce the deferred option-row miss before changing the backend. Keep the
  repair at the smallest reusable layer/hit-test seam and preserve paint order,
  layer containment, outside-dismiss ordering, and nested overlays.
- Do not special-case Select coordinates, duplicate handlers outside the node
  tree, keep the panel permanently in-flow, or retain test-only option rings or
  ids as mounted evidence.
- Keep Examples curated and human-facing. Focused tests, not specimens, are the
  behavior corpus.
- Work only in the selected clean worker worktree. Never edit, reset, clean, or
  stash the orchestrator checkout or another worktree.
- Do not merge the PR.

## Important Context

- **Planning lineage:** g16.018 merged in PR #93 and established equivalent
  TypeScript/Rust `SelectContext`, events, effects, and atomic transition
  results. Svelte and React now consume that machine. Typing reports query;
  value changes only on option selection, clear, or explicit allowed freeform
  commit.
- **Why this card is ready:** the operator-approved semantics and breaking Rust
  interface are landed. The orchestrator verified the exact API, paired web
  timing, composed required scopes, existing Node editing channels, and the
  still-present deferred pointer defect. The card fixes scope, acceptance,
  writable files, validation, and stop conditions.
- The native renderer already assigns stable runtime ids to trigger, clear,
  search, listbox, and options. Its search row is still static text.
- Existing Node/GPUI channels already support replacement text, edit keys,
  insertion, selection, submit, cancel, focus change, caret painting,
  navigation keys, focus requests, and document-level dismissal. Reuse them.
- The Pagination mounted regression starts its Select open and stamps a
  test-only focus ring/runtime target because pointer input misses deferred
  rows. Remove that workaround only after real pointer selection passes.
- The ledger must change only Select: 46 → 47 mounted and 128 → 127 missing.
  Known-delta totals stay 115 / 60; visual and broad accessibility evidence do
  not move.
- `effigy doctor` has accepted baseline scan findings recorded in
  `PAPERCUTS.md`. Record the baseline and keep unrelated cleanup out.
- **Open tensions:** editable focus and control-blur ordering need exact
  mounted proof; inside clicks must not race outside dismissal; the backend
  repair must stay generic but narrowly scoped. Bring any need for new Node
  vocabulary or a broader overlay redesign back to the orchestrator.
- **Report after:** editable Select and focused renderer proof; overlay
  reproducer/repair plus Pagination cleanup; mounted regression/ledger/closeout
- **Report to:** the operator, who relays progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, then run
the quick startup worktree-safety preflight below before broad repository
reads. Use the launcher-provided clean non-`main` worktree when present; do not
compare its generated name with these placeholders or create another worktree.

Start by reading the landed Select machine and renderer beside TextInput's
existing Node editing composition. Add focused renderer tests for a real search
field and exact event mapping before touching the backend. Then add a small
headless reproducer for pointer activation of a deferred option row, repair the
layer/hit-test seam, and prove existing nested dismissal behavior remains
unchanged. Finish by replacing the Pagination workaround with real pointer
proof, adding the complete mounted Select regression, and moving the single
ledger cell.

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
   actual root/branch and do not create another worktree merely because it
   differs from the named placeholders.
3. If the launcher supplied a dirty or `main` worktree, stop and report it.
   Only when the current context is otherwise unusable may you inspect the
   named worktree, then read `.agents.local.env` and require
   `AGENTS_WORKTREE_CONTAINER_DIR` for a unique manual fallback from
   `origin/main`. Ask the operator when absent. Never use `/tmp`, `TMPDIR`,
   or a guessed path; never clean, reset, stash-over, or discard another
   checkout.
4. From the selected worktree, run `git fetch origin`; confirm `HEAD` equals
   `origin/main`; confirm
   `git merge-base --is-ancestor b0703fc5d1aae065913ffaf7dc0ed8c4c900f4cd HEAD`;
   and confirm this handoff and ready card exist in `HEAD`.
5. Read `AGENTS.md`, the repo-local Northstar and Effigy skills, g16 README,
   g16.019, Select contract, source decision, g16.018 log, ledger, landed
   TypeScript/Rust machine and tests, Svelte/React preservation tests, Rust
   spec/render tests, TextInput's Node editing composition, GPUI layer/input
   backend and focused tests, Select/Pagination specimens, and mounted
   regressions.
6. Run `effigy tasks` and `effigy doctor`. Record existing baseline findings;
   do not widen into cleanup.

### While you work

- Execute only `g16.019` in its three meaningful batches. Keep commits aligned
  with coherent chunks, not model turns.
- Establish editable Select behavior and focused renderer tests before changing
  the generic backend.
- Reproduce the deferred pointer failure before repairing it. Keep focused
  backend tests around hit-testing, dismissal, and nested overlays.
- Use production pointer/keyboard/focus paths plus host rebuilds. Do not
  substitute direct callback invocation or test-only stamps.
- Keep the specimen curated and move only the Select ledger cell.
- Report after each meaningful chunk with changed files, validation, remaining
  work, new risks, and blockers.
- Stop if contract intent changes, new generic Node vocabulary is required, the
  overlay repair becomes broad, another component must change behavior, or
  validation changes the plan.

### When the assigned runway is complete

1. Run the card's required final validation, entirely headlessly:
   - focused `poodle-headless`, `poodle-specs`, and `poodle-render` Select
     tests;
   - focused Svelte and React Select preservation tests;
   - focused GPUI node-backend layer/hit-test tests and the named mounted Select
     regression;
   - the Pagination mounted regression after removing its workaround;
   - `effigy regressions:native` and `effigy probe:gpui-specimens`;
   - relevant handler/event, contract/spec, machine-shape, and role drift
     selectors when their prerequisites are available without admitting
     Jetstream;
   - `effigy test:parity-evidence-ledger` and
     `effigy check:parity-evidence-ledger`, proving 47 / 127 and 115 / 60;
   - `effigy ci:rust`, `effigy ci:native`, `effigy ci:web`,
     `effigy docs:check`, one final `effigy qa`, and
     `git diff --check origin/main...HEAD`.
2. Mark `g16.019` complete. Add one August execution log, update g16 and the
   roadmap front doors, and leave the next task as an orchestrator checkpoint.
   Do not choose or compile another lane.
3. Confirm only Select's mounted cell moved: 47 mounted / 127 missing;
   known-delta totals remain 115 / 60 and visual/accessibility totals remain
   unchanged.
4. Push the selected worker branch and open a reviewable PR against current
   `main`. The planning base above predates this handoff commit; it is not a
   self-referential hash.
5. In the PR body, link the card, contract, source decision, g16.018 log,
   changed surfaces, editable path, key/focus/dismiss mapping, overlay
   reproducer and repair, removed Pagination workaround, mounted regression,
   exact ledger totals, validation, and non-claims.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator independently inspects PR metadata, commits, changed files,
editable event mapping, host ownership, focus/dismiss ordering, bounded backend
repair, removal of test-only stamps, mounted evidence, one-cell ledger change,
closeout, and checks. Because worker and orchestrator share one GitHub identity,
the orchestrator posts the canonical verdict as a PR comment rather than formal
self-approval. If changes are requested, make only those changes on this
branch, push, and report through the operator. The operator must explicitly
authorize any merge.

- **Requested changes:** none yet
- **Closeout refs:** `g16.019`, source decision, one August execution log,
  g16 README, generation index, roadmap front door, and parity evidence ledger

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the blocker and stop rather than
making the handoff look more complete than it is.
