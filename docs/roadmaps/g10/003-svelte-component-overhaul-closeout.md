# g10.003 Svelte Component Overhaul Closeout

Status: pending
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

- [ ] review MediaPicker, MediaBrowsePanel, MediaUploadStatusPanel specimens
- [ ] review RelationPicker, PickerShell specimens
- [ ] review SplitView, SidebarNav specimens
- [ ] review ToastStack, ToastHost specimens
- [ ] review remaining workstation composites (AppHeader, CommandPalette, StatusBar)
- [ ] fix any visual issues found during review

### Batch 3.2 — Contract Sync

- [ ] update button contract (pressed state, danger hover/border, elevation stacking)
- [ ] update select contract (ghost variant, menuMinWidth, viewport anchor flipping)
- [ ] update text-input contract (slug mode done, verify search/multiline/suffix)
- [ ] update block-editor contract (pure shell, no default types)
- [ ] update data-table contract (expandedRowIds replacing predicate)
- [ ] update form-dialog contract (bare, columns passthrough)
- [ ] update dock-region contract (collapsible prop, edge borders)
- [ ] update detail-item contract (info popover for description)
- [ ] update page-header contract (arrow back link, actions-row restructure)
- [ ] update action-discovery-panel contract (tighter spacing)

### Batch 3.3 — Health Verification

- [ ] run effigy health and resolve any failures
- [ ] run effigy doctor and resolve any failures
- [ ] confirm svelte-check at zero errors
- [ ] confirm no stale noSurface or dead-code patterns remain

## Exit Criteria

- all composite specimens reviewed
- all changed contracts updated
- effigy health and doctor pass
- svelte-check zero errors
- one explicit next task left in the authority surface

## Next Task

Execute Batch 3.1: finish the composite specimen review pass.
