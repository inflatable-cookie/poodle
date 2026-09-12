---
title: g18.029 v0.4.0 candidate-scope admission
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
queue_approval: "Tom approved the separate reviewed v0.4.0 candidate-scope precursor on 2026-09-12 to unblock retained g18.006."
queue:
  capability: complex
  skipPRReview: false
  notifyOriginOnCloseout: true
tags: [coordination, handoff, worker, g18, g18.029, release-gate, package-install]
---

## What This Thread Was Doing

Execute [`g18.029`](../roadmaps/g18/029-v040-candidate-scope-admission.md) as
the separate release-infrastructure precursor for retained g18.006. Add a
closed `0.4.0` candidate policy that the ordinary PR lane can recognize and
validate without a workflow exception.

## Why It Matters

The candidate cannot currently pass its own local release gate or required PR
`web` check: ordinary installed-package certification rejects release-bearing
paths, while the only explicit candidate mode is historical `0.3.0` policy.
Preparing candidate inputs before this admission merges would waste every
generated and source-bound identity.

## Current State

The clean product baseline inspected before this planning-only promotion was
`ee5bdefce276558abb37c76486bc244680d812b4`; it contains the accepted g18.028
repair. Dispatch from the pushed planning commit that contains this handoff.
Retained g18.006 Queue task
`17ac3fee-de90-4b32-9672-1134770bb086`, workspace
`wks_85bb63e05d4b3f90` and branch
`ns-17ac3fee-de90-4b32-9672-1134770bb086` are preserved with no PR or unique
candidate commit. Its worker proved the plan-level admission gap and stopped.

## Boundaries

Follow g18.029 exactly. Own only the installed-package scope implementation,
its focused laws/readme and one execution log. Preserve strict mode and the
historical `g16.054-candidate` behavior. Do not edit release inputs, workflows,
component source, generated receipts, g18.006 state, tags, registries or
Desktop. Never run release mutation.

## Important Context

Ordinary CI receives no candidate environment variable. Safe admission must
come from validating the complete range against a closed g18.006 policy, not
from branch naming or a broad path exception. Candidate evidence follows a
frozen release-input commit; later commits must be evidence/log-only and leave
every candidate input byte unchanged. The hosted branch dry run has moved to
post-merge g18.009 and no workflow edit is authorized.

## Suggested Next Move

Start with committed-range fixtures reproducing the current rejection. Add the
smallest policy-selection structure that expresses both immutable g16.054 and
closed g18.006 rules, then bind automatic ordinary recognition and the
evidence-only suffix to production-path falsification laws.

## Completion Protocol

Open one non-draft PR from the Queue branch. Prove the focused scope and
web-pack-install suites, package-install documentation, `git diff --check`, and
required exact-head GitHub `web`/`rust` checks. Report `ready_for_review` with
the exact head and every acceptance/falsification result. Never merge, prepare
the candidate, resume g18.006, edit workflows, run a release gate, tag, publish
or mutate Desktop.
