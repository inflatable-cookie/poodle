# g02.009 App Shell And Workspace Shell Depth

Status: completed
Date: 2026-03-11
Owner: Flint Core

## Summary

- completed `g02.009`
- extended the Svelte workstation package with `AppHeader`, `ProjectHeader`, `ShellStatusBar`, and `WorkspaceShell`
- expanded the preview with a workstation shell composition section that exercises loading, offline, disconnected, and empty shell states above the command-discovery layer
- added the utility-region contract at `docs/contracts/workstation/shell-status-bar.md`
- added the normative shell-depth baseline at `docs/specs/017-app-shell-and-workspace-shell-depth-rules.md`

## Validation

- `bun run preview:build`
- `bun run tokens:build`
- `git diff --check`

## Notes

- this tranche intentionally deepens shell hierarchy and utility-region posture without taking on docking, tab orchestration, or persistence engines
- workstation shell state now distinguishes offline from disconnected, which is especially important for accessibility and recovery posture

## Next Task

Open `docs/roadmaps/g02/010-dock-split-view-tabs-and-persistence-orchestration.md` and build the next workstation batch above the now-live shell and command-discovery layers.
