import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import FilterToolbar from "../src/FilterToolbar.svelte";
import { asSnippet } from "./snippet";

describe("FilterToolbar (svelte)", () => {
  it("renders a labelled toolbar with the summary and controls", () => {
    const { container } = render(FilterToolbar, {
      props: { ariaLabel: "Project filters", summaryText: "Showing 24 of 156", children: asSnippet(() => "<input placeholder='search'>") },
    });
    const root = container.querySelector(".poodle-filter-toolbar") as HTMLElement;
    expect(root.getAttribute("role")).toBe("toolbar");
    expect(root.getAttribute("aria-label")).toBe("Project filters");
    expect(root.querySelector(".poodle-filter-toolbar__summary")?.textContent).toContain(
      "Showing 24 of 156",
    );
    expect(root.querySelector(".poodle-filter-toolbar__controls")).not.toBeNull();
  });

  it("hides the controls when collapsed and toggles via the collapse toggle", async () => {
    const { container } = render(FilterToolbar, {
      props: { collapsed: true, children: asSnippet(() => "<input>") },
    });
    const root = container.querySelector(".poodle-filter-toolbar") as HTMLElement;
    expect(root.dataset.collapsed).toBe("true");
    expect(root.querySelector(".poodle-filter-toolbar__controls")).toBeNull();

    const toggle = container.querySelector(".poodle-collapse-toggle") as HTMLButtonElement;
    expect(toggle.getAttribute("aria-expanded")).toBe("false");
    await fireEvent.click(toggle);
    expect(root.querySelector(".poodle-filter-toolbar__controls")).not.toBeNull();
  });

  it("renders no collapse toggle when not collapsible", () => {
    const { container } = render(FilterToolbar, {
      props: { collapsible: false, children: asSnippet(() => "<input>") },
    });
    expect(container.querySelector(".poodle-collapse-toggle")).toBeNull();
  });

  it("renders the actions and secondary regions when snippets are provided", () => {
    const { container } = render(FilterToolbar, {
      props: {
        children: asSnippet(() => "<input>"),
        actions: asSnippet(() => "<button>refresh</button>"),
        secondary: asSnippet(() => "<button>Reset all</button>"),
      },
    });
    expect(container.querySelector(".poodle-filter-toolbar__actions")).not.toBeNull();
    expect(container.querySelector(".poodle-filter-toolbar__secondary")).not.toBeNull();
  });

  it("projects the sticky flag", () => {
    const { container } = render(FilterToolbar, {
      props: { sticky: true, children: asSnippet(() => "<input>") },
    });
    expect(container.querySelector(".poodle-filter-toolbar")?.getAttribute("data-sticky")).toBe(
      "true",
    );
  });
});