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

Clean pushed Poodle main is `bb8999b68ae2be3366d93413abdd781e2adec1ea`.
`cargo test -p poodle-render --lib` deterministically fails only the named
provider and SegmentedControl tests. g18.006 remains intact on its retained
Queue task, worker and workspace with no candidate mutation. Tom chose a
separate repair lane rather than widening that release task.

## Boundaries

Follow g18.028 exactly. Own only the two named test files and focused evidence.
Assert `Some("Save")` for the provided button and `Some("Grid")` for the visible
segment fallback. Preserve the tests' negative wrapper/layout assertions. Do
not change renderer behavior, public contracts, candidate inputs, versions,
locks, changelog, release notes, workflows, Desktop, or g18.006 state.

## Important Context

The provider adds no wrapper semantics; the child button now correctly supplies
its own accessible label. The no-icon segment falls back to visible text and
must project that same text to accessibility. A passing patch changes stale
expectations, not those accepted behaviors. If either conclusion conflicts
with current contracts, stop rather than inventing another semantic rule.

## Suggested Next Move

Reproduce both failures, inspect the current contracts and blamed accepted
commits, then make the smallest assertion-only patch and run the full release
gate before opening the PR.

## Completion Protocol

Open one non-draft PR from the Queue branch. Prove the exact head with both
focused tests, `cargo test -p poodle-render --lib`,
`effigy release status --check-gates`, docs QA and `git diff --check`; report
`ready_for_review`. Never merge, resume g18.006, prepare a candidate, tag,
publish, edit workflows, or mutate Desktop.
