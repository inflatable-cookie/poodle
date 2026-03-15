# g09.004 — Missing Composite Components: Form and Data

Status: complete
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.003
Primary repos: `pug`

## Goals

- [ ] implement composite component structs for form, data, and browse
  patterns that exist in Svelte but are missing from GPUI
- [ ] composites compose primitives into higher-order UI patterns

## Execution Checklist

- [ ] create `PugDataTable` composite — header row, sortable columns, data
  rows with selection, pagination integration
- [ ] create `PugListShell` composite — stateful list container (empty,
  loading, error, populated) with scroll and selection
- [ ] create `PugGridShell` composite — responsive card grid with gap and
  column control
- [ ] create `PugDetailShell` composite — detail page container with header,
  metadata, and content sections
- [ ] create `PugDetailSection` composite — labeled detail section with
  optional collapse
- [ ] create `PugDetailRow` composite — key-value detail row with label
  and value alignment
- [ ] create `PugFilterToolbar` composite — horizontal bar with search input,
  filter chips, and clear-all
- [ ] create `PugPickerShell` composite — searchable selection panel with
  list, search field, and confirmation actions
- [ ] create `PugRelationPicker` composite — entity relationship picker
  combining search, results list, and selected items display
- [ ] create `PugSelectionSummary` composite — selected item count, clear
  action, and bulk action triggers
- [ ] create `PugPaginationSummary` composite — page info display with
  page navigation controls
- [ ] create `PugBulkActionBar` composite — contextual action bar shown
  when items are selected
- [ ] create `PugOrderBy` composite — sort field selector with direction
  toggle
- [ ] create `PugFormDialog` composite — dialog containing a form with
  validation, submit, and cancel actions
- [ ] create `PugConfirmAction` composite — confirmation dialog with
  destructive/safe action buttons and message
- [ ] register all composites in `lib.rs`
- [ ] verify compilation with `cargo check`

## Acceptance Criteria

- [ ] all 15 composite components compile and are exported
- [ ] each composite composes existing `Pug*` primitives (not raw `div()`)
- [ ] DataTable supports header click for sort indication
- [ ] ListShell renders appropriate state (empty, loading, items)
- [ ] FormDialog and ConfirmAction support `on_confirm` and `on_cancel`
  callbacks

## Next Task

Open `g09.005` and implement editing, media, and operational composites.
