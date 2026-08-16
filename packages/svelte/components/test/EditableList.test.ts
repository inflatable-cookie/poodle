import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import EditableList from "../src/EditableList.svelte";

const items = [
  { id: "a", label: "Alpha" },
  { id: "b", label: "Beta" },
  { id: "c", label: "Gamma" },
];

describe("EditableList (svelte)", () => {
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
    expect(rows[0].getAttribute("draggable")).toBe("true");
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
    const rows = [...container.querySelectorAll('[role="option"]')];

    await fireEvent.keyDown(rows[0], { key: " " });
    await fireEvent.keyDown(rows[0], { key: "ArrowDown" });
    await fireEvent.keyDown(rows[0], { key: " " });

    expect(onReorder).toHaveBeenCalledTimes(1);
    const reordered = onReorder.mock.calls[0][0] as typeof items;
    expect(reordered.map((item) => item.id)).toEqual(["b", "a", "c"]);
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