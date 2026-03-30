<script lang="ts">
  import { Eyebrow, OrderBy, type OrderByValue, type SortField } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let value: OrderByValue = [
    { key: "updatedAt", direction: "desc" },
    { key: "title", direction: "asc" },
  ];

  const fields: SortField[] = [
    { key: "title", label: "Title" },
    { key: "kind", label: "Kind" },
    { key: "updatedAt", label: "Updated", defaultDirection: "desc" },
    { key: "createdAt", label: "Created", defaultDirection: "desc" },
    { key: "visibility", label: "Visibility", disabled: true },
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Multi-field sort builder</Eyebrow>
    <OrderBy {fields} bind:value compact />
    <pre>{JSON.stringify(value, null, 2)}</pre>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <OrderBy fields={fields} value={[{ key: "title", direction: "asc" }]} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <OrderBy fields={fields} value={[{ key: "title", direction: "asc" }]} {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <OrderBy fields={fields} value={[{ key: "title", direction: "asc" }]} disabled />
  </div>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1.5rem; }
  .specimen__group { display: flex; flex-direction: column; gap: 0.5rem; }
  .specimen__stack { display: flex; flex-direction: column; gap: 0.5rem; }
  .specimen__row { display: flex; align-items: center; gap: 0.5rem; }
  .specimen__label { font-size: 0.75rem; font-family: var(--poodle-typography-code-family); color: var(--poodle-color-text-muted); min-width: 6rem; }
  pre { margin: 0; font-size: 0.75rem; }
</style>
