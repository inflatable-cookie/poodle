import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { DatePicker } from "../src/DatePicker";

// The surface is portalled by the anchored surface to the theme root, so it is
// not reachable from the render container.
function surfaceOf(): HTMLElement | null {
  return document.querySelector(".poodle-date-picker__surface");
}

function day(label: string): HTMLButtonElement {
  return document.querySelector(`[aria-label="${label}"]`) as HTMLButtonElement;
}

describe("DatePicker (react)", () => {
  it("opens a dialog surface with the calendar on trigger click", () => {
    const onOpenChange = vi.fn();
    const { container } = render(<DatePicker defaultValue="2026-03-14" onOpenChange={onOpenChange} />);
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-picker__trigger")!;

    expect(trigger.getAttribute("aria-haspopup")).toBe("dialog");
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(surfaceOf()).toBeNull();

    fireEvent.click(trigger);

    expect(surfaceOf()?.getAttribute("role")).toBe("dialog");
    expect(surfaceOf()?.querySelector(".poodle-calendar")).toBeTruthy();
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(onOpenChange).toHaveBeenCalledWith(true);
  });

  it("reports the selected ISO date, closes the overlay, and formats the trigger", () => {
    const onValueChange = vi.fn();
    const onOpenChange = vi.fn();
    const { container } = render(
      <DatePicker defaultValue="2026-03-14" onValueChange={onValueChange} onOpenChange={onOpenChange} />,
    );
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-picker__trigger")!;

    fireEvent.click(trigger);
    fireEvent.click(day("Mar 15, 2026"));

    expect(onValueChange).toHaveBeenCalledWith("2026-03-15");
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
    expect(container.querySelector(".poodle-date-picker__value")?.textContent).toBe("Mar 15, 2026");
  });

  it("closes on Escape and leaves the committed value untouched", () => {
    const onOpenChange = vi.fn();
    const { container } = render(<DatePicker defaultValue="2026-03-14" onOpenChange={onOpenChange} />);
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-picker__trigger")!;

    fireEvent.click(trigger);
    fireEvent.keyDown(document, { key: "Escape" });

    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
    expect(container.querySelector(".poodle-date-picker__value")?.textContent).toBe("Mar 14, 2026");
  });

  it("dismisses on outside mousedown", () => {
    const { container } = render(<DatePicker defaultValue="2026-03-14" />);
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-picker__trigger")!;

    fireEvent.click(trigger);
    expect(surfaceOf()).toBeTruthy();

    fireEvent.mouseDown(document.body);
    expect(surfaceOf()).toBeNull();
  });

  it("shows the placeholder when no value is set", () => {
    const { container } = render(<DatePicker placeholder="Pick a day" />);
    const value = container.querySelector<HTMLElement>(".poodle-date-picker__value")!;

    expect(value.getAttribute("data-placeholder")).toBe("true");
    expect(value.textContent).toBe("Pick a day");
  });

  it("stays closed and inert when disabled", () => {
    const onOpenChange = vi.fn();
    const { container } = render(<DatePicker disabled onOpenChange={onOpenChange} />);
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-picker__trigger")!;

    expect(trigger.disabled).toBe(true);

    fireEvent.click(trigger);
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).not.toHaveBeenCalled();
  });

  it("reports open change requests while a controlled closed state stays closed", () => {
    const onOpenChange = vi.fn();
    const { container } = render(<DatePicker open={false} onOpenChange={onOpenChange} />);
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-date-picker__trigger")!;

    fireEvent.click(trigger);

    expect(onOpenChange).toHaveBeenCalledWith(true);
    expect(surfaceOf()).toBeNull();
  });
});
