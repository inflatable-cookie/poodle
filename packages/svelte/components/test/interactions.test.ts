import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Checkbox from "../src/Checkbox.svelte";
import DockRegion from "../src/DockRegion.svelte";
import NavigationMenu from "../src/NavigationMenu.svelte";
import Switch from "../src/Switch.svelte";
import Tabs from "../src/Tabs.svelte";
import { asSnippet } from "./snippet";

// Interaction wiring: the @inflatable-cookie/poodle-core machines have their own suite; these
// assert the Svelte binding actually drives a click through to the documented
// callback (the machine -> DOM -> event round trip).
function layoutStrip(container: HTMLElement): void {
  [...container.querySelectorAll<HTMLElement>(".poodle-tabs__item")].forEach((item, index) => {
    const rect = {
      x: index * 100,
      y: 0,
      width: 100,
      height: 30,
      top: 0,
      left: index * 100,
      right: index * 100 + 100,
      bottom: 30,
      toJSON() {
        return this;
      },
    } as DOMRect;
    item.getBoundingClientRect = () => rect;
    const tab = item.querySelector<HTMLElement>(".poodle-tabs__tab");
    if (tab) {
      tab.getBoundingClientRect = () => rect;
      tab.setPointerCapture = vi.fn();
      tab.releasePointerCapture = vi.fn();
      tab.hasPointerCapture = () => false;
    }
  });
}

function dragPointer(type: string, x: number, y: number): PointerEvent {
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

describe("svelte interaction", () => {
  it("Checkbox fires onCheckedChange(true) on click", async () => {
    const onCheckedChange = vi.fn();
    const { getByRole } = render(Checkbox, { props: { onCheckedChange } });
    await fireEvent.click(getByRole("checkbox"));
    expect(onCheckedChange).toHaveBeenCalledWith(true);
  });

  it("Switch fires onCheckedChange(true) on click", async () => {
    const onCheckedChange = vi.fn();
    const { getByRole } = render(Switch, { props: { onCheckedChange } });
    await fireEvent.click(getByRole("switch"));
    expect(onCheckedChange).toHaveBeenCalledWith(true);
  });

  it("Tabs defaults to card variant with tint fill, no edge and no border", () => {
    const { container } = render(Tabs, {
      props: {
        items: [{ value: "a", label: "A" }],
      },
    });
    const root = container.querySelector(".poodle-tabs")!;
    expect(root.getAttribute("data-variant")).toBe("card");
    expect(root.getAttribute("data-active-fill")).toBe("tint");
    expect(root.getAttribute("data-active-edge")).toBe("none");
    expect(root.getAttribute("data-bordered")).toBe("false");
  });

  it("Tabs emits activeEdge and solid fill data attributes", () => {
    const { container } = render(Tabs, {
      props: {
        items: [{ value: "a", label: "A" }],
        variant: "card",
        activeEdge: "outline",
        activeFill: "solid",
      },
    });
    const root = container.querySelector(".poodle-tabs")!;
    expect(root.getAttribute("data-active-edge")).toBe("outline");
    expect(root.getAttribute("data-active-fill")).toBe("solid");
  });

  it("Tabs block + underline renders the underline edge", () => {
    const { container } = render(Tabs, {
      props: {
        items: [{ value: "a", label: "A" }],
        variant: "block",
        activeEdge: "underline",
      },
    });
    const root = container.querySelector(".poodle-tabs")!;
    // The edge axis is a single enum member: exactly one value is emitted.
    expect(root.getAttribute("data-variant")).toBe("block");
    expect(root.getAttribute("data-active-edge")).toBe("underline");
  });

  it("Tabs activeFill=none suppresses the selected fill while the underline renders", () => {
    const { container } = render(Tabs, {
      props: {
        items: [{ value: "a", label: "A" }],
        variant: "block",
        activeEdge: "underline",
        activeFill: "none",
      },
    });
    const root = container.querySelector(".poodle-tabs")!;
    expect(root.getAttribute("data-active-fill")).toBe("none");
    // The CSS suppression keys off root `[data-active-fill="none"]` paired
    // with the item's selected state; the underline still renders from
    // `data-active-edge="underline"`.
    expect(root.getAttribute("data-active-edge")).toBe("underline");
    const selected = root.querySelector('.poodle-tabs__item[data-selected="true"]')!;
    expect(selected).not.toBeNull();
    expect(selected.getAttribute("data-selected")).toBe("true");
  });

  it("NavigationMenu defaults to tint fill with no edge", () => {
    const { container } = render(NavigationMenu, {
      props: {
        items: [{ value: "a", label: "A" }],
      },
    });
    const root = container.querySelector(".poodle-navigation-menu")!;
    expect(root.getAttribute("data-active-fill")).toBe("tint");
    expect(root.getAttribute("data-active-edge")).toBe("none");
  });

  it("NavigationMenu emits activeEdge and solid fill data attributes", () => {
    const { container } = render(NavigationMenu, {
      props: {
        items: [{ value: "a", label: "A" }],
        activeEdge: "underline",
        activeFill: "solid",
      },
    });
    const root = container.querySelector(".poodle-navigation-menu")!;
    expect(root.getAttribute("data-active-edge")).toBe("underline");
    expect(root.getAttribute("data-active-fill")).toBe("solid");
  });

  it("NavigationMenu activeFill=none emits the no-fill attribute", () => {
    const { container } = render(NavigationMenu, {
      props: {
        items: [{ value: "a", label: "A" }],
        value: "a",
        activeEdge: "underline",
        activeFill: "none",
      },
    });
    const root = container.querySelector(".poodle-navigation-menu")!;
    expect(root.getAttribute("data-active-fill")).toBe("none");
    expect(root.getAttribute("data-active-edge")).toBe("underline");
    const open = root.querySelector('.poodle-navigation-menu__trigger[data-open="true"]')!;
    expect(open).not.toBeNull();
  });

  it("Tabs reorders through the shared substrate with no native payload", async () => {
    const onReorder = vi.fn();
    const { container, getAllByRole } = render(Tabs, {
      props: {
        items: [
          { value: "surface-1", label: "Surface 1" },
          { value: "surface-2", label: "Surface 2" },
        ],
        reorderable: true,
        onReorder,
      },
    });
    const [firstTab] = getAllByRole("tab");
    layoutStrip(container);

    await fireEvent(firstTab, dragPointer("pointerdown", 50, 15));
    await fireEvent(document, dragPointer("pointermove", 150, 15));
    await fireEvent(document, dragPointer("pointerup", 150, 15));

    expect(onReorder).toHaveBeenCalledWith(["surface-2", "surface-1"]);
    // Local reorder writes nothing to the platform: the native envelope is
    // the cross-window transport's, and a plain reorder has no host.
    for (const tab of getAllByRole("tab")) {
      expect(tab.getAttribute("draggable")).toBe("false");
    }
  });

  it("DockRegion writes no native panel payload of its own", async () => {
    const { getByRole } = render(DockRegion, {
      props: {
        edge: "left",
        collapsed: true,
        items: [{ value: "inspector", label: "Inspector" }],
        value: "inspector",
      },
    });

    // The `application/x-poodle-panel-drag` wire and the module-global panel
    // session are both gone. A panel now moves as an ordinary drag subject
    // inside one controller, and the only thing that ever reaches
    // `DataTransfer` is the bounded cross-window receipt — which needs a host
    // bridge this region was not given.
    const dataTransfer = new DataTransfer();
    await fireEvent.dragStart(getByRole("tab"), { dataTransfer });

    expect([...dataTransfer.types]).toEqual([]);
    expect(getByRole("tab").getAttribute("draggable")).toBe("false");
  });

  it("DockRegion showTabs=false omits the tab strip", () => {
    const { container, queryByRole } = render(DockRegion, {
      props: {
        edge: "left",
        showTabs: false,
        items: [{ value: "inspector", label: "Inspector" }],
        value: "inspector",
        children: asSnippet(() => "body"),
      },
    });

    expect(queryByRole("tab")).toBeNull();
    expect(
      container.querySelector(".poodle-dock-region__strip"),
    ).toBeNull();
    expect(
      container.querySelector('.poodle-dock-region[data-show-tabs="false"]'),
    ).toBeTruthy();
  });
});
