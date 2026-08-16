import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { SelectionSummary } from "../src/SelectionSummary";

const items = [
  { id: "button", label: "Button" },
  { id: "card", label: "Card" },
  { id: "dialog", label: "Dialog" },
  { id: "table", label: "Table" },
  { id: "tabs", label: "Tabs" },
];

describe("SelectionSummary (react)", () => {
  it("shows the empty placeholder and no clear link when nothing is selected", () => {
    const { container } = render(<SelectionSummary items={[]} />);
    expect(container.querySelector(".poodle-selection-summary__empty")?.textContent).toBe(
      "No selection",
    );
    expect(container.querySelector(".poodle-selection-summary__clear")).toBeNull();
  });

  it("renders removable chips and reports the removed id", () => {
    const onRemove = vi.fn();
    const { container } = render(<SelectionSummary items={items.slice(0, 2)} onRemove={onRemove} />);
    const chips = [...container.querySelectorAll(".poodle-selection-summary__chip")];
    expect(chips.length).toBe(2);
    expect(chips[0].getAttribute("aria-label")).toBe("Remove Button");

    fireEvent.click(chips[1]);
    expect(onRemove).toHaveBeenCalledWith("card");
  });

  it("truncates to maxVisibleItems and shows the overflow count", () => {
    const { container } = render(<SelectionSummary items={items} maxVisibleItems={3} />);
    expect(container.querySelectorAll(".poodle-selection-summary__chip").length).toBe(3);
    expect(container.querySelector(".poodle-selection-summary__overflow")?.textContent).toBe(
      "+2 more",
    );
  });

  it("reports the clear action from the inline clear link", () => {
    const onClear = vi.fn();
    const { container } = render(
      <SelectionSummary items={items.slice(0, 2)} onClear={onClear} />,
    );
    const clear = container.querySelector(".poodle-selection-summary__clear") as HTMLAnchorElement;
    fireEvent.click(clear);
    expect(onClear).toHaveBeenCalledTimes(1);
  });

  it("splits chips into activation and remove controls when onActivate is set", () => {
    const onActivate = vi.fn();
    const onRemove = vi.fn();
    const { container } = render(
      <SelectionSummary items={items.slice(0, 1)} onActivate={onActivate} onRemove={onRemove} />,
    );
    const chip = container.querySelector(".poodle-selection-summary__chip--split") as HTMLElement;
    expect(chip).not.toBeNull();

    const activate = chip.querySelector(
      ".poodle-selection-summary__chip-activate",
    ) as HTMLButtonElement;
    expect(activate.getAttribute("aria-label")).toBe("Edit Button");
    fireEvent.click(activate);
    expect(onActivate).toHaveBeenCalledWith("button");

    const remove = chip.querySelector(
      ".poodle-selection-summary__chip-remove",
    ) as HTMLButtonElement;
    expect(remove.getAttribute("aria-label")).toBe("Remove Button");
    fireEvent.click(remove);
    expect(onRemove).toHaveBeenCalledWith("button");
  });
});
