# g11.001 Reorganize Misplaced Components

Status: complete
Owner: Poodle Core

## Context

6 components were in `packages/gpui/components/src/composites/` but belong in
`primitives/` per the Svelte directory structure and updated contracts.

## Completed Actions

- [x] Moved breadcrumbs, list_card, nav_card, nav_card_grid, order_by,
      pagination_summary from `composites/` to `primitives/`
- [x] Updated `composites/mod.rs` — removed 6 module declarations and re-exports
- [x] Updated `primitives/mod.rs` — added 6 module declarations and re-exports
- [x] `cargo check -p poodle-gpui-preview` — clean
