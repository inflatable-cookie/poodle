---
title: g16.120 A1 landmarks and content roles closeout
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
tags: [coordination, handoff, worker, g16, g16.120]
---

`docs/roadmaps/g16/120-a1-landmarks-and-content-roles.md` is implemented on
`worker/g16-120-nucleus-a1-shell-agent-plan`, based on `origin/main` at
`cceb6646a2bf7776b670fb63f586bce037d0ee6e`. Nothing is merged.

The final runtime/evidence source pin is
`54646ba2369959150a1b4953e06de5871b3ffe8f`. The lock digest is
`c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`.

AppHeader, SplitView, AgentChatInput, and AgentPlan now match their paired
Svelte A1 snapshots. The four empty-diff receipts are committed beside the
complete 29 M1 / 25 A1 cohort; the superseded g16.114/NP-1 divergence stores
were consumed. The remaining active divergence stores are retained with their
paired snapshots, exact attributes, and rulings.

The ledger was regenerated and validates 176 rows. Execution details are in
`docs/logs/2026-09/20260905-g16-120-a1-landmarks-and-content-roles.md`.
Native, headless/Svelte, receipt, ledger, docs, diff, and hosted checks are
recorded there. No windowed selector ran. The worker pushes one PR and stops
for independent exact-head review; merge remains coordinator-owned.
