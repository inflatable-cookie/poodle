---
title: g16.042 semantic interaction cues research worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-215612-g16-042-semantic-interaction-cues-research.md
base_required: pushed-main
tags: [coordination, handoff, worker, research, pr]
---

## What This Thread Was Doing

Research whether optional semantic sound or haptic cues belong in Poodle.
Produce one bounded dossier; no cue API or implementation is authorized.

## Why It Matters

Cues cross accessibility, host capability, preference, lifecycle, and native
ownership. A reusable semantic role may be valuable, but correctness must never
depend on sound and Poodle must not absorb product jingles or sample libraries.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Planning branch/base: pushed `main` at `229d40c5fef8dc91ff718018b722d8fee7acc764`
- Worker branch: `research/g16-042-semantic-interaction-cues`
- Worker worktree: launcher-managed; named fallback
  `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-042-semantic-interaction-cues-research`
- Required sibling worktree links: none
- Active milestone: `docs/roadmaps/g16/README.md`
- Ready card: `docs/roadmaps/g16/042-semantic-interaction-cues-research.md`
- Canonical refs: `docs/architecture/012-semantic-motion-policy.md` and
  `docs/contracts/001-working-rules.md`
- Owned surface: `docs/research/value-tracks/semantic-interaction-cues.md` only
- Integration ownership: orchestrator owns roadmap, card, front-door, triage,
  and `PAPERCUTS.md`; report friction instead of editing shared surfaces
- Parallel lanes: `g16.040`, `g16.041`, and `g16.043`; other research PRs may
  merge first; `g16.036` remains serial
- Worker profile: day-to-day research; non-frontier
- Frontier-worker justification: none
- Validation: `effigy docs:lint`; `git diff --check origin/main...HEAD`
- PR: base current `main`, head worker branch, URL pending
- Review/merge: orchestrator exact-head review; worker never merges

## Boundaries

Follow the card. Inspect pinned Cuelume/audio-engine evidence and authoritative
platform accessibility/capability guidance. Audit generalized states and real
host policies. Do not create `CuePolicy`, roles, synthesis, samples, haptic
support, contracts, source, or an accessibility promise. Product-specific
sound stays consumer-owned. Stop on an unframed policy or permission decision.

## Important Context

Architecture 012 is relevant lifecycle evidence, not proof that sensory policy
should mirror motion policy. Test missing devices, muted environments, overlap,
unlock, background behavior, failure, assistive technology, capture, and
determinism before recommending architecture, recipe, consumer ownership, or
rejection.

## Suggested Next Move

Run preflight, read the card, pin primary sources, then build the policy and
runtime-ownership comparison around accessibility rather than library API.

## Completion Protocol

Before broad reads, run `git rev-parse --show-toplevel`,
`git branch --show-current`, `git status --porcelain`, and
`git worktree list --porcelain`. Reuse a clean launcher-provided non-`main`
worktree. Otherwise follow `docs/contracts/005-agent-local-paths.md`; never
guess paths or discard dirty state. Fetch origin, require `HEAD == origin/main`,
require the planning base above to be an ancestor, and load this tracked
handoff from `HEAD`; stop on mismatch. Sibling links are none.

Write only the dossier, validate, commit, refresh from current `main` if another
PR merged, push, and open a PR. Link the card, sources, accessibility findings,
recommendation, unresolved operator gates, and validation. Do not merge.
Requested changes stay on this branch. Orchestrator owns closeout.
