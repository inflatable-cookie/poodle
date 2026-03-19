# DataTable

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `DataTable`
- Layer: `composites`
- Summary: a tabular browse surface for structured rows with sorting,
  row-selection, and row-action affordances
- In scope: column headers, sortable headers, row selection, row actions, empty
  posture, table semantics
- Out of scope: domain-specific cell renderers, in-cell editing, full
  spreadsheet behavior, virtualization implementation details

## 2. Anatomy

```text
[Root]
  └── [Table]
        ├── [Header Row]
        │     ├── [Select-All Cell]
        │     ├── [Column Header...]
        │     └── [Actions Header] (optional)
        └── [Body]
              └── [Row...]
                    ├── [Selection Cell]
                    ├── [Data Cell...]
                    └── [Row Actions Cell] (optional)
```

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `columns` | `Array<{ id: string; label: string; align?: "start" \| "end"; isSortable?: boolean }>` | none | yes | canonical columns |
| `rows` | `Array<{ id: string; cells: Record<string, string>; summary?: string \| null }>` | none | yes | current visible rows |
| `selectedRowIds` | `string[]` | `[]` | no | controlled visible/filtered selection |
| `sortColumnId` | `string \| null` | `null` | no | current sort column |
| `sortDirection` | `"asc" \| "desc"` | `"asc"` | no | current sort direction |
| `rowActionLabel` | `string` | `"Open"` | no | action-copy baseline |
| `showRowActions` | `boolean` | `true` | no | row action column visibility |
| `emptyMessage` | `string` | implementation copy | no | empty posture copy |
| `ariaLabel` | `string` | `"Data table"` | no | accessible table name |

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | rows present | table body visible |
| empty | no rows | empty row/callout posture visible |
| partially selected | some visible rows selected | select-all control mixed |
| fully selected | all visible rows selected | select-all control checked |
| sorted | active sort column present | active sort indicator visible |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onSortChange` | sortable header requested | `{ columnId, direction }` | host owns data resort |
| `onRowToggle` | row selection changes | `{ rowId, selected }` | controlled selection |
| `onToggleAll` | select-all changes | `{ selected }` | current visible set only unless host expands scope |
| `onRowAction` | row action invoked | `{ rowId }` | host owns action menu or open behavior |

## 6. Accessibility

### Semantics

- Role: native table semantics for structured data
- Required behavior: stable header-to-cell relationships, row-selection naming,
  and explicit sort-state meaning
- the primary identifying column should remain a row header
- Selection rule: select-all semantics must describe the current visible scope,
  not silently imply every row across unrendered pages or virtualized segments
- Row-action rule: row actions need explicit accessible names tied to row
  context

### Keyboard

- `Tab`: moves through sortable headers, row-selection controls, and row-action
  triggers in DOM order
- `Space`: toggles row selection when selection control is focused
- `Enter` or `Space`: activates sortable headers and row actions when focused

### Focus And Announcement

- focus remains on the activated header/control rather than jumping into the
  table body unexpectedly
- sort changes and selection changes may be summarized by adjacent host-owned
  status copy or live status regions
- GPUI-native accessibility mapping notes: GPUI must intentionally recreate
  table/header/cell relationships, sort state, row selection state, and row
  action naming in the native accessibility tree

## 7. Composition

- parent expectations: `FilterToolbar`, `SearchField`, `BulkActionBar`,
  pagination summary, empty/loading/error shells
- child expectations: text-like cells in this baseline; richer cell renderers
  remain future work
- host ownership: filtering, sorting implementation, pagination, and
  persistence all stay host-owned

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Table shell | surface and border roles | table frame |
| Header row | subdued text and elevated background roles | header distinction |
| Selected row | accent/background roles | selection emphasis |
| Row/action controls | control, border, and focus roles | interaction affordances |

## 9. Svelte Notes

- should prefer real `<table>` markup
- sorting and selection callbacks stay host-controlled

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::data_table`
- GPUI implementation must not flatten structured table meaning into a generic
  scroll region with unlabeled text draws

## 11. Parity Checklist

- [ ] table/header/cell semantics match
- [ ] sort and selection meaning match
- [ ] visible-scope select-all semantics match
- [ ] row-action naming and keyboard semantics match

## 12. Specimen Definitions

All preview apps must render the following specimens identically.

### With sorting, column visibility, and export

A fully interactive data table with five rows of team-member data:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With sorting, column visibility, and export | columns: Name (sortable), Email (sortable), Role (sortable), Status (not sortable); 5 rows; `showColumnVisibility`, `showExport`; selection enabled | table with sortable column headers, select-all checkbox, row selection checkboxes, column visibility toggle, export action, and row actions; status bar shows selection count |

### Empty state

A data table with no rows:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Empty state | same columns, `rows=[]`, `emptyMessage="No team members match the current filters."` | table header visible with empty-state message in body |

## Next Task

Pair `DataTable` with `BulkActionBar` and pagination semantics instead of
folding those concerns directly into row or shell contracts.
