<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import {
    Button,
    Checkbox,
    Icon,
    Menu,
    Popover,
    SearchField,
    Select,
    TextInput,
    getUiPresentation,
    resolveSemanticControlSize,
  } from "@poodle/svelte-primitives";
  import type { ControlDensity, ControlSize, MenuItem, SemanticControlSizeRole } from "@poodle/svelte-primitives";

  import type {
    TableCellValue,
    TableColumn,
    TableFilters,
    TablePagination,
    TableRow,
    TableRowAction,
    TableSortDirection,
  } from "./types";

  export let ariaLabel = "Data table";
  export let columns: TableColumn[] = [];
  export let rows: TableRow[] = [];
  export let filters: TableFilters = {};
  export let pagination: TablePagination | null = null;
  export let loading = false;
  export let loadingRows = 5;
  export let selectable = false;
  export let selectedRowIds: string[] = [];
  export let sortColumnId: string | null = null;
  export let sortDirection: TableSortDirection = "asc";
  export let rowActionLabel = "Open";
  export let showRowActions = true;
  export let rowActions: TableRowAction[] | ((row: TableRow) => TableRowAction[]) = [];
  export let expandedRowWhen: (row: TableRow) => boolean = () => false;
  export let emptyMessage = "No rows match the current view.";
  export let hiddenColumnIds: string[] = [];
  export let showColumnVisibility = false;
  export let showExport = false;
  export let exportFilename = "export.csv";
  export let limitOptions: number[] = [10, 20, 50, 100];
  export let showLimitSelector = true;
  export let compact = false;
  export let striped = false;
  export let stickyHeader = false;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    sortChange: { columnId: string; direction: TableSortDirection };
    rowToggle: { rowId: string; selected: boolean };
    toggleAll: { selected: boolean };
    rowAction: { rowId: string };
    rowActionSelect: { rowId: string; row: TableRow; action: TableRowAction };
    columnVisibilityChange: { columnId: string; visible: boolean };
    exportCsv: { filename: string };
    rowClick: { rowId: string; row: TableRow };
    filterChange: { filters: TableFilters };
    pageChange: { page: number };
    limitChange: { limit: number };
  }>();

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: visibleColumns = columns.filter((column) => !hiddenColumnIds.includes(column.id));
  $: hideableColumns = columns.filter((column) => column.hideable !== false);
  $: hasFilters = visibleColumns.some((column) => column.filterable);
  $: selectableRowCount = selectable ? rows.length : 0;
  $: selectionCount = selectable ? rows.filter((row) => selectedRowIds.includes(row.id)).length : 0;
  $: allRowsSelected = selectableRowCount > 0 && selectionCount === selectableRowCount;
  $: mixedSelection = selectionCount > 0 && !allRowsSelected;
  $: hasCustomCellSlot = $$slots.cell !== undefined;
  $: hasExpandedRowSlot = $$slots.expandedRow !== undefined;
  $: hasEmptySlot = $$slots.empty !== undefined;
  $: hasRichRowActions = typeof rowActions === "function" || rowActions.length > 0;
  $: showLegacyRowAction = showRowActions && !hasRichRowActions;
  $: showActionsColumn = showRowActions && (showLegacyRowAction || hasRichRowActions);
  $: totalPages = pagination ? Math.max(1, Math.ceil(pagination.total / pagination.limit)) : 1;
  $: showPaginationFooter = pagination !== null && (totalPages > 1 || showLimitSelector);
  $: columnCount = visibleColumns.length + (selectable ? 1 : 0) + (showActionsColumn ? 1 : 0);

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
      kind: action.kind === "separator" ? "separator" : "action",
    };
  }

  function getActionButtonClass(action: TableRowAction): string {
    return action.tone === "danger"
      ? "data-table__row-action-btn data-table__row-action-btn--danger"
      : "data-table__row-action-btn";
  }

  function requestRowAction(row: TableRow, action: TableRowAction): void {
    if (action.disabled || action.kind === "separator") {
      return;
    }

    dispatch("rowActionSelect", { rowId: row.id, row, action });
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

    dispatch("rowClick", { rowId: row.id, row });
  }

  function requestFilterChange(columnId: string, value: string): void {
    dispatch("filterChange", {
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

    dispatch("sortChange", { columnId: column.id, direction });
  }

  function toggleColumnVisibility(columnId: string): void {
    const isHidden = hiddenColumnIds.includes(columnId);
    dispatch("columnVisibilityChange", { columnId, visible: isHidden });
  }

  function requestPageChange(page: number): void {
    if (!pagination) {
      return;
    }

    const nextPage = Math.min(Math.max(page, 1), totalPages);
    if (nextPage === pagination.page) {
      return;
    }

    dispatch("pageChange", { page: nextPage });
  }

  function requestLimitChange(limit: number): void {
    dispatch("limitChange", { limit });
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
    dispatch("exportCsv", { filename: exportFilename });
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
  class="data-table"
  class:data-table--compact={compact}
  class:data-table--striped={striped}
  class:data-table--sticky-header={stickyHeader}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  {#if showColumnVisibility || showExport}
    <div class="data-table__toolbar">
      {#if showExport}
        <button
          type="button"
          class="data-table__toolbar-btn"
          on:click={handleExport}
          aria-label="Export as CSV"
        >
          <Icon name="download" />
          Export
        </button>
      {/if}

      {#if showColumnVisibility && hideableColumns.length > 0}
        <Popover placement="bottom-end" ariaLabel="Column visibility">
          <span slot="trigger" class="data-table__toolbar-btn">
            <Icon name="columns-3" />
            Columns
          </span>
          <div class="data-table__col-menu" role="menu">
            {#each hideableColumns as column}
              <label class="data-table__col-menu-item">
                <Checkbox
                  ariaLabel={column.label}
                  checked={!hiddenColumnIds.includes(column.id)}
                  on:checkedChange={() => toggleColumnVisibility(column.id)}
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
    <caption class="data-table__caption">
      {#if selectable}
        {ariaLabel}. {selectionCount} selected row{selectionCount === 1 ? "" : "s"} out of {selectableRowCount}.
      {:else}
        {ariaLabel}.
      {/if}
    </caption>
    <thead>
      <tr>
        {#if selectable}
          <th class="data-table__selection">
            <Checkbox
              ariaLabel="Select all visible rows"
              checked={allRowsSelected}
              mixed={mixedSelection}
              on:checkedChange={(event) => dispatch("toggleAll", { selected: event.detail.checked })}
            />
          </th>
        {/if}
        {#each visibleColumns as column}
          <th
            style={getColumnStyle(column)}
            class:center-align={column.align === "center"}
            class:end-align={column.align === "end"}
            class:data-table__hide-mobile={column.hideOnMobile === true}
            aria-sort={column.sortable && sortColumnId === column.id ? (sortDirection === "asc" ? "ascending" : "descending") : column.sortable ? "none" : undefined}
          >
            {#if column.sortable}
              <button
                type="button"
                class="data-table__sort"
                on:click={() => requestSort(column)}
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
          <th scope="col" class="data-table__actions-header">Actions</th>
        {/if}
      </tr>
      {#if hasFilters}
        <tr class="data-table__filters-row">
          {#if selectable}
            <td class="data-table__selection" aria-hidden="true"></td>
          {/if}
          {#each visibleColumns as column}
            <td
              style={getColumnStyle(column)}
              class:center-align={column.align === "center"}
              class:end-align={column.align === "end"}
              class:data-table__hide-mobile={column.hideOnMobile === true}
            >
              {#if column.filterable}
                {#if column.filterType === "select" && column.filterOptions}
                  <Select
                    id={getFilterInputId(column)}
                    value={filters[column.id] ?? ""}
                    items={[
                      { value: "", label: "All" },
                      ...column.filterOptions.map((option) =>
                        typeof option === "string" ? { value: option, label: option } : option,
                      ),
                    ]}
                    onchange={(value) => requestFilterChange(column.id, value)}
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
                    on:valueChange={(event) => requestFilterChange(column.id, event.detail.value)}
                  />
                {:else}
                  <SearchField
                    id={getFilterInputId(column)}
                    value={filters[column.id] ?? ""}
                    placeholder={`Filter ${column.label.toLowerCase()}...`}
                    ariaLabel={`Filter ${column.label}`}
                    debounce={300}
                    on:valueChange={(event) => requestFilterChange(column.id, event.detail.value)}
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
          <tr class="data-table__loading-row">
            {#if selectable}
              <td class="data-table__selection">
                <span class="data-table__loading-block data-table__loading-block--checkbox"></span>
              </td>
            {/if}
            {#each visibleColumns as column}
              <td
                style={getColumnStyle(column)}
                class:center-align={column.align === "center"}
                class:end-align={column.align === "end"}
                class:data-table__hide-mobile={column.hideOnMobile === true}
              >
                <span class="data-table__loading-block"></span>
              </td>
            {/each}
            {#if showActionsColumn}
              <td class="data-table__actions">
                <span class="data-table__loading-block data-table__loading-block--action"></span>
              </td>
            {/if}
          </tr>
        {/each}
      {:else if rows.length === 0}
        <tr>
          <td colspan={columnCount} class="data-table__empty">
            {#if hasEmptySlot}
              <slot name="empty" />
            {:else}
              {emptyMessage}
            {/if}
          </td>
        </tr>
      {:else}
        {#each rows as row (row.id)}
          <tr
            class:selected={selectable && selectedRowIds.includes(row.id)}
            aria-selected={selectable ? selectedRowIds.includes(row.id) : undefined}
            on:click={(event) => handleRowClick(event, row)}
          >
            {#if selectable}
              <td class="data-table__selection">
                <Checkbox
                  ariaLabel={`Select row ${getRowPrimaryLabel(row)}`}
                  checked={selectedRowIds.includes(row.id)}
                  on:checkedChange={(event) => dispatch("rowToggle", { rowId: row.id, selected: event.detail.checked })}
                />
              </td>
            {/if}
            {#each visibleColumns as column, index}
              <svelte:element
                this={index === 0 && column.isRowHeader !== false ? "th" : "td"}
                scope={index === 0 && column.isRowHeader !== false ? "row" : undefined}
                style={getColumnStyle(column)}
                class:center-align={column.align === "center"}
                class:end-align={column.align === "end"}
                class:data-table__hide-mobile={column.hideOnMobile === true}
              >
                {#if hasCustomCellSlot}
                  <slot name="cell" column={column} row={row} value={row.cells[column.id] ?? null} />
                {:else}
                  <div class="data-table__cell">
                    <span>{stringifyCellValue(row.cells[column.id] ?? null) || "—"}</span>
                    {#if column.id === visibleColumns[0]?.id && row.summary}
                      <small>{row.summary}</small>
                    {/if}
                  </div>
                {/if}
              </svelte:element>
            {/each}
            {#if showActionsColumn}
              <td class="data-table__actions">
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
                        on:click|stopPropagation={() => requestRowAction(row, action)}
                      >
                        {action.label}
                      </button>
                    {/if}
                  {:else if actions.length > 0}
                    <Menu
                      items={actions.map(toMenuItem)}
                      ariaLabel={`Actions for ${getRowPrimaryLabel(row)}`}
                      placement="bottom-end"
                      on:action={(event) => handleMenuAction(row, actions, event.detail.value)}
                    >
                      <span slot="trigger" class="data-table__actions-trigger" data-row-action-trigger="true">
                        <Icon name="ellipsis" />
                        Actions
                      </span>
                    </Menu>
                  {/if}
                {:else if showLegacyRowAction}
                  <button
                    type="button"
                    class="data-table__row-action-btn"
                    aria-label={`${rowActionLabel} ${getRowPrimaryLabel(row)}`}
                    on:click|stopPropagation={() => dispatch("rowAction", { rowId: row.id })}
                  >
                    {rowActionLabel}
                  </button>
                {/if}
              </td>
            {/if}
          </tr>
          {#if hasExpandedRowSlot && expandedRowWhen(row)}
            <tr class="data-table__expanded-row">
              <td colspan={columnCount}>
                <div class="data-table__expanded-panel">
                  <slot name="expandedRow" row={row} />
                </div>
              </td>
            </tr>
          {/if}
        {/each}
      {/if}
    </tbody>
  </table>

  {#if showPaginationFooter && pagination}
    <div class="data-table__footer">
      <p class="data-table__pagination-summary">{getPaginationSummary()}</p>
      <div class="data-table__pagination-actions">
        {#if showLimitSelector && limitOptions.length > 0}
          <label class="data-table__limit">
            <span>Show</span>
            <Select
              id={`${ariaLabel.replace(/\s+/g, "-").toLowerCase()}-limit`}
              value={String(pagination.limit)}
              items={limitOptions.map((option) => ({ value: String(option), label: String(option) }))}
              onchange={(value) => requestLimitChange(Number(value))}
              ariaLabel="Items per page"
            />
            <span>per page</span>
          </label>
        {/if}
        {#if totalPages > 1}
          <div class="data-table__pagination-controls">
            <Button type="button" variant="ghost" size="sm" disabled={pagination.page <= 1} on:click={() => requestPageChange(1)}>
              First
            </Button>
            <Button type="button" variant="ghost" size="sm" disabled={pagination.page <= 1} on:click={() => requestPageChange(pagination.page - 1)}>
              Previous
            </Button>
            <span class="data-table__pagination-page">Page {pagination.page} of {totalPages}</span>
            <Button type="button" variant="ghost" size="sm" disabled={pagination.page >= totalPages} on:click={() => requestPageChange(pagination.page + 1)}>
              Next
            </Button>
            <Button type="button" variant="ghost" size="sm" disabled={pagination.page >= totalPages} on:click={() => requestPageChange(totalPages)}>
              Last
            </Button>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .data-table {
    overflow: auto;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-color-background-panel);
  }

  .data-table__toolbar {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: var(--poodle-space-inline-md);
    padding: var(--poodle-space-control-y) var(--poodle-space-panel-x);
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent);
  }

  .data-table__toolbar-btn {
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

  .data-table__toolbar-btn:hover {
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent);
  }

  .data-table__toolbar-btn:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .data-table__toolbar-btn :global(.poodle-icon) {
    width: 0.875rem;
    height: 0.875rem;
  }

  .data-table__col-menu {
    display: flex;
    flex-direction: column;
  }

  .data-table__col-menu-item {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-md);
    padding: var(--poodle-space-control-y) var(--poodle-space-control-x);
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    cursor: pointer;
    font-size: var(--poodle-typography-label-size);
    color: var(--poodle-color-text-primary);
  }

  .data-table__col-menu-item:hover {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  .data-table__caption {
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

  .data-table--sticky-header thead th {
    position: sticky;
    top: 0;
    z-index: 1;
  }

  .data-table__filters-row td {
    padding: var(--poodle-space-control-y) var(--poodle-space-panel-x);
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent);
  }

  .data-table__filters-row :global(.select),
  .data-table__filters-row :global(.text-input),
  .data-table__filters-row :global(.text-input__field) {
    width: 100%;
  }

  tbody tr.selected {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 8%, transparent);
  }

  tbody tr:hover {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 5%, transparent);
  }

  .data-table__selection {
    width: 3.25rem;
  }

  .data-table__sort,
  .data-table__row-action-btn,
  .data-table__action-link,
  .data-table__actions-trigger {
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

  .data-table__sort:focus-visible,
  .data-table__row-action-btn:focus-visible,
  .data-table__action-link:focus-visible,
  .data-table__actions-trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
    border-radius: var(--poodle-radius-control);
  }

  .data-table__cell {
    display: grid;
    gap: 0.25rem;
  }

  .data-table__cell small {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .center-align {
    text-align: center;
  }

  .end-align {
    text-align: right;
  }

  .data-table__actions-header,
  .data-table__actions {
    width: 7.5rem;
    text-align: right;
    white-space: nowrap;
  }

  .data-table__actions-trigger {
    justify-content: flex-end;
  }

  .data-table__actions-trigger :global(.poodle-icon) {
    width: 0.875rem;
    height: 0.875rem;
  }

  .data-table__row-action-btn--danger,
  .data-table__action-link.data-table__row-action-btn--danger {
    color: var(--poodle-color-feedback-danger-text);
  }

  .data-table__expanded-row td {
    padding: 0;
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 96%, transparent);
  }

  .data-table__expanded-panel {
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
  }

  .data-table__empty {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .data-table__footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--poodle-space-stack-md);
    padding: var(--poodle-space-control-y) var(--poodle-space-panel-x);
    border-top: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent);
  }

  .data-table__pagination-summary {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size);
  }

  .data-table__pagination-actions {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-lg);
    justify-content: flex-end;
    flex-wrap: wrap;
  }

  .data-table__limit {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size);
  }

  .data-table__limit :global(.select) {
    min-width: 5rem;
  }

  .data-table__pagination-controls {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  .data-table__pagination-page {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size);
    white-space: nowrap;
  }

  .data-table__loading-block {
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

  .data-table__loading-block--checkbox {
    width: 1.125rem;
    height: 1.125rem;
  }

  .data-table__loading-block--action {
    width: 4rem;
    margin-left: auto;
  }

  .data-table__hide-mobile {
    display: table-cell;
  }

  .data-table--compact th,
  .data-table--compact td,
  .data-table--compact .data-table__filters-row td,
  .data-table--compact .data-table__expanded-panel,
  .data-table--compact .data-table__footer {
    padding-top: var(--poodle-space-control-y-tight);
    padding-bottom: var(--poodle-space-control-y-tight);
  }

  .data-table--striped tbody tr:nth-child(even) {
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 82%, transparent);
  }

  .data-table--striped tbody tr.selected:nth-child(even) {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent);
  }

  .data-table[data-size="xs"] .data-table__toolbar-btn {
    font-size: 0.6875rem;
    padding: 0.1875rem 0.375rem;
  }

  .data-table[data-size="xs"] .data-table__toolbar-btn :global(.poodle-icon) {
    width: 0.75rem;
    height: 0.75rem;
  }

  .data-table[data-size="xs"] th,
  .data-table[data-size="xs"] td,
  .data-table[data-size="xs"] .data-table__filters-row td,
  .data-table[data-size="xs"] .data-table__footer {
    padding-left: 0.5rem;
    padding-right: 0.5rem;
    font-size: 0.6875rem;
  }

  .data-table[data-size="xs"] th,
  .data-table[data-size="xs"] td {
    padding-top: 0.25rem;
    padding-bottom: 0.25rem;
  }

  .data-table[data-size="xs"] .data-table__sort,
  .data-table[data-size="xs"] .data-table__row-action-btn,
  .data-table[data-size="xs"] .data-table__action-link,
  .data-table[data-size="xs"] .data-table__actions-trigger {
    min-height: 1.25rem;
  }

  .data-table[data-size="xs"] .data-table__selection {
    width: 2.5rem;
  }

  .data-table[data-size="sm"] .data-table__toolbar-btn {
    font-size: 0.71875rem;
    padding: 0.25rem 0.4375rem;
  }

  .data-table[data-size="sm"] th,
  .data-table[data-size="sm"] td,
  .data-table[data-size="sm"] .data-table__filters-row td,
  .data-table[data-size="sm"] .data-table__footer {
    padding-left: 0.625rem;
    padding-right: 0.625rem;
    font-size: 0.71875rem;
  }

  .data-table[data-size="sm"] th,
  .data-table[data-size="sm"] td {
    padding-top: 0.3125rem;
    padding-bottom: 0.3125rem;
  }

  .data-table[data-size="sm"] .data-table__sort,
  .data-table[data-size="sm"] .data-table__row-action-btn,
  .data-table[data-size="sm"] .data-table__action-link,
  .data-table[data-size="sm"] .data-table__actions-trigger {
    min-height: 1.375rem;
  }

  .data-table[data-size="sm"] .data-table__selection {
    width: 2.75rem;
  }

  .data-table[data-size="lg"] .data-table__toolbar-btn {
    font-size: 0.8125rem;
    padding: 0.375rem 0.625rem;
  }

  .data-table[data-size="lg"] .data-table__toolbar-btn :global(.poodle-icon) {
    width: 1rem;
    height: 1rem;
  }

  .data-table[data-size="lg"] th,
  .data-table[data-size="lg"] td,
  .data-table[data-size="lg"] .data-table__filters-row td,
  .data-table[data-size="lg"] .data-table__footer {
    padding-left: 0.875rem;
    padding-right: 0.875rem;
    font-size: 0.8125rem;
  }

  .data-table[data-size="lg"] th,
  .data-table[data-size="lg"] td {
    padding-top: 0.5rem;
    padding-bottom: 0.5rem;
  }

  .data-table[data-size="lg"] .data-table__sort,
  .data-table[data-size="lg"] .data-table__row-action-btn,
  .data-table[data-size="lg"] .data-table__action-link,
  .data-table[data-size="lg"] .data-table__actions-trigger {
    min-height: 2.125rem;
  }

  .data-table[data-size="lg"] .data-table__selection {
    width: 3.625rem;
  }

  .data-table[data-size="xl"] .data-table__toolbar-btn {
    font-size: 0.875rem;
    padding: 0.4375rem 0.75rem;
  }

  .data-table[data-size="xl"] .data-table__toolbar-btn :global(.poodle-icon) {
    width: 1.125rem;
    height: 1.125rem;
  }

  .data-table[data-size="xl"] th,
  .data-table[data-size="xl"] td,
  .data-table[data-size="xl"] .data-table__filters-row td,
  .data-table[data-size="xl"] .data-table__footer {
    padding-left: 1rem;
    padding-right: 1rem;
    font-size: 0.875rem;
  }

  .data-table[data-size="xl"] th,
  .data-table[data-size="xl"] td {
    padding-top: 0.625rem;
    padding-bottom: 0.625rem;
  }

  .data-table[data-size="xl"] .data-table__sort,
  .data-table[data-size="xl"] .data-table__row-action-btn,
  .data-table[data-size="xl"] .data-table__action-link,
  .data-table[data-size="xl"] .data-table__actions-trigger {
    min-height: 2.25rem;
  }

  .data-table[data-size="xl"] .data-table__selection {
    width: 4rem;
  }

  @media (max-width: 48rem) {
    .data-table__hide-mobile {
      display: none;
    }

    .data-table__footer {
      flex-direction: column;
      align-items: stretch;
    }

    .data-table__pagination-actions {
      justify-content: stretch;
      flex-direction: column;
      align-items: stretch;
    }

    .data-table__limit,
    .data-table__pagination-controls {
      justify-content: center;
    }
  }
</style>
