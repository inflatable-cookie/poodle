---
title: EditableLabel editing-model decision
kind: northstar-handoff
handoff_mode: planning-delegate
planning_mode: conversational-discovery
dispatch_authority: orchestrator
promotion_authority: orchestrator
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-230406-editable-label-decision-planning.md
base_required: pushed-main
tags: [planning, editable-label, editing, focus]
---

## Objective

Resolve EditableLabel's activation, draft, commit, cancel, blur, and focus
ownership as one cross-runtime decision packet. Do not implement.

## Settled Decisions — Do Not Re-Ask

- The operator chose to plan the decision rather than keep the lane blocked or
  accept current behavior by default.
- TextInput's accepted normalization, max-length, composition, and focus
  boundaries remain authoritative where reused.
- Current Enter, Escape, and blur/Tab evidence is input, not contract authority
  where runtimes disagree.

## Required Packet

Create exactly `docs/triage/20260901-230406-editable-label-decision.md`.
Audit all active runtimes and consumers, then recommend the public controlled/
uncontrolled shape, draft ownership, activation triggers, commit/cancel/blur
law, unchanged-result behavior, async policy, focus/selection restoration,
accessibility, migration, parity deltas, oracle, and implementation card split.
Do not ask the operator again; surface a reviewable recommendation.

Run docs lint and range diff check; commit, push, and open a one-file PR.
