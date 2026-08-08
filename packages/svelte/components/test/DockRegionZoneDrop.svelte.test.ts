import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import DockRegion from "../src/DockRegion.svelte";
import type { PanelTabItem } from "../src/types.ts";

const items: PanelTabItem[] = [
  { value: "explorer", label: "Explorer" },
  { value: "inspector", label: "Inspector" },
];

function dragTab(region: HTMLElement): DataTransfer {
  const tab = region.querySelector("[draggable='true']")!;
  const dataTransfer = new DataTransfer();
  fireEvent.dragStart(tab, { dataTransfer });
  return dataTransfer;
}

describe("DockRegion drop zones", () => {
  it("accepts a drop between two zones that share an edge", async () => {
    const onDropA = vi.fn();
    const onDropB = vi.fn();
    const a = render(DockRegion, {
      props: { items, value: "explorer", edge: "top", dragZoneId: "region:a", onPanelDrop: onDropA },
    });
    const b = render(DockRegion, {
      props: { items, value: "explorer", edge: "top", dragZoneId: "region:b", onPanelDrop: onDropB },
    });
    const regionA = a.container.querySelector("section")!;
    const regionB = b.container.querySelector("section")!;

    const dataTransfer = dragTab(regionA);
    await fireEvent.drop(regionB, { dataTransfer });

    expect(onDropB).toHaveBeenCalledOnce();
    expect(onDropB.mock.calls[0][0].panel).toMatchObject({
      panelId: "explorer",
      sourceEdge: "top",
      sourceZone: "region:a",
    });
    expect(onDropA).not.toHaveBeenCalled();
  });

  it("ignores a drop back onto the source zone in flexible sizing", async () => {
    const onDrop = vi.fn();
    const a = render(DockRegion, {
      props: { items, value: "explorer", edge: "top", dragZoneId: "region:a", onPanelDrop: onDrop },
    });
    const regionA = a.container.querySelector("section")!;

    const dataTransfer = dragTab(regionA);
    await fireEvent.drop(regionA, { dataTransfer });

    expect(onDrop).not.toHaveBeenCalled();
  });

  it("treats a legacy edge-only payload from the same edge as same-zone", async () => {
    const onDrop = vi.fn();
    const a = render(DockRegion, {
      props: { items, value: "explorer", edge: "top", onPanelDrop: onDrop },
    });
    const region = a.container.querySelector("section")!;

    const dataTransfer = new DataTransfer();
    dataTransfer.setData(
      "application/x-poodle-panel-drag",
      JSON.stringify({ panelId: "explorer", sourceEdge: "top" }),
    );
    await fireEvent.drop(region, { dataTransfer });

    expect(onDrop).not.toHaveBeenCalled();
  });
});
