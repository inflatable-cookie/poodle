# g02.008 Command Palette And Action-Discovery Depth

Status: completed
Date: 2026-03-11
Owner: Flint Core

## Summary

- completed `g02.008`
- added a real Svelte workstation package at `packages/svelte/workstation`
- added workstation components at `packages/svelte/workstation/src/CommandPalette.svelte` and `packages/svelte/workstation/src/ActionDiscoveryPanel.svelte`
- extended the preview with a workstation command-discovery section plus a modal command palette driven by grouped, host-ranked command data
- added workstation contracts at `docs/contracts/workstation/command-palette.md` and `docs/contracts/workstation/action-discovery-panel.md`
- added the normative command-discovery baseline at `docs/specs/016-command-palette-and-action-discovery-rules.md`

## Validation

- `bun install`
- `bun run preview:build`
- `bun run tokens:build`
- `git diff --check`

## Notes

- this tranche intentionally freezes launcher posture, grouped discovery, and keyboard semantics without pretending Flint owns downstream command registries or ranking heuristics
- the workstation package boundary is now explicit on the Svelte side instead of hiding shell-level behavior in generic composites

## Next Task

Open `docs/roadmaps/g02/009-app-shell-and-workspace-shell-depth.md` and build the next workstation batch above the now-live command-discovery layer.
