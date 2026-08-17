<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/table.css";
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
  // `ariaLabel` is the fallback name, not an override: a visible <caption>
  // already names the table, and aria-label would silently outrank the text
  // the reader can see.
  const resolvedAriaLabel = $derived(caption ? undefined : (ariaLabel ?? undefined));
</script>

<div class="poodle-table-shell" data-size={resolvedSize} data-density={resolvedDensity}>
  <table class="poodle-table" aria-label={resolvedAriaLabel}>
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

