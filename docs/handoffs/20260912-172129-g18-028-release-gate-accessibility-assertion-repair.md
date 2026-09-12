---
title: g18.028 Release-gate accessibility assertion repair
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-12
updated: 2026-09-12
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom selected a separate repair task on 2026-09-12 for the two stale accessibility assertions blocking retained g18.006."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
tags: [coordination, handoff, worker, g18, g18.028, release-gate, accessibility, rust]
---

## What This Thread Was Doing

Execute [`g18.028`](../roadmaps/g18/028-release-gate-accessibility-assertion-repair.md)
as a narrow test-only repair. Update two stale `poodle-render` expectations to
the already-accepted accessible-label semantics and reopen g18.006's release
gate.

## Why It Matters

Retained g18.006 reached candidate validation after the product sweep, but the
full Rust library gate is red because two tests still assert behavior from
before NP-4 and g16.119 accessibility work. The release must not bypass the
gate or regress accessible labels to satisfy obsolete expectations.

## Current State

PR #260 carries the approved two-assertion repair at
`390095f6f3653846af0d95d16d35a91c944389a0`; both focused tests and all 647
`poodle-render` library tests pass. Its web check is red only because the
source-bound Nucleus cohort still names the predecessor render source. The
same retained worker/PR now owns the complete coherent repin. g18.006 remains
intact and blocked until this PR closes.

## Boundaries

Follow g18.028 exactly. Preserve the two approved assertion edits and own only
the generated Nucleus receipt, manifest, parity-ledger and GPUI-census cohort
needed to bind them. Assert `Some("Save")` for the provided button and
`Some("Grid")` for the visible segment fallback. Preserve the tests' negative
wrapper/layout assertions. Do not change renderer behavior, public contracts,
candidate inputs, versions, locks, changelog, release notes, workflows,
Desktop, or g18.006 state.

## Important Context

The provider adds no wrapper semantics; the child button now correctly supplies
its own accessible label. The no-icon segment falls back to visible text and
must project that same text to accessibility. A passing patch changes stale
expectations, not those accepted behaviors. If either conclusion conflicts
with current contracts, stop rather than inventing another semantic rule.

## Suggested Next Move

Keep the approved assertion patch, regenerate the complete Nucleus cohort, and
run only the focused Rust and generated-evidence selectors listed by g18.028.
Push the stable head and let the required PR `web`/`rust` lanes supply broad
exact-head proof.

## Completion Protocol

Update the existing non-draft PR from the Queue branch. Prove the exact head
with both focused tests, `cargo test -p poodle-render --lib`,
`effigy test:nucleus-parity-receipts`,
`effigy check:parity-evidence-ledger`, `effigy check:gpui-census`,
`effigy docs:lint`, `git diff --check`, and the required GitHub `web`/`rust`
checks. Do not run local `effigy qa`, `effigy ci:web`, `effigy docs:check`, or
`effigy release status --check-gates`; g18.006 owns one full release run after
the final candidate is stable. Report `ready_for_review`. Never merge, resume
g18.006, prepare a candidate, tag, publish, edit workflows, or mutate Desktop.
