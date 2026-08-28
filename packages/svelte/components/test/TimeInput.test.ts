import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import TimeInput from "../src/TimeInput.svelte";

describe("TimeInput (svelte)", () => {
  it("renders a native time input with forwarded constraints", () => {
    const { container } = render(TimeInput, {
      props: { min: "08:00", max: "18:00", step: 300, ariaLabel: "Office hours" },
    });
    const input = container.querySelector("input.poodle-time-input") as HTMLInputElement;
    expect(input.type).toBe("time");
    expect(input.min).toBe("08:00");
    expect(input.max).toBe("18:00");
    expect(input.getAttribute("step")).toBe("300");
    expect(input.getAttribute("aria-label")).toBe("Office hours");
  });

  it("seeds the uncontrolled value from defaultValue and reports changes", async () => {
    const onValueChange = vi.fn();
    const { container } = render(TimeInput, {
      props: { defaultValue: "14:30", onValueChange },
    });
    const input = container.querySelector("input") as HTMLInputElement;
    expect(input.value).toBe("14:30");

    await fireEvent.input(input, { target: { value: "09:15" } });
    expect(onValueChange).toHaveBeenCalledWith("09:15");
  });

  it("reports a cleared value as null", async () => {
    const onValueChange = vi.fn();
    const { container } = render(TimeInput, { props: { defaultValue: "14:30", onValueChange } });
    const input = container.querySelector("input") as HTMLInputElement;

    await fireEvent.input(input, { target: { value: "" } });
    expect(onValueChange).toHaveBeenCalledWith(null);
  });

  it("stays controlled when value is supplied", async () => {
    const onValueChange = vi.fn();
    const { container } = render(TimeInput, { props: { value: "10:00", onValueChange } });
    const input = container.querySelector("input") as HTMLInputElement;
    expect(input.value).toBe("10:00");

    await fireEvent.input(input, { target: { value: "11:00" } });
    expect(onValueChange).toHaveBeenCalledWith("11:00");
  });

  it("disables the input and projects size and density", () => {
    const { container } = render(TimeInput, {
      props: { disabled: true, size: "lg", density: "compact" },
    });
    const input = container.querySelector("input") as HTMLInputElement;
    expect(input.disabled).toBe(true);
    expect(input.dataset.size).toBe("lg");
    expect(input.dataset.density).toBe("compact");
  });

  it("does not emit off-step or out-of-range native values and marks them invalid", async () => {
    const onValueChange = vi.fn();
    const { container } = render(TimeInput, {
      props: { defaultValue: "09:00", step: 300, min: "08:00", max: "18:00", onValueChange },
    });
    const input = container.querySelector("input") as HTMLInputElement;

    await fireEvent.input(input, { target: { value: "09:07" } });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(input.getAttribute("aria-invalid")).toBe("true");
    expect(input.value).toBe("09:07");
  });

  it("reverts an invalid draft on blur and Escape without emitting", async () => {
    const onValueChange = vi.fn();
    const { container } = render(TimeInput, {
      props: { defaultValue: "09:00", step: 300, min: "08:00", max: "18:00", onValueChange },
    });
    const input = container.querySelector("input") as HTMLInputElement;

    await fireEvent.input(input, { target: { value: "09:07" } });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(input.getAttribute("aria-invalid")).toBe("true");

    await fireEvent.blur(input);
    expect(onValueChange).not.toHaveBeenCalled();
    expect(input.value).toBe("09:00");
    expect(input.hasAttribute("aria-invalid")).toBe(false);

    await fireEvent.input(input, { target: { value: "09:07" } });
    await fireEvent.keyDown(input, { key: "Escape" });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(input.value).toBe("09:00");
  });

  it("discards an invalid draft when the controlled value is replaced", async () => {
    const onValueChange = vi.fn();
    const { container, rerender } = render(TimeInput, {
      props: { value: "09:00", step: 300, onValueChange },
    });
    const input = container.querySelector("input") as HTMLInputElement;

    await fireEvent.input(input, { target: { value: "09:07" } });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(input.getAttribute("aria-invalid")).toBe("true");

    await rerender({ value: "11:00", step: 300, onValueChange });
    expect(input.value).toBe("11:00");
    expect(input.hasAttribute("aria-invalid")).toBe(false);
    expect(onValueChange).not.toHaveBeenCalled();
  });
});