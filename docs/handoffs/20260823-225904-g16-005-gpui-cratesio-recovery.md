---
title: g15.059 GPUI crates.io recovery worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: complete
owner: Poodle native runtime
created: 2026-08-23
updated: 2026-08-24
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260823-225904-g16-005-gpui-cratesio-recovery.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, gpui, cratesio]
---

## What This Thread Was Doing

Consumer adoption exposed a release defect: Poodle `v0.2.1` changed its public
GPUI dependency from crates.io to an `inflatable-cookie/zed` fork so an internal
capture tool could use unpublished headless-renderer APIs. Cargo treats those
sources as different crate identities, so a normal consumer using crates.io
GPUI cannot pass GPUI types through Poodle.

The orchestrator reviewed the boundary and ran a disposable decision prototype.
Stock crates.io GPUI 0.2.2 can render and capture a real window with
`focus: false` while the foreground application remains unchanged. This worker
restores that public source boundary and replaces the fork-only capture
transport honestly.

## Why It Matters

Longhorn and other native consumers must be able to use Poodle beside ordinary
crates.io GPUI without aligning themselves to a private fork. Optional visual
evidence must not choose the crate identity exposed by a public runtime package.

This recovery blocks the `v0.2.2` candidate and every unfinished consumer
adoption lane.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base before this handoff commit:**
  `b118b317401a47d13f8f6b8e93e03caac0d64efc`
- **Pushed main verification:** local `HEAD` and `origin/main` matched at the
  planning base.
- **Planning checkout:** clean before this handoff was created.
- **Posture:** `strict-ready`; consumer rollout is paused behind this recovery.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  file activates the worker-only worktree preflight.
- **Worker branch:** `t3code/g16-005-gpui-cratesio-recovery`.
- **Worker worktree:** use the clean registered non-`main` worktree supplied by
  the launcher, regardless of its generated path or branch name.
- **Manual worktree command:** none pre-authorised. If the launcher context is
  unusable, follow the fallback in `## Completion Protocol` using the
  operator-selected `AGENTS_WORKTREE_CONTAINER_DIR` only.
- **Active policy:**
  `docs/specs/022-packaging-versioning-and-release-channel-rules.md`.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`.
- **Ready card:** `docs/roadmaps/g15/059-gpui-cratesio-recovery.md`.
- **Allowed runway:** `g15.059` only.
- **Remaining card budget:** one card. Do not start `g15.060`.
- **Dispatch topology:** serial. Release preparation and consumer adoption wait
  for review of this boundary.
- **Canonical refs:** `AGENTS.md`, `docs/contracts/001-working-rules.md`,
  spec 022, the g15 runway, and
  `docs/research/gpui-cratesio-nonactivating-capture.md`.
- **Historical evidence:**
  `docs/research/gpui-offscreen-capture-feasibility.md`, g15 cards `044`–`047`
  and `051`–`052`, plus their execution logs.
- **Model capability profile:** frontier coding model, high reasoning. This is
  a public dependency and validation-boundary correction.
- **Tool/runtime restrictions:** headless worker validation only. Never run a
  `*-windowed`, native-visual, visible GPUI, Jetstream preview/QA, release,
  workflow, tag, or publication command.
- **Required validation:** focused dependency/source checks and dual-dependency
  compile proof, focused GPUI tests, `effigy ci:native`, `effigy qa`,
  `effigy docs:check`, and `git diff --check origin/main...HEAD`.
- **PR base/head:** current pushed `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation.
- **Merge authorisation:** absent. Push a PR and stop.

## Boundaries

Implement exactly `g15.059`: restore crates.io GPUI 0.2.2 in the active node
backend and preview graphs, mechanically reverse the newer API migration,
replace the fork-only capture transport with a clearly named non-activating
window-server transport, and add deterministic source and consumer-compatibility
proof.

Preserve current component rendering, presentation cascade, focus rings,
fixture inventory, receipt integrity, comparison policy, specimens, and
headless interaction evidence. Use current `main` as authority; do not blindly
revert whole files to their pre-`g15.045` forms.

Out of scope:

- version changes, `0.2.2` release notes, candidate preparation, tags, or
  publication — `g15.060` and `g15.061` own those;
- `.github/workflows/` changes;
- a GPUI fork, Git patch, local Cargo override, compatibility alias, or second
  GPUI crate identity;
- a new renderer, component API change, comparator tolerance change, fixture
  change, or baseline refresh;
- running the real windowed diagnostic. The orchestrator owns that one
  explicitly approved review run.

Do not edit the dispatch ledger or active-roadmap status. Record implementation
evidence in the `g15.059` log; the orchestrator closes planning after review.
Do not merge the PR.

## Important Context

- Stock GPUI 0.2.2 has no public scene readback or headless renderer. Do not
  preserve the word `offscreen` by making a false claim.
- The accepted prototype opened one `focus: false`, `show: true` window, never
  called `App::activate`, sampled the foreground application every 50 ms, and
  captured its own window id successfully without one focus change.
- Window capture still needs a macOS window server and Screen Recording
  permission. It must remain outside default QA, CI, and release gates.
- No path may activate the app, call `makeKeyAndOrderFront`, use System Events
  activation, capture the desktop/region, or silently fall back.
- Rename the capture binary, selector, and receipt schema truthfully. Poodle is
  pre-1.0; do not leave aliases for the old false name.
- Retain the exact 18-case Button inventory and comparison rules. The worker
  should make the transport testable without executing the windowed path, then
  record the exact operator command needed for review.
- `g15.045` measured the forward API migration as 17 mechanical errors across
  nine files. Later g15 work touched some of those files, so reverse only the
  API differences and prove no behavioural loss.
- The source policy should fail if an active manifest or lock resolves `gpui`
  or `gpui_platform` from Git. It should not reject unrelated historical docs.
- Report after the dependency/API-reversal batch and again after the capture,
  policy, and evidence batch. Stop if the change needs architecture beyond the
  card.

## Suggested Next Move

Read this file first and run the four-command worktree preflight below before
broad repository reads. Reuse the launcher-provided worktree when it is clean,
registered, and non-`main`; do not create another because its generated name
differs from the placeholder branch.

Then read the card, spec 022, the new decision-prototype record, the current
manifests, and the g15 migration history. Start with the dependency graph and
mechanical API reversal. Get the full headless native board green before
rewiring capture so a renderer regression cannot hide inside the tooling
change.

## Completion Protocol

### Before editing

1. This file's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Run only:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. Reuse a clean registered non-`main` launcher worktree. If the launcher
   supplied `main`, a dirty checkout, or an unregistered checkout, stop and
   report it. Do not clean, reset, stash, or silently create a second worktree.
3. Fetch `origin`; require `HEAD == origin/main`; require planning base
   `b118b317401a47d13f8f6b8e93e03caac0d64efc` to be an ancestor; confirm this
   handoff exists in `HEAD`.
4. Read `AGENTS.md`, the repo-local Effigy skill, `g15.059`, the g15 README,
   spec 022, both GPUI capture research records, and the exact current source
   files the card names.
5. Run `effigy tasks` and the smallest clean dependency/native starting checks.

If no launcher worktree exists and the operator explicitly asks for manual
fallback, read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`,
and create a unique worktree/branch under that container from `origin/main`.
Ask if the key is absent. Never use `/tmp`, `TMPDIR`, or a guessed path.

### While working

- Execute only `g15.059`; keep commits aligned with the dependency/API batch
  and the capture/policy/evidence batch.
- Report meaningful chunks through the operator with changed files, validation
  actually run, remaining work, and blockers.
- Use Effigy selectors for validation. Do not run any windowed selector even if
  the implementation adds one.
- Stop on every card stop condition. Do not invent a new GPUI, capture,
  component, release, or compatibility architecture.

### Completion

1. Run the card's headless final validation: focused source-policy and
   dual-dependency compile proof, focused node-backend/preview tests,
   `effigy ci:native`, `effigy qa`, `effigy docs:check`, and
   `git diff --check origin/main...HEAD`.
2. Never run a windowed/native-visual/Jetstream preview or QA selector, release
   mutation, workflow, tag, or publication command.
3. Write
   `docs/logs/2026-08/20260823-g16-005-gpui-cratesio-recovery.md` with changed
   graphs and APIs, preserved behaviour, source-policy proof, exact headless
   validation, capture limitations, and the exact orchestrator-owned windowed
   review command.
4. Inspect the diff against the scoped and forbidden surfaces, commit, push,
   and open one PR against current `main`.
5. Link the card, spec, research, log, validation, and operator diagnostic in
   the PR body. Report the PR URL and stop. Never merge.

### Review and closeout

The orchestrator independently reviews the dependency identities, mechanical
API reversal, retained headless evidence, capture naming and focus boundary,
source-policy test, and PR checks. After code review, the orchestrator asks for
explicit operator approval and runs the one windowed diagnostic itself. Formal
self-approval may be unavailable; the orchestrator's PR comment is the
canonical review record.

After explicit operator merge authority and a green review gate, the
orchestrator merges, closes `g15.059`, and advances only `g15.060`. The release
candidate and publication remain separate work.
