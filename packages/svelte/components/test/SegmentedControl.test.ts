import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import SegmentedControl from "../src/SegmentedControl.svelte";
import type { SegmentedControlOption } from "../src/types";

const options: SegmentedControlOption[] = [
  { value: "grid", label: "Grid" },
  { value: "list", label: "List" },
  { value: "table", label: "Table" },
];

describe("SegmentedControl (svelte)", () => {
  it("selects a segment on click and reports the value", async () => {
    const onValueChange = vi.fn();
    const { container } = render(SegmentedControl, {
      props: { options, defaultValue: "grid", ariaLabel: "View mode", onValueChange },
    });
    const inputs = [...container.querySelectorAll<HTMLInputElement>(".poodle-segmented-control__control")];
    const segments = [...container.querySelectorAll(".poodle-segmented-control__segment")];

    await fireEvent.click(inputs[1]);

    expect(onValueChange).toHaveBeenCalledWith("list");
    expect(inputs[1].checked).toBe(true);
    expect(inputs[0].checked).toBe(false);
    expect(segments[1].getAttribute("data-selected")).toBe("true");
    expect(segments[0].getAttribute("data-selected")).toBe("false");
  });

  it("keeps disabled options and a disabled group inert", async () => {
    const onValueChange = vi.fn();
    const withDisabled = render(SegmentedControl, {
      props: {
        options: [{ ...options[0] }, { value: "archived", label: "Archived", disabled: true }],
        defaultValue: "grid",
        onValueChange,
      },
    });
    const disabledInput = withDisabled.container.querySelector<HTMLInputElement>(
      '.poodle-segmented-control__control[value="archived"]',
    )!;
    expect(disabledInput.disabled).toBe(true);
    await fireEvent.click(disabledInput);
    expect(onValueChange).not.toHaveBeenCalled();

    const group = render(SegmentedControl, {
      props: { options, disabled: true, defaultValue: "grid", onValueChange },
    });
    const inputs = [...group.container.querySelectorAll<HTMLInputElement>(".poodle-segmented-control__control")];
    expect(inputs.every((input) => input.disabled)).toBe(true);
    await fireEvent.click(inputs[1]);
    expect(onValueChange).not.toHaveBeenCalled();
  });

  it("exposes radiogroup semantics with hidden radios sharing one group name", () => {
    const { container } = render(SegmentedControl, {
      props: { options, value: "grid", ariaLabel: "View mode" },
    });
    const root = container.querySelector(".poodle-segmented-control")!;
    const inputs = [...container.querySelectorAll<HTMLInputElement>(".poodle-segmented-control__control")];

    expect(root.getAttribute("role")).toBe("radiogroup");
    expect(root.getAttribute("aria-label")).toBe("View mode");
    expect(inputs.every((input) => input.type === "radio")).toBe(true);
    const names = new Set(inputs.map((input) => input.name));
    expect(names.size).toBe(1);
    expect([...names][0]).toMatch(/^poodle-segmented-control-\d+$/);
  });

  it("uses the per-option aria-label as the segment accessible name", () => {
    const { container } = render(SegmentedControl, {
      props: {
        options: [
          { value: "grid", label: "Grid", ariaLabel: "Grid view" },
          { value: "list", label: "List" },
        ],
        value: "grid",
        ariaLabel: "View mode",
      },
    });
    const input = container.querySelector<HTMLInputElement>('.poodle-segmented-control__control[value="grid"]')!;
    expect(input.getAttribute("aria-label")).toBe("Grid view");
  });
});
