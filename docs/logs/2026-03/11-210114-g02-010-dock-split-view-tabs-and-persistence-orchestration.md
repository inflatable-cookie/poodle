# g02.010 Dock Split View Tabs And Persistence Orchestration

Status: completed
Date: 2026-03-11
Owner: Pug Core

## Summary

- completed `g02.010`
- extended the Svelte workstation package with `PanelTabs`, `SurfaceTabs`,
  `PanelHeader`, `PanelSurface`, `SplitView`, `DockRegion`, and a typed layout
  snapshot serializer
- expanded the preview with a real workstation orchestration section that
  exercises surface tabs, left and right dock regions, nested split views,
  close/reorder/collapse flows, and a host-owned serialized layout snapshot
- added the normative orchestration baseline at
  `docs/specs/018-dock-split-view-tabs-and-persistence-orchestration-rules.md`

## Validation

- `bun run preview:build`
- `bun run tokens:build`
- `git diff --check`

## Notes

- the persistence layer is still intentionally host-owned; this tranche freezes
  the snapshot shape and restore expectations, not a storage backend
- tab and split controls were hardened during implementation so keyboard
  selection, reorder, and resize semantics are exercised directly in the live
  preview rather than deferred to a later cleanup pass

## Next Task

Open
`docs/roadmaps/g02/011-accessibility-focus-keyboard-and-state-semantics-hardening.md`
and harden focus, keyboard, and state semantics across the advanced catalogue.
