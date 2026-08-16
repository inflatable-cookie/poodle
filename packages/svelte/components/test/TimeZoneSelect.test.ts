import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import TimeZoneSelect from "../src/TimeZoneSelect.svelte";

describe("TimeZoneSelect (svelte)", () => {
  // TimeZoneSelect is always searchable, so the trigger is the combobox input.
  const inputOf = (container: HTMLElement) =>
    container.querySelector(".poodle-select__input") as HTMLInputElement;
  const comboboxOf = (container: HTMLElement) =>
    container.querySelector('[role="combobox"]') as HTMLElement;

  it("renders a searchable combobox with the default timezone placeholder", () => {
    const { container } = render(TimeZoneSelect, { props: { ariaLabel: "Time zone" } });
    expect(comboboxOf(container)).not.toBeNull();
    expect(comboboxOf(container).getAttribute("aria-haspopup")).toBe("listbox");
    expect(comboboxOf(container).getAttribute("aria-label")).toBe("Time zone");
    expect(inputOf(container).getAttribute("placeholder")).toBe("Search time zones...");
  });

  it("falls back to the default timezone list when options are empty", async () => {
    const { container } = render(TimeZoneSelect, { props: { ariaLabel: "Time zone" } });
    await fireEvent.focus(inputOf(container));

    const values = [...document.querySelectorAll('[role="option"]')].map((el) =>
      el.getAttribute("data-value"),
    );
    expect(values.length).toBeGreaterThan(50);
    expect(values).toContain("America/New_York");
    expect(values).toContain("Europe/London");
  });

  it("uses host-provided options and reports the picked value", async () => {
    const onValueChange = vi.fn();
    const { container } = render(TimeZoneSelect, {
      props: {
        ariaLabel: "Time zone",
        options: [
          { value: "UTC", label: "UTC" },
          { value: "Asia/Tokyo", label: "Tokyo" },
        ],
        onValueChange,
      },
    });
    await fireEvent.focus(inputOf(container));

    const tokyo = [...document.querySelectorAll('[role="option"]')].find(
      (el) => el.getAttribute("data-value") === "Asia/Tokyo",
    ) as HTMLElement;
    await fireEvent.click(tokyo);

    expect(onValueChange).toHaveBeenCalledWith("Asia/Tokyo");
  });

  it("shows the pre-selected zone label on the trigger", () => {
    const { container } = render(TimeZoneSelect, {
      props: { defaultValue: "America/New_York", ariaLabel: "Time zone" },
    });
    expect(inputOf(container).value).toContain("America/New York");
  });

  it("disables the input when disabled", () => {
    const { container } = render(TimeZoneSelect, {
      props: { disabled: true, ariaLabel: "Time zone" },
    });
    expect(inputOf(container).disabled).toBe(true);
  });
});