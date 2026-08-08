---
title: g03.010 accessibility audit baseline
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, accessibility, audit]
---

## Summary

Completed `g03.010` by freezing an explicit section-based accessibility audit
baseline, adding a generated audit artifact, and making cross-runtime GPUI
accessibility blockers visible instead of implied.

## What changed

- added the normative spec `docs/specs/043-accessibility-audit-and-cross-runtime-delta-handling-baseline.md`
- completed `docs/roadmaps/g03/010-accessibility-audit-and-assistive-technology-conformance.md`
- added the accessibility audit manifest `packages/svelte/preview/src/accessibility.ts`
- added the generated report builder `packages/svelte/preview/scripts/build-accessibility-report.ts`
- added the generated artifact `packages/svelte/preview/artifacts/accessibility-report.json`
- extended `packages/svelte/preview/scripts/lint-docs.ts` so docs lint now validates accessibility audit coverage and blocker honesty
- wired `accessibility:report` into preview and root package scripts
- rolled the index and next-task surfaces forward in:
  - `docs/specs/README.md`
  - `docs/roadmaps/g03/README.md`
  - `docs/roadmaps/README.md`
  - `docs/README.md`
  - `README.md`

## Validation

- `bun run docs:lint`
- `bun run accessibility:report`
- `bun run docs:build`
- `git diff --check`

## Outcome

`g03.010` is now explicit. The repo has a machine-readable accessibility audit
target for every docs section, a generated artifact for repeat review, and an
honest statement of where GPUI or native assistive-technology evidence is still
blocked.

## Next

Move to `g03.011` and define deprecation, change-control, and release-channel
operations without reopening the now-explicit accessibility baseline.
