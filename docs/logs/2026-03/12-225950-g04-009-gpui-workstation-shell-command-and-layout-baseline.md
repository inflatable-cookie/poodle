---
title: g04.009 gpui workstation shell command and layout baseline
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, gpui, rust, workstation]
---

## Summary

Completed `g04.009` by introducing `pug-gpui-workstation` and freezing the
first GPUI workstation shell, command-discovery, and layout-orchestration
baseline.

## What changed

- added the normative baseline `docs/specs/056-gpui-workstation-shell-command-and-layout-baseline.md`
- completed `docs/roadmaps/g04/009-gpui-workstation-shell-command-discovery-and-layout-orchestration-parity.md`
- added the machine-readable artifact `packages/gpui/workstation-shell-command-layout-baseline.json`
- introduced `packages/gpui/workstation` with:
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
- added shared workstation support types for command actions, discovery
  sections, shell states, tab items, dock snapshots, and split orientation
  plus JSON layout persistence helpers
- pinned generic shell headers, panel chrome, dock and split state, shell
  command-discovery posture, and workspace snapshot semantics inside the GPUI
  workstation layer so downstream apps inherit the same contract semantics as
  Svelte
- added crate tests for shell headers, panel chrome, dock and split state,
  command discovery, shell status posture, and workspace layout snapshot
  round-tripping
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new GPUI
  workstation baseline artifact is machine-checked
- updated release metadata, package surfaces, and roadmap status so the repo
  now points at `g04.010`

## Validation

- `cargo fmt --manifest-path packages/gpui/workstation/Cargo.toml`
- `cargo check --manifest-path packages/gpui/workstation/Cargo.toml`
- `cargo test --manifest-path packages/gpui/workstation/Cargo.toml`
- `bun run --cwd packages/svelte/preview docs:lint`
- `bun run --cwd packages/svelte/preview build`
- `git diff --check`

## Outcome

`g04.009` is now explicit. Pug has a first real GPUI workstation crate for the
shared shell, command, dock, split, and snapshot layer, which materially
reduces the remaining “Svelte-only by default” gap before accessibility proof
and downstream adoption evidence.

## Next

Open `g04.010` and harden GPUI native accessibility, focus, keyboard, and
assistive-technology proof on top of the widened primitive, composite, and
workstation surface.
