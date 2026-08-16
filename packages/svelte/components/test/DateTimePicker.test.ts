import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import DateTimePicker from "../src/DateTimePicker.svelte";

function surfaceOf(): HTMLElement | null {
  return document.querySelector(".poodle-date-time-picker__surface");
}

function day(label: string): HTMLButtonElement {
  return document.querySelector(`[aria-label="${label}"]`) as HTMLButtonElement;
}

describe("DateTimePicker (svelte)", () => {
  it("composes the calendar and a time field in one dialog surface", async () => {
    const { container } = render(DateTimePicker, {
      props: { defaultValue: { date: "2026-03-14", time: "14:30" }, defaultOpen: true },
    });

    const surface = surfaceOf();
    expect(surface?.getAttribute("role")).toBe("dialog");
    expect(surface?.querySelector(".poodle-calendar")).toBeTruthy();
    expect(surface?.querySelector(".poodle-date-time-picker__time-label")?.textContent).toBe("Time");

    const time = surface?.querySelector('[aria-label="Time"]') as HTMLInputElement;
    expect(time.type).toBe("time");
    expect(time.value).toBe("14:30");

    const trigger = container.querySelector(".poodle-date-time-picker__value") as HTMLElement;
    expect(trigger.textContent).toBe("Mar 14, 2026, 2:30 PM");
  });

  it("commits a date-only value on day selection and shows the date in the trigger", async () => {
    const onValueChange = vi.fn();
    const { container } = render(DateTimePicker, {
      props: { defaultValue: { date: "2026-03-14", time: null }, onValueChange },
    });
    const trigger = container.querySelector(".poodle-date-time-picker__trigger") as HTMLButtonElement;
    const value = container.querySelector(".poodle-date-time-picker__value") as HTMLElement;

    await fireEvent.click(trigger);
    await fireEvent.click(day("Mar 15, 2026"));

    expect(onValueChange).toHaveBeenCalledWith({ date: "2026-03-15", time: null });
    expect(value.textContent).toBe("Mar 15, 2026");
    expect(value.getAttribute("data-placeholder")).toBe("true");
  });

  it("commits the time alongside the date from the time field", async () => {
    const onValueChange = vi.fn();
    const { container } = render(DateTimePicker, {
      props: { defaultValue: { date: "2026-03-14", time: null }, onValueChange },
    });
    const trigger = container.querySelector(".poodle-date-time-picker__trigger") as HTMLButtonElement;
    const value = container.querySelector(".poodle-date-time-picker__value") as HTMLElement;

    await fireEvent.click(trigger);
    await fireEvent.input(document.querySelector('[aria-label="Time"]') as HTMLInputElement, {
      target: { value: "14:30" },
    });

    expect(onValueChange).toHaveBeenCalledWith({ date: "2026-03-14", time: "14:30" });
    expect(value.textContent).toBe("Mar 14, 2026, 2:30 PM");
  });

  it("shows partial labels for date-only and time-only values", () => {
    const dateOnly = render(DateTimePicker, { props: { defaultValue: { date: "2026-03-14", time: null } } });
    expect(dateOnly.container.querySelector(".poodle-date-time-picker__value")?.textContent).toBe(
      "Mar 14, 2026",
    );

    const timeOnly = render(DateTimePicker, { props: { defaultValue: { date: null, time: "09:15" } } });
    expect(timeOnly.container.querySelector(".poodle-date-time-picker__value")?.textContent).toBe(
      "9:15 AM",
    );
  });

  it("dismisses on Escape without changing the value", async () => {
    const { container } = render(DateTimePicker, {
      props: { defaultValue: { date: "2026-03-14", time: "14:30" } },
    });
    const trigger = container.querySelector(".poodle-date-time-picker__trigger") as HTMLButtonElement;
    const value = container.querySelector(".poodle-date-time-picker__value") as HTMLElement;

    await fireEvent.click(trigger);
    await fireEvent.keyDown(document, { key: "Escape" });

    expect(surfaceOf()).toBeNull();
    expect(value.textContent).toBe("Mar 14, 2026, 2:30 PM");
  });

  it("stays closed and inert when disabled", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(DateTimePicker, { props: { disabled: true, onOpenChange } });
    const trigger = container.querySelector(".poodle-date-time-picker__trigger") as HTMLButtonElement;

    expect(trigger.disabled).toBe(true);

    await fireEvent.click(trigger);
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).not.toHaveBeenCalled();
  });
});
