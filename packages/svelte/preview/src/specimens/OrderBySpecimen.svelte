<script lang="ts">
  import { OrderBy, type OrderByValue, type SortField, type ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

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

<div class="specimen">
  <SpecimenGroup label="Multi-field sort builder">
    <OrderBy {fields} bind:value compact />
    <pre>{JSON.stringify(value, null, 2)}</pre>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <OrderBy {fields} bind:value={sizeValue} {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <OrderBy {fields} bind:value={densityValue} {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <OrderBy {fields} value={[{ key: "title", direction: "asc" }]} disabled />
  </SpecimenGroup>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1rem; }
  .specimen__stack { display: flex; flex-direction: column; gap: 0.5rem; }
  .specimen__row { display: flex; align-items: center; gap: 0.5rem; }
  .specimen__label { font-size: 0.75rem; font-family: var(--poodle-typography-code-family); color: var(--poodle-color-text-muted); min-width: 6rem; }
  pre { margin: 0; font-size: 0.75rem; }
</style>
