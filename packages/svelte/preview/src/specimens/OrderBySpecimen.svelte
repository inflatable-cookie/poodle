<script lang="ts">
  import { OrderBy, type OrderByValue, type SortField } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let value: OrderByValue = [
    { key: "updatedAt", direction: "desc" },
    { key: "title", direction: "asc" },
  ];

  let sizeValue: OrderByValue = [{ key: "title", direction: "asc" }];
  let densityValue: OrderByValue = [{ key: "title", direction: "asc" }];

  const fields: SortField[] = [
    { key: "title", label: "Title" },
    { key: "kind", label: "Kind" },
    { key: "updatedAt", label: "Updated", defaultDirection: "desc" },
    { key: "createdAt", label: "Created", defaultDirection: "desc" },
    { key: "visibility", label: "Visibility", disabled: true },
  ];
</script>

<SpecimenLayout>
  <SpecimenGroup label="Multi-field sort builder">
    <OrderBy {fields} bind:value compact />
    <pre>{JSON.stringify(value, null, 2)}</pre>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <OrderBy {fields} value={[{ key: "title", direction: "asc" }]} disabled />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <OrderBy {fields} {size} bind:value={sizeValue} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <OrderBy {fields} {density} bind:value={densityValue} />
  </svelte:fragment>
</SpecimenLayout>

<style>
  pre { margin: 0; font-size: 0.75rem; }
</style>
