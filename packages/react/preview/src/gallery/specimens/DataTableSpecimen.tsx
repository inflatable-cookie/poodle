import { useState, type CSSProperties } from "react";
import { DataTable, Pill } from "@inflatable-cookie/poodle-react";
import type { TableColumn, TableFilters, TablePagination, TableRow } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

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

const mutedText: CSSProperties = {
  margin: 0,
  fontSize: "0.8125rem",
  color: "var(--poodle-color-text-secondary)",
};

const expandButton: CSSProperties = {
  border: 0,
  background: "transparent",
  color: "var(--poodle-color-text-secondary)",
  font: "inherit",
  cursor: "pointer",
};

const incidentDetail: CSSProperties = {
  display: "grid",
  gap: "0.25rem",
  fontSize: "0.8125rem",
  color: "var(--poodle-color-text-secondary)",
};

export function DataTableSpecimen() {
  const [selectedRowIds, setSelectedRowIds] = useState<string[]>([]);
  const [sortColumnId, setSortColumnId] = useState<string | null>(null);
  const [sortDirection, setSortDirection] = useState<"asc" | "desc">("asc");
  const [hiddenColumnIds, setHiddenColumnIds] = useState<string[]>([]);
  const [lastAction, setLastAction] = useState("");
  const [expandedIncidentId, setExpandedIncidentId] = useState<string | null>(null);
  const [filters, setFilters] = useState<TableFilters>({ name: "", role: "" });
  const [pagination, setPagination] = useState<TablePagination>({ page: 1, limit: 10, total: 42 });

  const handleSortChange = (payload: { columnId: string; direction: "asc" | "desc" }) => {
    setSortColumnId(payload.columnId);
    setSortDirection(payload.direction);
    setLastAction(`Sorted by ${payload.columnId} ${payload.direction}`);
  };

  const handleRowToggle = (payload: { rowId: string; selected: boolean }) => {
    setSelectedRowIds((prev) =>
      payload.selected ? [...prev, payload.rowId] : prev.filter((id) => id !== payload.rowId),
    );
    setLastAction(`Toggled row ${payload.rowId}: ${payload.selected ? "selected" : "deselected"}`);
  };

  const handleToggleAll = (payload: { selected: boolean }) => {
    setSelectedRowIds(payload.selected ? rows.map((r) => r.id) : []);
    setLastAction(payload.selected ? "Selected all rows" : "Deselected all rows");
  };

  const handleRowAction = (payload: { rowId: string }) => {
    setLastAction(`Action on row ${payload.rowId}`);
  };

  const handleColumnVisibility = (payload: { columnId: string; visible: boolean }) => {
    setHiddenColumnIds((prev) =>
      payload.visible ? prev.filter((id) => id !== payload.columnId) : [...prev, payload.columnId],
    );
    setLastAction(`${payload.visible ? "Showed" : "Hid"} column: ${payload.columnId}`);
  };

  return (
    <SpecimenLayout
      sizes={(size) => <DataTable columns={columns} rows={rows} size={size} ariaLabel={`Data table at ${size}`} />}
      densities={(density) => <DataTable columns={columns} rows={rows} density={density} ariaLabel={`Data table at ${density} density`} />}
    >
      <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
        <SpecimenGroup bare label="With sorting, column visibility, and export">
          <DataTable
            columns={columns}
            rows={rows}
            selectable
            selectedRowIds={selectedRowIds}
            sortColumnId={sortColumnId}
            sortDirection={sortDirection}
            hiddenColumnIds={hiddenColumnIds}
            showColumnVisibility
            showExport
            ariaLabel="Team members"
            onSortChange={handleSortChange}
            onRowToggle={handleRowToggle}
            onToggleAll={handleToggleAll}
            onRowAction={handleRowAction}
            onColumnVisibilityChange={handleColumnVisibility}
          />
          {lastAction ? (
            <p style={mutedText}>Last action: <strong>{lastAction}</strong></p>
          ) : null}
          <p style={mutedText}>{selectedRowIds.length} of {rows.length} selected</p>
        </SpecimenGroup>

        <SpecimenGroup bare label="With filters and pagination">
          <DataTable
            columns={columns}
            rows={rows}
            filters={filters}
            pagination={pagination}
            compact
            striped
            stickyHeader
            ariaLabel="Directory table"
            onFilterChange={({ filters: nextFilters }) => setFilters(nextFilters)}
            onPageChange={({ page }) => setPagination((prev) => ({ ...prev, page }))}
            onLimitChange={({ limit }) => setPagination((prev) => ({ ...prev, page: 1, limit }))}
          />
        </SpecimenGroup>

        <SpecimenGroup bare label="With custom cells and expanded rows">
          <DataTable
            columns={incidentColumns}
            rows={incidentRows}
            showRowActions={false}
            expandedRowIds={expandedIncidentId ? [expandedIncidentId] : []}
            ariaLabel="Active incidents"
            cell={(column, row) => {
              const incident = row.data as Incident | undefined;
              if (column.id === "expand") {
                return (
                  <button
                    type="button"
                    style={expandButton}
                    onClick={() => setExpandedIncidentId((prev) => (prev === row.id ? null : row.id))}
                  >
                    {expandedIncidentId === row.id ? "Hide" : "Show"}
                  </button>
                );
              }
              if (column.id === "status") {
                return (
                  <Pill appearance="badge" tone={incident?.status === "open" ? "danger" : "success"}>
                    {row.cells.status}
                  </Pill>
                );
              }
              return row.cells[column.id];
            }}
            expandedRow={(row) => {
              const incident = row.data as Incident | undefined;
              if (!incident) {
                return null;
              }
              return (
                <div style={incidentDetail}>
                  <strong>{incident.endpoint}</strong>
                  <span>Owned by {incident.owner}</span>
                  <span>Updated {new Date(incident.updatedAt).toLocaleString()}</span>
                </div>
              );
            }}
          />
        </SpecimenGroup>

        <SpecimenGroup bare label="Empty state">
          <DataTable
            columns={columns}
            rows={[]}
            ariaLabel="Empty data table"
            emptyMessage="No team members match the current filters."
          />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
