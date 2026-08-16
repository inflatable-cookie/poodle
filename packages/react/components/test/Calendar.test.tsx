import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Calendar } from "../src/Calendar";

function day(container: HTMLElement, label: string): HTMLButtonElement {
  return container.querySelector(`[aria-label="${label}"]`) as HTMLButtonElement;
}

describe("Calendar (react)", () => {
  it("selects a single date and reports the ISO value with selected semantics", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <Calendar defaultValue="2026-03-14" onValueChange={onValueChange} />,
    );

    const selected = day(container, "Mar 15, 2026");
    fireEvent.click(selected);

    expect(onValueChange).toHaveBeenCalledWith("2026-03-15");
    expect(selected.getAttribute("data-selected")).toBe("true");
    expect(selected.closest('[role="gridcell"]')?.getAttribute("aria-selected")).toBe("true");
    expect(day(container, "Mar 14, 2026").getAttribute("data-selected")).toBe("false");
  });

  it("builds a two-click range in range mode and flags in-range days", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <Calendar mode="range" visibleMonth="2026-03-01" onValueChange={onValueChange} />,
    );

    fireEvent.click(day(container, "Mar 20, 2026"));
    expect(onValueChange).toHaveBeenLastCalledWith({ start: "2026-03-20", end: null });

    fireEvent.click(day(container, "Mar 10, 2026"));

    expect(onValueChange).toHaveBeenLastCalledWith({ start: "2026-03-10", end: "2026-03-20" });
    expect(day(container, "Mar 10, 2026").getAttribute("data-range-start")).toBe("true");
    expect(day(container, "Mar 20, 2026").getAttribute("data-range-end")).toBe("true");
    expect(day(container, "Mar 12, 2026").getAttribute("data-in-range")).toBe("true");
    expect(day(container, "Mar 5, 2026").getAttribute("data-in-range")).toBe("false");
  });

  it("moves focus with arrow keys via roving tabindex and selects with Enter", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <Calendar defaultValue="2026-03-14" onValueChange={onValueChange} />,
    );

    const from = day(container, "Mar 14, 2026");
    expect(from.tabIndex).toBe(0);

    fireEvent.keyDown(from, { key: "ArrowRight" });

    const next = day(container, "Mar 15, 2026");
    expect(next.tabIndex).toBe(0);
    expect(from.tabIndex).toBe(-1);
    expect(document.activeElement).toBe(next);

    fireEvent.keyDown(next, { key: "Enter" });
    expect(onValueChange).toHaveBeenCalledWith("2026-03-15");
  });

  it("navigates months with the nav buttons and reports the month", () => {
    const onMonthChange = vi.fn();
    const { container } = render(
      <Calendar defaultValue="2026-03-14" onMonthChange={onMonthChange} />,
    );

    expect(container.querySelector(".poodle-calendar__month-name")?.textContent).toBe("March");

    fireEvent.click(container.querySelector<HTMLButtonElement>('[aria-label="Previous month"]')!);
    expect(onMonthChange).toHaveBeenLastCalledWith("2026-02-01");
    expect(container.querySelector(".poodle-calendar__month-name")?.textContent).toBe("February");

    fireEvent.click(container.querySelector<HTMLButtonElement>('[aria-label="Next month"]')!);
    fireEvent.click(container.querySelector<HTMLButtonElement>('[aria-label="Next month"]')!);
    expect(onMonthChange).toHaveBeenLastCalledWith("2026-04-01");
    expect(container.querySelector(".poodle-calendar__month-name")?.textContent).toBe("April");
  });

  it("pages months with PageUp/PageDown and lands focus in the new month", () => {
    const onMonthChange = vi.fn();
    const { container } = render(
      <Calendar defaultValue="2026-03-14" onMonthChange={onMonthChange} />,
    );

    fireEvent.keyDown(day(container, "Mar 14, 2026"), { key: "PageDown" });

    expect(onMonthChange).toHaveBeenCalledWith("2026-04-01");
    expect(container.querySelector(".poodle-calendar__month-name")?.textContent).toBe("April");
    expect(day(container, "Apr 14, 2026").tabIndex).toBe(0);
  });

  it("disables all interaction when disabled", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <Calendar defaultValue="2026-03-14" disabled onValueChange={onValueChange} />,
    );

    const buttons = [...container.querySelectorAll<HTMLButtonElement>(".poodle-calendar__day")];
    expect(buttons.every((button) => button.disabled)).toBe(true);
    expect(
      (container.querySelector('[aria-label="Previous month"]') as HTMLButtonElement).disabled,
    ).toBe(true);

    fireEvent.click(day(container, "Mar 15, 2026"));
    expect(onValueChange).not.toHaveBeenCalled();
  });
});
