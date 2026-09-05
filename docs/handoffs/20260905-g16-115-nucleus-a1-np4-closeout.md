---
title: g16.115 Nucleus A1 NP-4 settings remediation closeout
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-for-independent-review
owner: Poodle Northstar orchestrator
created: 2026-09-05
updated: 2026-09-05
base: f9922fd1d558ae34f8888e524366791f79f942cb
runtime_checkpoint: c670294298830aa1a0ccae35810c46b66b8fb51b
tags: [coordination, handoff, worker, g16, g16.115]
---

## Outcome

NP-4 is implemented on the dedicated worker branch. TextInput and Callout have
validated A1 receipts. Switch remains owned by g16.111. RadioGroup,
ConfirmAction, and DetailItem are recorded as structural divergences with no
receipt and no mounted cell.

## Divergence evidence

Each divergent row has its executed GPUI snapshot and exact diff under
`docs/roadmaps/g16/nucleus-parity-receipts/a1-divergences/<row>/`. RadioGroup
differs on orientation/focus-order semantics. ConfirmAction lacks the Svelte
dialog/backdrop accessibility projection. DetailItem lacks the Svelte
description Popover projection. These are bounded repair-card inputs for
Chatterbox.

## Cohort identity

The complete 34-file Nucleus receipt cohort was re-emitted through
`effigy regressions:native` at runtime checkpoint `c67029429`. The manifest
resolution block is repinned to that source commit; the lock digest remains
`c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`. The
29-row manifest roster and scenario IDs are unchanged.

## Validation

`effigy regressions:native` passed (208 passed, 8 pre-existing ignored),
`effigy test:nucleus-a11y` passed (14), receipt contract tests passed (11),
the evidence ledger validated all 176 rows, `effigy docs:check` passed, and
`git diff --check` passed. No windowed selectors ran.

## Review boundary

This handoff is for fresh independent exact-head review after push. The worker
does not merge.
