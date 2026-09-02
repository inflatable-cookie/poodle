import { fireEvent, render } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { describe, expect, it, vi } from "vitest";

import { RangeSlider } from "../src/RangeSlider";

describe("RangeSlider (react)", () => {
  it("commits a standard thumb only after its value changed", () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const view = render(
      <RangeSlider
        value={[20, 80]}
        ariaLabel="Price range"
        onValueChange={onValueChange}
        onValueCommit={onValueCommit}
      />,
    );
    const lower = view.getByRole("slider", { name: "Price range minimum" });

    fireEvent.keyUp(lower, { key: "Tab" });
    fireEvent.mouseUp(lower);
    expect(onValueCommit).not.toHaveBeenCalled();

    fireEvent.input(lower, { target: { value: "25" } });
    fireEvent.keyUp(lower, { key: "ArrowRight" });
    expect(onValueChange).toHaveBeenCalledWith([25, 80]);
    expect(onValueCommit).toHaveBeenCalledOnce();
    expect(onValueCommit).toHaveBeenCalledWith([25, 80]);

    fireEvent.keyUp(lower, { key: "ArrowRight" });
    expect(onValueCommit).toHaveBeenCalledOnce();
  });
});

/**
 * g14.003 retained regression: the React shell shipped without the container
 * `role="group"` the Svelte shell had, so a screen reader heard two unrelated
 * sliders instead of one labelled range.
 */
describe("RangeSlider (react) semantics", () => {
  it("groups the two thumbs under one container", () => {
    const { container } = render(<RangeSlider value={[20, 80]} ariaLabel="Gain range" />);

    const group = container.querySelector('[role="group"]');
    expect(group).not.toBeNull();

    // The label lives on the thumbs, which is where the value is; the group is
    // what stops them being heard as two unrelated sliders. Standard thumbs are
    // real range inputs, so they carry the implicit slider role.
    const thumbs = [...group!.querySelectorAll('input[type="range"]')];
    expect(thumbs.map((thumb) => thumb.getAttribute("aria-label"))).toEqual([
      "Gain range minimum",
      "Gain range maximum",
    ]);
  });

  it("groups the embedded thumbs the same way", () => {
    const { container } = render(
      <RangeSlider value={[20, 80]} variant="embedded" ariaLabel="Gain range" />,
    );

    const group = container.querySelector('[role="group"]');
    expect(group).not.toBeNull();
    expect(group!.querySelectorAll('[role="slider"]').length).toBe(2);
  });
});

/**
 * g15.003 scrub semantics: the retained native regression
 * `a_scrub_reports_change_while_dragging_and_commits_once_at_release` asserts
 * the trace ["valueChange", "valueChange", "valueCommit"] for press -> drag ->
 * release. The web evidence asserts the same change/commit split through the
 * native-input path: live INPUT events emit change, the single release commit
 * emits one commit carrying the final pair.
 */
describe("RangeSlider (react) scrub semantics", () => {
  it("emits change per drag input and a single commit carrying the final pair", () => {
    const trace: string[] = [];
    const { container } = render(
      <RangeSlider
        value={[20, 80]}
        ariaLabel="Price range"
        onValueChange={() => trace.push("valueChange")}
        onValueCommit={() => trace.push("valueCommit")}
      />,
    );
    const lower = container.querySelector<HTMLInputElement>(".poodle-range-slider__control--lower")!;

    fireEvent.input(lower, { target: { value: "30" } });
    fireEvent.input(lower, { target: { value: "45" } });
    expect(trace).toEqual(["valueChange", "valueChange"]);

    fireEvent.keyUp(lower, { key: "ArrowRight" });
    expect(trace).toEqual(["valueChange", "valueChange", "valueCommit"]);
  });

  it("keeps the lower<=upper invariant when a thumb crosses its sibling", () => {
    const onValueChange = vi.fn();
    const onValueCommit = vi.fn();
    const { container } = render(
      <RangeSlider value={[20, 80]} ariaLabel="Price range" onValueChange={onValueChange} onValueCommit={onValueCommit} />,
    );
    const upper = container.querySelector<HTMLInputElement>(".poodle-range-slider__control--upper")!;

    fireEvent.input(upper, { target: { value: "10" } });
    expect(onValueChange).toHaveBeenCalledWith([20, 20]);

    fireEvent.keyUp(upper, { key: "ArrowRight" });
    expect(onValueCommit).toHaveBeenCalledWith([20, 20]);
  });

  it("reports per-thumb values and bounds", () => {
    const { container } = render(
      <RangeSlider value={[20, 80]} min={0} max={100} ariaLabel="Price range" />,
    );
    const lower = container.querySelector<HTMLInputElement>(".poodle-range-slider__control--lower")!;
    const upper = container.querySelector<HTMLInputElement>(".poodle-range-slider__control--upper")!;

    expect(lower.value).toBe("20");
    expect(upper.value).toBe("80");
    expect(lower.getAttribute("min")).toBe("0");
    expect(upper.getAttribute("max")).toBe("100");
  });

  it("applies the bounds guard when max is at or below min", () => {
    const { container } = render(
      <RangeSlider value={[5, 8]} min={10} max={10} ariaLabel="Price range" />,
    );
    const inputs = [...container.querySelectorAll<HTMLInputElement>(".poodle-range-slider__control")];
    expect(inputs.every((input) => input.getAttribute("max") === "11")).toBe(true);
  });

  it("disables both thumbs", () => {
    const { container } = render(
      <RangeSlider value={[30, 70]} disabled ariaLabel="Disabled range" />,
    );
    const inputs = [...container.querySelectorAll<HTMLInputElement>(".poodle-range-slider__control")];
    expect(inputs.every((input) => input.disabled)).toBe(true);
    expect(container.querySelector(".poodle-range-slider")!.getAttribute("data-disabled")).toBe("true");
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

describe("RangeSlider (react) block appearance", () => {
  it("omitting appearance keeps native range inputs", () => {
    const { container } = render(<RangeSlider value={[20, 80]} ariaLabel="Price range" />);
    expect(container.querySelector(".poodle-range-slider")!.getAttribute("data-appearance")).toBeNull();
    expect(container.querySelectorAll(".poodle-range-slider__control")).toHaveLength(2);
    expect(container.querySelector(".poodle-range-slider__capsule")).toBeNull();
  });

  it("does not paint ariaLabel as visible text", () => {
    const { container } = render(
      <RangeSlider appearance="block" value={[20, 80]} ariaLabel="Gain" />,
    );
    expect(container.textContent).not.toContain("Gain");
  });

  it("rejects vertical block before paint", () => {
    expect(() =>
      render(<RangeSlider appearance="block" orientation="vertical" value={[20, 80]} />),
    ).toThrow('RangeSlider appearance="block" rejects orientation="vertical"');
  });

  it("keeps two 44px hits and chooses lower on a tie", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <RangeSlider
        appearance="block"
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
    const css = readFileSync(
      new URL("../../../core/src/styles/range-slider.css", `file://${import.meta.dirname}/`),
      "utf8",
    );
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
});
