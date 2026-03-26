<script lang="ts">
  import { FilterToolbar } from "@poodle/svelte-composites";
  import { Eyebrow, Select, SearchField, Button, IconButton, type SelectOption } from "@poodle/svelte-primitives";

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

  let collapsed1 = false;
  let collapsed2 = true;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Responsive grid layout</Eyebrow>
    <FilterToolbar summaryText="Showing 24 of 156 items" ariaLabel="Item filters">
      <SearchField id="filter-search" placeholder="Search…" ariaLabel="Search items" />
      <Select id="filter-status" options={statusOptions} defaultValue="all" ariaLabel="Status" />
      <Select id="filter-type" options={typeOptions} defaultValue="all" ariaLabel="Type" />
      <Select id="filter-owner" options={ownerOptions} defaultValue="all" ariaLabel="Owner" />
    </FilterToolbar>
  </div>

  <div class="specimen__group">
    <Eyebrow>Collapsible with actions</Eyebrow>
    <FilterToolbar
      collapsible
      bind:collapsed={collapsed1}
      summaryText="Showing 24 of 156 items"
      ariaLabel="Collapsible filters"
    >
      <svelte:fragment slot="actions">
        <IconButton icon="refresh-cw" sizeRole="chrome" ariaLabel="Refresh" />
      </svelte:fragment>
      <SearchField id="col-search" placeholder="Search…" ariaLabel="Search" />
      <Select id="col-status" options={statusOptions} defaultValue="all" ariaLabel="Status" />
      <Select id="col-type" options={typeOptions} defaultValue="all" ariaLabel="Type" />
    </FilterToolbar>
  </div>

  <div class="specimen__group">
    <Eyebrow>Collapsed by default</Eyebrow>
    <FilterToolbar
      collapsible
      bind:collapsed={collapsed2}
      summaryText="3 filters active"
      ariaLabel="Collapsed filters"
    >
      <svelte:fragment slot="actions">
        <IconButton icon="refresh-cw" sizeRole="chrome" ariaLabel="Refresh" />
      </svelte:fragment>
      <SearchField id="col2-search" placeholder="Search…" ariaLabel="Search" />
      <Select id="col2-status" options={statusOptions} defaultValue="active" ariaLabel="Status" />
    </FilterToolbar>
  </div>

  <div class="specimen__group">
    <Eyebrow>With secondary slot</Eyebrow>
    <FilterToolbar ariaLabel="Project filters" columns={3}>
      <SearchField id="proj-search" placeholder="Filter projects…" ariaLabel="Filter" />
      <Select id="proj-status" options={statusOptions} defaultValue="all" ariaLabel="Status" />
      <Select id="proj-type" options={typeOptions} defaultValue="all" ariaLabel="Type" />
      <svelte:fragment slot="secondary">
        <Button variant="secondary" sizeRole="chrome">Reset all</Button>
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
