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
});
