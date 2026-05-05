<script lang="ts">
  import { FilterToolbar } from "@poodle/svelte";
  import { Select, TextInput, Button, IconButton, type SelectOption } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  const statusOptions: SelectOption[] = [
    { value: "all", label: "All statuses" },
    { value: "active", label: "Active" },
    { value: "archived", label: "Archived" },
    { value: "draft", label: "Draft" },
  ];

  const typeOptions: SelectOption[] = [
    { value: "all", label: "All types" },
    { value: "document", label: "Document" },
    { value: "spreadsheet", label: "Spreadsheet" },
    { value: "presentation", label: "Presentation" },
  ];

  const ownerOptions: SelectOption[] = [
    { value: "all", label: "All owners" },
    { value: "me", label: "Me" },
    { value: "team", label: "My team" },
  ];

  let collapsed1 = true;
  let collapsed2 = true;
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Responsive grid layout">
    <FilterToolbar summaryText="Showing 24 of 156 items" ariaLabel="Item filters">
      <TextInput id="filter-search" type="search" placeholder="Search…" ariaLabel="Search items" />
      <Select id="filter-status" options={statusOptions} defaultValue="all" ariaLabel="Status" />
      <Select id="filter-type" options={typeOptions} defaultValue="all" ariaLabel="Type" />
      <Select id="filter-owner" options={ownerOptions} defaultValue="all" ariaLabel="Owner" />
    </FilterToolbar>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="poodle-specimen__stack">
      {#each controlSizes as size}
        <FilterToolbar summaryText="Toolbar at {size}" {size} ariaLabel="Filter toolbar at {size}">
          <TextInput id="size-search-{size}" type="search" placeholder="Search…" ariaLabel="Search" />
          <Select id="size-status-{size}" options={statusOptions} defaultValue="all" ariaLabel="Status" />
        </FilterToolbar>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Collapsible with actions">
    <FilterToolbar
      collapsible
      bind:collapsed={collapsed1}
      summaryText="Showing 24 of 156 items"
      ariaLabel="Collapsible filters"
    >
      <svelte:fragment slot="actions">
        <IconButton icon="refresh-cw" sizeRole="chrome" ariaLabel="Refresh" />
      </svelte:fragment>
      <TextInput id="col-search" type="search" placeholder="Search…" ariaLabel="Search" />
      <Select id="col-status" options={statusOptions} defaultValue="all" ariaLabel="Status" />
      <Select id="col-type" options={typeOptions} defaultValue="all" ariaLabel="Type" />
    </FilterToolbar>
  </SpecimenGroup>

  <SpecimenGroup label="Explicit collapsed state">
    <FilterToolbar
      collapsible
      bind:collapsed={collapsed2}
      summaryText="3 filters active"
      ariaLabel="Collapsed filters"
    >
      <svelte:fragment slot="actions">
        <IconButton icon="refresh-cw" sizeRole="chrome" ariaLabel="Refresh" />
      </svelte:fragment>
      <TextInput id="col2-search" type="search" placeholder="Search…" ariaLabel="Search" />
      <Select id="col2-status" options={statusOptions} defaultValue="active" ariaLabel="Status" />
    </FilterToolbar>
  </SpecimenGroup>

  <SpecimenGroup label="With secondary slot">
    <FilterToolbar ariaLabel="Project filters" columns={3}>
      <TextInput id="proj-search" type="search" placeholder="Filter projects…" ariaLabel="Filter" />
      <Select id="proj-status" options={statusOptions} defaultValue="all" ariaLabel="Status" />
      <Select id="proj-type" options={typeOptions} defaultValue="all" ariaLabel="Type" />
      <svelte:fragment slot="secondary">
        <Button variant="secondary" sizeRole="chrome">Reset all</Button>
      </svelte:fragment>
    </FilterToolbar>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
