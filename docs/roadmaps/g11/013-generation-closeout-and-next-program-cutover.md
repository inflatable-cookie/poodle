# g11.013 — Generation Closeout And Next-Program Cutover

Status: complete
Owner: Pug Core
Updated: 2026-03-17
Depends on: g11.012
Primary repos: `pug`

## Goals

- [x] close `g11` with an explicit record of what the workstation substrate now
  owns
- [x] leave `g12` and `g13` with a clean contract baseline to implement against

## Execution Checklist

- [x] verify all `g11` milestones are complete
- [x] summarize the workstation substrate added by `g11`
- [x] record what still intentionally stays downstream
- [x] record what is deferred to `g12` (GPUI workstation parity) and `g13`
  (Jetstream workstation parity)
- [x] update roadmap front-door docs and generation index

## Generation Summary

### What g11 Added

**New generalized workstation components (Svelte):**
- `WorkspaceWindow` — multi-window host with surface ownership
- `StripRail` — four-edge strip with icon/mixed modes
- `ResizeHandle` — standalone resize interaction primitive
- `CollapseAffordance` — directional collapse/expand trigger
- `SplitDivider` — resize + collapse composition
- `HostedSurface` — external content host with 5 bounded states

**Deepened existing components:**
- `DockRegion` — active-panel emphasis (standard/quiet/strong), collapsed-tab
  posture (hidden/icon-strip)
- `PanelSurface` — variant system (utility/standard/focused)
- `SurfaceTabs` — window-aware identity (windowId)
- `PanelTabs` — dock-local context (dockId)

**New contracts:** workspace-window, workspace-layout, strip-rail,
resize-handle, split-divider, collapse-affordance, hosted-surface

**Expanded type surface:** v2 layout snapshot with 10 region keys, strip/dock/
center region snapshots, backwards-compatible with v1

### What Stays Downstream

- DAW semantics (transport, timeline, mixer, automation, clip editor)
- Window management policy (positions, sizes, defaults)
- Panel assignment policy (which panels go where)
- Strip item semantics (which icons, what they activate)
- Plugin lifecycle (discovery, installation, routing)
- Command routing and keyboard shortcut assignments
- Project identity and persistence

### What Is Deferred

- **g12:** GPUI implementation of all g11 contracts, Svelte/GPUI parity
  evidence, Loophole Spark adoption proof
- **g13:** Jetstream implementation (within rendering constraints),
  cross-runtime three-way parity evidence, delta register update

## Acceptance Criteria

- [x] `g11` is explicitly closed
- [x] the shared-versus-downstream workstation boundary is plainly recorded
- [x] `g12` and `g13` can begin from documented contracts without reopening
  scope

## Next Task

Open `g12` and begin GPUI workstation parity implementation.
