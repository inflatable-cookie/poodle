---
title: Toast same-id update convention translation
kind: northstar-handoff
handoff_mode: planning-delegate
planning_mode: conversational-discovery
dispatch_authority: orchestrator
promotion_authority: orchestrator
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-230403-toast-update-convention-translation.md
base_required: pushed-main
tags: [planning, toast, lifecycle, convention]
---

## Objective

Translate the accepted consumer-owned Toast lifecycle recommendation into a
narrow host update convention. Do not add a pending/settled field, promise
helper, required Progress slot, or second toast creation API.

## Settled Decisions — Do Not Re-Ask

- One stable id moves from sticky pending copy to settled copy by host update.
- Host/domain owns operation state and expiry; visuals never own timers.
- The operator authorized planning the convention, not a public lifecycle API.

## Required Packet

Create exactly `docs/triage/20260901-230403-toast-update-convention.md`.
Settle uniqueness, clearing/restarting the timer, discrete settlement
announcements versus numeric progress, action-focus replacement/removal,
durable danger remediation, native alert semantics, required oracles, and the
smallest contract/specimen destinations. Make an evidence-backed recommendation
without asking the operator again.

Run docs lint and range diff check; commit, push, and open a one-file PR.
