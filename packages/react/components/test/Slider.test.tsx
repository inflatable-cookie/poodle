import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Slider } from "../src/Slider";

describe("Slider (react)", () => {
  it("snaps live input to step and commits the same value on release", () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(
      <Slider value={50} step={10} ariaLabel="Volume" onValueChange={onValueChange} onValueCommit={onValueCommit} />,
    );
    const input = container.querySelector<HTMLInputElement>(".poodle-slider__control")!;

    fireEvent.input(input, { target: { value: "63" } });
    expect(onValueChange).toHaveBeenCalledWith(60);

    fireEvent.change(input, { target: { value: "63" } });
    expect(onValueCommit).toHaveBeenCalledWith(60);
  });

  it("clamps out-of-range input into the min/max window", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <Slider value={50} min={0} max={100} ariaLabel="Volume" onValueChange={onValueChange} />,
    );
    const input = container.querySelector<HTMLInputElement>(".poodle-slider__control")!;

    fireEvent.input(input, { target: { value: "-5" } });
    expect(onValueChange).toHaveBeenLastCalledWith(0);

    fireEvent.input(input, { target: { value: "150" } });
    expect(onValueChange).toHaveBeenLastCalledWith(100);
  });

  it("drives the fill percentage custom property from the value", () => {
    const { container } = render(<Slider value={65} ariaLabel="Volume" />);
    const root = container.querySelector(".poodle-slider")!;
    expect(root.getAttribute("style")).toContain("--poodle-slider-percent: 65%");
  });

  it("applies the bounds guard when max is at or below min", () => {
    const { container } = render(<Slider value={5} min={10} max={10} ariaLabel="Volume" />);
    const input = container.querySelector<HTMLInputElement>(".poodle-slider__control")!;
    expect(input.getAttribute("max")).toBe("11");
  });

  it("disables the native control", () => {
    const { container } = render(<Slider value={40} disabled ariaLabel="Volume" />);
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

  it("exposes slider ARIA fields on the embedded control", () => {
    const { container } = render(
      <Slider
        variant="embedded"
        value={40}
        min={0}
        max={100}
        orientation="vertical"
        ariaLabel="Gain"
        valueText="quiet"
      />,
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
