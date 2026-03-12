# 036 Command And Data Foundation Boundary

Status: active
Updated: 2026-03-12
Depends on: `010-data-table-selection-bulk-action-and-virtualization-rules.md`, `016-command-palette-and-action-discovery-rules.md`, `029-advanced-primitive-promotion-and-substrate-mapping.md`

## Purpose

The remaining widening ambiguity is no longer the date family. It is whether
command and data surfaces should enter foundation or stay in composite or
workstation ownership. This spec resolves that by promoting only the truly
low-level data pieces and keeping command discovery out of foundation.

## Promote To Foundation

The current low-level data tranche promotes:

- `Table`
- `Pagination`

These are generalized enough to serve broad app needs without absorbing
sorting, selection, browse summaries, or workstation command semantics.

## Keep Outside Foundation

The following remain outside foundation:

- `CommandPalette`
- `CommandPaletteShell`
- `DataTable`
- `PaginationSummary`

Those surfaces own richer ranking, browse-state, sorting, selection, or shell
semantics that exceed the low-level primitive layer.

## Ownership Split Rule

`Table` owns:

- low-level structured row and cell relationships
- caption and empty posture
- row-header meaning

`Pagination` owns:

- page-to-page navigation
- current-page state
- boundary and truncated-window controls

`DataTable` still owns:

- sorting
- selection
- row actions
- richer browse posture

Workstation command surfaces still own:

- ranking and grouped command results
- query and command-discovery workflow
- shell-level focus trap and launcher posture

## Accessibility Rule

Foundation data primitives must preserve:

- real structured table semantics
- current-page semantics and boundary control state

Foundation must not claim command-discovery meaning when the real contract is a
workstation overlay with grouped, ranked, host-owned actions.

## Current Risk

This tranche closes the low-level data gap, but richer data interaction and
command-discovery ownership remain intentionally above foundation. That is a
feature, not a hole: it keeps the primitive layer honest.

## Evidence

- `docs/contracts/foundation/table.md`
- `docs/contracts/foundation/pagination.md`
- `docs/contracts/composites/data-table.md`
- `docs/contracts/workstation/command-palette.md`
- `packages/svelte/primitives/README.md`

## Next Task

Freeze the primitive widening for a moment and assess which remaining families
are truly missing from foundation versus already properly owned in composites
or workstation surfaces.
