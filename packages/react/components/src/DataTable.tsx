import type { CSSProperties, MouseEvent, ReactNode } from "react";

import "@inflatable-cookie/poodle-styles/data-table.css";

import { Button } from "./Button";
import { Checkbox } from "./Checkbox";
import { Icon } from "./Icon";
import { IconButton } from "./IconButton";
import { Menu } from "./Menu";
import { Popover } from "./Popover";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import { Select } from "./Select";
import { TextInput } from "./TextInput";
import type {
  ControlDensity,
  ControlSize,
  MenuItem,
  SemanticControlSizeRole,
  TableCellValue,
  TableColumn,
  TableFilters,
  TablePagination,
  TableRow,
  TableRowAction,
  TableSortDirection,
} from "./types";

export interface DataTableProps {
  ariaLabel?: string;
  columns?: TableColumn[];
  rows?: TableRow[];
  filters?: TableFilters;
  pagination?: TablePagination | null;
  loading?: boolean;
  loadingRows?: number;
  selectable?: boolean;
  selectedRowIds?: string[];
  sortColumnId?: string | null;
  sortDirection?: TableSortDirection;
  rowActionLabel?: string;
  showRowActions?: boolean;
  rowActions?: TableRowAction[] | ((row: TableRow) => TableRowAction[]);
  expandedRowIds?: string[];
  emptyMessage?: string;
  hiddenColumnIds?: string[];
  showColumnVisibility?: boolean;
  showExport?: boolean;
  exportFilename?: string;
  limitOptions?: number[];
  showLimitSelector?: boolean;
  compact?: boolean;
  striped?: boolean;
  stickyHeader?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onSortChange?: ((payload: { columnId: string; direction: TableSortDirection }) => void) | null;
  onRowToggle?: ((payload: { rowId: string; selected: boolean }) => void) | null;
  onToggleAll?: ((payload: { selected: boolean }) => void) | null;
  onRowAction?: ((payload: { rowId: string }) => void) | null;
  onRowActionSelect?: ((payload: { rowId: string; row: TableRow; action: TableRowAction }) => void) | null;
  onColumnVisibilityChange?: ((payload: { columnId: string; visible: boolean }) => void) | null;
  onExportCsv?: ((payload: { filename: string }) => void) | null;
  onRowClick?: ((payload: { rowId: string; row: TableRow }) => void) | null;
  onFilterChange?: ((payload: { filters: TableFilters }) => void) | null;
  onPageChange?: ((payload: { page: number }) => void) | null;
  onLimitChange?: ((payload: { limit: number }) => void) | null;
  cell?: (column: TableColumn, row: TableRow, value: TableCellValue) => ReactNode;
  expandedRow?: (row: TableRow) => ReactNode;
  empty?: () => ReactNode;
}

function stringifyCellValue(value: TableCellValue): string {
  if (value === null || value === undefined) return "";
  return String(value);
}

function getColumnStyle(column: TableColumn): CSSProperties | undefined {
  const style: CSSProperties = {
    ...(column.width ? { width: column.width } : null),
    ...(column.minWidth ? { minWidth: column.minWidth } : null),
  };
  return Object.keys(style).length > 0 ? style : undefined;
}

function columnClass(column: TableColumn, extra?: string): string | undefined {
  const classes = [
    extra ?? "",
    column.align === "center" ? "poodle-center-align" : "",
    column.align === "end" ? "poodle-end-align" : "",
    column.hideOnMobile === true ? "poodle-data-table__hide-mobile" : "",
  ].filter(Boolean);
  return classes.length > 0 ? classes.join(" ") : undefined;
}

export function DataTable({
  ariaLabel = "Data table",
  columns = [],
  rows = [],
  filters = {},
  pagination = null,
  loading = false,
  loadingRows = 5,
  selectable = false,
  selectedRowIds = [],
  sortColumnId = null,
  sortDirection = "asc",
  rowActionLabel = "Open",
  showRowActions = true,
  rowActions = [],
  expandedRowIds = [],
  emptyMessage = "No rows match the current view.",
  hiddenColumnIds = [],
  showColumnVisibility = false,
  showExport = false,
  exportFilename = "export.csv",
  limitOptions = [10, 20, 50, 100],
  showLimitSelector = true,
  compact = false,
  striped = false,
  stickyHeader = false,
  size = null,
  sizeRole = "control",
  density = null,
  onSortChange = null,
  onRowToggle = null,
  onToggleAll = null,
  onRowAction = null,
  onRowActionSelect = null,
  onColumnVisibilityChange = null,
  onExportCsv = null,
  onRowClick = null,
  onFilterChange = null,
  onPageChange = null,
  onLimitChange = null,
  cell,
  expandedRow,
  empty,
}: DataTableProps) {
  const uiPresentation = useUiPresentation();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const visibleColumns = columns.filter((column) => !hiddenColumnIds.includes(column.id));
  const hideableColumns = columns.filter((column) => column.hideable !== false);
  const hasFilters = visibleColumns.some((column) => column.filterable);
  const selectableRowCount = selectable ? rows.length : 0;
  const selectionCount = selectable ? rows.filter((row) => selectedRowIds.includes(row.id)).length : 0;
  const allRowsSelected = selectableRowCount > 0 && selectionCount === selectableRowCount;
  const mixedSelection = selectionCount > 0 && !allRowsSelected;
  const expandedIdSet = new Set(expandedRowIds);
  const hasRichRowActions = typeof rowActions === "function" || rowActions.length > 0;
  const showLegacyRowAction = showRowActions && !hasRichRowActions;
  const showActionsColumn = showRowActions && (showLegacyRowAction || hasRichRowActions);
  const totalPages = pagination ? Math.max(1, Math.ceil(pagination.total / pagination.limit)) : 1;
  const showPaginationFooter = pagination !== null && (totalPages > 1 || showLimitSelector);
  const columnCount = visibleColumns.length + (selectable ? 1 : 0) + (showActionsColumn ? 1 : 0);

  const getRowPrimaryLabel = (row: TableRow) =>
    stringifyCellValue(row.cells[visibleColumns[0]?.id ?? "id"]) || row.id;

  function resolveRowActions(row: TableRow): TableRowAction[] {
    const resolved = typeof rowActions === "function" ? rowActions(row) : rowActions;
    return resolved.filter((action) => action.kind === "separator" || action.hidden !== true);
  }

  const toMenuItem = (action: TableRowAction): MenuItem => ({
    value: action.value,
    label: action.label,
    disabled: action.disabled,
    shortcutLabel: action.shortcutLabel,
    tone: action.tone,
    kind: action.kind === "separator" ? "separator" : "action",
  });

  const getActionButtonClass = (action: TableRowAction) =>
    action.tone === "danger"
      ? "poodle-data-table__row-action-btn poodle-data-table__row-action-btn--danger"
      : "poodle-data-table__row-action-btn";

  function requestRowAction(row: TableRow, action: TableRowAction): void {
    if (action.disabled || action.kind === "separator") return;
    onRowActionSelect?.({ rowId: row.id, row, action });
  }

  function handleRowClick(event: MouseEvent, row: TableRow): void {
    const target = event.target as HTMLElement | null;
    if (target?.closest("button, a, input, label, [role='button'], [role='menuitem'], [data-row-action-trigger='true']")) {
      return;
    }
    onRowClick?.({ rowId: row.id, row });
  }

  function requestSort(column: TableColumn): void {
    if (!column.sortable) return;
    const direction: TableSortDirection = sortColumnId === column.id && sortDirection === "asc" ? "desc" : "asc";
    onSortChange?.({ columnId: column.id, direction });
  }

  function requestPageChange(page: number): void {
    if (!pagination) return;
    const nextPage = Math.min(Math.max(page, 1), totalPages);
    if (nextPage === pagination.page) return;
    onPageChange?.({ page: nextPage });
  }

  function handleExport(): void {
    const header = visibleColumns.map((column) => column.label).join(",");
    const body = rows
      .map((row) =>
        visibleColumns
          .map((column) => {
            const value = stringifyCellValue(row.cells[column.id] ?? null);
            return value.includes(",") || value.includes('"') ? `"${value.replace(/"/g, '""')}"` : value;
          })
          .join(","),
      )
      .join("\n");
    const csv = `${header}\n${body}`;
    const blob = new Blob([csv], { type: "text/csv;charset=utf-8;" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = exportFilename;
    link.click();
    URL.revokeObjectURL(url);
    onExportCsv?.({ filename: exportFilename });
  }

  function getPaginationSummary(): string {
    if (!pagination || pagination.total === 0) return "Showing 0 results";
    const start = (pagination.page - 1) * pagination.limit + 1;
    const end = Math.min(pagination.page * pagination.limit, pagination.total);
    return `Showing ${start} to ${end} of ${pagination.total}`;
  }

  return (
    <div
      className={[
        "poodle-data-table",
        compact ? "poodle-data-table--compact" : "",
        striped ? "poodle-data-table--striped" : "",
        stickyHeader ? "poodle-data-table--sticky-header" : "",
      ]
        .filter(Boolean)
        .join(" ")}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      {showColumnVisibility || showExport ? (
        <div className="poodle-data-table__toolbar">
          {showExport ? (
            <button type="button" className="poodle-data-table__toolbar-btn" onClick={handleExport} aria-label="Export as CSV">
              <Icon name="download" />
              Export
            </button>
          ) : null}

          {showColumnVisibility && hideableColumns.length > 0 ? (
            <Popover
              placement="bottom-end"
              ariaLabel="Column visibility"
              trigger={
                <span className="poodle-data-table__toolbar-btn">
                  <Icon name="columns-3" />
                  Columns
                </span>
              }
            >
              <div className="poodle-data-table__col-menu" role="menu">
                {hideableColumns.map((column) => (
                  <label key={column.id} className="poodle-data-table__col-menu-item">
                    <Checkbox
                      ariaLabel={column.label}
                      checked={!hiddenColumnIds.includes(column.id)}
                      onCheckedChange={() =>
                        onColumnVisibilityChange?.({ columnId: column.id, visible: hiddenColumnIds.includes(column.id) })
                      }
                    />
                    <span>{column.label}</span>
                  </label>
                ))}
              </div>
            </Popover>
          ) : null}
        </div>
      ) : null}

      <table aria-label={ariaLabel}>
        <caption className="poodle-data-table__caption">
          {selectable
            ? `${ariaLabel}. ${selectionCount} selected row${selectionCount === 1 ? "" : "s"} out of ${selectableRowCount}.`
            : `${ariaLabel}.`}
        </caption>
        <thead>
          <tr>
            {selectable ? (
              <th className="poodle-data-table__selection">
                <Checkbox
                  ariaLabel="Select all visible rows"
                  checked={allRowsSelected}
                  mixed={mixedSelection}
                  onCheckedChange={(checked) => onToggleAll?.({ selected: checked })}
                />
              </th>
            ) : null}
            {visibleColumns.map((column) => (
              <th
                key={column.id}
                style={getColumnStyle(column)}
                className={columnClass(column)}
                aria-sort={
                  column.sortable && sortColumnId === column.id
                    ? sortDirection === "asc"
                      ? "ascending"
                      : "descending"
                    : column.sortable
                      ? "none"
                      : undefined
                }
              >
                {column.sortable ? (
                  <button
                    type="button"
                    className="poodle-data-table__sort"
                    onClick={() => requestSort(column)}
                    aria-label={`Sort by ${column.label}${sortColumnId === column.id ? `, currently ${sortDirection}` : ""}`}
                  >
                    <span>{column.label}</span>
                    {sortColumnId === column.id ? (
                      <span aria-hidden="true">
                        <Icon name={sortDirection === "asc" ? "arrow-up" : "arrow-down"} />
                      </span>
                    ) : null}
                  </button>
                ) : (
                  <span>{column.label}</span>
                )}
              </th>
            ))}
            {showActionsColumn ? (
              <th scope="col" className="poodle-data-table__actions-header">
                Actions
              </th>
            ) : null}
          </tr>
          {hasFilters ? (
            <tr className="poodle-data-table__filters-row">
              {selectable ? <td className="poodle-data-table__selection" aria-hidden="true" /> : null}
              {visibleColumns.map((column) => (
                <td key={column.id} style={getColumnStyle(column)} className={columnClass(column)}>
                  {column.filterable ? (
                    column.filterType === "select" && column.filterOptions ? (
                      <Select
                        id={`data-table-filter-${column.id}`}
                        value={filters[column.id] ?? ""}
                        options={[
                          { value: "", label: "All" },
                          ...column.filterOptions.map((option) =>
                            typeof option === "string" ? { value: option, label: option } : option,
                          ),
                        ]}
                        onValueChange={(nextValue) =>
                          onFilterChange?.({ filters: { ...filters, [column.id]: nextValue } })
                        }
                        ariaLabel={`Filter ${column.label}`}
                        clearable
                        valueLabel="All"
                      />
                    ) : column.filterType === "date" ? (
                      <TextInput
                        id={`data-table-filter-${column.id}`}
                        type="date"
                        value={filters[column.id] ?? ""}
                        ariaLabel={`Filter ${column.label}`}
                        onValueChange={(nextValue) =>
                          onFilterChange?.({ filters: { ...filters, [column.id]: nextValue } })
                        }
                      />
                    ) : (
                      <TextInput
                        id={`data-table-filter-${column.id}`}
                        type="search"
                        value={filters[column.id] ?? ""}
                        placeholder={`Filter ${column.label.toLowerCase()}...`}
                        ariaLabel={`Filter ${column.label}`}
                        debounce={300}
                        onValueChange={(nextValue) =>
                          onFilterChange?.({ filters: { ...filters, [column.id]: nextValue } })
                        }
                      />
                    )
                  ) : null}
                </td>
              ))}
              {showActionsColumn ? <td aria-hidden="true" /> : null}
            </tr>
          ) : null}
        </thead>
        <tbody>
          {loading && rows.length === 0 ? (
            Array.from({ length: loadingRows }, (_, index) => (
              <tr key={index} className="poodle-data-table__loading-row">
                {selectable ? (
                  <td className="poodle-data-table__selection">
                    <span className="poodle-data-table__loading-block poodle-data-table__loading-block--checkbox" />
                  </td>
                ) : null}
                {visibleColumns.map((column) => (
                  <td key={column.id} style={getColumnStyle(column)} className={columnClass(column)}>
                    <span className="poodle-data-table__loading-block" />
                  </td>
                ))}
                {showActionsColumn ? (
                  <td className="poodle-data-table__actions">
                    <span className="poodle-data-table__loading-block poodle-data-table__loading-block--action" />
                  </td>
                ) : null}
              </tr>
            ))
          ) : rows.length === 0 ? (
            <tr>
              <td colSpan={columnCount} className="poodle-data-table__empty">
                {empty ? empty() : emptyMessage}
              </td>
            </tr>
          ) : (
            rows.map((row) => {
              const rowSelected = selectable && selectedRowIds.includes(row.id);
              const actions = hasRichRowActions ? resolveRowActions(row) : [];
              const actionableActions = actions.filter((action) => action.kind !== "separator");
              return [
                <tr
                  key={row.id}
                  className={rowSelected ? "poodle-selected" : undefined}
                  aria-selected={selectable ? rowSelected : undefined}
                  onClick={(event) => handleRowClick(event, row)}
                >
                  {selectable ? (
                    <td className="poodle-data-table__selection">
                      <Checkbox
                        ariaLabel={`Select row ${getRowPrimaryLabel(row)}`}
                        checked={selectedRowIds.includes(row.id)}
                        onCheckedChange={(checked) => onRowToggle?.({ rowId: row.id, selected: checked })}
                      />
                    </td>
                  ) : null}
                  {visibleColumns.map((column, index) => {
                    const isRowHeader = index === 0 && column.isRowHeader !== false;
                    const Tag = isRowHeader ? "th" : "td";
                    return (
                      <Tag
                        key={column.id}
                        scope={isRowHeader ? "row" : undefined}
                        style={getColumnStyle(column)}
                        className={columnClass(column)}
                      >
                        {cell ? (
                          cell(column, row, row.cells[column.id] ?? null)
                        ) : (
                          <div className="poodle-data-table__cell">
                            <span>{stringifyCellValue(row.cells[column.id] ?? null) || "—"}</span>
                            {column.id === visibleColumns[0]?.id && row.summary ? <small>{row.summary}</small> : null}
                          </div>
                        )}
                      </Tag>
                    );
                  })}
                  {showActionsColumn ? (
                    <td className="poodle-data-table__actions">
                      {hasRichRowActions ? (
                        actionableActions.length === 1 && actions.length === 1 ? (
                          actionableActions[0].href ? (
                            <a
                              href={actionableActions[0].href ?? undefined}
                              className={getActionButtonClass(actionableActions[0])}
                              aria-label={`${actionableActions[0].label} ${getRowPrimaryLabel(row)}`}
                            >
                              {actionableActions[0].label}
                            </a>
                          ) : (
                            <button
                              type="button"
                              className={getActionButtonClass(actionableActions[0])}
                              aria-label={`${actionableActions[0].label} ${getRowPrimaryLabel(row)}`}
                              onClick={(event) => {
                                event.stopPropagation();
                                requestRowAction(row, actionableActions[0]);
                              }}
                            >
                              {actionableActions[0].label}
                            </button>
                          )
                        ) : actions.length > 0 ? (
                          <Menu
                            items={actions.map(toMenuItem)}
                            ariaLabel={`Actions for ${getRowPrimaryLabel(row)}`}
                            placement="bottom-end"
                            onAction={(value) => {
                              const action = actions.find((candidate) => candidate.value === value);
                              if (action) requestRowAction(row, action);
                            }}
                            trigger={
                              <span className="poodle-data-table__actions-trigger" data-row-action-trigger="true">
                                <IconButton
                                  icon="ellipsis"
                                  variant="ghost"
                                  sizeRole="chrome"
                                  ariaLabel={`Actions for ${getRowPrimaryLabel(row)}`}
                                  tooltip="Actions"
                                />
                              </span>
                            }
                          />
                        ) : null
                      ) : showLegacyRowAction ? (
                        <button
                          type="button"
                          className="poodle-data-table__row-action-btn"
                          aria-label={`${rowActionLabel} ${getRowPrimaryLabel(row)}`}
                          onClick={(event) => {
                            event.stopPropagation();
                            onRowAction?.({ rowId: row.id });
                          }}
                        >
                          {rowActionLabel}
                        </button>
                      ) : null}
                    </td>
                  ) : null}
                </tr>,
                expandedRow && expandedIdSet.has(row.id) ? (
                  <tr key={`${row.id}-expanded`} className="poodle-data-table__expanded-row">
                    <td colSpan={columnCount}>
                      <div className="poodle-data-table__expanded-panel">{expandedRow(row)}</div>
                    </td>
                  </tr>
                ) : null,
              ];
            })
          )}
        </tbody>
      </table>

      {showPaginationFooter && pagination ? (
        <div className="poodle-data-table__footer">
          <p className="poodle-data-table__pagination-summary">{getPaginationSummary()}</p>
          <div className="poodle-data-table__pagination-actions">
            {showLimitSelector && limitOptions.length > 0 ? (
              <label className="poodle-data-table__limit">
                <span>Show</span>
                <Select
                  id={`${ariaLabel.replace(/\s+/g, "-").toLowerCase()}-limit`}
                  value={String(pagination.limit)}
                  options={limitOptions.map((option) => ({ value: String(option), label: String(option) }))}
                  onValueChange={(nextValue) => onLimitChange?.({ limit: Number(nextValue) })}
                  ariaLabel="Items per page"
                />
                <span>per page</span>
              </label>
            ) : null}
            {totalPages > 1 ? (
              <div className="poodle-data-table__pagination-controls">
                <Button type="button" variant="ghost" size="sm" disabled={pagination.page <= 1} onClick={() => requestPageChange(1)}>
                  First
                </Button>
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  disabled={pagination.page <= 1}
                  onClick={() => requestPageChange(pagination.page - 1)}
                >
                  Previous
                </Button>
                <span className="poodle-data-table__pagination-page">
                  Page {pagination.page} of {totalPages}
                </span>
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  disabled={pagination.page >= totalPages}
                  onClick={() => requestPageChange(pagination.page + 1)}
                >
                  Next
                </Button>
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  disabled={pagination.page >= totalPages}
                  onClick={() => requestPageChange(totalPages)}
                >
                  Last
                </Button>
              </div>
            ) : null}
          </div>
        </div>
      ) : null}
    </div>
  );
}
