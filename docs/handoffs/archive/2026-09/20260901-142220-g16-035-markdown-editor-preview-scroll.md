---
title: g16.035 MarkdownEditor bounded preview scroll worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-142220-g16-035-markdown-editor-preview-scroll.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, markdown-editor, issue-fix]
---

## What This Thread Was Doing

The operator reported that a long MarkdownEditor preview contributes its full
rendered height, expands the surrounding editor layout, and does not become an
internal scroll pane. The orchestrator compiled outcome-scoped card g16.035.
This thread reproduces, diagnoses, fixes, proves, and closes that one issue.

This dispatches one bounded implementation lane. No transcript or second prompt
is part of the authority chain.

## Why It Matters

MarkdownEditor is a reusable content surface. Letting preview markup dictate
outer layout makes it unusable in shells, workstations, and other bounded
editing regions. The contract already assigns vertical scroll ownership to the
preview; implementation must make that true under a real host constraint.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `4b87baabaec83a65d1cd1adccccdc41c624897fc`
- **Pushed main verification:** local and `origin/main` both resolved to
  `4b87baabaec83a65d1cd1adccccdc41c624897fc` before this handoff was created
- **Planning checkout:** clean before this planning batch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** current MarkdownEditor contract,
  parity note, and active g16 roadmap; this commit adds ready card g16.035
- **Worker branch:** `fix/g16-035-markdown-editor-preview-scroll`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-035-markdown-editor-preview-scroll`
- **Worktree creation command:** `git worktree add -b
  fix/g16-035-markdown-editor-preview-scroll
  /Users/tom/.t3/worktrees/poodle/g16-035-markdown-editor-preview-scroll
  origin/main`
- **Worker worktree policy:** follow Completion Protocol; launcher worktree
  first, named/manual fallback only when required
- **Required sibling worktree links:** none
- **Active spec lane:** none; the component contract and issue-fix card govern
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g16/035-markdown-editor-bounded-preview-scroll.md`
- **Allowed runway:** g16.035 only
- **Remaining card budget:** one issue-fix card
- **Dispatch topology:** runs beside g16.034 shared motion implementation and
  the operator-driven drag-and-drop bug session
- **Parallel safety check:** g16.035 owns MarkdownEditor-specific source,
  contract, parity, tests, probe, card, and log. It must not edit motion policy,
  g16.034 pilot components, drag surfaces, or shared g16 front doors. Report an
  unexpected overlap instead of resolving it.
- **Surfaces this lane owns:** the g16.035 writable scope, its card, and one
  MarkdownEditor execution log
- **Integration ownership:** the orchestrator owns
  `docs/roadmaps/g16/README.md` and `docs/roadmaps/generation-index.md` closeout
  after same-repository merge ordering is known
- **Merge ordering:** same-repository PRs merge one at a time; the orchestrator
  refreshes this head against current `main` and re-reviews it if g16.034 or
  another sibling PR merges first
- **Canonical refs:** `docs/contracts/001-working-rules.md`;
  `docs/contracts/components/markdown-editor.md`,
  `docs/parity/markdown-editor.md`
- **Review oracle:** g16.035 `## Review Oracle`
- **Model capability profile:** matching non-frontier day-to-day profile for a
  bounded CSS/layout diagnosis and paired implementation
- **Frontier-worker justification:** none
- **Tool/runtime restrictions:** use Effigy; headless only; never run
  `*-windowed`, release, tag, publication, workflow mutation, or sibling-repo
  commands
- **Required validation:** focused paired component tests, bounded real-browser
  geometry/scroll proof in Chromium and WebKit, relevant Rust/native checks only
  if those sources change, drift/docs checks, `effigy ci:web`,
  `effigy docs:check`, and `git diff --check origin/main...HEAD`
- **PR base/head:** `main` ← `fix/g16-035-markdown-editor-preview-scroll`
- **PR URL:** pending
- **Review state:** awaiting implementation PR
- **Merge path:** orchestrator after accepted review of the current head and
  passing required checks

## Boundaries

- **In scope:** reproduce through the smallest complete contract-valid repair,
  cleanup, validation, falsification evidence, closeout, and PR for g16.035.
- **Out of scope:** new public sizing props, changed `minHeight` semantics,
  textarea auto-growth, markdown parsing changes, motion, drag-and-drop,
  releases, consumers, Jetstream admission, and broad layout architecture.
- **Outcome shape:** complete issue fix, not diagnostics-only. Diagnosis is part
  of the implementation lane.
- Do not invent architecture, change the settled behavior envelope, widen the
  roadmap, or choose a public API.
- Write only inside the named surfaces. Stop on shared mutable scope or a hidden
  dependency instead of silently resolving it.
- Work only in the clean worker worktree selected by Completion Protocol.
- Do not merge. Merge belongs to the orchestrator.

## Important Context

- **Planning lineage:** MarkdownEditor's contract already says the preview uses
  `overflow-y: auto`; current shared CSS sets that property but leaves the
  intrinsic-size/ancestor constraint chain unproved. Rust currently hides root
  overflow and does not declare preview scrolling.
- **Why the card is ready:** the observed failure, expected host/scroll
  behavior, non-goals, validation, evidence, and stop conditions are fixed. The
  exact causal edit remains worker judgment.
- **Decisions and preferences:** a definite surrounding host owns the maximum
  available height; short unconstrained content stays natural; no default
  viewport maximum and no new public height prop.
- **Open tensions:** CSS overflow alone is not proof without a definite shrink
  chain. GPUI may express equivalent overflow through existing Node layout but
  must not grow into a new tracked-scroll architecture in this lane.
- **Report after:** reproduction/diagnosis, then coherent fix/evidence, then
  final validation and PR.
- **Report to:** the operator through the active control plane.

## Suggested Next Move

Run the Completion Protocol preflight before broad reads. Read AGENTS.md,
g16.035, the MarkdownEditor contract/parity note, paired web implementations,
shared stylesheet, and native render path. Use Effigy to find the narrow test
surface, then reproduce the long-preview geometry before editing.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch; do not create another because generated names differ.
3. If current context is main, dirty, unregistered, or unusable, inspect the
   named worktree. If unusable, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask when absent. Create a unique
   worktree there from pushed `origin/main`. Never use `/tmp`, `TMPDIR`, or a
   guessed path; never clean, reset, stash over, or discard dirty state.
4. From the selected worktree, fetch with
   `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch
   origin`. Confirm `HEAD == origin/main`, confirm
   `4b87baabaec83a65d1cd1adccccdc41c624897fc` is an ancestor, and confirm this
   handoff exists in `HEAD`. Load the tracked blob. If the absolute dispatch
   file differs, stop. The committed `HEAD` copy is canonical.
5. Required sibling links are none.
6. Read the milestone, g16.035, AGENTS.md, and canonical refs.
7. Run cheap orientation checks and record what ran.

### While you work

- Own the full issue loop: reproduce, diagnose, repair, remove temporary
  instrumentation, validate, evidence, and closeout.
- Keep commits aligned with meaningful chunks.
- Report after each named chunk with changed files, validation, remaining work,
  risks, and blockers.
- Stop on missing contract, ambiguous intent, public API need, shared mutable
  files, or validation that changes the plan.

### When the assigned runway is complete

1. Run the required validation named above.
2. Falsify every g16.035 oracle row. Commit the real proof before planting the
   pre-fix behavior, confirm each proof fails for the intended geometry/scroll
   reason, restore, and rerun green.
3. Update the card and one September log. Do not edit the two orchestrator-owned
   g16 front doors.
4. Push the worker branch. If a sibling merged first, refresh against current
   main, revalidate, and state that in the PR.
5. Open a PR against current pushed main. Link the card, contract, parity note,
   changed surfaces, evidence, validation, and unresolved items.
6. Report the PR URL. Do not merge.

### Review And Merge Path

The orchestrator reviews the exact PR head against the canonical refs, diff,
checks, and every oracle row. Same-identity acceptance is recorded as a PR
comment. Blocking findings are `execution-miss`, `oracle-gap`,
`planning-change`, `validation-gap`, or `integration-drift`. Requested changes
are none at dispatch. An accepted exact head with passing checks and clean
mergeability is merged without another approval prompt.

- **Closeout refs:** g16.035, MarkdownEditor contract/parity note, one September
  execution log; the orchestrator owns g16 README and generation-index
  integration.

### Handoff Closeout

Leave the card and log honest. If blocked, record the blocker and stop rather
than making the handoff look complete.
