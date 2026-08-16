import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { DateTimeRangePicker } from "../src/DateTimeRangePicker";

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

describe("DateTimeRangePicker (react)", () => {
  it("composes a range calendar with paired start and end time fields", () => {
    const { container } = render(<DateTimeRangePicker defaultValue={FULL_RANGE} defaultOpen />);

    const surface = surfaceOf();
    expect(surface?.getAttribute("role")).toBe("dialog");
    expect(surface?.querySelector('.poodle-calendar[data-mode="range"]')).toBeTruthy();

    const labels = [
      ...(surface?.querySelectorAll(".poodle-date-time-range-picker__time-label") ?? []),
    ].map((label) => label.textContent);
    expect(labels).toEqual(["Start time", "End time"]);

    const start = surface?.querySelector<HTMLInputElement>('[aria-label="Start time"]')!;
    const end = surface?.querySelector<HTMLInputElement>('[aria-label="End time"]')!;
    expect(start.value).toBe("09:00");
    expect(end.value).toBe("17:00");

    const value = container.querySelector<HTMLElement>(".poodle-date-time-range-picker__value")!;
    expect(value.textContent).toContain("Mar 10, 2026");
    expect(value.textContent).toContain("Mar 14, 2026");
  });

  it("commits a complete date range and keeps the overlay open", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <DateTimeRangePicker
        defaultValue={{ start: { date: "2026-03-10", time: null }, end: { date: null, time: null } }}
        onValueChange={onValueChange}
      />,
    );
    const trigger = container.querySelector<HTMLButtonElement>(
      ".poodle-date-time-range-picker__trigger",
    )!;

    fireEvent.click(trigger);
    fireEvent.click(day("Mar 12, 2026"));

    expect(onValueChange).toHaveBeenLastCalledWith({
      start: { date: "2026-03-10", time: null },
      end: { date: "2026-03-12", time: null },
    });
    expect(surfaceOf()).toBeTruthy();
  });

  it("commits the start time into the start half of the range", () => {
    const onValueChange = vi.fn();
    render(
      <DateTimeRangePicker defaultValue={FULL_RANGE} defaultOpen onValueChange={onValueChange} />,
    );

    fireEvent.change(document.querySelector<HTMLInputElement>('[aria-label="Start time"]')!, {
      target: { value: "08:30" },
    });

    expect(onValueChange).toHaveBeenLastCalledWith({
      start: { date: "2026-03-10", time: "08:30" },
      end: { date: "2026-03-14", time: "17:00" },
    });
  });

  it("commits the end time into the end half of the range", () => {
    const onValueChange = vi.fn();
    render(
      <DateTimeRangePicker defaultValue={FULL_RANGE} defaultOpen onValueChange={onValueChange} />,
    );

    fireEvent.change(document.querySelector<HTMLInputElement>('[aria-label="End time"]')!, {
      target: { value: "18:00" },
    });

    expect(onValueChange).toHaveBeenLastCalledWith({
      start: { date: "2026-03-10", time: "09:00" },
      end: { date: "2026-03-14", time: "18:00" },
    });
  });

  it("dismisses on Escape without changing the value", () => {
    const { container } = render(<DateTimeRangePicker defaultValue={FULL_RANGE} />);
    const trigger = container.querySelector<HTMLButtonElement>(
      ".poodle-date-time-range-picker__trigger",
    )!;
    const value = container.querySelector<HTMLElement>(".poodle-date-time-range-picker__value")!;
    const before = value.textContent;

    fireEvent.click(trigger);
    fireEvent.keyDown(document, { key: "Escape" });

    expect(surfaceOf()).toBeNull();
    expect(value.textContent).toBe(before);
  });

  it("shows the placeholder by default and stays closed when disabled", () => {
    const { container } = render(<DateTimeRangePicker disabled />);
    const value = container.querySelector<HTMLElement>(".poodle-date-time-range-picker__value")!;
    const trigger = container.querySelector<HTMLButtonElement>(
      ".poodle-date-time-range-picker__trigger",
    )!;

    expect(value.textContent).toBe("Select date and time range");
    expect(value.getAttribute("data-placeholder")).toBe("true");
    expect(trigger.disabled).toBe(true);

    fireEvent.click(trigger);
    expect(surfaceOf()).toBeNull();
  });
});
