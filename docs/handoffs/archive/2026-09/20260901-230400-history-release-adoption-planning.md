---
title: HistoryCenter release and adoption planning
kind: northstar-handoff
handoff_mode: planning-delegate
planning_mode: conversational-discovery
dispatch_authority: orchestrator
promotion_authority: orchestrator
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-230400-history-release-adoption-planning.md
base_required: pushed-main
tags: [planning, release, history-center, loophole, papercuts]
---

## Objective

Plan the separately authorized publication of the corrected Poodle package and
the subsequent Loophole pin/adoption. Produce one decision packet; perform no
version, tag, publish, or sibling-repository mutation.

## Settled Decisions — Do Not Re-Ask

- The operator authorized the release/adoption lane on 2026-09-01.
- g16.033 and its packed v3 `HistoryEntry` proof are complete.
- Poodle publication precedes Loophole pin movement and rejection mapping.
- Longhorn `AlreadyAtTarget` is complete and must not be reopened.
- Poodle retains review/merge authority for Poodle work; Loophole owns adoption.

## Required Packet

Create exactly `docs/triage/20260901-230400-history-release-adoption-decision.md`.
Fix candidate versions, package set, release selectors, provenance/rollback
receipts, publication stop conditions, and the repository handoff boundary.
Record the capitalized `Papercuts` worker label for any later papercut worker.
Use current release docs and Effigy selectors. Do not ask product questions.

Run `effigy docs:lint` and `git diff --check origin/main...HEAD`; commit, push,
and open a one-file PR. The orchestrator owns review, promotion, release
authority checks, and every external mutation.
