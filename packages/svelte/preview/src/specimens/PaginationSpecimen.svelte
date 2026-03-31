<script lang="ts">
  import { Pagination, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let page1 = 1;
  let page2 = 5;
  let page3 = 2;
  let limit3 = 25;
  let page4 = 1;

  const totalItems = 248;
  const totalPagesForLimit3 = Math.ceil(totalItems / limit3);
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
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Pagination totalPages={10} {density} />
        </div>
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

  <div class="specimen__group">
    <Eyebrow>Simple variant with info and page size</Eyebrow>
    <Pagination
      page={page3}
      limit={limit3}
      total={totalItems}
      variant="simple"
      showLimitSelector
      limitOptions={[10, 25, 50, 100]}
      ariaLabel="Simple pagination"
      on:pageChange={(e) => (page3 = e.detail.page)}
      on:limitChange={(e) => {
        limit3 = e.detail.limit;
        page3 = 1;
      }}
    />
    <p>Page <strong>{page3}</strong> of {totalPagesForLimit3} with <strong>{limit3}</strong> per page</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Full variant</Eyebrow>
    <Pagination
      page={page4}
      limit={20}
      total={140}
      variant="full"
      ariaLabel="Full pagination"
      on:pageChange={(e) => (page4 = e.detail.page)}
    />
    <p>Page <strong>{page4}</strong> of 7</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Standalone (no container chrome)</Eyebrow>
    <Pagination
      currentPage={1}
      totalPages={10}
      standalone
      ariaLabel="Standalone pagination"
    />
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

  .specimen__row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
