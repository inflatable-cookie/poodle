---
title: g03.004 performance and render-cost hardening
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, performance, svelte, docs]
---

## Summary

Completed the first `g03.004` hardening batch by freezing the performance
baseline in docs and removing a set of obvious avoidable render-cost issues
from the Svelte preview harness.

## What changed

- added the normative baseline in `docs/specs/038-performance-render-cost-and-memory-hardening-baseline.md`
- completed the roadmap entry in `docs/roadmaps/g03/004-performance-render-cost-and-memory-profile-hardening.md`
- rolled the generation index forward in `docs/roadmaps/g03/README.md`
- hardened `packages/svelte/preview/src/App.svelte` by:
  - preindexing static search data for table, browse, picker, and command demos
  - reducing repeated filter and group rescans for command discovery
  - deduping preview token refresh work so semantic token reads only rerun when
    theme, density, or control-size state actually changes
  - removing the unnecessary async `tick()` hop from preview token refresh
  - freezing catalog and section lookup structures instead of rebuilding them
    reactively

## Validation

- `bun run docs:lint`
- `bun run parity:report`
- `bun run docs:build`
- `git diff --check`

## Outcome

`g03.004` is now explicit rather than implied. The repo has a documented
performance posture, an explicit hotspot model for Svelte and GPUI, and a
meaningful first pass removing avoidable work from the current Svelte review
surface.

## Next

Move to `g03.006` and define extension-SDK, composition-guidance, and starter
package expectations while keeping the new docs, parity, and performance
baselines frozen.
