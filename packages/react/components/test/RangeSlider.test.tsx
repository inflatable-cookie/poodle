import { fireEvent, render } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { describe, expect, it, vi } from "vitest";

import { RangeSlider } from "../src/RangeSlider";

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
  new URL("../../../core/src/styles/range-slider.css", `file://${import.meta.dirname}/`),
  "utf8",
);

describe("RangeSlider (react)", () => {
  it("omitting variant renders the fixed-anchor block as the default", () => {
    const { container } = render(<RangeSlider value={[20, 80]} ariaLabel="Price range" />);
    const root = container.querySelector(".poodle-range-slider")!;
    expect(root.getAttribute("data-variant")).toBe("block");
    expect(container.querySelector(".poodle-range-slider__capsule")).not.toBeNull();
    expect(container.querySelectorAll(".poodle-range-slider__control")).toHaveLength(0);
  });

  /**
   * g14.003 retained regression: the two thumbs stay grouped under one
   * labelled `role="group"` container so a screen reader hears one range,
   * not two unrelated sliders.
   */
  it("groups the two thumbs under one container", () => {
    const { container } = render(<RangeSlider value={[20, 80]} ariaLabel="Gain range" />);

    const group = container.querySelector('[role="group"]');
    expect(group).not.toBeNull();
    expect(group!.querySelectorAll('[role="slider"]').length).toBe(2);
    const thumbs = [...group!.querySelectorAll('[role="slider"]')];
    expect(thumbs.map((thumb) => thumb.getAttribute("aria-label"))).toEqual([
      "Gain range minimum",
      "Gain range maximum",
    ]);
  });

  /**
   * g15.003 scrub semantics: press -> drag -> release emits
   * ["valueChange", "valueChange", "valueCommit"], the same trace the
   * native regression asserts.
   */
  it("emits change per drag move and a single commit carrying the final pair", () => {
    const trace: string[] = [];
    const { container } = render(
      <RangeSlider
        defaultValue={[20, 80]}
        min={0}
        max={100}
        ariaLabel="Price range"
        onValueChange={() => trace.push("valueChange")}
        onValueCommit={() => trace.push("valueCommit")}
      />,
    );
    const root = container.querySelector<HTMLElement>(".poodle-range-slider")!;
    mockTrack(root, 100, 32);

    fireEvent.pointerDown(root, { button: 0, clientX: 10, clientY: 16, pointerId: 1 });
    fireEvent.pointerMove(root, { clientX: 30, clientY: 16, pointerId: 1 });
    fireEvent.pointerMove(root, { clientX: 45, clientY: 16, pointerId: 1 });
    // Press emits its own change (grab at 10), then two live drag changes.
    expect(trace).toEqual(["valueChange", "valueChange", "valueChange"]);
    fireEvent.pointerUp(root, { pointerId: 1 });
    expect(trace).toEqual(["valueChange", "valueChange", "valueChange", "valueCommit"]);
  });

  it("keeps the lower<=upper invariant when a thumb crosses its sibling", () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(
      <RangeSlider
        defaultValue={[20, 80]}
        min={0}
        max={100}
        ariaLabel="Price range"
        onValueChange={onValueChange}
        onValueCommit={onValueCommit}
      />,
    );
    const root = container.querySelector<HTMLElement>(".poodle-range-slider")!;
    mockTrack(root, 100, 32);

    fireEvent.pointerDown(root, { button: 0, clientX: 85, clientY: 16, pointerId: 1 });
    fireEvent.pointerMove(root, { clientX: 5, clientY: 16, pointerId: 1 });
    expect(onValueChange).toHaveBeenLastCalledWith([20, 20]);

    fireEvent.pointerUp(root, { pointerId: 1 });
    expect(onValueCommit).toHaveBeenCalledWith([20, 20]);
  });

  it("reports per-thumb values and bounds", () => {
    const { container } = render(
      <RangeSlider value={[20, 80]} min={0} max={100} ariaLabel="Price range" />,
    );
    const lower = container.querySelector(".poodle-range-slider__hit--lower")!;
    const upper = container.querySelector(".poodle-range-slider__hit--upper")!;

    expect(lower.getAttribute("aria-valuenow")).toBe("20");
    expect(upper.getAttribute("aria-valuenow")).toBe("80");
    expect(lower.getAttribute("aria-valuemin")).toBe("0");
    expect(upper.getAttribute("aria-valuemax")).toBe("100");
  });

  it("disables both thumbs", () => {
    const { container } = render(
      <RangeSlider value={[30, 70]} disabled ariaLabel="Disabled range" />,
    );
    const hits = [...container.querySelectorAll<HTMLElement>(".poodle-range-slider__hit")];
    expect(hits).toHaveLength(2);
    expect(hits.every((hit) => hit.getAttribute("aria-disabled") === "true")).toBe(true);
    expect(hits.every((hit) => hit.getAttribute("tabindex") === null)).toBe(true);
    expect(container.querySelector(".poodle-range-slider")!.getAttribute("data-disabled")).toBe("true");
  });

  it("keyboard adjusts each thumb with change then commit", () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(
      <RangeSlider
        defaultValue={[20, 80]}
        min={0}
        max={100}
        step={5}
        ariaLabel="Price range"
        onValueChange={onValueChange}
        onValueCommit={onValueCommit}
      />,
    );
    const lower = container.querySelector(".poodle-range-slider__hit--lower") as HTMLElement;
    fireEvent.keyDown(lower, { key: "ArrowRight" });
    expect(onValueChange).toHaveBeenLastCalledWith([25, 80]);
    expect(onValueCommit).toHaveBeenLastCalledWith([25, 80]);
    const upper = container.querySelector(".poodle-range-slider__hit--upper") as HTMLElement;
    fireEvent.keyDown(upper, { key: "End" });
    expect(onValueChange).toHaveBeenLastCalledWith([25, 100]);
  });
});

describe("RangeSlider (react) fixed block anchors", () => {
  it("does not paint ariaLabel as visible text", () => {
    const { container } = render(<RangeSlider value={[20, 80]} ariaLabel="Gain" />);
    expect(container.textContent).not.toContain("Gain");
  });

  it("paints required endpoints at fixed anchors and never a fallback", () => {
    const { container } = render(
      <RangeSlider value={[20, 80]} visibleLabel="Price" ariaLabel="Gain" />,
    );
    const slots = Array.from(
      container.querySelector<HTMLElement>(
        ".poodle-range-slider__inline--selected .poodle-range-slider__inline-row",
      )!.children,
    ).map((child) => child.textContent);
    // happy-dom lays out at zero width: the optional label suppresses, the
    // required endpoints stay. No fallback line exists in this variant.
    expect(slots).toEqual(["20", "", "80"]);
    expect(container.querySelector(".poodle-range-slider__fallback")).toBeNull();
    expect(container.querySelector(".poodle-range-slider__block-surface")).toBeNull();
  });

  it("clips the window layer between start and end and mirrors in RTL", () => {
    expect(css).toContain(
      ".poodle-range-slider[data-variant=\"block\"] .poodle-range-slider__inline--selected {\n    color: var(--poodle-recipe-range-slider-block-selected-text, var(--poodle-color-text-inverse));\n    clip-path: inset(0 calc(100% - var(--poodle-range-end)) 0 var(--poodle-range-start));\n  }",
    );
    expect(css).toContain(
      ".poodle-range-slider[data-variant=\"block\"] .poodle-range-slider__inline--remainder-start {\n    color: var(--poodle-recipe-range-slider-block-remainder-text, var(--poodle-color-text-primary));\n    clip-path: inset(0 calc(100% - var(--poodle-range-start)) 0 0);",
    );
    expect(css).toContain(
      ".poodle-range-slider[data-variant=\"block\"] .poodle-range-slider__inline--remainder-end {\n    color: var(--poodle-recipe-range-slider-block-remainder-text, var(--poodle-color-text-primary));\n    clip-path: inset(0 0 0 var(--poodle-range-end));",
    );
    // RTL mirrors the window; lower stays at the logical start.
    expect(css).toContain(
      "[data-direction=\"rtl\"] .poodle-range-slider__inline--selected {\n    clip-path: inset(0 var(--poodle-range-start) 0 calc(100% - var(--poodle-range-end)));",
    );
  });

  it("keeps two 44px hits and chooses lower on a tie", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <RangeSlider
        defaultValue={[50, 50]}
        min={0}
        max={100}
        size="xs"
        ariaLabel="Range"
        onValueChange={onValueChange}
      />,
    );
    const root = container.querySelector(".poodle-range-slider") as HTMLElement;
    const lower = container.querySelector(".poodle-range-slider__hit--lower") as HTMLElement;
    expect(container.querySelectorAll(".poodle-range-slider__hit")).toHaveLength(2);
    expect(css).toContain("--poodle-range-slider-block-hit: 44px");
    expect(css).toContain("pointer-events: auto");
    mockTrack(root, 100, 32);
    lower.setPointerCapture = vi.fn();
    fireEvent.pointerDown(lower, { button: 0, clientX: 50, clientY: 16, pointerId: 1 });
    fireEvent.pointerMove(lower, { clientX: 20, clientY: 16, pointerId: 1 });
    expect(onValueChange).toHaveBeenLastCalledWith([20, 50]);
    const upper = container.querySelector(".poodle-range-slider__hit--upper") as HTMLElement;
    fireEvent.keyDown(upper, { key: "ArrowRight" });
    expect(onValueChange).toHaveBeenLastCalledWith([20, 51]);
  });

  it("renders the vertical block upright with upper top, label center, lower bottom", () => {
    const { container } = render(
      <RangeSlider orientation="vertical" value={[20, 80]} visibleLabel="Price" />,
    );
    const root = container.querySelector(".poodle-range-slider")!;
    expect(root.getAttribute("data-orientation")).toBe("vertical");
    const slots = Array.from(
      container.querySelector<HTMLElement>(
        ".poodle-range-slider__inline--selected .poodle-range-slider__inline-row--vertical",
      )!.children,
    ).map((child) => child.textContent);
    // Upright: upper value at the physical top, label centered, lower at the
    // physical bottom.
    expect(slots).toEqual(["80", "", "20"]);
    expect(css).toContain(
      ".poodle-range-slider[data-variant=\"block\"][data-orientation=\"vertical\"] .poodle-range-slider__inline--selected {\n    clip-path: inset(calc(100% - var(--poodle-range-end)) 0 var(--poodle-range-start) 0);",
    );
    expect(css).toContain(
      ".poodle-range-slider[data-variant=\"block\"][data-orientation=\"vertical\"] .poodle-range-slider__hit--lower {\n    inset-inline-start: auto;\n    bottom: calc(var(--poodle-range-start) - (var(--poodle-range-slider-block-hit) / 2));",
    );
    // Vertical never mirrors with direction.
    expect(css).not.toContain("[data-orientation=\"vertical\"][data-direction=\"rtl\"]");
  });

  it("keeps the endpoint anchors fixed at extrema, equality, and overlap", () => {
    for (const value of [[0, 100], [50, 50], [0, 0], [100, 100]]) {
      const { container, unmount } = render(
        <RangeSlider value={value as [number, number]} visibleLabel="Price" ariaLabel="Gain" />,
      );
      const slots = Array.from(
        container.querySelector<HTMLElement>(
          ".poodle-range-slider__inline--selected .poodle-range-slider__inline-row",
        )!.children,
      ).map((child) => child.textContent);
      expect(slots).toEqual([String(value![0]), "", String(value![1])]);
      unmount();
    }
  });
});
