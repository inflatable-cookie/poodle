import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { ToggleGroup } from "../src/ToggleGroup";

const options = [
  { value: "grid", label: "Grid" },
  { value: "list", label: "List" },
  { value: "board", label: "Board" },
];

describe("ToggleGroup (react)", () => {
  it("renders single mode as a radiogroup with radio items", () => {
    const { container } = render(
      <ToggleGroup options={options} value="grid" ariaLabel="View mode" />,
    );
    const root = container.querySelector(".poodle-toggle-group") as HTMLElement;
    expect(root.getAttribute("role")).toBe("radiogroup");
    expect(root.getAttribute("aria-label")).toBe("View mode");

    const items = Array.from(root.querySelectorAll("button"));
    expect(items.map((item) => item.getAttribute("role"))).toEqual(["radio", "radio", "radio"]);
    expect(items.map((item) => item.getAttribute("aria-checked"))).toEqual([
      "true",
      "false",
      "false",
    ]);
  });

  it("renders multiple mode as a group with pressed buttons", () => {
    const { container } = render(
      <ToggleGroup options={options} value={["grid", "board"]} selectionMode="multiple" ariaLabel="Filter tags" />,
    );
    const root = container.querySelector(".poodle-toggle-group") as HTMLElement;
    expect(root.getAttribute("role")).toBe("group");

    const items = Array.from(root.querySelectorAll("button"));
    expect(items.map((item) => item.getAttribute("role"))).toEqual(["button", "button", "button"]);
    expect(items.map((item) => item.getAttribute("aria-pressed"))).toEqual([
      "true",
      "false",
      "true",
    ]);
  });

  it("emits value changes on item clicks in single and multiple modes", () => {
    const onValueChange = vi.fn();
    const { container } = render(<ToggleGroup options={options} onValueChange={onValueChange} />);

    const items = Array.from(
      container.querySelectorAll(".poodle-toggle-group__item"),
    ) as HTMLButtonElement[];
    fireEvent.click(items[1]);
    expect(onValueChange).toHaveBeenCalledWith("list");

    onValueChange.mockClear();
    const multi = render(
      <ToggleGroup options={options} selectionMode="multiple" onValueChange={onValueChange} />,
    );
    const multiItems = Array.from(
      multi.container.querySelectorAll(".poodle-toggle-group__item"),
    ) as HTMLButtonElement[];
    fireEvent.click(multiItems[0]);
    fireEvent.click(multiItems[2]);
    expect(onValueChange).toHaveBeenCalledWith(["grid", "board"]);
  });

  it("clears a single-mode selection to null only when allowDeactivation is set", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <ToggleGroup options={options} value="grid" allowDeactivation onValueChange={onValueChange} />,
    );
    const items = Array.from(
      container.querySelectorAll(".poodle-toggle-group__item"),
    ) as HTMLButtonElement[];

    fireEvent.click(items[0]);
    expect(onValueChange).toHaveBeenCalledWith(null);

    onValueChange.mockClear();
    const without = render(<ToggleGroup options={options} value="grid" onValueChange={onValueChange} />);
    const withoutItems = Array.from(
      without.container.querySelectorAll(".poodle-toggle-group__item"),
    ) as HTMLButtonElement[];
    fireEvent.click(withoutItems[0]);
    expect(onValueChange).toHaveBeenCalledWith("grid");
  });

  it("disables the group and individual items", () => {
    const { container } = render(
      <ToggleGroup
        options={[{ value: "a", label: "A" }, { value: "b", label: "B", disabled: true }]}
        disabled
      />,
    );
    const items = Array.from(
      container.querySelectorAll(".poodle-toggle-group__item"),
    ) as HTMLButtonElement[];
    expect(items.every((item) => item.disabled)).toBe(true);
  });
});