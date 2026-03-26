<script lang="ts">
  import { Pagination, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let page1 = 1;
  let page2 = 5;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <Pagination
      currentPage={page1}
      totalPages={10}
      ariaLabel="Results pagination"
      on:pageChange={(e) => (page1 = e.detail.page)}
    />
    <p>Page <strong>{page1}</strong> of 10</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <Pagination currentPage={1} totalPages={10} ariaLabel={size + " pagination"} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Middle of range</Eyebrow>
    <Pagination
      currentPage={page2}
      totalPages={20}
      siblingCount={2}
      ariaLabel="Extended pagination"
      on:pageChange={(e) => (page2 = e.detail.page)}
    />
    <p>Page <strong>{page2}</strong> of 20</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Few pages</Eyebrow>
    <Pagination currentPage={2} totalPages={3} ariaLabel="Short pagination" />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
