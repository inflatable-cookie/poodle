import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import ToggleGroup from "../src/ToggleGroup.svelte";

const options = [
  { value: "grid", label: "Grid" },
  { value: "list", label: "List" },
  { value: "board", label: "Board" },
];

const skipOptions = [
  { value: "grid", label: "Grid" },
  { value: "list", label: "List", disabled: true },
  { value: "board", label: "Board" },
];

function items(container: HTMLElement): HTMLButtonElement[] {
  return Array.from(container.querySelectorAll(".poodle-toggle-group__item")) as HTMLButtonElement[];
}

describe("ToggleGroup (svelte)", () => {
  it("renders single mode as a radiogroup with radio items", () => {
    const { container } = render(ToggleGroup, {
      props: { options, value: "grid", ariaLabel: "View mode" },
    });
    const root = container.querySelector(".poodle-toggle-group") as HTMLElement;
    expect(root.getAttribute("role")).toBe("radiogroup");
    expect(root.getAttribute("aria-label")).toBe("View mode");

    const buttons = Array.from(root.querySelectorAll("button"));
    expect(buttons.map((item) => item.getAttribute("role"))).toEqual(["radio", "radio", "radio"]);
    expect(buttons.map((item) => item.getAttribute("aria-checked"))).toEqual([
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

    const buttons = Array.from(root.querySelectorAll("button"));
    expect(buttons.map((item) => item.getAttribute("role"))).toEqual(["button", "button", "button"]);
    expect(buttons.map((item) => item.getAttribute("aria-pressed"))).toEqual([
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

    await fireEvent.click(items(container)[1]);
    expect(onValueChange).toHaveBeenCalledWith("list");

    onValueChange.mockClear();
    const multi = render(ToggleGroup, {
      props: { options, selectionMode: "multiple", onValueChange },
    });
    const multiItems = items(multi.container);
    await fireEvent.click(multiItems[0]);
    await fireEvent.click(multiItems[2]);
    expect(onValueChange).toHaveBeenCalledWith(["grid", "board"]);
  });

  it("clears a single-mode selection to null only when allowDeactivation is set", async () => {
    const onValueChange = vi.fn();
    const { container } = render(ToggleGroup, {
      props: { options, value: "grid", allowDeactivation: true, onValueChange },
    });

    await fireEvent.click(items(container)[0]);
    expect(onValueChange).toHaveBeenCalledWith(null);

    onValueChange.mockClear();
    const without = render(ToggleGroup, {
      props: { options, value: "grid", onValueChange },
    });
    await fireEvent.click(items(without.container)[0]);
    expect(onValueChange).toHaveBeenCalledWith("grid");
  });

  it("projects one selected or first-enabled tab stop in single mode", () => {
    const selected = render(ToggleGroup, { props: { options, value: "list" } });
    expect(items(selected.container).map((item) => item.tabIndex)).toEqual([-1, 0, -1]);

    const first = render(ToggleGroup, { props: { options } });
    expect(items(first.container).map((item) => item.tabIndex)).toEqual([0, -1, -1]);

    const skipped = render(ToggleGroup, {
      props: { options: skipOptions, value: "list" },
    });
    expect(items(skipped.container).map((item) => item.tabIndex)).toEqual([0, -1, -1]);
  });

  it("moves selection with Left/Right, wrapping and skipping disabled options", async () => {
    const onValueChange = vi.fn();
    const { container } = render(ToggleGroup, {
      props: { options: skipOptions, value: "grid", onValueChange },
    });
    const buttons = items(container);

    await fireEvent.keyDown(buttons[0], { key: "ArrowRight" });
    expect(onValueChange).toHaveBeenLastCalledWith("board");
    expect(document.activeElement).toBe(buttons[2]);

    onValueChange.mockClear();
    await fireEvent.keyDown(buttons[2], { key: "ArrowRight" });
    expect(onValueChange).toHaveBeenLastCalledWith("grid");
    expect(document.activeElement).toBe(buttons[0]);

    onValueChange.mockClear();
    await fireEvent.keyDown(buttons[0], { key: "ArrowLeft" });
    expect(onValueChange).toHaveBeenLastCalledWith("board");
  });

  it("keeps multiple mode as ordinary buttons and ignores arrows", async () => {
    const onValueChange = vi.fn();
    const { container } = render(ToggleGroup, {
      props: { options, value: ["grid"], selectionMode: "multiple", onValueChange },
    });
    const buttons = items(container);
    expect(buttons.every((item) => item.tabIndex === 0)).toBe(true);

    await fireEvent.keyDown(buttons[0], { key: "ArrowRight" });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(document.activeElement).not.toBe(buttons[1]);
  });

  it("keeps two same-valued groups independent", async () => {
    const leftChange = vi.fn();
    const rightChange = vi.fn();
    const left = render(ToggleGroup, {
      props: { options, value: "grid", onValueChange: leftChange },
    });
    const right = render(ToggleGroup, {
      props: { options, value: "grid", onValueChange: rightChange },
    });
    const leftItems = items(left.container);
    const rightItems = items(right.container);

    await fireEvent.keyDown(leftItems[0], { key: "ArrowRight" });
    expect(leftChange).toHaveBeenCalledWith("list");
    expect(rightChange).not.toHaveBeenCalled();
    expect(document.activeElement).toBe(leftItems[1]);
    expect(rightItems[0].tabIndex).toBe(0);
    expect(rightItems[1].tabIndex).toBe(-1);
  });

  it("disables the group and individual items", () => {
    const { container } = render(ToggleGroup, {
      props: {
        options: [{ value: "a", label: "A" }, { value: "b", label: "B", disabled: true }],
        disabled: true,
      },
    });
    const buttons = items(container);
    expect(buttons.every((item) => item.disabled)).toBe(true);
    expect(buttons.every((item) => item.tabIndex < 0 || item.disabled)).toBe(true);
  });
});
