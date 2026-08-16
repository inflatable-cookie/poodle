import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Slider from "../src/Slider.svelte";

describe("Slider (svelte)", () => {
  it("snaps live input to step and commits the same value on release", async () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(Slider, {
      props: { value: 50, step: 10, ariaLabel: "Volume", onValueChange, onValueCommit },
    });
    const input = container.querySelector<HTMLInputElement>(".poodle-slider__control")!;

    await fireEvent.input(input, { target: { value: "63" } });
    expect(onValueChange).toHaveBeenCalledWith(60);

    await fireEvent.change(input, { target: { value: "63" } });
    expect(onValueCommit).toHaveBeenCalledWith(60);
  });

  it("clamps out-of-range input into the min/max window", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Slider, {
      props: { value: 50, min: 0, max: 100, ariaLabel: "Volume", onValueChange },
    });
    const input = container.querySelector<HTMLInputElement>(".poodle-slider__control")!;

    await fireEvent.input(input, { target: { value: "-5" } });
    expect(onValueChange).toHaveBeenLastCalledWith(0);

    await fireEvent.input(input, { target: { value: "150" } });
    expect(onValueChange).toHaveBeenLastCalledWith(100);
  });

  it("drives the fill percentage custom property from the value", () => {
    const { container } = render(Slider, { props: { value: 65, ariaLabel: "Volume" } });
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("style")).toContain("--poodle-slider-percent: 65%");
  });

  it("applies the bounds guard when max is at or below min", () => {
    const { container } = render(Slider, { props: { value: 5, min: 10, max: 10, ariaLabel: "Volume" } });
    const input = container.querySelector<HTMLInputElement>(".poodle-slider__control")!;
    expect(input.getAttribute("max")).toBe("11");
  });

  it("disables the native control", () => {
    const { container } = render(Slider, { props: { value: 40, disabled: true, ariaLabel: "Volume" } });
    const input = container.querySelector<HTMLInputElement>(".poodle-slider__control")!;
    expect(input.disabled).toBe(true);
    expect(container.querySelector(".poodle-slider")!.getAttribute("data-disabled")).toBe("true");
  });
});
