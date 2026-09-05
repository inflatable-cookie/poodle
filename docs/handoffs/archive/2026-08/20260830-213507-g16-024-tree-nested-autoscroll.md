---
title: g16.024 Tree nested intent and auto-scroll worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: revision-authorised
owner: Poodle drag-and-drop substrate
created: 2026-08-30
updated: 2026-08-30
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260830-213507-g16-024-tree-nested-autoscroll.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, drag-drop, tree, svelte, react]
---

## What This Thread Was Doing

The orchestrator merged `g16.023` in PR #104. EditableList now proves the
shared web drag substrate for simple Svelte/React reorder, including touch and
windowed keyboard targets. This dispatch owns the next serial card only:
`g16.024`, which migrates Tree and adds the reusable nested-target geometry and
auto-scroll behavior that Tree exposes.

This is one bounded implementation lane. No transcript or second prompt is
part of the authority chain.

## Why It Matters

Tree is the programme's adversarial web proof. It combines nested targets,
before/inside/after intent, virtual rows, editing and selection controls, and
nested scroll containers. If this migration is dependable, the substrate has
proved more than flat reorder without pulling native, cross-window, or file
transport work forward.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `1067ed1e9f5108f1491aa536ed89b8fcd41f7392`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  the planning base before this handoff was created.
- **Planning checkout:** clean at the planning base.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** architecture 011, spec 069,
  completed card 023 and its log, ready card 024 with review oracle, and the
  g16/front-door continuation state.
- **Review planning amendment:** the orchestrator promoted a bounded
  `requestKeyboardDrop` command and narrowed the WebKit touch-evidence claim on
  current pushed `main`. The existing PR must fetch and rebase before revision.
- **Worker branch:** suggested manual fallback
  `t3code/g16-024-tree-nested-autoscroll`; use a clean launcher-provided
  non-`main` branch when supplied.
- **Worker worktree:** suggested manual fallback
  `/Users/tom/.t3/worktrees/poodle/g16-024-tree-nested-autoscroll`; use the
  launcher-provided worktree when supplied.
- **Worktree creation command:** normal path is launcher-managed. Manual
  fallback only after the protocol permits it:
  `git worktree add /Users/tom/.t3/worktrees/poodle/g16-024-tree-nested-autoscroll -b t3code/g16-024-tree-nested-autoscroll origin/main`.
- **Worker worktree policy:** follow the Completion Protocol; launcher
  worktree first, named/manual fallback only when required.
- **Required sibling worktree links:** none.
- **Active spec lane:**
  `docs/specs/069-dependable-drag-and-drop-substrate.md`.
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`.
- **Ready cards, in order:**
  `docs/roadmaps/g16/024-drag-drop-tree-nested-intent-and-auto-scroll.md`.
- **Allowed runway:** card 024 only, then stop.
- **Remaining card budget:** one card.
- **Dispatch topology:** serial behind merged card 023 and before card 025.
- **Parallel safety check:** no other worker may edit Tree, the web drag
  controller's geometry/auto-scroll seams, or the drag browser probe during
  this run. Later drag cards depend on this result.
- **Canonical refs:**
  `docs/architecture/011-drag-and-drop-substrate.md`,
  `docs/specs/069-dependable-drag-and-drop-substrate.md`,
  `docs/contracts/components/tree.md`, and
  `docs/contracts/001-working-rules.md`.
- **Review oracle:** card 024 `## Review Oracle`.
- **Model capability profile:** frontier coding model with high reasoning for
  nested arbitration, scroll ownership, virtualization, and lifecycle cleanup.
- **Tool/runtime restrictions:** headless only. Never run `*-windowed`, native
  visual, GPUI/Jetstream preview, release, publication, tag, or workflow
  mutation selectors.
- **Required validation:** focused paired Tree and substrate tests;
  deterministic geometry and auto-scroll tests; `effigy
  test:drag-drop-browser` for headless Chromium and WebKit; `effigy ci:web`;
  `effigy docs:check`; `effigy check:parity-evidence-ledger`; one final
  headless `effigy qa`; and `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` to the selected worker branch.
- **PR URL:** pending.
- **Review state:** changes requested; execution repairs accepted, bounded
  keyboard-command revision authorised.
- **Merge authorisation:** none. The worker must not merge.

## Boundaries

- **In scope:** card 024 only: Tree's Svelte/React migration; reusable web
  geometry, deepest-target arbitration, and nearest-scroll-container
  auto-scroll seams; paired mounted and headless browser proof; focused Tree
  contract/specimen updates; execution log and honest closeout.
- **Out of scope:** other components, Rust/Node/GPUI/Jetstream, Tabs,
  DockRegion, cross-window transport, host bridges, inbound files, drag-out,
  versions, releases, workflows, and sibling repositories.
- **Outcome shape:** smallest complete contract-valid migration. Own diagnosis,
  implementation, removal of temporary diagnostics, validation, evidence, and
  a reviewable PR; do not stop at a partial runtime unless a card stop
  condition is reached.
- Preserve Tree's current `onReorder(from, to, position)` contract and its
  selection, expansion, rename, checkbox, context-menu, keyboard-navigation,
  disabled, and Svelte virtualization behavior.
- Preserve Alt+Up/Down as a one-keystroke sibling move, but route it through the
  shared `requestKeyboardDrop` lifecycle over Tree's complete visible logical
  target catalogue. Space and Enter remain Tree selection/activation keys.
- Reuse the landed provider/controller/kernel. Do not create Tree-owned drag
  session state, an auto-scroll timer in Tree, fake hidden DOM targets, or an
  HTML `DataTransfer` fallback.
- One pointer position resolves to one deepest eligible target and one semantic
  position. One nearest eligible container owns scrolling. Eligibility is
  checked again at drop.
- Keep specimens human-facing. Exhaustive sensor/lifecycle evidence belongs in
  focused tests and the headless browser probe.
- Work only in the clean worker worktree selected by the Completion Protocol.
  Never edit the planning checkout or an unrelated dirty checkout.
- Do not merge. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** `g16.021` landed the paired semantic kernel in PR #96;
  `g16.022` landed the same-document web controller and framework bindings in
  PR #101; `g16.023` migrated EditableList in PR #104. Card 024 is the final
  web-only substrate proof before Rust/GPUI work in card 025.
- **Why the card is ready:** every dependency is merged; Tree's public move
  callback is documented; nested arbitration and transport ownership are fixed
  in architecture/spec; scope, validation, stop conditions, and an adversarial
  review oracle are explicit.
- **Decisions and preferences:** touch is first-class; ordinary scrolling wins
  before activation; keyboard reaches the same intent/commit path; semantic
  positions are `before`, `inside`, and `after`; nearest-scroll ownership and
  exactly-once cleanup belong to the shared web substrate.
- **Review decisions:** `requestKeyboardDrop({ sourceId, targetId, position })`
  is the bounded public command for established one-shot shortcuts. It starts
  the ordinary keyboard lifecycle and returns whether that lifecycle started;
  it is not a direct component callback. Chromium proves native touch
  hold-versus-scroll. Desktop Playwright WebKit proves touch-shaped
  hold/tolerance plus real mouse/keyboard geometry, auto-scroll, and cleanup;
  it must not be described as native touch-scroll proof.
- **Open tensions:** Svelte virtualization may unmount targets during a
  session, and nested containers can both be scrollable. Stable ids,
  invalidation, and source lifetime must remain coherent without a Tree-local
  lifecycle. Stop if that requires a broader Tree rewrite or a public callback
  change.
- **Report after:** reusable nested arbitration/auto-scroll primitives and
  their focused tests are green; then after the paired Tree migration and
  browser proof; immediately on any stop condition.
- **Report to:** the operator, who will relay progress and the PR URL to the
  orchestrator.

The motion-learning and Longhorn conformance-lab triage notes stay open and
outside this lane.

## Suggested Next Move

Fetch and rebase this PR branch onto current pushed `main`, then run the
Completion Protocol revision preflight before broad reads. Read
`AGENTS.md`, the g16 milestone, card 024, architecture 011, spec 069, the Tree
contract, the card 023 execution log, and the repo-local Effigy skill. Inspect
the promoted command contract before editing. Keep the accepted execution
repairs stable. Implement the controller command, paired framework exposure,
logical Tree targets, Alt+Up/Down routing, and focused lifecycle/browser proof.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run only: `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it as the launcher-provided worktree. Record
   its actual root/branch; do not compare it with the suggested fallback or
   create another worktree because names differ.
3. If the launcher supplied `main` or a dirty checkout, stop and report it.
   Otherwise, when a manual fallback is genuinely required, parse
   `.agents.local.env`, require the absolute `AGENTS_WORKTREE_CONTAINER_DIR`,
   and create a unique worktree/branch below it from `origin/main`. Never use
   `/tmp`, `TMPDIR`, or a guessed path; never clean, reset, stash over, or
   discard an existing checkout.
4. From the selected existing PR worktree, fetch origin, rebase the branch onto
   current `origin/main`, confirm `git merge-base --is-ancestor origin/main
   HEAD`, confirm `git merge-base --is-ancestor
   1067ed1e9f5108f1491aa536ed89b8fcd41f7392 HEAD`, and confirm
   `docs/handoffs/20260830-213507-g16-024-tree-nested-autoscroll.md` exists in
   that `HEAD`. Load it with `git show HEAD:<relative-path>` and compare it
   with the absolute dispatch file. Stop if they differ; the tracked blob is
   canonical.
5. Required sibling links are `none`; make no sibling-path mutation.
6. Read the authority and card refs named above, then use `effigy tasks` to
   confirm the focused selectors.

### While you work

- Execute only the authorised keyboard-command revision and its proof in
  coherent implementation/test chunks. Preserve the accepted auto-scroll,
  twisty exclusion, arbitration, and lifecycle repairs unless integration
  exposes a concrete defect.
- Own reproduction, diagnosis, implementation, cleanup, tests, evidence, and
  PR creation inside the card. A Svelte-only or React-only migration is not a
  complete result.
- Report after the shared geometry/auto-scroll tranche and after paired/browser
  evidence. Name changed files, checks actually run, remaining work, unchanged
  ledger state, risks, and blockers.
- Stop on any card stop condition, missing authority, public API conflict,
  virtualization rewrite, scope expansion, or validation result that changes
  the plan. Do not pull card 025 or later transports into this branch.

### When the assigned runway is complete

1. Run every required validation named above. Everything stays headless,
   including Chromium and WebKit.
2. Falsify the diff against card 024's review oracle: nested inner/outer scroll
   containers, all three positions, disable/removal before release, active
   cancellation, virtualization, drop-time revalidation, terminal cleanup, and
   Alt+Up/Down through the ordinary keyboard lifecycle. Map each claim to proof.
3. Update card 024, one August execution log, the Tree contract where behavior
   changed, g16/front-door continuation state, and unchanged ledger evidence.
4. Run `git diff --check origin/main...HEAD`, commit meaningful chunks, push the
   selected worker branch, and open a PR against current `main`.
5. The PR body must link card 024, architecture 011, spec 069, the Tree
   contract, focused paired tests, Chromium/WebKit evidence, active-source
   removal proof, unchanged ledger check, validation, and the execution log.
6. Report the PR URL and evidence to the operator. Do not merge or continue to
   card 025.

### Review and merge path

The orchestrator reviews the PR independently against the canonical refs,
current diff, checks, review oracle, mounted tests, and browser evidence. Since
worker and orchestrator share a GitHub identity, the verdict will be a PR
comment. Requested changes must stay on this branch. A planning change returns
to planning before implementation revision. The operator must explicitly
authorise merge.

- **Requested changes:** rebase onto the promoted planning amendment; add
  `requestKeyboardDrop` and paired framework exposure; move Tree Alt+Up/Down to
  that lifecycle over logical targets; add the required focused proof; keep the
  WebKit evidence claim within the documented headless boundary.
- **Closeout refs:** card 024, its August execution log, Tree contract, g16
  README, roadmap front door, unchanged parity ledger/checker, and the single
  next-task state.

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, ledger, and
next-task state honest. If blocked, record the exact blocker and stop rather
than making the handoff look complete.
