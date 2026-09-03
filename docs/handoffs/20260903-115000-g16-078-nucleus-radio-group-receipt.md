---
title: g16.078 Nucleus RadioGroup M1 preparation handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
review_authority: orchestrator
merge_authority: orchestrator
handoff: single-file-path-only
status: preparation-ready
owner: Poodle Northstar orchestrator
created: 2026-09-03
updated: 2026-09-03
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260903-115000-g16-078-nucleus-radio-group-receipt.md
base_required: current-pushed-main
tags: [coordination, handoff, worker, pr, g16, nucleus, gpui, radio-group, preparation]
---

Implement only the preparation phase of
`docs/roadmaps/g16/078-nucleus-radio-group-m1.md` in the launcher-provided
`feature/g16-078-nucleus-radio-group-receipt` worktree. Read AGENTS, Northstar,
the repo Effigy skill, g16.062, the manifest/ledger, RadioGroup contract/spec/
machine/renderer/adapter/backend, and the retained named test.

Strengthen the retained test through the real `IntoElement` path, commit the
counterexample before repair, complete focused falsification and validation,
push a draft PR, and pause. Do not touch manifest, receipts, generated ledger,
g16 front doors, releases, workflows, Nucleus, web, A1/V1, Jetstream, or run
windowed/native-visual selectors. Do not claim M1 completion. The orchestrator
will resume this same identity after g16.077 merges for rebase, cohort emission,
full boards, review, and merge.
