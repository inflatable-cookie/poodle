---
title: g16.013 TriStateSwitch contract and mounted parity worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260827-160028-g16-013-tri-state-switch-worker.md
base_commit: 8d5afec2393006703ef7d0d64beca9ff94c51e62
tags: [coordination, handoff, worker, pr, gpui, tri-state-switch]
---

## What This Thread Is Doing

Execute ready card `g16.013`. Cleanly migrate the Rust TriStateSwitch contract
from checkbox-shaped `CheckState` to semantic `TriStateValue`, align native
radio behavior and identity with the web authority, and prove the result
through mounted headless GPUI dispatch.

Start from this file. No copied transcript or second prompt is required.

## Why It Matters

TriStateSwitch currently means three different things at once. Svelte, React,
and the detailed contract use excluded/default/included and default to Default;
the Rust spec stores unchecked/mixed/checked and defaults to Excluded. GPUI then
makes every segment a tab stop, keeps disabled segments focusable, shares one
constant identity across instances, and emits same-value activation that the
web state machine rejects. The operator explicitly approved the clean pre-1.0
break needed to remove that drift.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Required base:** `8d5afec2393006703ef7d0d64beca9ff94c51e62`
- **Pushed-main verification:** local `HEAD` and `origin/main` both equalled the
  required base before the ready card and this handoff were compiled
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates worker-only worktree preflight
- **Worker branch:** `t3code/g16-013-tri-state-switch-mounted-parity`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-013-tri-state-switch-mounted-parity`
- **Worktree creation command:** `git worktree add -b t3code/g16-013-tri-state-switch-mounted-parity /Users/tom/.t3/worktrees/poodle/g16-013-tri-state-switch-mounted-parity origin/main`
- **Worktree policy:** use the clean, dedicated, non-`main` worktree supplied
  by the launcher even if its generated name differs. Never create a second
  worktree for a naming mismatch. Manual fallback requires the configured
  `AGENTS_WORKTREE_CONTAINER_DIR`; never guess `/tmp` or a repository child.
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready card:**
  `docs/roadmaps/g16/013-tri-state-switch-contract-and-mounted-parity.md`
- **Source decision:**
  `docs/triage/20260827-160028-post-g16-012-native-lane-decision.md`
- **Allowed runway:** execute `g16.013` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial; component spec, shared renderer, GPUI wrapper,
  mounted regression, and generated ledger are common hot files
- **Current ledger:** 41 mounted / 133 missing; only TriStateSwitch may move to
  42 mounted / 132 missing; known-delta totals stay 115 / 60
- **Canonical refs:** `AGENTS.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/components/tri-state-switch.md`, and the source decision
- **Tool/runtime restriction:** use repo-local Effigy selectors; everything is
  headless; never run `*-windowed`, native visual, Jetstream preview/QA,
  release, tag, or publication tasks
- **Known orientation finding:** `effigy doctor` is already red on the
  planning base from generated-in-src, oversized-file, and stale/broad
  suppression scans recorded in `PAPERCUTS.md`; report that baseline without
  absorbing cleanup
- **PR base/head:** `main` <- worker branch
- **Review state:** awaiting worker implementation and orchestrator review
- **Merge authority:** worker must not merge; explicit operator authorization
  follows orchestrator review

## Fixed Behavior

- `TriStateSwitchSpec` stores `TriStateValue`, defaults to Default, and exposes
  `with_value`. Remove legacy `state`, `with_state`, the undocumented general
  `label`, `with_label`, and TriStateSwitch-only CheckState conversion helpers.
  Do not add aliases, deprecated shims, fallbacks, or alternate constructors.
- Keep `CheckState` itself for checkbox semantics. Migrate Poodle-owned GPUI and
  deferred-Jetstream TriStateSwitch call sites directly.
- Native construction requires `TriStateSwitchHandlers::new(instance_id)` with
  optional typed `on_value_change`. Semantic ids stay separate from scoped
  backend runtime/focus ids.
- The root is one labelled radiogroup. Its three fixed segments are labelled
  radio buttons with selected state. Exactly the selected enabled segment is
  the tab stop; disabled controls have none.
- Pointer, Space, and Left/Right report a changed resulting value once. Arrows
  wrap and request focus for the target inside the originating instance.
  Same-value and disabled paths are inert.
- The host owns value state and rebuilds the spec. The renderer owns no hidden
  mutable selection.
- Reuse existing node/backend key, activation, role, selected-state, runtime-id,
  tab-index, focus-request, and structured-focus-ring channels.

## Boundaries

- Do not change public Svelte/React props, implementations, or behavior.
- Do not preserve the legacy Rust TriStateSwitch API. This breaking migration
  is approved and deliberate.
- Do not add generic node/backend vocabulary, a new selection machine,
  uncontrolled native state, arbitrary options, form-name behavior, or an
  exhaustive specimen matrix.
- Do not behaviorally repair or admit Jetstream. Its changes are compilation
  only.
- Do not change NumberInput, EditableLabel, Accordion, RadioGroup, ToggleGroup,
  SegmentedControl, visual comparison, or accessibility evidence.
- Do not edit release, version, workflow, downstream, publication, or sibling
  repository surfaces.
- Work only in the selected worker worktree. Never edit, reset, clean, or stash
  the orchestrator checkout.
- Do not merge the PR.

## Important Context

- The contract behavior machine and both web implementations reject selecting
  the current value. Contract section 10a contains a stale contradictory
  Jetstream sentence saying the active segment emits; correct that sentence.
- The web default is `default`. Rust's current `CheckState::Unchecked` default
  maps to excluded and is a measured defect, not an allowed delta.
- `TriStateValue` already exists and has the required fixed index ordering. Do
  not add another enum.
- RadioGroup and ToggleGroup already demonstrate the required native handler
  scope, roving tab stop, runtime identity, arrow focus request, and mounted
  host-rebuild patterns. Reuse their shape without coupling the components.
- The current GPUI specimen translates an integer selection into CheckState and
  every wrapper instance shares `poodle-tri-state-switch`. Replace both seams
  with semantic value and stable descriptive scopes.
- Keep the specimen human-centred. Correct its initial/default behavior and
  identity; do not turn it into the test matrix.

## Suggested Work Shape

1. Migrate the spec, enum helpers, contract, and all Rust call sites in one
   clean compilation batch. Add focused spec tests.
2. Add the scoped renderer handler boundary, radio semantics, roving focus,
   same-value/disabled inertia, and focused renderer tests.
3. Wire the GPUI wrapper/specimen, add the production mounted regression,
   regenerate only TriStateSwitch's ledger cell, close the docs, and run the
   complete headless validation board.

Report after the semantic API batch, then after mounted proof and closeout.
Stop immediately on a card stop condition.

## Required Validation

Use `effigy tasks` to confirm selector names in the worker worktree. At minimum
run:

- focused `poodle-specs` and `poodle-render` TriStateSwitch tests;
- focused compilation/tests for each Rust crate whose call sites change;
- focused Svelte and React TriStateSwitch tests;
- the named mounted TriStateSwitch regression;
- `effigy regressions:native`;
- `effigy probe:gpui-specimens`;
- `effigy drift:handlers`, `effigy drift:events`, `effigy drift:roles`, and
  relevant contract/spec drift selectors;
- `effigy test:parity-evidence-ledger`;
- `effigy check:parity-evidence-ledger`;
- `effigy ci:rust`;
- `effigy ci:native`;
- `effigy ci:web`;
- `effigy docs:check`;
- one final `effigy qa`;
- `git diff --check origin/main...HEAD`.

No windowed selector is authorized.

## Completion Protocol

### Before implementation

1. Run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. Accept the launcher-provided worktree only when it is registered, clean,
   and not on `main`. If unusable, inspect the named path, then read
   `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`. Ask the
   operator if it is absent. Never clean or reset another checkout.
3. Run `git fetch origin`; require `HEAD == origin/main`; require
   `git merge-base --is-ancestor 8d5afec2393006703ef7d0d64beca9ff94c51e62 HEAD`;
   and confirm this handoff and the ready card exist in `HEAD`.
4. Read `AGENTS.md`, the repo-local Northstar and Effigy skills, the g16
   README, card, component contract, source decision, ledger, shared renderer,
   GPUI wrapper/specimen, mounted driver, RadioGroup/ToggleGroup renderer
   patterns, and paired web implementations/tests.
5. Run `effigy tasks` and `effigy doctor`. Record the known doctor baseline;
   do not widen into cleanup.

### While implementing

- Execute only `g16.013` in the three meaningful batches above.
- Make the approved semantic API break cleanly. Compatibility shims are a
  defect, not a kindness.
- Establish focused spec/renderer behavior before adding mounted evidence.
- Use existing production renderer/backend channels and host-owned rebuilds.
- Keep every interaction scope explicit, non-empty, descriptive, and stable.
- Stop rather than widening if honest behavior needs a generic backend change
  or another component's semantic repair.

### When complete

1. Run the full validation above, entirely headlessly.
2. Mark the card complete and source decision resolved. Add one August
   execution log and leave g16 at an orchestrator evidence checkpoint. Do not
   compile or start `g16.014`.
3. Confirm only TriStateSwitch's mounted cell moves and totals are 42 mounted /
   132 missing; known-delta totals remain 115 / 60. Run
   `git diff --check origin/main...HEAD`.
4. Commit coherent batches, push the worker branch, and open a PR against
   current `main`.
5. The PR body must link the card, contract, and source decision; state the
   approved API break, default-value defect, callback/focus/identity repairs,
   mounted evidence, exact ledger delta, validation, and explicit non-claims.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review path

The orchestrator independently inspects PR metadata, commits, changed files,
clean removal of the legacy surface, all mechanical call-site migration,
renderer dispatch/focus/identity, specimen route, mounted production proof,
ledger delta, closeout, and checks. Because worker and orchestrator share one
GitHub identity, the orchestrator posts the canonical verdict as a PR comment
rather than formal self-approval. The operator authorizes merge after a green
review.

## Stop Conditions

- The web runtimes or detailed contract disagree on semantic value, default,
  fixed order, same-value inertia, disabled behavior, or callback payload.
- The migration requires a compatibility alias/fallback or a semantic change
  outside TriStateSwitch rather than mechanical call-site edits.
- Existing node/backend channels cannot express the contracted radio behavior
  and stable focus identity.
- Mounted proof bypasses production hit testing, focus, key dispatch, or host
  rebuild.
- The ledger generator changes another row or evidence column.
- Validation exposes work in NumberInput, EditableLabel, Accordion, Jetstream
  admission, release, workflow, publication, downstream, or sibling repos.
