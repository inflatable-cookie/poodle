<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Checkbox } from "@pug/svelte-primitives";

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

  const dispatch = createEventDispatcher<{
    sortChange: { columnId: string; direction: TableSortDirection };
    rowToggle: { rowId: string; selected: boolean };
    toggleAll: { selected: boolean };
    rowAction: { rowId: string };
  }>();

  $: selectableRowCount = rows.length;
  $: selectionCount = rows.filter((row) => selectedRowIds.includes(row.id)).length;
  $: allRowsSelected = selectableRowCount > 0 && selectionCount === selectableRowCount;
  $: mixedSelection = selectionCount > 0 && !allRowsSelected;

  function requestSort(column: TableColumn): void {
    if (!column.isSortable) {
      return;
    }

    const direction =
      sortColumnId === column.id && sortDirection === "asc"
        ? "desc"
        : "asc";

    dispatch("sortChange", { columnId: column.id, direction });
  }
</script>

<div class="data-table">
  <table aria-label={ariaLabel}>
    <caption class="data-table__caption">
      {ariaLabel}. {selectionCount} selected row{selectionCount === 1 ? "" : "s"} out of {selectableRowCount}.
    </caption>
    <thead>
      <tr>
        <th class="data-table__selection">
          <Checkbox
            ariaLabel="Select all visible rows"
            isChecked={allRowsSelected}
            isMixed={mixedSelection}
            on:checkedChange={(event) => dispatch("toggleAll", { selected: event.detail.checked })}
          />
        </th>
        {#each columns as column}
          <th
            scope="col"
            class:end-align={column.align === "end"}
            aria-sort={column.isSortable && sortColumnId === column.id ? (sortDirection === "asc" ? "ascending" : "descending") : column.isSortable ? "none" : undefined}
          >
            {#if column.isSortable}
              <button
                type="button"
                class="data-table__sort"
                on:click={() => requestSort(column)}
                aria-label={`Sort by ${column.label}${sortColumnId === column.id ? `, currently ${sortDirection}` : ""}`}
              >
                <span>{column.label}</span>
                {#if sortColumnId === column.id}
                  <span aria-hidden="true">{sortDirection === "asc" ? "↑" : "↓"}</span>
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
          <td colspan={columns.length + (showRowActions ? 2 : 1)} class="data-table__empty">
            {emptyMessage}
          </td>
        </tr>
      {:else}
        {#each rows as row}
          <tr class:selected={selectedRowIds.includes(row.id)} aria-selected={selectedRowIds.includes(row.id)}>
            <td class="data-table__selection">
              <Checkbox
                ariaLabel={`Select row ${row.cells[columns[0]?.id ?? "id"] ?? row.id}`}
                isChecked={selectedRowIds.includes(row.id)}
                on:checkedChange={(event) =>
                  dispatch("rowToggle", { rowId: row.id, selected: event.detail.checked })}
              />
            </td>
            {#each columns as column, index}
              <svelte:element this={index === 0 ? "th" : "td"} scope={index === 0 ? "row" : undefined} class:end-align={column.align === "end"}>
                <div class="data-table__cell">
                  <span>{row.cells[column.id] ?? "—"}</span>
                  {#if column.id === columns[0]?.id && row.summary}
                    <small>{row.summary}</small>
                  {/if}
                </div>
              </svelte:element>
            {/each}
            {#if showRowActions}
              <td class="data-table__actions">
                <button
                  type="button"
                  aria-label={`${rowActionLabel} ${row.cells[columns[0]?.id ?? "id"] ?? row.id}`}
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
    border: 1px solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    background: var(--pug-color-background-panel);
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  .data-table__caption {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }

  th,
  td {
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border-bottom: 1px solid var(--pug-color-border-subtle);
    text-align: left;
    vertical-align: middle;
  }

  thead th {
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-label-family);
    font-size: var(--pug-typography-label-size);
    font-weight: var(--pug-typography-label-weight);
    line-height: var(--pug-typography-label-lineHeight);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 92%, transparent);
  }

  tbody tr.selected {
    background: color-mix(in srgb, var(--pug-color-accent-base) 8%, transparent);
  }

  tbody tr:hover {
    background: color-mix(in srgb, var(--pug-color-accent-base) 5%, transparent);
  }

  .data-table__selection {
    width: 52px;
  }

  .data-table__sort,
  .data-table__actions button {
    display: inline-flex;
    align-items: center;
    gap: var(--pug-space-inline-sm);
    min-height: var(--pug-size-control-height);
    padding: 0;
    border: 0;
    background: transparent;
    color: inherit;
    cursor: pointer;
    font: inherit;
  }

  .data-table__sort:focus-visible,
  .data-table__actions button:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 2px;
    border-radius: var(--pug-radius-control);
  }

  .data-table__cell {
    display: grid;
    gap: 4px;
  }

  .data-table__cell small {
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
  }

  .end-align {
    text-align: right;
  }

  .data-table__actions-header,
  .data-table__actions {
    width: 120px;
    text-align: right;
  }

  .data-table__empty {
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
  }
</style>
