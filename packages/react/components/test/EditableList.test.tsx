import { fireEvent, render } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { EditableList } from "../src/EditableList";

const items = [
  { id: "a", label: "Alpha" },
  { id: "b", label: "Beta" },
  { id: "c", label: "Gamma" },
];

function asRect(top: number): DOMRect {
  return {
    x: 10,
    y: top,
    width: 200,
    height: 32,
    top,
    left: 10,
    right: 210,
    bottom: top + 32,
    toJSON: () => ({}),
  } as DOMRect;
}

function stackRows(container: HTMLElement): HTMLElement[] {
  const rows = [...container.querySelectorAll('[role="option"]')] as HTMLElement[];
  for (const [index, row] of rows.entries()) {
    row.getBoundingClientRect = () => asRect(10 + index * 40);
    row.setPointerCapture = vi.fn();
    row.releasePointerCapture = vi.fn();
  }
  return rows;
}

describe("EditableList (react)", () => {
  beforeEach(() => {
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.useRealTimers();
  });

  it("renders a labelled listbox of reorderable items", () => {
    const { container } = render(<EditableList items={items} ariaLabel="Tags" />);
    const list = container.querySelector(".poodle-editable-list") as HTMLElement;
    expect(list.getAttribute("role")).toBe("listbox");
    expect(list.getAttribute("aria-label")).toBe("Tags");

    const rows = [...container.querySelectorAll('[role="option"]')];
    expect(rows.length).toBe(3);
    expect(rows[0].getAttribute("aria-label")).toContain("Reorder Alpha. Position 1 of 3.");
    expect(rows[0].getAttribute("draggable")).toBe("false");
    expect(container.querySelectorAll(".poodle-editable-list__handle").length).toBe(3);
  });

  it("omits drag handles and keeps rows static when reordering is disabled", () => {
    const { container } = render(<EditableList items={items} reorderable={false} />);
    expect(container.querySelector(".poodle-editable-list__handle")).toBeNull();
    expect(container.querySelector('[role="option"]')?.getAttribute("draggable")).toBe("false");
  });

  it("adds an item through the input and reports it", () => {
    const onAdd = vi.fn();
    const onChange = vi.fn();
    const { container } = render(
      <EditableList items={items} editable addLabel="Add tag" onAdd={onAdd} onChange={onChange} />,
    );
    const input = container.querySelector("input") as HTMLInputElement;
    fireEvent.input(input, { target: { value: "Delta" } });
    const add = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Add tag"),
    ) as HTMLButtonElement;
    fireEvent.click(add);

    expect(onAdd).toHaveBeenCalledTimes(1);
    const added = onAdd.mock.calls[0][0] as { label: string };
    expect(added.label).toBe("Delta");
    expect(onChange).toHaveBeenCalledTimes(1);
    expect((onChange.mock.calls[0][0] as typeof items).length).toBe(4);
  });

  it("disables the add button while the input is empty", () => {
    const { container } = render(<EditableList items={items} editable addLabel="Add tag" />);
    const add = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Add tag"),
    ) as HTMLButtonElement;
    expect(add.disabled).toBe(true);
  });

  it("removes an item and reports the id", () => {
    const onRemove = vi.fn();
    const { container } = render(<EditableList items={items} editable onRemove={onRemove} />);
    const remove = container.querySelector(
      'button[aria-label="Remove Beta"]',
    ) as HTMLButtonElement;
    fireEvent.click(remove);
    expect(onRemove).toHaveBeenCalledWith("b");
  });

  it("reorders with keyboard grab, move, and drop", () => {
    const onReorder = vi.fn();
    const { container } = render(<EditableList items={items} onReorder={onReorder} />);
    const rows = stackRows(container);
    rows[0].focus();

    fireEvent.keyDown(rows[0], { key: " " });
    fireEvent.keyDown(rows[0], { key: "ArrowDown" });
    fireEvent.keyDown(rows[0], { key: " " });

    expect(onReorder).toHaveBeenCalledTimes(1);
    const reordered = onReorder.mock.calls[0][0] as typeof items;
    expect(reordered.map((item) => item.id)).toEqual(["b", "a", "c"]);
  });

  it("reorders with a pointer drag from the handle", () => {
    const onReorder = vi.fn();
    const { container } = render(<EditableList items={items} onReorder={onReorder} />);
    const rows = stackRows(container);
    const handle = rows[0].querySelector(".poodle-editable-list__handle") as HTMLElement;

    fireEvent.pointerDown(handle, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    fireEvent.pointerMove(handle, { pointerId: 1, clientX: 20, clientY: 110 });
    fireEvent.pointerUp(handle, { pointerId: 1, clientX: 20, clientY: 110 });

    expect(onReorder).toHaveBeenCalledTimes(1);
    expect((onReorder.mock.calls[0][0] as typeof items).map((item) => item.id)).toEqual(["b", "c", "a"]);
  });

  it("reorders with a touch-like pointer", () => {
    const onReorder = vi.fn();
    const { container } = render(<EditableList items={items} onReorder={onReorder} />);
    const rows = stackRows(container);
    const handle = rows[0].querySelector(".poodle-editable-list__handle") as HTMLElement;

    fireEvent.pointerDown(handle, {
      button: 0,
      pointerId: 1,
      pointerType: "pen",
      clientX: 20,
      clientY: 20,
    });
    fireEvent.pointerMove(handle, { pointerId: 1, pointerType: "pen", clientX: 20, clientY: 110 });
    fireEvent.pointerUp(handle, { pointerId: 1, pointerType: "pen", clientX: 20, clientY: 110 });

    expect(onReorder).toHaveBeenCalledTimes(1);
    expect((onReorder.mock.calls[0][0] as typeof items).map((item) => item.id)).toEqual(["b", "c", "a"]);
  });

  it("cancels a keyboard grab on Escape without reordering", () => {
    const onReorder = vi.fn();
    const { container } = render(<EditableList items={items} onReorder={onReorder} />);
    const rows = stackRows(container);
    rows[0].focus();

    fireEvent.keyDown(rows[0], { key: " " });
    fireEvent.keyDown(rows[0], { key: "ArrowDown" });
    fireEvent.keyDown(rows[0], { key: "Escape" });

    expect(onReorder).not.toHaveBeenCalled();
    expect(rows[0].hasAttribute("data-poodle-drag-source")).toBe(false);
  });

  it("does not drag from a remove button", () => {
    const onReorder = vi.fn();
    const onRemove = vi.fn();
    const { container } = render(
      <EditableList items={items} editable onReorder={onReorder} onRemove={onRemove} />,
    );
    const rows = stackRows(container);
    const remove = container.querySelector('button[aria-label="Remove Alpha"]') as HTMLButtonElement;

    fireEvent.pointerDown(remove, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    fireEvent.pointerMove(remove, { pointerId: 1, clientX: 20, clientY: 110 });
    fireEvent.pointerUp(remove, { pointerId: 1, clientX: 20, clientY: 110 });
    fireEvent.click(remove);

    expect(onReorder).not.toHaveBeenCalled();
    expect(onRemove).toHaveBeenCalledWith("a");
    expect(rows[0].hasAttribute("data-poodle-drag-source")).toBe(false);
  });

  it("keeps disabled rows inert to pointer and keyboard sensors", () => {
    const onReorder = vi.fn();
    const { container } = render(<EditableList items={items} disabled onReorder={onReorder} />);
    const rows = stackRows(container);
    const handle = rows[0].querySelector(".poodle-editable-list__handle") as HTMLElement;
    rows[0].focus();

    fireEvent.pointerDown(handle, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    fireEvent.pointerMove(handle, { pointerId: 1, clientX: 20, clientY: 110 });
    fireEvent.pointerUp(handle, { pointerId: 1, clientX: 20, clientY: 110 });
    fireEvent.keyDown(rows[0], { key: " " });
    fireEvent.keyDown(rows[0], { key: "ArrowDown" });

    expect(onReorder).not.toHaveBeenCalled();
  });

  it("cancels when the dragging source is removed", () => {
    const onReorder = vi.fn();
    const { container, rerender } = render(<EditableList items={items} onReorder={onReorder} />);
    const rows = stackRows(container);
    const handle = rows[0].querySelector(".poodle-editable-list__handle") as HTMLElement;

    fireEvent.pointerDown(handle, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    fireEvent.pointerMove(handle, { pointerId: 1, clientX: 20, clientY: 50 });
    expect(rows[0].getAttribute("data-poodle-drag-source")).toBe("dragging");

    rerender(<EditableList items={items.slice(1)} onReorder={onReorder} />);
    expect(onReorder).not.toHaveBeenCalled();
    expect(container.querySelector("[data-poodle-drag-source]")).toBeNull();
  });

  it("cancels when the current target is removed", () => {
    const onReorder = vi.fn();
    const { container, rerender } = render(<EditableList items={items} onReorder={onReorder} />);
    const rows = stackRows(container);
    const handle = rows[0].querySelector(".poodle-editable-list__handle") as HTMLElement;

    fireEvent.pointerDown(handle, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    fireEvent.pointerMove(handle, { pointerId: 1, clientX: 20, clientY: 110 });
    expect(rows[2].getAttribute("data-poodle-drop-target")).toBe("accepted");

    rerender(<EditableList items={items.slice(0, 2)} onReorder={onReorder} />);
    expect(onReorder).not.toHaveBeenCalled();
    expect(container.querySelector("[data-poodle-drop-target]")).toBeNull();
  });

  it("shows the counter and hides the add row at maxItems", () => {
    const { container } = render(<EditableList items={items} editable maxItems={2} />);
    expect(container.querySelector(".poodle-editable-list__count")?.textContent).toBe("3/2");
    expect(container.querySelector(".poodle-editable-list__add")).toBeNull();
  });

  it("renders workflow chrome with dirty-gated submit", async () => {
    const onSubmit = vi.fn().mockResolvedValue(undefined);
    const onCancel = vi.fn();
    const { container } = render(
      <EditableList items={items} onSubmit={onSubmit} onCancel={onCancel} submitLabel="Save Order" />,
    );
    const submit = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Save Order"),
    ) as HTMLButtonElement;
    expect(submit.disabled).toBe(true);

    const dirty = render(
      <EditableList items={items} onSubmit={onSubmit} dirty submitLabel="Save Order" />,
    );
    const enabledSubmit = [...dirty.container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Save Order"),
    ) as HTMLButtonElement;
    fireEvent.click(enabledSubmit);
    expect(onSubmit).toHaveBeenCalledTimes(1);
  });

  it("renders the error surface as an alert and the info surface as a status", () => {
    const { container } = render(
      <EditableList items={items} errorMessage="Save failed" infoMessage="Unsaved changes" />,
    );
    const error = container.querySelector('[role="alert"]') as HTMLElement;
    expect(error.textContent).toContain("Save failed");
    const info = container.querySelector('[role="status"]') as HTMLElement;
    expect(info.textContent).toContain("Unsaved changes");
  });
});