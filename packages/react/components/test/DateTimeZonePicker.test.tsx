import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { DateTimeZonePicker } from "../src/DateTimeZonePicker";

const timeZoneOptions = [
  { value: "UTC", label: "UTC" },
  { value: "Asia/Tokyo", label: "Tokyo" },
];

describe("DateTimeZonePicker (react)", () => {
  // The surface is portalled by the anchored surface to the theme root, so it is
  // not reachable from the render container — same pattern as DatePicker.
  const surfaceOf = () => document.querySelector(".poodle-date-time-zone-picker__surface") as HTMLElement | null;
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-date-time-zone-picker__trigger") as HTMLButtonElement;

  it("exposes a dialog trigger with expanded state and shows the placeholder", () => {
    const { container } = render(<DateTimeZonePicker ariaLabel="Event time" />);
    const trigger = triggerOf(container);
    expect(trigger.getAttribute("aria-haspopup")).toBe("dialog");
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(container.querySelector(".poodle-date-time-zone-picker__value")?.textContent).toBe(
      "Select date, time, and zone",
    );
    expect(surfaceOf()).toBeNull();
  });

  it("opens a dialog surface composed of calendar, time, and timezone fields", () => {
    const onOpenChange = vi.fn();
    const { container } = render(<DateTimeZonePicker ariaLabel="Event time" onOpenChange={onOpenChange} />);
    const trigger = triggerOf(container);

    fireEvent.click(trigger);

    const surface = surfaceOf();
    expect(surface).not.toBeNull();
    expect(surface?.getAttribute("role")).toBe("dialog");
    expect(surface?.querySelector(".poodle-calendar")).not.toBeNull();
    expect(surface?.querySelector('input[type="time"]')).not.toBeNull();
    expect(surface?.querySelector('[role="combobox"]')).not.toBeNull();
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(onOpenChange).toHaveBeenCalledWith(true);
  });

  it("closes on Escape and reports the close without losing the value", () => {
    const onOpenChange = vi.fn();
    const { container } = render(
      <DateTimeZonePicker
        defaultValue={{ date: "2026-03-14", time: "10:00", timeZone: "America/Los_Angeles" }}
        ariaLabel="Event time"
        onOpenChange={onOpenChange}
      />,
    );
    const trigger = triggerOf(container);
    expect(container.querySelector(".poodle-date-time-zone-picker__value")?.textContent).toContain(
      "Mar 14, 2026",
    );

    fireEvent.click(trigger);
    fireEvent.keyDown(document, { key: "Escape" });

    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
    expect(container.querySelector(".poodle-date-time-zone-picker__value")?.textContent).toContain(
      "Mar 14, 2026",
    );
  });

  it("dismisses on outside mousedown", () => {
    const { container } = render(<DateTimeZonePicker ariaLabel="Event time" />);
    fireEvent.click(triggerOf(container));
    expect(surfaceOf()).not.toBeNull();

    fireEvent.mouseDown(document.body);
    expect(surfaceOf()).toBeNull();
  });

  it("commits a portalled timezone option without dismissing the picker", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <DateTimeZonePicker
        ariaLabel="Event time"
        timeZoneOptions={timeZoneOptions}
        onValueChange={onValueChange}
      />,
    );
    fireEvent.click(triggerOf(container));

    const timezoneInput = surfaceOf()?.querySelector(".poodle-select__input") as HTMLInputElement;
    fireEvent.focus(timezoneInput);

    const tokyo = [...document.querySelectorAll('[role="option"]')].find(
      (el) => el.getAttribute("data-value") === "Asia/Tokyo",
    ) as HTMLElement;
    expect(tokyo).not.toBeNull();
    // The option lives in Select's portal, not in the picker surface. A
    // synthetic onValueChange call would miss the mousedown-before-commit bug.
    expect(tokyo.closest(".poodle-date-time-zone-picker__surface")).toBeNull();

    fireEvent.mouseDown(tokyo);
    expect(surfaceOf()).not.toBeNull();
    expect(onValueChange).not.toHaveBeenCalled();

    fireEvent.click(tokyo);

    expect(onValueChange).toHaveBeenCalledWith(expect.objectContaining({ timeZone: "Asia/Tokyo" }));
    expect(surfaceOf()).not.toBeNull();
  });

  it("dismisses the whole composite in one outside press while the timezone list is open", () => {
    const { container } = render(
      <DateTimeZonePicker
        ariaLabel="Event time"
        timeZoneOptions={timeZoneOptions}
      />,
    );
    fireEvent.click(triggerOf(container));

    const timezoneInput = surfaceOf()?.querySelector(".poodle-select__input") as HTMLInputElement;
    fireEvent.focus(timezoneInput);
    expect(document.querySelector('[role="option"]')).not.toBeNull();

    fireEvent.mouseDown(document.body);

    expect(surfaceOf()).toBeNull();
    expect(document.querySelector('[role="option"]')).toBeNull();
  });

  it("unwinds Escape through the nested timezone list before closing the picker", () => {
    const onOpenChange = vi.fn();
    const { container } = render(
      <DateTimeZonePicker ariaLabel="Event time" timeZoneOptions={timeZoneOptions} onOpenChange={onOpenChange} />,
    );
    fireEvent.click(triggerOf(container));

    const timezoneInput = surfaceOf()?.querySelector(".poodle-select__input") as HTMLInputElement;
    fireEvent.focus(timezoneInput);
    expect(document.querySelector('[role="option"]')).not.toBeNull();

    fireEvent.keyDown(document, { key: "Escape" });
    expect(document.querySelector('[role="option"]')).toBeNull();
    expect(surfaceOf()).not.toBeNull();
    expect(onOpenChange).not.toHaveBeenCalledWith(false);

    fireEvent.keyDown(document, { key: "Escape" });
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
  });

  it("commits the chosen date into the value through the composed calendar", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <DateTimeZonePicker
        defaultValue={{ date: "2026-03-14", time: null, timeZone: null }}
        ariaLabel="Event time"
        onValueChange={onValueChange}
      />,
    );
    fireEvent.click(triggerOf(container));

    const day = document.querySelector('[aria-label="Mar 15, 2026"]') as HTMLElement;
    fireEvent.click(day);

    expect(onValueChange).toHaveBeenCalledWith(
      expect.objectContaining({ date: "2026-03-15" }),
    );
  });

  it("stays closed and inert when disabled", () => {
    const onOpenChange = vi.fn();
    const { container } = render(
      <DateTimeZonePicker disabled ariaLabel="Event time" onOpenChange={onOpenChange} />,
    );
    const trigger = triggerOf(container);
    expect(trigger.disabled).toBe(true);

    fireEvent.click(trigger);
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).not.toHaveBeenCalled();
  });

  it("projects size and density on the root", () => {
    const { container } = render(
      <DateTimeZonePicker size="lg" density="compact" ariaLabel="Event time" />,
    );
    const root = container.querySelector(".poodle-date-time-zone-picker") as HTMLElement;
    expect(root.dataset.size).toBe("lg");
    expect(root.dataset.density).toBe("compact");
  });
});