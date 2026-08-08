<script lang="ts">
  import "@inflatable-cookie/poodle-styles/pagination-summary.css";
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

