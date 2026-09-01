/**
 * DockRegion panel movement on the shared drag substrate (g16.026).
 *
 * The old `dockPanelDragSession` module global let any two regions in a
 * document find each other. It is gone, and what replaces it is ordinary
 * controller scope: a panel moves between regions when one controller holds
 * both registrations, and not otherwise. These cases pin both halves of that,
 * because "no hidden local bus" is only a real claim if the self-provided pair
 * is proved not to cross-drop.
 */

import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import DockRegion from "../src/DockRegion.svelte";
import DockRegionZoneDropHarness from "./DockRegionZoneDropHarness.svelte";
import type { PanelTabItem } from "../src/types.ts";

const items: PanelTabItem[] = [
  { value: "explorer", label: "Explorer" },
  { value: "inspector", label: "Inspector" },
];

/**
 * Lay the two stacks out side by side.
 *
 * happy-dom measures everything as an empty box at the origin, so without this
 * every target contains every point and "dropped on region B" would mean
 * nothing.
 */
function layout(container: HTMLElement): void {
  const regions = [...container.querySelectorAll<HTMLElement>("section")];
  regions.forEach((region, regionIndex) => {
    const originX = regionIndex * 400;
    box(region, originX, 0, 400, 100);
    [...region.querySelectorAll<HTMLElement>(".poodle-dock-region__stack-item")].forEach(
      (item, index) => {
        box(item, originX + index * 100, 20, 100, 60);
      },
    );
  });
}

function layoutColumn(container: HTMLElement): void {
  [...container.querySelectorAll<HTMLElement>("section")].forEach((region, regionIndex) => {
    const originX = regionIndex * 400;
    box(region, originX, 0, 120, 200);
    [...region.querySelectorAll<HTMLElement>(".poodle-dock-region__stack-item")].forEach(
      (item, index) => {
        box(item, originX + 20, index * 80, 80, 80);
      },
    );
  });
}

function box(element: HTMLElement, x: number, y: number, width: number, height: number): void {
  const rect = {
    x,
    y,
    width,
    height,
    top: y,
    left: x,
    right: x + width,
    bottom: y + height,
    toJSON() {
      return this;
    },
  } as DOMRect;
  element.getBoundingClientRect = () => rect;
  element.setPointerCapture = vi.fn();
  element.releasePointerCapture = vi.fn();
  element.hasPointerCapture = () => false;
}

function pointer(type: string, x: number, y: number): PointerEvent {
  return new PointerEvent(type, {
    bubbles: true,
    cancelable: true,
    pointerId: 1,
    pointerType: "mouse",
    button: 0,
    buttons: type === "pointerup" ? 0 : 1,
    isPrimary: true,
    clientX: x,
    clientY: y,
  });
}

function stackItems(container: HTMLElement): HTMLElement[] {
  return [...container.querySelectorAll<HTMLElement>(".poodle-dock-region__stack-item")];
}

function layoutTabs(container: HTMLElement): void {
  [...container.querySelectorAll<HTMLElement>("section")].forEach((region, regionIndex) => {
    const originX = regionIndex * 400;
    box(region, originX, 0, 400, 100);
    [...region.querySelectorAll<HTMLElement>(".poodle-tabs__item")].forEach((item, index) => {
      box(item, originX + index * 100, 0, 100, 30);
      const tab = item.querySelector<HTMLElement>(".poodle-tabs__tab");
      if (tab) box(tab, originX + index * 100, 0, 100, 30);
    });
  });
}

describe("DockRegion panel movement", () => {
  beforeEach(() => {
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("cross-drops between two sibling regions under one provider", async () => {
    const onPanelDropA = vi.fn();
    const onPanelDropB = vi.fn();
    const { container } = render(DockRegionZoneDropHarness, {
      props: { shared: true, items, onPanelDropA, onPanelDropB },
    });
    layout(container);

    // Region A's first panel, dropped into region B.
    const [sourceItem] = stackItems(container);
    await fireEvent(sourceItem, pointer("pointerdown", 50, 50));
    await fireEvent(document, pointer("pointermove", 90, 50));
    await fireEvent(document, pointer("pointermove", 420, 50));
    await fireEvent(document, pointer("pointerup", 420, 50));

    expect(onPanelDropB).toHaveBeenCalledOnce();
    expect(onPanelDropB.mock.calls[0][0].panel).toEqual({
      panelId: "explorer",
      sourceEdge: "top",
      sourceZone: "region:a",
    });
    expect(onPanelDropB.mock.calls[0][0].targetEdge).toBe("top");
    expect(onPanelDropB.mock.calls[0][0].index).toBe(0);
    expect(onPanelDropA).not.toHaveBeenCalled();
  });

  it("a flexible strip drop lands at the hovered tab, not the end", async () => {
    const onPanelDropB = vi.fn();
    const { container } = render(DockRegionZoneDropHarness, {
      props: {
        shared: true,
        sizing: "flexible",
        items,
        itemsB: [{ value: "outline", label: "Outline" }],
        onPanelDropB,
      },
    });
    layoutTabs(container);

    const source = container.querySelectorAll<HTMLElement>('[role="tab"]')[0]!;
    await fireEvent(source, pointer("pointerdown", 50, 15));
    await fireEvent(document, pointer("pointermove", 90, 15));
    await fireEvent(document, pointer("pointermove", 420, 15));
    await fireEvent(document, pointer("pointerup", 420, 15));

    expect(onPanelDropB).toHaveBeenCalledOnce();
    expect(onPanelDropB.mock.calls[0][0].panel.panelId).toBe("explorer");
    expect(onPanelDropB.mock.calls[0][0].index).toBe(0);
  });

  it("refuses a panel on a hovered flexible tab when canAcceptPanel says no", async () => {
    const onPanelDropB = vi.fn();
    const { container } = render(DockRegionZoneDropHarness, {
      props: {
        shared: true,
        sizing: "flexible",
        items,
        itemsB: [{ value: "outline", label: "Outline" }],
        canAcceptPanel: (panelId: string) => panelId !== "explorer",
        onPanelDropB,
      },
    });
    layoutTabs(container);

    const source = container.querySelectorAll<HTMLElement>('[role="tab"]')[0]!;
    await fireEvent(source, pointer("pointerdown", 50, 15));
    await fireEvent(document, pointer("pointermove", 90, 15));
    await fireEvent(document, pointer("pointermove", 420, 15));
    expect(container.querySelectorAll("[data-drop-target]")).toHaveLength(0);
    await fireEvent(document, pointer("pointerup", 420, 15));
    expect(onPanelDropB).not.toHaveBeenCalled();
  });

  it("inserts before or after a static stack item from the hovered half", async () => {
    const onPanelDropB = vi.fn();
    const { container } = render(DockRegionZoneDropHarness, {
      props: { shared: true, items, onPanelDropB },
    });
    layout(container);
    expect(container.querySelector(".poodle-dock-region__stack")?.getAttribute("data-direction")).toBe(
      "row",
    );

    const [sourceItem] = stackItems(container);
    await fireEvent(sourceItem, pointer("pointerdown", 50, 50));
    await fireEvent(document, pointer("pointermove", 90, 50));
    await fireEvent(document, pointer("pointermove", 490, 50));
    await fireEvent(document, pointer("pointerup", 490, 50));
    expect(onPanelDropB).toHaveBeenCalledOnce();
    expect(onPanelDropB.mock.calls[0][0].index).toBe(1);

    onPanelDropB.mockClear();
    await fireEvent(sourceItem, pointer("pointerdown", 50, 50));
    await fireEvent(document, pointer("pointermove", 90, 50));
    await fireEvent(document, pointer("pointermove", 410, 50));
    await fireEvent(document, pointer("pointerup", 410, 50));
    expect(onPanelDropB.mock.calls[0][0].index).toBe(0);
  });

  it("inserts before or after a side-edge static stack item from the hovered Y half", async () => {
    const onPanelDropB = vi.fn();
    const { container } = render(DockRegionZoneDropHarness, {
      props: { shared: true, items, edge: "left", onPanelDropB },
    });
    layoutColumn(container);
    expect(container.querySelector(".poodle-dock-region__stack")?.getAttribute("data-direction")).toBe(
      "column",
    );

    const [sourceItem] = stackItems(container);
    await fireEvent(sourceItem, pointer("pointerdown", 50, 40));
    await fireEvent(document, pointer("pointermove", 90, 40));
    await fireEvent(document, pointer("pointermove", 460, 100));
    await fireEvent(document, pointer("pointerup", 460, 100));
    expect(onPanelDropB).toHaveBeenCalledOnce();
    expect(onPanelDropB.mock.calls[0][0].index).toBe(1);

    onPanelDropB.mockClear();
    await fireEvent(sourceItem, pointer("pointerdown", 50, 40));
    await fireEvent(document, pointer("pointermove", 90, 40));
    await fireEvent(document, pointer("pointermove", 460, 20));
    await fireEvent(document, pointer("pointerup", 460, 20));
    expect(onPanelDropB.mock.calls[0][0].index).toBe(0);
  });

  it("keeps local reorder but discovers no sibling when each region provides itself", async () => {
    const onPanelDropA = vi.fn();
    const onPanelDropB = vi.fn();
    const onReorderA = vi.fn();
    const { container } = render(DockRegionZoneDropHarness, {
      props: { shared: false, items, onPanelDropA, onPanelDropB, onReorderA },
    });
    layout(container);

    const [first] = stackItems(container);
    await fireEvent(first, pointer("pointerdown", 50, 50));
    await fireEvent(document, pointer("pointermove", 90, 50));
    await fireEvent(document, pointer("pointermove", 450, 50));

    // Region B is not in this controller's registry, so it is not a candidate
    // and nothing in it lights up.
    expect(container.querySelectorAll("[data-drop-target]")).toHaveLength(0);

    await fireEvent(document, pointer("pointerup", 450, 50));
    expect(onPanelDropA).not.toHaveBeenCalled();
    expect(onPanelDropB).not.toHaveBeenCalled();

    // Its own stack still reorders.
    await fireEvent(first, pointer("pointerdown", 50, 50));
    await fireEvent(document, pointer("pointermove", 90, 50));
    await fireEvent(document, pointer("pointermove", 150, 50));
    await fireEvent(document, pointer("pointerup", 150, 50));
    expect(onReorderA).toHaveBeenCalledWith(["inspector", "explorer"]);
  });

  it("reorders within one region rather than reporting a transfer", async () => {
    const onPanelDropA = vi.fn();
    const onReorderA = vi.fn();
    const { container } = render(DockRegionZoneDropHarness, {
      props: { shared: true, items, onPanelDropA, onReorderA },
    });
    layout(container);

    const [first] = stackItems(container);
    await fireEvent(first, pointer("pointerdown", 50, 50));
    await fireEvent(document, pointer("pointermove", 90, 50));
    await fireEvent(document, pointer("pointermove", 150, 50));
    await fireEvent(document, pointer("pointerup", 150, 50));

    expect(onReorderA).toHaveBeenCalledWith(["inspector", "explorer"]);
    expect(onPanelDropA).not.toHaveBeenCalled();
  });

  it("refuses a panel the receiving region's rules reject, on hover and at drop", async () => {
    const onPanelDropB = vi.fn();
    const { container } = render(DockRegionZoneDropHarness, {
      props: {
        shared: true,
        items,
        canAcceptPanel: (panelId: string) => panelId !== "explorer",
        onPanelDropB,
      },
    });
    layout(container);

    const [sourceItem] = stackItems(container);
    await fireEvent(sourceItem, pointer("pointerdown", 50, 50));
    await fireEvent(document, pointer("pointermove", 90, 50));
    await fireEvent(document, pointer("pointermove", 450, 50));

    expect(container.querySelectorAll("[data-drop-target]")).toHaveLength(0);

    await fireEvent(document, pointer("pointerup", 450, 50));
    expect(onPanelDropB).not.toHaveBeenCalled();
  });

  it("never leaks the encoded subject id through a public callback", async () => {
    const onValueChange = vi.fn();
    const onClose = vi.fn();
    const onReorder = vi.fn();
    const { container, getAllByRole } = render(DockRegion, {
      props: {
        edge: "left",
        items: [
          { value: "explorer", label: "Explorer", closable: true },
          { value: "inspector", label: "Inspector", closable: true },
        ],
        value: "explorer",
        onValueChange,
        onClose,
        onReorder,
      },
    });

    // The strip's tab values are the substrate's subject ids, and those carry
    // the panel's edge and zone. Everything a consumer sees has to be the
    // panel value it supplied.
    const [firstTab, secondTab] = getAllByRole("tab");
    await fireEvent.click(secondTab);
    expect(onValueChange).toHaveBeenCalledWith("inspector");

    const [close] = [...container.querySelectorAll<HTMLElement>(".poodle-tabs__close")];
    await fireEvent.click(close);
    expect(onClose).toHaveBeenCalledWith("explorer");

    firstTab.focus();
    await fireEvent.keyDown(firstTab, { key: "ArrowRight", altKey: true });
    expect(onReorder).toHaveBeenCalledWith(["inspector", "explorer"]);

    for (const call of [
      ...onValueChange.mock.calls,
      ...onClose.mock.calls,
      ...onReorder.mock.calls.flat(),
    ].flat()) {
      expect(JSON.stringify(call)).not.toContain("poodle-panel:");
    }
  });
});