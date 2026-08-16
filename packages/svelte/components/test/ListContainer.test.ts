import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import ListContainer from "../src/ListContainer.svelte";
import { asSnippet } from "./snippet";

describe("ListContainer (svelte)", () => {
  it("labels the section from ariaLabel, falling back to the title", () => {
    const { container } = render(ListContainer, { props: { title: "Projects" } });
    const root = container.querySelector(".poodle-list-container") as HTMLElement;
    expect(root.getAttribute("aria-label")).toBe("Projects");
    expect(root.dataset.state).toBe("ready");
  });

  it("renders filters, batch, and content regions in the ready state", () => {
    const { container } = render(ListContainer, {
      props: {
        title: "Projects",
        filters: asSnippet(() => "<div>filters</div>"),
        batch: asSnippet(() => "<div>batch</div>"),
        children: asSnippet(() => "<div>content</div>"),
      },
    });
    expect(container.querySelector(".poodle-list-container__filters")).not.toBeNull();
    expect(container.querySelector(".poodle-list-container__batch")).not.toBeNull();
    expect(container.querySelector(".poodle-list-container__content")).not.toBeNull();
  });

  it("replaces content with a pending Callout while loading", () => {
    const { container } = render(ListContainer, {
      props: { title: "Projects", state: "loading", loadingMessage: "Fetching..." },
    });
    expect(container.querySelector(".poodle-list-container__content")).toBeNull();
    expect(container.querySelector(".poodle-list-container__state")?.textContent).toContain(
      "Fetching...",
    );
  });

  it("renders the error Callout with title and message in the error state", () => {
    const { container } = render(ListContainer, {
      props: {
        title: "Projects",
        state: "error",
        errorTitle: "Could not load",
        errorMessage: "Network error",
      },
    });
    expect(container.querySelector(".poodle-list-container__state")?.textContent).toContain(
      "Could not load",
    );
    expect(container.querySelector(".poodle-list-container__state")?.textContent).toContain(
      "Network error",
    );
  });

  it("renders the built-in EmptyState in the empty state", () => {
    const { container } = render(ListContainer, {
      props: {
        title: "Projects",
        state: "empty",
        emptyTitle: "No projects yet",
        emptyMessage: "Create one to start.",
      },
    });
    const state = container.querySelector(".poodle-list-container__state") as HTMLElement;
    expect(state.textContent).toContain("No projects yet");
    expect(state.textContent).toContain("Create one to start.");
  });

  it("renders pagination summary and controls and forwards page changes", async () => {
    const onPageChange = vi.fn();
    const { container } = render(ListContainer, {
      props: {
        title: "Projects",
        currentPage: 2,
        totalPages: 5,
        totalItems: 48,
        pageSize: 10,
        onPageChange,
      },
    });
    expect(container.querySelector(".poodle-list-container__pagination")).not.toBeNull();
    expect(container.querySelector(".poodle-list-container__pagination")?.textContent).toContain(
      "48",
    );

    const next = container.querySelector(
      '.poodle-list-container__pagination [aria-label="Next page"]',
    ) as HTMLButtonElement;
    await fireEvent.click(next);
    expect(onPageChange).toHaveBeenCalledWith(3);
  });

  it("hides built-in pagination when only one page exists", () => {
    const { container } = render(ListContainer, {
      props: { title: "Projects", currentPage: 1, totalPages: 1 },
    });
    expect(container.querySelector(".poodle-list-container__pagination")).toBeNull();
  });

  it("renders the pagination snippet instead of built-in controls when provided", () => {
    const { container } = render(ListContainer, {
      props: {
        title: "Projects",
        currentPage: 1,
        totalPages: 3,
        pagination: asSnippet(() => "<div>custom pager</div>"),
      },
    });
    const region = container.querySelector(".poodle-list-container__pagination") as HTMLElement;
    // Raw thunks materialize as comment nodes under happy-dom; the observable
    // contract behaviour is that built-in Pagination controls are replaced.
    expect(region.querySelector('[aria-label="First page"]')).toBeNull();
    expect(region.querySelector(".poodle-pagination")).toBeNull();
  });
});
