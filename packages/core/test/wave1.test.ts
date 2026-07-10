import { describe, expect, test } from "bun:test";

import { switchTransition } from "../src/switch";
import { singleSelectTransition, type SingleSelectContext } from "../src/single-select";
import { toggleGroupTransition, type ToggleGroupContext } from "../src/toggle-group";
import {
  normalizeRangeValue,
  rangeSliderTransition,
  safeSliderMax,
  sliderTransition,
  snapToStep,
  type RangeSliderContext,
  type SliderContext,
} from "../src/slider";

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
});
