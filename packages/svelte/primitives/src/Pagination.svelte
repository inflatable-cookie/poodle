<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let currentPage = 1;
  export let totalPages = 1;
  export let siblingCount = 1;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    pageChange: { page: number };
  }>();

  $: safeTotalPages = Math.max(1, totalPages);
  $: safeCurrentPage = Math.min(Math.max(1, currentPage), safeTotalPages);
  $: visiblePages = buildVisiblePages(safeCurrentPage, safeTotalPages, siblingCount);

  function buildVisiblePages(
    page: number,
    count: number,
    siblings: number
  ): Array<number | "ellipsis"> {
    const pages = new Set<number>([1, count]);

    for (let candidate = page - siblings; candidate <= page + siblings; candidate += 1) {
      if (candidate >= 1 && candidate <= count) {
        pages.add(candidate);
      }
    }

    const sorted = Array.from(pages).sort((left, right) => left - right);
    const result: Array<number | "ellipsis"> = [];

    for (let index = 0; index < sorted.length; index += 1) {
      const current = sorted[index];
      const previous = sorted[index - 1];

      if (previous && current - previous > 1) {
        result.push("ellipsis");
      }

      result.push(current);
    }

    return result;
  }

  function requestPage(page: number): void {
    if (page < 1 || page > safeTotalPages || page === safeCurrentPage) {
      return;
    }

    dispatch("pageChange", { page });
  }
</script>

<nav class="pagination" aria-label={ariaLabel ?? "Pagination"}>
  <button
    type="button"
    class="pagination__button"
    disabled={safeCurrentPage <= 1}
    aria-label="Previous page"
    on:click={() => requestPage(safeCurrentPage - 1)}
  >
    Previous
  </button>

  <div class="pagination__pages">
    {#each visiblePages as page, index}
      {#if page === "ellipsis"}
        <span class="pagination__ellipsis" aria-hidden="true">…</span>
      {:else}
        <button
          type="button"
          class="pagination__button"
          data-current={page === safeCurrentPage}
          aria-current={page === safeCurrentPage ? "page" : undefined}
          aria-label={`Page ${page}`}
          on:click={() => requestPage(page)}
        >
          {page}
        </button>
      {/if}
    {/each}
  </div>

  <button
    type="button"
    class="pagination__button"
    disabled={safeCurrentPage >= safeTotalPages}
    aria-label="Next page"
    on:click={() => requestPage(safeCurrentPage + 1)}
  >
    Next
  </button>
</nav>

<style>
  .pagination {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    flex-wrap: wrap;
  }

  .pagination__pages {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
  }

  .pagination__button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 2.25rem;
    height: calc(var(--pug-size-control-height) - 0.125rem);
    padding: 0 0.75rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 78%, transparent);
    border-radius: var(--pug-radius-control);
    background: var(--pug-color-background-surface);
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font-family: var(--pug-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
  }

  .pagination__button[data-current="true"] {
    background: color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent);
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 42%, var(--pug-color-border-default));
  }

  .pagination__button:hover:not(:disabled),
  .pagination__button:focus-visible {
    background: color-mix(in srgb, var(--pug-color-accent-base) 12%, transparent);
    outline: none;
  }

  .pagination__ellipsis {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 1.5rem;
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
  }

  .pagination__button:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }
</style>
