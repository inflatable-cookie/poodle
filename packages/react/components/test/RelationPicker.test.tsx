import { act, fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { RelationPicker } from "../src/RelationPicker";
import type { DrillDownConfig, PickerItem } from "../src/types";

const items: PickerItem[] = [
  { id: "btn", label: "Button", description: "Primary control" },
  { id: "card", label: "Card", meta: "Container" },
  { id: "locked", label: "Locked", disabled: true },
];

describe("RelationPicker (react)", () => {
  it("renders the candidate list with the available-candidates label", () => {
    const { container } = render(<RelationPicker title="Select components" items={items} />);
    const list = container.querySelector(
      '.poodle-relation-picker__list[aria-label="Available candidates"]',
    );
    expect(list).not.toBeNull();
    expect(container.querySelectorAll(".poodle-relation-picker__item").length).toBe(3);
  });

  it("toggles selection in multiple mode and reports the ids", () => {
    const onSelectionChange = vi.fn();
    const { container } = render(<RelationPicker items={items} onSelectionChange={onSelectionChange} />);
    const firstButton = container.querySelector(
      ".poodle-relation-picker__item-button",
    ) as HTMLButtonElement;
    fireEvent.click(firstButton);
    expect(onSelectionChange).toHaveBeenCalledWith(["btn"]);

    fireEvent.click(firstButton);
    expect(onSelectionChange).toHaveBeenCalledWith([]);
  });

  it("selects exactly one item in single mode", () => {
    const onSelectionChange = vi.fn();
    const { container } = render(
      <RelationPicker items={items} selectionMode="single" onSelectionChange={onSelectionChange} />,
    );
    const buttons = [...container.querySelectorAll(".poodle-relation-picker__item-button")];
    fireEvent.click(buttons[0]);
    expect(onSelectionChange).toHaveBeenCalledWith(["btn"]);
    fireEvent.click(buttons[1]);
    expect(onSelectionChange).toHaveBeenCalledWith(["card"]);
  });

  it("never toggles a disabled candidate", () => {
    const onSelectionChange = vi.fn();
    const { container } = render(<RelationPicker items={items} onSelectionChange={onSelectionChange} />);
    const disabledRow = [...container.querySelectorAll(".poodle-relation-picker__item")].find(
      (row) => row.getAttribute("data-disabled") === "true",
    ) as HTMLElement;
    expect(disabledRow).not.toBeNull();
    fireEvent.click(disabledRow.querySelector("button") as HTMLButtonElement);
    expect(onSelectionChange).not.toHaveBeenCalled();
  });

  it("marks selected rows with data-selected and reports the confirm ids", () => {
    const onConfirm = vi.fn();
    const { container } = render(
      <RelationPicker items={items} selectedIds={["card"]} onConfirm={onConfirm} />,
    );
    const rows = [...container.querySelectorAll(".poodle-relation-picker__item")];
    expect(rows[1].getAttribute("data-selected")).toBe("true");

    const confirm = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Confirm selection"),
    ) as HTMLButtonElement;
    fireEvent.click(confirm);
    expect(onConfirm).toHaveBeenCalledWith(["card"]);
  });

  it("reports the cancel action", () => {
    const onCancel = vi.fn();
    const { container } = render(<RelationPicker items={items} onCancel={onCancel} />);
    const cancel = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Cancel"),
    ) as HTMLButtonElement;
    fireEvent.click(cancel);
    expect(onCancel).toHaveBeenCalledTimes(1);
  });

  it("reports query changes from the search field", () => {
    const onQueryChange = vi.fn();
    const { container } = render(<RelationPicker items={items} onQueryChange={onQueryChange} />);
    const search = container.querySelector("#relation-picker-search") as HTMLInputElement;
    fireEvent.input(search, { target: { value: "card" } });
    expect(onQueryChange).toHaveBeenCalledWith("card");
  });

  it("shows the selection summary only when something is selected", () => {
    const none = render(<RelationPicker items={items} />);
    expect(none.container.querySelector(".poodle-selection-summary")).toBeNull();

    const selected = render(<RelationPicker items={items} selectedIds={["btn"]} />);
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
    const { container } = render(
      <RelationPicker items={items} drillDown={drillDown} onDrillContext={onDrillContext} />,
    );

    expect(container.querySelector(".poodle-drill-level-label")?.textContent).toContain("Category");
    const drillButtons = [...container.querySelectorAll(".poodle-drill-list__button")];
    expect(drillButtons.length).toBe(2);

    fireEvent.click(drillButtons[0]);
    // drillSelect advances depth synchronously; the finalItems promise resolves
    // in a microtask, so flush it inside act.
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });
    expect(onDrillContext).toHaveBeenCalledWith({ category: "cat-a" });

    // Post-drill: breadcrumbs with the chosen level and the final candidate list.
    expect(container.querySelector(".poodle-drill-breadcrumbs")).not.toBeNull();
    expect(container.querySelector(".poodle-relation-picker__list")).not.toBeNull();
  });
});