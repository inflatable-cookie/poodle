import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { FilterToolbar } from "../src/FilterToolbar";

describe("FilterToolbar (react)", () => {
  it("renders a labelled toolbar with the summary and controls", () => {
    const { container } = render(
      <FilterToolbar ariaLabel="Project filters" summaryText="Showing 24 of 156">
        <input placeholder="search" />
      </FilterToolbar>,
    );
    const root = container.querySelector(".poodle-filter-toolbar") as HTMLElement;
    expect(root.getAttribute("role")).toBe("toolbar");
    expect(root.getAttribute("aria-label")).toBe("Project filters");
    expect(root.querySelector(".poodle-filter-toolbar__summary")?.textContent).toContain(
      "Showing 24 of 156",
    );
    expect(root.querySelector(".poodle-filter-toolbar__controls")).not.toBeNull();
  });

  it("hides the controls when collapsed and toggles via the collapse toggle", () => {
    const { container } = render(
      <FilterToolbar defaultCollapsed>
        <input />
      </FilterToolbar>,
    );
    const root = container.querySelector(".poodle-filter-toolbar") as HTMLElement;
    expect(root.dataset.collapsed).toBe("true");
    expect(root.querySelector(".poodle-filter-toolbar__controls")).toBeNull();

    const toggle = container.querySelector(".poodle-collapse-toggle") as HTMLButtonElement;
    expect(toggle.getAttribute("aria-expanded")).toBe("false");
    fireEvent.click(toggle);
    expect(root.querySelector(".poodle-filter-toolbar__controls")).not.toBeNull();
  });

  it("renders no collapse toggle when not collapsible", () => {
    const { container } = render(
      <FilterToolbar collapsible={false}>
        <input />
      </FilterToolbar>,
    );
    expect(container.querySelector(".poodle-collapse-toggle")).toBeNull();
  });

  it("renders the actions and secondary regions when provided", () => {
    const { container } = render(
      <FilterToolbar actions={<button>refresh</button>} secondary={<button>Reset all</button>}>
        <input />
      </FilterToolbar>,
    );
    expect(container.querySelector(".poodle-filter-toolbar__actions")).not.toBeNull();
    expect(container.querySelector(".poodle-filter-toolbar__secondary")).not.toBeNull();
  });

  it("projects the sticky flag", () => {
    const { container } = render(
      <FilterToolbar sticky>
        <input />
      </FilterToolbar>,
    );
    expect(container.querySelector(".poodle-filter-toolbar")?.getAttribute("data-sticky")).toBe(
      "true",
    );
  });
});