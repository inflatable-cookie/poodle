import { useState } from "react";
import {
  Calendar,
  ListGrid,
  Pagination,
  PaginationSummary,
  Region,
  Table,
  type DateRangeValue,
} from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

function PaginationSpecimen() {
  const [pg, setPg] = useState(3);
  return (
    <SpecimenSection title="Pagination">
      <Pagination page={pg} total={240} limit={20} onPageChange={setPg} showLimitSelector />
      <span data-testid="page-value">page: {pg}</span>
      <PaginationSummary currentPage={pg} totalPages={12} totalItems={240} pageSize={20} />
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "pagination", title: "Pagination", render: () => <PaginationSpecimen /> });

registerSpecimen({
  slug: "table",
  title: "Table",
  render: () => (
    <SpecimenSection title="Table">
      <Table
        caption="Team"
        columns={[
          { id: "name", label: "Name", isRowHeader: true },
          { id: "role", label: "Role" },
          { id: "count", label: "Items", align: "end" },
        ]}
        rows={[
          { id: "1", cells: { name: "Ada", role: "Engineer", count: 12 } },
          { id: "2", cells: { name: "Grace", role: "Admiral", count: 3 } },
        ]}
      />
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "list-grid",
  title: "ListGrid",
  render: () => (
    <SpecimenSection title="ListGrid">
      <ListGrid minItemWidth="12rem" maxColumns={3}>
        <Region label="a" />
        <Region label="b" />
        <Region label="c" />
        <Region label="d" />
      </ListGrid>
    </SpecimenSection>
  ),
});

function CalendarSpecimen() {
  const [date, setDate] = useState<string | null>("2026-07-14");
  const [range, setRange] = useState<DateRangeValue>({ start: null, end: null });
  return (
    <SpecimenSection title="Calendar">
      <Calendar value={date} onValueChange={(v) => typeof v === "string" && setDate(v)} />
      <span data-testid="calendar-value">date: {date}</span>
      <Calendar mode="range" value={range} onValueChange={(v) => typeof v === "object" && setRange(v)} />
      <span data-testid="calendar-range">
        range: {range.start ?? "-"}..{range.end ?? "-"}
      </span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "calendar", title: "Calendar", render: () => <CalendarSpecimen /> });
