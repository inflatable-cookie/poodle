---
title: g16.003 RadioGroup native identity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260826-124447-g16-003-radio-group-native-identity-worker.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, parity, gpui, radio-group]
---

## What This Thread Was Doing

`g16.002` closed mounted GPUI evidence for Checkbox, Switch, and
SegmentedControl. RadioGroup stopped because stateless shared Rust rendering
had no lifetime-stable identity for two unnamed groups with the same option
values. The operator has now fixed that boundary: native RadioGroup receives a
required host-owned interaction scope through a handler bundle; web `name`
behavior stays unchanged.

This is one bounded implementation thread. Start from this file; no copied
transcript or second prompt is required.

## Why It Matters

RadioGroup already has coherent selection semantics and real implementations
in every active runtime, but GPUI cannot prove roving focus honestly while
option ids collide across instances. This lane repairs the identity boundary
and closes one exact mounted-evidence cell without reviving a generic
conformance schema. The landed handler pattern is also the prerequisite for a
later, separate ToggleGroup semantic/API lane.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `0265b82951e8cd5c58a9d016c59c792998ceb52d`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled the
  planning base before this handoff was created
- **Planning checkout:** clean at the recorded base
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** updated RadioGroup contract,
  `docs/roadmaps/g16/003-radio-group-native-identity-and-mounted-parity.md`,
  the g16/front-door runway, and the selection-control decision note
- **Worker branch:** `t3code/g16-003-radio-group-native-identity`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-003-radio-group-native-identity`
- **Worktree creation command:** `git worktree add -b t3code/g16-003-radio-group-native-identity /Users/tom/.t3/worktrees/poodle/g16-003-radio-group-native-identity origin/main`
- **Worker worktree policy:** use a clean launcher-provided non-`main`
  registered worktree first, whatever its generated name. Only fall back to
  the named path and then `.agents.local.env` when the current context is
  unusable. Never create a second worktree merely because names differ.
- **Active spec lane:** the promoted RadioGroup contract; no further semantic
  or product decision is delegated
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g16/003-radio-group-native-identity-and-mounted-parity.md`
- **Allowed runway:** execute `g16.003` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial; ToggleGroup is explicitly sequenced after
  this pattern lands
- **Parallel safety check:** both lanes would touch native interaction
  identity, the GPUI mounted regression file, parity-ledger generator, and g16
  front doors; they are not independent
- **Canonical refs:** `AGENTS.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`, and
  `docs/contracts/components/radio-group.md`
- **Model capability profile:** capable coding model, medium reasoning; stop
  for frontier/orchestrator review on a new node capability, generic identity
  architecture, contract contradiction, or scope expansion
- **Tool/runtime restrictions:** use the repo-local Effigy skill and selectors;
  everything stays headless; never run `*-windowed`, native visual, Jetstream
  preview/QA, release, tag, or publication tasks
- **Required validation:** focused changed-module/crate tests,
  `effigy regressions:native`, `effigy probe:gpui-specimens`,
  `effigy test:parity-evidence-ledger`,
  `effigy check:parity-evidence-ledger`, `effigy ci:native`,
  `effigy ci:web`, `effigy docs:check`, one final `effigy qa`, and
  `git diff --check origin/main...HEAD`
- **Known orientation finding:** `effigy doctor` is already red on the recorded
  main base from the open oversized-file and broad-suppression scans in
  `PAPERCUTS.md`; record it, but do not absorb that unrelated debt
- **PR base/head:** `main` <- worker branch
- **PR URL:** pending
- **Review state:** awaiting worker implementation and orchestrator review
- **Merge authorisation:** worker must not merge; explicit operator authority
  is required after orchestrator review

## Boundaries

Keep the run inside RadioGroup native identity and mounted behavior.

- **In scope:** `RadioGroupHandlers` with required stable scope and optional
  callback; scoped per-option runtime ids; focus treatment; roving tab-stop,
  arrow movement, wrapping and disabled skipping; mechanical Rust call-site
  migration; real mounted GPUI proof; exact ledger regeneration; one log.
- **Out of scope:** RadioGroup web API or implementation changes, ToggleGroup,
  generic `RenderContext` identity, other component migrations, new node
  vocabulary, specimen redesign, visual fixtures or thresholds, accessibility
  promotion, Jetstream backend behavior/admission, workflows, releases, and
  downstream repositories.
- The required native scope is not the web form `name`. Never derive it from
  render order, a constant group label, selected value, or option values alone.
- Static native call sites still provide a stable descriptive scope and leave
  the callback absent. Deferred Jetstream call sites may change only enough to
  compile against the new shared-render signature.
- Do not infer native accessibility parity from node role/state assertions.
  The ledger's GPUI accessibility cell remains `manual`.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or clean/reset an unrelated checkout.
- Do not merge the PR.

## Important Context

- **Planning lineage:** g16.001 measured 174 portable native components and 29
  mounted GPUI cells. `g16.002` moved Checkbox, Switch, and SegmentedControl,
  taking that count to 32, but correctly stopped RadioGroup and ToggleGroup
  rather than inventing their missing semantics.
- **Why this card is ready:** the RadioGroup contract already fixes selection,
  focus entry, orientation axes, wrap, disabled skipping, and same-value
  inertia. The operator chose the only missing API boundary. Existing node
  keys, focus requests, headless driver, and SegmentedControl roving code are
  available patterns; no new cross-runtime authority is needed.
- **Decisions and preferences:** native identity lives in
  `RadioGroupHandlers::new(instance_id)`; the scope is required and stable;
  `on_change` remains optional; host rebuild owns selection; Svelte and React
  stay untouched; semantic ids and backend runtime ids remain distinct.
- **Open tensions:** changing the shared render signature touches compile-only
  callers. Keep that migration mechanical. Stop if existing node vocabulary
  cannot represent the required focus result or if a call site needs a wider
  semantic change.
- **Report after:** the handler/call-site migration plus focused render tests,
  then the completed mounted/ledger batch; report earlier on any stop condition
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Read this handoff and run the four-command worktree preflight below before
broad reads. Then inspect the RadioGroup contract, renderer, GPUI facade,
headless driver, and SegmentedControl's scoped roving implementation.

Take two meaningful chunks: first introduce the handler boundary and migrate
callers with focused compilation; then implement the scoped mounted behavior,
regression, ledger, and closeout. Do not copy SegmentedControl blindly—preserve
RadioGroup's own orientation and same-value rules.

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
   `.agents.local.env`, require the absolute
   `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique worktree/branch there
   from `origin/main`. Ask the operator if the key is absent. Never use `/tmp`,
   `TMPDIR`, a repository child, or a guessed path. Never clean, reset, stash,
   or discard the original checkout. If the launcher itself supplied a dirty
   or `main` worktree, stop and report it instead of silently creating another.
4. From the selected worktree, run `git fetch origin`; confirm `HEAD` equals
   `origin/main`; confirm
   `git merge-base --is-ancestor 0265b82951e8cd5c58a9d016c59c792998ceb52d HEAD`;
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `docs/roadmaps/g16/README.md`, the assigned card,
   `docs/contracts/components/radio-group.md`, and the canonical
   architecture/working-rule refs named above.
6. Read `.agents/skills/effigy/SKILL.md`, then use `effigy tasks` and
   `effigy doctor` for the required orientation. Record the known doctor
   baseline without widening into its unrelated cleanup.

### While you work

- Execute only `g16.003`. Keep commits aligned with the two meaningful chunks,
  not model turns.
- Drive the real mounted backend/input path. Direct handler calls, transition
  helper calls, spec inspection, and specimen construction do not count as the
  mounted proof.
- After each chunk, report changed files, validation actually run, remaining
  acceptance, defects, and blockers through the operator.
- Stop on any card stop condition. Do not invent a generic identity substrate,
  alter the web API, or absorb ToggleGroup.

### When the assigned runway is complete

1. Run the full validation named in `Current State`, entirely headlessly.
2. Update the card status, regenerate the ledger through its source, add one
   August execution log, and leave g16's next task as an orchestrator review
   checkpoint. Do not compile or implement the ToggleGroup card.
3. Run `git diff --check origin/main...HEAD` and confirm the worktree is clean
   after committing.
4. Push the worker branch and open a reviewable PR against current `main`.
5. The PR body must link the g16 milestone/card, name the mounted tests, report
   the handler/API migration and exact call sites, show the two-instance
   identity proof, give the ledger before/after count, list validation, and
   preserve unresolved evidence gaps.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect PR metadata, commits, changed files, diff, tests,
ledger lineage, and checks independently. Because worker and orchestrator share
the GitHub identity, the orchestrator will post the canonical verdict as a PR
comment rather than formal self-approval. Requested changes are currently none.
The operator must explicitly authorise merge after a green review.

- **Closeout refs:** the assigned card, g16 README/front doors, generated
  parity ledger, one August log, and the single next-task checkpoint

### Handoff closeout

Before calling the runway complete, leave the card, log, ledger, roadmap, and
next-task state honest. If blocked, record the exact blocker and stop rather
than making the handoff appear complete.
