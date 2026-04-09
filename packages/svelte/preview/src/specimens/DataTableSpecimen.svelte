<script lang="ts">
  import { DataTable } from "@poodle/svelte-composites";
  import type { TableColumn, TableFilters, TablePagination, TableRow } from "@poodle/svelte-composites";
  import { Pill } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

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
  let expandedIncidentId: string | null = null;
  let filters: TableFilters = {
    name: "",
    role: "",
  };
  let pagination: TablePagination = {
    page: 1,
    limit: 10,
    total: 42,
  };

  type Incident = {
    status: "open" | "resolved";
    endpoint: string;
    owner: string;
    updatedAt: string;
  };

  const incidentColumns: TableColumn[] = [
    { id: "expand", label: "", width: "3rem", align: "center", hideable: false, isRowHeader: false },
    { id: "status", label: "Status", width: "7rem" },
    { id: "endpoint", label: "Endpoint", width: "minmax(14rem, 1fr)" },
    { id: "owner", label: "Owner", width: "10rem" },
  ];

  const incidentRows: TableRow<Incident>[] = [
    {
      id: "incident-1",
      cells: { expand: "", status: "Open", endpoint: "POST /api/orders", owner: "Alice" },
      data: { status: "open", endpoint: "POST /api/orders", owner: "Alice", updatedAt: "2026-03-27T11:18:00Z" },
    },
    {
      id: "incident-2",
      cells: { expand: "", status: "Resolved", endpoint: "GET /api/catalog", owner: "Bob" },
      data: { status: "resolved", endpoint: "GET /api/catalog", owner: "Bob", updatedAt: "2026-03-27T09:42:00Z" },
    },
  ];

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
  <SpecimenGroup label="With sorting, column visibility, and export">
    <DataTable
      {columns}
      {rows}
      selectable
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
  </SpecimenGroup>

  <SpecimenGroup label="With filters and pagination">
    <DataTable
      {columns}
      {rows}
      {filters}
      {pagination}
      compact
      striped
      stickyHeader
      ariaLabel="Directory table"
      on:filterChange={(event) => (filters = event.detail.filters)}
      on:pageChange={(event) => (pagination = { ...pagination, page: event.detail.page })}
      on:limitChange={(event) => (pagination = { ...pagination, page: 1, limit: event.detail.limit })}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <DataTable
          {columns}
          {rows}
          {size}
          ariaLabel="Data table at {size}"
        />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With custom cells and expanded rows">
    <DataTable
      columns={incidentColumns}
      rows={incidentRows}
      showRowActions={false}
      expandedRowIds={expandedIncidentId ? [expandedIncidentId] : []}
      ariaLabel="Active incidents"
    >
      <svelte:fragment slot="cell" let:column let:row>
        {@const incident = row.data as Incident | undefined}
        {#if column.id === "expand"}
          <button type="button" class="expand-button" on:click={() => (expandedIncidentId = expandedIncidentId === row.id ? null : row.id)}>
            {expandedIncidentId === row.id ? "Hide" : "Show"}
          </button>
        {:else if column.id === "status"}
          <Pill appearance="badge" tone={incident?.status === "open" ? "danger" : "success"}>
            {row.cells.status}
          </Pill>
        {:else}
          {row.cells[column.id]}
        {/if}
      </svelte:fragment>
      <svelte:fragment slot="expandedRow" let:row>
        {@const incident = row.data as Incident | undefined}
        {#if incident}
          <div class="incident-detail">
            <strong>{incident.endpoint}</strong>
            <span>Owned by {incident.owner}</span>
            <span>Updated {new Date(incident.updatedAt).toLocaleString()}</span>
          </div>
        {/if}
      </svelte:fragment>
    </DataTable>
  </SpecimenGroup>

  <SpecimenGroup label="Empty state">
    <DataTable
      {columns}
      rows={[]}
      ariaLabel="Empty data table"
      emptyMessage="No team members match the current filters."
    />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .last-action,
  .selection-count {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }

  .expand-button {
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-secondary);
    font: inherit;
    cursor: pointer;
  }

  .incident-detail {
    display: grid;
    gap: 0.25rem;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
