<script lang="ts">
  import { FilterToolbar } from "@pug/svelte-composites";
  import { Eyebrow, Select, SearchField, Button, type SelectOption } from "@pug/svelte-primitives";

  const statusOptions: SelectOption[] = [
    { value: "all", label: "All statuses" },
    { value: "active", label: "Active" },
    { value: "archived", label: "Archived" },
    { value: "draft", label: "Draft" },
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>With filters and summary</Eyebrow>
    <FilterToolbar summaryText="Showing 24 of 156 items" ariaLabel="Item filters">
      <SearchField id="filter-search" placeholder="Search…" ariaLabel="Search items" />
      <Select id="filter-status" options={statusOptions} defaultValue="all" ariaLabel="Status filter" />
    </FilterToolbar>
  </div>

  <div class="specimen__group">
    <Eyebrow>With secondary actions</Eyebrow>
    <FilterToolbar ariaLabel="Project filters">
      <SearchField id="proj-search" placeholder="Filter projects…" ariaLabel="Filter" />
      <svelte:fragment slot="secondary">
        <Button variant="secondary" size="sm">Reset</Button>
      </svelte:fragment>
    </FilterToolbar>
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
</style>
