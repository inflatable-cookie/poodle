---
title: g16.119 A1 focus and state semantics closeout
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: pushed-awaiting-review
owner: Poodle Northstar orchestrator
created: 2026-09-05
updated: 2026-09-05
base_required: pushed-main
tags: [coordination, handoff, worker, g16, g16.119]
---

`docs/roadmaps/g16/119-a1-focus-and-state-semantics.md` is implemented and
pushed on `worker/g16-119-a1-focus-state-semantics`. Nothing is merged.

Execution log: `docs/logs/2026-09/20260905-g16-119-a1-focus-and-state-semantics.md`.

Two things the reviewer should decide rather than assume:

1. Four of the five rows keep a recorded divergence instead of an empty-diff
   receipt. Each ruling is in
   `docs/roadmaps/g16/nucleus-parity-receipts/a1-divergences/README.md`. The
   Menu one is the card's stop condition firing: Svelte makes every enabled
   menu item a sequential tab stop and `menu.md` states no tab-stop rule, so
   the contract does not decide it. That question is returned to Chatterbox.
2. The lane restored three A1 probes it does not own (`callout`,
   `editable-label`, `text-input`). They had committed receipts but no probe
   in any commit, so the cohort could not be re-emitted at the repin. Each
   restored probe reproduces its committed `<row>.gpui.json` byte-for-byte.
   If the reviewer would rather this land separately, the change is one
   isolated commit (`test(g16.119): restore the three dropped A1 probes`).

Repin: `e2630da998d13466a5de8ff266f0f1e9dc371e13`. Cohort re-emitted at that
head; ledger regenerated and validated. No windowed selector was run.
