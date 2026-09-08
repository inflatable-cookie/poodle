---
title: g17.003 background-safe non-activation proof worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-08
updated: 2026-09-08
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Operator approved the durable background-safe capture correction on 2026-09-08 after confirming Poodle Lab must permit normal foreground app use during background runs."
tags: [coordination, handoff, worker, g17, g17.003, capture, foreground]
---

Execute `docs/roadmaps/g17/003-background-safe-nonactivation-proof.md` from
current `origin/main` under the ready frontier in `docs/roadmaps/dispatch.md`.
Scope is g17.003 only. Produce one PR, record the execution log and Lab adoption
request, run the relevant headless/windowless checks, push, and stop for
independent exact-head review. Do not merge or run any windowed selector.
