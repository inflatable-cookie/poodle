import { readFileSync } from "node:fs";
import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import DockRegion from "../src/DockRegion.svelte";
import type { PanelTabItem } from "../src/types.ts";

// g13-040: the five tab pass-throughs. The Svelte<->React parity gate diffs
// anatomy classes, so it cannot see whether a forwarded prop actually lands —
// only a matching test catches that. DockRegionTabPassThroughs.test.tsx is the
// React mirror.
const items: PanelTabItem[] = [
  { value: "explorer", label: "Explorer", icon: "folder" },
  { value: "search", label: "Search", icon: "search" },
];

function tabsOf(container: HTMLElement): HTMLElement {
  const tabs = container.querySelector<HTMLElement>(".poodle-tabs");
  if (!tabs) throw new Error("no .poodle-tabs rendered");
  return tabs;
}

describe("DockRegion tab pass-throughs (svelte)", () => {
  it("defaults forward Tabs' current effective values", () => {
    const { container } = render(DockRegion, {
      props: { items, value: "explorer" },
    });
    const tabs = tabsOf(container);
    expect(tabs.getAttribute("data-active-edge")).toBe("underline");
    expect(tabs.getAttribute("data-active-fill")).toBe("tint");
    expect(tabs.getAttribute("data-bordered")).toBe("false");
    expect(tabs.hasAttribute("data-full-width")).toBe(false);
    const item = tabs.querySelector(".poodle-tabs__tab")!;
    expect(item.getAttribute("draggable")).toBe("true");
  });

  it("forwards all five props to Tabs", () => {
    const { container } = render(DockRegion, {
      props: {
        items,
        value: "explorer",
        tabActiveEdge: "outline",
        tabActiveFill: "solid",
        tabBordered: true,
        tabFullWidth: true,
        tabReorderable: false,
      },
    });
    const tabs = tabsOf(container);
    expect(tabs.getAttribute("data-active-edge")).toBe("outline");
    expect(tabs.getAttribute("data-active-fill")).toBe("solid");
    expect(tabs.getAttribute("data-bordered")).toBe("true");
    expect(tabs.hasAttribute("data-full-width")).toBe(true);
    const item = tabs.querySelector(".poodle-tabs__tab")!;
    expect(item.getAttribute("draggable")).toBe("false");
  });

  it("tabActiveEdge=none renders no underline; the default keeps it", () => {
    const none = render(DockRegion, {
      props: { items, value: "explorer", tabActiveEdge: "none" },
    });
    expect(tabsOf(none.container).getAttribute("data-active-edge")).toBe("none");

    const underline = render(DockRegion, {
      props: { items, value: "explorer" },
    });
    expect(tabsOf(underline.container).getAttribute("data-active-edge")).toBe("underline");
  });

  it("tabReorderable=false produces non-reorderable tabs; the default stays reorderable", () => {
    const locked = render(DockRegion, {
      props: { items, value: "explorer", tabReorderable: false },
    });
    const lockedItem = tabsOf(locked.container).querySelector(".poodle-tabs__tab")!;
    expect(lockedItem.getAttribute("draggable")).toBe("false");

    const defaulted = render(DockRegion, {
      props: { items, value: "explorer" },
    });
    const defaultItem = tabsOf(defaulted.container).querySelector(".poodle-tabs__tab")!;
    expect(defaultItem.getAttribute("draggable")).toBe("true");
  });

  it("applies at every call site: collapsed vertical and horizontal icon-strips", () => {
    // Vertical-edge collapsed strip — the branch the dock-tabs papercut missed.
    const vertical = render(DockRegion, {
      props: {
        edge: "left",
        items,
        value: "explorer",
        collapsed: true,
        collapsedPosture: "icon-strip",
        tabActiveEdge: "none",
        tabActiveFill: "solid",
        tabBordered: true,
        tabFullWidth: true,
        tabReorderable: false,
      },
    });
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

    // Top/bottom-edge collapsed strip keeps horizontal tabs.
    const horizontal = render(DockRegion, {
      props: {
        edge: "bottom",
        items,
        value: "explorer",
        collapsed: true,
        collapsedPosture: "icon-strip",
        tabActiveEdge: "none",
        tabActiveFill: "solid",
        tabBordered: true,
        tabFullWidth: true,
        tabReorderable: false,
      },
    });
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
    const { container } = render(DockRegion, {
      props: {
        items,
        value: "explorer",
        tabActiveEdge: "none",
        tabActiveFill: "solid",
        tabBordered: true,
        tabFullWidth: true,
        tabReorderable: false,
      },
    });
    const tabs = tabsOf(container);
    // DockRegion never forwards collapseWhenOverflow: no measure shell, and
    // the real tablist renders instead of a collapse menu.
    expect(tabs.querySelector(".poodle-tabs__measure-shell")).toBeNull();
    expect(tabs.querySelector('[role="tablist"]')).not.toBeNull();
    // Labels still render — the five did not force the compact path.
    expect(tabs.querySelector(".poodle-tabs__label")?.textContent).toBe("Explorer");
  });
});

// The underline hook is a CSS declaration; happy-dom cannot resolve the
// var()-of-var fallback chain at computed-value time, so the contract is
// asserted on the stylesheet declaration itself: one hook, both orientations,
// the current value as fallback.
describe("DockRegion underline recipe hook (tabs.css)", () => {
  const tabsCss = readFileSync(
    new URL("../../../core/src/styles/tabs.css", `file://${import.meta.dirname}/`),
    "utf8",
  );
  const hook = "var(--poodle-recipe-tabs-active-underline-border, var(--poodle-color-accent-base))";

  it("wraps the horizontal underline colour in the hook", () => {
    expect(tabsCss).toContain(`border-bottom-color: ${hook}`);
  });

  it("wraps the vertical underline colour in the same hook", () => {
    expect(tabsCss).toContain(`border-right-color: ${hook}`);
  });

  it("leaves the current accent colour as the fallback", () => {
    // The fallback is exactly the pre-hook value, so an unset hook renders
    // byte-identically to today.
    expect(hook).toContain("var(--poodle-color-accent-base)");
  });
});
