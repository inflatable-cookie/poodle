# DataTable

Status: contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `DataTable`
- Layer: `composites`
- Summary: a tabular browse surface for structured rows with sorting,
  row-selection, column visibility, CSV export, and row-action affordances
- In scope: column headers, sortable headers, row selection, row actions, empty
  posture, column visibility popover, CSV export, table semantics
- Out of scope: domain-specific cell renderers, in-cell editing, full
  spreadsheet behavior, virtualization implementation details

## 2. Types

### TableColumn

```ts
type TableColumn = {
  id: string;
  label: string;
  align?: "start" | "end";
  isSortable?: boolean;
  isHideable?: boolean;
};
```

### TableRow

```ts
type TableRow = {
  id: string;
  cells: Record<string, string>;
  summary?: string | null;
};
```

### TableSortDirection

```ts
type TableSortDirection = "asc" | "desc";
```

## 3. Anatomy

```text
[Root]
  ├── [Toolbar]  (optional, when showColumnVisibility or showExport)
  │     ├── [Export Button]  (optional)
  │     └── [Column Visibility Popover]  (optional)
  │           └── [Column Menu]  role="menu"
  │                 └── [Column Menu Item...]  Checkbox + label
  └── [Table]  aria-label
        ├── [Caption]  visually hidden, accessible
        ├── [Header Row]
        │     ├── [Select-All Cell]  Checkbox
        │     ├── [Column Header...]
        │     │     └── [Sort Button]  (if isSortable)
        │     └── [Actions Header]  (optional)
        └── [Body]
              └── [Row...]  aria-selected
                    ├── [Selection Cell]  Checkbox
                    ├── [Data Cell...]
                    │     └── [Cell Content]
                    │           ├── <span> value
                    │           └── <small> summary  (optional, first column only)
                    └── [Row Actions Cell]  (optional)
                          └── <button> rowActionLabel
```

## 4. Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `columns` | `TableColumn[]` | none | yes | column definitions |
| `rows` | `TableRow[]` | none | yes | current visible rows |
| `selectedRowIds` | `string[]` | `[]` | no | controlled selection |
| `sortColumnId` | `string \| null` | `null` | no | current sort column |
| `sortDirection` | `TableSortDirection` | `"asc"` | no | current sort direction |
| `rowActionLabel` | `string` | `"Open"` | no | label for row action buttons |
| `showRowActions` | `boolean` | `true` | no | row action column visibility |
| `emptyMessage` | `string` | `"No rows match the current view."` | no | empty posture copy |
| `ariaLabel` | `string` | `"Data table"` | no | accessible table name |
| `hiddenColumnIds` | `string[]` | `[]` | no | ids of columns currently hidden |
| `showColumnVisibility` | `boolean` | `false` | no | show column visibility toggle in toolbar |
| `showExport` | `boolean` | `false` | no | show CSV export button in toolbar |
| `exportFilename` | `string` | `"export.csv"` | no | filename for CSV download |

## 5. Events

| Event | When It Fires | Payload |
|-------|---------------|---------|
| `sortChange` | sortable header clicked | `{ columnId: string; direction: TableSortDirection }` |
| `rowToggle` | individual row selection changes | `{ rowId: string; selected: boolean }` |
| `toggleAll` | select-all checkbox changes | `{ selected: boolean }` |
| `rowAction` | row action button clicked | `{ rowId: string }` |
| `columnVisibilityChange` | column visibility toggled | `{ columnId: string; visible: boolean }` |
| `exportCsv` | CSV export completed | `{ filename: string }` |

## 6. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | rows present | table body visible |
| empty | no rows | empty row with `emptyMessage` spanning all columns |
| partially selected | some visible rows selected | select-all checkbox in mixed state |
| fully selected | all visible rows selected | select-all checkbox checked |
| sorted | active sort column present | sort icon (arrow-up/arrow-down) visible on active header |
| toolbar visible | `showColumnVisibility` or `showExport` is true | toolbar row above table |

## 7. Accessibility

### Semantics

- Native `<table>` markup with `aria-label`
- Visually hidden `<caption>` providing table name and selection count
- Sort headers: `aria-sort` attribute (`ascending`, `descending`, or `none`)
- First data column uses `<th scope="row">`; remaining use `<td>`
- Row selection: `aria-selected` on `<tr>`
- Select-all: describes "all visible rows"
- Row action buttons: `aria-label` includes row context

### Keyboard

- `Tab`: moves through sortable headers, selection controls, and row-action
  triggers in DOM order
- `Space`: toggles row selection when checkbox is focused
- `Enter` or `Space`: activates sortable headers and row actions when focused

### Focus

- Focus-visible ring on sort buttons and row action buttons
- Sort changes and selection changes do not move focus

## 8. Toolbar

When `showExport` or `showColumnVisibility` is true, a toolbar renders above
the table:

- **Export button**: icon (download) + "Export" label; triggers client-side CSV
  generation and download, then dispatches `exportCsv`
- **Column visibility**: Popover (placement: bottom-end) with Checkbox list of
  hideable columns (`isHideable !== false`); toggling dispatches
  `columnVisibilityChange`
- Only columns where `isHideable` is not explicitly `false` appear in the
  visibility menu
- Hidden columns are excluded from both rendering and CSV export

## 9. Composition

- Composes: `Checkbox`, `Icon`, `Popover`
- Parent expectations: `FilterToolbar`, `SearchField`, `BulkActionBar`,
  pagination summary
- Host ownership: filtering, sorting implementation, pagination, and
  persistence all stay host-owned

## 10. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Table shell | `background-panel`, `border-subtle`, `radius-surface` | table frame |
| Toolbar | `background-elevated` | toolbar distinction |
| Header row | `text-secondary`, `background-elevated` | header distinction |
| Selected row | `accent-base` at 8% | selection emphasis |
| Hover row | `accent-base` at 5% | hover feedback |
| Sort/action buttons | `border-width-focus`, `accent-focusRing` | focus ring |

## 11. Specimen Definitions

### With Sorting, Column Visibility, And Export

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With sorting, column visibility, and export | columns: Name (sortable), Email (sortable), Role (sortable), Status (not sortable); 5 rows; `showColumnVisibility`, `showExport`; selection enabled | table with sortable column headers, select-all checkbox, row selection checkboxes, column visibility toggle, export action, and row actions; status bar shows selection count |

### Empty State

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Empty state | same columns, `rows=[]`, `emptyMessage="No team members match the current filters."` | table header visible with empty-state message in body |
