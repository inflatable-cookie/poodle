import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { SplitView } from "../src/SplitView";

// Mirrors packages/svelte/components/test/SplitView.svelte.test.ts — the
// collapse-toggle contract (both-collapsed stays recoverable) behaves
// identically in both frameworks.
describe("SplitView collapse toggles (react)", () => {
  it("keeps both expand toggles when both panes are collapsed", () => {
    const { getByRole } = render(
      <SplitView
        showCollapsePrimary
        showCollapseSecondary
        primaryCollapsed
        secondaryCollapsed
        primary={<div />}
        secondary={<div />}
      />,
    );

    expect(getByRole("button", { name: "Expand primary" })).toBeTruthy();
    expect(getByRole("button", { name: "Expand secondary" })).toBeTruthy();
  });

  it("shows only the collapse affordances of an all-open split", () => {
    const { getByRole, queryByRole } = render(
      <SplitView
        showCollapsePrimary
        showCollapseSecondary
        primary={<div />}
        secondary={<div />}
      />,
    );

    expect(getByRole("button", { name: "Collapse primary" })).toBeTruthy();
    expect(getByRole("button", { name: "Collapse secondary" })).toBeTruthy();
    expect(queryByRole("button", { name: "Expand primary" })).toBeNull();
  });

  it("keeps the collapsed pane's expand toggle when the other pane is open", () => {
    const { getByRole, queryByRole } = render(
      <SplitView
        showCollapsePrimary
        showCollapseSecondary
        primaryCollapsed
        primary={<div />}
        secondary={<div />}
      />,
    );

    expect(getByRole("button", { name: "Expand primary" })).toBeTruthy();
    expect(queryByRole("button", { name: "Expand secondary" })).toBeNull();
  });

  it("a hidden pane is absent without being a collapse", () => {
    const { container } = render(
      <SplitView
        showCollapsePrimary
        showCollapseSecondary
        primaryHidden
        primary={<div />}
        secondary={<div />}
      />,
    );
    const root = container.querySelector(".poodle-split-view")!;

    expect(root.hasAttribute("data-primary-collapsed")).toBe(false);
    const primaryPane = container.querySelector(".poodle-split-view__pane--primary")!;
    expect((primaryPane as HTMLElement).style.flexBasis).toBe("0px");
  });
});

describe("SplitView toggle visibility (react)", () => {
  function renderSplit(props: Record<string, unknown>) {
    const { container } = render(
      <SplitView
        showCollapsePrimary
        showCollapseSecondary
        primary={<div />}
        secondary={<div />}
        {...props}
      />,
    );
    return container.querySelector(".poodle-split-view") as HTMLElement;
  }

  it("defaults to always-visible toggles", () => {
    expect(renderSplit({}).dataset.toggleVisibility).toBe("always");
  });

  it("marks the root for hover reveal when asked", () => {
    expect(renderSplit({ toggleVisibility: "hover" }).dataset.toggleVisibility).toBe("hover");
  });

  it("still renders the toggles in hover mode — the reveal is presentational", () => {
    const { getByRole } = render(
      <SplitView
        showCollapsePrimary
        showCollapseSecondary
        toggleVisibility="hover"
        primary={<div />}
        secondary={<div />}
      />,
    );

    expect(getByRole("button", { name: "Collapse primary" })).toBeTruthy();
    expect(getByRole("button", { name: "Collapse secondary" })).toBeTruthy();
  });

  it("renders data-divider='line' only when divider prop is true", () => {
    expect(renderSplit({ divider: true }).getAttribute("data-divider")).toBe("line");
    expect(renderSplit({ divider: false }).getAttribute("data-divider")).toBeNull();
    expect(renderSplit({}).getAttribute("data-divider")).toBeNull();
  });
});