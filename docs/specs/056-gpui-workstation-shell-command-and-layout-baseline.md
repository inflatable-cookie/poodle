# 056 GPUI Workstation Shell, Command, And Layout Baseline

Status: active
Updated: 2026-03-12
Depends on: `055-gpui-data-browse-detail-picker-and-media-composite-baseline.md`

## Purpose

Freeze the first GPUI workstation layer above the widened primitive and
composite surface. This baseline adds generic shell headers, panel chrome,
dock and split orchestration, command discovery, and workspace snapshot
posture so GPUI can host the same high-level shell UI families as Svelte
without drifting into app-specific workstation logic.

## Package Rule

The `g04.009` tranche introduces `pug-gpui-workstation` with:

- `AppHeaderSpec`
- `ProjectHeaderSpec`
- `PanelSurfaceSpec`
- `PanelHeaderSpec`
- `PanelTabsSpec`
- `SurfaceTabsSpec`
- `DockRegionSpec`
- `SplitViewSpec`
- `CommandPaletteShellSpec`
- `CommandPaletteSpec`
- `ActionDiscoveryPanelSpec`
- `ShellStatusBarSpec`
- `WorkspaceShellSpec`

It also exposes workstation layout persistence helpers:

- `serialize_workspace_layout_snapshot`
- `parse_workspace_layout_snapshot`

## Contract Coverage Rule

The crate must stay aligned to the existing workstation contracts for:

- `app-header`
- `project-header`
- `panel-surface`
- `panel-header`
- `panel-tabs`
- `surface-tabs`
- `dock-region`
- `split-view`
- `command-palette-shell`
- `command-palette`
- `action-discovery-panel`
- `shell-status-bar`
- `workspace-shell`

## Workstation Shell Rule

This baseline freezes the shared workstation shell posture that later GPUI
apps must inherit:

- app-level and project-level header identity remain distinct
- panel chrome remains generic and contract-owned
- surface tabs and panel tabs keep their separate shell roles
- dock and split layout meaning stays explicit through snapshots and ratios
- the top-level workspace shell remains the ownership boundary for utility
  overlays and shell layout regions

## Command Rule

This tranche also freezes the shell-level command-discovery posture:

- modal command-launcher shell semantics remain distinct from ranked results
- grouped inline discovery remains visible outside the modal launcher
- active command meaning, grouped sections, and disabled actions stay explicit
- shell-level invocation and restoration posture remain contract-owned

## Persistence Rule

The workstation baseline must preserve a stable snapshot posture:

- dock edges remain explicit
- active surface and dock panel identity remain explicit
- split ratios remain serializable and parseable in a stable host-owned format
- persistence helpers may stay lightweight, but the public snapshot meaning
  must remain aligned with Svelte

## Runtime Honesty Rule

This tranche remains explicit about current depth:

- shell-region hierarchy, command posture, dock or split state, and persistence
  snapshots are explicit
- native drag regions, menu ownership, multi-window shell movement, focus
  traps, and final accessibility proof still belong to later `g04` milestones

The repo may expose these workstation shells as contract-backed GPUI specs
before every one of them is rendered as a fully mounted native shell.

## Token Rule

These workstation shells must continue resolving from the existing GPUI token,
primitive, and composite baselines for at least:

- shell background, border, and elevation roles
- tab-strip, panel-surface, and overlay roles
- spacing and density cadence across headers, docks, and status bars
- command palette overlay and results hierarchy
- canvas versus panel versus elevated shell separation

## Seed Evidence

- `packages/gpui/workstation-shell-command-layout-baseline.json`
- `packages/gpui/workstation/Cargo.toml`
- `packages/gpui/workstation/README.md`
- `packages/gpui/workstation/src/lib.rs`
- `packages/gpui/workstation/src/types.rs`
- `packages/gpui/workstation/src/persistence.rs`
- `packages/gpui/workstation/src/app_header.rs`
- `packages/gpui/workstation/src/project_header.rs`
- `packages/gpui/workstation/src/panel_surface.rs`
- `packages/gpui/workstation/src/panel_header.rs`
- `packages/gpui/workstation/src/panel_tabs.rs`
- `packages/gpui/workstation/src/surface_tabs.rs`
- `packages/gpui/workstation/src/dock_region.rs`
- `packages/gpui/workstation/src/split_view.rs`
- `packages/gpui/workstation/src/command_palette_shell.rs`
- `packages/gpui/workstation/src/command_palette.rs`
- `packages/gpui/workstation/src/action_discovery_panel.rs`
- `packages/gpui/workstation/src/shell_status_bar.rs`
- `packages/gpui/workstation/src/workspace_shell.rs`

## Next Task

Carry this first GPUI workstation baseline into `g04.010`, hardening native
accessibility, focus, keyboard, and assistive-technology proof on top of the
widened primitive, composite, and workstation surface.
