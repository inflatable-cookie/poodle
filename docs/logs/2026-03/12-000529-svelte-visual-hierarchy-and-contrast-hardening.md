# 2026-03-12: Svelte Visual Hierarchy And Contrast Hardening

## Summary

Closed a meaningful Svelte-side visual hardening batch across the remaining
workstation and utility surfaces so the preview is not relying on docs-shell
polish alone.

## What Changed

- tightened workstation control geometry in command palette, action discovery,
  dock collapse, project header, and selection summary surfaces
- reduced pill usage for normal controls and metadata affordances
- shifted command/discovery items toward tonal separation instead of border-led
  framing
- improved light-theme visibility for default cards and workstation utility
  containers
- froze the resulting rules in
  `docs/specs/023-svelte-visual-hierarchy-and-contrast-baseline.md`

## Evidence

- `packages/svelte/workstation/src/CommandPalette.svelte`
- `packages/svelte/workstation/src/ProjectHeader.svelte`
- `packages/svelte/workstation/src/DockRegion.svelte`
- `packages/svelte/workstation/src/ActionDiscoveryPanel.svelte`
- `packages/svelte/components/src/SelectionSummary.svelte`
- `packages/svelte/components/src/MediaPreview.svelte`
- `packages/svelte/components/src/Card.svelte`
- `packages/svelte/preview/src/App.svelte`
- `packages/svelte/preview/src/app.css`

## Validation

- `bun run preview:build`
- `git diff --check`
