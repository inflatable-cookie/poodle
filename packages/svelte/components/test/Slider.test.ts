import { fireEvent, render } from "@testing-library/svelte";
import { readFileSync } from "node:fs";
import { describe, expect, it, vi } from "vitest";

import Slider from "../src/Slider.svelte";

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

const css = readFileSync(
  new URL("../../../core/src/styles/slider.css", `file://${import.meta.dirname}/`),
  "utf8",
);

describe("Slider (svelte)", () => {
  it("drives the fill percentage custom property from the value", () => {
    const { container } = render(Slider, { props: { value: 65, ariaLabel: "Volume" } });
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("style")).toContain("--poodle-slider-percent: 65%");
  });

  it("disables the control and drops the tab stop", () => {
    const { container } = render(Slider, { props: { value: 40, disabled: true, ariaLabel: "Volume" } });
    expect(container.querySelector(".poodle-slider")!.getAttribute("data-disabled")).toBe("true");
    expect(container.querySelector(".poodle-slider")!.getAttribute("tabindex")).toBeNull();
  });

  it("omitting variant renders the block capsule as the default", () => {
    const { container } = render(Slider, { props: { value: 50, ariaLabel: "Volume" } });
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("data-variant")).toBe("block");
    expect(container.querySelector(".poodle-slider__capsule")).not.toBeNull();
    expect(container.querySelector(".poodle-slider__control")).toBeNull();
  });

  it("renders embedded as the dense track alternative", () => {
    const { container } = render(Slider, {
      props: { variant: "embedded", value: 50, ariaLabel: "Volume" },
    });
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("data-variant")).toBe("embedded");
    expect(container.querySelector(".poodle-slider__capsule")).toBeNull();
  });
});

describe("Slider (svelte) embedded semantics", () => {
  it("normalizes a horizontal pointer along the track and commits once", async () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(Slider, {
      props: {
        variant: "embedded",
        value: 0,
        min: 0,
        max: 100,
        step: 10,
        ariaLabel: "Gain",
        onValueChange,
        onValueCommit,
      },
    });
    const root = container.querySelector<HTMLElement>(".poodle-slider")!;
    mockTrack(root, 100, 20);

    await fireEvent.pointerDown(root, { button: 0, clientX: 44, clientY: 10, pointerId: 1 });
    expect(onValueChange).toHaveBeenLastCalledWith(40);
    await fireEvent.pointerMove(root, { clientX: 76, clientY: 10, pointerId: 1 });
    expect(onValueChange).toHaveBeenLastCalledWith(80);
    await fireEvent.pointerUp(root, { pointerId: 1 });
    expect(onValueCommit).toHaveBeenCalledOnce();
    expect(onValueCommit).toHaveBeenCalledWith(80);
  });

  it("normalizes a vertical pointer from the bottom", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Slider, {
      props: {
        variant: "embedded",
        orientation: "vertical",
        value: 0,
        min: 0,
        max: 100,
        step: 10,
        ariaLabel: "Gain",
        onValueChange,
      },
    });
    const root = container.querySelector<HTMLElement>(".poodle-slider")!;
    mockTrack(root, 20, 100);

    await fireEvent.pointerDown(root, { button: 0, clientX: 10, clientY: 80, pointerId: 1 });
    expect(onValueChange).toHaveBeenLastCalledWith(20);
    await fireEvent.pointerMove(root, { clientX: 10, clientY: 0, pointerId: 1 });
    expect(onValueChange).toHaveBeenLastCalledWith(100);
  });

  it("emits change then commit for arrows, Home, and End", async () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(Slider, {
      props: {
        variant: "embedded",
        value: 50,
        min: 0,
        max: 100,
        step: 10,
        ariaLabel: "Gain",
        onValueChange,
        onValueCommit,
      },
    });
    const root = container.querySelector<HTMLElement>(".poodle-slider")!;

    await fireEvent.keyDown(root, { key: "ArrowRight" });
    expect(onValueChange).toHaveBeenLastCalledWith(60);
    expect(onValueCommit).toHaveBeenLastCalledWith(60);
    await fireEvent.keyDown(root, { key: "ArrowUp" });
    expect(onValueChange).toHaveBeenLastCalledWith(70);
    await fireEvent.keyDown(root, { key: "ArrowLeft" });
    expect(onValueChange).toHaveBeenLastCalledWith(60);
    await fireEvent.keyDown(root, { key: "ArrowDown" });
    expect(onValueChange).toHaveBeenLastCalledWith(50);
    await fireEvent.keyDown(root, { key: "Home" });
    expect(onValueChange).toHaveBeenLastCalledWith(0);
    expect(onValueCommit).toHaveBeenLastCalledWith(0);
    await fireEvent.keyDown(root, { key: "End" });
    expect(onValueChange).toHaveBeenLastCalledWith(100);
    expect(onValueCommit).toHaveBeenLastCalledWith(100);
    expect(onValueChange).toHaveBeenCalledTimes(6);
    expect(onValueCommit).toHaveBeenCalledTimes(6);
  });

  it("exposes slider ARIA fields on the embedded control", () => {
    const { container } = render(Slider, {
      props: {
        variant: "embedded",
        value: 40,
        min: 0,
        max: 100,
        orientation: "vertical",
        ariaLabel: "Gain",
        valueText: "quiet",
      },
    });
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

  it("ignores pointer and keyboard while disabled", async () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(Slider, {
      props: {
        variant: "embedded",
        value: 40,
        disabled: true,
        ariaLabel: "Gain",
        onValueChange,
        onValueCommit,
      },
    });
    const root = container.querySelector<HTMLElement>(".poodle-slider")!;
    mockTrack(root, 100, 20);
    await fireEvent.pointerDown(root, { button: 0, clientX: 80, clientY: 10, pointerId: 1 });
    await fireEvent.keyDown(root, { key: "ArrowRight" });
    expect(onValueChange).not.toHaveBeenCalled();
    expect(onValueCommit).not.toHaveBeenCalled();
    expect(root.getAttribute("tabindex")).toBeNull();
  });
});

describe("Slider (svelte) block variant", () => {
  it("does not paint ariaLabel as visible text", () => {
    const { container } = render(Slider, {
      props: { value: 50, ariaLabel: "Gain" },
    });
    expect(container.textContent).not.toContain("Gain");
    expect(container.querySelector(".poodle-slider")!.getAttribute("aria-label")).toBe("Gain");
  });

  it("does not use visibleLabel as the accessible name", () => {
    const { container } = render(Slider, {
      props: { value: 50, visibleLabel: "Blur" },
    });
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("aria-label")).toBeNull();
    expect(container.querySelector(".poodle-slider__capsule")!.getAttribute("aria-hidden")).toBe(
      "true",
    );
  });

  it("keeps a 44px hit target and forced-color roles in CSS", () => {
    const { container } = render(Slider, { props: { value: 50, size: "xs" } });
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("data-variant")).toBe("block");
    const hit = container.querySelector(".poodle-slider__hit") as HTMLElement;
    expect(hit).not.toBeNull();
    expect(css).toContain("--poodle-slider-block-hit: 44px");
    expect(css).toContain("pointer-events: auto");
    expect(css).toContain("min-height: max(var(--poodle-slider-block-min-height), var(--poodle-slider-block-hit))");
  });

  it("commits once across cancel then lost capture", async () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(Slider, {
      props: {
        value: 0,
        min: 0,
        max: 100,
        ariaLabel: "Volume",
        onValueChange,
        onValueCommit,
      },
    });
    const root = container.querySelector(".poodle-slider") as HTMLElement;
    mockTrack(root, 100, 32);
    await fireEvent.pointerDown(root, { button: 0, clientX: 40, clientY: 16, pointerId: 1 });
    await fireEvent.pointerMove(root, { clientX: 70, clientY: 16, pointerId: 1 });
    await fireEvent.pointerCancel(root, { pointerId: 1 });
    await fireEvent.lostPointerCapture(root, { pointerId: 1 });
    expect(onValueCommit).toHaveBeenCalledOnce();
  });

  it("paints one stable row through two clipped layers and never a fallback", () => {
    const { container } = render(Slider, {
      props: { value: 50, visibleLabel: "Blur", ariaLabel: "Gain" },
    });
    const selected = container.querySelector<HTMLElement>(".poodle-slider__inline--selected")!;
    const remainder = container.querySelector<HTMLElement>(".poodle-slider__inline--remainder")!;
    expect(selected).not.toBeNull();
    expect(remainder).not.toBeNull();
    // Both layers carry the same row shape: a label slot and a value slot.
    for (const layer of [selected, remainder]) {
      expect(layer.querySelector(".poodle-slider__inline-label")).not.toBeNull();
      expect(layer.querySelector(".poodle-slider__inline-value")!.textContent).toBe("50");
    }
    // The fills are paint-only; text lives in the clipped layers.
    expect(container.querySelector(".poodle-slider__fill")!.textContent).toBe("");
    expect(container.querySelector(".poodle-slider__remainder")!.textContent).toBe("");
    expect(container.querySelector(".poodle-slider__fallback")).toBeNull();
  });

  it("suppresses the optional label before the exact value at zero-width spans", () => {
    // happy-dom lays out at zero width, so this is the whole-track collision
    // journey: the label slot stays in place (empty) while the exact numeric
    // value keeps painting inside the capsule.
    const { container } = render(Slider, {
      props: { value: 67, visibleLabel: "Blur", ariaLabel: "Gain" },
    });
    const label = container.querySelector(".poodle-slider__inline--selected .poodle-slider__inline-label")!;
    expect(label.textContent).toBe("");
    const value = container.querySelector(".poodle-slider__inline--selected .poodle-slider__inline-value")!;
    expect(value.textContent).toBe("67");
    expect(container.querySelector(".poodle-slider__fallback")).toBeNull();
  });

  it("keeps the block capsule rounded-square and the thumb circular in CSS", () => {
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"] .poodle-slider__capsule {\n    position: relative;\n    display: block;\n    width: 100%;\n    min-height: var(--poodle-slider-block-min-height);\n    border-radius: var(--poodle-radius-control);",
    );
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"] .poodle-slider__thumb {\n    width: var(--poodle-slider-block-thumb);\n    height: var(--poodle-slider-block-thumb);\n    border-radius: 999px;",
    );
  });

  it("clips the selected layer at the fill boundary and mirrors the clip in RTL", () => {
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"] .poodle-slider__inline--selected {\n    color: var(--poodle-recipe-slider-block-selected-text, var(--poodle-color-text-inverse));\n    clip-path: inset(0 calc(100% - var(--poodle-slider-percent, 0%)) 0 0);",
    );
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"] .poodle-slider__inline--remainder {\n    color: var(--poodle-recipe-slider-block-remainder-text, var(--poodle-color-text-primary));\n    clip-path: inset(0 0 0 var(--poodle-slider-percent, 0%));",
    );
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"][data-orientation=\"horizontal\"][data-direction=\"rtl\"] .poodle-slider__inline--selected {\n    clip-path: inset(0 0 0 calc(100% - var(--poodle-slider-percent, 0%)));",
    );
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"][data-orientation=\"horizontal\"][data-direction=\"rtl\"] .poodle-slider__inline--remainder {\n    clip-path: inset(0 var(--poodle-slider-percent, 0%) 0 0);",
    );
  });

  it("renders the same layers in RTL and keeps the layers pointer-inert", () => {
    const { container } = render(Slider, {
      props: {
        direction: "rtl",
        value: 20,
        visibleLabel: "Opacity",
        ariaLabel: "Opacity",
      },
    });
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("data-direction")).toBe("rtl");
    expect(container.querySelector(".poodle-slider__inline--selected")).not.toBeNull();
    expect(container.querySelector(".poodle-slider__inline--remainder")).not.toBeNull();
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"] .poodle-slider__inline {\n    position: absolute;\n    inset-block: 0;\n    inset-inline: 0;\n    display: flex;\n    align-items: center;\n    padding-inline: 0.5rem;\n    overflow: hidden;\n    white-space: nowrap;\n    pointer-events: none;",
    );
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
      ".poodle-slider[data-variant=\"block\"] .poodle-slider__inline-row--vertical {\n    position: relative;\n    flex-direction: column;\n    align-items: center;\n    justify-content: space-between;\n    width: 100%;\n    height: 100%;\n  }",
    );
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"] .poodle-slider__inline-row--vertical .poodle-slider__inline-label {\n    position: absolute;\n    left: 0;\n    right: 0;\n    top: 50%;\n    transform: translateY(-50%);\n    display: flex;\n    align-items: center;\n    justify-content: center;\n  }",
    );
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"][data-orientation=\"vertical\"] .poodle-slider__inline--selected {\n    clip-path: inset(calc(100% - var(--poodle-slider-percent, 0%)) 0 0 0);",
    );
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"][data-orientation=\"vertical\"] .poodle-slider__inline--remainder {\n    clip-path: inset(0 0 var(--poodle-slider-percent, 0%) 0);",
    );
    expect(css).toContain(
      ".poodle-slider[data-variant=\"block\"][data-orientation=\"vertical\"] .poodle-slider__hit {\n    inset-inline-start: auto;\n    left: 50%;\n    top: auto;\n    bottom: calc(var(--poodle-slider-percent) - (var(--poodle-slider-block-hit) / 2));",
    );
    // Vertical never mirrors with direction: the RTL clip rules stay
    // horizontal-only.
    expect(css).not.toContain(
      "[data-orientation=\"vertical\"][data-direction=\"rtl\"]",
    );
  });

  it("renders the vertical row with the value slot above the label", () => {
    const { container } = render(Slider, {
      props: { orientation: "vertical", value: 40, visibleLabel: "Blur", ariaLabel: "Blur" },
    });
    const row = container.querySelector(".poodle-slider__inline-row--vertical")!;
    const children = Array.from(row.children);
    expect(children[0].className).toContain("poodle-slider__inline-value");
    expect(children[1].className).toContain("poodle-slider__inline-label");
    expect(container.querySelector(".poodle-slider")!.getAttribute("data-orientation")).toBe(
      "vertical",
    );
  });

  it("keeps the row glyph slots value-independent across the whole range", async () => {
    const journeys: Array<{ label: string; value: string }> = [];
    for (const value of [0, 50, 100]) {
      const { container, unmount } = render(Slider, {
        props: { value, visibleLabel: "Blur", ariaLabel: "Blur" },
      });
      const label = container.querySelector(
        ".poodle-slider__inline--selected .poodle-slider__inline-label",
      )!;
      const valueSlot = container.querySelector(
        ".poodle-slider__inline--selected .poodle-slider__inline-value",
      )!;
      journeys.push({ label: label.textContent!, value: valueSlot.textContent! });
      unmount();
    }
    expect(journeys).toHaveLength(3);
    // happy-dom lays out at zero width, so the optional label is suppressed
    // at every value — but the exact value keeps its slot unchanged, and no
    // value ever moves or hides it.
    expect(journeys[0]).toEqual({ label: "", value: "0" });
    expect(journeys[1]).toEqual({ label: "", value: "50" });
    expect(journeys[2]).toEqual({ label: "", value: "100" });
  });
});
