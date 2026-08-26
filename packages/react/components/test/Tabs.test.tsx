import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Tabs } from "../src/Tabs";

const items = [
  { value: "mix", label: "Mix" },
  { value: "master", label: "Master", disabled: true },
  { value: "notes", label: "Notes", closable: true },
];

function tabs() {
  return screen.getAllByRole("tab");
}

describe("Tabs (react)", () => {
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

  it("pointer drag reports start, target, drop order, and end cleanup", () => {
    const onDragStart = vi.fn();
    const onDragEnd = vi.fn();
    const onReorder = vi.fn();
    const { container } = render(
      <Tabs
        items={items}
        defaultValue="mix"
        reorderable
        onDragStart={onDragStart}
        onDragEnd={onDragEnd}
        onReorder={onReorder}
      />,
    );
    const [source, , target] = tabs();
    const dataTransfer = new DataTransfer();
    fireEvent.dragStart(source, { dataTransfer });
    expect(onDragStart).toHaveBeenCalledWith("mix", expect.any(Object));
    expect(container.querySelector('[data-drag-source="true"]')).not.toBeNull();

    const targetItem = target.closest(".poodle-tabs__item")!;
    fireEvent.dragOver(targetItem, { dataTransfer });
    expect(container.querySelector('[data-drop-target="true"]')).toBe(targetItem);

    fireEvent.dragLeave(targetItem);
    expect(container.querySelector('[data-drop-target="true"]')).toBeNull();

    fireEvent.dragOver(targetItem, { dataTransfer });
    fireEvent.drop(targetItem, { dataTransfer });
    expect(onReorder).toHaveBeenCalledWith(["master", "notes", "mix"]);

    fireEvent.dragEnd(source);
    expect(onDragEnd).toHaveBeenCalledWith("mix", expect.any(Object));
    expect(container.querySelector("[data-drag-source]")).toBeNull();
    expect(container.querySelector("[data-drop-target]")).toBeNull();
  });

  it("cancelling a drag clears transient source and target state", () => {
    const onReorder = vi.fn();
    const { container } = render(
      <Tabs items={items} defaultValue="mix" reorderable onReorder={onReorder} />,
    );
    const [source, , target] = tabs();
    const dataTransfer = new DataTransfer();
    fireEvent.dragStart(source, { dataTransfer });
    fireEvent.dragOver(target.closest(".poodle-tabs__item")!, { dataTransfer });
    fireEvent.dragEnd(source);
    expect(onReorder).not.toHaveBeenCalled();
    expect(container.querySelector("[data-drag-source]")).toBeNull();
    expect(container.querySelector("[data-drop-target]")).toBeNull();
  });

  it("disabled tabs are not draggable", () => {
    render(<Tabs items={items} defaultValue="mix" reorderable />);
    expect(tabs()[1].getAttribute("draggable")).toBe("false");
    expect(tabs()[0].getAttribute("draggable")).toBe("true");
  });
});
