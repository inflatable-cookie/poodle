<script lang="ts">
  import { Pagination } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let page1 = 1;
  let page2 = 5;
  let page3 = 2;
  let limit3 = 25;
  let page4 = 1;

  const totalItems = 248;
  const totalPagesForLimit3 = Math.ceil(totalItems / limit3);
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <Pagination
      currentPage={page1}
      totalPages={10}
      ariaLabel="Results pagination"
      on:pageChange={(e) => (page1 = e.detail.page)}
    />
    <p>Page <strong>{page1}</strong> of 10</p>
  </SpecimenGroup>

  <SpecimenGroup label="Middle of range">
    <Pagination
      currentPage={page2}
      totalPages={20}
      siblingCount={2}
      ariaLabel="Extended pagination"
      on:pageChange={(e) => (page2 = e.detail.page)}
    />
    <p>Page <strong>{page2}</strong> of 20</p>
  </SpecimenGroup>

  <SpecimenGroup label="Few pages">
    <Pagination currentPage={2} totalPages={3} ariaLabel="Short pagination" />
  </SpecimenGroup>

  <SpecimenGroup label="Simple variant with info and page size">
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
  </SpecimenGroup>

  <SpecimenGroup label="Full variant">
    <Pagination
      page={page4}
      limit={20}
      total={140}
      variant="full"
      ariaLabel="Full pagination"
      on:pageChange={(e) => (page4 = e.detail.page)}
    />
    <p>Page <strong>{page4}</strong> of 7</p>
  </SpecimenGroup>

  <SpecimenGroup label="Standalone (no container chrome)">
    <Pagination
      currentPage={1}
      totalPages={10}
      standalone
      ariaLabel="Standalone pagination"
    />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <Pagination currentPage={1} totalPages={10} {size} ariaLabel={size + " pagination"} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Pagination totalPages={10} {density} />
  </svelte:fragment>
</SpecimenLayout>
