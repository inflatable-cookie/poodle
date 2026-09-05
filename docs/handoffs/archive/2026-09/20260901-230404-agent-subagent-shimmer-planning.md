---
title: AgentSubagent ownership and shimmer benchmark planning
kind: northstar-handoff
handoff_mode: planning-delegate
planning_mode: conversational-discovery
dispatch_authority: orchestrator
promotion_authority: orchestrator
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260901-230404-agent-subagent-shimmer-planning.md
base_required: pushed-main
tags: [planning, agent-subagent, shimmer, benchmark]
---

## Objective

Plan AgentSubagent contract/runtime ownership first, then a bounded web-only
shimmer benchmark for its running activity line. Produce one sequencing packet;
do not implement either surface.

## Settled Decisions — Do Not Re-Ask

- AgentSubagent running activity is the chosen semantic host.
- The operator authorized host planning followed by the benchmark.
- Shimmer is explicitly web-only unless native text-mask support is later
  proved; native semantics remain static.
- No generic TextShimmer, arbitrary Text/AgentMessage animation, copied Pen, or
  unsupported GPU claim.
- Architecture 012 owns full/reduced/frozen behavior.

## Required Packet

Create exactly `docs/triage/20260901-230404-agent-subagent-shimmer-decision.md`.
Define the AgentSubagent prerequisite, host ownership, static/reduced/forced
fallbacks, selection/copy and accessibility rules, benchmark engines/content/
node counts, layout/paint/layer/memory/frame budgets, promotion thresholds,
evidence retention, serial edges, and stop conditions. Do not ask the operator
again; unresolved numeric choices become explicit recommendations for review.

Run docs lint and range diff check; commit, push, and open a one-file PR.
