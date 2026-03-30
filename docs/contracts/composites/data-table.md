# DataTable

Status: contract
Updated: 2026-03-27

## 1. Purpose

- Component name: `DataTable`
- Layer: `composites`
- Summary: a tabular browse surface for structured rows with sorting,
  row-selection, column visibility, CSV export, custom cell rendering,
  expanded rows, host-owned filters, pagination, loading rows, and row-action
  affordances
- In scope: column headers, sortable headers, filter row, row selection, row
  actions, empty posture, loading posture, custom cell rendering, expanded
  rows, row click, pagination footer, column visibility popover, CSV export,
  table semantics, compact/striped/sticky presentation
- Out of scope: domain-specific cell renderers, in-cell editing, full
  spreadsheet behavior, virtualization implementation details

## 2. Types

### TableColumn

```ts
type TableColumn = {
  id: string;
  label: string;
  align?: "start" | "center" | "end";
  sortable?: boolean;
  hideable?: boolean;
  width?: string;
  minWidth?: string;
  hideOnMobile?: boolean;
  isRowHeader?: boolean;
  filterable?: boolean;
  filterType?: "text" | "select" | "date";
  filterOptions?: Array<{ value: string; label: string } | string>;
};
```

### TableRow

```ts
type TableRow<TData = unknown> = {
  id: string;
  cells: Record<string, string | number | null>;
  summary?: string | null;
  data?: TData;
};
```

### TableRowAction

```ts
type TableRowAction = {
  value: string;
  label: string;
  disabled?: boolean;
  kind?: "action" | "separator";
  href?: string | null;
  shortcutLabel?: string;
  tone?: "default" | "danger";
  hidden?: boolean;
};
```

### TableSortDirection

```ts
type TableSortDirection = "asc" | "desc";
```

### TableFilters

```ts
type TableFilters = Record<string, string>;
```

### TablePagination

```ts
type TablePagination = {
  page: number;
  limit: number;
  total: number;
};
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
        │     │     └── [Sort Button]  (if sortable)
        │     └── [Actions Header]  (optional)
        ├── [Filter Row]  (optional, when visible filterable columns exist)
        │     └── [Filter Cell...]
        ├── [Body]
              └── [Row...]  aria-selected
                    ├── [Selection Cell]  Checkbox
                    ├── [Data Cell...]
                    │     └── [Cell Content]
                    │           ├── <span> value
                    │           └── <small> summary  (optional, first column only)
                    └── [Row Actions Cell]  (optional)
                          └── <button> rowActionLabel
  └── [Footer]  (optional, when pagination is present)
        ├── [Pagination Summary]
        ├── [Limit Selector]
        └── [Pagination Controls]
```

## 4. Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `columns` | `TableColumn[]` | none | yes | column definitions |
| `rows` | `TableRow[]` | none | yes | current visible rows |
| `filters` | `TableFilters` | `{}` | no | controlled filter values keyed by column id |
| `pagination` | `TablePagination \| null` | `null` | no | controlled pagination footer state |
| `loading` | `boolean` | `false` | no | shows loading rows when true and no rows are currently present |
| `loadingRows` | `number` | `5` | no | number of skeleton rows in loading posture |
| `selectable` | `boolean` | `false` | no | shows selection column and selection caption copy |
| `selectedRowIds` | `string[]` | `[]` | no | controlled selection |
| `sortColumnId` | `string \| null` | `null` | no | current sort column |
| `sortDirection` | `TableSortDirection` | `"asc"` | no | current sort direction |
| `rowActionLabel` | `string` | `"Open"` | no | label for row action buttons |
| `showRowActions` | `boolean` | `true` | no | row action column visibility |
| `rowActions` | `TableRowAction[] \| ((row: TableRow) => TableRowAction[])` | `[]` | no | richer per-row action model |
| `expandedRowWhen` | `(row: TableRow) => boolean` | `() => false` | no | shows the `expandedRow` slot for matching rows |
| `emptyMessage` | `string` | `"No rows match the current view."` | no | empty posture copy |
| `ariaLabel` | `string` | `"Data table"` | no | accessible table name |
| `hiddenColumnIds` | `string[]` | `[]` | no | ids of columns currently hidden |
| `showColumnVisibility` | `boolean` | `false` | no | show column visibility toggle in toolbar |
| `showExport` | `boolean` | `false` | no | show CSV export button in toolbar |
| `exportFilename` | `string` | `"export.csv"` | no | filename for CSV download |
| `limitOptions` | `number[]` | `[10, 20, 50, 100]` | no | page-size options for footer selector |
| `showLimitSelector` | `boolean` | `true` | no | page-size selector visibility |
| `compact` | `boolean` | `false` | no | tighter table spacing |
| `striped` | `boolean` | `false` | no | alternating row backgrounds |
| `stickyHeader` | `boolean` | `false` | no | sticky header treatment while scrolling |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

## 5. Events

| Event | When It Fires | Payload |
|-------|---------------|---------|
| `sortChange` | sortable header clicked | `{ columnId: string; direction: TableSortDirection }` |
| `rowToggle` | individual row selection changes | `{ rowId: string; selected: boolean }` |
| `toggleAll` | select-all checkbox changes | `{ selected: boolean }` |
| `rowAction` | row action button clicked | `{ rowId: string }` |
| `rowActionSelect` | rich row action selected | `{ rowId: string; row: TableRow; action: TableRowAction }` |
| `columnVisibilityChange` | column visibility toggled | `{ columnId: string; visible: boolean }` |
| `exportCsv` | CSV export completed | `{ filename: string }` |
| `rowClick` | non-interactive row surface clicked | `{ rowId: string; row: TableRow }` |
| `filterChange` | column filter value changes | `{ filters: TableFilters }` |
| `pageChange` | pagination page changes | `{ page: number }` |
| `limitChange` | page-size changes | `{ limit: number }` |

## 6. Slots

| Slot | Scope | Purpose |
|------|-------|---------|
| `cell` | `{ column, row, value }` | host-owned custom cell rendering |
| `expandedRow` | `{ row }` | host-owned detail content beneath matching rows |
| `empty` | none | host-owned empty state |

## 7. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | rows present | table body visible |
| empty | no rows | empty row with `emptyMessage` spanning all columns |
| loading | `loading` and no rows | loading skeleton rows visible |
| partially selected | some visible rows selected | select-all checkbox in mixed state |
| fully selected | all visible rows selected | select-all checkbox checked |
| sorted | active sort column present | sort icon (arrow-up/arrow-down) visible on active header |
| filtered | visible filterable columns exist | filter row rendered under headers |
| paginated | `pagination` present | footer summary and controls visible |
| toolbar visible | `showColumnVisibility` or `showExport` is true | toolbar row above table |

## 8. Accessibility

### Semantics

- Native `<table>` markup with `aria-label`
- Visually hidden `<caption>` providing table name and selection count
- Sort headers: `aria-sort` attribute (`ascending`, `descending`, or `none`)
- Filter controls: labeled per-column via accessible `aria-label`
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

## 9. Toolbar

When `showExport` or `showColumnVisibility` is true, a toolbar renders above
the table:

- **Export button**: icon (download) + "Export" label; triggers client-side CSV
  generation and download, then dispatches `exportCsv`
- **Column visibility**: Popover (placement: bottom-end) with Checkbox list of
  hideable columns (`hideable !== false`); toggling dispatches
  `columnVisibilityChange`
- Only columns where `hideable` is not explicitly `false` appear in the
  visibility menu
- Hidden columns are excluded from both rendering and CSV export

## 10. Composition

- Composes: `Checkbox`, `Icon`, `Popover`
- Parent expectations: `FilterToolbar`, `BulkActionBar`
- Host ownership: filtering, sorting implementation, pagination, and
  persistence all stay host-owned even though the table now renders the generic
  filter row and footer controls
- Slot ownership: custom cells, expanded rows, and empty-state language stay
  host-owned

## 10. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Table shell | `background-panel`, `border-subtle`, `radius-surface` | table frame |
| Toolbar | `background-elevated` | toolbar distinction |
| Header row | `text-secondary`, `background-elevated` | header distinction |
| Selected row | `accent-base` at 8% | selection emphasis |
| Hover row | `accent-base` at 5% | hover feedback |
| Sort/action buttons | `border-width-focus`, `accent-focusRing` | focus ring |

### Token Usage — Exact CSS Values

#### `.data-table` (Root)

| Property | Value |
|----------|-------|
| `overflow` | `auto` |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `var(--poodle-color-background-panel)` |

#### `.data-table__toolbar`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `flex-end` |
| `gap` | `0.5rem` |
| `padding` | `0.5rem 0.75rem` |
| `border-bottom` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent)` |

#### `.data-table__toolbar-btn`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `padding` | `0.25rem 0.625rem` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `line-height` | `1` |
| `transition` | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### `.data-table__toolbar-btn:hover`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent)` |

#### `.data-table__toolbar-btn:focus-visible`

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

#### `.data-table__toolbar-btn :global(.poodle-icon)`

| Property | Value |
|----------|-------|
| `width` | `0.875rem` |
| `height` | `0.875rem` |

#### `.data-table__col-menu`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |

#### `.data-table__col-menu-item`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.5rem` |
| `padding` | `0.375rem 0.5rem` |
| `border-radius` | `calc(var(--poodle-radius-control) - 0.125rem)` |
| `cursor` | `pointer` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `color` | `var(--poodle-color-text-primary)` |

#### `.data-table__col-menu-item:hover`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent)` |

#### `table`

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `border-collapse` | `collapse` |

#### `.data-table__caption` (Visually Hidden)

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `width` | `0.0625rem` |
| `height` | `0.0625rem` |
| `padding` | `0` |
| `margin` | `-0.0625rem` |
| `overflow` | `hidden` |
| `clip` | `rect(0, 0, 0, 0)` |
| `white-space` | `nowrap` |
| `border` | `0` |

#### `th`, `td`

| Property | Value |
|----------|-------|
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border-bottom` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `text-align` | `left` |
| `vertical-align` | `middle` |
| `font-size` | `var(--poodle-typography-label-size)` |

#### `thead th`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `line-height` | `var(--poodle-typography-label-lineHeight)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent)` |

#### `tbody tr.selected`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 8%, transparent)` |

#### `tbody tr:hover`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 5%, transparent)` |

#### `.data-table__selection`

| Property | Value |
|----------|-------|
| `width` | `3.25rem` |

#### `.data-table__sort`, `.data-table__actions button`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `min-height` | `var(--poodle-size-control-height)` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `inherit` |
| `cursor` | `pointer` |
| `font` | `inherit` |

#### `.data-table__sort:focus-visible`, `.data-table__actions button:focus-visible`

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |
| `border-radius` | `var(--poodle-radius-control)` |

#### `.data-table__cell`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.25rem` |

#### `.data-table__cell small`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |

#### `.end-align`

| Property | Value |
|----------|-------|
| `text-align` | `right` |

#### `.data-table__actions-header`, `.data-table__actions`

| Property | Value |
|----------|-------|
| `width` | `7.5rem` |
| `text-align` | `right` |

#### `.data-table__empty`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |

### CSS Classes Used for State Selectors

| Class | Element | Purpose |
|-------|---------|---------|
| `.selected` | `tbody tr` | selected row highlight |
| `.end-align` | `th`, `td` | right-aligned columns |

## 11. Specimen Definitions

### With Sorting, Column Visibility, And Export

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With sorting, column visibility, and export | columns: Name (sortable), Email (sortable), Role (sortable), Status (not sortable); 5 rows; `showColumnVisibility`, `showExport`; selection enabled | table with sortable column headers, select-all checkbox, row selection checkboxes, column visibility toggle, export action, and row actions; status bar shows selection count |

### Empty State

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Empty state | same columns, `rows=[]`, `emptyMessage="No team members match the current filters."` | table header visible with empty-state message in body |
