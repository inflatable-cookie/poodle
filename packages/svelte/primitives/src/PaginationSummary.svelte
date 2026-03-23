<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let currentPage = 1;
  export let totalPages = 1;
  export let totalItems = 0;
  export let pageSize = 5;

  const dispatch = createEventDispatcher<{
    pageChange: { page: number };
  }>();

  $: fromItem = totalItems === 0 ? 0 : ((currentPage - 1) * pageSize) + 1;
  $: toItem = Math.min(currentPage * pageSize, totalItems);

  function requestPage(page: number): void {
    if (page < 1 || page > totalPages || page === currentPage) {
      return;
    }

    dispatch("pageChange", { page });
  }
</script>

<div class="pagination-summary" role="group" aria-label="Pagination">
  <p class="pagination-summary__copy">
    Showing {fromItem}-{toItem} of {totalItems}
  </p>
  <div class="pagination-summary__actions">
    <button type="button" on:click={() => requestPage(currentPage - 1)} disabled={currentPage <= 1}>
      Previous
    </button>
    <span>Page {currentPage} of {totalPages}</span>
    <button
      type="button"
      on:click={() => requestPage(currentPage + 1)}
      disabled={currentPage >= totalPages}
    >
      Next
    </button>
  </div>
</div>

<style>
  .pagination-summary {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: var(--poodle-space-inline-md);
  }

  .pagination-summary__copy {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .pagination-summary__actions {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
  }

  .pagination-summary__actions button {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
  }

  .pagination-summary__actions button:disabled {
    opacity: var(--poodle-state-opacity-disabled);
    cursor: not-allowed;
  }

  .pagination-summary__actions button:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>
