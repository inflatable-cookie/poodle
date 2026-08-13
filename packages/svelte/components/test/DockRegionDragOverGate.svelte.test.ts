import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";

import { dockPanelDragSession } from "@inflatable-cookie/poodle-core";

import DockRegion from "../src/DockRegion.svelte";
import type { PanelTabItem } from "../src/types.ts";

// The drag-session gate (dock-region.md §8): a hovered dock can only read
// `DataTransfer.types`, never the payload, so it learns the in-flight panel
// from the shared `dockPanelDragSession` and consults `canAcceptPanel` before
// showing the drop-zone affordance. Foreign drags with no session entry keep
// the old permissive behaviour.

const items: PanelTabItem[] = [
  { value: "explorer", label: "Explorer" },
  { value: "inspector", label: "Inspector" },
];

const PANEL_DRAG_TYPE = "application/x-poodle-panel-drag";

afterEach(() => {
  dockPanelDragSession.clear();
});

function dragTab(region: HTMLElement): DataTransfer {
  const tab = region.querySelector("[draggable='true']")!;
  const dataTransfer = new DataTransfer();
  fireEvent.dragStart(tab, { dataTransfer });
  return dataTransfer;
}

function foreignPayload(): DataTransfer {
  const dataTransfer = new DataTransfer();
  dataTransfer.setData(
    PANEL_DRAG_TYPE,
    JSON.stringify({ panelId: "explorer", sourceEdge: "left" }),
  );
  return dataTransfer;
}

describe("DockRegion dragover consults canAcceptPanel", () => {
  it("does not highlight a dock the in-flight panel's rules forbid", async () => {
    const onPanelDrop = vi.fn();
    const source = render(DockRegion, {
      props: { items, value: "explorer", edge: "left", canAcceptPanel: () => true },
    });
    const target = render(DockRegion, {
      props: {
        items,
        value: "inspector",
        edge: "right",
        canAcceptPanel: (panelId: string) => panelId !== "explorer",
        onPanelDrop,
      },
    });
    const region = target.container.querySelector("section")!;

    const dataTransfer = dragTab(source.container.querySelector("section")!);
    const accepted = await fireEvent.dragOver(region, { dataTransfer });

    expect(accepted).toBe(true);
    expect(
      target.container.querySelector(".poodle-dock-region__drop-zone"),
    ).toBeNull();
  });

  it("highlights a dock whose rules admit the panel", async () => {
    const source = render(DockRegion, {
      props: { items, value: "explorer", edge: "left" },
    });
    const target = render(DockRegion, {
      props: { items, value: "inspector", edge: "right" },
    });
    const region = target.container.querySelector("section")!;

    const dataTransfer = dragTab(source.container.querySelector("section")!);
    const accepted = await fireEvent.dragOver(region, { dataTransfer });

    expect(accepted).toBe(false);
    expect(
      target.container.querySelector(".poodle-dock-region__drop-zone"),
    ).not.toBeNull();
  });

  it("keeps a foreign drag permissive when no session is announced", async () => {
    const target = render(DockRegion, {
      props: {
        items,
        value: "inspector",
        canAcceptPanel: () => false,
      },
    });
    const region = target.container.querySelector("section")!;

    const accepted = await fireEvent.dragOver(region, {
      dataTransfer: foreignPayload(),
    });

    expect(accepted).toBe(false);
    expect(
      target.container.querySelector(".poodle-dock-region__drop-zone"),
    ).not.toBeNull();
  });

  it("gates on a session announced by a host outside DockRegion", async () => {
    dockPanelDragSession.announce({
      panelId: "transport",
      sourceEdge: "bottom",
    });
    const target = render(DockRegion, {
      props: {
        items,
        value: "inspector",
        canAcceptPanel: (panelId: string) => panelId !== "transport",
      },
    });
    const region = target.container.querySelector("section")!;

    const accepted = await fireEvent.dragOver(region, {
      dataTransfer: foreignPayload(),
    });

    expect(accepted).toBe(true);
    expect(
      target.container.querySelector(".poodle-dock-region__drop-zone"),
    ).toBeNull();
  });

  it("suppresses the stack insert indicator for a forbidden panel", async () => {
    const source = render(DockRegion, {
      props: { items, sizing: "static", edge: "left" },
    });
    const target = render(DockRegion, {
      props: {
        items,
        sizing: "static",
        edge: "right",
        canAcceptPanel: (panelId: string) => panelId !== "explorer",
      },
    });
    const stackItem = target.container.querySelector(
      ".poodle-dock-region__stack-item",
    )!;

    const dataTransfer = dragTab(source.container.querySelector("section")!);
    await fireEvent.dragOver(stackItem, { dataTransfer });

    expect(
      target.container.querySelector("[data-drop-target]"),
    ).toBeNull();
  });
});
