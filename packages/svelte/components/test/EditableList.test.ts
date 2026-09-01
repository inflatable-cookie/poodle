import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import EditableList from "../src/EditableList.svelte";
import EditableListEmbeddedFixture from "./EditableListEmbeddedFixture.svelte";

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

function pointer(
  type: "pointerdown" | "pointermove" | "pointerup",
  init: PointerEventInit,
): PointerEvent {
  return new PointerEvent(type, {
    bubbles: true,
    cancelable: true,
    pointerId: 1,
    pointerType: "mouse",
    button: 0,
    buttons: type === "pointerdown" || type === "pointermove" ? 1 : 0,
    isPrimary: true,
    clientX: 0,
    clientY: 0,
    ...init,
  });
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

describe("EditableList (svelte)", () => {
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
    const { container } = render(EditableList, {
      props: { items, ariaLabel: "Tags" },
    });
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
    const { container } = render(EditableList, {
      props: { items, reorderable: false },
    });
    expect(container.querySelector(".poodle-editable-list__handle")).toBeNull();
    expect(container.querySelector('[role="option"]')?.getAttribute("draggable")).toBe("false");
  });

  it("adds an item through the input and reports it", async () => {
    const onAdd = vi.fn();
    const onChange = vi.fn();
    const { container } = render(EditableList, {
      props: { items, editable: true, addLabel: "Add tag", onAdd, onChange },
    });
    const input = container.querySelector("input") as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "Delta" } });
    const add = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Add tag"),
    ) as HTMLButtonElement;
    await fireEvent.click(add);

    expect(onAdd).toHaveBeenCalledTimes(1);
    const added = onAdd.mock.calls[0][0] as { label: string };
    expect(added.label).toBe("Delta");
    expect(onChange).toHaveBeenCalledTimes(1);
    expect((onChange.mock.calls[0][0] as typeof items).length).toBe(4);
  });

  it("disables the add button while the input is empty", () => {
    const { container } = render(EditableList, {
      props: { items, editable: true, addLabel: "Add tag" },
    });
    const add = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Add tag"),
    ) as HTMLButtonElement;
    expect(add.disabled).toBe(true);
  });

  it("removes an item and reports the id", async () => {
    const onRemove = vi.fn();
    const { container } = render(EditableList, {
      props: { items, editable: true, onRemove },
    });
    const remove = container.querySelector(
      'button[aria-label="Remove Beta"]',
    ) as HTMLButtonElement;
    await fireEvent.click(remove);
    expect(onRemove).toHaveBeenCalledWith("b");
  });

  it("reorders with keyboard grab, move, and drop", async () => {
    const onReorder = vi.fn();
    const { container } = render(EditableList, { props: { items, onReorder } });
    const rows = stackRows(container);
    rows[0].focus();

    await fireEvent.keyDown(rows[0], { key: " " });
    await fireEvent.keyDown(rows[0], { key: "ArrowDown" });
    await fireEvent.keyDown(rows[0], { key: " " });

    expect(onReorder).toHaveBeenCalledTimes(1);
    const reordered = onReorder.mock.calls[0][0] as typeof items;
    expect(reordered.map((item) => item.id)).toEqual(["b", "a", "c"]);
  });

  it("reorders upward with keyboard previous as before", async () => {
    const onReorder = vi.fn();
    const { container } = render(EditableList, { props: { items, onReorder } });
    const rows = stackRows(container);
    rows[1].focus();

    await fireEvent.keyDown(rows[1], { key: " " });
    await fireEvent.keyDown(rows[1], { key: "ArrowUp" });
    await fireEvent.keyDown(rows[1], { key: " " });

    expect((onReorder.mock.calls[0][0] as typeof items).map((item) => item.id)).toEqual(["b", "a", "c"]);
  });

  it("reorders across a windowSize boundary without paging first", async () => {
    const onReorder = vi.fn();
    const { container } = render(EditableList, { props: { items, windowSize: 2, onReorder } });
    const rows = stackRows(container);
    expect(rows).toHaveLength(2);
    rows[1].focus();

    await fireEvent.keyDown(rows[1], { key: " " });
    await fireEvent.keyDown(rows[1], { key: "ArrowDown" });
    await fireEvent.keyDown(rows[1], { key: " " });

    expect((onReorder.mock.calls[0][0] as typeof items).map((item) => item.id)).toEqual(["a", "c", "b"]);
  });

  it("reorders backward across a windowSize boundary from the second page", async () => {
    const onReorder = vi.fn();
    const { container } = render(EditableList, { props: { items, windowSize: 2, onReorder } });
    const next = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Next"),
    ) as HTMLButtonElement;
    await fireEvent.click(next);
    const rows = stackRows(container);
    rows[0].focus();

    await fireEvent.keyDown(rows[0], { key: " " });
    await fireEvent.keyDown(rows[0], { key: "ArrowUp" });
    await fireEvent.keyDown(rows[0], { key: " " });

    expect((onReorder.mock.calls[0][0] as typeof items).map((item) => item.id)).toEqual(["a", "c", "b"]);
  });

  it("reorders with a pointer drag from the handle", async () => {
    const onReorder = vi.fn();
    const { container } = render(EditableList, { props: { items, onReorder } });
    const rows = stackRows(container);
    const handle = rows[0].querySelector(".poodle-editable-list__handle") as HTMLElement;

    await fireEvent.pointerDown(handle, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    await fireEvent.pointerMove(handle, { pointerId: 1, clientX: 20, clientY: 110 });
    await fireEvent.pointerUp(handle, { pointerId: 1, clientX: 20, clientY: 110 });

    expect(onReorder).toHaveBeenCalledTimes(1);
    expect((onReorder.mock.calls[0][0] as typeof items).map((item) => item.id)).toEqual(["b", "c", "a"]);
  });

  it("lands at the hovered row even on the origin-facing half", async () => {
    const onReorder = vi.fn();
    const { container } = render(EditableList, { props: { items, onReorder } });
    const rows = stackRows(container);
    const handle = rows[0].querySelector(".poodle-editable-list__handle") as HTMLElement;

    // Beta sits at top=50, height=32. y=60 is its origin-facing half, which
    // used to resolve `before` and no-op the move back to Alpha's slot.
    await fireEvent.pointerDown(handle, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    await fireEvent.pointerMove(handle, { pointerId: 1, clientX: 20, clientY: 60 });
    await fireEvent.pointerUp(handle, { pointerId: 1, clientX: 20, clientY: 60 });

    expect(onReorder).toHaveBeenCalledTimes(1);
    expect((onReorder.mock.calls[0][0] as typeof items).map((item) => item.id)).toEqual(["b", "a", "c"]);
  });

  it("reorders with a touch hold", async () => {
    vi.useFakeTimers();
    const onReorder = vi.fn();
    const { container } = render(EditableList, { props: { items, onReorder } });
    const rows = stackRows(container);
    const handle = rows[0].querySelector(".poodle-editable-list__handle") as HTMLElement;

    handle.dispatchEvent(pointer("pointerdown", { pointerType: "touch", clientX: 20, clientY: 20 }));
    vi.advanceTimersByTime(250);
    handle.dispatchEvent(pointer("pointermove", { pointerType: "touch", clientX: 20, clientY: 110 }));
    handle.dispatchEvent(pointer("pointerup", { pointerType: "touch", clientX: 20, clientY: 110 }));

    expect(onReorder).toHaveBeenCalledTimes(1);
    expect((onReorder.mock.calls[0][0] as typeof items).map((item) => item.id)).toEqual(["b", "c", "a"]);
  });

  it("cancels touch when movement exceeds tolerance before the hold", async () => {
    vi.useFakeTimers();
    const onReorder = vi.fn();
    const { container } = render(EditableList, { props: { items, onReorder } });
    const rows = stackRows(container);
    const handle = rows[0].querySelector(".poodle-editable-list__handle") as HTMLElement;

    handle.dispatchEvent(pointer("pointerdown", { pointerType: "touch", clientX: 20, clientY: 20 }));
    handle.dispatchEvent(pointer("pointermove", { pointerType: "touch", clientX: 20, clientY: 40 }));
    vi.advanceTimersByTime(250);
    handle.dispatchEvent(pointer("pointerup", { pointerType: "touch", clientX: 20, clientY: 40 }));

    expect(onReorder).not.toHaveBeenCalled();
    expect(rows[0].hasAttribute("data-poodle-drag-source")).toBe(false);
  });

  it("cancels a keyboard grab on Escape without reordering", async () => {
    const onReorder = vi.fn();
    const { container } = render(EditableList, { props: { items, onReorder } });
    const rows = stackRows(container);
    rows[0].focus();

    await fireEvent.keyDown(rows[0], { key: " " });
    await fireEvent.keyDown(rows[0], { key: "ArrowDown" });
    await fireEvent.keyDown(rows[0], { key: "Escape" });

    expect(onReorder).not.toHaveBeenCalled();
    expect(rows[0].hasAttribute("data-poodle-drag-source")).toBe(false);
  });

  it("does not drag from a remove button", async () => {
    const onReorder = vi.fn();
    const onRemove = vi.fn();
    const { container } = render(EditableList, { props: { items, editable: true, onReorder, onRemove } });
    const rows = stackRows(container);
    const remove = container.querySelector('button[aria-label="Remove Alpha"]') as HTMLButtonElement;

    await fireEvent.pointerDown(remove, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    await fireEvent.pointerMove(remove, { pointerId: 1, clientX: 20, clientY: 110 });
    await fireEvent.pointerUp(remove, { pointerId: 1, clientX: 20, clientY: 110 });
    await fireEvent.click(remove);

    expect(onReorder).not.toHaveBeenCalled();
    expect(onRemove).toHaveBeenCalledWith("a");
    expect(rows[0].hasAttribute("data-poodle-drag-source")).toBe(false);
  });

  it("does not start an embedded-handle drag from editing or action descendants", async () => {
    const onReorder = vi.fn();
    const { container } = render(EditableListEmbeddedFixture, { props: { items, onReorder } });
    const rows = stackRows(container);
    const editor = container.querySelector('[data-testid="edit-a"]') as HTMLElement;
    const action = container.querySelector('[data-testid="action-a"]') as HTMLElement;

    editor.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    editor.dispatchEvent(pointer("pointermove", { clientX: 20, clientY: 110 }));
    editor.dispatchEvent(pointer("pointerup", { clientX: 20, clientY: 110 }));
    action.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    action.dispatchEvent(pointer("pointermove", { clientX: 20, clientY: 110 }));
    action.dispatchEvent(pointer("pointerup", { clientX: 20, clientY: 110 }));

    expect(onReorder).not.toHaveBeenCalled();
    expect(rows[0].hasAttribute("data-poodle-drag-source")).toBe(false);
  });

  it("keeps disabled rows inert to pointer and keyboard sensors", async () => {
    const onReorder = vi.fn();
    const { container } = render(EditableList, { props: { items, disabled: true, onReorder } });
    const rows = stackRows(container);
    const handle = rows[0].querySelector(".poodle-editable-list__handle") as HTMLElement;
    rows[0].focus();

    await fireEvent.pointerDown(handle, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    await fireEvent.pointerMove(handle, { pointerId: 1, clientX: 20, clientY: 110 });
    await fireEvent.pointerUp(handle, { pointerId: 1, clientX: 20, clientY: 110 });
    await fireEvent.keyDown(rows[0], { key: " " });
    await fireEvent.keyDown(rows[0], { key: "ArrowDown" });

    expect(onReorder).not.toHaveBeenCalled();
  });

  it("cancels when the dragging source is removed", async () => {
    const onReorder = vi.fn();
    const { container, rerender } = render(EditableList, { props: { items, onReorder } });
    const rows = stackRows(container);
    const handle = rows[0].querySelector(".poodle-editable-list__handle") as HTMLElement;

    await fireEvent.pointerDown(handle, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    await fireEvent.pointerMove(handle, { pointerId: 1, clientX: 20, clientY: 50 });
    expect(rows[0].getAttribute("data-poodle-drag-source")).toBe("dragging");

    await rerender({ items: items.slice(1), onReorder });
    expect(onReorder).not.toHaveBeenCalled();
    expect(container.querySelector("[data-poodle-drag-source]")).toBeNull();
  });

  it("cancels when the current target is removed", async () => {
    const onReorder = vi.fn();
    const { container, rerender } = render(EditableList, { props: { items, onReorder } });
    const rows = stackRows(container);
    const handle = rows[0].querySelector(".poodle-editable-list__handle") as HTMLElement;

    await fireEvent.pointerDown(handle, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    await fireEvent.pointerMove(handle, { pointerId: 1, clientX: 20, clientY: 110 });
    expect(rows[2].getAttribute("data-poodle-drop-target")).toBe("accepted");

    await rerender({ items: items.slice(0, 2), onReorder });
    expect(onReorder).not.toHaveBeenCalled();
    expect(container.querySelector("[data-poodle-drop-target]")).toBeNull();
  });

  it("shows the counter and hides the add row at maxItems", () => {
    const { container } = render(EditableList, {
      props: { items, editable: true, maxItems: 2 },
    });
    expect(container.querySelector(".poodle-editable-list__count")?.textContent).toBe("3/2");
    expect(container.querySelector(".poodle-editable-list__add")).toBeNull();
  });

  it("renders workflow chrome with dirty-gated submit", async () => {
    const onSubmit = vi.fn().mockResolvedValue(undefined);
    const onCancel = vi.fn();
    const { container } = render(EditableList, {
      props: { items, onSubmit, onCancel, submitLabel: "Save Order" },
    });
    const submit = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Save Order"),
    ) as HTMLButtonElement;
    expect(submit.disabled).toBe(true);

    const dirty = render(EditableList, {
      props: { items, onSubmit, dirty: true, submitLabel: "Save Order" },
    });
    const enabledSubmit = [...dirty.container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Save Order"),
    ) as HTMLButtonElement;
    await fireEvent.click(enabledSubmit);
    expect(onSubmit).toHaveBeenCalledTimes(1);
  });

  it("renders the error surface as an alert and the info surface as a status", () => {
    const { container } = render(EditableList, {
      props: { items, errorMessage: "Save failed", infoMessage: "Unsaved changes" },
    });
    const error = container.querySelector('[role="alert"]') as HTMLElement;
    expect(error.textContent).toContain("Save failed");
    const info = container.querySelector('[role="status"]') as HTMLElement;
    expect(info.textContent).toContain("Unsaved changes");
  });
});