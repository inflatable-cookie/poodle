<script lang="ts">
  import { Table, Eyebrow, type TableColumn, type TableRow } from "@poodle/svelte-primitives";

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

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Standard table</Eyebrow>
    <Table {columns} {rows} ariaLabel="Team members" />
  </div>

  <div class="specimen__group">
    <Eyebrow>With caption</Eyebrow>
    <Table {columns} {rows} caption="Q1 team allocation" ariaLabel="Team allocation table" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Minimal key-value</Eyebrow>
    <Table columns={minimalColumns} rows={minimalRows} ariaLabel="Package info" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Empty state</Eyebrow>
    <Table {columns} rows={[]} ariaLabel="Empty table" emptyMessage="No team members found." />
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
