<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Checkbox, Icon, Popover, getUiPresentation, resolveSemanticControlSize } from "@poodle/svelte-primitives";
  import type { ControlSize, SemanticControlSizeRole } from "@poodle/svelte-primitives";

  import type { TableColumn, TableRow, TableSortDirection } from "./types";

  export let ariaLabel = "Data table";
  export let columns: TableColumn[] = [];
  export let rows: TableRow[] = [];
  export let selectedRowIds: string[] = [];
  export let sortColumnId: string | null = null;
  export let sortDirection: TableSortDirection = "asc";
  export let rowActionLabel = "Open";
  export let showRowActions = true;
  export let emptyMessage = "No rows match the current view.";
  export let hiddenColumnIds: string[] = [];
  export let showColumnVisibility = false;
  export let showExport = false;
  export let exportFilename = "export.csv";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";

  const dispatch = createEventDispatcher<{
    sortChange: { columnId: string; direction: TableSortDirection };
    rowToggle: { rowId: string; selected: boolean };
    toggleAll: { selected: boolean };
    rowAction: { rowId: string };
    columnVisibilityChange: { columnId: string; visible: boolean };
    exportCsv: { filename: string };
  }>();

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);

  $: visibleColumns = columns.filter((c) => !hiddenColumnIds.includes(c.id));
  $: hideableColumns = columns.filter((c) => c.hideable !== false);
  $: selectableRowCount = rows.length;
  $: selectionCount = rows.filter((row) => selectedRowIds.includes(row.id)).length;
  $: allRowsSelected = selectableRowCount > 0 && selectionCount === selectableRowCount;
  $: mixedSelection = selectionCount > 0 && !allRowsSelected;

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

  function handleExport(): void {
    const header = visibleColumns.map((c) => c.label).join(",");
    const body = rows
      .map((row) =>
        visibleColumns
          .map((col) => {
            const val = row.cells[col.id] ?? "";
            return val.includes(",") || val.includes('"')
              ? `"${val.replace(/"/g, '""')}"`
              : val;
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
</script>

<div class="data-table" data-size={resolvedSize}>
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
            {#each hideableColumns as col}
              <label class="data-table__col-menu-item">
                <Checkbox
                  ariaLabel={col.label}
                  checked={!hiddenColumnIds.includes(col.id)}
                  on:checkedChange={() => toggleColumnVisibility(col.id)}
                />
                <span>{col.label}</span>
              </label>
            {/each}
          </div>
        </Popover>
      {/if}
    </div>
  {/if}

  <table aria-label={ariaLabel}>
    <caption class="data-table__caption">
      {ariaLabel}. {selectionCount} selected row{selectionCount === 1 ? "" : "s"} out of {selectableRowCount}.
    </caption>
    <thead>
      <tr>
        <th class="data-table__selection">
          <Checkbox
            ariaLabel="Select all visible rows"
            checked={allRowsSelected}
            mixed={mixedSelection}
            on:checkedChange={(event) => dispatch("toggleAll", { selected: event.detail.checked })}
          />
        </th>
        {#each visibleColumns as column}
          <th
            scope="col"
            class:end-align={column.align === "end"}
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
        {#if showRowActions}
          <th scope="col" class="data-table__actions-header">Actions</th>
        {/if}
      </tr>
    </thead>
    <tbody>
      {#if rows.length === 0}
        <tr>
          <td colspan={visibleColumns.length + (showRowActions ? 2 : 1)} class="data-table__empty">
            {emptyMessage}
          </td>
        </tr>
      {:else}
        {#each rows as row}
          <tr class:selected={selectedRowIds.includes(row.id)} aria-selected={selectedRowIds.includes(row.id)}>
            <td class="data-table__selection">
              <Checkbox
                ariaLabel={`Select row ${row.cells[visibleColumns[0]?.id ?? "id"] ?? row.id}`}
                checked={selectedRowIds.includes(row.id)}
                on:checkedChange={(event) =>
                  dispatch("rowToggle", { rowId: row.id, selected: event.detail.checked })}
              />
            </td>
            {#each visibleColumns as column, index}
              <svelte:element this={index === 0 ? "th" : "td"} scope={index === 0 ? "row" : undefined} class:end-align={column.align === "end"}>
                <div class="data-table__cell">
                  <span>{row.cells[column.id] ?? "—"}</span>
                  {#if column.id === visibleColumns[0]?.id && row.summary}
                    <small>{row.summary}</small>
                  {/if}
                </div>
              </svelte:element>
            {/each}
            {#if showRowActions}
              <td class="data-table__actions">
                <button
                  type="button"
                  aria-label={`${rowActionLabel} ${row.cells[visibleColumns[0]?.id ?? "id"] ?? row.id}`}
                  on:click={() => dispatch("rowAction", { rowId: row.id })}
                >
                  {rowActionLabel}
                </button>
              </td>
            {/if}
          </tr>
        {/each}
      {/if}
    </tbody>
  </table>
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
  .data-table__actions button {
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
  }

  .data-table__sort:focus-visible,
  .data-table__actions button:focus-visible {
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

  .end-align {
    text-align: right;
  }

  .data-table__actions-header,
  .data-table__actions {
    width: 7.5rem;
    text-align: right;
  }

  .data-table__empty {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }
</style>
