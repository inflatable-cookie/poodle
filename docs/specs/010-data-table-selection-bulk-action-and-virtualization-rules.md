# 010 Data Table, Selection, Bulk Action, And Virtualization Rules

Status: active
Updated: 2026-03-11
Depends on: `003-accessibility-and-assistive-technology-baseline.md`, `005-product-composite-composition-and-information-architecture-rules.md`, `009-form-shell-validation-and-action-row-rules.md`

## Purpose

Freeze the first rules for tabular browse surfaces so sorting, row selection,
bulk actions, pagination, and future virtualization all share one documented
meaning.

## Structured Data Rule

When content is genuinely tabular, Pug should use table semantics rather than a
generic list disguised as rows and columns.

This requires:

- explicit column headers
- stable cell-to-header relationships
- real sort meaning
- real selection-control meaning

## Host Ownership Rule

The table composite may expose:

- sort requests
- row-selection requests
- select-all requests
- row-action requests

It does not own:

- query execution
- persistence
- server fetch policy
- destructive confirmation flows
- or data mutations

Those remain host-owned.

## Visible-Scope Selection Rule

Select-all semantics must be explicit about scope.

This baseline defines the default scope as the currently visible row set.

If a host wants “all filtered rows across all pages” behavior, that broader
scope must be explicitly documented and surfaced in UI copy instead of being
implied by the same checkbox.

## Bulk Action Rule

Bulk action surfaces are selection-reactive composites, not permanent browse
toolbars.

They must:

- state how many rows/items are selected
- preserve action order and naming
- provide a clear-selection affordance
- avoid icon-only summaries

## Pagination Interplay Rule

Pagination and selection must not silently invalidate each other.

At minimum:

- page changes may preserve or clear off-page selection, but the policy must be
  explicit
- range summary must remain textual
- previous/next disabled state must be explicit at page boundaries

This baseline allows selection to persist across pages so long as the current
visible scope is still clearly communicated.

## Virtualization Rule

Virtualization is a documented concern starting in `g02.002`, but not a
required implementation in this baseline.

The rules are:

- virtualization strategy stays implementation detail
- table semantics, sort meaning, and selection meaning may not disappear when
  virtualization is introduced
- GPUI and Svelte may realize large-data rendering differently, but they must
  preserve the same user-facing scope rules and accessibility semantics

## Accessibility Rule

Both runtimes must preserve:

- structured table meaning
- sort state
- row-selection state
- bulk-action summary meaning
- pagination range and boundary semantics

Svelte should use native table and checkbox/button behavior first.
GPUI must explicitly recreate the same meaning in the native accessibility
tree and event system.

## Seed Evidence

- `docs/contracts/composites/data-table.md`
- `docs/contracts/composites/bulk-action-bar.md`
- `docs/contracts/composites/pagination-summary.md`
- `packages/svelte/primitives/src/Checkbox.svelte`
- `packages/svelte/composites/src/DataTable.svelte`
- `packages/svelte/composites/src/BulkActionBar.svelte`
- `packages/svelte/composites/src/PaginationSummary.svelte`
- `packages/svelte/preview/src/App.svelte`

## Next Task

Carry this baseline into `g02.003` so list, grid, filter, search, and
pagination depth reuse one browse and selection posture.
