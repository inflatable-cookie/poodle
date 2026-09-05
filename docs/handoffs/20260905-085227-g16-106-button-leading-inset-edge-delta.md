---
title: g16.106 Button leading-inset edge delta worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-05
updated: 2026-09-05
handoff_path: /Users/tom/.paseo/worktrees/1ugbsx1t/g16-106-button-leading-inset-edge-delta/docs/handoffs/20260905-085227-g16-106-button-leading-inset-edge-delta.md
base_required: pushed-main
tags: [coordination, handoff, worker, g16, g16.106]
---

## What This Thread Was Doing

Dispatch the Chatterbox-approved g16.106 Button leading-inset edge delta
diagnosis. The card and current dispatch manifest are authoritative.

This dispatches one bounded implementation lane. No transcript or second
prompt is part of the authority chain.

## Why It Matters

The lab Button batch reports a 1.0 vs 0.5 logical-px leading edge on two
fixtures. Poodle must prove whether that is a rounding defect in
`poodle-render` or a GPUI rasterisation delta, then leave either an exact
inset repair or a contracted known delta.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `inflatable-cookie/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `9481cc95dbd65c1dff8c73a6b74b9504cf19b077`
- **Pushed main verification:** `HEAD == origin/main` at that SHA
- **Planning checkout:** clean launcher worktree; do not edit
  `/Users/tom/Dev/projects/poodle`
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** card
  `docs/roadmaps/g16/106-button-leading-inset-edge-delta.md`; dispatch
  manifest revision that promoted g16.106–108
- **Worker branch:** `worker/g16.106-button-leading-inset-edge-delta`
- **Worker worktree:** `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-106-button-leading-inset-edge-delta`
- **Worktree creation command:** launcher-provided; reuse this worktree
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required.
- **Required sibling worktree links:** none
- **Active spec lane:** none
- **Roadmap milestone:** `docs/roadmaps/g16/`
- **Ready cards, in order:** `docs/roadmaps/g16/106-button-leading-inset-edge-delta.md`
- **Allowed runway:** g16.106 only
- **Remaining card budget:** one card
- **Coordinator agent ID:** parent orchestrator; notifyOnFinish
- **Delivery route:** coordinator-attached child with `notifyOnFinish: true`
- **Dispatch topology:** concurrent with g16.107, g16.108, g16.097
- **Parallel safety check:** no shared mutable scope with 107/108 except
  append-only `PAPERCUTS.md`; reserved closeout is coordinator-owned
- **Surfaces this lane owns:** `packages/render/src/button.rs`,
  `packages/render/src/presentation.rs` (only if `rem_to_px` is the cause),
  their tests, `test/visual/button-comparison/policy.ts` (one role finding,
  if contracted), ledger known-delta generator inputs, execution log,
  `PAPERCUTS.md` (append only)
- **Integration ownership:** coordinator at merge owns `g16/README.md`,
  `generation-index.md`, `dispatch.md`
- **Merge ordering:** same-repository PRs merge one at a time
- **Canonical refs:** `docs/architecture/001-poodle-system-shape.md`;
  `docs/contracts/001-working-rules.md`; `docs/contracts/components/button.md`
- **Review oracle:** the g16.106 card table
- **Model capability profile:** capable coding model, medium reasoning (Rust)
- **Worker provider/model identity:** Grok 4.6
- **Frontier-worker justification:** none
- **Tool/runtime restrictions:** never run `*-windowed` selectors
- **Required validation:** `cargo test -p poodle-render`,
  `effigy regressions:native`, `effigy docs:check`,
  `git diff --check origin/main...HEAD`
- **PR base/head:** `main` ← `worker/g16.106-button-leading-inset-edge-delta`
- **PR URL:** pending
- **Review state:** awaiting review after PR open
- **Merge path:** orchestrator after accepted review of the current head and
  passing required checks

## Boundaries

Please keep this run inside the named runway:

- **In scope:** g16.106 diagnosis and exact-inset repair or contracted
  `gpui-snaps-subpixel-edge`
- **Out of scope:** tolerance tuning, lab edits, other components, GPUI
  paint-path changes, node-vocabulary expansion, reserved closeout surfaces
- **Outcome shape:** issue-fix. Diagnosis through the node inventory, then
  the smallest in-scope repair or honest contract.
- Do not invent architecture, change contracts beyond the contracted known
  delta, widen the roadmap, or choose an unresolved product decision.
- Write only inside surfaces this lane owns. Leave integration closeout to
  the coordinator.
- Work only in this clean worker worktree. Never edit the planning checkout.
- Do not merge the PR.

## Important Context

- **Planning lineage:** lab Button batch at `fb839407b`; triage
  `docs/triage/20260904-155753-lab-button-run-findings.md` Finding 1
- **Why these cards are ready:** operator accepted the 2026-09-05 promotion
- **Decisions and preferences:** Svelte is the reference; do not tune
  comparator tolerances
- **Open tensions:** if the cause is a new node capability or GPUI paint,
  stop and escalate to Chatterbox
- **Report after:** diagnosis plus the matching outcome, validation, PR
- **Report to:** the owning coordinator through the linked child result

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then read
`AGENTS.md`, the card, and canonical refs from this worktree. Reproduce both
fixtures through the node inventory and either fix the emitted inset or
contract the raster delta.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad
   reads, run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and
   its branch is not `main`, accept it as the launcher-provided worktree.
3. From the selected worktree, fetch origin, confirm `HEAD == origin/main`
   at the planning base, and load this tracked handoff from `HEAD`.
4. Required sibling links: none.
5. Read the card, `AGENTS.md`, and canonical refs.
6. Run the repo's cheap orientation checks and record what you actually ran.

### While you work

- Execute g16.106 only.
- Own reproduce, diagnose, implement or contract, clean up temporary
  diagnostics, validate, and evidence.
- Stop on missing contracts, ambiguous intent, scope expansion, or a
  validation result that changes the plan.

### When the assigned runway is complete

1. Run the required final validation.
2. Falsify the diff against the card's review oracle.
3. Update card/log evidence.
4. Push the worker branch and open one PR against current `main`. Do not merge.

### Review and merge path

The orchestrator launches an independent review child in this worker
workspace. Merge belongs to the orchestrator after accepted exact-head
review and passing checks.

- **Closeout refs:** card, this log, reserved front doors at merge
