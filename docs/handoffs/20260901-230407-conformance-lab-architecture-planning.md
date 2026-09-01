---
title: Dedicated conformance lab architecture planning
kind: northstar-handoff
handoff_mode: planning-delegate
planning_mode: conversational-discovery
dispatch_authority: orchestrator
promotion_authority: orchestrator
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-230407-conformance-lab-architecture-planning.md
base_required: pushed-main
tags: [planning, conformance, lab, longhorn, gpui, capture]
---

## Objective

Plan the dedicated internal Poodle conformance-lab repository and its
short-lived native capture process. Do not create a repository, window, capture,
or dependency.

## Settled Decisions — Do Not Re-Ask

- The lab belongs in a dedicated repository, not a Longhorn example.
- Native GPUI capture uses one operator-approved non-activating process per
  fixture, not a long-running sidecar.
- Poodle packages never depend on the lab. Named fixtures are bounded adapters,
  not a universal scene schema. Default QA/CI remains headless.

## Required Packet

Create exactly `docs/triage/20260901-230407-conformance-lab-architecture.md`.
Define repository ownership/bootstrap authority, Longhorn control boundary,
fixture manifest, process protocol, capture lifecycle, focus guarantees,
artifact/provenance retention, security, manual/windowed approval, MVP tranche,
validation, and stop conditions. Do not ask the operator again.

Run docs lint and range diff check; commit, push, and open a one-file PR.
