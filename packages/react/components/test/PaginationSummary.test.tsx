import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { PaginationSummary } from "../src/PaginationSummary";

describe("PaginationSummary (react)", () => {
  it("computes the item range and page count from props", () => {
    const { container } = render(
      <PaginationSummary currentPage={1} totalPages={8} totalItems={156} pageSize={20} />,
    );
    expect(container.querySelector(".poodle-pagination-summary__copy")?.textContent).toBe(
      "Showing 1-20 of 156",
    );
    expect(container.querySelector(".poodle-pagination-summary")?.getAttribute("aria-label")).toBe(
      "Showing 1-20 of 156 across 8 pages",
    );
  });

  it("clamps the last-page range to the item total", () => {
    const { container } = render(
      <PaginationSummary currentPage={8} totalPages={8} totalItems={156} pageSize={20} />,
    );
    expect(container.querySelector(".poodle-pagination-summary__copy")?.textContent).toBe(
      "Showing 141-156 of 156",
    );
  });

  it("shows an empty range for an empty dataset", () => {
    const { container } = render(<PaginationSummary />);
    expect(container.querySelector(".poodle-pagination-summary__copy")?.textContent).toBe(
      "Showing 0-0 of 0",
    );
    expect(container.querySelector(".poodle-pagination-summary")?.getAttribute("aria-label")).toBe(
      "Showing 0-0 of 0 across 1 pages",
    );
  });

  it("announces summary changes through a polite live region", () => {
    const { container } = render(
      <PaginationSummary currentPage={5} totalPages={50} totalItems={1000} pageSize={20} />,
    );
    expect(container.querySelector(".poodle-pagination-summary")?.getAttribute("aria-live")).toBe(
      "polite",
    );
    expect(container.querySelector(".poodle-pagination-summary__copy")?.textContent).toBe(
      "Showing 81-100 of 1000",
    );
  });
});
