import { describe, expect, test } from "bun:test";

import {
  clampRatingDisplayValue,
  normalizeRatingValue,
  ratingFillRatio,
  ratingKeyboardStep,
  ratingPointerValue,
  ratingSelectValue,
  resolveRatingStep,
  trimRatingFraction,
} from "../src/rating";
import { resizeAxisPosition, resizeDragDelta, resizeKeydownStep } from "../src/resize";

describe("rating math", () => {
  test("step resolution caps at 1 and rejects invalid", () => {
    expect(resolveRatingStep(0.5)).toBe(0.5);
    expect(resolveRatingStep(2)).toBe(1);
    expect(resolveRatingStep(0)).toBe(1);
    expect(resolveRatingStep(Number.NaN)).toBe(1);
  });

  test("normalize clamps then snaps; null passes through", () => {
    expect(normalizeRatingValue(3.26, 5, 0.5)).toBe(3.5);
    expect(normalizeRatingValue(9, 5, 1)).toBe(5);
    expect(normalizeRatingValue(-2, 5, 1)).toBe(0);
    expect(normalizeRatingValue(null, 5, 1)).toBeNull();
    expect(clampRatingDisplayValue(2.123456, 5)).toBe(2.1235);
  });

  test("fraction trimming and fill ratio", () => {
    expect(trimRatingFraction(3)).toBe("3");
    expect(trimRatingFraction(3.5)).toBe("3.5");
    expect(trimRatingFraction(3.25)).toBe("3.25");
    expect(ratingFillRatio(2, 2.5)).toBe(0.5);
    expect(ratingFillRatio(3, 2.5)).toBe(0);
    expect(ratingFillRatio(0, 2.5)).toBe(1);
  });

  test("pointer value snaps up within the item, capped at itemCount", () => {
    expect(ratingPointerValue(0.3, 2, 0.5, 5)).toBe(2.5);
    expect(ratingPointerValue(0.6, 2, 0.5, 5)).toBe(3);
    expect(ratingPointerValue(0.01, 2, 0.5, 5)).toBe(2.5); // minimum one step
    expect(ratingPointerValue(1, 4, 1, 5)).toBe(5);
  });

  test("clear-on-reselect and keyboard stepping", () => {
    expect(ratingSelectValue(3, 3, true)).toBeNull();
    expect(ratingSelectValue(3, 3, false)).toBe(3);
    expect(ratingSelectValue(4, 3, true)).toBe(4);
    expect(ratingKeyboardStep(3, 1, 0.5, 5, 0.5)).toBe(3.5);
    expect(ratingKeyboardStep(5, 1, 0.5, 5, 0.5)).toBe(5);
    expect(ratingKeyboardStep(0.5, -1, 0.5, 5, 0.5)).toBe(0.5); // floors at minSelectable, matching pre-machine behavior
  });
});

describe("resize machinery", () => {
  test("axis position and drag delta", () => {
    expect(resizeAxisPosition("horizontal", 120, 40)).toBe(120);
    expect(resizeAxisPosition("vertical", 120, 40)).toBe(40);
    expect(resizeDragDelta(100, 112)).toEqual({ delta: 12, position: 112 });
  });

  test("keyboard steps respect orientation; Home/End saturate", () => {
    expect(resizeKeydownStep("ArrowRight", "horizontal")).toBe(8);
    expect(resizeKeydownStep("ArrowLeft", "horizontal")).toBe(-8);
    expect(resizeKeydownStep("ArrowRight", "vertical")).toBeNull();
    expect(resizeKeydownStep("ArrowDown", "vertical")).toBe(8);
    expect(resizeKeydownStep("Home", "horizontal")).toBe(-9999);
    expect(resizeKeydownStep("End", "vertical")).toBe(9999);
    expect(resizeKeydownStep("a", "horizontal")).toBeNull();
  });
});
