import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import ToggleGroup from "../src/ToggleGroup.svelte";

const options = [
  { value: "grid", label: "Grid" },
  { value: "list", label: "List" },
  { value: "board", label: "Board" },
];

describe("ToggleGroup (svelte)", () => {
  it("renders single mode as a radiogroup with radio items", () => {
    const { container } = render(ToggleGroup, {
      props: { options, value: "grid", ariaLabel: "View mode" },
    });
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
    const { container } = render(ToggleGroup, {
      props: { options, value: ["grid", "board"], selectionMode: "multiple", ariaLabel: "Filter tags" },
    });
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

  it("emits value changes on item clicks in single and multiple modes", async () => {
    const onValueChange = vi.fn();
    const { container } = render(ToggleGroup, {
      props: { options, onValueChange },
    });

    const items = Array.from(container.querySelectorAll(".poodle-toggle-group__item")) as HTMLButtonElement[];
    await fireEvent.click(items[1]);
    expect(onValueChange).toHaveBeenCalledWith("list");

    onValueChange.mockClear();
    const multi = render(ToggleGroup, {
      props: { options, selectionMode: "multiple", onValueChange },
    });
    const multiItems = Array.from(
      multi.container.querySelectorAll(".poodle-toggle-group__item"),
    ) as HTMLButtonElement[];
    await fireEvent.click(multiItems[0]);
    await fireEvent.click(multiItems[2]);
    expect(onValueChange).toHaveBeenCalledWith(["grid", "board"]);
  });

  it("clears a single-mode selection to null only when allowDeactivation is set", async () => {
    const onValueChange = vi.fn();
    const { container } = render(ToggleGroup, {
      props: { options, value: "grid", allowDeactivation: true, onValueChange },
    });
    const items = Array.from(container.querySelectorAll(".poodle-toggle-group__item")) as HTMLButtonElement[];

    await fireEvent.click(items[0]);
    expect(onValueChange).toHaveBeenCalledWith(null);

    onValueChange.mockClear();
    const without = render(ToggleGroup, {
      props: { options, value: "grid", onValueChange },
    });
    const withoutItems = Array.from(
      without.container.querySelectorAll(".poodle-toggle-group__item"),
    ) as HTMLButtonElement[];
    await fireEvent.click(withoutItems[0]);
    expect(onValueChange).toHaveBeenCalledWith("grid");
  });

  it("disables the group and individual items", () => {
    const { container } = render(ToggleGroup, {
      props: {
        options: [{ value: "a", label: "A" }, { value: "b", label: "B", disabled: true }],
        disabled: true,
      },
    });
    const items = Array.from(container.querySelectorAll(".poodle-toggle-group__item")) as HTMLButtonElement[];
    expect(items.every((item) => item.disabled)).toBe(true);
  });
});