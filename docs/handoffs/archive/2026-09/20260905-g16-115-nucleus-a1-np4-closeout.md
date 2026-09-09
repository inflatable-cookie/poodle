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
base: eaa2e7ba20d6de585ab19f13bcb385cd1ef9b9bc
runtime_checkpoint: ecfdc3a662c3e937782c07f35b042e7c3bb0e294
tags: [coordination, handoff, worker, g16, g16.115]
---

## Outcome

NP-4 is implemented on the dedicated worker branch. TextInput and Callout have
validated A1 receipts. Switch remains owned by g16.111. RadioGroup,
ConfirmAction, and DetailItem are recorded as structural divergences with no
receipt and no mounted cell.

## Divergence evidence

Each divergent row has its executed GPUI snapshot and exact diff under
`docs/roadmaps/g16/nucleus-parity-receipts/a1-divergences/<row>/`; the paired
Svelte snapshot remains at the committed scenario snapshot path. RadioGroup's
current pair is `radio-group.gpui.json` plus `radio-group.a1-diff.json` and
records exactly `orientation` at index 0 and `focus_order` at index 2 against
`test/nucleus-a11y/snapshots/radio-group.svelte.json`. The stale duplicate
`radio-group-run.*` bundle was removed. ConfirmAction lacks the Svelte
dialog/backdrop accessibility projection. DetailItem lacks the Svelte
description Popover projection. These are bounded repair-card inputs for
Chatterbox.

## Cohort identity

The complete 41-file Nucleus receipt cohort (29 M1 rows plus the current A1
receipts) was re-emitted through
`effigy regressions:native` at runtime checkpoint `ecfdc3a6`. The manifest
resolution block is repinned to that source commit; the lock digest remains
`c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`. The
29-row manifest roster and scenario IDs are unchanged.

## Validation

The final rebased native run recorded 218 passed, 0 failed, and 1 ignored
(the pre-existing ignored test inherited from origin/main; the NP-4 divergence
probes ran in the native board). `effigy test:nucleus-a11y` passed (30), receipt
contract tests passed (11),
the evidence ledger validated all 176 rows, `effigy docs:check` passed, and
`git diff --check` passed. No windowed selectors ran.

## Review boundary

This handoff is for fresh independent exact-head review after push. The worker
does not merge.
