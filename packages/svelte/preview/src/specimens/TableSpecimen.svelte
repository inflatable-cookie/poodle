<script lang="ts">
  import { Table, type TableColumn, type TableRow } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const columns: TableColumn[] = [
    { id: "name", label: "Name", isRowHeader: true },
    { id: "role", label: "Role" },
    { id: "status", label: "Status" },
    { id: "hours", label: "Hours", align: "end" },
  ];

  const rows: TableRow[] = [
    { id: "1", cells: { name: "Alice Chen", role: "Engineer", status: "Active", hours: "40" } },
    { id: "2", cells: { name: "Bob Martinez", role: "Designer", status: "Active", hours: "36" } },
    { id: "3", cells: { name: "Carol Patel", role: "PM", status: "On leave", hours: "0" } },
    { id: "4", cells: { name: "Dan Okoro", role: "Engineer", status: "Active", hours: "42" } },
  ];

  const minimalColumns: TableColumn[] = [
    { id: "key", label: "Property", isRowHeader: true },
    { id: "value", label: "Value" },
  ];

  const minimalRows: TableRow[] = [
    { id: "1", cells: { key: "Version", value: "2.4.1" } },
    { id: "2", cells: { key: "License", value: "MIT" } },
    { id: "3", cells: { key: "Bundle size", value: "12.3 kB" } },
  ];
</script>

<SpecimenLayout bareVariants>
  <SpecimenGroup bare label="Standard table">
    <Table {columns} {rows} ariaLabel="Team members" />
  </SpecimenGroup>

  <SpecimenGroup bare label="With caption">
    <Table {columns} {rows} caption="Q1 team allocation" ariaLabel="Team allocation table" />
  </SpecimenGroup>

  <SpecimenGroup bare label="Minimal key-value">
    <Table columns={minimalColumns} rows={minimalRows} ariaLabel="Package info" />
  </SpecimenGroup>

  <SpecimenGroup bare label="Empty state">
    <Table {columns} rows={[]} ariaLabel="Empty table" emptyMessage="No team members found." />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <div class="poodle-variant-block">
      <Table columns={minimalColumns} rows={minimalRows} ariaLabel="Table at {size}" {size} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-variant-block">
      <Table columns={minimalColumns} rows={minimalRows} ariaLabel="Table at {density}" {density} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-variant-block {
    width: 100%;
  }
</style>
