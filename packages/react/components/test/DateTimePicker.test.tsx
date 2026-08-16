import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { DateTimePicker } from "../src/DateTimePicker";

function surfaceOf(): HTMLElement | null {
  return document.querySelector(".poodle-date-time-picker__surface");
}

function day(label: string): HTMLButtonElement {
  return document.querySelector(`[aria-label="${label}"]`) as HTMLButtonElement;
}

describe("DateTimePicker (react)", () => {
  it("composes the calendar and a time field in one dialog surface", () => {
    const { container } = render(
      <DateTimePicker defaultValue={{ date: "2026-03-14", time: "14:30" }} defaultOpen />,
    );

    const surface = surfaceOf();
    expect(surface?.getAttribute("role")).toBe("dialog");
    expect(surface?.querySelector(".poodle-calendar")).toBeTruthy();
    expect(surface?.querySelector(".poodle-date-time-picker__time-label")?.textContent).toBe("Time");

    const time = surface?.querySelector<HTMLInputElement>('[aria-label="Time"]')!;
    expect(time.type).toBe("time");
    expect(time.value).toBe("14:30");

    const trigger = container.querySelector<HTMLElement>(".poodle-date-time-picker__value")!;
    expect(trigger.textContent).toBe("Mar 14, 2026, 2:30 PM");
  });

  it("commits a date-only value on day selection and shows the date in the trigger", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <DateTimePicker defaultValue={{ date: "2026-03-14", time: null }} onValueChange={onValueChange} />,
    );
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-time-picker__trigger")!;
    const value = container.querySelector<HTMLElement>(".poodle-date-time-picker__value")!;

    fireEvent.click(trigger);
    fireEvent.click(day("Mar 15, 2026"));

    expect(onValueChange).toHaveBeenCalledWith({ date: "2026-03-15", time: null });
    expect(value.textContent).toBe("Mar 15, 2026");
    expect(value.getAttribute("data-placeholder")).toBe("true");
  });

  it("commits the time alongside the date from the time field", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <DateTimePicker defaultValue={{ date: "2026-03-14", time: null }} onValueChange={onValueChange} />,
    );
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-time-picker__trigger")!;
    const value = container.querySelector<HTMLElement>(".poodle-date-time-picker__value")!;

    fireEvent.click(trigger);
    fireEvent.change(document.querySelector<HTMLInputElement>('[aria-label="Time"]')!, {
      target: { value: "14:30" },
    });

    expect(onValueChange).toHaveBeenCalledWith({ date: "2026-03-14", time: "14:30" });
    expect(value.textContent).toBe("Mar 14, 2026, 2:30 PM");
  });

  it("shows partial labels for date-only and time-only values", () => {
    const dateOnly = render(<DateTimePicker defaultValue={{ date: "2026-03-14", time: null }} />);
    expect(dateOnly.container.querySelector(".poodle-date-time-picker__value")?.textContent).toBe(
      "Mar 14, 2026",
    );

    const timeOnly = render(<DateTimePicker defaultValue={{ date: null, time: "09:15" }} />);
    expect(timeOnly.container.querySelector(".poodle-date-time-picker__value")?.textContent).toBe(
      "9:15 AM",
    );
  });

  it("dismisses on Escape without changing the value", () => {
    const { container } = render(
      <DateTimePicker defaultValue={{ date: "2026-03-14", time: "14:30" }} />,
    );
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-time-picker__trigger")!;
    const value = container.querySelector<HTMLElement>(".poodle-date-time-picker__value")!;

    fireEvent.click(trigger);
    fireEvent.keyDown(document, { key: "Escape" });

    expect(surfaceOf()).toBeNull();
    expect(value.textContent).toBe("Mar 14, 2026, 2:30 PM");
  });

  it("stays closed and inert when disabled", () => {
    const onOpenChange = vi.fn();
    const { container } = render(<DateTimePicker disabled onOpenChange={onOpenChange} />);
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-time-picker__trigger")!;

    expect(trigger.disabled).toBe(true);

    fireEvent.click(trigger);
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).not.toHaveBeenCalled();
  });
});
