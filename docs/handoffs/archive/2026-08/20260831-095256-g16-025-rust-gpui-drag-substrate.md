---
title: g16.025 Rust and GPUI drag substrate worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle drag-and-drop substrate
created: 2026-08-31
updated: 2026-08-31
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260831-095256-g16-025-rust-gpui-drag-substrate.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, drag-drop, rust, gpui]
---

## What This Thread Was Doing

The orchestrator merged `g16.024` in PR #107. The paired semantic kernel, web
controller, EditableList, and Tree now prove the drag lifecycle through the
web runtime. This dispatch owns the next serial card only: `g16.025`, which
projects the shared Rust kernel through renderer-neutral Node construction and
stock crates.io GPUI 0.2.2.

This is one bounded implementation lane. No transcript or second prompt is
part of the authority chain.

## Why It Matters

Poodle already has working GPUI payload channels, but they use backend-owned
session state rather than the shared Rust semantic kernel and cannot provide
independent controller scopes. This card converges that path without throwing
away real mounted behavior, inventing a GPUI fork, or making false touch/pen
claims.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `c48c5dadf38dc095a13ae84e365221b9cc25e1a1`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  the planning base before this handoff was created.
- **Planning checkout:** clean at the planning base before the promotion batch.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included in the dispatch commit:** architecture 011,
  spec 069, merged card/log 024, ready card 025 with capability decision and
  review oracle, and the g16/front-door continuation state.
- **Worker branch:** suggested manual fallback
  `t3code/g16-025-rust-gpui-drag-substrate`; use a clean launcher-provided
  non-`main` branch when supplied.
- **Worker worktree:** suggested manual fallback
  `/Users/tom/.t3/worktrees/poodle/g16-025-rust-gpui-drag-substrate`; use the
  launcher-provided worktree when supplied.
- **Worktree creation command:** launcher-managed normally. Manual fallback
  only after the completion protocol permits it:
  `git worktree add /Users/tom/.t3/worktrees/poodle/g16-025-rust-gpui-drag-substrate -b t3code/g16-025-rust-gpui-drag-substrate origin/main`.
- **Worker worktree policy:** follow the Completion Protocol; launcher
  worktree first, named/manual fallback only when required.
- **Required sibling worktree links:** none.
- **Active spec lane:**
  `docs/specs/069-dependable-drag-and-drop-substrate.md`.
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`.
- **Ready cards, in order:**
  `docs/roadmaps/g16/025-drag-drop-rust-gpui-substrate.md`.
- **Allowed runway:** card 025 only, then stop.
- **Remaining card budget:** one card.
- **Dispatch topology:** serial behind merged card 024 and before card 026.
- **Parallel safety check:** no parallel worker may edit the Rust drag kernel,
  Node interaction vocabulary, poodle-render drag builders, GPUI backend drag
  path, native mounted regressions, or parity ledger during this run.
- **Canonical refs:**
  `docs/architecture/011-drag-and-drop-substrate.md`,
  `docs/specs/069-dependable-drag-and-drop-substrate.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/architecture/001-poodle-system-shape.md`, and the Tabs, EditableList,
  Tree, and ModelCatalogueEditor component contracts.
- **Review oracle:** card 025 `## Review Oracle`.
- **Model capability profile:** frontier coding model with high reasoning for a
  public native substrate, lifecycle ownership, identity, and renderer seams.
- **Tool/runtime restrictions:** headless only. Never run `*-windowed`, native
  visual, or Jetstream preview/QA selectors. Do not add a GPUI fork, platform
  input beneath GPUI, release mutation, workflow edit, or sibling change.
- **Required validation:** focused Rust/kernel/render/backend tests; named
  mounted GPUI regressions; `effigy probe:gpui-specimens`; `effigy
  regressions:native`; ledger tests/check; `effigy ci:rust`; `effigy
  ci:native`; `effigy docs:check`; one final headless `effigy qa`; and `git
  diff --check origin/main...HEAD`.
- **PR base/head:** `main` to the selected worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation and orchestrator review.
- **Merge authorisation:** none. The worker must not merge.

## Boundaries

- **In scope:** card 025 only: renderer-neutral drag source/target vocabulary;
  poodle-render construction; a reusable public GPUI controller/provider path;
  stock GPUI mouse/keyboard translation, hit testing, capture-equivalent
  movement, focus, announcements, preview, cancellation, and cleanup; custom
  fixtures; representative component convergence; honest ledger changes; one
  execution log and closeout.
- **Out of scope:** Svelte/React, touch or pen implementation beneath GPUI,
  GPUI forks, platform-specific AppKit/Win32/Linux input, cross-window bridges,
  Tabs public web migration, DockRegion migration, files, drag-out, continuous
  value gestures, tokens, versions, releases, workflows, and siblings.
- **Outcome shape:** one complete contract-valid native substrate and migration
  slice, with real mounted proof and a reviewable PR. Diagnosis-only or a
  controller that components cannot consume is incomplete.
- Reuse the landed Rust semantic kernel. Extend it only for a focused proven
  defect; do not create a second lifecycle in Node or GPUI.
- Converge the existing GPUI `drag_payload` / hover / leave / drop / end path.
  Preserve useful stock `on_drag`, `on_drag_move`, `on_mouse_up_out`, Escape,
  focus, and mounted component behavior.
- Expose immutable native input capabilities. Stock GPUI certifies mouse,
  keyboard, and in-window capture-equivalent movement. Pen, touch, and
  device-originated pointer cancellation remain unsupported. Mouse synthesis
  is never evidence for them.
- Each provider/controller owns its session. Remove the current collision-prone
  backend-global session only after two-scope proof passes.
- Keep Node renderer-neutral: no GPUI geometry, events, entities, windows, or
  durable mutation cross that boundary.
- Preserve current public component callbacks. Move a ledger cell only after a
  named real-dispatch mounted test proves the whole authored behavior.
- Keep Jetstream compile maintenance mechanical and renderer-neutral. Do not
  run or claim Jetstream behavior.
- Work only in the clean worker worktree selected by the Completion Protocol.
  Never edit the planning checkout or an unrelated dirty checkout.
- Do not merge. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** `g16.021` landed the paired semantic kernel in PR #96;
  `g16.022` landed the public web controller in PR #101; `g16.023` migrated
  EditableList in PR #104; `g16.024` migrated Tree and nested auto-scroll in
  PR #107. Card 025 is the first native controller projection.
- **Why the card is ready:** all dependencies are merged; the exact GPUI 0.2.2
  input surface was inspected; the operator approved the desktop capability
  boundary; architecture/spec/card now state the scope, acceptance, stop
  conditions, validation, and adversarial oracle.
- **Decisions and preferences:** use crates.io GPUI 0.2.2 only. Full touch
  remains mandatory on browsers, Electron, and Tauri webviews. GPUI mouse and
  keyboard are certified now; pen, touch, and device-cancel are named debt.
  Stock typed `on_drag_move` is the capture-equivalent in-window mechanism.
- **Open tensions:** existing GPUI payload state is thread-local and
  backend-global; the semantic kernel is scoped and effect-driven. Component
  callbacks and controlled host rebuilds must stay stable while ownership
  moves. Stop if this needs a public component callback break, GPUI fork, or
  Node exposure of backend types.
- **Report after:** first, the renderer-neutral controller/provider vocabulary
  plus custom two-scope fixture passes; second, representative component
  migrations and the full headless native board; immediately on any stop
  condition.
- **Report to:** the operator, who will relay progress and the PR URL to the
  orchestrator.

The motion-learning and Longhorn conformance-lab triage notes remain outside
this lane.

## Suggested Next Move

Run the Completion Protocol preflight before broad reads. Then read
`AGENTS.md`, card 025, architecture 011, spec 069, the Node/render architecture,
the four named component contracts, the g16.021 kernel log, and the g16.024
closeout log. Inspect the existing Rust kernel and current GPUI payload session
before selecting edits. Start with the public native controller/provider and
two-scope custom fixture, not a component migration.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run only `git rev-parse --show-toplevel`, `git branch --show-current`, `git
   status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch. Do not compare generated names with the fallback or
   create another worktree.
3. If the launcher supplied `main` or a dirty checkout, stop and report it. If
   a manual fallback is genuinely required, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and create a unique worktree/branch there
   from `origin/main`. Never guess `/tmp`, clean, reset, stash over, or discard
   another checkout.
4. In the selected worktree, record this handoff's relative path. Run
   `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch
   origin`. Confirm `HEAD == origin/main`, confirm `git merge-base
   --is-ancestor c48c5dadf38dc095a13ae84e365221b9cc25e1a1 HEAD`, and confirm
   this handoff exists in `HEAD`. Load it with `git show HEAD:<relative-path>`.
   If the absolute file differs, stop; the tracked blob is canonical.
5. Required sibling links are `none`; make no sibling-path mutation.
6. Read the named authority, card, and repo-local Effigy skill. Use `effigy
   tasks` only after the worktree decision to confirm selector names.

### While you work

- Execute card 025 in coherent substrate, adapter, and migration/test chunks.
- Report after each named meaningful chunk with changed files, validation run,
  ledger movement, remaining work, risks, and blockers.
- Use real mounted GPUI dispatch. Direct handler invocation may support unit
  tests but cannot prove a mounted acceptance row.
- Try to preserve existing good payload behavior while replacing lifecycle
  ownership. Do not leave dual active controllers or compatibility aliases.
- Stop on a missing kernel contract, public callback break, GPUI fork need,
  platform-input expansion, false capability claim, Node/backend leak, or
  validation result that changes the plan.

### When the assigned runway is complete

1. Run every required validation named above. Everything stays headless.
2. Falsify the diff against card 025's review oracle: two providers, nested
   overlap, live eligibility change, source/target removal, host rebuild,
   outside release, keyboard drop, repeated Escape, exactly-once callbacks,
   and false pen/touch claims. Map every claim to proof.
3. Update card 025, one August execution log, relevant contracts, honest ledger
   cells/checker, g16/front-door continuation state, and the single next task.
4. Run `git diff --check origin/main...HEAD`, commit meaningful chunks, push the
   selected worker branch, and open a PR against current `main`.
5. The PR body must link architecture 011, spec 069, card 025, changed public
   surfaces, capability matrix, custom and component mounted evidence, ledger
   result, validation, and execution log.
6. Report the PR URL and evidence to the operator. Do not merge or start 026.

### Review and merge path

The orchestrator reviews the PR independently against the canonical refs,
current diff, checks, review oracle, mounted dispatch tests, and capability
claims. Because worker and orchestrator share a GitHub identity, the verdict is
a PR comment. Requested changes remain on this branch; a planning change
returns to planning first. The operator must explicitly authorise merge.

- **Requested changes:** none; first review pending.
- **Closeout refs:** card 025, its August execution log, architecture 011, spec
  069, affected component contracts, g16 README, roadmap front door, parity
  ledger/checker, and one next-task state.

### Handoff closeout

Leave the card, roadmap, log, capability matrix, ledger, and next-task state
honest. If blocked, record the exact blocker and stop instead of marking the
runway complete.
