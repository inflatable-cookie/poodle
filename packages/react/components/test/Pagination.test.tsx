import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Pagination } from "../src/Pagination";

describe("Pagination (react)", () => {
  it("disables boundary buttons and reports navigation via onPageChange", () => {
    const onPageChange = vi.fn();
    const first = render(
      <Pagination currentPage={1} totalPages={5} ariaLabel="Results pagination" onPageChange={onPageChange} />,
    );
    const prev = first.container.querySelector<HTMLButtonElement>('button[aria-label="Previous page"]')!;
    const next = first.container.querySelector<HTMLButtonElement>('button[aria-label="Next page"]')!;
    expect(prev.disabled).toBe(true);
    expect(next.disabled).toBe(false);

    fireEvent.click(next);
    expect(onPageChange).toHaveBeenCalledWith(2);

    fireEvent.click(next);
    expect(onPageChange).toHaveBeenCalledTimes(2);
    expect(onPageChange).toHaveBeenCalledWith(2);

    const last = render(<Pagination currentPage={5} totalPages={5} onPageChange={onPageChange} />);
    const lastNext = last.container.querySelector<HTMLButtonElement>('button[aria-label="Next page"]')!;
    expect(lastNext.disabled).toBe(true);
  });

  it("clamps out-of-range page props into the valid window", () => {
    const { container } = render(<Pagination currentPage={99} totalPages={5} />);
    const current = container.querySelector('.poodle-pagination__button[data-current="true"]')!;
    expect(current.textContent?.trim()).toBe("5");

    const low = render(<Pagination page={-3} totalPages={5} />);
    const lowCurrent = low.container.querySelector('.poodle-pagination__button[data-current="true"]')!;
    expect(lowCurrent.textContent?.trim()).toBe("1");
  });

  it("marks the current page and truncates distant ranges with ellipsis", () => {
    const { container } = render(
      <Pagination currentPage={5} totalPages={20} siblingCount={1} ariaLabel="Extended pagination" />,
    );
    const current = container.querySelector('button[aria-label="Page 5"]')!;
    expect(current.getAttribute("aria-current")).toBe("page");
    expect(current.getAttribute("data-current")).toBe("true");

    const labels = [...container.querySelectorAll<HTMLButtonElement>(".poodle-pagination__button")]
      .map((button) => button.getAttribute("aria-label"))
      .filter((label) => label?.startsWith("Page "))
      .map((label) => Number(label?.slice(5)));
    expect(labels).toEqual([1, 4, 5, 6, 20]);
    expect(container.querySelectorAll(".poodle-pagination__ellipsis").length).toBe(2);
  });

  it("hides entirely when a single page exists and no limit selector is shown", () => {
    const { container } = render(<Pagination totalPages={1} currentPage={1} />);
    expect(container.querySelector("nav")).toBeNull();

    const withLimit = render(<Pagination totalPages={1} currentPage={1} showLimitSelector />);
    expect(withLimit.container.querySelector("nav")).not.toBeNull();
  });

  it("shows the info row and reports limit changes", () => {
    const onLimitChange = vi.fn();
    const { container } = render(
      <Pagination
        page={3}
        limit={25}
        total={248}
        showLimitSelector
        limitOptions={[10, 25, 50, 100]}
        onLimitChange={onLimitChange}
      />,
    );
    expect(container.querySelector(".poodle-pagination__info")?.textContent).toContain("Showing 51 to 75 of 248");

    const select = container.querySelector<HTMLSelectElement>(".poodle-pagination__limit select")!;
    fireEvent.change(select, { target: { value: "50" } });
    expect(onLimitChange).toHaveBeenCalledWith(50);
  });

  it("routes adjacent moves and jumps through the controller when provided", () => {
    const goToPage = vi.fn();
    const nextPage = vi.fn();
    const prevPage = vi.fn();
    const onPageChange = vi.fn();
    const controller = {
      currentPage: 3,
      pageSize: 20,
      total: 140,
      totalPages: 7,
      showingFrom: 41,
      showingTo: 60,
      hasPrevPage: true,
      hasNextPage: true,
      prevPage,
      nextPage,
      setPageSize: vi.fn(),
      goToPage,
    };
    const { container } = render(
      <Pagination controller={controller} variant="full" ariaLabel="Full pagination" onPageChange={onPageChange} />,
    );
    expect(container.querySelector(".poodle-pagination__summary")?.textContent).toContain("Page 3 of 7");

    fireEvent.click(container.querySelector<HTMLButtonElement>('button[aria-label="Next page"]')!);
    expect(nextPage).toHaveBeenCalledOnce();
    expect(onPageChange).not.toHaveBeenCalled();

    fireEvent.click(container.querySelector<HTMLButtonElement>('button[aria-label="First page"]')!);
    expect(goToPage).toHaveBeenCalledWith(1);

    fireEvent.click(container.querySelector<HTMLButtonElement>('button[aria-label="Last page"]')!);
    expect(goToPage).toHaveBeenCalledWith(7);
  });

  it("falls back to onPageChange for non-adjacent jumps without goToPage", () => {
    const onPageChange = vi.fn();
    const controller = {
      currentPage: 3,
      pageSize: 20,
      total: 100,
      totalPages: 5,
      showingFrom: 41,
      showingTo: 60,
      hasPrevPage: true,
      hasNextPage: true,
      prevPage: vi.fn(),
      nextPage: vi.fn(),
      setPageSize: vi.fn(),
    };
    const { container } = render(<Pagination controller={controller} onPageChange={onPageChange} />);
    const page5 = container.querySelector<HTMLButtonElement>('button[aria-label="Page 5"]')!;
    fireEvent.click(page5);
    expect(onPageChange).toHaveBeenCalledWith(5);
  });

  it("shows the simple item-range summary", () => {
    const { container } = render(<Pagination page={3} limit={25} total={248} variant="simple" />);
    const summary = container.querySelector(".poodle-pagination__summary")?.textContent ?? "";
    expect(summary).toContain("51–75");
    expect(summary).toContain("of 248");
  });

  it("disables page buttons and the limit selector while loading", () => {
    const { container } = render(
      <Pagination
        currentPage={2}
        totalPages={5}
        showLimitSelector
        limitOptions={[10, 25, 50]}
        loading
      />,
    );
    const next = container.querySelector<HTMLButtonElement>('button[aria-label="Next page"]')!;
    const select = container.querySelector<HTMLSelectElement>(".poodle-pagination__limit select")!;
    expect(next.disabled).toBe(true);
    expect(select.disabled).toBe(true);
    expect(container.querySelector(".poodle-pagination--loading")).not.toBeNull();
  });
});
