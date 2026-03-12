# g03.004 Performance, Render-Cost, And Memory Profile Hardening

Status: completed
Owner: Pug Core
Updated: 2026-03-12
Depends on: g02.016
Primary repos: `pug`

## Goals

- [x] define performance expectations for the component suite
- [x] identify memory and render-cost hotspots
- [x] define hardening priorities for both Svelte and GPUI surfaces

## Execution Checklist

- [x] define performance expectations for shared components
- [x] identify likely memory and render-cost hotspots
- [x] define hardening priorities for Svelte and GPUI separately where needed

## Completed Work

- froze the normative baseline in `docs/specs/038-performance-render-cost-and-memory-hardening-baseline.md`
- reduced avoidable repeated work in `packages/svelte/preview/src/App.svelte` by preindexing static demo search data, collapsing repeated command rescans, and deduping preview token refresh work
- made the current Svelte and GPUI hotspot model explicit instead of leaving performance hardening as an implied future concern

## Acceptance Criteria

- [x] performance hardening posture is explicit
- [x] hotspot and priority model is explicit

## Next Task

Open `g03.006` and define extension SDK, composition guidance, and starter
package expectations without reopening the now-frozen docs, parity, and
performance baselines.
