import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import SplitView from "../src/SplitView.svelte";

describe("SplitView collapse toggles", () => {
  it("keeps both expand toggles when both panes are collapsed", () => {
    const { getByRole } = render(SplitView, {
      props: {
        showCollapsePrimary: true,
        showCollapseSecondary: true,
        primaryCollapsed: true,
        secondaryCollapsed: true,
        primary: () => {},
        secondary: () => {},
      },
    });

    expect(getByRole("button", { name: "Expand primary" })).toBeTruthy();
    expect(getByRole("button", { name: "Expand secondary" })).toBeTruthy();
  });

  it("shows only the collapse affordances of an all-open split", () => {
    const { getByRole, queryByRole } = render(SplitView, {
      props: {
        showCollapsePrimary: true,
        showCollapseSecondary: true,
        primary: () => {},
        secondary: () => {},
      },
    });

    expect(getByRole("button", { name: "Collapse primary" })).toBeTruthy();
    expect(getByRole("button", { name: "Collapse secondary" })).toBeTruthy();
    expect(queryByRole("button", { name: "Expand primary" })).toBeNull();
  });

  it("keeps the collapsed pane's expand toggle when the other pane is open", () => {
    const { getByRole, queryByRole } = render(SplitView, {
      props: {
        showCollapsePrimary: true,
        showCollapseSecondary: true,
        primaryCollapsed: true,
        primary: () => {},
        secondary: () => {},
      },
    });

    expect(getByRole("button", { name: "Expand primary" })).toBeTruthy();
    expect(queryByRole("button", { name: "Expand secondary" })).toBeNull();
  });

  it("a hidden pane is absent without being a collapse", () => {
    const { container } = render(SplitView, {
      props: {
        showCollapsePrimary: true,
        showCollapseSecondary: true,
        primaryHidden: true,
        primary: () => {},
        secondary: () => {},
      },
    });
    const root = container.querySelector(".poodle-split-view")!;

    expect(root.hasAttribute("data-primary-collapsed")).toBe(false);
    const primaryPane = container.querySelector(".poodle-split-view__pane--primary")!;
    expect(primaryPane.getAttribute("style")).toContain("flex-basis: 0px");
  });
});

describe("SplitView toggle visibility", () => {
  function renderSplit(props: Record<string, unknown>) {
    const { container } = render(SplitView, {
      props: {
        showCollapsePrimary: true,
        showCollapseSecondary: true,
        primary: () => {},
        secondary: () => {},
        ...props,
      },
    });
    return container.querySelector(".poodle-split-view") as HTMLElement;
  }

  it("defaults to always-visible toggles", () => {
    expect(renderSplit({}).dataset.toggleVisibility).toBe("always");
  });

  it("marks the root for hover reveal when asked", () => {
    expect(renderSplit({ toggleVisibility: "hover" }).dataset.toggleVisibility).toBe("hover");
  });

  it("still renders the toggles in hover mode — the reveal is presentational", () => {
    // Hover reveal must not unmount the buttons: they stay in the a11y tree
    // and reachable by Tab, which is what :focus-within brings back on screen.
    const { getByRole } = render(SplitView, {
      props: {
        showCollapsePrimary: true,
        showCollapseSecondary: true,
        toggleVisibility: "hover",
        primary: () => {},
        secondary: () => {},
      },
    });

    expect(getByRole("button", { name: "Collapse primary" })).toBeTruthy();
    expect(getByRole("button", { name: "Collapse secondary" })).toBeTruthy();
  });
});
