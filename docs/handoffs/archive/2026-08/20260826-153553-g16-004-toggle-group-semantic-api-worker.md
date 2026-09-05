---
title: g16.004 ToggleGroup semantic API and mounted parity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260826-153553-g16-004-toggle-group-semantic-api-worker.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, parity, toggle-group, web, gpui]
---

## What This Thread Was Doing

`g16.002` correctly stopped ToggleGroup instead of fabricating parity over
contradictory semantics. PR #77 then landed `g16.003`, proving the required
host-owned native identity pattern on RadioGroup. The operator accepted the
ToggleGroup direction: resulting-selection payloads, single-mode radiogroup
roving focus, typed Rust results through the existing transition, and the same
required native interaction scope.

Those decisions are now promoted into the component contract and compiled as
one bounded implementation card. Start from this file; no copied transcript or
second prompt is required.

## Why It Matters

ToggleGroup currently looks structurally complete while its runtimes disagree:
web reports the resulting selection, Rust rendering reports only the activated
option, contracted single-mode arrow focus is absent on web, and native option
ids collide across instances. This lane closes the semantic seam and one exact
mounted-evidence cell without reviving a generic conformance framework.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `75fc597979dfb249ba579047aeba687f413242c6`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled the
  planning base before this handoff was created
- **Planning checkout:** clean at the recorded base
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** promoted ToggleGroup contract,
  `docs/roadmaps/g16/004-toggle-group-semantic-api-and-mounted-parity.md`,
  resolved decision note, and updated g16/front-door runway
- **Worker branch:** `t3code/g16-004-toggle-group-semantic-api`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-004-toggle-group-semantic-api`
- **Worktree creation command:** `git worktree add -b t3code/g16-004-toggle-group-semantic-api /Users/tom/.t3/worktrees/poodle/g16-004-toggle-group-semantic-api origin/main`
- **Worker worktree policy:** use a clean launcher-provided non-`main`
  registered worktree first, whatever its generated name. Only fall back to
  the named path and then `.agents.local.env` when the current context is
  unusable. Never create a second worktree merely because names differ.
- **Active contract:** `docs/contracts/components/toggle-group.md`; semantic
  choices are promoted and not delegated
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready card:**
  `docs/roadmaps/g16/004-toggle-group-semantic-api-and-mounted-parity.md`
- **Allowed runway:** execute `g16.004` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial; shared web semantics, Rust handler migration,
  mounted GPUI proof, and ledger close as one coherent lane
- **Parallel safety check:** all chunks touch the ToggleGroup contract/API,
  behavior tests, native renderer, mounted regression file, and ledger; do not
  split them across worktrees
- **Canonical refs:** `AGENTS.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`, and
  `docs/contracts/components/toggle-group.md`
- **Model capability profile:** capable coding model, high reasoning; stop for
  orchestrator review on new node vocabulary, a second behavior authority, or
  any contract contradiction
- **Tool/runtime restrictions:** use the repo-local Effigy skill and selectors;
  everything stays headless; never run `*-windowed`, native visual, Jetstream
  preview/QA, release, tag, or publication tasks
- **Required validation:** focused core/web/Rust ToggleGroup tests and machine
  conformance, changed Rust crate tests, `effigy regressions:native`,
  `effigy probe:gpui-specimens`, `effigy test:parity-evidence-ledger`,
  `effigy check:parity-evidence-ledger`, `effigy ci:native`, `effigy ci:web`,
  `effigy docs:check`, one final `effigy qa`, and
  `git diff --check origin/main...HEAD`
- **Known orientation finding:** `effigy doctor` is already red on the planning
  base from the open generated-in-src, oversized-file, and stale/broad
  suppression scans recorded in `PAPERCUTS.md`; report the baseline without
  absorbing unrelated cleanup
- **Planning validation:** `effigy docs:check` green on the planning base
- **PR base/head:** `main` <- worker branch
- **PR URL:** pending
- **Review state:** awaiting worker implementation and orchestrator review
- **Merge authorisation:** worker must not merge; explicit operator authority
  is required after orchestrator review

## Boundaries

Keep the run inside ToggleGroup semantic/API and mounted parity.

- **In scope:** pure shared web focus helpers; Svelte and React roving behavior
  and focused tests; `ToggleGroupHandlers` with required stable scope and typed
  resulting-selection callback; transition-backed native activation; scoped
  runtime ids, focus treatment, tab stops and arrow movement; mechanical Rust
  call-site migration; mounted GPUI proof; exact ledger regeneration; one log.
- **Out of scope:** other components, a generic roving-focus or identity
  architecture, new node vocabulary, public web prop changes, specimen
  redesign, visual fixtures/thresholds, accessibility promotion, Jetstream
  backend behavior/admission, workflows, versions, releases, and downstream
  repositories.
- Preserve the existing same-value rule: a valid activation in
  non-deactivating single mode emits the same resulting selection. Do not copy
  RadioGroup's same-value inertia into ToggleGroup.
- Single-mode arrows are horizontal Left/Right only. They select and focus the
  next enabled option. Multiple mode keeps ordinary button tab order and does
  not intercept arrows.
- Use the existing headless `ToggleGroupValue` and `toggle_group_transition`.
  Do not add another result enum or let hosts reconstruct membership from the
  activated option.
- The required native scope is construction data, not a web prop or semantic
  value. Never derive it from render order, a constant group label, selected
  value, or option values alone.
- Web focus must stay inside the mounted component instance. Never use a
  document-global option selector.
- Deferred Jetstream call sites may change only enough to compile against the
  shared-render signature. Do not run or claim its backend.
- Do not infer native accessibility parity from node role/state assertions.
  The ledger's GPUI accessibility cell remains `manual`.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or clean/reset an unrelated checkout.
- Do not merge the PR.

## Important Context

- **Planning lineage:** `g16.001` measured 174 portable native components and
  29 mounted GPUI cells. `g16.002` moved Checkbox, Switch, and
  SegmentedControl to 32. `g16.003` moved RadioGroup to 33 and landed the
  required handler-scope pattern. ToggleGroup is the last stopped selection
  control from that batch.
- **Why the card is ready:** the operator chose the semantic direction; the
  current TS and Rust headless transitions already agree; RadioGroup proves
  instance-scoped native focus; existing node keys, runtime ids, focus
  requests, focus rings, and headless driver are sufficient. The missing work
  is bounded implementation and evidence.
- **Current contradiction:** Svelte and React use the resulting-selection
  transition but leave every single-mode button tabbable. Shared Rust has the
  right headless transition but bypasses it in the renderer with `Fn(&str)`.
  GPUI specimens locally reconstruct multiple membership. The stale Jetstream
  note has been removed from the contract.
- **Decisions and preferences:** public web props stay unchanged; selection
  always flows through the shared transition; native callback takes owned
  `ToggleGroupValue`; host rebuild owns controlled state; semantic ids and
  backend runtime ids stay distinct; specimen pages remain human-centred.
- **Open tension:** `ToggleGroupSpec` stores selected values as a vector for
  both modes while the headless enum distinguishes single and multiple. Keep
  the conversion local and explicit. Stop if it cannot be lossless under the
  promoted contract.
- **Report after:** first the shared core/web behavior batch, then the native
  handler/mounted/ledger batch; report earlier on any stop condition
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Read this handoff and run the worktree preflight below before broad reads. Then
read the promoted contract and card, current TS/Rust transitions, both web
components/tests, the shared renderer, the GPUI ToggleGroup specimen, and
`g16.003`'s RadioGroup handler/mounted pattern.

Implement in two meaningful chunks. First land pure focus derivation and align
Svelte/React with focused tests. Then replace the Rust activated-option seam,
migrate callers, drive the real mounted GPUI path, regenerate the ledger, and
close the docs. Do not copy RadioGroup blindly: ToggleGroup preserves
same-value emission and has distinct multiple-mode focus behavior.

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
   `git merge-base --is-ancestor 75fc597979dfb249ba579047aeba687f413242c6 HEAD`;
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `docs/roadmaps/g16/README.md`, the assigned card, the
   ToggleGroup contract, and the canonical architecture/working-rule refs.
6. Read `.agents/skills/effigy/SKILL.md`, then use `effigy tasks` and
   `effigy doctor` for orientation. Record the known doctor baseline without
   widening into unrelated cleanup.

### While you work

- Execute only `g16.004`. Keep commits aligned with the two meaningful chunks,
  not model turns.
- Drive selection through existing transitions in both languages. Helper or
  direct-handler tests support the implementation but do not replace mounted
  GPUI evidence.
- Drive the real mounted backend/input path with host rebuilds. Spec inspection
  and specimen construction do not count as the mounted proof.
- After each chunk, report changed files, validation actually run, remaining
  acceptance, defects, and blockers through the operator.
- Stop on any card stop condition. Do not invent a generic substrate, widen
  public web APIs, alter other component semantics, or absorb Jetstream.

### When the assigned runway is complete

1. Run the full validation named in `Current State`, entirely headlessly.
2. Mark the card complete, regenerate the ledger through its source, add the
   August execution log, and leave g16's next task as an orchestrator review
   checkpoint. Do not compile or implement another card.
3. Run `git diff --check origin/main...HEAD` and confirm the worktree is clean
   after committing.
4. Push the worker branch and open a reviewable PR against current `main`.
5. The PR body must link the g16 milestone/card and promoted contract; name the
   core/web/mounted tests; report the handler/API and call-site migration;
   demonstrate same-value, deactivation, multiple-result and two-instance
   behavior; give ledger before/after counts; list validation; and preserve
   unresolved accessibility/visual gaps.
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
