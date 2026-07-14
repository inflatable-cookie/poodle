import { useMemo, useState } from "react";
import { DataTable, type TableFilters, type TableRow, type TableSortDirection } from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

const ALL_ROWS: TableRow[] = [
  { id: "r1", cells: { name: "Alpha", owner: "ada", size: 120 } },
  { id: "r2", cells: { name: "Beta", owner: "grace", size: 40 }, summary: "archived" },
  { id: "r3", cells: { name: "Gamma", owner: "ada", size: 300 } },
];

function DataTableSpecimen() {
  const [selected, setSelected] = useState<string[]>([]);
  const [sortCol, setSortCol] = useState<string | null>(null);
  const [sortDir, setSortDir] = useState<TableSortDirection>("asc");
  const [filters, setFilters] = useState<TableFilters>({});
  const [lastAction, setLastAction] = useState("");
  const [hidden, setHidden] = useState<string[]>([]);

  const rows = useMemo(() => {
    let out = [...ALL_ROWS];
    const nameFilter = filters.name?.toLowerCase();
    if (nameFilter) out = out.filter((r) => String(r.cells.name).toLowerCase().includes(nameFilter));
    if (sortCol) {
      out.sort((a, b) => {
        const av = String(a.cells[sortCol] ?? "");
        const bv = String(b.cells[sortCol] ?? "");
        return sortDir === "asc" ? av.localeCompare(bv) : bv.localeCompare(av);
      });
    }
    return out;
  }, [filters, sortCol, sortDir]);

  return (
    <SpecimenSection title="DataTable">
      <DataTable
        ariaLabel="Projects"
        columns={[
          { id: "name", label: "Name", sortable: true, filterable: true, isRowHeader: true },
          { id: "owner", label: "Owner", sortable: true },
          { id: "size", label: "Size", align: "end", hideable: true },
        ]}
        rows={rows}
        selectable
        selectedRowIds={selected}
        sortColumnId={sortCol}
        sortDirection={sortDir}
        filters={filters}
        hiddenColumnIds={hidden}
        showColumnVisibility
        rowActions={[
          { value: "open", label: "Open" },
          { value: "delete", label: "Delete", tone: "danger" },
        ]}
        pagination={{ page: 1, limit: 2, total: 3 }}
        onSortChange={({ columnId, direction }) => {
          setSortCol(columnId);
          setSortDir(direction);
        }}
        onRowToggle={({ rowId, selected: isSel }) =>
          setSelected((prev) => (isSel ? [...prev, rowId] : prev.filter((id) => id !== rowId)))
        }
        onToggleAll={({ selected: isSel }) => setSelected(isSel ? rows.map((r) => r.id) : [])}
        onFilterChange={({ filters: next }) => setFilters(next)}
        onRowActionSelect={({ rowId, action }) => setLastAction(`${action.value}:${rowId}`)}
        onColumnVisibilityChange={({ columnId, visible }) =>
          setHidden((prev) => (visible ? prev.filter((id) => id !== columnId) : [...prev, columnId]))
        }
      />
      <span data-testid="dt-selected">sel: {selected.join(",")}</span>
      <span data-testid="dt-sort">sort: {sortCol ?? "-"}:{sortDir}</span>
      <span data-testid="dt-action">act: {lastAction}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "data-table", title: "DataTable", render: () => <DataTableSpecimen /> });
