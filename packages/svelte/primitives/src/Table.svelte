<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, TableColumn, TableRow } from "./types";

  export let columns: TableColumn[] = [];
  export let rows: TableRow[] = [];
  export let caption: string | null = null;
  export let emptyMessage = "No rows available.";
  export let ariaLabel: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: rowHeaderColumnId =
    columns.find((column) => column.isRowHeader)?.id ?? columns[0]?.id ?? null;
</script>

<div class="table-shell" data-size={resolvedSize} data-density={resolvedDensity} aria-label={ariaLabel ?? undefined}>
  <table class="table">
    {#if caption}
      <caption class="table__caption">{caption}</caption>
    {/if}

    <thead>
      <tr>
        {#each columns as column (column.id)}
          <th class="table__header" data-align={column.align ?? "start"} scope="col">
            {column.label}
          </th>
        {/each}
      </tr>
    </thead>

    <tbody>
      {#if rows.length === 0}
        <tr>
          <td class="table__empty" colspan={Math.max(columns.length, 1)}>
            {emptyMessage}
          </td>
        </tr>
      {:else}
        {#each rows as row (row.id)}
          <tr class="table__row">
            {#each columns as column (column.id)}
              {#if column.id === rowHeaderColumnId}
                <th
                  class="table__cell table__cell--row-header"
                  data-align={column.align ?? "start"}
                  scope="row"
                >
                  {row.cells[column.id] ?? ""}
                </th>
              {:else}
                <td class="table__cell" data-align={column.align ?? "start"}>
                  {row.cells[column.id] ?? ""}
                </td>
              {/if}
            {/each}
          </tr>
        {/each}
      {/if}
    </tbody>
  </table>
</div>

<style>
  .table-shell {
    min-width: 0;
    overflow-x: auto;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 78%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 96%, transparent);
  }

  .table {
    width: 100%;
    border-collapse: collapse;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .table__caption {
    caption-side: top;
    padding: 0.625rem 0.75rem;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.8125rem;
    font-weight: 500;
    text-align: left;
  }

  .table__header,
  .table__cell,
  .table__empty {
    padding: 0.5rem 0.75rem;
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
    text-align: left;
    vertical-align: middle;
  }

  .table__header {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    background: color-mix(in srgb, var(--poodle-surface) 91%, var(--poodle-color-text-primary));
  }

  .table__cell--row-header {
    font-weight: 600;
  }

  .table__header[data-align="end"],
  .table__cell[data-align="end"] {
    text-align: right;
  }

  tbody .table__row:last-child .table__cell,
  tbody .table__row:last-child .table__cell--row-header,
  tbody tr:last-child .table__empty {
    border-bottom: 0;
  }

  .table__empty {
    color: var(--poodle-color-text-secondary);
  }

  /* Size variants */
  .table-shell[data-size="xs"] .table { font-size: 0.6875rem; }
  .table-shell[data-size="xs"] .table__header { font-size: 0.5625rem; }
  .table-shell[data-size="xs"] .table__header,
  .table-shell[data-size="xs"] .table__cell { padding-block: 0.3125rem; }

  .table-shell[data-size="sm"] .table { font-size: 0.75rem; }
  .table-shell[data-size="sm"] .table__header { font-size: 0.625rem; }
  .table-shell[data-size="sm"] .table__header,
  .table-shell[data-size="sm"] .table__cell { padding-block: 0.375rem; }

  .table-shell[data-size="lg"] .table { font-size: 0.875rem; }
  .table-shell[data-size="lg"] .table__header { font-size: 0.75rem; }
  .table-shell[data-size="lg"] .table__header,
  .table-shell[data-size="lg"] .table__cell { padding-block: 0.625rem; }

  .table-shell[data-size="xl"] .table { font-size: 0.9375rem; }
  .table-shell[data-size="xl"] .table__header { font-size: 0.8125rem; }
  .table-shell[data-size="xl"] .table__header,
  .table-shell[data-size="xl"] .table__cell { padding-block: 0.75rem; }

  /* Density variants */
  .table-shell[data-density="compact"] .table__header,
  .table-shell[data-density="compact"] .table__cell { padding-inline: 0.5rem; }

  .table-shell[data-density="comfortable"] .table__header,
  .table-shell[data-density="comfortable"] .table__cell { padding-inline: 1.125rem; }
</style>
