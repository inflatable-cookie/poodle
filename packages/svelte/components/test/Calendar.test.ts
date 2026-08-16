import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Calendar from "../src/Calendar.svelte";

function day(container: HTMLElement, label: string): HTMLButtonElement {
  return container.querySelector(`[aria-label="${label}"]`) as HTMLButtonElement;
}

describe("Calendar (svelte)", () => {
  it("selects a single date and reports the ISO value with selected semantics", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Calendar, { props: { defaultValue: "2026-03-14", onValueChange } });

    const selected = day(container, "Mar 15, 2026");
    await fireEvent.click(selected);

    expect(onValueChange).toHaveBeenCalledWith("2026-03-15");
    expect(selected.getAttribute("data-selected")).toBe("true");
    expect(selected.closest('[role="gridcell"]')?.getAttribute("aria-selected")).toBe("true");
    expect(day(container, "Mar 14, 2026").getAttribute("data-selected")).toBe("false");
  });

  it("builds a two-click range in range mode and flags in-range days", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Calendar, {
      props: { mode: "range", visibleMonth: "2026-03-01", onValueChange },
    });

    await fireEvent.click(day(container, "Mar 20, 2026"));
    expect(onValueChange).toHaveBeenLastCalledWith({ start: "2026-03-20", end: null });

    await fireEvent.click(day(container, "Mar 10, 2026"));

    expect(onValueChange).toHaveBeenLastCalledWith({ start: "2026-03-10", end: "2026-03-20" });
    expect(day(container, "Mar 10, 2026").getAttribute("data-range-start")).toBe("true");
    expect(day(container, "Mar 20, 2026").getAttribute("data-range-end")).toBe("true");
    expect(day(container, "Mar 12, 2026").getAttribute("data-in-range")).toBe("true");
    expect(day(container, "Mar 5, 2026").getAttribute("data-in-range")).toBe("false");
  });

  it("moves focus with arrow keys via roving tabindex and selects with Enter", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Calendar, { props: { defaultValue: "2026-03-14", onValueChange } });

    const from = day(container, "Mar 14, 2026");
    expect(from.tabIndex).toBe(0);

    await fireEvent.keyDown(from, { key: "ArrowRight" });

    const next = day(container, "Mar 15, 2026");
    expect(next.tabIndex).toBe(0);
    expect(from.tabIndex).toBe(-1);
    expect(document.activeElement).toBe(next);

    await fireEvent.keyDown(next, { key: "Enter" });
    expect(onValueChange).toHaveBeenCalledWith("2026-03-15");
  });

  it("navigates months with the nav buttons and reports the month", async () => {
    const onMonthChange = vi.fn();
    const { container } = render(Calendar, { props: { defaultValue: "2026-03-14", onMonthChange } });

    expect(container.querySelector(".poodle-calendar__month-name")?.textContent).toBe("March");

    await fireEvent.click(container.querySelector('[aria-label="Previous month"]') as HTMLButtonElement);
    expect(onMonthChange).toHaveBeenLastCalledWith("2026-02-01");
    expect(container.querySelector(".poodle-calendar__month-name")?.textContent).toBe("February");

    await fireEvent.click(container.querySelector('[aria-label="Next month"]') as HTMLButtonElement);
    await fireEvent.click(container.querySelector('[aria-label="Next month"]') as HTMLButtonElement);
    expect(onMonthChange).toHaveBeenLastCalledWith("2026-04-01");
    expect(container.querySelector(".poodle-calendar__month-name")?.textContent).toBe("April");
  });

  it("pages months with PageUp/PageDown and lands focus in the new month", async () => {
    const onMonthChange = vi.fn();
    const { container } = render(Calendar, { props: { defaultValue: "2026-03-14", onMonthChange } });

    await fireEvent.keyDown(day(container, "Mar 14, 2026"), { key: "PageDown" });

    expect(onMonthChange).toHaveBeenCalledWith("2026-04-01");
    expect(container.querySelector(".poodle-calendar__month-name")?.textContent).toBe("April");
    expect(day(container, "Apr 14, 2026").tabIndex).toBe(0);
  });

  it("disables all interaction when disabled", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Calendar, {
      props: { defaultValue: "2026-03-14", disabled: true, onValueChange },
    });

    const buttons = [...container.querySelectorAll<HTMLButtonElement>(".poodle-calendar__day")];
    expect(buttons.every((button) => button.disabled)).toBe(true);
    expect(
      (container.querySelector('[aria-label="Previous month"]') as HTMLButtonElement).disabled,
    ).toBe(true);

    await fireEvent.click(day(container, "Mar 15, 2026"));
    expect(onValueChange).not.toHaveBeenCalled();
  });
});
