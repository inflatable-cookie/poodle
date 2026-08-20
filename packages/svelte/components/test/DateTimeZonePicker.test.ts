import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import DateTimeZonePicker from "../src/DateTimeZonePicker.svelte";

const timeZoneOptions = [
  { value: "UTC", label: "UTC" },
  { value: "Asia/Tokyo", label: "Tokyo" },
];

describe("DateTimeZonePicker (svelte)", () => {
  // The surface is portalled by the anchored action to the theme root, so it is
  // not reachable from the render container — same pattern as DatePicker.
  const surfaceOf = () => document.querySelector(".poodle-date-time-zone-picker__surface") as HTMLElement | null;
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-date-time-zone-picker__trigger") as HTMLButtonElement;

  it("exposes a dialog trigger with expanded state and shows the placeholder", () => {
    const { container } = render(DateTimeZonePicker, { props: { ariaLabel: "Event time" } });
    const trigger = triggerOf(container);
    expect(trigger.getAttribute("aria-haspopup")).toBe("dialog");
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(container.querySelector(".poodle-date-time-zone-picker__value")?.textContent).toBe(
      "Select date, time, and zone",
    );
    expect(surfaceOf()).toBeNull();
  });

  it("opens a dialog surface composed of calendar, time, and timezone fields", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(DateTimeZonePicker, { props: { ariaLabel: "Event time", onOpenChange } });
    const trigger = triggerOf(container);

    await fireEvent.click(trigger);

    const surface = surfaceOf();
    expect(surface).not.toBeNull();
    expect(surface?.getAttribute("role")).toBe("dialog");
    expect(surface?.querySelector(".poodle-calendar")).not.toBeNull();
    expect(surface?.querySelector('input[type="time"]')).not.toBeNull();
    expect(surface?.querySelector('[role="combobox"]')).not.toBeNull();
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(onOpenChange).toHaveBeenCalledWith(true);
  });

  it("closes on Escape and reports the close without losing the value", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(DateTimeZonePicker, {
      props: {
        defaultValue: { date: "2026-03-14", time: "10:00", timeZone: "America/Los_Angeles" },
        ariaLabel: "Event time",
        onOpenChange,
      },
    });
    const trigger = triggerOf(container);
    expect(container.querySelector(".poodle-date-time-zone-picker__value")?.textContent).toContain(
      "Mar 14, 2026",
    );

    await fireEvent.click(trigger);
    await fireEvent.keyDown(document, { key: "Escape" });

    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
    expect(container.querySelector(".poodle-date-time-zone-picker__value")?.textContent).toContain(
      "Mar 14, 2026",
    );
  });

  it("dismisses on outside mousedown", async () => {
    const { container } = render(DateTimeZonePicker, { props: { ariaLabel: "Event time" } });
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf()).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf()).toBeNull();
  });

  it("commits a portalled timezone option without dismissing the picker", async () => {
    const onValueChange = vi.fn();
    const { container } = render(DateTimeZonePicker, {
      props: {
        ariaLabel: "Event time",
        timeZoneOptions,
        onValueChange,
      },
    });
    await fireEvent.click(triggerOf(container));

    const timezoneInput = surfaceOf()?.querySelector(".poodle-select__input") as HTMLInputElement;
    await fireEvent.focus(timezoneInput);

    const tokyo = [...document.querySelectorAll('[role="option"]')].find(
      (el) => el.getAttribute("data-value") === "Asia/Tokyo",
    ) as HTMLElement;
    expect(tokyo).not.toBeNull();
    // The option lives in Select's portal, not in the picker surface. A
    // synthetic onValueChange call would miss the mousedown-before-commit bug.
    expect(tokyo.closest(".poodle-date-time-zone-picker__surface")).toBeNull();

    await fireEvent.mouseDown(tokyo);
    expect(surfaceOf()).not.toBeNull();
    expect(onValueChange).not.toHaveBeenCalled();

    await fireEvent.click(tokyo);

    expect(onValueChange).toHaveBeenCalledWith(expect.objectContaining({ timeZone: "Asia/Tokyo" }));
    expect(surfaceOf()).not.toBeNull();
  });

  it("dismisses the whole composite in one outside press while the timezone list is open", async () => {
    const { container } = render(DateTimeZonePicker, {
      props: {
        ariaLabel: "Event time",
        timeZoneOptions,
      },
    });
    await fireEvent.click(triggerOf(container));

    const timezoneInput = surfaceOf()?.querySelector(".poodle-select__input") as HTMLInputElement;
    await fireEvent.focus(timezoneInput);
    expect(document.querySelector('[role="option"]')).not.toBeNull();

    await fireEvent.mouseDown(document.body);

    expect(surfaceOf()).toBeNull();
    expect(document.querySelector('[role="option"]')).toBeNull();
  });

  it("unwinds Escape through the nested timezone list before closing the picker", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(DateTimeZonePicker, {
      props: { ariaLabel: "Event time", timeZoneOptions, onOpenChange },
    });
    await fireEvent.click(triggerOf(container));

    const timezoneInput = surfaceOf()?.querySelector(".poodle-select__input") as HTMLInputElement;
    await fireEvent.focus(timezoneInput);
    expect(document.querySelector('[role="option"]')).not.toBeNull();

    await fireEvent.keyDown(document, { key: "Escape" });
    expect(document.querySelector('[role="option"]')).toBeNull();
    expect(surfaceOf()).not.toBeNull();
    expect(onOpenChange).not.toHaveBeenCalledWith(false);

    await fireEvent.keyDown(document, { key: "Escape" });
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
  });

  it("commits the chosen date into the value through the composed calendar", async () => {
    const onValueChange = vi.fn();
    const { container } = render(DateTimeZonePicker, {
      props: { defaultValue: { date: "2026-03-14", time: null, timeZone: null }, ariaLabel: "Event time", onValueChange },
    });
    await fireEvent.click(triggerOf(container));

    const day = document.querySelector('[aria-label="Mar 15, 2026"]') as HTMLElement;
    await fireEvent.click(day);

    expect(onValueChange).toHaveBeenCalledWith(
      expect.objectContaining({ date: "2026-03-15" }),
    );
  });

  it("stays closed and inert when disabled", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(DateTimeZonePicker, {
      props: { disabled: true, ariaLabel: "Event time", onOpenChange },
    });
    const trigger = triggerOf(container);
    expect(trigger.disabled).toBe(true);

    await fireEvent.click(trigger);
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).not.toHaveBeenCalled();
  });

  it("projects size and density on the root", () => {
    const { container } = render(DateTimeZonePicker, {
      props: { size: "lg", density: "compact", ariaLabel: "Event time" },
    });
    const root = container.querySelector(".poodle-date-time-zone-picker") as HTMLElement;
    expect(root.dataset.size).toBe("lg");
    expect(root.dataset.density).toBe("compact");
  });
});