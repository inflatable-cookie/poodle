import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import RangeSlider from "../src/RangeSlider.svelte";

describe("RangeSlider (svelte)", () => {
  it("reports live change while a thumb moves and commits once at release", async () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(RangeSlider, {
      props: {
        value: [20, 80],
        ariaLabel: "Price range",
        onValueChange,
        onValueCommit,
      },
    });
    const lower = container.querySelector<HTMLInputElement>(".poodle-range-slider__control--lower")!;

    await fireEvent.input(lower, { target: { value: "40" } });
    expect(onValueChange).toHaveBeenCalledWith([40, 80]);

    await fireEvent.change(lower, { target: { value: "40" } });
    expect(onValueCommit).toHaveBeenCalledOnce();
    expect(onValueCommit).toHaveBeenCalledWith([40, 80]);
  });

  it("keeps the lower<=upper invariant when a thumb crosses its sibling", async () => {
    const onValueChange = vi.fn();
    const { container } = render(RangeSlider, {
      props: { value: [20, 80], ariaLabel: "Price range", onValueChange },
    });
    const upper = container.querySelector<HTMLInputElement>(".poodle-range-slider__control--upper")!;

    await fireEvent.input(upper, { target: { value: "10" } });
    expect(onValueChange).toHaveBeenCalledWith([20, 20]);
  });

  it("exposes per-thumb labels, values, and bounds", () => {
    const { container } = render(RangeSlider, {
      props: { value: [20, 80], min: 0, max: 100, ariaLabel: "Price range" },
    });
    const lower = container.querySelector<HTMLInputElement>(".poodle-range-slider__control--lower")!;
    const upper = container.querySelector<HTMLInputElement>(".poodle-range-slider__control--upper")!;

    expect(lower.getAttribute("aria-label")).toBe("Price range minimum");
    expect(upper.getAttribute("aria-label")).toBe("Price range maximum");
    expect(lower.value).toBe("20");
    expect(upper.value).toBe("80");
    expect(lower.getAttribute("min")).toBe("0");
    expect(upper.getAttribute("max")).toBe("100");
  });

  it("drives the fill window custom properties from the pair", () => {
    const { container } = render(RangeSlider, { props: { value: [20, 80], ariaLabel: "Price range" } });
    const root = container.querySelector(".poodle-range-slider")!;
    const style = root.getAttribute("style") ?? "";
    expect(style).toContain("--poodle-range-start: 20%");
    expect(style).toContain("--poodle-range-end: 80%");
  });

  it("applies the bounds guard when max is at or below min", () => {
    const { container } = render(RangeSlider, {
      props: { value: [5, 8], min: 10, max: 10, ariaLabel: "Price range" },
    });
    const inputs = [...container.querySelectorAll<HTMLInputElement>(".poodle-range-slider__control")];
    expect(inputs.every((input) => input.getAttribute("max") === "11")).toBe(true);
  });

  it("disables both thumbs", () => {
    const { container } = render(RangeSlider, {
      props: { value: [30, 70], disabled: true, ariaLabel: "Disabled range" },
    });
    const inputs = [...container.querySelectorAll<HTMLInputElement>(".poodle-range-slider__control")];
    expect(inputs.every((input) => input.disabled)).toBe(true);
    expect(container.querySelector(".poodle-range-slider")!.getAttribute("data-disabled")).toBe("true");
  });
});
