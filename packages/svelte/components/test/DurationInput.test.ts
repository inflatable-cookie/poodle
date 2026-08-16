import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import DurationInput from "../src/DurationInput.svelte";

function field(container: HTMLElement, label: string): HTMLInputElement {
  return container.querySelector(`[aria-label="${label}"]`) as HTMLInputElement;
}

describe("DurationInput (svelte)", () => {
  it("carries seconds over to minutes and hours on ArrowUp and reports totals", async () => {
    const onChange = vi.fn();
    const { container } = render(DurationInput, {
      props: { hours: 0, minutes: 59, seconds: 59, onChange },
    });

    await fireEvent.keyDown(field(container, "Seconds"), { key: "ArrowUp" });

    expect(onChange).toHaveBeenCalledWith({ hours: 1, minutes: 0, seconds: 0, totalSeconds: 3600 });
    expect(field(container, "Hours").value).toBe("01");
    expect(field(container, "Seconds").value).toBe("00");
  });

  it("borrows minutes and hours on ArrowDown", async () => {
    const onChange = vi.fn();
    const { container } = render(DurationInput, {
      props: { hours: 1, minutes: 0, seconds: 0, onChange },
    });

    await fireEvent.keyDown(field(container, "Seconds"), { key: "ArrowDown" });

    expect(onChange).toHaveBeenCalledWith({ hours: 0, minutes: 59, seconds: 59, totalSeconds: 3599 });
    expect(field(container, "Hours").value).toBe("00");
    expect(field(container, "Minutes").value).toBe("59");
  });

  it("clamps direct numeric entry per segment and to maxHours", async () => {
    const onChange = vi.fn();
    const { container } = render(DurationInput, {
      props: { hours: 0, minutes: 0, seconds: 0, maxHours: 12, onChange },
    });

    await fireEvent.input(field(container, "Minutes"), { target: { value: "75" } });
    expect(onChange).toHaveBeenLastCalledWith({ hours: 0, minutes: 59, seconds: 0, totalSeconds: 3540 });

    await fireEvent.input(field(container, "Hours"), { target: { value: "99" } });
    expect(onChange).toHaveBeenLastCalledWith({ hours: 12, minutes: 59, seconds: 0, totalSeconds: 46740 });
  });

  it("swallows upward carry at the maxHours bound", async () => {
    const onChange = vi.fn();
    const { container } = render(DurationInput, {
      props: { hours: 99, minutes: 59, seconds: 0, onChange },
    });

    await fireEvent.keyDown(field(container, "Minutes"), { key: "ArrowUp" });

    expect(onChange).toHaveBeenCalledWith({ hours: 99, minutes: 0, seconds: 0, totalSeconds: 356400 });
    expect(field(container, "Hours").value).toBe("99");
  });

  it("flags data-invalid when the total falls outside min/max bounds", () => {
    const over = render(DurationInput, { props: { hours: 2, minutes: 0, maxTotalSeconds: 3600 } });
    expect(over.container.querySelector(".poodle-duration-input")?.getAttribute("data-invalid")).toBe("true");

    const under = render(DurationInput, { props: { hours: 0, minutes: 0, minTotalSeconds: 60 } });
    expect(under.container.querySelector(".poodle-duration-input")?.getAttribute("data-invalid")).toBe("true");

    const valid = render(DurationInput, { props: { hours: 1, minutes: 0 } });
    expect(valid.container.querySelector(".poodle-duration-input")?.getAttribute("data-invalid")).toBe("false");
  });

  it("hides the seconds segment when showSeconds is false", () => {
    const { container } = render(DurationInput, { props: { hours: 1, minutes: 30, showSeconds: false } });
    expect(container.querySelector('[aria-label="Seconds"]')).toBeNull();
    expect(container.querySelectorAll('[aria-label="Hours"]').length).toBe(1);
  });

  it("disables every field and ignores keyboard edits", async () => {
    const onChange = vi.fn();
    const { container } = render(DurationInput, {
      props: { hours: 1, minutes: 30, disabled: true, onChange },
    });

    expect((field(container, "Hours") as HTMLInputElement).disabled).toBe(true);
    expect((field(container, "Minutes") as HTMLInputElement).disabled).toBe(true);
    expect((field(container, "Seconds") as HTMLInputElement).disabled).toBe(true);

    await fireEvent.keyDown(field(container, "Hours"), { key: "ArrowUp" });
    expect(onChange).not.toHaveBeenCalled();
  });
});
