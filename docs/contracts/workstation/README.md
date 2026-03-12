# Workstation Contracts

Status: active
Updated: 2026-03-11

Workstation contracts define reusable desktop and pro-tool shell patterns that
sit above product composites but stay below app-specific workstation widgets.

## Current Contracts

- `panel-surface.md`
- `app-header.md`
- `project-header.md`
- `panel-header.md`
- `panel-tabs.md`
- `surface-tabs.md`
- `dock-region.md`
- `split-view.md`
- `workspace-shell.md`
- `command-palette-shell.md`
- `command-palette.md`
- `action-discovery-panel.md`
- `shell-status-bar.md`

## Boundary Rule

Workstation contracts may own:

- app and project shell headers
- panel headers, panel tabs, and dock regions
- split layouts and workspace shell region expectations
- shell-level command palette posture

They may not own:

- DAW-specific transport or timeline widgets
- mixer strips or audio-console semantics
- project-specific workspace orchestration logic
- storage backends for persistence

## Next Task

Use this workstation baseline while executing `g02.011` and later workstation
or hardening tranches, without reopening the shell boundary already frozen in
`g01`.
