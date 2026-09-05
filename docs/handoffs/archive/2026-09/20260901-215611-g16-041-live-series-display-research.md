---
title: g16.041 live series display research worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-215611-g16-041-live-series-display-research.md
base_required: pushed-main
tags: [coordination, handoff, worker, research, pr]
---

## What This Thread Was Doing

Research whether a live windowed series belongs in Poodle, a current component,
or its consumers. Produce one dossier and benchmark plan; implement nothing.

## Why It Matters

Streaming data can look like MetricTile, WaveformDisplay, a chart, or product
chrome while owning very different data and performance contracts. Poodle
needs that boundary before any renderer capability or public primitive exists.

## Current State

- Repository: `/Users/tom/Dev/projects/poodle`
- Planning branch/base: pushed `main` at `229d40c5fef8dc91ff718018b722d8fee7acc764`
- Worker branch: `research/g16-041-live-series-display`
- Worker worktree: launcher-managed; named fallback
  `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-041-live-series-display-research`
- Required sibling worktree links: none
- Active milestone: `docs/roadmaps/g16/README.md`
- Ready card: `docs/roadmaps/g16/041-live-series-display-research.md`
- Canonical refs: `docs/architecture/012-semantic-motion-policy.md`,
  `docs/contracts/components/metric-tile.md`, and
  `docs/contracts/components/waveform-display.md`
- Owned surface: `docs/research/value-tracks/live-series-display.md` only
- Integration ownership: orchestrator owns roadmap, card, front-door, triage,
  and `PAPERCUTS.md`; report friction instead of editing shared surfaces
- Parallel lanes: `g16.040`, `g16.042`, and `g16.043`; other research PRs may
  merge first; `g16.036` remains serial
- Worker profile: day-to-day research; non-frontier
- Frontier-worker justification: none
- Validation: `effigy docs:lint`; `git diff --check origin/main...HEAD`
- PR: base current `main`, head worker branch, URL pending
- Review/merge: orchestrator exact-head review; worker never merges

## Boundaries

Follow the card. Pin Liveline primary evidence; audit named Poodle surfaces and
real consumers; compare SVG, canvas, renderer-neutral polyline, and static-host
routes with explicit budgets. Do not introduce `LiveSeries`, a charting layer,
canvas/path exceptions, scrubbing, source edits, contracts, or promotion.
Stop if no real consumer evidence can be inspected honestly.

## Important Context

Merged PR #124 supplies the motion/capture boundary but does not authorize a
new rendering capability. The dossier must separate static sparklines, audio
waveforms, full charts, and streaming windows, and must name who owns point
identity, time, gaps, downsampling, pause, and accessibility summaries.

## Suggested Next Move

Run preflight, read the card and refs, pin Liveline, then build the consumer and
renderer comparison before choosing extend/add/consumer-owned/reject.

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
budgets, recommendation, gates, and validation. Do not merge. Requested changes
stay on this branch. Orchestrator owns closeout.
