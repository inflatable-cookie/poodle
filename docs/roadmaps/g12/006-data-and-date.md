# g12.006 React Batch: Data And Date

Status: in progress (2026-07-14)
Owner: Poodle core
Depends on: `g12.005`

## Progress

- [x] Wave 1: Pagination (controller-or-props API, buildVisiblePages
  windows, limit selector, scroll-into-view), PaginationSummary, Table
  (row headers, alignment, empty state; full TableColumn/TableRow types
  shared with DataTable), ListGrid, Calendar (single + range modes, full
  keyboard grid via the shared date helpers — arrows/Home/End/PageUp/
  PageDown with cross-month focus handoff, month/year inline editing).
  Verified 7/7: page click + prev + ellipsis + summary window, table
  structure, grid display, day select, arrow+Enter keyboard select, month
  nav, range start..end.
- [x] Wave 2: DatePicker, DateRangePicker (Calendar composition: trigger
  label formatting, month follow, outside/Escape dismissal, close on
  complete selection). Verified 3/3. DateTimePicker done (Calendar +
  TimeInput composition, verified). DateTimeRangePicker + DateTimeZonePicker
  -> wave 2c.
- [ ] Wave 3: EditableList, LogList, Tree
- [ ] Wave 4: DataTable, ListCard
