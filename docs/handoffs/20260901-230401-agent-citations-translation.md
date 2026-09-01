---
title: Agent citations translation memo
kind: northstar-handoff
handoff_mode: planning-delegate
planning_mode: conversational-discovery
dispatch_authority: orchestrator
promotion_authority: orchestrator
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-230401-agent-citations-translation.md
base_required: pushed-main
tags: [planning, agent, citations, sources]
---

## Objective

Translate the accepted citations research into one decision memo for authored
semantic content, inline references, and a source list. Do not implement or
make raw AgentMessage markdown citation-aware.

## Settled Decisions — Do Not Re-Ask

- The operator chose a translation memo, not a hold or rejection.
- Agent citations remain a composition; provenance, verification, URL
  admission, redirects, authentication, and actions remain consumer-owned.
- Automatic numeric/markdown extraction, transcript-owned source records,
  streaming effects, and provider vocabulary remain rejected.
- No public primitive is admitted until the memo proves independent reuse.

## Required Packet

Create exactly `docs/triage/20260901-230401-agent-citations-translation.md`.
Resolve the smallest candidate split, authored content shape, mark interaction,
mark-to-source accessibility, unavailable targets, copied text, URL rendering
invariant, first consumer evidence, cohort boundary, and promotion or hold
oracles. Derive recommendations from the dossier and current contracts; do not
ask the operator again in this lane.

Run docs lint and range diff check; commit, push, and open a one-file PR.
