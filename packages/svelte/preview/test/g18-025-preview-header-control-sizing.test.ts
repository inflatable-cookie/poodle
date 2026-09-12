import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Harness from "./HeaderSizingHarness.svelte";

/**
 * g18.025: the preview header is fixed `md` chrome. Every painted control —
 * ThemeSelect, both ToggleGroups, the (block) Slider, and the search
 * TextInput — must resolve `md` no matter which specimen size the ambient
 * app-shell scale carries, and the header's journeys (Size, Density, Search,
 * Contrast) must keep firing while the chrome stays pinned.
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

function assertFixedChrome(container: HTMLElement): void {
  const theme = container.querySelector(".poodle-theme-select");
  const groups = container.querySelectorAll(".poodle-toggle-group");
  const slider = container.querySelector(".poodle-slider");
  const input = container.querySelector(".poodle-text-input");
  expect(groups.length).toBe(2);
  expect(theme?.getAttribute("data-size")).toBe("md");
  for (const group of groups) {
    expect(group.getAttribute("data-size")).toBe("md");
  }
  expect(slider?.getAttribute("data-size")).toBe("md");
  expect(input?.getAttribute("data-size")).toBe("md");
}

function assertSelectionIs(container: HTMLElement, group: HTMLElement, value: string): void {
  const item = group.querySelector(`[data-toggle-value="${value}"]`);
  expect(item).not.toBeNull();
  expect(item?.getAttribute("data-selected")).toBe("true");
}

describe("g18.025 preview header control sizing (svelte)", () => {
  it("keeps every control at the fixed md chrome while the ambient scale is xl", () => {
    const { container } = render(Harness, { sizeScale: "xl", controlSize: "xl" });
    assertFixedChrome(container);
    // The header still reflects the xl selection — only the chrome is pinned.
    assertSelectionIs(container, sizeGroup(container), "xl");
  });

  it("keeps every control at the fixed md chrome while the ambient scale is xs", () => {
    const { container } = render(Harness, { sizeScale: "xs", controlSize: "xs" });
    assertFixedChrome(container);
    assertSelectionIs(container, sizeGroup(container), "xs");
  });

  it("keeps the md chrome through every specimen size stop", () => {
    for (const controlSize of ["xs", "sm", "md", "lg", "xl"] as const) {
      const { container, unmount } = render(Harness, { sizeScale: controlSize, controlSize });
      assertFixedChrome(container);
      assertSelectionIs(container, sizeGroup(container), controlSize);
      unmount();
    }
  });

  it("keeps the five header journeys firing while the chrome stays pinned", async () => {
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
    assertFixedChrome(container);

    // Size journey: selecting a specimen size changes the catalogue axis…
    await fireEvent.click(sizeGroup(container).querySelector('[data-toggle-value="sm"]')!);
    expect(onControlSizeChange).toHaveBeenCalledWith("sm");
    // …while the density group is a different control and untouched by it.
    await fireEvent.click(densityGroup(container).querySelector('[data-toggle-value="comfortable"]')!);
    expect(onDensityChange).toHaveBeenCalledWith("comfortable");
    assertFixedChrome(container);

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
    assertFixedChrome(container);
  });
});
