---
title: g16.117 Select A1 alignment worker handoff
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
tags: [coordination, handoff, worker, g16, g16.117]
---

Execute `docs/roadmaps/g16/117-select-a1-alignment.md` from current `origin/main` (dispatch manifest revision 17). Apply the card's five contract decisions, use the recommended non-focusable decorative chevron if no operator answer arrives, end with an empty-diff Select A1 receipt, and delete the divergence store. Scope is g16.117 only. Produce one PR, record the execution log and closeout handoff, run relevant headless checks, push, and stop for independent exact-head review. Do not merge or run windowed selectors.
