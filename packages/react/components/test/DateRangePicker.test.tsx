import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { DateRangePicker } from "../src/DateRangePicker";

function surfaceOf(): HTMLElement | null {
  return document.querySelector(".poodle-date-range-picker__surface");
}

function day(label: string): HTMLButtonElement {
  return document.querySelector(`[aria-label="${label}"]`) as HTMLButtonElement;
}

function openOnDay(container: HTMLElement, label: string): void {
  const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-range-picker__trigger")!;
  fireEvent.click(trigger);
  for (let i = 0; i < 24 && !day(label); i++) {
    const prev = document.querySelector<HTMLButtonElement>('[aria-label="Previous month"]');
    if (!prev) return;
    fireEvent.click(prev);
  }
}

describe("DateRangePicker (react)", () => {
  it("shows the start with a pending end label and stays open after the first click", () => {
    const onValueChange = vi.fn();
    const { container } = render(<DateRangePicker onValueChange={onValueChange} />);
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-range-picker__trigger")!;
    const value = container.querySelector<HTMLElement>(".poodle-date-range-picker__value")!;

    expect(value.textContent).toBe("Select date range");
    expect(value.getAttribute("data-placeholder")).toBe("true");

    openOnDay(container, "Mar 10, 2026");
    fireEvent.click(day("Mar 10, 2026"));

    expect(value.textContent).toBe("Mar 10, 2026 – End date");
    expect(value.getAttribute("data-placeholder")).toBe("false");
    expect(surfaceOf()).toBeTruthy();
  });

  it("completes the range on the second click, reports it, and auto-closes", () => {
    const onValueChange = vi.fn();
    const onOpenChange = vi.fn();
    const { container } = render(<DateRangePicker onValueChange={onValueChange} onOpenChange={onOpenChange} />);
    const value = container.querySelector<HTMLElement>(".poodle-date-range-picker__value")!;

    openOnDay(container, "Mar 10, 2026");
    fireEvent.click(day("Mar 10, 2026"));
    fireEvent.click(day("Mar 14, 2026"));

    expect(onValueChange).toHaveBeenLastCalledWith({ start: "2026-03-10", end: "2026-03-14" });
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
    expect(surfaceOf()).toBeNull();
    expect(value.textContent).toBe("Mar 10, 2026 – Mar 14, 2026");
  });

  it("normalizes endpoints when the second click precedes the first", () => {
    const onValueChange = vi.fn();
    const { container } = render(<DateRangePicker onValueChange={onValueChange} />);

    openOnDay(container, "Mar 14, 2026");
    fireEvent.click(day("Mar 14, 2026"));
    fireEvent.click(day("Mar 10, 2026"));

    expect(onValueChange).toHaveBeenLastCalledWith({ start: "2026-03-10", end: "2026-03-14" });
  });

  it("dismisses on Escape without changing the value", () => {
    const { container } = render(
      <DateRangePicker defaultValue={{ start: "2026-03-01", end: null }} />,
    );
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-range-picker__trigger")!;
    const value = container.querySelector<HTMLElement>(".poodle-date-range-picker__value")!;

    fireEvent.click(trigger);
    fireEvent.keyDown(document, { key: "Escape" });

    expect(surfaceOf()).toBeNull();
    expect(value.textContent).toBe("Mar 1, 2026 – End date");
  });

  it("displays a committed range from defaultValue", () => {
    const { container } = render(
      <DateRangePicker defaultValue={{ start: "2026-03-01", end: "2026-03-14" }} />,
    );
    const value = container.querySelector<HTMLElement>(".poodle-date-range-picker__value")!;

    expect(value.textContent).toBe("Mar 1, 2026 – Mar 14, 2026");
    expect(value.getAttribute("data-placeholder")).toBe("false");
  });

  it("stays closed and inert when disabled", () => {
    const onOpenChange = vi.fn();
    const { container } = render(<DateRangePicker disabled onOpenChange={onOpenChange} />);
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-range-picker__trigger")!;

    expect(trigger.disabled).toBe(true);

    fireEvent.click(trigger);
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).not.toHaveBeenCalled();
  });
});
