import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { ListContainer } from "../src/ListContainer";

describe("ListContainer (react)", () => {
  it("labels the section from ariaLabel, falling back to the title", () => {
    const { container } = render(<ListContainer title="Projects" />);
    const root = container.querySelector(".poodle-list-container") as HTMLElement;
    expect(root.getAttribute("aria-label")).toBe("Projects");
    expect(root.dataset.state).toBe("ready");
  });

  it("renders filters, batch, and content regions in the ready state", () => {
    const { container } = render(
      <ListContainer
        title="Projects"
        filters={<div>filters</div>}
        batch={<div>batch</div>}
      >
        content
      </ListContainer>,
    );
    expect(container.querySelector(".poodle-list-container__filters")).not.toBeNull();
    expect(container.querySelector(".poodle-list-container__batch")).not.toBeNull();
    expect(container.querySelector(".poodle-list-container__content")).not.toBeNull();
  });

  it("replaces content with a pending Callout while loading", () => {
    const { container } = render(
      <ListContainer title="Projects" state="loading" loadingMessage="Fetching..." />,
    );
    expect(container.querySelector(".poodle-list-container__content")).toBeNull();
    expect(container.querySelector(".poodle-list-container__state")?.textContent).toContain(
      "Fetching...",
    );
  });

  it("renders the error Callout with title and message in the error state", () => {
    const { container } = render(
      <ListContainer
        title="Projects"
        state="error"
        errorTitle="Could not load"
        errorMessage="Network error"
      />,
    );
    expect(container.querySelector(".poodle-list-container__state")?.textContent).toContain(
      "Could not load",
    );
    expect(container.querySelector(".poodle-list-container__state")?.textContent).toContain(
      "Network error",
    );
  });

  it("renders the built-in EmptyState in the empty state", () => {
    const { container } = render(
      <ListContainer
        title="Projects"
        state="empty"
        emptyTitle="No projects yet"
        emptyMessage="Create one to start."
      />,
    );
    const state = container.querySelector(".poodle-list-container__state") as HTMLElement;
    expect(state.textContent).toContain("No projects yet");
    expect(state.textContent).toContain("Create one to start.");
  });

  it("renders pagination summary and controls and forwards page changes", () => {
    const onPageChange = vi.fn();
    const { container } = render(
      <ListContainer
        title="Projects"
        currentPage={2}
        totalPages={5}
        totalItems={48}
        pageSize={10}
        onPageChange={onPageChange}
      />,
    );
    expect(container.querySelector(".poodle-list-container__pagination")).not.toBeNull();
    expect(container.querySelector(".poodle-list-container__pagination")?.textContent).toContain(
      "48",
    );

    const next = container.querySelector(
      '.poodle-list-container__pagination [aria-label="Next page"]',
    ) as HTMLButtonElement;
    fireEvent.click(next);
    expect(onPageChange).toHaveBeenCalledWith(3);
  });

  it("hides built-in pagination when only one page exists", () => {
    const { container } = render(
      <ListContainer title="Projects" currentPage={1} totalPages={1} />,
    );
    expect(container.querySelector(".poodle-list-container__pagination")).toBeNull();
  });

  it("renders the pagination snippet instead of built-in controls when provided", () => {
    const { container } = render(
      <ListContainer title="Projects" currentPage={1} totalPages={3} pagination={<div>custom pager</div>} />,
    );
    expect(container.querySelector(".poodle-list-container__pagination")?.textContent).toContain(
      "custom pager",
    );
  });
});
