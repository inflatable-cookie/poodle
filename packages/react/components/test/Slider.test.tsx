import { fireEvent, render } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { describe, expect, it, vi } from "vitest";

import { Slider } from "../src/Slider";

const css = readFileSync(
  new URL("../../../core/src/styles/slider.css", `file://${import.meta.dirname}/`),
  "utf8",
);

function mockTrack(root: HTMLElement, width: number, height: number): void {
  root.getBoundingClientRect = () =>
    ({
      x: 0,
      y: 0,
      left: 0,
      top: 0,
      right: width,
      bottom: height,
      width,
      height,
      toJSON: () => ({}),
    }) as DOMRect;
  root.setPointerCapture ??= () => {};
  root.releasePointerCapture ??= () => {};
}

describe("Slider (react)", () => {
  it("drives the fill percentage custom property from the value", () => {
    const { container } = render(<Slider value={65} ariaLabel="Volume" />);
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("style")).toContain("--poodle-slider-percent: 65%");
  });

  it("exposes slider ARIA fields and a tab stop", () => {
    const { container } = render(
      <Slider value={40} min={0} max={100} orientation="vertical" ariaLabel="Gain" valueText="quiet" />,
    );
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("role")).toBe("slider");
    expect(root.getAttribute("aria-label")).toBe("Gain");
    expect(root.getAttribute("aria-valuemin")).toBe("0");
    expect(root.getAttribute("aria-valuemax")).toBe("100");
    expect(root.getAttribute("aria-valuenow")).toBe("40");
    expect(root.getAttribute("aria-valuetext")).toBe("quiet");
    expect(root.getAttribute("aria-orientation")).toBe("vertical");
    expect(root.getAttribute("tabindex")).toBe("0");
  });

  it("drops the tab stop while disabled and ignores pointer and keyboard", () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(
      <Slider value={40} disabled ariaLabel="Gain" onValueChange={onValueChange} onValueCommit={onValueCommit} />,
    );
    const root = container.querySelector<HTMLElement>(".poodle-slider")!;
    mockTrack(root, 100, 32);
    fireEvent.pointerDown(root, { button: 0, clientX: 80, clientY: 16, pointerId: 1 });
    fireEvent.keyDown(root, { key: "ArrowRight" });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(onValueCommit).not.toHaveBeenCalled();
    expect(root.getAttribute("tabindex")).toBeNull();
  });

  it("omitting variant renders the block capsule as the default", () => {
    const { container } = render(<Slider value={50} ariaLabel="Volume" />);
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("data-variant")).toBe("block");
    expect(container.querySelector(".poodle-slider__capsule")).not.toBeNull();
    expect(container.querySelector(".poodle-slider__control")).toBeNull();
  });

  it("anchors bipolar block fill at the center", () => {
    const { container } = render(
      <Slider value={-0.5} min={-1} max={1} step={0.01} polarity="bipolar" ariaLabel="Drive" />,
    );
    const style = container.querySelector(".poodle-slider")!.getAttribute("style");
    expect(style).toContain("--poodle-slider-fill-start: 25%");
    expect(style).toContain("--poodle-slider-fill-span: 25%");
    expect(css).toContain("inset-inline-start: var(--poodle-slider-fill-start)");
    expect(css).toContain("width: var(--poodle-slider-fill-span)");
  });

  it("renders embedded as the dense track alternative", () => {
    const { container } = render(<Slider variant="embedded" value={50} ariaLabel="Volume" />);
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("data-variant")).toBe("embedded");
    expect(container.querySelector(".poodle-slider__capsule")).toBeNull();
  });
});

describe("Slider (react) embedded semantics", () => {
  it("normalizes a horizontal pointer along the track and commits once", () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(
      <Slider
        variant="embedded"
        defaultValue={0}
        min={0}
        max={100}
        step={10}
        ariaLabel="Gain"
        onValueChange={onValueChange}
        onValueCommit={onValueCommit}
      />,
    );
    const root = container.querySelector<HTMLElement>(".poodle-slider")!;
    mockTrack(root, 100, 20);

    fireEvent.pointerDown(root, { button: 0, clientX: 44, clientY: 10, pointerId: 1 });
    expect(onValueChange).toHaveBeenLastCalledWith(40);
    fireEvent.pointerMove(root, { clientX: 76, clientY: 10, pointerId: 1 });
    expect(onValueChange).toHaveBeenLastCalledWith(80);
    fireEvent.pointerUp(root, { pointerId: 1 });
    expect(onValueCommit).toHaveBeenCalledOnce();
    expect(onValueCommit).toHaveBeenCalledWith(80);
  });

  it("normalizes a vertical pointer from the bottom", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <Slider
        variant="embedded"
        orientation="vertical"
        defaultValue={0}
        min={0}
        max={100}
        step={10}
        ariaLabel="Gain"
        onValueChange={onValueChange}
      />,
    );
    const root = container.querySelector<HTMLElement>(".poodle-slider")!;
    mockTrack(root, 20, 100);

    fireEvent.pointerDown(root, { button: 0, clientX: 10, clientY: 80, pointerId: 1 });
    expect(onValueChange).toHaveBeenLastCalledWith(20);
    fireEvent.pointerMove(root, { clientX: 10, clientY: 0, pointerId: 1 });
    expect(onValueChange).toHaveBeenLastCalledWith(100);
  });

  it("emits change then commit for arrows, Home, and End", () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(
      <Slider
        variant="embedded"
        defaultValue={50}
        min={0}
        max={100}
        step={10}
        ariaLabel="Gain"
        onValueChange={onValueChange}
        onValueCommit={onValueCommit}
      />,
    );
    const root = container.querySelector<HTMLElement>(".poodle-slider")!;

    fireEvent.keyDown(root, { key: "ArrowRight" });
    expect(onValueChange).toHaveBeenLastCalledWith(60);
    expect(onValueCommit).toHaveBeenLastCalledWith(60);
    fireEvent.keyDown(root, { key: "ArrowUp" });
    expect(onValueChange).toHaveBeenLastCalledWith(70);
    fireEvent.keyDown(root, { key: "ArrowLeft" });
    expect(onValueChange).toHaveBeenLastCalledWith(60);
    fireEvent.keyDown(root, { key: "ArrowDown" });
    expect(onValueChange).toHaveBeenLastCalledWith(50);
    fireEvent.keyDown(root, { key: "Home" });
    expect(onValueChange).toHaveBeenLastCalledWith(0);
    expect(onValueCommit).toHaveBeenLastCalledWith(0);
    fireEvent.keyDown(root, { key: "End" });
    expect(onValueChange).toHaveBeenLastCalledWith(100);
    expect(onValueCommit).toHaveBeenLastCalledWith(100);
    expect(onValueChange).toHaveBeenCalledTimes(6);
    expect(onValueCommit).toHaveBeenCalledTimes(6);
  });

  it("ignores pointer and keyboard while disabled", () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(
      <Slider
        variant="embedded"
        value={40}
        disabled
        ariaLabel="Gain"
        onValueChange={onValueChange}
        onValueCommit={onValueCommit}
      />,
    );
    const root = container.querySelector<HTMLElement>(".poodle-slider")!;
    mockTrack(root, 100, 20);
    fireEvent.pointerDown(root, { button: 0, clientX: 80, clientY: 10, pointerId: 1 });
    fireEvent.keyDown(root, { key: "ArrowRight" });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(onValueCommit).not.toHaveBeenCalled();
    expect(root.getAttribute("tabindex")).toBeNull();
  });
});

describe("Slider (react) block variant", () => {
  it("does not paint ariaLabel as visible text", () => {
    const { container } = render(<Slider value={50} ariaLabel="Gain" />);
    expect(container.textContent).not.toContain("Gain");
    expect(container.querySelector(".poodle-slider")!.getAttribute("aria-label")).toBe("Gain");
  });

  it("does not use visibleLabel as the accessible name", () => {
    const { container } = render(<Slider value={50} visibleLabel="Blur" />);
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("aria-label")).toBeNull();
    expect(container.querySelector(".poodle-slider__capsule")!.getAttribute("aria-hidden")).toBe(
      "true",
    );
  });

  it("keeps a 44px auto hit target at xs", () => {
    const { container } = render(<Slider value={50} size="xs" ariaLabel="Gain" />);
    const root = container.querySelector<HTMLElement>(".poodle-slider")!;
    expect(root.getAttribute("data-variant")).toBe("block");
    expect(container.querySelector(".poodle-slider__hit")).not.toBeNull();
    expect(css).toContain("--poodle-slider-block-hit: 44px");
    expect(css).toContain("pointer-events: auto");
    // g18.024: the accessibility envelope is measurable but never spacing.
    expect(css).toContain("min-height: var(--poodle-slider-block-height)");
    expect(css).not.toContain("max(var(--poodle-slider-block");
    expect(css).not.toContain("--poodle-slider-block-min-height");
  });

  it("consumes the shared control-size axis with no private ladder", () => {
    // The shared ladder is 24/28/36/44/52px for xs–xl; the block capsule
    // matches Button/Input/Select edge for edge at every size.
    for (const [size, rem] of [
      ["xs", "1.5rem"],
      ["sm", "1.75rem"],
      ["md", "2.25rem"],
      ["lg", "2.75rem"],
      ["xl", "3.25rem"],
    ] as const) {
      expect(css).toContain(
        `.poodle-slider[data-variant="block"][data-size="${size}"] { --poodle-slider-block-height: ${rem};`,
      );
    }
    // Density never inflates the block capsule.
    expect(css).toContain('.poodle-slider[data-variant="embedded"][data-density="compact"]');
    expect(css).not.toContain('.poodle-slider[data-density="compact"]');
  });

  it("dispatches from the hit outside the capsule footprint", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <Slider defaultValue={50} min={0} max={100} step={10} ariaLabel="Gain" onValueChange={onValueChange} />,
    );
    const root = container.querySelector<HTMLElement>(".poodle-slider")!;
    const hit = container.querySelector<HTMLElement>(".poodle-slider__hit")!;
    root.getBoundingClientRect = () => ({ left: 0, top: 0, width: 200, height: 44, right: 200, bottom: 44, x: 0, y: 0, toJSON: () => ({}) } as DOMRect);
    hit.getBoundingClientRect = () => ({ left: 78, top: -8, width: 44, height: 44, right: 122, bottom: 36, x: 78, y: -8, toJSON: () => ({}) } as DOMRect);
    hit.setPointerCapture = vi.fn();
    fireEvent.pointerDown(hit, { button: 0, pointerId: 1, clientX: 100, clientY: 0 });
    expect(onValueChange).toHaveBeenCalled();
  });

  it("commits once across cancel then lost capture", () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(
      <Slider
        defaultValue={0}
        min={0}
        max={100}
        ariaLabel="Volume"
        onValueChange={onValueChange}
        onValueCommit={onValueCommit}
      />,
    );
    const root = container.querySelector(".poodle-slider") as HTMLElement;
    const hit = container.querySelector(".poodle-slider__hit") as HTMLElement;
    mockTrack(root, 100, 32);
    hit.setPointerCapture = vi.fn();
    fireEvent.pointerDown(hit, { button: 0, clientX: 40, clientY: 16, pointerId: 1 });
    fireEvent.pointerMove(hit, { clientX: 70, clientY: 16, pointerId: 1 });
    fireEvent.pointerCancel(hit, { pointerId: 1 });
    fireEvent.lostPointerCapture(hit, { pointerId: 1 });
    expect(onValueCommit).toHaveBeenCalledOnce();
  });

  it("paints one stable row through two clipped layers and never a fallback", () => {
    const { container } = render(<Slider value={50} visibleLabel="Blur" ariaLabel="Gain" />);
    const selected = container.querySelector<HTMLElement>(".poodle-slider__inline--selected")!;
    const remainder = container.querySelector<HTMLElement>(".poodle-slider__inline--remainder")!;
    expect(selected).not.toBeNull();
    expect(remainder).not.toBeNull();
    for (const layer of [selected, remainder]) {
      expect(layer.querySelector(".poodle-slider__inline-label")).not.toBeNull();
      expect(layer.querySelector(".poodle-slider__inline-value")!.textContent).toBe("50");
    }
    expect(container.querySelector(".poodle-slider__fill")!.textContent).toBe("");
    expect(container.querySelector(".poodle-slider__remainder")!.textContent).toBe("");
    expect(container.querySelector(".poodle-slider__fallback")).toBeNull();
  });

  it("suppresses the optional label before the exact value at zero-width spans", () => {
    const { container } = render(<Slider value={67} visibleLabel="Blur" ariaLabel="Gain" />);
    const label = container.querySelector(".poodle-slider__inline--selected .poodle-slider__inline-label")!;
    expect(label.textContent).toBe("");
    const value = container.querySelector(".poodle-slider__inline--selected .poodle-slider__inline-value")!;
    expect(value.textContent).toBe("67");
    expect(container.querySelector(".poodle-slider__fallback")).toBeNull();
  });

  it("keeps the block capsule rounded-square and the thumb circular in CSS", () => {
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"] .poodle-slider__capsule {\n    position: relative;\n    display: block;\n    width: 100%;\n    min-height: var(--poodle-slider-block-height);\n    border-radius: var(--poodle-radius-control);",
    );
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"] .poodle-slider__thumb {\n    width: var(--poodle-slider-block-thumb);\n    height: var(--poodle-slider-block-thumb);\n    border-radius: 999px;",
    );
  });

  it("clips selected text to the center-anchored fill and mirrors it in RTL", () => {
    expect(css).toContain(
      "clip-path: inset(0 calc(100% - var(--poodle-slider-fill-start) - var(--poodle-slider-fill-span)) 0 var(--poodle-slider-fill-start));",
    );
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"] .poodle-slider__inline--remainder {\n    color: var(--poodle-recipe-slider-block-remainder-text, var(--poodle-color-text-primary));",
    );
    expect(css).toContain(
      "clip-path: inset(0 var(--poodle-slider-fill-start) 0 calc(100% - var(--poodle-slider-fill-start) - var(--poodle-slider-fill-span)));",
    );
  });

  it("renders the same layers in RTL", () => {
    const { container } = render(
      <Slider direction="rtl" value={20} visibleLabel="Opacity" ariaLabel="Opacity" />,
    );
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("data-direction")).toBe("rtl");
    expect(container.querySelector(".poodle-slider__inline--selected")).not.toBeNull();
    expect(container.querySelector(".poodle-slider__inline--remainder")).not.toBeNull();
  });

  it("maps selected fill to Highlight and remainder to Canvas", () => {
    expect(css).toContain(".poodle-slider[data-variant=\"block\"] .poodle-slider__capsule {\n      background: Canvas;");
    expect(css).toContain(".poodle-slider[data-variant=\"block\"] .poodle-slider__fill {\n      background: Highlight;");
    expect(css).toContain(".poodle-slider[data-variant=\"block\"] .poodle-slider__inline--selected {\n      color: HighlightText;");
    expect(css).toContain(".poodle-slider[data-variant=\"block\"] .poodle-slider__inline--remainder {\n      color: CanvasText;");
    expect(css).not.toMatch(/\.poodle-slider__fill \{\s*background: Canvas/);
  });

  it("keeps vertical block upright: value top, label centered, clip along the block axis", () => {
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"] .poodle-slider__inline-row--vertical {\n    position: relative;\n    flex-direction: column;\n    align-items: center;\n    justify-content: space-between;\n    width: 100%;\n    height: 100%;\n    box-sizing: border-box;\n    /* g18.024: a fixed, value-independent block inset keeps the anchored\n       text fully inside the rail at every shared size. */\n    padding-block: 0.25rem;\n  }",
    );
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"] .poodle-slider__inline-row--vertical .poodle-slider__inline-label {\n    position: absolute;\n    left: 0;\n    right: 0;\n    top: 50%;\n    transform: translateY(-50%);\n    display: flex;\n    align-items: center;\n    justify-content: center;\n  }",
    );
    expect(css).toContain(
      "clip-path: inset(calc(100% - var(--poodle-slider-fill-start) - var(--poodle-slider-fill-span)) 0 var(--poodle-slider-fill-start) 0);",
    );
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"][data-orientation=\"vertical\"] .poodle-slider__hit {\n    inset-inline-start: auto;\n    left: 50%;\n    top: auto;\n    bottom: calc(var(--poodle-slider-percent) - (var(--poodle-slider-block-hit) / 2));",
    );
    // g18.024: the vertical rail is the shared capsule size, not the hit
    // envelope.
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"][data-orientation=\"vertical\"] {\n    width: var(--poodle-slider-block-height);\n    min-width: var(--poodle-slider-block-height);",
    );
    // Vertical never mirrors with direction.
    expect(css).not.toContain("[data-orientation=\"vertical\"][data-direction=\"rtl\"]");
  });

  it("renders fractional default values as short step-aware decimals", () => {
    // step 0.01 implies two decimals; the snapped 0.85 must never leak a
    // binary tail. The custom formatter remains the authoritative override.
    const { container } = render(
      <Slider value={0.8500000000000001} min={0} max={1} step={0.01} ariaLabel="Drive" />,
    );
    const value = container.querySelector(".poodle-slider__inline--selected .poodle-slider__inline-value")!;
    expect(value.textContent).toBe("0.85");
  });

  it("renders the vertical row with the value slot above the label", () => {
    const { container } = render(
      <Slider orientation="vertical" value={40} visibleLabel="Blur" ariaLabel="Blur" />,
    );
    const row = container.querySelector(".poodle-slider__inline-row--vertical")!;
    const children = Array.from(row.children);
    expect(children[0].className).toContain("poodle-slider__inline-value");
    expect(children[1].className).toContain("poodle-slider__inline-label");
    expect(container.querySelector(".poodle-slider")!.getAttribute("data-orientation")).toBe(
      "vertical",
    );
  });

  it("keeps the row glyph slots value-independent across the whole range", () => {
    const journeys: Array<{ label: string; value: string }> = [];
    for (const value of [0, 50, 100]) {
      const { container, unmount } = render(
        <Slider value={value} visibleLabel="Blur" ariaLabel="Blur" />,
      );
      const label = container.querySelector(
        ".poodle-slider__inline--selected .poodle-slider__inline-label",
      )!;
      const valueSlot = container.querySelector(
        ".poodle-slider__inline--selected .poodle-slider__inline-value",
      )!;
      journeys.push({ label: label.textContent!, value: valueSlot.textContent! });
      unmount();
    }
    // happy-dom lays out at zero width, so the optional label is suppressed
    // at every value — but the exact value keeps its slot unchanged.
    expect(journeys[0]).toEqual({ label: "", value: "0" });
    expect(journeys[1]).toEqual({ label: "", value: "50" });
    expect(journeys[2]).toEqual({ label: "", value: "100" });
  });
});
