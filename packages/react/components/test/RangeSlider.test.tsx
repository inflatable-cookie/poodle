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
