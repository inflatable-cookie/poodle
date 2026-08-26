import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Tabs from "../src/Tabs.svelte";

const items = [
  { value: "mix", label: "Mix" },
  { value: "master", label: "Master", disabled: true },
  { value: "notes", label: "Notes", closable: true },
];

function tabs() {
  return screen.getAllByRole("tab");
}

describe("Tabs (svelte)", () => {
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

  it("pointer drag reports start, target, drop order, and end cleanup", async () => {
    const onDragStart = vi.fn();
    const onDragEnd = vi.fn();
    const onReorder = vi.fn();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "mix", reorderable: true, onDragStart, onDragEnd, onReorder },
    });
    const [source, , target] = tabs();
    const dataTransfer = new DataTransfer();
    await fireEvent.dragStart(source, { dataTransfer });
    expect(onDragStart).toHaveBeenCalledWith("mix", expect.any(DragEvent));
    expect(container.querySelector('[data-drag-source="true"]')).not.toBeNull();

    const targetItem = target.closest(".poodle-tabs__item")!;
    await fireEvent.dragOver(targetItem, { dataTransfer });
    expect(container.querySelector('[data-drop-target="true"]')).toBe(targetItem);

    await fireEvent.dragLeave(targetItem);
    expect(container.querySelector('[data-drop-target="true"]')).toBeNull();

    await fireEvent.dragOver(targetItem, { dataTransfer });
    await fireEvent.drop(targetItem, { dataTransfer });
    expect(onReorder).toHaveBeenCalledWith(["master", "notes", "mix"]);

    await fireEvent.dragEnd(source);
    expect(onDragEnd).toHaveBeenCalledWith("mix", expect.any(DragEvent));
    expect(container.querySelector("[data-drag-source]")).toBeNull();
    expect(container.querySelector("[data-drop-target]")).toBeNull();
  });

  it("cancelling a drag clears transient source and target state", async () => {
    const onReorder = vi.fn();
    const { container } = render(Tabs, {
      props: { items, defaultValue: "mix", reorderable: true, onReorder },
    });
    const [source, , target] = tabs();
    const dataTransfer = new DataTransfer();
    await fireEvent.dragStart(source, { dataTransfer });
    await fireEvent.dragOver(target.closest(".poodle-tabs__item")!, { dataTransfer });
    await fireEvent.dragEnd(source);
    expect(onReorder).not.toHaveBeenCalled();
    expect(container.querySelector("[data-drag-source]")).toBeNull();
    expect(container.querySelector("[data-drop-target]")).toBeNull();
  });

  it("disabled tabs are not draggable", () => {
    render(Tabs, { props: { items, defaultValue: "mix", reorderable: true } });
    expect(tabs()[1].getAttribute("draggable")).toBe("false");
    expect(tabs()[0].getAttribute("draggable")).toBe("true");
  });
});
