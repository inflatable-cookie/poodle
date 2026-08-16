import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import SelectionSummary from "../src/SelectionSummary.svelte";

const items = [
  { id: "button", label: "Button" },
  { id: "card", label: "Card" },
  { id: "dialog", label: "Dialog" },
  { id: "table", label: "Table" },
  { id: "tabs", label: "Tabs" },
];

describe("SelectionSummary (svelte)", () => {
  it("shows the empty placeholder and no clear link when nothing is selected", () => {
    const { container } = render(SelectionSummary, { props: { items: [] } });
    expect(container.querySelector(".poodle-selection-summary__empty")?.textContent).toBe(
      "No selection",
    );
    expect(container.querySelector(".poodle-selection-summary__clear")).toBeNull();
  });

  it("renders removable chips and reports the removed id", async () => {
    const onRemove = vi.fn();
    const { container } = render(SelectionSummary, {
      props: { items: items.slice(0, 2), onRemove },
    });
    const chips = [...container.querySelectorAll(".poodle-selection-summary__chip")];
    expect(chips.length).toBe(2);
    expect(chips[0].getAttribute("aria-label")).toBe("Remove Button");

    await fireEvent.click(chips[1]);
    expect(onRemove).toHaveBeenCalledWith("card");
  });

  it("truncates to maxVisibleItems and shows the overflow count", () => {
    const { container } = render(SelectionSummary, {
      props: { items, maxVisibleItems: 3 },
    });
    expect(container.querySelectorAll(".poodle-selection-summary__chip").length).toBe(3);
    expect(container.querySelector(".poodle-selection-summary__overflow")?.textContent).toBe(
      "+2 more",
    );
  });

  it("reports the clear action from the inline clear link", async () => {
    const onClear = vi.fn();
    const { container } = render(SelectionSummary, {
      props: { items: items.slice(0, 2), onClear },
    });
    const clear = container.querySelector(".poodle-selection-summary__clear") as HTMLAnchorElement;
    await fireEvent.click(clear);
    expect(onClear).toHaveBeenCalledTimes(1);
  });

  it("splits chips into activation and remove controls when onActivate is set", async () => {
    const onActivate = vi.fn();
    const onRemove = vi.fn();
    const { container } = render(SelectionSummary, {
      props: { items: items.slice(0, 1), onActivate, onRemove },
    });
    const chip = container.querySelector(".poodle-selection-summary__chip--split") as HTMLElement;
    expect(chip).not.toBeNull();

    const activate = chip.querySelector(
      ".poodle-selection-summary__chip-activate",
    ) as HTMLButtonElement;
    expect(activate.getAttribute("aria-label")).toBe("Edit Button");
    await fireEvent.click(activate);
    expect(onActivate).toHaveBeenCalledWith("button");

    const remove = chip.querySelector(
      ".poodle-selection-summary__chip-remove",
    ) as HTMLButtonElement;
    expect(remove.getAttribute("aria-label")).toBe("Remove Button");
    await fireEvent.click(remove);
    expect(onRemove).toHaveBeenCalledWith("button");
  });
});
