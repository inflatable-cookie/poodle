# g09.011 — Specimen Upgrade: Composites Batch 1 (Form, Data, Detail)

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.004
Primary repos: `pug`

## Goals

- [ ] break grouped composite specimens into per-component files using real
  Pug composite component instances
- [ ] eliminate `misc_composites.rs` catchall — every composite gets its own
  specimen file

## Execution Checklist

- [ ] create `data_table.rs` specimen using `PugDataTable` with sortable
  columns, row selection, and pagination
- [ ] split `detail_shell.rs` into separate files:
  - [ ] `detail_shell.rs` → `PugDetailShell` with header and content sections
  - [ ] `detail_section.rs` → `PugDetailSection` with collapsible content
  - [ ] `detail_row.rs` → `PugDetailRow` with label-value pairs
- [ ] create `list_shell.rs` specimen using `PugListShell` showing empty,
  loading, and populated states
- [ ] create `grid_shell.rs` specimen using `PugGridShell` with responsive
  card grid
- [ ] create `filter_toolbar.rs` specimen using `PugFilterToolbar` with
  search input and filter chips
- [ ] create `picker_shell.rs` specimen using `PugPickerShell` with search
  and selection
- [ ] create `relation_picker.rs` specimen using `PugRelationPicker` with
  entity search and selected items
- [ ] create `selection_summary.rs` specimen using `PugSelectionSummary`
  with count and clear action
- [ ] create `pagination_summary.rs` specimen using `PugPaginationSummary`
  with page navigation
- [ ] create `bulk_action_bar.rs` specimen using `PugBulkActionBar` with
  contextual actions
- [ ] create `order_by.rs` specimen using `PugOrderBy` with field selector
  and direction toggle
- [ ] create `form_dialog.rs` specimen using `PugFormDialog` with form
  fields and validation
- [ ] create `confirm_action.rs` specimen using `PugConfirmAction` with
  destructive and safe variants
- [ ] remove or archive `misc_composites.rs` — all its slugs now route to
  individual files
- [ ] update `mod.rs` with new module declarations and slug routing
- [ ] verify all composite slugs render without panic

## Acceptance Criteria

- [ ] `misc_composites.rs` is removed — no catchall specimen files remain
- [ ] every composite slug routes to a dedicated specimen file
- [ ] each specimen uses real `Pug*` composite component constructors
- [ ] `cargo check` passes for the preview crate

## Next Task

Open `g09.012` and upgrade remaining composite and workstation specimens.
