# 017 App Shell And Workspace Shell Depth Rules

Status: active
Updated: 2026-03-11
Depends on: `006-workstation-shell-and-panel-system-rules.md`, `016-command-palette-and-action-discovery-rules.md`

## Purpose

Freeze the next workstation shell rules for deeper app-header, project-header, workspace-shell, and utility-region composition so downstream apps can build dense shells without inventing ad hoc structure.

## Shell Layering Rule

The deeper workstation shell keeps distinct vertical layers when present:

- app-global header
- project/workspace-scoped header
- surface navigation or other shell-local utility rows
- main workspace body
- persistent utility/status row
- overlay host

Hosts may omit layers.
They should not collapse these meanings into one unlabeled toolbar strip.

## Header Separation Rule

`AppHeader` and `ProjectHeader` remain distinct.

- `AppHeader` owns app-wide identity and shell utilities
- `ProjectHeader` owns project/workspace context and scoped actions

The same visible row should not silently stand in for both meanings without documentation.

## Utility Region Rule

Persistent utility metadata belongs in a dedicated shell utility/status region when it needs to stay visible during workspace interaction.

That region may include:

- active surface summary
- connection or sync posture
- current command or scope hints
- other low-urgency shell metadata

Critical remediation still belongs on stronger inline messaging surfaces.

## Shell State Rule

Workspace shell state should remain explicit.

At minimum, deeper shell implementations should distinguish:

- loading
- empty
- offline
- disconnected
- ready

Offline and disconnected are not interchangeable.

## Command Interplay Rule

Workspace shells should expose a clear path into command discovery.

That path may live in:

- app header actions
- project header actions
- or another persistent shell affordance

Hosts may vary the placement.
They should not make command discovery reachable only through invisible or undocumented gestures.

## Accessibility Rule

Both runtimes must preserve:

- named shell hierarchy
- stable header and utility ordering
- explicit shell-state meaning
- keyboard reachability to command discovery and shell utilities
- deterministic focus restoration when command overlays close

Svelte should use native header, region, button, and text semantics first.
GPUI must recreate equivalent shell hierarchy and utility meaning in the native accessibility tree.

## Seed Evidence

- `docs/contracts/workstation/app-header.md`
- `docs/contracts/workstation/project-header.md`
- `docs/contracts/workstation/workspace-shell.md`
- `docs/contracts/workstation/shell-status-bar.md`
- `packages/svelte/workstation/src/AppHeader.svelte`
- `packages/svelte/workstation/src/ProjectHeader.svelte`
- `packages/svelte/workstation/src/WorkspaceShell.svelte`
- `packages/svelte/workstation/src/ShellStatusBar.svelte`
- `packages/svelte/preview/src/App.svelte`
