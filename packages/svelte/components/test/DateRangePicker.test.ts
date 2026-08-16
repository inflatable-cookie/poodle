import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import DateRangePicker from "../src/DateRangePicker.svelte";

function surfaceOf(): HTMLElement | null {
  return document.querySelector(".poodle-date-range-picker__surface");
}

function day(label: string): HTMLButtonElement {
  return document.querySelector(`[aria-label="${label}"]`) as HTMLButtonElement;
}

async function openOnDay(container: HTMLElement, label: string): Promise<void> {
  const trigger = container.querySelector(".poodle-date-range-picker__trigger") as HTMLButtonElement;
  await fireEvent.click(trigger);
  for (let i = 0; i < 24 && !day(label); i++) {
    const prev = document.querySelector('[aria-label="Previous month"]') as HTMLButtonElement;
    if (!prev) return;
    await fireEvent.click(prev);
  }
}

describe("DateRangePicker (svelte)", () => {
  it("shows the start with a pending end label and stays open after the first click", async () => {
    const onValueChange = vi.fn();
    const { container } = render(DateRangePicker, { props: { onValueChange } });
    const trigger = container.querySelector(".poodle-date-range-picker__trigger") as HTMLButtonElement;
    const value = container.querySelector(".poodle-date-range-picker__value") as HTMLElement;

    expect(value.textContent).toBe("Select date range");
    expect(value.getAttribute("data-placeholder")).toBe("true");

    await openOnDay(container, "Mar 10, 2026");
    await fireEvent.click(day("Mar 10, 2026"));

    expect(value.textContent).toBe("Mar 10, 2026 – End date");
    expect(value.getAttribute("data-placeholder")).toBe("false");
    expect(surfaceOf()).toBeTruthy();
  });

  it("completes the range on the second click, reports it, and auto-closes", async () => {
    const onValueChange = vi.fn();
    const onOpenChange = vi.fn();
    const { container } = render(DateRangePicker, { props: { onValueChange, onOpenChange } });
    const value = container.querySelector(".poodle-date-range-picker__value") as HTMLElement;

    await openOnDay(container, "Mar 10, 2026");
    await fireEvent.click(day("Mar 10, 2026"));
    await fireEvent.click(day("Mar 14, 2026"));

    expect(onValueChange).toHaveBeenLastCalledWith({ start: "2026-03-10", end: "2026-03-14" });
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
    expect(surfaceOf()).toBeNull();
    expect(value.textContent).toBe("Mar 10, 2026 – Mar 14, 2026");
  });

  it("normalizes endpoints when the second click precedes the first", async () => {
    const onValueChange = vi.fn();
    const { container } = render(DateRangePicker, { props: { onValueChange } });

    await openOnDay(container, "Mar 14, 2026");
    await fireEvent.click(day("Mar 14, 2026"));
    await fireEvent.click(day("Mar 10, 2026"));

    expect(onValueChange).toHaveBeenLastCalledWith({ start: "2026-03-10", end: "2026-03-14" });
  });

  it("dismisses on Escape without changing the value", async () => {
    const { container } = render(DateRangePicker, {
      props: { defaultValue: { start: "2026-03-01", end: null } },
    });
    const trigger = container.querySelector(".poodle-date-range-picker__trigger") as HTMLButtonElement;
    const value = container.querySelector(".poodle-date-range-picker__value") as HTMLElement;

    await fireEvent.click(trigger);
    await fireEvent.keyDown(document, { key: "Escape" });

    expect(surfaceOf()).toBeNull();
    expect(value.textContent).toBe("Mar 1, 2026 – End date");
  });

  it("displays a committed range from defaultValue", () => {
    const { container } = render(DateRangePicker, {
      props: { defaultValue: { start: "2026-03-01", end: "2026-03-14" } },
    });
    const value = container.querySelector(".poodle-date-range-picker__value") as HTMLElement;

    expect(value.textContent).toBe("Mar 1, 2026 – Mar 14, 2026");
    expect(value.getAttribute("data-placeholder")).toBe("false");
  });

  it("stays closed and inert when disabled", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(DateRangePicker, { props: { disabled: true, onOpenChange } });
    const trigger = container.querySelector(".poodle-date-range-picker__trigger") as HTMLButtonElement;

    expect(trigger.disabled).toBe(true);

    await fireEvent.click(trigger);
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).not.toHaveBeenCalled();
  });
});
