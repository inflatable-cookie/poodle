---
title: g18.020 RichTextEditor heading mode select
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-11
updated: 2026-09-11
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom requested one RichTextEditor text-mode dropdown with Normal text and H1–H6 on 2026-09-11, then explicitly chose consumer-configurable heading levels."
queue:
  dependsOn:
    - 5b42e240-5e63-4bdd-8035-7f2285776c6e
    - 6bb46544-966a-4af0-af7c-e53846ad0407
  capability: complex
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.020, rich-text, headings, select]
---

## What This Thread Was Doing

Execute [`g18.020`](../roadmaps/g18/020-rich-text-heading-mode-select.md)
after g18.014 and g18.018 close. Replace the separate heading buttons with one
configurable text-mode Select and extend real heading support through H6.

## Why It Matters

The toolbar currently exposes fixed H1–H3 buttons, leaving H4–H6 inaccessible
and consuming unnecessary toolbar space. The schema itself stops at level 3,
so a presentation-only dropdown would misrepresent document capability.

## Current State

g18.013 merged the shared command presentation and grouped Poodle controls.
g18.014 is working in the paired specimens; g18.018 is in exact-head review on
the paired engines. This task waits for both to avoid conflicting workspaces.
The existing custom Select already supports custom trigger and option rendering.

## Boundaries

Follow g18.020 exactly. Keep granular public heading commands, extend them to
H6, and project the configured subset as one selector with intrinsic Normal and
Mixed states. Extend schema/editor/renderer semantics together. Do not add a
composite command, arbitrary extensions, toolbar slots, H7+, release work,
native parity, or consumer changes.

## Important Context

Place the selector at the first resolved heading command and preserve all other
command order. The trigger must not resize with heading typography; only menu
options preview the document scale. Selecting a level sets it exactly rather
than toggling. Treat the trigger as one toolbar roving stop and let an open
Select own its keyboard events.

## Suggested Next Move

Rebase after both prerequisites merge. First plant shared H4–H6 schema/renderer
failures and paired sparse-subset projection tests, then compose the Poodle
Select without duplicating listbox behavior.

## Completion Protocol

Open one non-draft PR from the queue-owned branch, prove the exact head with
focused paired core/component/browser/accessibility checks, both package and
preview builds, docs QA and `git diff --check`, then report
`ready_for_review`. Never merge, release g18.011, resume g18.006/g18.009,
publish, or mutate Desktop.
