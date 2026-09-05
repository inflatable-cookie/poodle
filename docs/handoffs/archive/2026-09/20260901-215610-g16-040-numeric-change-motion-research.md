---
title: g16.040 numeric change motion research worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-215610-g16-040-numeric-change-motion-research.md
base_required: pushed-main
tags: [coordination, handoff, worker, research, pr]
---

## What This Thread Was Doing

Research numeric display motion now that the shared motion policy has merged.
Produce one evidence-backed dossier; do not implement or promote an API.

## Why It Matters

Poodle has overlapping NumberFlow, Calligraph, and Transitions.dev evidence.
This lane decides whether those findings support a semantic Poodle role,
recipe-only guidance, static behavior, or rejection without pulling editing
controls into the motion system.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Planning branch/base: pushed `main` at `229d40c5fef8dc91ff718018b722d8fee7acc764`
- Worker branch: `research/g16-040-numeric-change-motion`
- Worker worktree: launcher-managed; named fallback
  `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-040-numeric-change-motion-research`
- Required sibling worktree links: none
- Active milestone: `docs/roadmaps/g16/README.md`
- Ready card: `docs/roadmaps/g16/040-numeric-change-motion-research.md`
- Canonical refs: `docs/architecture/012-semantic-motion-policy.md`,
  `docs/contracts/components/value-readout.md`,
  `docs/contracts/components/metric-tile.md`, and
  `docs/research/value-tracks/transitions-dev-catalogue.md`
- Owned surface: `docs/research/value-tracks/numeric-change-motion.md` only
- Integration ownership: the orchestrator owns roadmap, card, front-door,
  triage, and `PAPERCUTS.md` changes; report friction instead of editing them
- Parallel lanes: `g16.041`, `g16.042`, and `g16.043`; `g16.037`–`039` and
  `g16.044` await dossier review; `g16.036` remains a serial public-API lane
- Worker profile: day-to-day research; non-frontier
- Frontier-worker justification: none
- Validation: `effigy docs:lint`; `git diff --check origin/main...HEAD`
- PR: base current `main`, head worker branch, URL pending
- Review/merge: orchestrator exact-head review; worker never merges

## Boundaries

Follow the card exactly. Inspect durable primary or pinned NumberFlow and
Calligraph sources, reuse rather than repeat the Transitions.dev audit, compare
the named Poodle consumers, and cover active-cohort feasibility and motion
policy. Do not add components, roles, dependencies, contracts, source,
roadmaps, consumers, or editing-control behavior. Stop on an unresolved
product threshold that the card does not frame.

## Important Context

PR #124 merged the full/reduced/frozen authority at `369a24f8c`. Reduced and
frozen semantics, formatting ownership, accessibility, and rapid retargeting
must be assessed against that landed behavior. A product or library page is
mutable evidence; pin code/release citations where possible and record license
limits. The output is research, not promotion authority.

## Suggested Next Move

Run the preflight below, read the card and named refs, then build the dossier as
one coherent evidence pass. Report only when the dossier and PR are reviewable,
or earlier if a card-level decision is missing.

## Completion Protocol

Before broad reads, run `git rev-parse --show-toplevel`,
`git branch --show-current`, `git status --porcelain`, and
`git worktree list --porcelain`. Reuse a clean launcher-provided non-`main`
worktree. Otherwise follow `docs/contracts/005-agent-local-paths.md`; never
guess a worktree path or discard dirty state. Fetch origin, require
`HEAD == origin/main`, require the planning base above to be an ancestor, and
load this tracked handoff from `HEAD`; stop if it differs from the absolute
file. Required sibling links are none.

Read `AGENTS.md`, the milestone, card, and canonical refs. Write only the owned
dossier. Validate, commit meaningful work, rebase onto current `main` if a
sibling merged, push, and open a PR. The PR body links the card, sources,
recommendation, unresolved gates, and validation. Do not merge. Requested
changes return to this same branch. Closeout refs remain orchestrator-owned.
