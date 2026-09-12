import { describe, expect, test } from "bun:test";

import { switchTransition } from "../src/switch.ts";
import { singleSelectTransition, type SingleSelectContext } from "../src/single-select.ts";
import { toggleGroupTransition, type ToggleGroupContext } from "../src/toggle-group.ts";
import {
  createRangeSliderControlContext,
  createSliderControlContext,
  normalizeRangeValue,
  normalizeSliderValue,
  rangeSliderControlTransition,
  rangeSliderVisualState,
  rangeSliderTransition,
  safeSliderMax,
  sliderControlTransition,
  sliderVisualState,
  sliderTransition,
  snapToStep,
  blockInlineFits,
  blockItemFits,
  blockRegionAvailable,
  physicalToValueNorm,
  defaultVisibleValueText,
  resolveRangeVisibleValue,
  resolveSliderVisibleValue,
  layoutRangeSliderBlock,
  layoutSliderBlock,
  type RangeSliderContext,
  type SliderContext,
} from "../src/slider.ts";

describe("switchTransition", () => {
  const ctx = { checked: false, disabled: false, readOnly: false };

  test("toggle emits callback; disabled inert; readOnly reverts", () => {
    const on = switchTransition(ctx, { type: "TOGGLE", nextChecked: true });
    expect(on.context.checked).toBe(true);
    expect(on.effects).toEqual([{ type: "emitCheckedChange", checked: true }]);

    expect(switchTransition({ ...ctx, disabled: true }, { type: "TOGGLE", nextChecked: true }).effects).toEqual([]);
    expect(switchTransition({ ...ctx, readOnly: true }, { type: "TOGGLE", nextChecked: true }).effects).toEqual([
      { type: "revertNativeChecked" },
    ]);
  });
});

describe("singleSelectTransition", () => {
  const ctx: SingleSelectContext = {
    value: "a",
    options: [{ value: "a" }, { value: "b", disabled: true }, { value: "c" }],
    disabled: false,
  };

  test("selects enabled option and emits", () => {
    const result = singleSelectTransition(ctx, { type: "SELECT", value: "c" });
    expect(result.context.value).toBe("c");
    expect(result.effects).toEqual([{ type: "emitValueChange", value: "c" }]);
  });

  test("disabled option, disabled group, unknown value, and same value are inert", () => {
    expect(singleSelectTransition(ctx, { type: "SELECT", value: "b" }).effects).toEqual([]);
    expect(singleSelectTransition({ ...ctx, disabled: true }, { type: "SELECT", value: "c" }).effects).toEqual([]);
    expect(singleSelectTransition(ctx, { type: "SELECT", value: "zz" }).effects).toEqual([]);
    expect(singleSelectTransition(ctx, { type: "SELECT", value: "a" }).effects).toEqual([]);
  });
});

describe("toggleGroupTransition", () => {
  const base: ToggleGroupContext = {
    value: null,
    options: [{ value: "x" }, { value: "y" }, { value: "z", disabled: true }],
    selectionMode: "single",
    allowDeactivation: false,
    disabled: false,
  };

  test("single mode selects; reselect stays without allowDeactivation", () => {
    const selected = toggleGroupTransition(base, { type: "TOGGLE", value: "x" });
    expect(selected.context.value).toBe("x");

    const reselected = toggleGroupTransition(selected.context, { type: "TOGGLE", value: "x" });
    expect(reselected.context.value).toBe("x");
    expect(reselected.effects).toEqual([{ type: "emitValueChange", value: "x" }]);
  });

  test("allowDeactivation clears on reselect", () => {
    const ctx = { ...base, value: "x", allowDeactivation: true };
    const result = toggleGroupTransition(ctx, { type: "TOGGLE", value: "x" });
    expect(result.context.value).toBeNull();
    expect(result.effects).toEqual([{ type: "emitValueChange", value: null }]);
  });

  test("multiple mode toggles membership", () => {
    const ctx: ToggleGroupContext = { ...base, selectionMode: "multiple", value: ["x"] };
    const added = toggleGroupTransition(ctx, { type: "TOGGLE", value: "y" });
    expect(added.context.value).toEqual(["x", "y"]);

    const removed = toggleGroupTransition(added.context, { type: "TOGGLE", value: "x" });
    expect(removed.context.value).toEqual(["y"]);
  });

  test("disabled option and disabled group are inert", () => {
    expect(toggleGroupTransition(base, { type: "TOGGLE", value: "z" }).effects).toEqual([]);
    expect(toggleGroupTransition({ ...base, disabled: true }, { type: "TOGGLE", value: "x" }).effects).toEqual([]);
  });
});

describe("slider", () => {
  const ctx: SliderContext = { value: 50, min: 0, max: 100, step: 10, disabled: false };

  test("INPUT snaps and clamps, emits change; COMMIT emits commit", () => {
    const input = sliderTransition(ctx, { type: "INPUT", raw: 44 });
    expect(input.context.value).toBe(40);
    expect(input.effects).toEqual([{ type: "emitValueChange", value: 40 }]);

    const commit = sliderTransition(ctx, { type: "COMMIT", raw: 999 });
    expect(commit.context.value).toBe(100);
    expect(commit.effects).toEqual([{ type: "emitValueCommit", value: 100 }]);
  });

  test("degenerate range widens; invalid step passes through", () => {
    expect(safeSliderMax(10, 10)).toBe(11);
    expect(safeSliderMax(10, 5)).toBe(11);
    expect(snapToStep(7, 0, 0)).toBe(7);
  });

  test("keeps both bounds reachable when step does not divide the range", () => {
    const uneven: SliderContext = { value: 0, min: 0, max: 1, step: 0.3, disabled: false };
    expect(normalizeSliderValue(uneven, 0)).toBe(0);
    expect(normalizeSliderValue(uneven, 1)).toBe(1);
    expect(normalizeSliderValue(uneven, 2)).toBe(1);
    expect(normalizeSliderValue(uneven, 0.89)).toBeCloseTo(0.9);
    expect(sliderTransition(uneven, { type: "INPUT", raw: 1 }).context.value).toBe(1);
  });

  test("negative half ties snap toward positive infinity from min", () => {
    // Portable tie law shared with poodle-headless: Math.round half-up,
    // anchored at (raw - min) / step — never raw / step and never f64::round.
    expect(snapToStep(-0.5, 0, 1)).toBe(0);
    expect(snapToStep(-1, 0, 2)).toBe(0);
    expect(snapToStep(-1.5, 0, 1)).toBe(-1);
    expect(snapToStep(5, 10, 10)).toBe(10);
    expect(snapToStep(15, 10, 10)).toBe(20);
    expect(snapToStep(0.5, 0, 1)).toBe(1);
  });

  test("embedded pointer gestures map laws and publish unipolar/bipolar fill geometry", () => {
    let control = createSliderControlContext({ value: 0, min: -1, max: 1, step: 0, polarity: "bipolar" });
    let result = sliderControlTransition(control, { type: "POINTER_BEGIN", valueNorm: 0.25 });
    expect(result.context.value).toBe(-0.5);
    expect(result.effects).toEqual([{ type: "emitValueChange", value: -0.5 }]);
    control = result.context;
    expect(sliderVisualState(control)).toMatchObject({ centerNorm: 0.5, fillStartNorm: 0.25, fillSpanNorm: 0.25, fillTone: "negative", pointerActive: true });
    result = sliderControlTransition(control, { type: "POINTER_END" });
    expect(result.effects).toEqual([{ type: "emitValueCommit", value: -0.5 }]);

    const negative = sliderVisualState(createSliderControlContext({ value: -0.25, min: -1, max: 0, step: 0 }));
    expect(negative).toMatchObject({ centerNorm: 1, fillStartNorm: 0.75, fillSpanNorm: 0.25 });
  });

  test("press, move, and end emit live change then one commit", () => {
    let control = createSliderControlContext({ value: 0, min: 0, max: 100, step: 10 });
    let result = sliderControlTransition(control, { type: "POINTER_BEGIN", valueNorm: 0.44 });
    expect(result.effects).toEqual([{ type: "emitValueChange", value: 40 }]);
    control = result.context;
    result = sliderControlTransition(control, { type: "POINTER_MOVE", valueNorm: 0.76 });
    expect(result.effects).toEqual([{ type: "emitValueChange", value: 80 }]);
    control = result.context;
    result = sliderControlTransition(control, { type: "POINTER_END" });
    expect(result.effects).toEqual([{ type: "emitValueCommit", value: 80 }]);
    expect(result.context.pointerActive).toBe(false);
  });

  test("SET_VALUE rebuilds without emitting", () => {
    const set = sliderTransition(ctx, { type: "SET_VALUE", value: 70 });
    expect(set.context.value).toBe(70);
    expect(set.effects).toEqual([]);
    const control = sliderControlTransition(createSliderControlContext({ value: 0, step: 10 }), { type: "SET_VALUE", value: 74 });
    expect(control.context.value).toBe(70);
    expect(control.effects).toEqual([]);
  });

  test("disabled pointer is inert", () => {
    const control = createSliderControlContext({ disabled: true, value: 50 });
    expect(sliderControlTransition(control, { type: "POINTER_BEGIN", valueNorm: 0.9 }).effects).toEqual([]);
    expect(sliderControlTransition(control, { type: "POINTER_MOVE", valueNorm: 0.9 }).effects).toEqual([]);
    expect(sliderControlTransition(control, { type: "POINTER_END" }).effects).toEqual([]);
  });
});

describe("rangeSlider", () => {
  const ctx: RangeSliderContext = { value: [20, 80], min: 0, max: 100, step: 5, disabled: false };

  test("thumbs cannot cross", () => {
    const lower = rangeSliderTransition(ctx, { type: "INPUT", thumb: "lower", raw: 95 });
    expect(lower.context.value).toEqual([80, 80]);

    const upper = rangeSliderTransition(ctx, { type: "INPUT", thumb: "upper", raw: 3 });
    expect(upper.context.value).toEqual([20, 20]);
  });

  test("normalizeRangeValue orders and clamps a reversed pair", () => {
    expect(normalizeRangeValue({ ...ctx, value: [90, 10] })).toEqual([10, 90]);
    expect(normalizeRangeValue({ ...ctx, value: [-5, 200] })).toEqual([0, 100]);
  });

  test("commit emits commit effect with normalized pair", () => {
    const result = rangeSliderTransition(ctx, { type: "COMMIT", thumb: "upper", raw: 62 });
    expect(result.context.value).toEqual([20, 60]);
    expect(result.effects).toEqual([{ type: "emitValueCommit", value: [20, 60] }]);
  });

  test("keeps range bounds reachable when step does not divide the range", () => {
    const uneven: RangeSliderContext = { value: [0.3, 0.6], min: 0, max: 1, step: 0.3, disabled: false };
    expect(rangeSliderTransition(uneven, { type: "INPUT", thumb: "lower", raw: 0 }).context.value).toEqual([0, 0.6]);
    expect(rangeSliderTransition(uneven, { type: "INPUT", thumb: "upper", raw: 1 }).context.value).toEqual([0.3, 1]);
  });

  test("embedded pointer keeps the selected thumb and exposes the bipolar center", () => {
    let control = createRangeSliderControlContext({ value: [-0.5, 0.5], min: -1, max: 1, step: 0, polarity: "bipolar" });
    expect(rangeSliderVisualState(control)).toMatchObject({
      negativeFillStartNorm: 0.25,
      negativeFillSpanNorm: 0.25,
      positiveFillStartNorm: 0.5,
      positiveFillSpanNorm: 0.25,
      fillSplitAtCenter: true,
    });
    expect(rangeSliderVisualState(createRangeSliderControlContext({
      value: [-0.5, 0], min: -1, max: 1, step: 0, polarity: "bipolar",
    })).fillSplitAtCenter).toBe(false);
    let result = rangeSliderControlTransition(control, { type: "POINTER_BEGIN", valueNorm: 0.1 });
    expect(result.context.activeThumb).toBe("lower");
    control = result.context;
    result = rangeSliderControlTransition(control, { type: "POINTER_MOVE", valueNorm: 0.9 });
    expect(result.context.value).toEqual([0.5, 0.5]);
    expect(result.context.activeThumb).toBe("lower");
    control = result.context;
    expect(rangeSliderVisualState(control)).toMatchObject({ lowerNorm: 0.75, upperNorm: 0.75, centerNorm: 0.5, fillSpanNorm: 0 });
    result = rangeSliderControlTransition(control, { type: "POINTER_END" });
    expect(result.effects).toEqual([{ type: "emitValueCommit", value: [0.5, 0.5] }]);
  });
});

describe("block appearance helpers", () => {
  test("equality fits and required-minus-one falls back", () => {
    expect(blockItemFits(40, 40)).toBe(true);
    expect(blockItemFits(40, 39.2)).toBe(true);
    expect(blockItemFits(40, 41)).toBe(false);
    expect(blockRegionAvailable(56, 8)).toBe(40);
  });

  test("inline fit is all-or-nothing across assigned items", () => {
    const measure = (text: string) => text.length * 10;
    expect(blockInlineFits(
      [{ text: "Blur", unoccludedSpan: 56 }, { text: "67", unoccludedSpan: 56 }],
      measure,
    )).toBe(true);
    expect(blockInlineFits(
      [{ text: "Blur", unoccludedSpan: 56 }, { text: "too-long-value", unoccludedSpan: 56 }],
      measure,
    )).toBe(false);
    expect(blockInlineFits([{ text: null, unoccludedSpan: 8 }], measure)).toBe(true);
  });

  test("visible channels default to the step-aware decimal and omit empty text", () => {
    expect(resolveSliderVisibleValue(67, 0, 1)).toBe("67");
    expect(resolveSliderVisibleValue(67, 0, 1, () => "")).toBeNull();
  });

  // One shared default serializer. The snapped value rounds and zero-fills to
  // the precision implied by min and a finite positive step, negative zero
  // normalizes, and binary tails never survive.
  test("the default display serializer emits fixed-width step-aware decimals", () => {
    // Fractional step arithmetic without a binary tail.
    expect(defaultVisibleValueText(0.8500000000000001, 0, 0.05)).toBe("0.85");
    expect(defaultVisibleValueText(0.1 + 0.2, 0, 0.1)).toBe("0.3");
    // Precision takes the coarser of min and step into account.
    expect(defaultVisibleValueText(0.35000000000000003, 0.05, 0.1)).toBe("0.35");
    // Integer steps and integer minima never show decimal noise.
    expect(defaultVisibleValueText(44.99999999999999, 0, 1)).toBe("45");
    expect(defaultVisibleValueText(80, 0, 5)).toBe("80");
    // Decimal precision is zero-filled so the label width stays stable.
    expect(defaultVisibleValueText(Number((0.5).toFixed(2)), 0, 0.25)).toBe("0.50");
    expect(defaultVisibleValueText(1.2, 0.2, 0.2)).toBe("1.2");
    // Negative zero normalizes.
    expect(defaultVisibleValueText(-1.1e-16, -1, 0.1)).toBe("0.0");
    // Custom units stay consumer-owned; the serializer only rounds digits.
    expect(defaultVisibleValueText(-0.45, -1, 0.01)).toBe("-0.45");
    // Without a finite positive step the value keeps its shortest form.
    expect(defaultVisibleValueText(0.30000000000000004, 0, 0)).toBe(String(0.30000000000000004));
  });

  test("both range endpoints resolve through the same serializer", () => {
    expect(resolveRangeVisibleValue(0.8500000000000001, 0, 0.05, "lower")).toBe("0.85");
    expect(resolveRangeVisibleValue(0.30000000000000004, 0, 0.1, "upper")).toBe("0.3");
    expect(resolveRangeVisibleValue(20, 0, 10, "lower", (value) => `${value}%`)).toBe("20%");
  });

  test("rtl remaps physical position without changing numeric meaning", () => {
    expect(physicalToValueNorm(0.2, "ltr")).toBeCloseTo(0.2);
    expect(physicalToValueNorm(0.2, "rtl")).toBeCloseTo(0.8);
  });

  test("a second pointer end is inert", () => {
    let control = createSliderControlContext({ value: 10, pointerActive: true });
    const first = sliderControlTransition(control, { type: "POINTER_END" });
    expect(first.effects).toEqual([{ type: "emitValueCommit", value: 10 }]);
    const second = sliderControlTransition(first.context, { type: "POINTER_END" });
    expect(second.effects).toEqual([]);
  });

  test("equal-value range pointer tie chooses lower and holds it", () => {
    let control = createRangeSliderControlContext({ value: [50, 50], min: 0, max: 100, step: 1 });
    let result = rangeSliderControlTransition(control, { type: "POINTER_BEGIN", valueNorm: 0.5 });
    expect(result.context.activeThumb).toBe("lower");
    control = result.context;
    result = rangeSliderControlTransition(control, { type: "POINTER_MOVE", valueNorm: 0.2 });
    expect(result.context.activeThumb).toBe("lower");
    expect(result.context.value).toEqual([20, 50]);
  });

  test("layoutSliderBlock coexists on whole-track fit and suppresses the label on collision", () => {
    // 80px capsule → 64px available; 40+24 fits exactly, one pixel more misses.
    const equal = layoutSliderBlock({
      capsuleSpan: 80,
      label: "Blur",
      valueText: "67",
      measure: (text) => (text === "Blur" ? 40 : 24),
    });
    expect(equal.labelInline).toBe(true);
    expect(equal.valueInline).toBe(true);
    const collision = layoutSliderBlock({
      capsuleSpan: 80,
      label: "Blur",
      valueText: "67",
      measure: (text) => (text === "Blur" ? 40 : 25),
    });
    expect(collision.labelInline).toBe(false);
    // The exact numeric value is never suppressed.
    expect(collision.valueInline).toBe(true);
  });

  test("layoutSliderBlock is value-independent: it never reads a selected span", () => {
    const layout = layoutSliderBlock({
      capsuleSpan: 160,
      label: "Blur",
      valueText: "67",
      measure: (text) => text.length * 10,
    });
    expect(layout).toEqual({ labelInline: true, valueInline: true });
  });

  test("layoutSliderBlock keeps an absent channel in its slot", () => {
    const soloLabel = layoutSliderBlock({
      capsuleSpan: 160,
      label: "Blur",
      valueText: null,
      measure: (text) => text.length * 10,
    });
    expect(soloLabel).toEqual({ labelInline: true, valueInline: false });
    const soloValue = layoutSliderBlock({
      capsuleSpan: 160,
      label: null,
      valueText: "67",
      measure: (text) => text.length * 10,
    });
    expect(soloValue).toEqual({ labelInline: false, valueInline: true });
  });

  test("layoutRangeSliderBlock anchors endpoints and value-independently coexists the label", () => {
    const law = (text: string) => text.length * 10;
    const layout = (capsuleSpan: number) =>
      layoutRangeSliderBlock({
        capsuleSpan,
        label: "Price",
        lowerText: "20",
        upperText: "80",
        measure: law,
      });
    // Endpoints are required and never suppressed.
    expect(layout(300)).toEqual({ labelInline: true, lowerInline: true, upperInline: true });
    // Narrow capsule: only the optional label is suppressed.
    expect(layout(80)).toEqual({ labelInline: false, lowerInline: true, upperInline: true });
    // The law is whole-capsule: value pairs cannot change the decision.
    const solo = layoutRangeSliderBlock({
      capsuleSpan: 160,
      label: null,
      lowerText: "0",
      upperText: "100",
      measure: law,
    });
    expect(solo).toEqual({ labelInline: false, lowerInline: true, upperInline: true });
  });
});
