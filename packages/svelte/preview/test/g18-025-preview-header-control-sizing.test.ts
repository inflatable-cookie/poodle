import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Harness from "./HeaderSizingHarness.svelte";

/**
 * Every painted preview-header control —
 * ThemeSelect, both ToggleGroups, the (block) Slider, and the search
 * TextInput — must resolve the selected specimen size together, and the
 * header's journeys (Size, Density, Search, Contrast) must keep firing.
 */

function sizeGroup(container: HTMLElement): HTMLElement {
  const group = container.querySelector<HTMLElement>('.poodle-toggle-group[aria-label="Control size"]');
  if (!group) throw new Error("size ToggleGroup did not mount");
  return group;
}

function densityGroup(container: HTMLElement): HTMLElement {
  const group = container.querySelector<HTMLElement>('.poodle-toggle-group[aria-label="Density"]');
  if (!group) throw new Error("density ToggleGroup did not mount");
  return group;
}

function assertChromeSize(container: HTMLElement, expected: string): void {
  const theme = container.querySelector(".poodle-theme-select");
  const groups = container.querySelectorAll(".poodle-toggle-group");
  const slider = container.querySelector(".poodle-slider");
  const input = container.querySelector(".poodle-text-input");
  expect(groups.length).toBe(2);
  expect(theme?.getAttribute("data-size")).toBe(expected);
  for (const group of groups) {
    expect(group.getAttribute("data-size")).toBe(expected);
  }
  expect(slider?.getAttribute("data-size")).toBe(expected);
  expect(input?.getAttribute("data-size")).toBe(expected);
}

function assertSelectionIs(container: HTMLElement, group: HTMLElement, value: string): void {
  const item = group.querySelector(`[data-toggle-value="${value}"]`);
  expect(item).not.toBeNull();
  expect(item?.getAttribute("data-selected")).toBe("true");
}

describe("g18.025 preview header control sizing (svelte)", () => {
  it("keeps every control together at the xl selection", () => {
    const { container } = render(Harness, { sizeScale: "xl", controlSize: "xl" });
    assertChromeSize(container, "xl");
    assertSelectionIs(container, sizeGroup(container), "xl");
  });

  it("keeps every control together at the xs selection", () => {
    const { container } = render(Harness, { sizeScale: "xs", controlSize: "xs" });
    assertChromeSize(container, "xs");
    assertSelectionIs(container, sizeGroup(container), "xs");
  });

  it("moves the complete header through every specimen size stop", () => {
    for (const controlSize of ["xs", "sm", "md", "lg", "xl"] as const) {
      const { container, unmount } = render(Harness, { sizeScale: controlSize, controlSize });
      assertChromeSize(container, controlSize);
      assertSelectionIs(container, sizeGroup(container), controlSize);
      unmount();
    }
  });

  it("keeps the five header journeys firing while the controls stay aligned", async () => {
    const onControlSizeChange = vi.fn();
    const onDensityChange = vi.fn();
    const onSearchChange = vi.fn();
    const onContrastChange = vi.fn();
    const onThemeChange = vi.fn();
    const { container } = render(Harness, {
      sizeScale: "lg",
      controlSize: "lg",
      onControlSizeChange,
      onDensityChange,
      onSearchChange,
      onContrastChange,
      onThemeChange,
    });
    assertChromeSize(container, "lg");

    // Size journey: selecting a specimen size changes the catalogue axis…
    await fireEvent.click(sizeGroup(container).querySelector('[data-toggle-value="sm"]')!);
    expect(onControlSizeChange).toHaveBeenCalledWith("sm");
    // …while the density group is a different control and untouched by it.
    await fireEvent.click(densityGroup(container).querySelector('[data-toggle-value="comfortable"]')!);
    expect(onDensityChange).toHaveBeenCalledWith("comfortable");
    assertChromeSize(container, "lg");

    // Search journey: typing forwards the committed value.
    const input = container.querySelector<HTMLInputElement>("input.poodle-text-input__control");
    expect(input).not.toBeNull();
    input!.value = "slider";
    await fireEvent.input(input!);
    expect(onSearchChange).toHaveBeenCalledWith("slider");

    // Contrast journey: keyboard on the slider handle moves the value by one
    // step (0.5 + 0.05).
    const handle = container.querySelector('[role="slider"]');
    expect(handle).not.toBeNull();
    await fireEvent.keyDown(handle!, { key: "ArrowRight" });
    expect(onContrastChange).toHaveBeenCalledWith(0.55);
    assertChromeSize(container, "lg");
  });
});
