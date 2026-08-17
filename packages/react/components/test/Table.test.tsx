import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Table } from "../src/Table";

const columns = [
  { id: "name", label: "Name", isRowHeader: true },
  { id: "role", label: "Role" },
  { id: "hours", label: "Hours", align: "end" as const },
];

const rows = [
  { id: "1", cells: { name: "Ada", role: "Engineer", hours: "40" } },
  { id: "2", cells: { name: "Grace", role: "Analyst", hours: "36" } },
];

describe("Table (react)", () => {
  it("renders column headers with col scope and native table semantics", () => {
    const { container } = render(<Table columns={columns} rows={rows} />);
    expect(container.querySelector("table.poodle-table")).not.toBeNull();
    const headers = Array.from(container.querySelectorAll("thead th"));
    expect(headers.map((th) => th.getAttribute("scope"))).toEqual(["col", "col", "col"]);
    expect(headers.map((th) => th.textContent)).toEqual(["Name", "Role", "Hours"]);
  });

  it("marks the row-header column cell as a row-scoped th", () => {
    const { container } = render(<Table columns={columns} rows={rows} />);
    const rowHeaderCells = container.querySelectorAll("tbody th[scope='row']");
    expect(rowHeaderCells.length).toBe(2);
    expect(rowHeaderCells[0].textContent).toBe("Ada");
    expect(container.querySelectorAll("tbody td").length).toBe(4);
  });

  it("renders a visible caption and projects end alignment", () => {
    const { container } = render(<Table columns={columns} rows={rows} caption="Q1 allocation" />);
    expect(container.querySelector(".poodle-table__caption")?.textContent).toBe("Q1 allocation");
    const hoursHeader = container.querySelector("thead th[data-align='end']");
    expect(hoursHeader).not.toBeNull();
  });

  it("shows the empty-state row spanning all columns when rows is empty", () => {
    const { container } = render(
      <Table columns={columns} rows={[]} emptyMessage="No team members found." />,
    );
    const empty = container.querySelector(".poodle-table__empty") as HTMLElement;
    expect(empty.textContent).toBe("No team members found.");
    expect(empty.getAttribute("colspan")).toBe("3");
    expect(container.querySelectorAll("tbody tr").length).toBe(1);
  });

  it("applies the accessible name to the table element when no caption exists", () => {
    const { container } = render(<Table columns={columns} rows={[]} ariaLabel="Team roster" />);
    const table = container.querySelector("table.poodle-table") as HTMLElement;
    expect(table.getAttribute("aria-label")).toBe("Team roster");
    expect(container.querySelector(".poodle-table-shell")?.getAttribute("aria-label")).toBeNull();
  });

  it("lets a visible caption outrank ariaLabel as the accessible name", () => {
    const { container } = render(
      <Table columns={columns} rows={rows} caption="Q3 revenue" ariaLabel="Team roster" />,
    );
    const table = container.querySelector("table.poodle-table") as HTMLElement;
    expect(container.querySelector(".poodle-table__caption")?.textContent).toBe("Q3 revenue");
    expect(table.getAttribute("aria-label")).toBeNull();
  });

  it("projects size and density data attributes on the shell", () => {
    const { container } = render(<Table columns={columns} rows={rows} size="lg" density="compact" />);
    const shell = container.querySelector(".poodle-table-shell") as HTMLElement;
    expect(shell.dataset.size).toBe("lg");
    expect(shell.dataset.density).toBe("compact");
  });
});