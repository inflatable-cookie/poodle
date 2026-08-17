import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import TriStateSwitch from "../src/TriStateSwitch.svelte";

describe("TriStateSwitch (svelte)", () => {
  it("renders a radiogroup with three radios in fixed order", () => {
    const { container } = render(TriStateSwitch, { props: { ariaLabel: "Filter mode" } });
    const root = container.querySelector(".poodle-tri-state-switch") as HTMLElement;
    expect(root.getAttribute("role")).toBe("radiogroup");
    expect(root.getAttribute("aria-label")).toBe("Filter mode");

    const radios = Array.from(root.querySelectorAll("input[type='radio']")) as HTMLInputElement[];
    expect(radios.map((radio) => radio.getAttribute("aria-label"))).toEqual([
      "Exclude",
      "Default",
      "Include",
    ]);
    expect(radios.map((radio) => radio.checked)).toEqual([false, true, false]);
  });

  it("reflects the controlled value in checked state and data attributes", () => {
    const { container } = render(TriStateSwitch, {
      props: { value: "included", ariaLabel: "Filter mode" },
    });
    const root = container.querySelector(".poodle-tri-state-switch") as HTMLElement;
    const radios = Array.from(root.querySelectorAll("input[type='radio']")) as HTMLInputElement[];
    expect(radios.map((radio) => radio.checked)).toEqual([false, false, true]);
    expect(root.dataset.state).toBe("included");
  });

  it("reports value changes and updates the shared state", async () => {
    const onValueChange = vi.fn();
    const { container } = render(TriStateSwitch, {
      props: { value: "default", ariaLabel: "Filter mode", onValueChange },
    });
    const radios = Array.from(
      container.querySelectorAll("input[type='radio']"),
    ) as HTMLInputElement[];

    await fireEvent.change(radios[0]);
    expect(onValueChange).toHaveBeenCalledWith("excluded");
  });

  it("renders custom option labels", () => {
    const { container } = render(TriStateSwitch, {
      props: {
        ariaLabel: "Visibility filter",
        options: { excluded: "Hide", default: "All", included: "Show" },
      },
    });
    const labels = Array.from(
      container.querySelectorAll(".poodle-tri-state-switch__segment"),
    ).map((el) => el.textContent);
    expect(labels).toEqual(["Hide", "All", "Show"]);
  });

  it("disables every radio when disabled", () => {
    const { container } = render(TriStateSwitch, {
      props: { value: "excluded", disabled: true, ariaLabel: "Disabled switch" },
    });
    const root = container.querySelector(".poodle-tri-state-switch") as HTMLElement;
    const radios = Array.from(root.querySelectorAll("input[type='radio']")) as HTMLInputElement[];
    expect(radios.every((radio) => radio.disabled)).toBe(true);
    expect(root.getAttribute("aria-disabled")).toBe("true");
  });
});