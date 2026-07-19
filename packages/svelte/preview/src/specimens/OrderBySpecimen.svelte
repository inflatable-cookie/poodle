<script lang="ts">
  import { OrderBy, type OrderByValue, type SortField } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let value = $state<OrderByValue>([
    { key: "updatedAt", direction: "desc" },
    { key: "title", direction: "asc" },
  ]);

  let sizeValue = $state<OrderByValue>([{ key: "title", direction: "asc" }]);
  let densityValue = $state<OrderByValue>([{ key: "title", direction: "asc" }]);
  let iconValue = $state<OrderByValue>([
    { key: "updatedAt", direction: "desc" },
    { key: "title", direction: "asc" },
  ]);

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

  <SpecimenGroup label="Icon trigger">
    <OrderBy {fields} bind:value={iconValue} triggerVariant="icon" />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <OrderBy {fields} {size} bind:value={sizeValue} />
  {/snippet}

  {#snippet densities(density)}
    <OrderBy {fields} {density} bind:value={densityValue} />
  {/snippet}
</SpecimenLayout>

<style>
  pre { margin: 0; font-size: 0.75rem; }
</style>
