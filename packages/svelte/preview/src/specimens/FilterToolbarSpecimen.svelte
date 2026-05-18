<script lang="ts">
  import { FilterToolbar } from "@poodle/svelte";
  import { Select, TextInput, Button, IconButton, type SelectOption } from "@poodle/svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

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

<SpecimenLayout bareVariants>
  <div class="poodle-specimen">
    <SpecimenGroup label="Responsive grid layout">
      <FilterToolbar summaryText="Showing 24 of 156 items" ariaLabel="Item filters">
        <TextInput id="filter-search" type="search" placeholder="Search…" ariaLabel="Search items" />
        <Select id="filter-status" options={statusOptions} defaultValue="all" ariaLabel="Status" />
        <Select id="filter-type" options={typeOptions} defaultValue="all" ariaLabel="Type" />
        <Select id="filter-owner" options={ownerOptions} defaultValue="all" ariaLabel="Owner" />
      </FilterToolbar>
    </SpecimenGroup>

    <SpecimenGroup label="Collapsible with actions">
      <FilterToolbar
        collapsible
        bind:collapsed={collapsed1}
        summaryText="Showing 24 of 156 items"
        ariaLabel="Collapsible filters"
      >
        {#snippet actions()}
          <IconButton icon="refresh-cw" sizeRole="chrome" ariaLabel="Refresh" />
        {/snippet}
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
        {#snippet actions()}
          <IconButton icon="refresh-cw" sizeRole="chrome" ariaLabel="Refresh" />
        {/snippet}
        <TextInput id="col2-search" type="search" placeholder="Search…" ariaLabel="Search" />
        <Select id="col2-status" options={statusOptions} defaultValue="active" ariaLabel="Status" />
      </FilterToolbar>
    </SpecimenGroup>

    <SpecimenGroup label="With secondary slot">
      <FilterToolbar ariaLabel="Project filters" columns={3}>
        <TextInput id="proj-search" type="search" placeholder="Filter projects…" ariaLabel="Filter" />
        <Select id="proj-status" options={statusOptions} defaultValue="all" ariaLabel="Status" />
        <Select id="proj-type" options={typeOptions} defaultValue="all" ariaLabel="Type" />
        {#snippet secondary()}
          <Button variant="secondary" sizeRole="chrome">Reset all</Button>
        {/snippet}
      </FilterToolbar>
    </SpecimenGroup>
  </div>

  {#snippet sizes(size)}
    <div class="poodle-specimen__variant">
      <FilterToolbar summaryText="Toolbar at {size}" {size} ariaLabel="Filter toolbar at {size}">
        <TextInput id="size-search-{size}" type="search" placeholder="Filter projects…" ariaLabel="Search" />
        <Select id="size-status-{size}" options={statusOptions} defaultValue="all" ariaLabel="Status" />
        <Select id="size-type-{size}" options={typeOptions} defaultValue="all" ariaLabel="Type" />
      </FilterToolbar>
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__variant">
      <FilterToolbar
        summaryText="Toolbar at {density}"
        {density}
        ariaLabel="Filter toolbar at {density}"
      >
        <TextInput id="density-search-{density}" type="search" placeholder="Filter projects…" ariaLabel="Search" />
        <Select id="density-status-{density}" options={statusOptions} defaultValue="all" ariaLabel="Status" />
        <Select id="density-type-{density}" options={typeOptions} defaultValue="all" ariaLabel="Type" />
      </FilterToolbar>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__variant {
    width: min(100%, 64rem);
  }
</style>
