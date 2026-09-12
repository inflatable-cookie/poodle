---
title: g18.031 release metadata and GPUI census provenance
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

The `0.4.0` candidate cannot be coherent while release status reads unrelated
root tooling as `0.1.0` and 65 GPUI receipts are regenerated with a literal
`0.3.0`. Both are recurring infrastructure faults outside the closed candidate
surface. Fix them once without widening that surface.

## Current State

Main is clean and pushed at
`9f30dc6277581aa451c326054bee621d4ad2c330`. Retained Queue task
`17ac3fee-de90-4b32-9672-1134770bb086`, workspace `wks_85bb63e05d4b3f90`,
branch `ns-17ac3fee-de90-4b32-9672-1134770bb086` and worker history remain
preserved with no candidate commit or PR. g18.030 is complete. npm public
latest remains `0.3.0`.

## Boundaries

Follow g18.031 exactly. Do not change any package or Cargo version, root
`package.json`, g18.029's candidate policy, changelog, release notes, locks,
workflows, tags, registries, Desktop or the retained task. Do not run mounted
tests, full `qa` or release gates. The worker never merges or resumes g18.006.

## Important Context

Configure `[release]` in `effigy.toml` with explicit
`version-file = "packages/core/package.json"`, the existing changelog,
`tag-format = "v{version}"` and `pre-1-0 = true`; preserve the existing
headless release gate. Root `package.json` is private workspace tooling, not a
release package.

Derive expected-test receipt `package_version` from
`packages/gpui/preview/Cargo.toml`. Add fail-closed pure parsing and stale-file
checks. Regenerate checked-in census artifacts from the existing execution
record only; no accepted capability, source commit, lock hash, run ID or test
body should change.

## Suggested Next Move

Plant focused release-source and manifest-version laws first. Then implement
the two derivations, run one deterministic census regeneration and inspect the
diff before focused validation.

## Completion Protocol

Run only `test:gpui-census`, `check:gpui-census`, a read-only release
status/config probe, `docs:lint` and `git diff --check`. Open one non-draft PR
and report `ready_for_review` immediately with the exact head, resolved Effigy
version source, receipt count and provenance-only regeneration diff. Queue owns
CI, review and merge. No release mutation or g18.006 continuation occurs here.
