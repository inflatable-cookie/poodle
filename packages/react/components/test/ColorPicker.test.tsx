import { hexToHsv, hsvToHex, hsvToRgb, rgbToHsv } from "@inflatable-cookie/poodle-core";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { ColorPicker } from "../src/ColorPicker";

// The surface is portalled by the anchored surface to the theme root, so it is
// not reachable from the render container.
function surfaceOf(): HTMLElement | null {
  return document.querySelector(".poodle-color-picker__surface");
}

describe("ColorPicker (react)", () => {
  it("opens the surface on trigger click and closes it on Escape", () => {
    const onOpenChange = vi.fn();
    const { container } = render(<ColorPicker value="#6366f1" onOpenChange={onOpenChange} />);
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-color-picker__trigger")!;

    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(trigger.getAttribute("aria-haspopup")).toBe("dialog");

    fireEvent.click(trigger);
    expect(surfaceOf()?.getAttribute("role")).toBe("dialog");
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(onOpenChange).toHaveBeenCalledWith(true);

    fireEvent.keyDown(document, { key: "Escape" });
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it("selects a swatch and reports the normalized hex with selected semantics", () => {
    const onChange = vi.fn();
    render(
      <ColorPicker value="#6366f1" swatches={["#ef4444", "#22c55e"]} defaultOpen onChange={onChange} />,
    );

    const selected = document.querySelector<HTMLButtonElement>('[aria-label="#ef4444"]')!;
    const other = document.querySelector<HTMLButtonElement>('[aria-label="#22c55e"]')!;

    expect(selected.getAttribute("aria-selected")).toBe("false");

    fireEvent.click(selected);

    expect(onChange).toHaveBeenCalledWith("#ef4444");
    expect(selected.getAttribute("aria-selected")).toBe("true");
    expect(selected.classList.contains("poodle-color-picker__swatch--active")).toBe(true);
    expect(other.getAttribute("aria-selected")).toBe("false");
    expect((document.querySelector('[aria-label="Hex color value"]') as HTMLInputElement).value).toBe(
      "#ef4444",
    );
  });

  it("adjusts saturation on the gradient pad with arrow keys and commits", () => {
    const onChange = vi.fn();
    render(<ColorPicker value="#6366f1" defaultOpen onChange={onChange} />);

    const gradient = document.querySelector<HTMLElement>(
      '[role="slider"][aria-label="Saturation and brightness"]',
    )!;
    const { h, s, v } = hexToHsv("#6366f1");

    expect(gradient.getAttribute("aria-valuetext")).toBe(`Saturation ${s}%, Brightness ${v}%`);

    fireEvent.keyDown(gradient, { key: "ArrowRight" });

    expect(onChange).toHaveBeenCalledWith(hsvToHex(h, s + 1, v));
    expect(gradient.getAttribute("aria-valuetext")).toBe(`Saturation ${s + 1}%, Brightness ${v}%`);
  });

  it("commits a hex typed into the inline trigger input", () => {
    const onChange = vi.fn();
    render(<ColorPicker value="#6366f1" onChange={onChange} />);

    const input = document.querySelector<HTMLInputElement>('[aria-label="Hex color value"]')!;
    fireEvent.change(input, { target: { value: "#ff0000" } });

    expect(onChange).toHaveBeenCalledWith("#ff0000");
  });

  it("edits a channel in RGB mode and exposes the alpha control when enabled", () => {
    const onChange = vi.fn();
    render(
      <ColorPicker value="#22c55e" defaultMode="rgb" showAlpha defaultOpen onChange={onChange} />,
    );

    expect(document.querySelector('[aria-label="Opacity"]')).toBeTruthy();
    expect(document.querySelector('[aria-label="Alpha"]')).toBeTruthy();

    const { h, s, v } = hexToHsv("#22c55e");
    const { g, b } = hsvToRgb(h, s, v);
    const red = document.querySelector<HTMLInputElement>('[aria-label="Red"]')!;

    fireEvent.change(red, { target: { value: "200" } });

    const expected = hsvToHex(...Object.values(rgbToHsv(200, g, b)));
    expect(onChange).toHaveBeenCalledWith(expected);
  });

  it("stays closed and inert when disabled", () => {
    const onOpenChange = vi.fn();
    const { container } = render(<ColorPicker value="#22c55e" disabled onOpenChange={onOpenChange} />);
    const trigger = container.querySelector<HTMLButtonElement>(".poodle-color-picker__trigger")!;

    expect(trigger.disabled).toBe(true);
    expect(container.querySelector(".poodle-color-picker")?.getAttribute("data-disabled")).toBe("true");

    fireEvent.click(trigger);
    expect(surfaceOf()).toBeNull();
    expect(onOpenChange).not.toHaveBeenCalled();
  });
});
