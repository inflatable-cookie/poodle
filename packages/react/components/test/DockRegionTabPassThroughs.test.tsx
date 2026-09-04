import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { DockRegion } from "../src";
import type { PanelTabItem } from "../src/types";

// Mirror of DockRegionTabPassThroughs.svelte.test.ts (g13-040). The
// Svelte<->React parity gate diffs anatomy classes only, so a prop that lands
// in one runtime and not the other passes it silently — a matching test is the
// only thing that catches that.
const items: PanelTabItem[] = [
  { value: "explorer", label: "Explorer", icon: "folder" },
  { value: "search", label: "Search", icon: "search" },
];

function tabsOf(container: HTMLElement): HTMLElement {
  const tabs = container.querySelector<HTMLElement>(".poodle-tabs");
  if (!tabs) throw new Error("no .poodle-tabs rendered");
  return tabs;
}

describe("DockRegion tab pass-throughs (react)", () => {
  it("defaults forward Tabs' current effective values", () => {
    const { container } = render(<DockRegion items={items} value="explorer" />);
    const tabs = tabsOf(container);
    expect(tabs.getAttribute("data-active-edge")).toBe("underline");
    expect(tabs.getAttribute("data-active-fill")).toBe("tint");
    expect(tabs.getAttribute("data-bordered")).toBe("false");
    expect(tabs.hasAttribute("data-full-width")).toBe(false);
    const item = tabs.querySelector(".poodle-tabs__item")!;
    expect(item.getAttribute("data-reorderable")).toBe("true");
  });

  it("forwards all five props to Tabs", () => {
    const { container } = render(
      <DockRegion
        items={items}
        value="explorer"
        tabActiveEdge="outline"
        tabActiveFill="solid"
        tabBordered
        tabFullWidth
        tabReorderable={false}
      />,
    );
    const tabs = tabsOf(container);
    expect(tabs.getAttribute("data-active-edge")).toBe("outline");
    expect(tabs.getAttribute("data-active-fill")).toBe("solid");
    expect(tabs.getAttribute("data-bordered")).toBe("true");
    expect(tabs.hasAttribute("data-full-width")).toBe(true);
    const item = tabs.querySelector(".poodle-tabs__item")!;
    expect(item.getAttribute("data-reorderable")).toBeNull();
  });

  it("tabActiveEdge=none renders no underline; the default keeps it", () => {
    const none = render(<DockRegion items={items} value="explorer" tabActiveEdge="none" />);
    expect(tabsOf(none.container).getAttribute("data-active-edge")).toBe("none");

    const underline = render(<DockRegion items={items} value="explorer" />);
    expect(tabsOf(underline.container).getAttribute("data-active-edge")).toBe("underline");
  });

  it("tabReorderable=false produces non-reorderable tabs; the default stays reorderable", () => {
    const locked = render(<DockRegion items={items} value="explorer" tabReorderable={false} />);
    const lockedItem = tabsOf(locked.container).querySelector(".poodle-tabs__item")!;
    expect(lockedItem.getAttribute("data-reorderable")).toBeNull();

    const defaulted = render(<DockRegion items={items} value="explorer" />);
    const defaultItem = tabsOf(defaulted.container).querySelector(".poodle-tabs__item")!;
    expect(defaultItem.getAttribute("data-reorderable")).toBe("true");
  });

  it("applies at every call site: collapsed vertical and horizontal icon-strips", () => {
    const vertical = render(
      <DockRegion
        edge="left"
        items={items}
        value="explorer"
        collapsed
        collapsedPosture="icon-strip"
        tabActiveEdge="none"
        tabActiveFill="solid"
        tabBordered
        tabFullWidth
        tabReorderable={false}
      />,
    );
    const vStrip = vertical.container.querySelector<HTMLElement>(
      '.poodle-dock-region__strip[data-orientation="vertical"]',
    )!;
    expect(vStrip).not.toBeNull();
    const vTabs = vStrip.querySelector(".poodle-tabs")!;
    expect(vTabs.getAttribute("data-active-edge")).toBe("none");
    expect(vTabs.getAttribute("data-active-fill")).toBe("solid");
    expect(vTabs.getAttribute("data-bordered")).toBe("true");
    expect(vTabs.hasAttribute("data-full-width")).toBe(true);
    expect(vTabs.querySelector(".poodle-tabs__tab")!.getAttribute("draggable")).toBe("false");

    const horizontal = render(
      <DockRegion
        edge="bottom"
        items={items}
        value="explorer"
        collapsed
        collapsedPosture="icon-strip"
        tabActiveEdge="none"
        tabActiveFill="solid"
        tabBordered
        tabFullWidth
        tabReorderable={false}
      />,
    );
    const hStrip = horizontal.container.querySelector<HTMLElement>(
      '.poodle-dock-region__strip[data-orientation="horizontal"]',
    )!;
    expect(hStrip).not.toBeNull();
    const hTabs = hStrip.querySelector(".poodle-tabs")!;
    expect(hTabs.getAttribute("data-active-edge")).toBe("none");
    expect(hTabs.getAttribute("data-active-fill")).toBe("solid");
    expect(hTabs.getAttribute("data-bordered")).toBe("true");
    expect(hTabs.hasAttribute("data-full-width")).toBe(true);
    expect(hTabs.querySelector(".poodle-tabs__tab")!.getAttribute("draggable")).toBe("false");
  });

  it("does not disturb the tab strip or turn on collapseWhenOverflow (R1a)", () => {
    const { container } = render(
      <DockRegion
        items={items}
        value="explorer"
        tabActiveEdge="none"
        tabActiveFill="solid"
        tabBordered
        tabFullWidth
        tabReorderable={false}
      />,
    );
    const tabs = tabsOf(container);
    expect(tabs.querySelector(".poodle-tabs__measure-shell")).toBeNull();
    expect(tabs.querySelector('[role="tablist"]')).not.toBeNull();
    expect(tabs.querySelector(".poodle-tabs__label")?.textContent).toBe("Explorer");
  });
});

describe("DockRegion showCollapseToggle (react)", () => {
  it("renders collapse toggle by default when collapsible is true", () => {
    const { getByRole } = render(
      <DockRegion items={items} value="explorer" collapsible />,
    );
    expect(getByRole("button", { name: "Collapse left dock" })).toBeTruthy();
  });

  it("suppresses collapse toggle when showCollapseToggle is false in expanded mode", () => {
    const { queryByRole } = render(
      <DockRegion items={items} value="explorer" collapsible showCollapseToggle={false} />,
    );
    expect(queryByRole("button", { name: "Collapse left dock" })).toBeNull();
  });

  it("suppresses collapse toggle when showCollapseToggle is false in collapsed icon-strip mode", () => {
    const { queryByRole } = render(
      <DockRegion
        items={items}
        value="explorer"
        collapsible
        collapsed
        collapsedPosture="icon-strip"
        showCollapseToggle={false}
      />,
    );
    expect(queryByRole("button", { name: "Expand left dock" })).toBeNull();
  });

  it("suppresses collapse toggle when showCollapseToggle is false in collapsed hidden mode", () => {
    const { queryByRole } = render(
      <DockRegion
        items={items}
        value="explorer"
        collapsible
        collapsed
        collapsedPosture="hidden"
        showCollapseToggle={false}
      />,
    );
    expect(queryByRole("button", { name: "Expand left dock" })).toBeNull();
  });
});
