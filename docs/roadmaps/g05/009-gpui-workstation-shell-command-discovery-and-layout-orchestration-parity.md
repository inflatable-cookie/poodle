# g05.009 GPUI Workstation Shell, Command Discovery, And Layout Orchestration Parity

Status: completed
Owner: Flint Core
Updated: 2026-03-12
Depends on: g05.003, g05.006, g05.008
Primary repos: `flint`

## Goals

- [x] implement the GPUI workstation shell and command-discovery layer
- [x] prove that the generic workstation foundation can exist natively instead
  of remaining Svelte-only guidance

## Execution Checklist

- [x] implement GPUI shell headers, panel surfaces, tabs, command palette, and
  dock or split orchestration against the current workstation contracts
- [x] align layout persistence helpers and shell state posture to shared semantics
- [x] document native-shell deltas such as windowing, menu ownership, and drag regions
- [x] verify the workstation layer is credible enough for downstream GPUI adoption proofs

## Acceptance Criteria

- [x] GPUI workstation shell posture is explicit
- [x] GPUI command and layout orchestration parity posture is explicit

## Completed Work

- added the normative baseline `docs/specs/056-gpui-workstation-shell-command-and-layout-baseline.md`
- added the machine-readable artifact `packages/gpui/workstation-shell-command-layout-baseline.json`
- introduced `packages/gpui/workstation` as the first public-intent GPUI workstation crate with:
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
- added shared workstation support types for command actions, discovery sections, shell states, tab items, dock snapshots, and split orientation plus JSON layout persistence helpers
- froze generic shell headers, panel chrome, dock and split state, shell command-discovery posture, and workspace snapshot semantics inside the GPUI workstation layer so downstream apps inherit the same contract semantics as Svelte
- added crate tests for shell headers, panel chrome, dock and split state, command discovery, shell status posture, and workspace layout snapshot round-tripping
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new GPUI workstation baseline artifact is machine-checked
- updated release metadata, package surfaces, and roadmap status so the repo now points at `g05.010`

## Next Task

Open `g05.010` and harden GPUI native accessibility, focus, keyboard, and
assistive-technology proof.
