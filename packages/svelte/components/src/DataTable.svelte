<script lang="ts">
  import type { Snippet } from "svelte";

  import Button from "./Button.svelte";
  import Checkbox from "./Checkbox.svelte";
  import Icon from "./Icon.svelte";
  import IconButton from "./IconButton.svelte";
  import Menu from "./Menu.svelte";
  import Popover from "./Popover.svelte";
  import Select from "./Select.svelte";
  import TextInput from "./TextInput.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, MenuItem, SemanticControlSizeRole } from "./types";

  import type {
    TableCellValue,
    TableColumn,
    TableFilters,
    TablePagination,
    TableRow,
    TableRowAction,
    TableSortDirection,
  } from "./types";

  interface Props {
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
    cell?: Snippet<[TableColumn, TableRow, TableCellValue]>;
    expandedRow?: Snippet<[TableRow]>;
    empty?: Snippet<[]>;
  }

  let {
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
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const visibleColumns = $derived(columns.filter((column) => !hiddenColumnIds.includes(column.id)));
  const hideableColumns = $derived(columns.filter((column) => column.hideable !== false));
  const hasFilters = $derived(visibleColumns.some((column) => column.filterable));
  const selectableRowCount = $derived(selectable ? rows.length : 0);
  const selectionCount = $derived(selectable ? rows.filter((row) => selectedRowIds.includes(row.id)).length : 0);
  const allRowsSelected = $derived(selectableRowCount > 0 && selectionCount === selectableRowCount);
  const mixedSelection = $derived(selectionCount > 0 && !allRowsSelected);
  const hasCustomCellSnippet = $derived(Boolean(cell));
  const hasExpandedRowSnippet = $derived(Boolean(expandedRow));
  const expandedIdSet = $derived(new Set(expandedRowIds));
  const hasEmptySnippet = $derived(Boolean(empty));
  const hasRichRowActions = $derived(typeof rowActions === "function" || rowActions.length > 0);
  const showLegacyRowAction = $derived(showRowActions && !hasRichRowActions);
  const showActionsColumn = $derived(showRowActions && (showLegacyRowAction || hasRichRowActions));
  const totalPages = $derived(pagination ? Math.max(1, Math.ceil(pagination.total / pagination.limit)) : 1);
  const showPaginationFooter = $derived(pagination !== null && (totalPages > 1 || showLimitSelector));
  const columnCount = $derived(visibleColumns.length + (selectable ? 1 : 0) + (showActionsColumn ? 1 : 0));

  function stringifyCellValue(value: TableCellValue): string {
    if (value === null || value === undefined) {
      return "";
    }

    return String(value);
  }

  function getColumnStyle(column: TableColumn): string | undefined {
    const styles = [
      column.width ? `width: ${column.width}` : null,
      column.minWidth ? `min-width: ${column.minWidth}` : null,
    ].filter(Boolean);

    return styles.length > 0 ? styles.join("; ") : undefined;
  }

  function getFilterInputId(column: TableColumn): string {
    return `data-table-filter-${column.id}`;
  }

  function getRowPrimaryLabel(row: TableRow): string {
    return stringifyCellValue(row.cells[visibleColumns[0]?.id ?? "id"]) || row.id;
  }

  function resolveRowActions(row: TableRow): TableRowAction[] {
    const resolved = typeof rowActions === "function" ? rowActions(row) : rowActions;
    return resolved.filter((action) => action.kind === "separator" || action.hidden !== true);
  }

  function toMenuItem(action: TableRowAction): MenuItem {
    return {
      value: action.value,
      label: action.label,
      disabled: action.disabled,
      shortcutLabel: action.shortcutLabel,
      tone: action.tone,
      kind: action.kind === "separator" ? "separator" : "action",
    };
  }

  function getActionButtonClass(action: TableRowAction): string {
    return action.tone === "danger"
      ? "poodle-data-table__row-action-btn poodle-data-table__row-action-btn--danger"
      : "poodle-data-table__row-action-btn";
  }

  function requestRowAction(row: TableRow, action: TableRowAction): void {
    if (action.disabled || action.kind === "separator") {
      return;
    }

    onRowActionSelect?.({ rowId: row.id, row, action });
  }

  function handleMenuAction(row: TableRow, actions: TableRowAction[], value: string): void {
    const action = actions.find((candidate) => candidate.value === value);
    if (!action) {
      return;
    }

    requestRowAction(row, action);
  }

  function handleRowClick(event: MouseEvent, row: TableRow): void {
    const target = event.target as HTMLElement | null;
    if (target?.closest("button, a, input, label, [role='button'], [role='menuitem'], [data-row-action-trigger='true']")) {
      return;
    }

    onRowClick?.({ rowId: row.id, row });
  }

  function requestFilterChange(columnId: string, value: string): void {
    onFilterChange?.({
      filters: {
        ...filters,
        [columnId]: value,
      },
    });
  }

  function requestSort(column: TableColumn): void {
    if (!column.sortable) {
      return;
    }

    const direction =
      sortColumnId === column.id && sortDirection === "asc"
        ? "desc"
        : "asc";

    onSortChange?.({ columnId: column.id, direction });
  }

  function toggleColumnVisibility(columnId: string): void {
    const isHidden = hiddenColumnIds.includes(columnId);
    onColumnVisibilityChange?.({ columnId, visible: isHidden });
  }

  function requestPageChange(page: number): void {
    if (!pagination) {
      return;
    }

    const nextPage = Math.min(Math.max(page, 1), totalPages);
    if (nextPage === pagination.page) {
      return;
    }

    onPageChange?.({ page: nextPage });
  }

  function requestLimitChange(limit: number): void {
    onLimitChange?.({ limit });
  }

  function handleExport(): void {
    const header = visibleColumns.map((column) => column.label).join(",");
    const body = rows
      .map((row) =>
        visibleColumns
          .map((column) => {
            const value = stringifyCellValue(row.cells[column.id] ?? null);
            return value.includes(",") || value.includes('"')
              ? `"${value.replace(/"/g, '""')}"`
              : value;
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
    if (!pagination || pagination.total === 0) {
      return "Showing 0 results";
    }

    const start = (pagination.page - 1) * pagination.limit + 1;
    const end = Math.min(pagination.page * pagination.limit, pagination.total);
    return `Showing ${start} to ${end} of ${pagination.total}`;
  }
</script>

<div
  class="poodle-data-table"
  class:poodle-data-table--compact={compact}
  class:poodle-data-table--striped={striped}
  class:poodle-data-table--sticky-header={stickyHeader}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  {#if showColumnVisibility || showExport}
    <div class="poodle-data-table__toolbar">
      {#if showExport}
        <button
          type="button"
          class="poodle-data-table__toolbar-btn"
          onclick={handleExport}
          aria-label="Export as CSV"
        >
          <Icon name="download" />
          Export
        </button>
      {/if}

      {#if showColumnVisibility && hideableColumns.length > 0}
        <Popover placement="bottom-end" ariaLabel="Column visibility">
          {#snippet trigger()}
            <span class="poodle-data-table__toolbar-btn">
              <Icon name="columns-3" />
              Columns
            </span>
          {/snippet}
          <div class="poodle-data-table__col-menu" role="menu">
            {#each hideableColumns as column}
              <label class="poodle-data-table__col-menu-item">
                <Checkbox
                  ariaLabel={column.label}
                  checked={!hiddenColumnIds.includes(column.id)}
                  onCheckedChange={() => toggleColumnVisibility(column.id)}
                />
                <span>{column.label}</span>
              </label>
            {/each}
          </div>
        </Popover>
      {/if}
    </div>
  {/if}

  <table aria-label={ariaLabel}>
    <caption class="poodle-data-table__caption">
      {#if selectable}
        {ariaLabel}. {selectionCount} selected row{selectionCount === 1 ? "" : "s"} out of {selectableRowCount}.
      {:else}
        {ariaLabel}.
      {/if}
    </caption>
    <thead>
      <tr>
        {#if selectable}
          <th class="poodle-data-table__selection">
            <Checkbox
              ariaLabel="Select all visible rows"
              checked={allRowsSelected}
              mixed={mixedSelection}
              onCheckedChange={(checked) => onToggleAll?.({ selected: checked })}
            />
          </th>
        {/if}
        {#each visibleColumns as column}
          <th
            style={getColumnStyle(column)}
            class:poodle-center-align={column.align === "center"}
            class:poodle-end-align={column.align === "end"}
            class:poodle-data-table__hide-mobile={column.hideOnMobile === true}
            aria-sort={column.sortable && sortColumnId === column.id ? (sortDirection === "asc" ? "ascending" : "descending") : column.sortable ? "none" : undefined}
          >
            {#if column.sortable}
              <button
                type="button"
                class="poodle-data-table__sort"
                onclick={() => requestSort(column)}
                aria-label={`Sort by ${column.label}${sortColumnId === column.id ? `, currently ${sortDirection}` : ""}`}
              >
                <span>{column.label}</span>
                {#if sortColumnId === column.id}
                  <span aria-hidden="true"><Icon name={sortDirection === "asc" ? "arrow-up" : "arrow-down"} /></span>
                {/if}
              </button>
            {:else}
              <span>{column.label}</span>
            {/if}
          </th>
        {/each}
        {#if showActionsColumn}
          <th scope="col" class="poodle-data-table__actions-header">Actions</th>
        {/if}
      </tr>
      {#if hasFilters}
        <tr class="poodle-data-table__filters-row">
          {#if selectable}
            <td class="poodle-data-table__selection" aria-hidden="true"></td>
          {/if}
          {#each visibleColumns as column}
            <td
              style={getColumnStyle(column)}
              class:poodle-center-align={column.align === "center"}
              class:poodle-end-align={column.align === "end"}
              class:poodle-data-table__hide-mobile={column.hideOnMobile === true}
            >
              {#if column.filterable}
                {#if column.filterType === "select" && column.filterOptions}
                  <Select
                    id={getFilterInputId(column)}
                    value={filters[column.id] ?? ""}
                    options={[
                      { value: "", label: "All" },
                      ...column.filterOptions.map((option) =>
                        typeof option === "string" ? { value: option, label: option } : option,
                      ),
                    ]}
                    onValueChange={(nextValue) => requestFilterChange(column.id, nextValue)}
                    ariaLabel={`Filter ${column.label}`}
                    clearable
                    valueLabel="All"
                  />
                {:else if column.filterType === "date"}
                  <TextInput
                    id={getFilterInputId(column)}
                    type="date"
                    value={filters[column.id] ?? ""}
                    ariaLabel={`Filter ${column.label}`}
                    onValueChange={(nextValue) => requestFilterChange(column.id, nextValue)}
                  />
                {:else}
                  <TextInput
                    id={getFilterInputId(column)}
                    type="search"
                    value={filters[column.id] ?? ""}
                    placeholder={`Filter ${column.label.toLowerCase()}...`}
                    ariaLabel={`Filter ${column.label}`}
                    debounce={300}
                    onValueChange={(nextValue) => requestFilterChange(column.id, nextValue)}
                  />
                {/if}
              {/if}
            </td>
          {/each}
          {#if showActionsColumn}
            <td aria-hidden="true"></td>
          {/if}
        </tr>
      {/if}
    </thead>
    <tbody>
      {#if loading && rows.length === 0}
        {#each Array(loadingRows) as _, index (index)}
          <tr class="poodle-data-table__loading-row">
            {#if selectable}
              <td class="poodle-data-table__selection">
                <span class="poodle-data-table__loading-block poodle-data-table__loading-block--checkbox"></span>
              </td>
            {/if}
            {#each visibleColumns as column}
              <td
                style={getColumnStyle(column)}
                class:poodle-center-align={column.align === "center"}
                class:poodle-end-align={column.align === "end"}
                class:poodle-data-table__hide-mobile={column.hideOnMobile === true}
              >
                <span class="poodle-data-table__loading-block"></span>
              </td>
            {/each}
            {#if showActionsColumn}
              <td class="poodle-data-table__actions">
                <span class="poodle-data-table__loading-block poodle-data-table__loading-block--action"></span>
              </td>
            {/if}
          </tr>
        {/each}
      {:else if rows.length === 0}
        <tr>
          <td colspan={columnCount} class="poodle-data-table__empty">
            {#if hasEmptySnippet}
              {@render empty?.()}
            {:else}
              {emptyMessage}
            {/if}
          </td>
        </tr>
      {:else}
        {#each rows as row (row.id)}
          <tr
            class:poodle-selected={selectable && selectedRowIds.includes(row.id)}
            aria-selected={selectable ? selectedRowIds.includes(row.id) : undefined}
            onclick={(event) => handleRowClick(event, row)}
          >
            {#if selectable}
              <td class="poodle-data-table__selection">
                <Checkbox
                  ariaLabel={`Select row ${getRowPrimaryLabel(row)}`}
                  checked={selectedRowIds.includes(row.id)}
                  onCheckedChange={(checked) => onRowToggle?.({ rowId: row.id, selected: checked })}
                />
              </td>
            {/if}
            {#each visibleColumns as column, index}
              <svelte:element
                this={index === 0 && column.isRowHeader !== false ? "th" : "td"}
                scope={index === 0 && column.isRowHeader !== false ? "row" : undefined}
                style={getColumnStyle(column)}
                class:poodle-center-align={column.align === "center"}
                class:poodle-end-align={column.align === "end"}
                class:poodle-data-table__hide-mobile={column.hideOnMobile === true}
              >
                {#if hasCustomCellSnippet}
                  {@render cell?.(column, row, row.cells[column.id] ?? null)}
                {:else}
                  <div class="poodle-data-table__cell">
                    <span>{stringifyCellValue(row.cells[column.id] ?? null) || "—"}</span>
                    {#if column.id === visibleColumns[0]?.id && row.summary}
                      <small>{row.summary}</small>
                    {/if}
                  </div>
                {/if}
              </svelte:element>
            {/each}
            {#if showActionsColumn}
              <td class="poodle-data-table__actions">
                {#if hasRichRowActions}
                  {@const actions = resolveRowActions(row)}
                  {@const actionableActions = actions.filter((action) => action.kind !== "separator")}
                  {#if actionableActions.length === 1 && actions.length === 1}
                    {@const action = actionableActions[0]}
                    {#if action.href}
                      <a
                        href={action.href}
                        class={getActionButtonClass(action)}
                        aria-label={`${action.label} ${getRowPrimaryLabel(row)}`}
                      >
                        {action.label}
                      </a>
                    {:else}
                      <button
                        type="button"
                        class={getActionButtonClass(action)}
                        aria-label={`${action.label} ${getRowPrimaryLabel(row)}`}
                        onclick={(event) => {
                          event.stopPropagation();
                          requestRowAction(row, action);
                        }}
                      >
                        {action.label}
                      </button>
                    {/if}
                  {:else if actions.length > 0}
                    <Menu
                      items={actions.map(toMenuItem)}
                      ariaLabel={`Actions for ${getRowPrimaryLabel(row)}`}
                      placement="bottom-end"
                      onAction={(value) => handleMenuAction(row, actions, value)}
                    >
                      {#snippet trigger()}
                        <span class="poodle-data-table__actions-trigger" data-row-action-trigger="true">
                          <IconButton
                            icon="ellipsis"
                            variant="ghost"
                            sizeRole="chrome"
                            ariaLabel={`Actions for ${getRowPrimaryLabel(row)}`}
                            tooltip="Actions"
                          />
                        </span>
                      {/snippet}
                    </Menu>
                  {/if}
                {:else if showLegacyRowAction}
                  <button
                    type="button"
                    class="poodle-data-table__row-action-btn"
                    aria-label={`${rowActionLabel} ${getRowPrimaryLabel(row)}`}
                    onclick={(event) => {
                      event.stopPropagation();
                      onRowAction?.({ rowId: row.id });
                    }}
                  >
                    {rowActionLabel}
                  </button>
                {/if}
              </td>
            {/if}
          </tr>
          {#if hasExpandedRowSnippet && expandedIdSet.has(row.id)}
            <tr class="poodle-data-table__expanded-row">
              <td colspan={columnCount}>
                <div class="poodle-data-table__expanded-panel">
                  {@render expandedRow?.(row)}
                </div>
              </td>
            </tr>
          {/if}
        {/each}
      {/if}
    </tbody>
  </table>

  {#if showPaginationFooter && pagination}
    <div class="poodle-data-table__footer">
      <p class="poodle-data-table__pagination-summary">{getPaginationSummary()}</p>
      <div class="poodle-data-table__pagination-actions">
        {#if showLimitSelector && limitOptions.length > 0}
          <label class="poodle-data-table__limit">
            <span>Show</span>
            <Select
              id={`${ariaLabel.replace(/\s+/g, "-").toLowerCase()}-limit`}
              value={String(pagination.limit)}
              options={limitOptions.map((option) => ({ value: String(option), label: String(option) }))}
              onValueChange={(nextValue) => requestLimitChange(Number(nextValue))}
              ariaLabel="Items per page"
            />
            <span>per page</span>
          </label>
        {/if}
        {#if totalPages > 1}
          <div class="poodle-data-table__pagination-controls">
            <Button type="button" variant="ghost" size="sm" disabled={pagination.page <= 1} onClick={() => requestPageChange(1)}>
              First
            </Button>
            <Button type="button" variant="ghost" size="sm" disabled={pagination.page <= 1} onClick={() => requestPageChange(pagination.page - 1)}>
              Previous
            </Button>
            <span class="poodle-data-table__pagination-page">Page {pagination.page} of {totalPages}</span>
            <Button type="button" variant="ghost" size="sm" disabled={pagination.page >= totalPages} onClick={() => requestPageChange(pagination.page + 1)}>
              Next
            </Button>
            <Button type="button" variant="ghost" size="sm" disabled={pagination.page >= totalPages} onClick={() => requestPageChange(totalPages)}>
              Last
            </Button>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .poodle-data-table {
    overflow: auto;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-color-background-panel);
  }

  .poodle-data-table__toolbar {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: var(--poodle-space-inline-md);
    padding: var(--poodle-space-control-y) var(--poodle-space-panel-x);
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent);
  }

  .poodle-data-table__toolbar-btn {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    padding: var(--poodle-space-control-y) var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    font: inherit;
    font-size: var(--poodle-typography-label-size);
    line-height: 1;
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-data-table__toolbar-btn:hover {
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent);
  }

  .poodle-data-table__toolbar-btn:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-data-table__toolbar-btn :global(.poodle-icon) {
    width: 0.875rem;
    height: 0.875rem;
  }

  .poodle-data-table__col-menu {
    display: flex;
    flex-direction: column;
  }

  .poodle-data-table__col-menu-item {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-md);
    padding: var(--poodle-space-control-y) var(--poodle-space-control-x);
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    cursor: pointer;
    font-size: var(--poodle-typography-label-size);
    color: var(--poodle-color-text-primary);
  }

  .poodle-data-table__col-menu-item:hover {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  .poodle-data-table__caption {
    position: absolute;
    width: 0.0625rem;
    height: 0.0625rem;
    padding: 0;
    margin: -0.0625rem;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }

  th,
  td {
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    text-align: left;
    vertical-align: middle;
    font-size: var(--poodle-typography-label-size);
  }

  thead th {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent);
  }

  .poodle-data-table--sticky-header thead th {
    position: sticky;
    top: 0;
    z-index: 1;
  }

  .poodle-data-table__filters-row td {
    padding: var(--poodle-space-control-y) var(--poodle-space-panel-x);
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent);
  }

  .poodle-data-table__filters-row :global(.poodle-select),
  .poodle-data-table__filters-row :global(.poodle-text-input),
  .poodle-data-table__filters-row :global(.poodle-text-input__field) {
    width: 100%;
  }

  tbody tr.poodle-selected {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 8%, transparent);
  }

  tbody tr:hover {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 5%, transparent);
  }

  .poodle-data-table__selection {
    width: 3.25rem;
  }

  .poodle-data-table__sort,
  .poodle-data-table__row-action-btn,
  .poodle-data-table__action-link,
  .poodle-data-table__actions-trigger {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    min-height: var(--poodle-size-control-height);
    padding: 0;
    border: 0;
    background: transparent;
    color: inherit;
    cursor: pointer;
    font: inherit;
    text-decoration: none;
  }

  .poodle-data-table__sort:focus-visible,
  .poodle-data-table__row-action-btn:focus-visible,
  .poodle-data-table__action-link:focus-visible,
  .poodle-data-table__actions-trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
    border-radius: var(--poodle-radius-control);
  }

  .poodle-data-table__cell {
    display: grid;
    gap: 0.25rem;
  }

  .poodle-data-table__cell small {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .poodle-center-align {
    text-align: center;
  }

  .poodle-end-align {
    text-align: right;
  }

  .poodle-data-table__actions-header,
  .poodle-data-table__actions {
    width: 3.5rem;
    text-align: right;
    white-space: nowrap;
  }

  .poodle-data-table__actions-trigger {
    justify-content: flex-end;
  }

  .poodle-data-table__actions-trigger :global(.poodle-icon) {
    width: 0.875rem;
    height: 0.875rem;
  }

  .poodle-data-table__row-action-btn--danger,
  .poodle-data-table__action-link.poodle-data-table__row-action-btn--danger {
    color: var(--poodle-color-feedback-danger-text);
  }

  .poodle-data-table__expanded-row td {
    padding: 0;
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 96%, transparent);
  }

  .poodle-data-table__expanded-panel {
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
  }

  .poodle-data-table__empty {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .poodle-data-table__footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--poodle-space-stack-md);
    padding: var(--poodle-space-control-y) var(--poodle-space-panel-x);
    border-top: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent);
  }

  .poodle-data-table__pagination-summary {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size);
  }

  .poodle-data-table__pagination-actions {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-lg);
    justify-content: flex-end;
    flex-wrap: wrap;
  }

  .poodle-data-table__limit {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size);
  }

  .poodle-data-table__limit :global(.poodle-select) {
    min-width: 5rem;
  }

  .poodle-data-table__pagination-controls {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  .poodle-data-table__pagination-page {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size);
    white-space: nowrap;
  }

  .poodle-data-table__loading-block {
    display: inline-flex;
    width: 100%;
    height: 0.875rem;
    border-radius: var(--poodle-radius-control);
    background: linear-gradient(
      90deg,
      color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent) 0%,
      color-mix(in srgb, var(--poodle-color-background-panel) 88%, transparent) 50%,
      color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent) 100%
    );
  }

  .poodle-data-table__loading-block--checkbox {
    width: 1.125rem;
    height: 1.125rem;
  }

  .poodle-data-table__loading-block--action {
    width: 4rem;
    margin-left: auto;
  }

  .poodle-data-table__hide-mobile {
    display: table-cell;
  }

  .poodle-data-table--compact th,
  .poodle-data-table--compact td,
  .poodle-data-table--compact .poodle-data-table__filters-row td,
  .poodle-data-table--compact .poodle-data-table__expanded-panel,
  .poodle-data-table--compact .poodle-data-table__footer {
    padding-top: 0.25rem;
    padding-bottom: 0.25rem;
  }

  .poodle-data-table--striped tbody tr:nth-child(even) {
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 82%, transparent);
  }

  .poodle-data-table--striped tbody tr.poodle-selected:nth-child(even) {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent);
  }

  .poodle-data-table[data-size="xs"] .poodle-data-table__toolbar-btn {
    font-size: 0.6875rem;
    padding: 0.1875rem 0.375rem;
  }

  .poodle-data-table[data-size="xs"] .poodle-data-table__toolbar-btn :global(.poodle-icon) {
    width: 0.75rem;
    height: 0.75rem;
  }

  .poodle-data-table[data-size="xs"] th,
  .poodle-data-table[data-size="xs"] td,
  .poodle-data-table[data-size="xs"] .poodle-data-table__filters-row td,
  .poodle-data-table[data-size="xs"] .poodle-data-table__footer {
    font-size: 0.6875rem;
  }

  .poodle-data-table[data-size="xs"] th,
  .poodle-data-table[data-size="xs"] td,
  .poodle-data-table[data-size="xs"] .poodle-data-table__filters-row td {
    padding-left: 0.5rem;
    padding-right: 0.5rem;
  }

  .poodle-data-table[data-size="xs"] .poodle-data-table__sort,
  .poodle-data-table[data-size="xs"] .poodle-data-table__row-action-btn,
  .poodle-data-table[data-size="xs"] .poodle-data-table__action-link,
  .poodle-data-table[data-size="xs"] .poodle-data-table__actions-trigger {
    min-height: 1.25rem;
  }

  .poodle-data-table[data-size="xs"] .poodle-data-table__selection {
    width: 2.5rem;
  }

  .poodle-data-table[data-size="sm"] .poodle-data-table__toolbar-btn {
    font-size: 0.71875rem;
    padding: 0.25rem 0.4375rem;
  }

  .poodle-data-table[data-size="sm"] th,
  .poodle-data-table[data-size="sm"] td,
  .poodle-data-table[data-size="sm"] .poodle-data-table__filters-row td,
  .poodle-data-table[data-size="sm"] .poodle-data-table__footer {
    font-size: 0.71875rem;
  }

  .poodle-data-table[data-size="sm"] th,
  .poodle-data-table[data-size="sm"] td,
  .poodle-data-table[data-size="sm"] .poodle-data-table__filters-row td {
    padding-left: 0.625rem;
    padding-right: 0.625rem;
  }

  .poodle-data-table[data-size="sm"] .poodle-data-table__sort,
  .poodle-data-table[data-size="sm"] .poodle-data-table__row-action-btn,
  .poodle-data-table[data-size="sm"] .poodle-data-table__action-link,
  .poodle-data-table[data-size="sm"] .poodle-data-table__actions-trigger {
    min-height: 1.375rem;
  }

  .poodle-data-table[data-size="sm"] .poodle-data-table__selection {
    width: 2.75rem;
  }

  .poodle-data-table[data-size="lg"] .poodle-data-table__toolbar-btn {
    font-size: 0.8125rem;
    padding: 0.375rem 0.625rem;
  }

  .poodle-data-table[data-size="lg"] .poodle-data-table__toolbar-btn :global(.poodle-icon) {
    width: 1rem;
    height: 1rem;
  }

  .poodle-data-table[data-size="lg"] th,
  .poodle-data-table[data-size="lg"] td,
  .poodle-data-table[data-size="lg"] .poodle-data-table__filters-row td,
  .poodle-data-table[data-size="lg"] .poodle-data-table__footer {
    font-size: 0.8125rem;
  }

  .poodle-data-table[data-size="lg"] th,
  .poodle-data-table[data-size="lg"] td,
  .poodle-data-table[data-size="lg"] .poodle-data-table__filters-row td {
    padding-left: 0.875rem;
    padding-right: 0.875rem;
  }

  .poodle-data-table[data-size="lg"] .poodle-data-table__sort,
  .poodle-data-table[data-size="lg"] .poodle-data-table__row-action-btn,
  .poodle-data-table[data-size="lg"] .poodle-data-table__action-link,
  .poodle-data-table[data-size="lg"] .poodle-data-table__actions-trigger {
    min-height: 2.125rem;
  }

  .poodle-data-table[data-size="lg"] .poodle-data-table__selection {
    width: 3.625rem;
  }

  .poodle-data-table[data-size="xl"] .poodle-data-table__toolbar-btn {
    font-size: 0.875rem;
    padding: 0.4375rem 0.75rem;
  }

  .poodle-data-table[data-size="xl"] .poodle-data-table__toolbar-btn :global(.poodle-icon) {
    width: 1.125rem;
    height: 1.125rem;
  }

  .poodle-data-table[data-size="xl"] th,
  .poodle-data-table[data-size="xl"] td,
  .poodle-data-table[data-size="xl"] .poodle-data-table__filters-row td,
  .poodle-data-table[data-size="xl"] .poodle-data-table__footer {
    font-size: 0.875rem;
  }

  .poodle-data-table[data-size="xl"] th,
  .poodle-data-table[data-size="xl"] td,
  .poodle-data-table[data-size="xl"] .poodle-data-table__filters-row td {
    padding-left: 1rem;
    padding-right: 1rem;
  }

  .poodle-data-table[data-size="xl"] .poodle-data-table__sort,
  .poodle-data-table[data-size="xl"] .poodle-data-table__row-action-btn,
  .poodle-data-table[data-size="xl"] .poodle-data-table__action-link,
  .poodle-data-table[data-size="xl"] .poodle-data-table__actions-trigger {
    min-height: 2.25rem;
  }

  .poodle-data-table[data-size="xl"] .poodle-data-table__selection {
    width: 4rem;
  }

  /* ── Density variants ─────────────────────────────────────── */

  .poodle-data-table[data-density="compact"] th,
  .poodle-data-table[data-density="compact"] td {
    padding-top: 0.25rem;
    padding-bottom: 0.25rem;
  }

  .poodle-data-table[data-density="compact"] .poodle-data-table__footer {
    padding-left: var(--poodle-space-control-x);
    padding-right: var(--poodle-space-control-x);
  }

  .poodle-data-table[data-density="comfortable"] th,
  .poodle-data-table[data-density="comfortable"] td {
    padding-top: calc(var(--poodle-space-panel-y) * 1.25);
    padding-bottom: calc(var(--poodle-space-panel-y) * 1.25);
  }

  .poodle-data-table[data-density="comfortable"] .poodle-data-table__footer {
    padding-left: calc(var(--poodle-space-panel-x) * 1.25);
    padding-right: calc(var(--poodle-space-panel-x) * 1.25);
  }

  @media (max-width: 48rem) {
    .poodle-data-table__hide-mobile {
      display: none;
    }

    .poodle-data-table__footer {
      flex-direction: column;
      align-items: stretch;
    }

    .poodle-data-table__pagination-actions {
      justify-content: stretch;
      flex-direction: column;
      align-items: stretch;
    }

    .poodle-data-table__limit,
    .poodle-data-table__pagination-controls {
      justify-content: center;
    }
  }
</style>
