---
title: g16.006 Tabs drag, keyboard, and mounted parity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260826-195641-g16-006-tabs-drag-keyboard-mounted-parity-worker.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, parity, tabs, drag, gpui]
---

## What This Thread Was Doing

The orchestrator re-measured the post-`g16.005` parity ledger and returned to
the Tabs drag-lifecycle stop rather than choosing another easy mounted test.
Inspection showed that Tree and ModelCatalogueEditor already consume the shared
payload/drop seam, but that seam is incomplete and currently wrong in two
places: every zone sees every drag move, and drop discards the last computed
edge. Stock GPUI exposes enough start, release, drop, and root-key routing to
repair it without a fork.

That decision is promoted into the Tabs contract and compiled as one bounded
implementation card. Complete the existing reusable payload lifecycle, consume
it in shared Rust Tabs, and prove the result through the real mounted GPUI tree.
Start from this file; no copied transcript or second prompt is required.

## Why It Matters

Tabs is a foundation primitive used throughout Poodle consumers. Its web
implementations already support selection, roving focus, close, keyboard
reorder, pointer reorder, and transient source/target state; shared Rust does
not. Closing the generic payload seam also makes existing Tree and
ModelCatalogueEditor drag behavior more trustworthy without creating another
cross-runtime behavior plane.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `375fb9511d303e0aee196de77ba72fd636305a8c`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled the
  planning base before this handoff was created
- **Planning checkout:** clean at the recorded base
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** promoted Tabs contract, resolved
  drag-lifecycle triage, ready `g16.006` card, and current g16/front-door runway
- **Worker branch:** `t3code/g16-006-tabs-drag-keyboard-mounted-parity`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-006-tabs-drag-keyboard-mounted-parity`
- **Worktree creation command:** `git worktree add -b t3code/g16-006-tabs-drag-keyboard-mounted-parity /Users/tom/.t3/worktrees/poodle/g16-006-tabs-drag-keyboard-mounted-parity origin/main`
- **Worker worktree policy:** use a clean launcher-provided non-`main`
  registered worktree first, whatever its generated name. Only fall back to
  the named path and then `.agents.local.env` when the current context is
  unusable. Never create a second worktree merely because names differ.
- **Active contract:** `docs/contracts/components/tabs.md`; semantic native
  lifecycle, complete-order reorder, and web-only DOM-event decisions are
  promoted
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready card:**
  `docs/roadmaps/g16/006-tabs-drag-keyboard-and-mounted-parity.md`
- **Allowed runway:** execute `g16.006` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial; generic payload lifecycle and Tabs consumption
  share node vocabulary, GPUI root input, renderer handlers, mounted tests, and
  the parity ledger
- **Parallel safety check:** do not split lifecycle and Tabs work across
  worktrees; one half cannot be accepted or honestly tested without the other
- **Canonical refs:** `AGENTS.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/components/tabs.md`, and
  `docs/triage/20260826-173329-tabs-native-drag-lifecycle.md`
- **Model capability profile:** capable coding model, high reasoning; event
  ordering, public Rust handler changes, and stock-GPUI constraints require
  careful review
- **Tool/runtime restrictions:** use the repo-local Effigy skill and selectors;
  everything stays headless; never run `*-windowed`, native visual, Jetstream
  preview/QA, release, tag, or publication tasks
- **Required validation:** focused TS/Rust Tabs machine and web component tests;
  focused node/renderer/GPUI backend lifecycle tests; retained Tree and
  ModelCatalogueEditor evidence; `effigy regressions:native`;
  `effigy probe:gpui-specimens`; `effigy test:parity-evidence-ledger`;
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

Keep the run inside the existing payload/drop seam and Tabs semantic/mounted
parity.

- **In scope:** focused Tabs machine/web proof; bounded node start/end/leave and
  Delete vocabulary; GPUI hit-testing, retained edge, drop/end ordering,
  outside/Escape cancellation; shared Rust Tabs handler/reorder/close/focus
  repair; controlled GPUI Tabs state; mechanical callers; retained Tree and
  ModelCatalogueEditor regressions; one mounted Tabs proof; exact ledger
  regeneration; one log.
- **Out of scope:** a new gesture architecture, raw pointer coordinates in
  nodes, GPUI patches/forks, specimen or CSS redesign, overflow/history/tooltip
  changes, other component contract redesign, visual fixtures, accessibility
  promotion, Jetstream admission, workflows, versions, releases, and downstream
  repositories.
- Keep payload drag separate from delta-only `on_drag` and value-control
  `on_scrub`. Do not combine their runtime types or handler semantics.
- Use `tabs_transition` for pointer and keyboard reorder and emit the complete
  next order. Hosts apply results and rebuild; they do not reproduce reorder
  math.
- Deliver hover only to the hit zone, preserve the last semantic `DropEdge`,
  fire successful drop before source end, and fire end exactly once on every
  completion/cancellation path.
- DOM `PointerEvent`/`DragEvent` objects remain web-only. Native handlers carry
  values and semantic target state only.
- Tree and ModelCatalogueEditor are regression consumers. Correct generic
  backend behavior and adapt mechanically, but do not redesign their contracts
  or move their ledger cells.
- Node-level role/focus assertions are not broad native accessibility proof.
  Keep Tabs' accessibility ledger cell `manual`.
- Deferred Jetstream call sites may change only enough to compile against
  shared signatures. Do not run or claim its backend.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or clean/reset an unrelated checkout.
- Do not merge the PR.

## Important Context

- **Planning lineage:** `g16.001` measured 174 portable native components and
  29 mounted GPUI cells. `g16.002`–`g16.005` moved six controls to 35 mounted /
  139 missing. This card moves only Tabs to 36 / 138.
- **Why the card is ready:** the TypeScript and Rust Tabs machines already own
  ordered reorder effects; both web shells expose the complete lifecycle;
  shared Rust already has scoped tab ids, roving focus, panel semantics, and
  transient drag visuals; node payload/drop vocabulary and real GPUI drop
  routing already exist. The missing lifecycle fields are bounded additions to
  that seam.
- **Current backend defects:** `on_drag_move` is capture-wide and the adapter
  does not check the zone bounds before invoking `on_drop_hover`; `on_drop`
  emits `DropEdge::default()` instead of the last hover edge; no semantic
  start/end/leave reaches shared composition.
- **Current Tabs defects:** `TabsHandlers` has no reorder or drag callbacks;
  tab nodes publish no payload/drop intent; the native key path hardcodes
  `reorderable: false`, treats Alt+Arrow as ordinary focus movement, and cannot
  map Delete.
- **Decisions and preferences:** preserve the Svelte contract; no keyboard-only
  native exception; stock crates.io GPUI 0.2.2 only; specimens remain
  human-centred; Jetstream remains deferred.
- **Open tension:** exactly-once cancellation may require a small backend-owned
  active payload session shared by the production and headless root hosts. That
  is authorized. Stop if it requires component-specific global state or GPUI
  internals/forking.
- **Report after:** first the generic node/backend lifecycle with focused tests;
  then Tabs machine/web/shared-Rust and mounted/ledger closeout. Report earlier
  on any stop condition.
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Read this handoff and run the worktree preflight below before broad reads. Then
read the promoted Tabs contract and card, existing TS/Rust machines, both web
components/tests, `poodle-node` payload types, GPUI drop listeners/root host,
shared Rust Tabs, Tree and ModelCatalogueEditor consumers, the GPUI Tabs facade,
and mounted driver patterns.

Implement in two meaningful chunks. First make the reusable payload lifecycle
correct and prove event ordering on stock GPUI while retaining existing drag
paths. Then consume it in Tabs, finish keyboard/close behavior, drive controlled
mounted input, regenerate the ledger, and close the docs. Do not paper over a
missing runtime event with direct handler invocation.

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
   `git merge-base --is-ancestor 375fb9511d303e0aee196de77ba72fd636305a8c HEAD`;
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `docs/roadmaps/g16/README.md`, the assigned card, Tabs
   contract, resolved triage note, and canonical architecture/working-rule refs.
6. Read `.agents/skills/effigy/SKILL.md`, then use `effigy tasks` and
   `effigy doctor` for orientation. Record the known doctor baseline without
   widening into unrelated cleanup.

### While you work

- Execute only `g16.006`. Keep commits aligned with the two meaningful chunks,
  not model turns.
- Use focused direct machine/backend tests as support, but use mounted backend
  input and controlled host rebuilds for the Tabs ledger claim.
- Preserve existing Tree and ModelCatalogueEditor focused/mounted behavior and
  the delta-drag/scrub regressions after correcting the generic payload seam.
- After each chunk, report changed files, validation actually run, remaining
  acceptance, event-order evidence, defects, and blockers through the operator.
- Stop on any card stop condition. Do not invent generic architecture, patch
  GPUI, alter unrelated APIs, or admit Jetstream.

### When the assigned runway is complete

1. Run the full validation named in `Current State`, entirely headlessly.
2. Mark the card complete, regenerate the ledger through its source, add the
   August execution log, and leave g16's next task as an orchestrator review
   checkpoint. Do not compile or implement another card.
3. Run `git diff --check origin/main...HEAD` and confirm the worktree is clean
   after committing.
4. Push the worker branch and open a reviewable PR against current `main`.
5. The PR body must link the g16 milestone/card, promoted Tabs contract, and
   resolved triage note; name lifecycle and mounted tests; report exact event
   ordering, retained Tree/ModelCatalogueEditor evidence, Tabs handler/key
   changes, ledger before/after counts, validation, and remaining gaps.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect PR metadata, commits, changed files, diff, tests,
public Rust API migration, GPUI lifecycle ordering, retained consumers, ledger
lineage, and checks independently. Because worker and orchestrator share the
GitHub identity, the orchestrator will post the canonical verdict as a PR
comment rather than formal self-approval. Requested changes are currently none.
The operator must explicitly authorise merge after a green review.

- **Closeout refs:** assigned card, g16 README/front doors, Tabs contract,
  resolved triage note, generated parity ledger, and one August log

### Handoff closeout

Before calling the runway complete, leave the card, log, ledger, roadmap, and
next-task state honest. If blocked, record the exact blocker and stop rather
than making the handoff appear complete.
