---
title: g17.004 GPUI cohort programmatic append replay
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-08
updated: 2026-09-08
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Operator approved on 2026-09-08: do whatever is required to finish the blocked Lab cohort lane."
tags: [coordination, handoff, worker, g17, g17.004, gpui, cohort]
---

## What This Thread Is Doing

Execute `docs/roadmaps/g17/004-gpui-cohort-programmatic-append.md` from the
exact pushed planning commit. Repair the stale GPUI window-capture action parser
and replay path exposed by the retained Lab `g01.006` run.

## Current State

The Lab run from clean head `79069cf823ddb4fb4dce47005ece4c9115187cb2`
completed 96/174 capture attempts, then failed at
`cohort/agent-transcript/initial [gpui]`. Pinned Poodle `583aa173` rejected
`programmatic_append`, although the canonical TypeScript A1 contract, scenario,
web extractors, and Rust headless A1 model already support it. Zero Lab records
were persisted; no processes or windows remain; the existing Lab task stays
blocked and must not be replaced.

## Boundaries

Follow the card exactly. Extend the production cohort parser, host state, and
replay controller so AgentTranscript after-actions appends the declared closed
item once. Reject malformed items and use on other components. Preserve all
other capture and component contracts.

Do not invoke any windowed selector, change scenarios, alter receipts, edit
workflows, publish a release, touch the Lab repository, or merge a PR.

## Completion Protocol

Use Effigy to select focused checks. Add binding regressions for parsing,
initial-state stability, one append, and fail-closed misuse. Run the relevant
headless tests, docs checks, `git diff --check`, and one windowless binary build.
Write the execution log and a named Lab adoption request. Commit, push, open one
PR from the queue-owned branch, and report `ready_for_review` with its number and
exact head. Leave the tracked tree clean. The plugin owns independent review,
merge, and closeout.
