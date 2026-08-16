import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { CardToggleGroup } from "../src/CardToggleGroup";

const items = [
  { value: "list", label: "List view", count: 12 },
  { value: "grid", label: "Grid view" },
  { value: "compact", label: "Compact view", disabled: true },
];

describe("CardToggleGroup (react)", () => {
  it("renders a labelled group with pressed-state buttons", () => {
    const { container } = render(
      <CardToggleGroup items={items} value="grid" ariaLabel="Choose a view" />,
    );
    const root = container.querySelector(".poodle-card-toggle-group") as HTMLElement;
    expect(root.getAttribute("role")).toBe("group");
    expect(root.getAttribute("aria-label")).toBe("Choose a view");

    const options = [...container.querySelectorAll('[role="button"]')];
    expect(options[1].getAttribute("aria-pressed")).toBe("true");
    expect(options[0].getAttribute("aria-pressed")).toBe("false");
    expect(options[2].getAttribute("aria-disabled")).toBe("true");
  });

  it("renders the count badge when provided", () => {
    const { container } = render(<CardToggleGroup items={items} value="list" />);
    expect(container.querySelector(".poodle-card-toggle-group__count")?.textContent).toContain(
      "12",
    );
  });

  it("reports the next value on toggle and stays selected without deactivation", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <CardToggleGroup items={items} value="list" onValueChange={onValueChange} />,
    );
    const options = [...container.querySelectorAll('[role="button"]')];
    fireEvent.click(options[1]);
    expect(onValueChange).toHaveBeenCalledWith("grid");
  });

  it("clears the value when the active card is toggled with deactivation allowed", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <CardToggleGroup
        items={items}
        value="list"
        allowDeactivation
        onValueChange={onValueChange}
      />,
    );
    const options = [...container.querySelectorAll('[role="button"]')];
    fireEvent.click(options[0]);
    expect(onValueChange).toHaveBeenCalledWith(null);
  });

  it("keeps the active card selected when deactivation is not allowed", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <CardToggleGroup items={items} value="list" onValueChange={onValueChange} />,
    );
    const options = [...container.querySelectorAll('[role="button"]')];
    fireEvent.click(options[0]);
    // Card buttons re-emit on reselect (toggle-group semantics); the value
    // stays "list" and the option remains pressed.
    expect(onValueChange).toHaveBeenCalledWith("list");
    expect(options[0].getAttribute("aria-pressed")).toBe("true");
  });

  it("never activates disabled items", () => {
    const onValueChange = vi.fn();
    const { container } = render(<CardToggleGroup items={items} onValueChange={onValueChange} />);
    const options = [...container.querySelectorAll('[role="button"]')];
    fireEvent.click(options[2]);
    expect(onValueChange).not.toHaveBeenCalled();
  });

  it("toggles with Enter and Space on the focused option", () => {
    const onValueChange = vi.fn();
    const { container } = render(<CardToggleGroup items={items} onValueChange={onValueChange} />);
    const options = [...container.querySelectorAll('[role="button"]')];
    fireEvent.keyDown(options[0], { key: " " });
    expect(onValueChange).toHaveBeenCalledWith("list");
  });
});