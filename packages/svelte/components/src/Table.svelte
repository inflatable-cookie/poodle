<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, TableColumn, TableRow } from "./types";

  let {
    columns = [],
    rows = [],
    caption = null,
    emptyMessage = "No rows available.",
    ariaLabel = null,
    size = null,
    sizeRole = "control",
    density = null,
  }: {
    columns?: TableColumn[];
    rows?: TableRow[];
    caption?: string | null;
    emptyMessage?: string;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
  } = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const rowHeaderColumnId = $derived(
    columns.find((column) => column.isRowHeader)?.id ?? columns[0]?.id ?? null,
  );
</script>

<div class="poodle-table-shell" data-size={resolvedSize} data-density={resolvedDensity} aria-label={ariaLabel ?? undefined}>
  <table class="poodle-table">
    {#if caption}
      <caption class="poodle-table__caption">{caption}</caption>
    {/if}

    <thead>
      <tr>
        {#each columns as column (column.id)}
          <th class="poodle-table__header" data-align={column.align ?? "start"} scope="col">
            {column.label}
          </th>
        {/each}
      </tr>
    </thead>

    <tbody>
      {#if rows.length === 0}
        <tr>
          <td class="poodle-table__empty" colspan={Math.max(columns.length, 1)}>
            {emptyMessage}
          </td>
        </tr>
      {:else}
        {#each rows as row (row.id)}
          <tr class="poodle-table__row">
            {#each columns as column (column.id)}
              {#if column.id === rowHeaderColumnId}
                <th
                  class="poodle-table__cell poodle-table__cell--row-header"
                  data-align={column.align ?? "start"}
                  scope="row"
                >
                  {row.cells[column.id] ?? ""}
                </th>
              {:else}
                <td class="poodle-table__cell" data-align={column.align ?? "start"}>
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
  .poodle-table-shell {
    min-width: 0;
    overflow-x: auto;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 78%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-recipe-table-fill, color-mix(in srgb, var(--poodle-color-background-panel) 96%, transparent));
  }

  .poodle-table {
    width: 100%;
    border-collapse: collapse;
    color: var(--poodle-recipe-table-text, var(--poodle-color-text-primary));
    font-family: var(--poodle-typography-body-family);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .poodle-table__caption {
    caption-side: top;
    padding: 0.625rem 0.75rem;
    color: var(--poodle-recipe-table-caption-text, var(--poodle-color-text-secondary));
    font-family: var(--poodle-typography-label-family);
    font-size: 0.8125rem;
    font-weight: 500;
    text-align: left;
  }

  .poodle-table__header,
  .poodle-table__cell,
  .poodle-table__empty {
    padding: 0.5rem 0.75rem;
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
    text-align: left;
    vertical-align: middle;
  }

  .poodle-table__header {
    color: var(--poodle-recipe-table-header-text, var(--poodle-color-text-secondary));
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    background: var(--poodle-recipe-table-header-fill, color-mix(in srgb, var(--poodle-surface) 91%, var(--poodle-color-text-primary)));
  }

  .poodle-table__cell--row-header {
    font-weight: 600;
  }

  .poodle-table__header[data-align="end"],
  .poodle-table__cell[data-align="end"] {
    text-align: right;
  }

  tbody .poodle-table__row:last-child .poodle-table__cell,
  tbody .poodle-table__row:last-child .poodle-table__cell--row-header,
  tbody tr:last-child .poodle-table__empty {
    border-bottom: 0;
  }

  .poodle-table__empty {
    color: var(--poodle-recipe-table-empty-text, var(--poodle-color-text-secondary));
  }

  /* Size variants */
  .poodle-table-shell[data-size="xs"] .poodle-table { font-size: 0.6875rem; }
  .poodle-table-shell[data-size="xs"] .poodle-table__header { font-size: 0.5625rem; }
  .poodle-table-shell[data-size="xs"] .poodle-table__header,
  .poodle-table-shell[data-size="xs"] .poodle-table__cell { padding-block: 0.3125rem; }

  .poodle-table-shell[data-size="sm"] .poodle-table { font-size: 0.75rem; }
  .poodle-table-shell[data-size="sm"] .poodle-table__header { font-size: 0.625rem; }
  .poodle-table-shell[data-size="sm"] .poodle-table__header,
  .poodle-table-shell[data-size="sm"] .poodle-table__cell { padding-block: 0.375rem; }

  .poodle-table-shell[data-size="lg"] .poodle-table { font-size: 0.875rem; }
  .poodle-table-shell[data-size="lg"] .poodle-table__header { font-size: 0.75rem; }
  .poodle-table-shell[data-size="lg"] .poodle-table__header,
  .poodle-table-shell[data-size="lg"] .poodle-table__cell { padding-block: 0.625rem; }

  .poodle-table-shell[data-size="xl"] .poodle-table { font-size: 0.9375rem; }
  .poodle-table-shell[data-size="xl"] .poodle-table__header { font-size: 0.8125rem; }
  .poodle-table-shell[data-size="xl"] .poodle-table__header,
  .poodle-table-shell[data-size="xl"] .poodle-table__cell { padding-block: 0.75rem; }

  /* Density variants */
  .poodle-table-shell[data-density="compact"] .poodle-table__header,
  .poodle-table-shell[data-density="compact"] .poodle-table__cell { padding-inline: 0.5rem; }

  .poodle-table-shell[data-density="comfortable"] .poodle-table__header,
  .poodle-table-shell[data-density="comfortable"] .poodle-table__cell { padding-inline: 1.125rem; }
</style>
