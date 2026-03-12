# 018 Dock Split View Tabs And Persistence Orchestration Rules

Status: active
Updated: 2026-03-11
Depends on: `006-workstation-shell-and-panel-system-rules.md`, `017-app-shell-and-workspace-shell-depth-rules.md`

## Purpose

Freeze the next workstation-shell rules for dock orchestration, split-view
composition, tab restore semantics, and persistence snapshot shape so
downstream apps can build dense workspaces without inventing incompatible shell
engines.

## Orchestration Layer Rule

This tranche defines orchestration posture above the shell-depth baseline.

It may define:

- how surface tabs, dock panels, and split regions compose into one workspace
- what reorder, close, collapse, and restore intents mean
- what state must survive persistence snapshots

It does not define:

- a drag-and-drop engine
- storage or sync backends
- cross-window movement policy
- app-specific panel graphs

## Surface Tab Rule

`SurfaceTabs` remain the top-level workspace surface switcher.

At minimum they must preserve:

- active surface identity
- deterministic focus when tabs reorder or close
- add, rename, move, and close intents as explicit host callbacks
- keyboard selection and reordering posture

Closing one surface must restore focus and active selection predictably.

## Dock Region Rule

Dock regions preserve local panel groups without redefining the meaning of the
active center surface.

Dock orchestration must keep explicit:

- dock edge
- active panel within that dock
- ordered panel list
- collapsed or expanded state
- restore posture after close or collapse

Empty docks are valid.
They should stay labeled instead of collapsing into anonymous chrome.

## Split View Rule

Split layouts may compose recursively.

For multi-region workstation shells the persisted model must preserve each
meaningful divider separately.

The current baseline uses:

- a primary split ratio for left dock versus the remainder
- a secondary split ratio for center surface versus right dock

Resizable dividers must remain keyboard-operable and focusable.

## Persistence Snapshot Rule

The shell-level persistence snapshot may serialize:

- active surface
- surface order
- primary and secondary split ratios
- left and right dock snapshots

It must not require one storage backend.

Serialization is a contract boundary, not a persistence engine.

## Restore Rule

Hosts restoring a saved workspace should restore:

- active surface before rendering surface-local context
- dock panel order before focusable panel actions appear
- collapsed state without losing dock naming
- split ratios before the shell presents itself as stable

If part of a saved layout cannot be restored, the host should degrade
deterministically and preserve visible recovery meaning.

## Accessibility Rule

Both runtimes must preserve:

- keyboard reachability for surface tabs, panel tabs, and split dividers
- focus continuity after reorder, close, collapse, expand, and restore
- named dock and panel regions
- explicit state exposure for collapsed regions
- non-pointer access to resize semantics

Svelte should use native tabbable controls and region semantics where possible.
GPUI must recreate equivalent naming, ordering, and divider semantics in the
native accessibility tree.

## Seed Evidence

- `docs/contracts/workstation/panel-tabs.md`
- `docs/contracts/workstation/surface-tabs.md`
- `docs/contracts/workstation/dock-region.md`
- `docs/contracts/workstation/split-view.md`
- `packages/svelte/workstation/src/PanelTabs.svelte`
- `packages/svelte/workstation/src/SurfaceTabs.svelte`
- `packages/svelte/workstation/src/DockRegion.svelte`
- `packages/svelte/workstation/src/SplitView.svelte`
- `packages/svelte/workstation/src/persistence.ts`
- `packages/svelte/preview/src/App.svelte`

## Next Task

Carry this workstation orchestration baseline into `g02.011` and harden focus,
keyboard, and state semantics consistently across the now-broader catalogue.
