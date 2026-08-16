import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import DatePicker from "../src/DatePicker.svelte";

// The surface is portalled by the anchored action to the theme root, so it is
// not reachable from the render container.
function surfaceOf(): HTMLElement | null {
  return document.querySelector(".poodle-date-picker__surface");
}

function day(label: string): HTMLButtonElement {
  return document.querySelector(`[aria-label="${label}"]`) as HTMLButtonElement;
}

describe("DatePicker (svelte)", () => {
  it("opens a dialog surface with the calendar on trigger click", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(DatePicker, { props: { defaultValue: "2026-03-14", onOpenChange } });
    const trigger = container.querySelector(".poodle-date-picker__trigger") as HTMLButtonElement;

    expect(trigger.getAttribute("aria-haspopup")).toBe("dialog");
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(surfaceOf()).toBeNull();

    await fireEvent.click(trigger);

    expect(surfaceOf()?.getAttribute("role")).toBe("dialog");
    expect(surfaceOf()?.querySelector(".poodle-calendar")).toBeTruthy();
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(onOpenChange).toHaveBeenCalledWith(true);
  });

  it("reports the selected ISO date, closes the overlay, and formats the trigger", async () => {
    const onValueChange = vi.fn();
    const onOpenChange = vi.fn();
    const { container } = render(DatePicker, {
      props: { defaultValue: "2026-03-14", onValueChange, onOpenChange },
    });
    const trigger = container.querySelector(".poodle-date-picker__trigger") as HTMLButtonElement;

    await fireEvent.click(trigger);
    await fireEvent.click(day("Mar 15, 2026"));

    expect(onValueChange).toHaveBeenCalledWith("2026-03-15");
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
    expect(container.querySelector(".poodle-date-picker__value")?.textContent).toBe("Mar 15, 2026");
  });

  it("closes on Escape and leaves the committed value untouched", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(DatePicker, { props: { defaultValue: "2026-03-14", onOpenChange } });
    const trigger = container.querySelector(".poodle-date-picker__trigger") as HTMLButtonElement;

    await fireEvent.click(trigger);
    await fireEvent.keyDown(document, { key: "Escape" });

    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
    expect(container.querySelector(".poodle-date-picker__value")?.textContent).toBe("Mar 14, 2026");
  });

  it("dismisses on outside mousedown", async () => {
    const { container } = render(DatePicker, { props: { defaultValue: "2026-03-14" } });
    const trigger = container.querySelector(".poodle-date-picker__trigger") as HTMLButtonElement;

    await fireEvent.click(trigger);
    expect(surfaceOf()).toBeTruthy();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf()).toBeNull();
  });

  it("shows the placeholder when no value is set", () => {
    const { container } = render(DatePicker, { props: { placeholder: "Pick a day" } });
    const value = container.querySelector(".poodle-date-picker__value") as HTMLElement;

    expect(value.getAttribute("data-placeholder")).toBe("true");
    expect(value.textContent).toBe("Pick a day");
  });

  it("stays closed and inert when disabled", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(DatePicker, { props: { disabled: true, onOpenChange } });
    const trigger = container.querySelector(".poodle-date-picker__trigger") as HTMLButtonElement;

    expect(trigger.disabled).toBe(true);

    await fireEvent.click(trigger);
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).not.toHaveBeenCalled();
  });

  it("reports open change requests while a controlled closed state stays closed", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(DatePicker, { props: { open: false, onOpenChange } });
    const trigger = container.querySelector(".poodle-date-picker__trigger") as HTMLButtonElement;

    await fireEvent.click(trigger);

    expect(onOpenChange).toHaveBeenCalledWith(true);
    expect(surfaceOf()).toBeNull();
  });
});
