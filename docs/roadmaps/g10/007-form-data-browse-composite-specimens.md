# g10.007 — Form, Data, and Browse Composite Specimens

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.006
Primary repos: `pug`

## Goals

- [ ] create per-component specimens for form, data, and browse composites

## Execution Checklist

- [ ] create `data_table.rs` — DataTable with header row, sortable columns,
  data rows, and selection
- [ ] create `list_shell.rs` — ListShell showing empty, loading, error,
  and populated states
- [ ] create `grid_shell.rs` — GridShell with responsive card grid
- [ ] create `detail_shell.rs` — DetailShell with header and content sections
- [ ] create `detail_section.rs` — DetailSection with collapsible content
- [ ] create `detail_row.rs` — DetailRow with label-value pairs
- [ ] create `filter_toolbar.rs` — FilterToolbar with search and filter chips
- [ ] create `picker_shell.rs` — PickerShell with search and selection list
- [ ] create `relation_picker.rs` — RelationPicker with entity search
- [ ] create `selection_summary.rs` — SelectionSummary with count and clear
- [ ] create `pagination_summary.rs` — PaginationSummary with page controls
- [ ] create `bulk_action_bar.rs` — BulkActionBar with contextual actions
- [ ] create `order_by.rs` — OrderBy with field selector and direction
- [ ] create `form_dialog.rs` — FormDialog with form fields and validation
- [ ] create `confirm_action.rs` — ConfirmAction with destructive and safe
  variants
- [ ] register all modules and wire slug routing
- [ ] verify all 15 specimens render without panic

## Acceptance Criteria

- [ ] all 15 composite specimens render in the preview app
- [ ] DataTable displays a functioning table with header and rows
- [ ] dialog composites (FormDialog, ConfirmAction) open via ScreenStack
- [ ] `cargo check` passes

## Next Task

Open `g10.008` and build editing, media, and operational composite specimens.
