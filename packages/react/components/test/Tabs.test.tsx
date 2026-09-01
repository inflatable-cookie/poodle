import { act, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { Tabs } from "../src/Tabs";

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

function send(target: EventTarget, event: PointerEvent): void {
  act(() => {
    target.dispatchEvent(event);
  });
}

describe("Tabs (react)", () => {
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

  it("automatic arrows skip disabled tabs and commit selection", () => {
    const onValueChange = vi.fn();
    render(<Tabs items={items} defaultValue="mix" onValueChange={onValueChange} />);
    tabs()[0].focus();
    fireEvent.keyDown(tabs()[0], { key: "ArrowRight" });
    expect(onValueChange).toHaveBeenCalledWith("notes");
    expect(tabs().map((tab) => tab.getAttribute("tabindex"))).toEqual(["-1", "-1", "0"]);
  });

  it("manual arrows move the tab stop without committing", () => {
    const onValueChange = vi.fn();
    render(
      <Tabs items={items} defaultValue="mix" activationMode="manual" onValueChange={onValueChange} />,
    );
    tabs()[0].focus();
    fireEvent.keyDown(tabs()[0], { key: "ArrowRight" });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(tabs().map((tab) => tab.getAttribute("aria-selected"))).toEqual([
      "true",
      "false",
      "false",
    ]);
    expect(tabs().map((tab) => tab.getAttribute("tabindex"))).toEqual(["-1", "-1", "0"]);
  });

  it("vertical arrows are the orientation axis", () => {
    const onValueChange = vi.fn();
    render(
      <Tabs items={items} defaultValue="mix" orientation="vertical" onValueChange={onValueChange} />,
    );
    tabs()[0].focus();
    fireEvent.keyDown(tabs()[0], { key: "ArrowRight" });
    expect(onValueChange).not.toHaveBeenCalled();
    fireEvent.keyDown(tabs()[0], { key: "ArrowDown" });
    expect(onValueChange).toHaveBeenCalledWith("notes");
  });

  it("Delete closes only a closable tab", () => {
    const onClose = vi.fn();
    render(<Tabs items={items} defaultValue="mix" onClose={onClose} />);
    fireEvent.keyDown(tabs()[0], { key: "Delete" });
    expect(onClose).not.toHaveBeenCalled();
    fireEvent.keyDown(tabs()[2], { key: "Delete" });
    expect(onClose).toHaveBeenCalledWith("notes");
  });

  it("Alt+Arrow emits the complete next order", () => {
    const onReorder = vi.fn();
    render(<Tabs items={items} defaultValue="mix" reorderable onReorder={onReorder} />);
    tabs()[0].focus();
    fireEvent.keyDown(tabs()[0], { key: "ArrowRight", altKey: true });
    expect(onReorder).toHaveBeenCalledWith(["master", "mix", "notes"]);
  });

  it("a pointer drag posts the target and commits the same order the old drop did", async () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tabs items={items} defaultValue="mix" reorderable onReorder={onReorder} />,
    );
    layout(container);
    const [source] = tabs();
    const [sourceItem, , targetItem] = itemsOf(container);

    send(source, pointer("pointerdown", 50, 15));
    send(document, pointer("pointermove", 90, 15));
    expect(sourceItem.getAttribute("data-drag-source")).toBe("true");

    send(document, pointer("pointermove", 250, 15));
    expect(targetItem.getAttribute("data-drop-target")).toBe("true");

    send(document, pointer("pointerup", 250, 15));

    // Trailing half of the target: the tab lands at that tab.
    expect(onReorder).toHaveBeenCalledWith(["master", "notes", "mix"]);
    expect(container.querySelector("[data-drag-source]")).toBeNull();
    expect(container.querySelector("[data-drop-target]")).toBeNull();
  });

  it("Escape cancels a drag and clears transient source and target state", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tabs items={items} defaultValue="mix" reorderable onReorder={onReorder} />,
    );
    layout(container);
    const [source] = tabs();

    send(source, pointer("pointerdown", 50, 15));
    send(document, pointer("pointermove", 250, 15));
    expect(container.querySelector("[data-drop-target]")).not.toBeNull();

    act(() => {
      fireEvent.keyDown(document, { key: "Escape" });
    });

    expect(onReorder).not.toHaveBeenCalled();
    expect(container.querySelector("[data-drag-source]")).toBeNull();
    expect(container.querySelector("[data-drop-target]")).toBeNull();
  });

  it("dragging over a sibling then back to origin does not swap", () => {
    const onReorder = vi.fn();
    const files = [
      { value: "index.ts", label: "index.ts" },
      { value: "App.svelte", label: "App.svelte", closable: true },
      { value: "utils.ts", label: "utils.ts", closable: true },
      { value: "types.ts", label: "types.ts", closable: true },
    ];
    const { container } = render(
      <Tabs items={files} defaultValue="App.svelte" reorderable onReorder={onReorder} />,
    );
    layout(container);
    const source = tabs()[1];
    const [, sourceItem, siblingItem] = itemsOf(container);

    send(source, pointer("pointerdown", 150, 15));
    send(document, pointer("pointermove", 190, 15));
    send(document, pointer("pointermove", 270, 15));
    expect(siblingItem.getAttribute("data-drop-target")).toBe("true");

    send(document, pointer("pointermove", 150, 15));
    expect(sourceItem.getAttribute("data-drop-target")).toBeNull();
    expect(siblingItem.getAttribute("data-drop-target")).toBeNull();

    send(document, pointer("pointerup", 150, 15));
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("dropping on a sibling lands at that sibling even on the origin-facing half", () => {
    const onReorder = vi.fn();
    const files = [
      { value: "index.ts", label: "index.ts" },
      { value: "App.svelte", label: "App.svelte", closable: true },
      { value: "utils.ts", label: "utils.ts", closable: true },
      { value: "types.ts", label: "types.ts", closable: true },
    ];
    const { container } = render(
      <Tabs items={files} defaultValue="App.svelte" reorderable onReorder={onReorder} />,
    );
    layout(container);
    const source = tabs()[1];
    const [, , siblingItem] = itemsOf(container);

    send(source, pointer("pointerdown", 150, 15));
    send(document, pointer("pointermove", 190, 15));
    send(document, pointer("pointermove", 220, 15));
    expect(siblingItem.getAttribute("data-drop-target")).toBe("true");

    send(document, pointer("pointerup", 220, 15));
    expect(onReorder).toHaveBeenCalledWith(["index.ts", "utils.ts", "App.svelte", "types.ts"]);
  });

  it("a second drag uses the post-reorder layout, not the original slots", () => {
    const onReorder = vi.fn();
    const files = [
      { value: "index.ts", label: "index.ts" },
      { value: "App.svelte", label: "App.svelte", closable: true },
      { value: "utils.ts", label: "utils.ts", closable: true },
      { value: "types.ts", label: "types.ts", closable: true },
    ];
    const { container } = render(
      <Tabs items={files} defaultValue="App.svelte" reorderable onReorder={onReorder} />,
    );
    layout(container);
    const source = tabs()[1];

    send(source, pointer("pointerdown", 150, 15));
    send(document, pointer("pointermove", 190, 15));
    send(document, pointer("pointermove", 250, 15));
    send(document, pointer("pointerup", 250, 15));
    expect(onReorder).toHaveBeenCalledWith(["index.ts", "utils.ts", "App.svelte", "types.ts"]);

    layout(container);
    const moved = [...tabs()].find((tab) => tab.getAttribute("data-value") === "App.svelte")!;
    const occupant = itemsOf(container)[1];

    send(moved, pointer("pointerdown", 250, 15));
    send(document, pointer("pointermove", 260, 15));
    expect(occupant.getAttribute("data-drop-target")).toBeNull();
    expect(moved.closest(".poodle-tabs__item")?.getAttribute("data-drop-target")).toBeNull();
  });

  it("the drag preview follows the pointer while the hover target stays put", () => {
    const files = [
      { value: "index.ts", label: "index.ts" },
      { value: "App.svelte", label: "App.svelte", closable: true },
    ];
    const { container } = render(<Tabs items={files} defaultValue="App.svelte" reorderable />);
    layout(container);

    send(tabs()[1], pointer("pointerdown", 150, 15));
    send(document, pointer("pointermove", 190, 15));
    const preview = container.querySelector<HTMLElement>(".poodle-drag-preview");
    expect(preview).not.toBeNull();
    expect(preview?.style.transform).toBe(`translate3d(${190 + 12}px, ${15 + 12}px, 0)`);

    send(document, pointer("pointermove", 170, 18));
    expect(preview?.style.transform).toBe(`translate3d(${170 + 12}px, ${18 + 12}px, 0)`);
  });

  it("a tab dropped on itself is refused rather than reordered", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tabs items={items} defaultValue="mix" reorderable onReorder={onReorder} />,
    );
    layout(container);
    const [source] = tabs();
    const [sourceItem] = itemsOf(container);

    send(source, pointer("pointerdown", 50, 15));
    send(document, pointer("pointermove", 90, 15));
    expect(sourceItem.getAttribute("data-drop-target")).toBeNull();

    send(document, pointer("pointerup", 90, 15));
    expect(onReorder).not.toHaveBeenCalled();
  });

  it("a disabled tab cannot be picked up but is still a place to put one", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tabs items={items} defaultValue="mix" reorderable onReorder={onReorder} />,
    );
    layout(container);
    const [, disabled] = tabs();
    const [enabledItem, disabledItem] = itemsOf(container);

    expect(enabledItem.getAttribute("data-reorderable")).toBe("true");
    expect(disabledItem.getAttribute("data-reorderable")).toBeNull();

    send(disabled, pointer("pointerdown", 150, 15));
    send(document, pointer("pointermove", 250, 15));
    expect(container.querySelector("[data-drag-source]")).toBeNull();
    send(document, pointer("pointerup", 250, 15));

    // Still a landing position: a disabled tab occupies an index, and a
    // reorder that could not pass through it would be a different result.
    send(tabs()[0], pointer("pointerdown", 50, 15));
    send(document, pointer("pointermove", 150, 15));
    expect(disabledItem.getAttribute("data-drop-target")).toBe("true");
    send(document, pointer("pointerup", 150, 15));
    expect(onReorder).toHaveBeenCalledWith(["master", "mix", "notes"]);
  });

  it("no tab advertises a native drag without a cross-window host", () => {
    const { container } = render(<Tabs items={items} defaultValue="mix" reorderable />);
    for (const tab of itemsOf(container).map((item) => item.querySelector(".poodle-tabs__tab"))) {
      expect(tab?.getAttribute("draggable")).toBe("false");
    }
  });
});
