---
title: g16.111 Nucleus A1 accessibility receipt foundation worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: pr-open-awaiting-review
owner: Poodle Northstar orchestrator
created: 2026-09-05
updated: 2026-09-05
handoff_path: /Users/tom/.paseo/worktrees/1ugbsx1t/g16-111-nucleus-a1-accessibility-receipt/docs/handoffs/20260905-g16-111-nucleus-a1-accessibility-receipt-foundation.md
base_required: pushed-main
tags: [coordination, handoff, worker, g16, g16.111]
---

## What This Thread Was Doing

Execute g16.111, the Nucleus A1 paired accessibility receipt foundation, from
`docs/roadmaps/g16/111-nucleus-a1-accessibility-receipt-foundation.md` under
dispatch manifest revision 7. The card and the current Northstar docs are
authoritative.

## Current State

- **Repository:** `inflatable-cookie/poodle`
- **Workspace (absolute):** `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-111-nucleus-a1-accessibility-receipt`
- **Base:** `d8aea4aea639642f834c95261e63250b27f4dd60` (promoted, `origin/main`)
- **Worker branch:** `worker/g16.111-nucleus-a1-accessibility-receipt-foundation`
- **Head:** the pushed tip of the worker branch at PR open; the receipt
  source pin is `ca7b76c4049eaa25cf0fb98e2287acb7a4a27c66` (manifest
  `resolution.source_commit`), and every later commit on the branch touches
  docs and evidence only
- **Allowed runway:** g16.111 only; g16.112–116 stay held
- **Reserved coordinator paths:** `docs/roadmaps/g16/README.md`,
  `docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`
- **Execution log:** `docs/logs/2026-09/20260905-g16-111-nucleus-a1-accessibility-receipt-foundation.md`

## Scope Delivered

- Shared scenario files `test/nucleus-a11y/scenarios/{switch,tabs,select}.json`
  read by both runtimes and hashed into every snapshot and receipt.
- GPUI extractor in `packages/gpui/preview/src/headless_driver.rs`
  (`accessibility_nodes`, `focus_traversal`); A1 normalisation, diff, and
  receipt emission in `packages/gpui/preview/src/nucleus_receipts.rs`; three
  proofs in `packages/gpui/preview/tests/headless/nucleus_a11y.rs` joined to
  `effigy regressions:native`.
- Svelte extractor and `nucleus-a11y` vitest project under `test/nucleus-a11y/`
  (`effigy test:nucleus-a11y`); `dom-accessibility-api` added as a dev
  dependency.
- Receipt schema `proof_level: "A1"` with an `accessibility` block; validator
  and ledger consume validated A1 receipts (Switch, Tabs now `mounted`).
- Select diverges from the Svelte reference; its proof is `#[ignore]` with the
  diff recorded in the log as the NP-2 (`g16.113`) repair candidate.

## Checks Run

`effigy regressions:native`, `effigy test:nucleus-a11y`,
`effigy test:nucleus-parity-receipts`, `effigy test:parity-evidence-ledger`,
`effigy check:parity-evidence-ledger`, `effigy docs:check`, `effigy ci:web`,
`git diff --check origin/main...HEAD`. Results are in the execution log. No
`*-windowed` selector was run.

## Stop Conditions

- Select A1 cannot pass without a component change (trigger role, listbox
  name, focusable options, indicator button); the card forbids repairs here.
  Reported, not fixed. Escalation owner: Chatterbox.
- Focus order for untracked focusable nodes (the Tabs panel) is not
  attributable through the backend registry; the extractor reports the
  declared tab order from the node record and cross-checks gpui's executed
  traversal for tracked nodes. No behaviour change was made.

## Completion Protocol

The PR is open against `main` and not merged. Review proceeds as an
independent exact-head review in this same workspace; the coordinator owns
merge and the reserved closeout surfaces.
