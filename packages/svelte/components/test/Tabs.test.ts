import { fireEvent, render, screen } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import Tabs from "../src/Tabs.svelte";

const items = [
  { value: "mix", label: "Mix" },
  { value: "master", label: "Master", disabled: true },
  { value: "notes", label: "Notes", closable: true },
];

function tabs() {
  return screen.getAllByRole("tab");
}

function itemsOf(container: HTMLElement): HTMLElement[] {
  return [...container.querySelectorAll<HTMLElement>(".poodle-tabs__item")];
}

/**
 * Lay the strip out horizontally.
 *
 * The substrate hit-tests measured rectangles, and happy-dom measures
 * everything as an empty box at the origin — every tab would contain every
 * point. Giving each item its real place is what makes "dropped on the third
 * tab" mean anything here.
 */
function layout(container: HTMLElement): void {
  itemsOf(container).forEach((item, index) => {
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

describe("Tabs (svelte)", () => {
  beforeEach(() => {
    // The controller coalesces pointer movement onto a frame; the tests drive
    // it synchronously so a move and its hit test are one step.
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("automatic arrows skip disabled tabs and commit selection", async () => {
    const onValueChange = vi.fn();
    render(Tabs, { props: { items, defaultValue: "mix", onValueChange } });
    tabs()[0].focus();
    await fireEvent.keyDown(tabs()[0], { key: "ArrowRight" });
    expect(onValueChange).toHaveBeenCalledWith("notes");
    expect(tabs().map((tab) => tab.getAttribute("tabindex"))).toEqual(["-1", "-1", "0"]);
  });

  it("manual arrows move the tab stop without committing", async () => {
    const onValueChange = vi.fn();
    render(Tabs, {
      props: { items, defaultValue: "mix", activationMode: "manual", onValueChange },
    });
    tabs()[0].focus();
    await fireEvent.keyDown(tabs()[0], { key: "ArrowRight" });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(tabs().map((tab) => tab.getAttribute("aria-selected"))).toEqual([
      "true",
      "false",
      "false",
    ]);
    expect(tabs().map((tab) => tab.getAttribute("tabindex"))).toEqual(["-1", "-1", "0"]);
  });

  it("vertical arrows are the orientation axis", async () => {
    const onValueChange = vi.fn();
    render(Tabs, {
      props: { items, defaultValue: "mix", orientation: "vertical", onValueChange },
    });
    tabs()[0].focus();
    await fireEvent.keyDown(tabs()[0], { key: "ArrowRight" });
    expect(onValueChange).not.toHaveBeenCalled();
    await fireEvent.keyDown(tabs()[0], { key: "ArrowDown" });
    expect(onValueChange).toHaveBeenCalledWith("notes");
  });

  it("Delete closes only a closable tab", async () => {
    const onClose = vi.fn();
    render(Tabs, { props: { items, defaultValue: "mix", onClose } });
    await fireEvent.keyDown(tabs()[0], { key: "Delete" });
    expect(onClose).not.toHaveBeenCalled();
    await fireEvent.keyDown(tabs()[2], { key: "Delete" });
    expect(onClose).toHaveBeenCalledWith("notes");
  });

  it("Alt+Arrow emits the complete next order", async () => {
    const onReorder = vi.fn();
    render(Tabs, {
      props: { items, defaultValue: "mix", reorderable: true, onReorder },
    });
    tabs()[0].focus();
    await fireEvent.keyDown(tabs()[0], { key: "ArrowRight", altKey: true });
    expect(onReorder).toHaveBeenCalledWith(["master", "mix", "notes"]);
  });

  it("a pointer drag posts the target and commits the same order the old drop did", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "mix", reorderable: true, onReorder },
    });
    layout(container);
    const [source] = tabs();
    const [sourceItem, , targetItem] = itemsOf(container);

    await fireEvent(source, pointer("pointerdown", 50, 15));
    await fireEvent(document, pointer("pointermove", 90, 15));
    expect(sourceItem.getAttribute("data-drag-source")).toBe("true");

    await fireEvent(document, pointer("pointermove", 250, 15));
    expect(targetItem.getAttribute("data-drop-target")).toBe("true");

    await fireEvent(document, pointer("pointerup", 250, 15));
    await Promise.resolve();

    // Trailing half of the target: the tab lands at that tab.
    expect(onReorder).toHaveBeenCalledWith(["master", "notes", "mix"]);
    expect(container.querySelector("[data-drag-source]")).toBeNull();
    expect(container.querySelector("[data-drop-target]")).toBeNull();
  });

  it("Escape cancels a drag and clears transient source and target state", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "mix", reorderable: true, onReorder },
    });
    layout(container);
    const [source] = tabs();

    await fireEvent(source, pointer("pointerdown", 50, 15));
    await fireEvent(document, pointer("pointermove", 250, 15));
    expect(container.querySelector("[data-drop-target]")).not.toBeNull();

    await fireEvent.keyDown(document, { key: "Escape" });

    expect(onReorder).not.toHaveBeenCalled();
    expect(container.querySelector("[data-drag-source]")).toBeNull();
    expect(container.querySelector("[data-drop-target]")).toBeNull();
  });

  it("dragging over a sibling then back to origin does not swap", async () => {
    const onReorder = vi.fn();
    const files = [
      { value: "index.ts", label: "index.ts" },
      { value: "App.svelte", label: "App.svelte", closable: true },
      { value: "utils.ts", label: "utils.ts", closable: true },
      { value: "types.ts", label: "types.ts", closable: true },
    ];
    const { container } = render(Tabs, {
      props: { items: files, defaultValue: "App.svelte", reorderable: true, onReorder },
    });
    layout(container);
    const source = tabs()[1];
    const [, sourceItem, siblingItem] = itemsOf(container);

    await fireEvent(source, pointer("pointerdown", 150, 15));
    await fireEvent(document, pointer("pointermove", 190, 15));
    await fireEvent(document, pointer("pointermove", 270, 15));
    expect(siblingItem.getAttribute("data-drop-target")).toBe("true");

    await fireEvent(document, pointer("pointermove", 150, 15));
    expect(sourceItem.getAttribute("data-drop-target")).toBeNull();
    expect(siblingItem.getAttribute("data-drop-target")).toBeNull();

    await fireEvent(document, pointer("pointerup", 150, 15));
    await Promise.resolve();
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("the origin-facing half of a sibling is a return to origin, not a swap", async () => {
    const onReorder = vi.fn();
    const files = [
      { value: "index.ts", label: "index.ts" },
      { value: "App.svelte", label: "App.svelte", closable: true },
      { value: "utils.ts", label: "utils.ts", closable: true },
      { value: "types.ts", label: "types.ts", closable: true },
    ];
    const { container } = render(Tabs, {
      props: { items: files, defaultValue: "App.svelte", reorderable: true, onReorder },
    });
    layout(container);
    const source = tabs()[1];
    const [, , siblingItem] = itemsOf(container);

    await fireEvent(source, pointer("pointerdown", 150, 15));
    await fireEvent(document, pointer("pointermove", 190, 15));
    await fireEvent(document, pointer("pointermove", 270, 15));
    expect(siblingItem.getAttribute("data-drop-target")).toBe("true");

    await fireEvent(document, pointer("pointermove", 220, 15));
    await fireEvent(document, pointer("pointerup", 220, 15));
    await Promise.resolve();
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("a tab dropped on itself is refused rather than reordered", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "mix", reorderable: true, onReorder },
    });
    layout(container);
    const [source] = tabs();
    const [sourceItem] = itemsOf(container);

    await fireEvent(source, pointer("pointerdown", 50, 15));
    await fireEvent(document, pointer("pointermove", 90, 15));
    expect(sourceItem.getAttribute("data-drop-target")).toBeNull();

    await fireEvent(document, pointer("pointerup", 90, 15));
    await Promise.resolve();
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("a disabled tab cannot be picked up but is still a place to put one", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "mix", reorderable: true, onReorder },
    });
    layout(container);
    const [, disabled] = tabs();
    const [enabledItem, disabledItem] = itemsOf(container);

    expect(enabledItem.getAttribute("data-reorderable")).toBe("true");
    expect(disabledItem.getAttribute("data-reorderable")).toBeNull();

    // Not a source.
    await fireEvent(disabled, pointer("pointerdown", 150, 15));
    await fireEvent(document, pointer("pointermove", 250, 15));
    expect(container.querySelector("[data-drag-source]")).toBeNull();
    await fireEvent(document, pointer("pointerup", 250, 15));

    // Still a landing position: a disabled tab occupies an index, and a
    // reorder that could not pass through it would be a different result.
    await fireEvent(tabs()[0], pointer("pointerdown", 50, 15));
    await fireEvent(document, pointer("pointermove", 150, 15));
    expect(disabledItem.getAttribute("data-drop-target")).toBe("true");
    await fireEvent(document, pointer("pointerup", 150, 15));
    await Promise.resolve();
    expect(onReorder).toHaveBeenCalledWith(["master", "mix", "notes"]);
  });

  it("no tab advertises a native drag without a cross-window host", () => {
    const { container } = render(Tabs, {
      props: { items, defaultValue: "mix", reorderable: true },
    });
    for (const tab of itemsOf(container).map((item) => item.querySelector(".poodle-tabs__tab"))) {
      expect(tab?.getAttribute("draggable")).toBe("false");
    }
  });
});
