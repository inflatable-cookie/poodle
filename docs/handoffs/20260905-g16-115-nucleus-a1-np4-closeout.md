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
base: ef9049f158863ec181dee46123a8e59c0c957091
runtime_checkpoint: b033a1565f129b776a4d6e3cf8ec9ae7aa53afcf
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

The complete 34-file Nucleus receipt cohort was re-emitted through
`effigy regressions:native` at runtime checkpoint `b033a1565`. The manifest
resolution block is repinned to that source commit; the lock digest remains
`c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`. The
29-row manifest roster and scenario IDs are unchanged.

## Validation

The first rebased native run recorded 206 passed, 2 failed, and 8 ignored;
the two failures were stale current-main Select migration expectations. The
two assertions were updated to the merged `Button` role as legitimate rebase
conflict repair. The final native run recorded 208 passed, 0 failed, and 8
ignored: 5 inherited from origin/main plus 3 NP-4 divergence probes for
RadioGroup, ConfirmAction, and DetailItem.
`effigy test:nucleus-a11y` passed (14), receipt contract tests passed (11),
the evidence ledger validated all 176 rows, `effigy docs:check` passed, and
`git diff --check` passed. No windowed selectors ran.

## Review boundary

This handoff is for fresh independent exact-head review after push. The worker
does not merge.
