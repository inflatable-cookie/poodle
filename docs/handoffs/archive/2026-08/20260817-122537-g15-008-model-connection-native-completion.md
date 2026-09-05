---
title: g15.008 model-connection native completion worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-17
updated: 2026-08-17
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260817-122537-g15-008-model-connection-native-completion.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, model-connection, gpui]
---

## What This Thread Is Doing

Poodle is finishing the full Svelte v0.2.0 release roster, then closing the
active native cohort. The web ModelConnectionPicker, ModelConnectionSetup,
ModelConnectionCard, and ModelCatalogueEditor references are approved and
landed. This worker adds their hand-written Rust data surfaces, pure headless
behaviour, shared `poodle-render` composition, and GPUI specimens.

This is one bounded implementation lane. Start from this file without a copied
transcript or a second prompt.

## Why It Matters

Nucleus needs the model-connection management surface for Swallowtail routes.
Poodle renders and requests changes; it must not acquire provider truth,
credentials, discovery, persistence, update policy, model defaults, or route
fallbacks. The implementation must also avoid reviving either failed
cross-language architecture: no portable component interface and no universal
conformance corpus.

The card now states the native binding explicitly. Specs carry safe controlled
data. Handler structs carry callbacks. Host nodes compose outside specs. Small
pure Rust functions mirror the already-approved web state machines and are
proved with owner-local vectors naming the same inputs and outputs.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `07e8ca84ad4d1ce6761166a3a5107feedd4b8ea0`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled the
  planning base before this handoff was created
- **Planning checkout:** clean `main`; implementation edits are forbidden there
- **Worker branch:** `t3code/g15-008-model-connection-native-completion`
- **Worker worktree:** `/Users/tom/.t3/worktrees/poodle/g15-008-model-connection-native-completion`
- **Worktree creation command:** `git fetch origin && git worktree add /Users/tom/.t3/worktrees/poodle/g15-008-model-connection-native-completion -b t3code/g15-008-model-connection-native-completion origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path and branch;
  do not create a second worktree for that reason. If the supplied context is
  unusable, follow `docs/contracts/005-agent-local-paths.md` and never guess a
  worktree container.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready card:** `docs/roadmaps/g15/008-model-connection-family-native-completion.md`
- **Allowed runway:** g15.008 only, Batches A through D
- **Remaining card budget:** one whole roadmap card
- **Dispatch topology:** serial
- **Parallel safety check:** none. g15.009 is deliberately blocked because it
  shares Rust registries, native evidence surfaces, and release rosters with
  this lane.
- **Canonical architecture:** `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/product-guardrails.md`
- **Canonical working rules:** `docs/contracts/001-working-rules.md`,
  `docs/contracts/005-agent-local-paths.md`
- **Canonical product history:** `docs/roadmaps/g14/018-model-connection-web-reference.md`,
  `docs/roadmaps/g14/019-model-connection-reference-review.md`,
  `docs/roadmaps/g14/020-model-connection-active-runtime-completion.md`,
  `docs/specs/067-model-connection-management.md`
- **Component contracts:** `docs/contracts/components/model-connection-picker.md`,
  `docs/contracts/components/model-connection-setup.md`,
  `docs/contracts/components/model-connection-card.md`,
  `docs/contracts/components/model-catalogue-editor.md`
- **Release evidence:** `docs/roadmaps/g15/release-baseline-roster.md`,
  `docs/roadmaps/g15/release-gap-register.md`,
  `docs/roadmaps/g14/conformance-estate.md`
- **Approved web behaviour source:** `packages/core/src/model-connection.ts`
  plus the four Svelte and React implementations/tests/specimens
- **Model capability profile:** frontier coding model, high reasoning — public
  Rust surface, controlled bindings, repeated host composition, reorder/drag,
  focus, and accessibility interactions
- **Tool/runtime restrictions:** never run `*-windowed`,
  `test:native-visual`, `qa:jetstream`, or any Jetstream selector. Use only the
  headless conformance path admitted by Effigy.
- **Required validation:** focused tests for every touched crate; the card's
  named selectors; final headless `effigy qa`; and
  `git diff --check origin/main...HEAD`
- **PR base/head:** `main` <- selected worker branch
- **PR URL:** pending
- **Review state:** awaiting implementation and orchestrator review
- **Merge authorisation:** none. The worker pushes a PR and stops.

## Boundaries

Keep this run inside g15.008.

- **In scope:** the four named Rust specs; safe structural enums/data;
  `poodle-headless` behaviour mirrors and focused tests; renderer handlers and
  host-composition seams; GPUI specimens and mounted interaction evidence;
  native-binding notes in the four contracts; the four native roster/register
  rows; one August batch log; new papercuts.
- **Out of scope:** Svelte/React implementation or specimen changes; Nucleus or
  Swallowtail changes; provider/route registries; credentials; OAuth;
  discovery/probes; persistence; update policy; model defaults/favourites/
  options; ModelPicker redesign; ModelConnectionList; Jetstream; shared
  corpus/comparator/interface machinery.
- `<Name>Spec` is cloneable controlled display data only. No callback, closure,
  backend type, credential, or host `Node` belongs in a spec.
- Web `default*` seeds are not Rust fields. GPUI/AppState owns current values,
  callbacks request changes, and the host updates the next spec.
- Repeated host content is keyed by opaque IDs. Provider marks come from host
  content; the generic mark is only the specimen fallback.
- Keep pure behaviour in `poodle-headless`; keep renderer composition and event
  requests in `poodle-render`; keep GPUI limited to backend interpretation,
  mounted input, lifecycle, and drawing.
- Preserve exact route IDs. Never add fallback routing.
- Direct routes submit without a configure step. Configured routes render only
  host-supplied configuration content; Poodle never sees its values.
- Enabled state remains independent of readiness and availability.
- Catalogue state owns shown order and visibility only. It emits complete
  shown-order ID payloads and explicit visibility requests; it does not own
  model policy.
- Preserve the operator-approved web specimens unchanged. If native evidence
  exposes a web defect or demands a public web change, stop and return that
  decision to the orchestrator.
- Do not edit g15.009 or later cards, generation status, dispatch, front doors,
  task definitions, workflows, dependency manifests, or release machinery.
- Work only in the selected worker worktree. Never edit, clean, reset, or stash
  the orchestrator checkout or another worker's checkout.
- Do not merge the PR.

## Required Behaviour and Evidence

### ModelConnectionPicker

- Preserve source order while filtering the approved searchable fields.
- Group exactly by the supplied availability classification.
- Do not select disabled or unsupported options.
- Prove exact selection payloads, radio semantics, roving focus, query changes,
  result announcements, and empty/loading/error/ready postures.
- Selected state replaces the generic/provider leading mark; it does not add a
  second trailing indicator.

### ModelConnectionSetup

- `choose` continues directly to submit when the selected option does not
  require configuration.
- Otherwise it requests `configure`, renders host-owned configuration content,
  and submits only from the configured stage.
- Prove back/cancel/value/query/submit requests and every pending/disabled
  guard. Never manufacture a generic credential form or stepper.

### ModelConnectionCard

- Preserve independently controlled disclosure and enabled state.
- Prove status/access copy, active-check precedence, disabled dimming, closed
  accessory composition, action composition, narrow layout, and focus return
  after collapse.
- Keep the provider mark immediately left of the name without indenting the
  summary lines.

### ModelCatalogueEditor

- Derive shown/hidden rows from supplied items without mutating host state.
- Prove explicit up/down moves, keyboard grab/move/drop, admitted pointer drag,
  complete shown-order payloads, hide/restore payloads, focus-after-hide, and
  announcements.
- Keep info/custom actions optional. Prove empty, error, disabled, shown-only,
  and mixed shown/hidden states distinctly.

### Cross-language behaviour evidence

- Use explicit owner-local TS and Rust cases for the same inputs and observable
  outputs. Existing core tests may supply the TS side; add only missing focused
  cases.
- Do not create a shared fixture file, portable schema, normalized observation,
  runtime adapter, screenshot comparator, or new conformance framework.
- Name evidence separately for Svelte, React, Rust headless/render, and GPUI.
  Jetstream is program-deferred and cannot be counted as passing.

## Suggested Execution

1. **Batch A — declarations and behaviour:** add native-binding notes; define
   safe Rust data; port the pure approved state-machine behaviour into
   `poodle-headless`; add paired owner-local vectors.
2. **Batch B — render and host bindings:** implement all four renderers,
   handlers, keyed host-composition seams, accessibility/focus, and focused
   render tests.
3. **Batch C — GPUI evidence:** add curated specimens and mounted headless
   interactions. Preserve the web specimen presentation.
4. **Batch D — release evidence:** reconcile only the four native rows; write
   the batch log; rebase; run final gates; push the PR.

Report after Batch A, after Batch B, and with the final PR. Each report should
name changed files, tests actually run, remaining work, and any stop condition.

## Completion Protocol

### Before starting

1. Read this handoff, then run one read-only safety probe:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. Accept a clean registered non-`main` launcher worktree even when its path or
   branch differs from the placeholders. Do not create another worktree merely
   to match names.
3. If the current context is `main`, dirty, or unregistered, stop before broad
   reads and follow the worktree policy in
   `docs/contracts/005-agent-local-paths.md`. Never use `/tmp`, `TMPDIR`, a
   repository-adjacent guess, or another thread's worktree.
4. Fetch origin. Confirm this handoff exists in `HEAD`; confirm
   `git merge-base --is-ancestor 07e8ca84ad4d1ce6761166a3a5107feedd4b8ea0 HEAD`.
   A launcher-created branch may include the handoff commit beyond that
   planning base.
5. Read `AGENTS.md`, the repo-local Effigy skill, g15 README, g15.008, and all
   canonical refs named above before editing.
6. Run `effigy tasks` for selector routing. Treat known repository-wide doctor
   scan debt as baseline unless the branch worsens it.

### While working

- Keep commits aligned with the four meaningful batches.
- Update the contract before any observable binding decision not already
  stated there.
- Test pure transitions in `poodle-headless`; test node shape/callbacks in
  `poodle-render`; test mounted input/focus in GPUI.
- Record small execution friction in `PAPERCUTS.md`. Stop on an architectural,
  product-policy, security, or public-web decision.
- Do not run broad suites after every edit. Finish a coherent batch, then run
  its narrow selectors.

### Before opening the PR

1. Fetch and rebase onto current `origin/main`, then rerun the final validation
   on the rebased head.
2. Run at minimum:
   - `effigy test:core`
   - `cargo test -p poodle-headless`
   - `cargo test -p poodle-specs`
   - `cargo test -p poodle-render`
   - `effigy check:gpui`
   - `effigy regressions:native`
   - `effigy docs:check`
   - `effigy qa` once, headlessly, on the final rebased head
   - `git diff --check origin/main...HEAD`
3. Never substitute a windowed selector for missing evidence. If Effigy routes
   a required selector to a focus-taking/windowed runtime, stop and report the
   routing defect rather than running it.
4. Update only g15.008's native roster/register rows and write one August log
   naming every runtime's evidence and intentional binding difference. Do not
   edit roadmap status, dispatch, or generation front doors.
5. Push the selected branch and open a reviewable PR against current `main`.
6. In the PR body, link this handoff, g15.008, spec 067, all four component
   contracts, public Rust surface changes, evidence, validation, and unresolved
   items.
7. Report the PR URL and pushed SHA to the operator. Do not merge.

## Review and Merge Path

The orchestrator will independently inspect PR metadata, scope, commits,
public Rust API, component contracts, behaviour vectors, node composition,
mounted GPUI interactions, docs evidence, and gates. The canonical verdict
will be a PR comment. The worker addresses review findings and rebases when
asked. Only the orchestrator merges after acceptance.

Closeout authority remains with the orchestrator:

- `docs/roadmaps/g15/008-model-connection-family-native-completion.md`
- `docs/roadmaps/g15/README.md`
- `docs/roadmaps/generation-index.md`
- `docs/roadmaps/README.md`
- `docs/roadmaps/dispatch.md`
- `docs/roadmaps/g15/release-baseline-roster.md`
- `docs/roadmaps/g15/release-gap-register.md`
- the four component contracts and worker batch log

If blocked, leave the card and evidence honest and report the exact boundary.
Do not weaken the active-cohort claim to make the lane appear complete.
