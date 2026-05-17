<script lang="ts">
  let {
    currentPage = 1,
    totalPages = 1,
    totalItems = 0,
    pageSize = 5,
  }: {
    currentPage?: number;
    totalPages?: number;
    totalItems?: number;
    pageSize?: number;
  } = $props();

  const fromItem = $derived(totalItems === 0 ? 0 : ((currentPage - 1) * pageSize) + 1);
  const toItem = $derived(Math.min(currentPage * pageSize, totalItems));
</script>

<div
  class="poodle-pagination-summary"
  aria-live="polite"
  aria-label={`Showing ${fromItem}-${toItem} of ${totalItems} across ${totalPages} pages`}
>
  <p class="poodle-pagination-summary__copy">
    Showing {fromItem}-{toItem} of {totalItems}
  </p>
</div>

<style>
  .poodle-pagination-summary {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: var(--poodle-space-inline-md);
  }

  .poodle-pagination-summary__copy {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-label-size, 0.75rem);
    line-height: var(--poodle-typography-label-lineHeight, 1.4);
  }

</style>
