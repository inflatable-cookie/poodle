---
title: g16.022 drag-and-drop web custom-surface substrate worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle drag-and-drop substrate
created: 2026-08-30
updated: 2026-08-30
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260830-153354-g16-022-drag-drop-web-substrate.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, drag-drop, svelte, react]
---

## What This Thread Was Doing

The orchestrator has merged and closed the continuous-audio lane through
`g16.032`. It has now returned to the dependable drag-and-drop programme and
promoted its next serial card, `g16.022`.

This worker owns the first mounted web adapter over the paired semantic kernel
landed in `g16.021`: one framework-free DOM controller, idiomatic Svelte and
React bindings, real pointer/touch/keyboard sensors, preview and announcement
projection, and exact teardown. This is a bounded implementation lane; the
worker does not need the originating conversation or another prompt.

## Why It Matters

Poodle components and consumers currently repeat drag event choreography and
repeat the same failures: lost cleanup, stale targets, touch/scroll conflicts,
incorrect nested selection, and framework-specific drift. The semantic kernel
now gives both language pairs one lifecycle. This card makes that lifecycle
usable by arbitrary web surfaces before any existing Poodle component is
migrated onto it.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `e39c5ab1aec97d939ae1788c1b0152b74acbb06c`
- **Pushed main verification:** local `HEAD` equalled `origin/main` at that SHA
  before this handoff and its planning changes were created.
- **Planning checkout:** clean before the closeout/readiness batch.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included by the tracked handoff commit:** PR #100
  closeout; the exact public web custom-surface API in spec 069; ready card
  `g16.022`; and updated g16/front-door next-task state.
- **Worker branch:** use the launcher-provided clean non-`main` branch;
  suggested manual fallback is `t3code/g16-022-drag-drop-web-substrate`.
- **Worker worktree:** harness-managed. Do not create a second worktree when
  the launcher already supplied one.
- **Worktree creation command:** none on the normal launcher path. If manual
  fallback is genuinely required, first parse `.agents.local.env`, require an
  absolute `AGENTS_WORKTREE_CONTAINER_DIR`, then create a unique worktree under
  that container from `origin/main`. Never guess a path.
- **Required sibling worktree links:** none.
- **Active spec lane:**
  `docs/specs/069-dependable-drag-and-drop-substrate.md`.
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`.
- **Ready cards, in order:**
  `docs/roadmaps/g16/022-drag-drop-web-custom-surface-substrate.md`.
- **Allowed runway:** card 022, all four batches, then stop.
- **Remaining card budget:** one card.
- **Dispatch topology:** serial drag-and-drop web-substrate lane.
- **Parallel safety check:** no other worker may edit the drag kernel, core web
  controller, Svelte/React provider surfaces, or drag browser fixtures during
  this run. Card 023 waits for this mounted substrate; card 025 is further
  downstream and edits Node/GPUI routing.
- **Canonical refs:**
  `docs/architecture/011-drag-and-drop-substrate.md`,
  `docs/specs/069-dependable-drag-and-drop-substrate.md`,
  `docs/architecture/006-headless-core-and-machine-model.md`, and
  `docs/contracts/001-working-rules.md`.
- **Model capability profile:** frontier coding capability with high reasoning;
  this lane owns public API, input lifetime, asynchronous drop completion,
  touch arbitration, focus, and cleanup.
- **Tool/runtime restrictions:** headless only. Never run `*-windowed`, native
  visual, GPUI/Jetstream preview, release, publication, tag, or workflow
  mutation selectors.
- **Required validation:** focused core/Svelte/React tests; mounted custom
  fixtures; headless Chromium and WebKit probes; `effigy ci:web`;
  `effigy docs:check`; `effigy check:parity-evidence-ledger`; one final
  headless `effigy qa`; and `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` to the selected worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation, then orchestrator review.
- **Merge authorisation:** none. The worker must not merge.

## Boundaries

Please keep this run inside card 022:

- **In scope:** the exact core controller and public types fixed in spec 069;
  same-document Pointer Events for mouse, pen, and touch; keyboard sensor;
  cached/invalidation-driven geometry; kernel effect execution; immutable
  snapshot/capability reads; Svelte provider/context/actions; React
  provider/hooks/prop getters; overlay/live region; custom fixtures; browser
  evidence; exports, docs, and closeout.
- **Out of scope:** migration of Tabs, EditableList, Tree, BlockEditor,
  OrderBy, ModelCatalogueEditor, DockRegion, or any consumer; deletion of old
  Tabs/DockRegion exports; auto-scroll; cross-window transport; DataTransfer;
  inbound files; drag-out; Rust/Node/GPUI/Jetstream work; tokens, releases,
  workflows, sibling repositories, and component ledger movement.
- **Outcome shape:** smallest complete contract-valid implementation, with
  temporary diagnostics removed, validation recorded, evidence updated, and a
  reviewable PR. Do not stop at a capability probe or root-cause report unless
  a card stop condition is met.
- Use the landed `dragSessionTransition` and `resolveDropTarget` kernel. Do not
  duplicate session phase, terminal, arbitration, or exactly-once state in the
  DOM or framework adapters.
- Keep the controller provider-scoped and connected to one root/document.
  Never add a module singleton or default document registry. Duplicate live
  ids fail; registration cleanup is idempotent; two providers remain isolated.
- Touch scrolling wins until the configured hold/tolerance boundary. Do not
  apply global `touch-action:none` or use HTML Drag and Drop as internal
  authority.
- Framework adapters must be idiomatic. Shared semantics are equal; Svelte
  actions and React prop getters do not need identical syntax.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

Stop and return to the operator if the landed kernel must change materially;
if the exact public API fixed in spec 069 is insufficient or contradictory;
if correct touch behavior needs global scroll suppression or HTML Drag and
Drop authority; if browser evidence needs a focus-stealing/windowed harness;
or if the work expands into auto-scroll, component migration, cross-window,
files, native runtimes, releases, or another repository.

## Important Context

- **Planning lineage:** architecture 011 and spec 069 were promoted from the
  operator's requirement for dependable mouse, pen, touch, keyboard,
  cross-window, inbound-file, and drag-out support. `g16.021`, merged in PR
  #96, landed only the paired TypeScript/Rust semantic kernel and 25 shared
  session vectors plus 7 arbitration vectors. This card is the next adapter
  layer, not a new semantic model.
- **Why this card is ready:** its kernel dependency is merged; spec 069 now
  fixes the controller, registration, handle, snapshot, activation,
  announcement, preview, capability, commit-result, Svelte, and React public
  shapes; the card names four bounded batches, mounted/browser evidence,
  writable scope, stop conditions, and continuation.
- **Decisions and preferences:** Pointer Events are the internal web
  transport. Touch is first-class from this card. The provider owns one overlay
  and one polite live region. Source/target labels provide usable default
  announcements; `describeAnnouncement` customizes copy without becoming a
  second lifecycle callback. An injected controller is disconnected, not
  destroyed, when its provider unmounts.
- **Open tensions:** browser capture and touch/scroll behavior differ between
  Chromium and WebKit, so both need real headless evidence. Async `onDrop`
  completion must remain session-identified so late results cannot resurrect a
  cancelled or superseded session. Prop getters must compose consumer refs and
  handlers rather than replace them.
- **Report after:** the framework-free controller and focused lifetime tests
  pass; then after both framework adapters and browser probes pass, or
  immediately when a stop condition is reached.
- **Report to:** the operator, who will relay progress and the final PR URL to
  the orchestrator.

The open motion-learning and Longhorn conformance-lab triage notes remain
separate programme choices. EditableLabel remains a component decision gate.
Do not pull any of them into this card.

## Suggested Next Move

Start by reading this file from the top. Before any broad repository read, run
the four-command worktree safety preflight below. Once the selected worktree is
verified against pushed `main`, read `AGENTS.md`, card 022, the g16 README,
architecture 011, spec 069, architecture 006, working rules, the g16.021 card
and log, and the repo-local Effigy skill.

Then inspect `packages/core/src/drag-drop.ts` and its shared-vector tests. Build
Batch 1 around that API: one provider-scoped DOM controller, registration
handles, one effect runner, one sensor lifetime, immutable reads, and exact
cleanup. Prove that foundation before adding either framework adapter.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before
   broad reads run only: `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it as the launcher-provided worktree. Record
   its actual root and branch. Do not compare them with suggested placeholders
   or create another worktree because they differ.
3. If the launcher supplied `main` or a dirty checkout, stop and report it. Do
   not silently create a second worktree. A manual fallback is allowed only
   when the current context is otherwise unusable outside that launcher
   failure: parse `.agents.local.env` as data, require an absolute
   `AGENTS_WORKTREE_CONTAINER_DIR`, ask the operator if it is absent, and create
   a unique worktree/branch under it from `origin/main`. Never use `/tmp`,
   `TMPDIR`, or a guessed path; never clean, reset, stash over, or discard an
   existing checkout.
4. From the selected worktree, fetch origin and confirm `HEAD == origin/main`.
   Confirm `git merge-base --is-ancestor
   e39c5ab1aec97d939ae1788c1b0152b74acbb06c HEAD` succeeds. Confirm this
   repository-relative handoff exists in the selected `HEAD`, load it with
   `git show HEAD:docs/handoffs/20260830-153354-g16-022-drag-drop-web-substrate.md`,
   and compare it with the absolute dispatch file. Stop if they differ; the
   committed `HEAD` copy is canonical.
5. Required sibling links are `none`; make no sibling-path mutation.
6. Read the authority and card refs named above, then run `effigy tasks` and
   select the focused checks from card 022.

### While you work

- Execute the four card batches in order. Keep commits aligned with coherent
  tranches, not model turns.
- Own implementation, browser diagnosis, cleanup, tests, evidence, and PR
  creation within the card. A sensor or capability probe alone does not finish
  the lane.
- Report after Batch 1 and after the two framework adapters, naming changed
  files, checks actually run, remaining work, ledger state, and blockers.
- Stop on any card stop condition, missing authority, public API conflict,
  scope expansion, or validation result that changes the plan. Do not invent a
  wider architecture.

### When the assigned runway is complete

1. Run every check in card 022. Everything stays headless, including Chromium
   and WebKit.
2. Update card 022, one August execution log, spec 069 only for exact landed
   names if necessary, g16/front-door continuation state, and the unchanged
   ledger evidence. Do not move a component row.
3. Run `git diff --check origin/main...HEAD` and leave the worktree clean after
   committing.
4. Push the selected worker branch and open a reviewable PR against current
   `main`.
5. The PR body must link card 022, architecture 011, spec 069, controller and
   registration APIs, framework fixtures, Chromium/WebKit evidence, cleanup
   traces, unchanged ledger proof, validation, and the execution log.
6. Return the PR URL and evidence to the operator. Do not merge and do not
   continue into card 023, auto-scroll, cross-window, files, drag-out, native
   runtimes, component migration, or a release.

### Review and merge path

The orchestrator will review the PR independently against the current head,
card, canonical refs, diff, checks, mounted fixtures, and browser evidence.
Because the worker and orchestrator share a GitHub identity, the orchestrator
will post the verdict as a PR comment. Requested changes: none yet. The
operator must explicitly authorise any merge.

- **Closeout refs:** card 022, its August execution log, spec 069 exact landed
  names, g16 README, roadmap front door, generation index, unchanged parity
  ledger/checker, and the single next-task state.

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, ledger, and
next-task state honest. If blocked, record the exact blocker and stop rather
than making the handoff look complete.
