<script lang="ts">
  import { Eyebrow, OrderBy, type ActiveSort, type SortField } from "@poodle/svelte-primitives";

  let activeSort: ActiveSort | null = null;

  const fields: SortField[] = [
    { value: "name", label: "Name" },
    { value: "date", label: "Date" },
    { value: "size", label: "Size" },
    { value: "type", label: "Type", isDisabled: true },
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
    <Eyebrow>Disabled</Eyebrow>
    <OrderBy
      fields={[{ value: "name", label: "Name" }, { value: "date", label: "Date" }]}
      isDisabled
    />
  </div>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1.5rem; }
  .specimen__group { display: flex; flex-direction: column; gap: 0.5rem; }
  p { margin: 0; }
</style>
