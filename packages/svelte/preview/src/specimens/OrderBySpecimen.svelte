<script lang="ts">
  import { Eyebrow, OrderBy, type ActiveSort, type SortField } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let activeSort: ActiveSort | null = null;

  const fields: SortField[] = [
    { value: "name", label: "Name" },
    { value: "date", label: "Date" },
    { value: "size", label: "Size" },
    { value: "type", label: "Type", disabled: true },
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Sort controls</Eyebrow>
    <OrderBy
      {fields}
      bind:activeSort
      on:change={(e) => {}}
    />
    {#if activeSort}
      <p>Sorted by: <strong>{activeSort.field}</strong> ({activeSort.direction})</p>
    {:else}
      <p>No active sort</p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <OrderBy fields={fields} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <OrderBy fields={fields} {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <OrderBy
      fields={[{ value: "name", label: "Name" }, { value: "date", label: "Date" }]}
      disabled
    />
  </div>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1.5rem; }
  .specimen__group { display: flex; flex-direction: column; gap: 0.5rem; }
  .specimen__stack { display: flex; flex-direction: column; gap: 0.5rem; }
  .specimen__row { display: flex; align-items: center; gap: 0.5rem; }
  .specimen__label { font-size: 0.75rem; font-family: var(--poodle-typography-code-family); color: var(--poodle-color-text-muted); min-width: 6rem; }
  p { margin: 0; }
</style>
