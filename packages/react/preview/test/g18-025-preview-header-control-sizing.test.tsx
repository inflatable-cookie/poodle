import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { UiPresentationProvider } from "@inflatable-cookie/poodle-react";
import { DisplayControls } from "../src/gallery/DisplayControls";

/**
 * Every painted preview-header control —
 * ThemeSelect, both ToggleGroups, the (block) Slider, and the search
 * TextInput — must resolve the selected specimen size together, and the
 * header's journeys (Size, Density, Search, Contrast) must keep firing.
 */

function mountHeader(overrides: {
  sizeScale?: "xs" | "sm" | "md" | "lg" | "xl";
  controlSize?: "xs" | "sm" | "md" | "lg" | "xl";
  onControlSizeChange?: (value: string) => void;
  onDensityChange?: (value: string) => void;
  onSearchChange?: (value: string) => void;
  onContrastChange?: (value: number) => void;
  onThemeChange?: (value: string) => void;
}) {
  const handlers = {
    onThemeChange: overrides.onThemeChange ?? (() => {}),
    onDensityChange: overrides.onDensityChange ?? (() => {}),
    onControlSizeChange: overrides.onControlSizeChange ?? (() => {}),
    onSearchChange: overrides.onSearchChange ?? (() => {}),
    onContrastChange: overrides.onContrastChange ?? (() => {}),
  };
  return render(
    <UiPresentationProvider
      sizeScale={overrides.sizeScale ?? "sm"}
      density="compact"
    >
      <DisplayControls
        theme="eclipse"
        density="compact"
        controlSize={overrides.controlSize ?? "sm"}
        search=""
        contrast={0.5}
        onThemeChange={handlers.onThemeChange}
        onDensityChange={handlers.onDensityChange}
        onControlSizeChange={handlers.onControlSizeChange}
        onSearchChange={handlers.onSearchChange}
        onContrastChange={handlers.onContrastChange}
      />
    </UiPresentationProvider>,
  );
}

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

describe("g18.025 preview header control sizing (react)", () => {
  it("keeps every control together at the xl selection", () => {
    const { container, unmount } = mountHeader({ sizeScale: "xl", controlSize: "xl" });
    assertChromeSize(container, "xl");
    assertSelectionIs(container, sizeGroup(container), "xl");
    unmount();
  });

  it("keeps every control together at the xs selection", () => {
    const { container, unmount } = mountHeader({ sizeScale: "xs", controlSize: "xs" });
    assertChromeSize(container, "xs");
    assertSelectionIs(container, sizeGroup(container), "xs");
    unmount();
  });

  it("moves the complete header through every specimen size stop", () => {
    for (const controlSize of ["xs", "sm", "md", "lg", "xl"] as const) {
      const { container, unmount } = mountHeader({ sizeScale: controlSize, controlSize });
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
    const { container } = mountHeader({
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
    fireEvent.click(sizeGroup(container).querySelector('[data-toggle-value="sm"]')!);
    expect(onControlSizeChange).toHaveBeenCalledWith("sm");
    // …while the density group is a different control and untouched by it.
    fireEvent.click(densityGroup(container).querySelector('[data-toggle-value="comfortable"]')!);
    expect(onDensityChange).toHaveBeenCalledWith("comfortable");
    assertChromeSize(container, "lg");

    // Search journey: typing forwards the committed value.
    const input = container.querySelector<HTMLInputElement>("input.poodle-text-input__control");
    expect(input).not.toBeNull();
    fireEvent.change(input!, { target: { value: "slider" } });
    await Promise.resolve();
    expect(onSearchChange).toHaveBeenCalledWith("slider");

    // Contrast journey: keyboard on the slider handle moves the value by one
    // step (0.5 + 0.05).
    const handle = container.querySelector('[role="slider"]');
    expect(handle).not.toBeNull();
    fireEvent.keyDown(handle!, { key: "ArrowRight" });
    expect(onContrastChange).toHaveBeenCalledWith(0.55);
    assertChromeSize(container, "lg");
  });
});
