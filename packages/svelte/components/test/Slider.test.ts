import { fireEvent, render } from "@testing-library/svelte";
import { readFileSync } from "node:fs";
import { describe, expect, it, vi } from "vitest";

import Slider from "../src/Slider.svelte";

describe("Slider (svelte)", () => {
  it("snaps live input to step and commits the same value on release", async () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(Slider, {
      props: { value: 50, step: 10, ariaLabel: "Volume", onValueChange, onValueCommit },
    });
    const input = container.querySelector<HTMLInputElement>(".poodle-slider__control")!;

    await fireEvent.input(input, { target: { value: "63" } });
    expect(onValueChange).toHaveBeenCalledWith(60);

    await fireEvent.change(input, { target: { value: "63" } });
    expect(onValueCommit).toHaveBeenCalledWith(60);
  });

  it("clamps out-of-range input into the min/max window", async () => {
    const onValueChange = vi.fn();
    const { container } = render(Slider, {
      props: { value: 50, min: 0, max: 100, ariaLabel: "Volume", onValueChange },
    });
    const input = container.querySelector<HTMLInputElement>(".poodle-slider__control")!;

    await fireEvent.input(input, { target: { value: "-5" } });
    expect(onValueChange).toHaveBeenLastCalledWith(0);

    await fireEvent.input(input, { target: { value: "150" } });
    expect(onValueChange).toHaveBeenLastCalledWith(100);
  });

  it("drives the fill percentage custom property from the value", () => {
    const { container } = render(Slider, { props: { value: 65, ariaLabel: "Volume" } });
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("style")).toContain("--poodle-slider-percent: 65%");
  });

  it("applies the bounds guard when max is at or below min", () => {
    const { container } = render(Slider, { props: { value: 5, min: 10, max: 10, ariaLabel: "Volume" } });
    const input = container.querySelector<HTMLInputElement>(".poodle-slider__control")!;
    expect(input.getAttribute("max")).toBe("11");
  });

  it("disables the native control", () => {
    const { container } = render(Slider, { props: { value: 40, disabled: true, ariaLabel: "Volume" } });
    const input = container.querySelector<HTMLInputElement>(".poodle-slider__control")!;
    expect(input.disabled).toBe(true);
    expect(container.querySelector(".poodle-slider")!.getAttribute("data-disabled")).toBe("true");
  });
});

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

describe("Slider (svelte) block appearance", () => {
  it("omitting appearance keeps the track anatomy", () => {
    const { container } = render(Slider, { props: { value: 50, ariaLabel: "Volume" } });
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("data-appearance")).toBeNull();
    expect(container.querySelector(".poodle-slider__control")).not.toBeNull();
    expect(container.querySelector(".poodle-slider__capsule")).toBeNull();
  });

  it("does not paint ariaLabel as visible text", () => {
    const { container } = render(Slider, {
      props: { appearance: "block", value: 50, ariaLabel: "Gain" },
    });
    expect(container.textContent).not.toContain("Gain");
    expect(container.querySelector(".poodle-slider")!.getAttribute("aria-label")).toBe("Gain");
  });

  it("rejects vertical block before paint", () => {
    expect(() =>
      render(Slider, { props: { appearance: "block", orientation: "vertical", value: 40 } }),
    ).toThrow('Slider appearance="block" rejects orientation="vertical"');
  });

  it("keeps a 44px hit target and forced-color roles in CSS", () => {
    const { container } = render(Slider, { props: { appearance: "block", value: 50, size: "xs" } });
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("data-appearance")).toBe("block");
    const hit = container.querySelector(".poodle-slider__hit") as HTMLElement;
    expect(hit).not.toBeNull();
    const css = readFileSync(
      new URL("../../../core/src/styles/slider.css", `file://${import.meta.dirname}/`),
      "utf8",
    );
    expect(css).toContain("--poodle-slider-block-hit: 44px");
  });

  it("commits once across cancel then lost capture", async () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(Slider, {
      props: {
        appearance: "block",
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

  it("maps selected fill to Highlight and remainder to Canvas", () => {
    const css = readFileSync(
      new URL("../../../core/src/styles/slider.css", `file://${import.meta.dirname}/`),
      "utf8",
    );
    expect(css).toContain(".poodle-slider[data-appearance=\"block\"] .poodle-slider__capsule {\n      background: Canvas;");
    expect(css).toContain(".poodle-slider[data-appearance=\"block\"] .poodle-slider__fill {\n      background: Highlight;");
    expect(css).not.toMatch(/\.poodle-slider__fill \{\s*background: Canvas/);
  });
});
