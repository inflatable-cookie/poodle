import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import RelationPicker from "../src/RelationPicker.svelte";
import type { DrillDownConfig, PickerItem } from "../src/types";

const items: PickerItem[] = [
  { id: "btn", label: "Button", description: "Primary control" },
  { id: "card", label: "Card", meta: "Container" },
  { id: "locked", label: "Locked", disabled: true },
];

describe("RelationPicker (svelte)", () => {
  it("renders the candidate list with the available-candidates label", () => {
    const { container } = render(RelationPicker, { props: { title: "Select components", items } });
    const list = container.querySelector(
      '.poodle-relation-picker__list[aria-label="Available candidates"]',
    );
    expect(list).not.toBeNull();
    expect(container.querySelectorAll(".poodle-relation-picker__item").length).toBe(3);
  });

  it("toggles selection in multiple mode and reports the ids", async () => {
    const onSelectionChange = vi.fn();
    const { container } = render(RelationPicker, {
      props: { items, onSelectionChange },
    });
    const firstButton = container.querySelector(
      ".poodle-relation-picker__item-button",
    ) as HTMLButtonElement;
    await fireEvent.click(firstButton);
    expect(onSelectionChange).toHaveBeenCalledWith(["btn"]);

    await fireEvent.click(firstButton);
    expect(onSelectionChange).toHaveBeenCalledWith([]);
  });

  it("selects exactly one item in single mode", async () => {
    const onSelectionChange = vi.fn();
    const { container } = render(RelationPicker, {
      props: { items, selectionMode: "single", onSelectionChange },
    });
    const buttons = [...container.querySelectorAll(".poodle-relation-picker__item-button")];
    await fireEvent.click(buttons[0]);
    expect(onSelectionChange).toHaveBeenCalledWith(["btn"]);
    await fireEvent.click(buttons[1]);
    expect(onSelectionChange).toHaveBeenCalledWith(["card"]);
  });

  it("never toggles a disabled candidate", async () => {
    const onSelectionChange = vi.fn();
    const { container } = render(RelationPicker, {
      props: { items, onSelectionChange },
    });
    const disabledRow = [...container.querySelectorAll(".poodle-relation-picker__item")].find(
      (row) => row.getAttribute("data-disabled") === "true",
    ) as HTMLElement;
    expect(disabledRow).not.toBeNull();
    await fireEvent.click(disabledRow.querySelector("button") as HTMLButtonElement);
    expect(onSelectionChange).not.toHaveBeenCalled();
  });

  it("marks selected rows with data-selected and reports the confirm ids", async () => {
    const onConfirm = vi.fn();
    const { container } = render(RelationPicker, {
      props: { items, selectedIds: ["card"], onConfirm },
    });
    const rows = [...container.querySelectorAll(".poodle-relation-picker__item")];
    expect(rows[1].getAttribute("data-selected")).toBe("true");

    const confirm = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Confirm selection"),
    ) as HTMLButtonElement;
    await fireEvent.click(confirm);
    expect(onConfirm).toHaveBeenCalledWith(["card"]);
  });

  it("reports the cancel action", async () => {
    const onCancel = vi.fn();
    const { container } = render(RelationPicker, { props: { items, onCancel } });
    const cancel = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Cancel"),
    ) as HTMLButtonElement;
    await fireEvent.click(cancel);
    expect(onCancel).toHaveBeenCalledTimes(1);
  });

  it("reports query changes from the search field", async () => {
    const onQueryChange = vi.fn();
    const { container } = render(RelationPicker, { props: { items, onQueryChange } });
    const search = container.querySelector("#relation-picker-search") as HTMLInputElement;
    await fireEvent.input(search, { target: { value: "card" } });
    expect(onQueryChange).toHaveBeenCalledWith("card");
  });

  it("shows the selection summary only when something is selected", () => {
    const none = render(RelationPicker, { props: { items } });
    expect(none.container.querySelector(".poodle-selection-summary")).toBeNull();

    const selected = render(RelationPicker, { props: { items, selectedIds: ["btn"] } });
    expect(selected.container.querySelector(".poodle-selection-summary")).not.toBeNull();
  });

  it("drills into the first level and advances through drill selection", async () => {
    const drillDown: DrillDownConfig = {
      levels: [
        {
          key: "category",
          label: "Category",
          items: [
            { id: "cat-a", label: "Category A", count: 3 },
            { id: "cat-b", label: "Category B", count: 1 },
          ],
        },
      ],
      finalItems: (query) =>
        Promise.resolve(
          items.filter((item) =>
            query ? item.label.toLowerCase().includes(query.toLowerCase()) : true,
          ),
        ),
    };
    const onDrillContext = vi.fn();
    const { container } = render(RelationPicker, {
      props: { items, drillDown, onDrillContext },
    });

    expect(container.querySelector(".poodle-drill-level-label")?.textContent).toContain(
      "Category",
    );
    const drillButtons = [...container.querySelectorAll(".poodle-drill-list__button")];
    expect(drillButtons.length).toBe(2);

    await fireEvent.click(drillButtons[0]);
    expect(onDrillContext).toHaveBeenCalledWith({ category: "cat-a" });

    // Post-drill: breadcrumbs with the chosen level and the final candidate list.
    expect(container.querySelector(".poodle-drill-breadcrumbs")).not.toBeNull();
    expect(container.querySelector(".poodle-relation-picker__list")).not.toBeNull();
  });
});