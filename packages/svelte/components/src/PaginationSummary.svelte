<script lang="ts">
  export let currentPage = 1;
  export let totalPages = 1;
  export let totalItems = 0;
  export let pageSize = 5;

  $: fromItem = totalItems === 0 ? 0 : ((currentPage - 1) * pageSize) + 1;
  $: toItem = Math.min(currentPage * pageSize, totalItems);
</script>

<div
  class="pagination-summary"
  aria-live="polite"
  aria-label={`Showing ${fromItem}-${toItem} of ${totalItems} across ${totalPages} pages`}
>
  <p class="pagination-summary__copy">
    Showing {fromItem}-{toItem} of {totalItems}
  </p>
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
    font-size: var(--poodle-typography-label-size, 0.75rem);
    line-height: var(--poodle-typography-label-lineHeight, 1.4);
  }

</style>
