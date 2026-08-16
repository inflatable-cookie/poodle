import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import DateTimeRangePicker from "../src/DateTimeRangePicker.svelte";

const FULL_RANGE = {
  start: { date: "2026-03-10", time: "09:00" },
  end: { date: "2026-03-14", time: "17:00" },
};

function surfaceOf(): HTMLElement | null {
  return document.querySelector(".poodle-date-time-range-picker__surface");
}

function day(label: string): HTMLButtonElement {
  return document.querySelector(`[aria-label="${label}"]`) as HTMLButtonElement;
}

describe("DateTimeRangePicker (svelte)", () => {
  it("composes a range calendar with paired start and end time fields", async () => {
    const { container } = render(DateTimeRangePicker, {
      props: { defaultValue: FULL_RANGE, defaultOpen: true },
    });

    const surface = surfaceOf();
    expect(surface?.getAttribute("role")).toBe("dialog");
    expect(surface?.querySelector('.poodle-calendar[data-mode="range"]')).toBeTruthy();

    const labels = [
      ...(surface?.querySelectorAll(".poodle-date-time-range-picker__time-label") ?? []),
    ].map((label) => label.textContent);
    expect(labels).toEqual(["Start time", "End time"]);

    const start = surface?.querySelector('[aria-label="Start time"]') as HTMLInputElement;
    const end = surface?.querySelector('[aria-label="End time"]') as HTMLInputElement;
    expect(start.value).toBe("09:00");
    expect(end.value).toBe("17:00");

    const value = container.querySelector(".poodle-date-time-range-picker__value") as HTMLElement;
    expect(value.textContent).toContain("Mar 10, 2026");
    expect(value.textContent).toContain("Mar 14, 2026");
  });

  it("commits a complete date range and keeps the overlay open", async () => {
    const onValueChange = vi.fn();
    const { container } = render(DateTimeRangePicker, {
      props: {
        defaultValue: { start: { date: "2026-03-10", time: null }, end: { date: null, time: null } },
        onValueChange,
      },
    });
    const trigger = container.querySelector(
      ".poodle-date-time-range-picker__trigger",
    ) as HTMLButtonElement;

    await fireEvent.click(trigger);
    await fireEvent.click(day("Mar 12, 2026"));

    expect(onValueChange).toHaveBeenLastCalledWith({
      start: { date: "2026-03-10", time: null },
      end: { date: "2026-03-12", time: null },
    });
    expect(surfaceOf()).toBeTruthy();
  });

  it("commits the start time into the start half of the range", async () => {
    const onValueChange = vi.fn();
    const { container } = render(DateTimeRangePicker, {
      props: { defaultValue: FULL_RANGE, defaultOpen: true, onValueChange },
    });

    await fireEvent.input(document.querySelector('[aria-label="Start time"]') as HTMLInputElement, {
      target: { value: "08:30" },
    });

    expect(onValueChange).toHaveBeenLastCalledWith({
      start: { date: "2026-03-10", time: "08:30" },
      end: { date: "2026-03-14", time: "17:00" },
    });
  });

  it("commits the end time into the end half of the range", async () => {
    const onValueChange = vi.fn();
    render(DateTimeRangePicker, {
      props: { defaultValue: FULL_RANGE, defaultOpen: true, onValueChange },
    });

    await fireEvent.input(document.querySelector('[aria-label="End time"]') as HTMLInputElement, {
      target: { value: "18:00" },
    });

    expect(onValueChange).toHaveBeenLastCalledWith({
      start: { date: "2026-03-10", time: "09:00" },
      end: { date: "2026-03-14", time: "18:00" },
    });
  });

  it("dismisses on Escape without changing the value", async () => {
    const { container } = render(DateTimeRangePicker, { props: { defaultValue: FULL_RANGE } });
    const trigger = container.querySelector(
      ".poodle-date-time-range-picker__trigger",
    ) as HTMLButtonElement;
    const value = container.querySelector(".poodle-date-time-range-picker__value") as HTMLElement;
    const before = value.textContent;

    await fireEvent.click(trigger);
    await fireEvent.keyDown(document, { key: "Escape" });

    expect(surfaceOf()).toBeNull();
    expect(value.textContent).toBe(before);
  });

  it("shows the placeholder by default and stays closed when disabled", async () => {
    const { container } = render(DateTimeRangePicker, { props: { disabled: true } });
    const value = container.querySelector(".poodle-date-time-range-picker__value") as HTMLElement;
    const trigger = container.querySelector(
      ".poodle-date-time-range-picker__trigger",
    ) as HTMLButtonElement;

    expect(value.textContent).toBe("Select date and time range");
    expect(value.getAttribute("data-placeholder")).toBe("true");
    expect(trigger.disabled).toBe(true);

    await fireEvent.click(trigger);
    expect(surfaceOf()).toBeNull();
  });
});
