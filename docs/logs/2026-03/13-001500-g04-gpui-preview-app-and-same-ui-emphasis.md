---
title: g04 gpui preview app and same-ui emphasis
status: completed
owner: nucleus
updated: 2026-03-13
tags: [logs, roadmap, gpui, parity]
---

## Summary

Adjusted the newly opened `g04` roadmap so GPUI work is framed more explicitly
as building the same contract-owned UI as the Svelte surface, with a native
GPUI preview app or review shell that supports side-by-side comparison where it
materially improves parity review.

## What changed

- updated `docs/roadmaps/g04/README.md` to state more directly that Loophole-facing shared surfaces should target the same UI across Svelte and GPUI, not merely "similar enough" implementations
- tightened `docs/roadmaps/g04/001-gpui-contract-audit-parity-priority-matrix-and-implementation-order.md` so the first audit must identify side-by-side review target surfaces
- renamed and expanded `docs/roadmaps/g04/002-gpui-theme-runtime-token-application-and-native-preview-app-baseline.md` so the GPUI review surface is explicitly a preview app baseline rather than a vague native harness
- expanded `docs/roadmaps/g04/011-cross-runtime-parity-report-intentional-delta-register-and-acceptance-harness-expansion.md` so side-by-side comparison evidence is part of the later parity plan

## Validation

- `git diff --check`

## Outcome

`g04` now states the stronger parity intent more clearly: GPUI should track the
Svelte surface closely enough to support direct comparison, with explicit
native deltas rather than loose conceptual similarity.

## Next

Open `g04.001` and define which Svelte preview sections and shared surfaces
must have matching GPUI review surfaces before the larger implementation
tranches begin.
