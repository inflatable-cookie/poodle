import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { RangeSlider } from "../src/RangeSlider";

describe("RangeSlider (react)", () => {
  it("commits a standard thumb only after its value changed", () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const view = render(
      <RangeSlider
        value={[20, 80]}
        ariaLabel="Price range"
        onValueChange={onValueChange}
        onValueCommit={onValueCommit}
      />,
    );
    const lower = view.getByRole("slider", { name: "Price range minimum" });

    fireEvent.keyUp(lower, { key: "Tab" });
    fireEvent.mouseUp(lower);
    expect(onValueCommit).not.toHaveBeenCalled();

    fireEvent.input(lower, { target: { value: "25" } });
    fireEvent.keyUp(lower, { key: "ArrowRight" });
    expect(onValueChange).toHaveBeenCalledWith([25, 80]);
    expect(onValueCommit).toHaveBeenCalledOnce();
    expect(onValueCommit).toHaveBeenCalledWith([25, 80]);

    fireEvent.keyUp(lower, { key: "ArrowRight" });
    expect(onValueCommit).toHaveBeenCalledOnce();
  });
});

/**
 * g14.003 retained regression: the React shell shipped without the container
 * `role="group"` the Svelte shell had, so a screen reader heard two unrelated
 * sliders instead of one labelled range.
 */
describe("RangeSlider (react) semantics", () => {
  it("groups the two thumbs under one container", () => {
    const { container } = render(<RangeSlider value={[20, 80]} ariaLabel="Gain range" />);

    const group = container.querySelector('[role="group"]');
    expect(group).not.toBeNull();

    // The label lives on the thumbs, which is where the value is; the group is
    // what stops them being heard as two unrelated sliders. Standard thumbs are
    // real range inputs, so they carry the implicit slider role.
    const thumbs = [...group!.querySelectorAll('input[type="range"]')];
    expect(thumbs.map((thumb) => thumb.getAttribute("aria-label"))).toEqual([
      "Gain range minimum",
      "Gain range maximum",
    ]);
  });

  it("groups the embedded thumbs the same way", () => {
    const { container } = render(
      <RangeSlider value={[20, 80]} variant="embedded" ariaLabel="Gain range" />,
    );

    const group = container.querySelector('[role="group"]');
    expect(group).not.toBeNull();
    expect(group!.querySelectorAll('[role="slider"]').length).toBe(2);
  });
});
