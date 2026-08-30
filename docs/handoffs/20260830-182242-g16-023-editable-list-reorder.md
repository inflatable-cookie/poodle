---
title: g16.023 EditableList simple reorder migration worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: revision-authorised
owner: Poodle drag-and-drop substrate
created: 2026-08-30
updated: 2026-08-30
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260830-182242-g16-023-editable-list-reorder.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, drag-drop, editable-list, svelte, react]
---

## What This Thread Was Doing

The orchestrator merged the provider-scoped web drag substrate in PR #101 and
then caught a sequencing conflict before dispatching its first component
migration. Tabs' native drag callbacks still power DockRegion's external-drag
bridge, so Tabs now moves with that real consumer in `g16.026`.

This worker owns the corrected `g16.023`: migrate EditableList in Svelte and
React from component-local HTML drag-and-drop to the landed Poodle substrate.
PR #104 stopped correctly when review exposed that mounted-only keyboard
targets cannot preserve windowed reorder. The operator chose the orchestrator's
logical-target recommendation. This same worker may now revise PR #104 against
the promoted architecture/spec/card decision.

## Why It Matters

EditableList is the first production component proof that the new substrate is
usable beyond custom fixtures. It exercises flat reorder, row editing, action
buttons, disabled state, touch scrolling, keyboard movement, focus return, and
complete resulting-order callbacks without the unrelated Tabs/DockRegion host
bridge. A clean result establishes the migration pattern Tree can build on.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `1d1d0937c077f32a2491cec614a9f167b41796bd`
- **Planning base meaning:** current `main` before this revised handoff and its
  logical-target decision are committed. The worker must fetch and rebase onto
  the newer commit containing this tracked revision before continuing.
- **Planning checkout:** clean at the planning base before this revision.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the revision:** the logical keyboard target
  architecture/spec decision, revised card 023 review oracle, resolved
  Tabs/DockRegion sequencing note, and updated g16/front-door state.
- **Worker branch:** use the launcher-provided clean non-`main` branch;
  suggested manual fallback is `t3code/g16-023-editable-list-reorder`.
- **Worker worktree:** harness-managed. Do not create a second worktree when
  the launcher already supplied one.
- **Worktree creation command:** none on the normal launcher path. If manual
  fallback is genuinely required, parse `.agents.local.env`, require an
  absolute `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique worktree below
  it from `origin/main`. Never guess a path.
- **Required sibling worktree links:** none.
- **Active spec lane:**
  `docs/specs/069-dependable-drag-and-drop-substrate.md`.
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`.
- **Ready cards, in order:**
  `docs/roadmaps/g16/023-drag-drop-simple-reorder-migrations.md`.
- **Allowed runway:** card 023 only, then stop.
- **Remaining card budget:** one card.
- **Dispatch topology:** serial drag-and-drop component migration.
- **Parallel safety check:** no other worker may edit EditableList or the web
  drag controller/provider fixtures during this run. Tabs, Tree, native drag,
  and the host bridge remain later serial cards.
- **Canonical refs:**
  `docs/architecture/011-drag-and-drop-substrate.md`,
  `docs/specs/069-dependable-drag-and-drop-substrate.md`,
  `docs/contracts/components/editable-list.md`,
  `docs/contracts/001-working-rules.md`, and
  `docs/triage/20260830-180816-tabs-drag-host-bridge-sequencing.md`.
- **Model capability profile:** capable coding model with high reasoning for
  input lifetime, focus, editing-control arbitration, and public behavior.
- **Tool/runtime restrictions:** headless only. Never run `*-windowed`, native
  visual, GPUI/Jetstream preview, release, publication, tag, or workflow
  mutation selectors.
- **Required validation:** focused paired EditableList tests; web custom-surface
  preservation tests; headless Chromium/WebKit drag probes; relevant contract,
  callback, and capability checks; `effigy ci:web`; `effigy docs:check`;
  `effigy check:parity-evidence-ledger`; one final headless `effigy qa`; and
  `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` to the selected worker branch.
- **PR URL:** <https://github.com/inflatable-cookie/poodle/pull/104>.
- **Review state:** changes required; planning blocker resolved, implementation
  revision authorised for all four posted findings.
- **Merge authorisation:** none. The worker must not merge.

## Boundaries

Keep this run inside card 023:

- **In scope:** Svelte and React EditableList source/target registration;
  pointer, touch, and keyboard reorder; one resulting-order path; editing and
  action-button arbitration; disabled/non-reorderable inertia; focus and
  announcements; focused tests and specimens; removal of EditableList-local
  HTML drag state/handlers; the approved element-free ordered logical keyboard
  target registration plus paired bindings; evidence and closeout.
- **Out of scope:** Tabs, `tabs-reorder.ts`, DockRegion, Tree, other drag
  components, nested placement, auto-scroll, Rust/Node/GPUI/Jetstream,
  cross-window transport, DataTransfer bridge work, inbound files, drag-out,
  versions, releases, workflows, and sibling repositories.
- **Outcome shape:** smallest complete contract-valid migration, with temporary
  diagnostics removed, validation recorded, evidence updated, and a reviewable
  PR. Do not stop at a partial adapter or diagnosis unless a card stop
  condition is met.
- Preserve EditableList's current public API and complete next-order callback.
  Do not introduce a compatibility alias, dual drag controller, or silent HTML
  fallback.
- A row drag sensor must not steal editing fields, buttons, selection, taps, or
  pre-activation touch scrolling. Disabled and non-reorderable rows stay inert.
- Reuse the landed provider/controller/kernel. Do not fork the lifecycle into
  an EditableList-specific controller or duplicate semantic session state.
- Implement only the promoted logical keyboard target API in architecture 011,
  spec 069, and card 023. It is keyboard-only, element-free, ordered, and
  direction-aware; it reuses target eligibility and commit. Stop on any wider
  public API or architecture change.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** `g16.021` landed the paired semantic kernel in PR #96;
  `g16.022` landed the same-document controller and Svelte/React bindings in PR
  #101. Spec 069 now sequences EditableList first, Tree second, native
  convergence third, then Tabs and DockRegion together with the host bridge.
- **Why this card is ready:** its substrate dependency is merged; the operator
  approved the sequencing correction; the card has one component, two paired
  web runtimes, fixed behavior, explicit validation, and strict stop
  conditions. It deletes no public Tabs surface.
- **Decisions and preferences:** preserve human-facing specimens; exhaustive
  sensor cases belong in focused tests. Touch is first-class. Keyboard uses the
  same semantic result path. Result callbacks report the complete next item
  order. No HTML `DataTransfer` authority remains inside EditableList. Windowed
  keyboard keeps its public cross-page behavior through logical targets; the
  component must not page or unmount its source before drop.
- **Review correction:** the current centre-point keyboard resolver always
  produces `after`, so ArrowUp needs explicit previous-direction proof even
  outside windowed mode. The approved registry covers every item, not only the
  hidden page.
- **Open tensions:** row composition includes text editing and action buttons,
  so source handles and event composition need deliberate boundaries. A
  component-owned provider is acceptable only if it stays provider-scoped and
  does not prevent a consumer's surrounding drag provider from composing;
  stop if the landed API cannot express that cleanly.
- **Report after:** one runtime has the migration and its interaction tests
  green; then after paired runtime, browser, and full validation, or
  immediately when a stop condition is reached.
- **Report to:** the operator, who will relay progress and the final PR URL to
  the orchestrator.

The motion-learning and Longhorn conformance-lab notes remain separate. Tabs'
host-preparation callbacks are intentionally untouched until `g16.026`.

## Suggested Next Move

Read this file from the top. The original worktree preflight is already closed.
Fetch and rebase PR #104 onto pushed `main`, then read `AGENTS.md`, card 023,
the g16 README, architecture 011,
spec 069, the EditableList contract, the sequencing note, the g16.022 execution
log, and the repo-local Effigy skill.

Then inspect both EditableList implementations and their focused tests. Map the
current source, target, editing, button, disabled, and callback paths before
editing. Migrate one coherent paired behavior tranche rather than replacing
handlers mechanically.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before
   broad reads run only: `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it as the launcher-provided worktree. Record
   its actual root and branch. Do not compare it with suggested placeholders
   or create another worktree because it differs.
3. If the launcher supplied `main` or a dirty checkout, stop and report it. Do
   not silently create a second worktree. A manual fallback is allowed only
   when the current context is otherwise unusable outside that launcher
   failure: parse `.agents.local.env` as data, require an absolute
   `AGENTS_WORKTREE_CONTAINER_DIR`, ask the operator if it is absent, and create
   a unique worktree/branch below it from `origin/main`. Never use `/tmp`,
   `TMPDIR`, or a guessed path; never clean, reset, stash over, or discard an
   existing checkout.
4. This is an existing PR revision. From the selected worktree, fetch origin,
   rebase the branch onto `origin/main`, and confirm `git merge-base
   --is-ancestor 1d1d0937c077f32a2491cec614a9f167b41796bd HEAD` succeeds. Confirm this
   repository-relative handoff exists in the selected `HEAD`, load it with
   `git show HEAD:docs/handoffs/20260830-182242-g16-023-editable-list-reorder.md`,
   and compare it with the absolute dispatch file. Stop if they differ; the
   committed `HEAD` copy is canonical.
5. Required sibling links are `none`; make no sibling-path mutation.
6. Read the authority and card refs named above, then run `effigy tasks` and
   select the focused checks from card 023.

### While you work

- Execute the one card in coherent implementation/test chunks. Keep commits
  aligned with meaningful tranches, not model turns.
- Own diagnosis, implementation, cleanup, tests, evidence, and PR creation
  inside the card. A partial Svelte-only or React-only migration is not a
  completed outcome.
- Report after the first runtime migration and after paired/browser evidence,
  naming changed files, checks actually run, remaining work, ledger state, and
  blockers.
- Stop on any card stop condition, missing authority, public API conflict,
  provider-composition failure, scope expansion, or validation result that
  changes the plan. Do not pull Tabs or DockRegion into this branch.

### When the assigned runway is complete

1. Run every check named in card 023. Everything stays headless, including
   Chromium and WebKit.
2. Update card 023, one August execution log, the EditableList contract, the
   resolved sequencing note only if its recorded outcome needs clarification,
   g16/front-door continuation state, and unchanged ledger evidence.
3. Run `git diff --check origin/main...HEAD` and leave the worktree clean after
   committing.
4. Push the selected worker branch and open a reviewable PR against current
   `main`.
5. The PR body must link card 023, architecture 011, spec 069, the EditableList
   contract, paired component tests, browser evidence, active-source removal
   proof, unchanged ledger check, validation, and the execution log.
6. Return the PR URL and evidence to the operator. Do not merge and do not
   continue into Tree, Tabs, native drag, the host bridge, files, or drag-out.

### Review and merge path

The orchestrator will review the PR independently against the current head,
card, canonical refs, diff, checks, mounted fixtures, and browser evidence.
Because worker and orchestrator share a GitHub identity, the orchestrator posts
the verdict as a PR comment. The four findings in the existing changes-required
comment remain open until the revised head is independently cleared. The
operator must explicitly authorise any merge.

- **Closeout refs:** card 023, its August execution log, the EditableList
  contract, g16 README, roadmap front door, generation index, unchanged parity
  ledger/checker, and the single next-task state.

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, ledger, and
next-task state honest. If blocked, record the exact blocker and stop rather
than making the handoff look complete.
