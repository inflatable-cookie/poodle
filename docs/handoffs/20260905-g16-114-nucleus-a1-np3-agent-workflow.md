---
title: g16.114 Nucleus A1 NP-3 agent workflow worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-05
updated: 2026-09-05
base_required: pushed-main
tags: [coordination, handoff, worker, g16, g16.114]
---

Execute `docs/roadmaps/g16/114-nucleus-a1-np3-agent-workflow.md` from current `origin/main` (dispatch manifest revision 17). Scope is g16.114 only. Produce one PR, record the execution log and closeout handoff, run relevant headless checks, push, and stop for independent exact-head review. Do not merge or run windowed selectors.
