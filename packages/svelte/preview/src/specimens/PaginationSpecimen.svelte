<script lang="ts">
  import { Pagination } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let page1 = $state(1);
  let page2 = $state(5);
  let page3 = $state(2);
  let limit3 = $state(25);
  let page4 = $state(1);

  const totalItems = 248;
  const totalPagesForLimit3 = $derived(Math.ceil(totalItems / limit3));
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <Pagination
      currentPage={page1}
      totalPages={10}
      ariaLabel="Results pagination"
      onPageChange={(page) => (page1 = page)}
    />
    <p>Page <strong>{page1}</strong> of 10</p>
  </SpecimenGroup>

  <SpecimenGroup label="Middle of range">
    <Pagination
      currentPage={page2}
      totalPages={20}
      siblingCount={2}
      ariaLabel="Extended pagination"
      onPageChange={(page) => (page2 = page)}
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
      onPageChange={(page) => (page3 = page)}
      onLimitChange={(limit) => {
        limit3 = limit;
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
      onPageChange={(page) => (page4 = page)}
    />
    <p>Page <strong>{page4}</strong> of 7</p>
  </SpecimenGroup>

  <SpecimenGroup label="With container chrome">
    <Pagination
      currentPage={1}
      totalPages={10}
      chrome
      ariaLabel="Pagination with chrome"
    />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <Pagination currentPage={1} totalPages={10} {size} ariaLabel={size + " pagination"} />
  {/snippet}

  {#snippet densities(density)}
    <Pagination totalPages={10} {density} />
  {/snippet}
</SpecimenLayout>
