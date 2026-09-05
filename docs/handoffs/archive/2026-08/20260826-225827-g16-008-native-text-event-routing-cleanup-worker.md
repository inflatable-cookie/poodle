---
title: g16.008 native text event routing cleanup worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260826-225827-g16-008-native-text-event-routing-cleanup-worker.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, gpui, text-input, focus]
---

## What This Thread Is Doing

Execute ready card `g16.008`. Repair two generic GPUI text-event defects found
while closing TextInput mounted evidence: Tab is incorrectly routed through
the submit channel, and blur clears transient text state under the focused root
id even when a composite field paints that state under a derived value-node id.

This is a bounded substrate repair. It must preserve the contracts of
TextInput, CodeInput, DurationInput, and EditableLabel while moving no parity
ledger cell. Start from this file; no copied transcript or second prompt is
required.

## Why It Matters

The next native editable-control lane would inherit these defects. Fixing them
once in the node/backend seam prevents later component proofs from certifying
incorrect focus behavior or carrying stale measurement/composition state.
EditableLabel is the important counterexample: it does commit on Tab, but the
contract says that happens because Tab moves focus and blur commits, not because
the backend treats Tab as generic submit.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `444806719ecc44889531929bedcfc69e1ec9ee0e`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled the
  planning base before this handoff was created
- **Planning checkout:** clean at the recorded base
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** ready `g16.008` card, updated
  g16/front-door runway, and promoted TextInput cleanup triage
- **Worker branch:** `t3code/g16-008-native-text-event-routing-cleanup`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-008-native-text-event-routing-cleanup`
- **Worktree creation command:** `git worktree add -b t3code/g16-008-native-text-event-routing-cleanup /Users/tom/.t3/worktrees/poodle/g16-008-native-text-event-routing-cleanup origin/main`
- **Worker worktree policy:** use a clean launcher-provided non-`main`
  registered worktree first, whatever its generated name. Only fall back to
  the named path and then `.agents.local.env` when the current context is
  unusable. Never create a second worktree merely because names differ.
- **Ready card:**
  `docs/roadmaps/g16/008-native-text-event-routing-cleanup.md`
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Allowed runway:** execute `g16.008` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial; node vocabulary, GPUI dispatch/state identity,
  EditableLabel blur behavior, and mounted proofs overlap directly
- **Current ledger invariant:** 37 mounted / 137 missing; no row moves
- **Related but out-of-scope triage:**
  `docs/triage/20260826-213343-number-input-native-value-model.md`
- **Canonical refs:** `AGENTS.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`, and the TextInput, CodeInput,
  DurationInput, and EditableLabel component contracts
- **Model capability profile:** capable coding model, high reasoning; real
  focus traversal, blur ordering, and root/value cache identity need careful
  mounted verification
- **Tool/runtime restrictions:** use the repo-local Effigy skill and selectors;
  everything stays headless; never run `*-windowed`, native visual, Jetstream
  preview/QA, release, tag, or publication tasks
- **Required validation:** focused node/render/backend tests; mounted routing
  tests for all four affected controls; retained composite text-entry
  regressions; `effigy regressions:native`; `effigy probe:gpui-specimens`;
  parity-ledger test/check with no generated diff; `effigy ci:native`;
  `effigy docs:check`; one final `effigy qa`; and
  `git diff --check origin/main...HEAD`
- **Known orientation finding:** `effigy doctor` is already red on the planning
  base from generated-in-src, oversized-file, and stale/broad suppression scans
  recorded in `PAPERCUTS.md`; report that baseline without absorbing cleanup
- **Planning validation:** `effigy docs:check` and `git diff --check` passed;
  existing Svelte build warnings remain non-failing baseline output
- **PR base/head:** `main` <- worker branch
- **PR URL:** pending
- **Review state:** awaiting worker implementation and orchestrator review
- **Merge authorization:** worker must not merge; explicit operator authority
  is required after orchestrator review

## Boundaries

- Treat generic `Interaction::on_submit` as Enter submission only. Tab and
  Shift+Tab belong to real sequential focus traversal.
- Do not replace traversal with direct focus-handler calls or a test-only focus
  path.
- Preserve TextInput Enter submit and Escape cancel.
- Preserve CodeInput value/completion while Tab leaves the control.
- Preserve DurationInput segment ordering and value while Tab/Shift+Tab move
  through its stops and then out.
- Preserve EditableLabel Enter commit and Escape cancel. Its Tab commit must
  happen exactly once through observed blur before focus advances.
- Centralize painted text-state key selection inside the backend. Childless
  inputs paint under their root id; composite TextInput paints under its
  derived value child.
- Clear transient measured/scroll/blink/marked/composing state on blur. Do not
  clear mounted-lifetime undo history just because focus moved.
- If the direct-input undo key is proven mismatched, repair it through the same
  identity helper and retain focused undo/redo evidence.
- Use explicit ids in mounted fixtures. Prove independent fields do not share
  state.
- Do not change public component props or move any ledger evidence.
- Keep NumberInput, multiline, slug, broad IME, accessibility, visual,
  Jetstream, release, and downstream work out of scope.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or clean/reset an unrelated checkout.
- Do not merge the PR.

## Important Context

- The current node comment says submit means Enter or Tab. Correct the comment
  and implementation together; do not leave the renderer-neutral vocabulary
  lying about the backend.
- `interaction.rs` currently checks `matches!(key, "enter" | "tab")` before
  forwarding edit keys.
- Focus is held by the field root. Composite TextInput's measured line, scroll,
  blink, marked range, composition, and paint history live under
  `<field-id>-value`; childless input paint uses the root id.
- `input_text::forget` intentionally does not clear undo history. Preserve that
  mounted-lifetime behavior.
- EditableLabel currently puts its commit handler on `on_submit`. Add only the
  smallest blur observation needed to satisfy its existing contract; do not
  redesign its public event API.
- The ledger is generated authority. Run its checks, but a regenerated content
  change is a card failure rather than expected closeout.
- Report after the generic routing/state-key repair, then after mounted
  cross-control proof and docs closeout. Report immediately on any stop
  condition.
- Report to the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Run the worktree preflight below. Then read the ready card, four component
contracts, node `Interaction`, GPUI `interaction.rs` and `input_text.rs`, the
four shared Rust renderers, existing backend tests, mounted driver helpers, and
the `g16.007` TextInput regression.

Implement in two meaningful chunks. First correct the node meaning, backend key
routing, painted-state identity, and focused tests. Then preserve each
component's behavior through real mounted focus/key dispatch, retain composite
regressions, and close the docs. Stop rather than widening if real Tab
traversal requires a new focus architecture.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before any
   broad repository read, run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root and branch. Do not create another worktree because generated
   names differ.
3. Only if the current context is `main`, dirty, unregistered, or unusable,
   inspect the named worktree. If it cannot be used, read `.agents.local.env`,
   require the absolute `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique
   worktree/branch there from `origin/main`. Ask the operator if the key is
   absent. Never use `/tmp`, `TMPDIR`, a repository child, or a guessed path.
   Never clean, reset, stash, or discard the original checkout.
4. From the selected worktree, run `git fetch origin`; confirm `HEAD` equals
   `origin/main`; confirm
   `git merge-base --is-ancestor 444806719ecc44889531929bedcfc69e1ec9ee0e HEAD`;
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, the g16 README, assigned card, four component contracts,
   ledger, and canonical architecture/working-rule refs.
6. Read `.agents/skills/effigy/SKILL.md`, then use `effigy tasks` and
   `effigy doctor` for orientation. Record the known doctor baseline without
   widening into unrelated cleanup.

### While you work

- Execute only `g16.008`. Keep commits aligned with the two meaningful chunks,
  not model turns.
- Mounted tests must drive production key dispatch and real focus handles. A
  direct handler invocation does not prove this card.
- Check event order explicitly where EditableLabel blur commits and focus
  advances.
- Keep the generated ledger untouched and preserve retained TextInput and
  composite text-entry regressions.
- Stop on any card stop condition. Do not invent a focus manager, native
  editor, public compatibility layer, or broader component migration.

### When the assigned runway is complete

1. Run the full validation named in `Current State`, entirely headlessly.
2. Mark the card complete, mark the source triage resolved, add one August
   execution log, and leave g16's next task as an orchestrator checkpoint. Do
   not compile or implement another card.
3. Confirm the ledger has no diff. Run
   `git diff --check origin/main...HEAD` and confirm the worktree is clean after
   committing.
4. Push the worker branch and open a reviewable PR against current `main`.
5. The PR body must link the card and four contracts; name focused and mounted
   tests; report Enter/Tab/Shift+Tab/blur ordering, painted-state identity,
   history retention, retained regressions, invariant ledger totals,
   validation, and explicit non-claims.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect PR metadata, commits, changed files, generic key
routing, real focus traversal, EditableLabel blur ordering, root/value cache
identity, mounted proofs, unchanged ledger, and checks independently. Because
worker and orchestrator share the GitHub identity, the orchestrator will post
the canonical verdict as a PR comment rather than formal self-approval. The
operator must explicitly authorize merge after a green review.

### Handoff closeout

Leave the card, execution log, triage disposition, roadmap/front doors, and PR
body mutually consistent. Record exact failures rather than claiming broad
native text parity. End at the orchestrator checkpoint.
