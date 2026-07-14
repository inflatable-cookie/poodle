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
  TimeInput composition, verified). DateTimeRangePicker
  (range calendar + per-end TimeInputs) and DateTimeZonePicker (+
  TimeZoneSelect: searchable Select over defaultTimeZoneOptions) done —
  the whole date-picker family is ported. Verified: range completion with
  end time, zoned pick via timezone search.
- [x] Wave 3: EditableList (applyReorder + listReorderKeyIntent keyboard
  grab/move/drop, drag, windowing), LogList (stream + audit variants),
  Tree (full WAI-ARIA pattern over the tree machinery: treeKeydownIntent
  nav, expand/collapse with lazy loadChildren, checkbox cascade via
  treeCheckState/treeToggleCheck with native indeterminate, shift-range
  via treeRangeSelection, F2 inline rename, Alt-arrow sibling reorder,
  drag-drop before/after/inside, opt-in virtual windowing via
  treeVirtualWindow). Checkbox upgraded from pilot stub to full parity
  (id/size/density/selectedColor/mixed-indeterminate/parts records).
  Verified 7/7 tree probes + 3/3 editable-list + 2/2 log-list.
- [ ] Wave 4: DataTable, ListCard
