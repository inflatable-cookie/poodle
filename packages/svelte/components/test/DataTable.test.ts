import { fireEvent, render } from "@testing-library/svelte";
import { beforeEach, describe, expect, it, vi } from "vitest";

import DataTable from "../src/DataTable.svelte";
import type { TableColumn, TableRow } from "../src/types";

const columns: TableColumn[] = [
  { id: "name", label: "Name", sortable: true },
  { id: "role", label: "Role", sortable: true },
  { id: "status", label: "Status" },
];

const rows: TableRow[] = [
  { id: "r1", cells: { name: "Ada", role: "Admin", status: "Active" } },
  { id: "r2", cells: { name: "Lin", role: "Editor", status: "Idle" } },
];

let lastBlob: Blob | null = null;

beforeEach(() => {
  lastBlob = null;
  URL.createObjectURL = ((blob: Blob) => {
    lastBlob = blob;
    return "blob:mock";
  }) as typeof URL.createObjectURL;
  URL.revokeObjectURL = () => {};
});

describe("DataTable (svelte)", () => {
  it("renders rows and reports the next sort direction from the controlled state", async () => {
    const onSortChange = vi.fn();
    const { container } = render(DataTable, { props: { columns, rows, onSortChange } });
    expect(container.querySelector("tbody")?.textContent).toContain("Ada");

    const sortButton = container.querySelector(".poodle-data-table__sort") as HTMLButtonElement;
    await fireEvent.click(sortButton);
    expect(onSortChange).toHaveBeenCalledWith({ columnId: "name", direction: "asc" });

    const descending = render(DataTable, {
      props: { columns, rows, sortColumnId: "name", sortDirection: "asc", onSortChange },
    });
    const activeSortButton = descending.container.querySelector(
      ".poodle-data-table__sort",
    ) as HTMLButtonElement;
    await fireEvent.click(activeSortButton);
    expect(onSortChange).toHaveBeenCalledWith({ columnId: "name", direction: "desc" });
  });

  it("projects aria-sort on the active sort column", () => {
    const { container } = render(DataTable, {
      props: { columns, rows, sortColumnId: "role", sortDirection: "desc" },
    });
    const headers = [...container.querySelectorAll("th")];
    const nameHeader = headers.find((th) => th.textContent?.includes("Name")) as HTMLElement;
    const roleHeader = headers.find((th) => th.textContent?.includes("Role")) as HTMLElement;
    expect(nameHeader.getAttribute("aria-sort")).toBe("none");
    expect(roleHeader.getAttribute("aria-sort")).toBe("descending");
  });

  it("reports row toggles and select-all from the selection column", async () => {
    const onRowToggle = vi.fn();
    const onToggleAll = vi.fn();
    const { container } = render(DataTable, {
      props: { columns, rows, selectable: true, onRowToggle, onToggleAll },
    });
    const rowCheckbox = container.querySelector(
      'input[aria-label="Select row Ada"]',
    ) as HTMLInputElement;
    expect(rowCheckbox).not.toBeNull();
    await fireEvent.click(rowCheckbox);
    expect(onRowToggle).toHaveBeenCalledWith({ rowId: "r1", selected: true });

    const selectAll = container.querySelector(
      'input[aria-label="Select all visible rows"]',
    ) as HTMLInputElement;
    await fireEvent.click(selectAll);
    expect(onToggleAll).toHaveBeenCalledWith({ selected: true });
  });

  it("marks selected rows with aria-selected", () => {
    const { container } = render(DataTable, {
      props: { columns, rows, selectable: true, selectedRowIds: ["r2"] },
    });
    const rowElements = [...container.querySelectorAll("tbody tr")];
    expect(rowElements[1].getAttribute("aria-selected")).toBe("true");
  });

  it("shows the empty message when no rows are present", () => {
    const { container } = render(DataTable, {
      props: { columns, rows: [], emptyMessage: "No members match" },
    });
    expect(container.querySelector(".poodle-data-table__empty")?.textContent).toContain(
      "No members match",
    );
  });

  it("renders skeleton rows while loading with no rows", () => {
    const { container } = render(DataTable, {
      props: { columns, rows: [], loading: true, loadingRows: 3 },
    });
    expect(container.querySelectorAll("tbody tr.poodle-data-table__loading-row").length).toBe(3);
  });

  it("renders the pagination footer and reports page changes", async () => {
    const onPageChange = vi.fn();
    const { container } = render(DataTable, {
      props: {
        columns,
        rows,
        pagination: { page: 2, limit: 10, total: 25 },
        onPageChange,
      },
    });
    expect(container.querySelector(".poodle-data-table__pagination-summary")?.textContent).toContain(
      "Showing 11 to 20 of 25",
    );

    const next = [...container.querySelectorAll("button")].find(
      (button) => button.textContent?.trim() === "Next",
    ) as HTMLButtonElement;
    await fireEvent.click(next);
    expect(onPageChange).toHaveBeenCalledWith({ page: 3 });

    const last = [...container.querySelectorAll("button")].find(
      (button) => button.textContent?.trim() === "Last",
    ) as HTMLButtonElement;
    await fireEvent.click(last);
    expect(onPageChange).toHaveBeenCalledWith({ page: 3 });
  });

  it("exports CSV through the toolbar and reports the filename", async () => {
    const onExportCsv = vi.fn();
    const { container } = render(DataTable, {
      props: { columns, rows, showExport: true, exportFilename: "team.csv", onExportCsv },
    });
    const exportButton = container.querySelector(
      'button[aria-label="Export as CSV"]',
    ) as HTMLButtonElement;
    await fireEvent.click(exportButton);

    expect(onExportCsv).toHaveBeenCalledWith({ filename: "team.csv" });
    expect(await lastBlob?.text()).toContain("Name,Role,Status");
    expect(await lastBlob?.text()).toContain("Ada,Admin,Active");
  });

  it("excludes hidden columns from rendering and from the CSV export", async () => {
    const onExportCsv = vi.fn();
    const { container } = render(DataTable, {
      props: {
        columns,
        rows,
        hiddenColumnIds: ["role"],
        showExport: true,
        onExportCsv,
      },
    });
    const headers = [...container.querySelectorAll("thead th")].map(
      (th) => th.textContent,
    );
    expect(headers.some((text) => text?.includes("Role"))).toBe(false);

    const exportButton = container.querySelector(
      'button[aria-label="Export as CSV"]',
    ) as HTMLButtonElement;
    await fireEvent.click(exportButton);
    const csv = await lastBlob?.text();
    expect(csv).toContain("Name,Status");
    expect(csv).not.toContain("Role");
  });

  it("renders the filter row for filterable columns", () => {
    const filterable = [
      ...columns,
      { id: "team", label: "Team", filterable: true, filterType: "text" as const },
    ];
    const { container } = render(DataTable, { props: { columns: filterable, rows } });
    const filterInput = container.querySelector(
      'input[aria-label="Filter Team"]',
    ) as HTMLInputElement;
    expect(filterInput).not.toBeNull();
  });
});
