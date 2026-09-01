---
title: g16.043 in-place toast lifecycle research worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-215613-g16-043-in-place-toast-lifecycle-research.md
base_required: pushed-main
tags: [coordination, handoff, worker, research, pr]
---

## What This Thread Was Doing

Research an additive pending-to-settled toast lifecycle over one stable identity.
Produce a state-machine dossier; do not import Sonner's imperative helper or
change ToastHost/ToastStack.

## Why It Matters

Async operations often need one visible record to move from pending to success
or failure. That can easily blur operation ownership, announcements, focus,
expiry, replacement, and motion. Poodle needs those semantics settled first.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Planning branch/base: pushed `main` at `229d40c5fef8dc91ff718018b722d8fee7acc764`
- Worker branch: `research/g16-043-in-place-toast-lifecycle`
- Worker worktree: launcher-managed; named fallback
  `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-043-in-place-toast-lifecycle-research`
- Required sibling worktree links: none
- Active milestone: `docs/roadmaps/g16/README.md`
- Ready card: `docs/roadmaps/g16/043-in-place-toast-lifecycle-research.md`
- Canonical refs: `docs/architecture/012-semantic-motion-policy.md`,
  `docs/contracts/components/toast-host.md`, and
  `docs/contracts/components/toast-stack.md`
- Owned surface: `docs/research/value-tracks/in-place-toast-lifecycle.md` only
- Integration ownership: orchestrator owns roadmap, card, front-door, triage,
  and `PAPERCUTS.md`; report friction instead of editing shared surfaces
- Parallel lanes: `g16.040`, `g16.041`, and `g16.042`; other research PRs may
  merge first; `g16.036` remains serial
- Worker profile: day-to-day research; non-frontier
- Frontier-worker justification: none
- Validation: `effigy docs:lint`; `git diff --check origin/main...HEAD`
- PR: base current `main`, head worker branch, URL pending
- Review/merge: orchestrator exact-head review; worker never merges

## Boundaries

Follow the card. Pin Sonner primary evidence, audit current ToastHost/ToastStack
stores and two real async consumers, and sketch lifecycle/identity/focus/timer
ownership across the active cohort. Do not add fields, callbacks, promises,
components, source, contracts, or a public state machine. Stop if consumer or
announcement authority cannot be established from evidence.

## Important Context

PR #124 is now the exact Toast motion baseline. It leaves items externally
controlled: a dismiss request does not make a still-supplied row inert, and
visual lifecycle never owns expiry. Preserve those facts while studying
same-id updates, progress churn, duplicate completion, cancellation, focused
actions, errors, removal, and unmount.

## Suggested Next Move

Run preflight, read the card/contracts, pin Sonner, trace current Poodle stores
and consumers, then derive the smallest honest state-machine recommendation.

## Completion Protocol

Before broad reads, run `git rev-parse --show-toplevel`,
`git branch --show-current`, `git status --porcelain`, and
`git worktree list --porcelain`. Reuse a clean launcher-provided non-`main`
worktree. Otherwise follow `docs/contracts/005-agent-local-paths.md`; never
guess paths or discard dirty state. Fetch origin, require `HEAD == origin/main`,
require the planning base above to be an ancestor, and load this tracked
handoff from `HEAD`; stop on mismatch. Sibling links are none.

Write only the dossier, validate, commit, refresh from current `main` if another
PR merged, push, and open a PR. Link the card, pinned evidence, consumer audit,
state-machine recommendation, unresolved gates, and validation. Do not merge.
Requested changes stay on this branch. Orchestrator owns closeout.
