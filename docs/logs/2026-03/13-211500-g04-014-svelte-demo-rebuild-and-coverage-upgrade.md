---
title: g04.014 svelte demo rebuild and coverage upgrade
status: completed
owner: nucleus
updated: 2026-03-13
tags: [logs, roadmap, svelte, demo, parity]
---

## Summary

Completed `g04.014` by rebuilding the Svelte preview around the shared demo-app
contract instead of leaving the demo target buried inside the old section-by-
section docs surface.

## What changed

- added preview-local components for the rebuilt shell split:
  - `packages/svelte/preview/src/components/SharedDemoApp.svelte`
  - `packages/svelte/preview/src/components/DocsCatalogHub.svelte`
  - `packages/svelte/preview/src/components/TokenToolsPanel.svelte`
- rebuilt `packages/svelte/preview/src/App.svelte` so the docs host now does
  three explicit jobs:
  - launch the shared demo screens
  - host the docs-only catalog shell
  - host token inspection tools outside the shared demo boundary
- turned the preview navigation into a cleaner destination model built around
  the shared demo screens plus docs-only tools rather than one long list of
  old source sections
- moved the Svelte demo onto one coherent shared shell with six contract-owned
  screens instead of separate inline section examples
- upgraded direct public-surface adoption in the rebuilt demo:
  - primitives now directly cover dialog, drawer, popover, menu, toolbar,
    segmented control, table, tabs, date picker, callout, radio group, switch,
    text area, and status indicator in addition to the earlier shell controls
  - composites now directly cover `PickerShell` and `SelectionSummary`
  - workstation now directly covers `PanelHeader` and `PanelTabs`
- updated `packages/svelte/preview/src/parity.ts` so the parity artifact counts
  the rebuilt demo coverage honestly
- rolled `packages/shared-demo-app-audit.json` forward so its package-coverage
  counts match the stronger rebuilt demo surface
- marked `packages/shared-demo-app-contract.json` as `rebuilt` for the Svelte
  runtime binding
- added the normative baseline
  `docs/specs/061-svelte-demo-app-rebuild-and-coverage-upgrade-baseline.md`
- completed `docs/roadmaps/g04/014-svelte-demo-app-rebuild-component-adoption-and-coverage-upgrade.md`
- rolled the repo forward so `g04` now points at `g04.015`

## Validation

- `effigy docs:build`
- `effigy docs:check`
- `git diff --check`

## Outcome

The Svelte side now has a much stronger shared demo target for GPUI to match.
The docs shell still exists, but it no longer doubles as the implicit demo app.
The parity artifact now records materially stronger Svelte demo coverage:

- `@pug/svelte-primitives`: `28/63`
- `@pug/svelte-composites`: `20/20`
- `@pug/svelte-workstation`: `14/14`

## Next

Open `g04.015` and implement the same shared demo app in GPUI, using the
rebuilt Svelte shell and screen model for side-by-side review rather than the
older section-by-section preview surface.
