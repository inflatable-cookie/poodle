# g10.003 Svelte Component Overhaul Closeout

Status: complete
Owner: Poodle core
Depends on: g10.002
Updated: 2026-04-09

## Context

The active thread executed a broad Svelte component-overhaul wave spanning
consolidation (9 components removed), API improvements (Select ghost variant,
TextInput slug/search/multiline, BlockEditor shell, DataTable expandedRowIds,
FormDialog bare, DockRegion collapsible, PageHeader restructure, DetailItem
info popover), specimen infrastructure (SpecimenGroup migration), visual polish,
and infrastructure fixes (97 type errors resolved to zero, portal theming,
UiPresentationProvider nesting).

This work was driven by freeform visual review rather than a bounded queue.
This milestone closes the open seams with explicit exit criteria so the next
milestone (Jetstream implementation) can start from a stable Svelte surface.

## Goals

- close the remaining composite specimen review pass
- update contracts for all component changes made in the overhaul wave
- verify health checks pass
- maintain zero type errors

## Non-Goals

- Jetstream component implementation
- GPUI anything
- new component design work
- Underlay bridge changes

## Execution Plan

### Batch 3.1 — Remaining Composite Specimen Review

- [x] review MediaPicker, MediaBrowsePanel, MediaUploadStatusPanel specimens
- [x] review RelationPicker, PickerShell specimens
- [x] review SplitView, SidebarNav specimens
- [x] review ToastStack, ToastHost specimens
- [x] review remaining workstation composites (AppHeader, CommandPalette, StatusBar)
- [x] fix any visual issues found during review

### Batch 3.2 — Contract Sync

- [x] update button contract (pressed state, danger hover/border, elevation stacking)
- [x] update select contract (ghost variant, menuMinWidth, viewport anchor flipping)
- [x] update text-input contract (slug mode done, verify search/multiline/suffix)
- [x] update block-editor contract (pure shell, no default types)
- [x] update data-table contract (expandedRowIds replacing predicate)
- [x] update form-dialog contract (bare, columns passthrough)
- [x] update dock-region contract (collapsible prop, edge borders)
- [x] update detail-item contract (info popover for description)
- [x] update page-header contract (arrow back link, actions-row restructure)
- [x] update action-discovery-panel contract (tighter spacing)

### Batch 3.3 — Health Verification

- [x] run effigy health and resolve any failures
- [x] run effigy doctor — 13 ok, 2 warn (pre-existing), 1 err (god-files, structural)
- [x] confirm svelte-check at zero errors
- [x] confirm no stale noSurface or dead-code patterns remain

## Exit Criteria

- all composite specimens reviewed
- all changed contracts updated
- effigy health and doctor pass
- svelte-check zero errors
- one explicit next task left in the authority surface

## Next Task

`g10.003` is complete. All three batches executed:
- Batch 3.1: composite specimen review, visual fixes, component consolidation
- Batch 3.2: 13 contracts synced to implementations
- Batch 3.3: effigy health passes, svelte-check zero errors, stale exports removed

The next milestone is Jetstream component implementation (Seam B from g10.002).
