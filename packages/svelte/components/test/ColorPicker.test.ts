import { hexToHsv, hsvToHex, hsvToRgb, rgbToHsv } from "@inflatable-cookie/poodle-core";
import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import ColorPicker from "../src/ColorPicker.svelte";

// The surface is portalled by the anchored action to the theme root, so it is
// not reachable from the render container.
function surfaceOf(): HTMLElement | null {
  return document.querySelector(".poodle-color-picker__surface");
}

describe("ColorPicker (svelte)", () => {
  it("opens the surface on trigger click and closes it on Escape", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(ColorPicker, { props: { value: "#6366f1", onOpenChange } });
    const trigger = container.querySelector(".poodle-color-picker__trigger") as HTMLButtonElement;

    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(trigger.getAttribute("aria-haspopup")).toBe("dialog");

    await fireEvent.click(trigger);
    expect(surfaceOf()?.getAttribute("role")).toBe("dialog");
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(onOpenChange).toHaveBeenCalledWith(true);

    await fireEvent.keyDown(document, { key: "Escape" });
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it("selects a swatch and reports the normalized hex with selected semantics", async () => {
    const onChange = vi.fn();
    render(ColorPicker, {
      props: {
        value: "#6366f1",
        swatches: ["#ef4444", "#22c55e"],
        defaultOpen: true,
        onChange,
      },
    });

    const selected = document.querySelector('[aria-label="#ef4444"]') as HTMLButtonElement;
    const other = document.querySelector('[aria-label="#22c55e"]') as HTMLButtonElement;

    expect(selected.getAttribute("aria-selected")).toBe("false");

    await fireEvent.click(selected);

    expect(onChange).toHaveBeenCalledWith("#ef4444");
    expect(selected.getAttribute("aria-selected")).toBe("true");
    expect(selected.classList.contains("poodle-color-picker__swatch--active")).toBe(true);
    expect(other.getAttribute("aria-selected")).toBe("false");
    expect((document.querySelector('[aria-label="Hex color value"]') as HTMLInputElement).value).toBe(
      "#ef4444",
    );
  });

  it("adjusts saturation on the gradient pad with arrow keys and commits", async () => {
    const onChange = vi.fn();
    render(ColorPicker, { props: { value: "#6366f1", defaultOpen: true, onChange } });

    const gradient = document.querySelector(
      '[role="slider"][aria-label="Saturation and brightness"]',
    ) as HTMLElement;
    const { h, s, v } = hexToHsv("#6366f1");

    expect(gradient.getAttribute("aria-valuetext")).toBe(`Saturation ${s}%, Brightness ${v}%`);

    await fireEvent.keyDown(gradient, { key: "ArrowRight" });

    expect(onChange).toHaveBeenCalledWith(hsvToHex(h, s + 1, v));
    expect(gradient.getAttribute("aria-valuetext")).toBe(`Saturation ${s + 1}%, Brightness ${v}%`);
  });

  it("commits a hex typed into the inline trigger input", async () => {
    const onChange = vi.fn();
    render(ColorPicker, { props: { value: "#6366f1", onChange } });

    const input = document.querySelector('[aria-label="Hex color value"]') as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "#ff0000" } });

    expect(onChange).toHaveBeenCalledWith("#ff0000");
  });

  it("edits a channel in RGB mode and exposes the alpha control when enabled", async () => {
    const onChange = vi.fn();
    render(ColorPicker, {
      props: {
        value: "#22c55e",
        defaultMode: "rgb",
        showAlpha: true,
        defaultOpen: true,
        onChange,
      },
    });

    expect(document.querySelector('[aria-label="Opacity"]')).toBeTruthy();
    expect(document.querySelector('[aria-label="Alpha"]')).toBeTruthy();

    const { h, s, v } = hexToHsv("#22c55e");
    const { g, b } = hsvToRgb(h, s, v);
    const red = document.querySelector('[aria-label="Red"]') as HTMLInputElement;

    await fireEvent.input(red, { target: { value: "200" } });

    const adjusted = rgbToHsv(200, g, b);
    expect(onChange).toHaveBeenCalledWith(hsvToHex(adjusted.h, adjusted.s, adjusted.v));
  });

  it("stays closed and inert when disabled", async () => {
    const onOpenChange = vi.fn();
    const { container } = render(ColorPicker, {
      props: { value: "#22c55e", disabled: true, onOpenChange },
    });
    const trigger = container.querySelector(".poodle-color-picker__trigger") as HTMLButtonElement;

    expect(trigger.disabled).toBe(true);
    expect(container.querySelector(".poodle-color-picker")?.getAttribute("data-disabled")).toBe("true");

    await fireEvent.click(trigger);
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).not.toHaveBeenCalled();
  });
});
