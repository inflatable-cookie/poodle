<script lang="ts">
  import { DataTable } from "@poodle/svelte-composites";
  import type { TableColumn, TableRow } from "@poodle/svelte-composites";
  import { Eyebrow } from "@poodle/svelte-primitives";

  const columns: TableColumn[] = [
    { id: "name", label: "Name", sortable: true, hideable: false },
    { id: "email", label: "Email", sortable: true, hideable: true },
    { id: "role", label: "Role", sortable: true, hideable: true },
    { id: "status", label: "Status", sortable: false, hideable: true },
  ];

  const rows: TableRow[] = [
    { id: "1", cells: { name: "Alice Chen", email: "alice@example.com", role: "Engineer", status: "Active" }, summary: "Senior frontend engineer" },
    { id: "2", cells: { name: "Bob Martinez", email: "bob@example.com", role: "Designer", status: "Active" }, summary: "UX lead" },
    { id: "3", cells: { name: "Carol Patel", email: "carol@example.com", role: "PM", status: "On leave" }, summary: "Product manager" },
    { id: "4", cells: { name: "Dan Okoro", email: "dan@example.com", role: "Engineer", status: "Active" }, summary: "Backend engineer" },
    { id: "5", cells: { name: "Eve Nakamura", email: "eve@example.com", role: "Designer", status: "Active" }, summary: "Visual designer" },
  ];

  let selectedRowIds: string[] = [];
  let sortColumnId: string | null = null;
  let sortDirection: "asc" | "desc" = "asc";
  let hiddenColumnIds: string[] = [];
  let lastAction = "";

  function handleSortChange(event: CustomEvent<{ columnId: string; direction: "asc" | "desc" }>): void {
    sortColumnId = event.detail.columnId;
    sortDirection = event.detail.direction;
    lastAction = `Sorted by ${event.detail.columnId} ${event.detail.direction}`;
  }

  function handleRowToggle(event: CustomEvent<{ rowId: string; selected: boolean }>): void {
    if (event.detail.selected) {
      selectedRowIds = [...selectedRowIds, event.detail.rowId];
    } else {
      selectedRowIds = selectedRowIds.filter((id) => id !== event.detail.rowId);
    }
    lastAction = `Toggled row ${event.detail.rowId}: ${event.detail.selected ? "selected" : "deselected"}`;
  }

  function handleToggleAll(event: CustomEvent<{ selected: boolean }>): void {
    selectedRowIds = event.detail.selected ? rows.map((r) => r.id) : [];
    lastAction = event.detail.selected ? "Selected all rows" : "Deselected all rows";
  }

  function handleRowAction(event: CustomEvent<{ rowId: string }>): void {
    lastAction = `Action on row ${event.detail.rowId}`;
  }

  function handleColumnVisibility(event: CustomEvent<{ columnId: string; visible: boolean }>): void {
    if (event.detail.visible) {
      hiddenColumnIds = hiddenColumnIds.filter((id) => id !== event.detail.columnId);
    } else {
      hiddenColumnIds = [...hiddenColumnIds, event.detail.columnId];
    }
    lastAction = `${event.detail.visible ? "Showed" : "Hid"} column: ${event.detail.columnId}`;
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>With sorting, column visibility, and export</Eyebrow>
    <DataTable
      {columns}
      {rows}
      {selectedRowIds}
      {sortColumnId}
      {sortDirection}
      {hiddenColumnIds}
      showColumnVisibility
      showExport
      ariaLabel="Team members"
      on:sortChange={handleSortChange}
      on:rowToggle={handleRowToggle}
      on:toggleAll={handleToggleAll}
      on:rowAction={handleRowAction}
      on:columnVisibilityChange={handleColumnVisibility}
    />
    {#if lastAction}
      <p class="last-action">Last action: <strong>{lastAction}</strong></p>
    {/if}
    <p class="selection-count">{selectedRowIds.length} of {rows.length} selected</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Empty state</Eyebrow>
    <DataTable
      {columns}
      rows={[]}
      ariaLabel="Empty data table"
      emptyMessage="No team members match the current filters."
    />
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

  .last-action,
  .selection-count {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
