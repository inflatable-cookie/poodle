---
title: g18.031 release metadata and GPUI census provenance
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: complete — PR #264 merged as `aa659504b2a6222eb34c7423fcfd877a32138ba8`
owner: Poodle Northstar orchestrator
created: 2026-09-12
updated: 2026-09-12
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom's operator-directed g18.006 takeover authorized Poodle Chatterbox to resolve its plan-level release blockers and continue the retained task through Northstar Queue."
queue:
  capability: complex
  skipPRReview: false
  notifyOriginOnCloseout: true
tags: [coordination, handoff, worker, g18, g18.031, release, census, provenance]
---

## What This Thread Was Doing

Execute [`g18.031`](../roadmaps/g18/031-release-metadata-and-census-provenance.md)
as the structural precursor for retained g18.006. Give Effigy the correct
public release version source and remove the GPUI census generator's embedded
release version.

## Why It Matters

The `0.4.0` candidate cannot be coherent while root repository metadata stays
at `0.1.0` and 65 GPUI receipts are regenerated with a literal `0.3.0`.
Fix both recurring infrastructure faults and keep publication scope unchanged.

## Current State

Main is clean and merged at
`aa659504b2a6222eb34c7423fcfd877a32138ba8` (PR #264, reviewed head
`a3f2a513a1d684a60a639c4904a606c6598c1f4d`, accepted review `5649246995`).
Retained Queue task `17ac3fee-de90-4b32-9672-1134770bb086`, workspace
`wks_85bb63e05d4b3f90`, branch
`ns-17ac3fee-de90-4b32-9672-1134770bb086` remain preserved for the next
candidate lane with no candidate commit or PR. npm public latest remains
`0.3.0`.

## Post-merge closeout

Accepted review is [PR comment
5649246995](https://github.com/inflatable-cookie/poodle/pull/264#issuecomment-5649246995),
bound to the reviewed head with `ready_to_merge`; it found no blocking findings.
The worker/reviewer validation and exact-head merge evidence are recorded in
the execution log. Closeout reran no suites. No task-specific failure is
deferred; retained g18.006 resumes the `0.4.0` candidate on current main, and
g18.009 owns the later hosted release sequence.

## Boundaries

Follow g18.031 exactly. Align only root `package.json` to current `0.3.0` and
add only its exact version transition to g18.029's candidate policy. Do not
change any other package/Cargo version, root manifest field, changelog, release
notes, locks, workflows, tags, registries, Desktop or the retained task. Do not
run mounted tests, full `qa` or release gates. The worker never merges or
resumes g18.006.

## Important Context

Configure `[release]` in `effigy.toml` with explicit
`version-file = "package.json"`, the existing changelog,
`tag-format = "v{version}"` and `pre-1-0 = true`; preserve the existing
headless release gate. Root `package.json` stays private and unpublished, but
its version is truthful repository metadata and moves in lockstep.

Derive expected-test receipt `package_version` from
`packages/gpui/preview/Cargo.toml`. Add fail-closed pure parsing and stale-file
checks. Regenerate checked-in census artifacts from the existing execution
record only; no accepted capability, source commit, lock hash, run ID or test
body should change.

## Suggested Next Move

Plant focused release-source, exact root candidate-transition and
manifest-version laws first. Then implement the derivations, run one
deterministic census regeneration and inspect the diff before focused
validation.

## Completion Protocol

Run only the focused candidate-scope suite, `test:gpui-census`,
`check:gpui-census`, a read-only release status/config probe, `docs:lint` and
`git diff --check`. Open one non-draft PR
and report `ready_for_review` immediately with the exact head, resolved Effigy
version source, receipt count and provenance-only regeneration diff. Queue owns
CI, review and merge. No release mutation or g18.006 continuation occurs here.
