# g02.002 Data Table And Bulk-Action Suite

Status: completed
Date: 2026-03-11
Owner: Poodle Core

## Summary

- completed `g02.002`
- added table-suite contracts at
  `docs/contracts/composites/data-table.md`,
  `docs/contracts/composites/bulk-action-bar.md`, and
  `docs/contracts/composites/pagination-summary.md`
- added the normative table-selection and virtualization baseline at
  `docs/specs/010-data-table-selection-bulk-action-and-virtualization-rules.md`
- created the first Svelte composites package at `packages/svelte/composites`
- added a `Checkbox` implementation to `packages/svelte/primitives`
- extended the preview with sorting, filtering, visible-scope row selection,
  bulk actions, row actions, and pagination

## Validation

- `bun install`
- `bun run preview:build`
- `bun run tokens:build`
- `git diff --check`

## Notes

- the current table baseline keeps filtering, sorting, pagination, and action
  execution host-owned
- virtualization is now explicitly documented as a concern, but not required
  implementation in this tranche

## Next Task

Open `docs/roadmaps/g02/003-lists-grids-filters-pagination-and-search-depth.md`
and build the broader browse-shell depth above the completed table baseline.
