/**
 * Slider and RangeSlider behavior machines.
 * Contracts: docs/contracts/components/slider.md, range-slider.md,
 * "Behavior Machine" sections.
 *
 * Keyboard and pointer interaction come from the native range input; the
 * machine owns value normalization (step snapping, clamping, degenerate-range
 * guard, thumb-crossing prevention) and the change/commit callback split.
 */

export function clampValue(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}

export function snapToStep(value: number, min: number, step: number): number {
  if (!Number.isFinite(step) || step <= 0) {
    return value;
  }

  return min + Math.round((value - min) / step) * step;
}

/** Degenerate ranges (max <= min) widen to one step so percentage math stays finite. */
export function safeSliderMax(min: number, max: number): number {
  return max <= min ? min + 1 : max;
}

export interface SliderContext {
  value: number;
  min: number;
  max: number;
  step: number;
  disabled: boolean;
}

export type SliderEvent =
  | { type: "INPUT"; raw: number }
  | { type: "COMMIT"; raw: number }
  | { type: "SET_VALUE"; value: number };

export type SliderEffect =
  | { type: "emitValueChange"; value: number }
  | { type: "emitValueCommit"; value: number };

export interface SliderResult {
  context: SliderContext;
  effects: SliderEffect[];
}

export function normalizeSliderValue(context: SliderContext, raw: number): number {
  return clampValue(snapToStep(raw, context.min, context.step), context.min, safeSliderMax(context.min, context.max));
}

export function sliderTransition(context: SliderContext, event: SliderEvent): SliderResult {
  switch (event.type) {
    case "INPUT": {
      const value = normalizeSliderValue(context, event.raw);

      return { context: { ...context, value }, effects: [{ type: "emitValueChange", value }] };
    }
    case "COMMIT": {
      const value = normalizeSliderValue(context, event.raw);

      return { context: { ...context, value }, effects: [{ type: "emitValueCommit", value }] };
    }
    case "SET_VALUE": {
      return { context: { ...context, value: event.value }, effects: [] };
    }
  }
}

// ── Range (two-thumb) variant ──

export interface RangeSliderContext {
  value: [number, number];
  min: number;
  max: number;
  step: number;
  disabled: boolean;
}

export type RangeSliderEvent =
  | { type: "INPUT"; thumb: "lower" | "upper"; raw: number }
  | { type: "COMMIT"; thumb: "lower" | "upper"; raw: number }
  | { type: "SET_VALUE"; value: [number, number] };

export type RangeSliderEffect =
  | { type: "emitValueChange"; value: [number, number] }
  | { type: "emitValueCommit"; value: [number, number] };

export interface RangeSliderResult {
  context: RangeSliderContext;
  effects: RangeSliderEffect[];
}

/** Display pair with thumbs ordered and clamped into range. */
export function normalizeRangeValue(context: RangeSliderContext): [number, number] {
  const max = safeSliderMax(context.min, context.max);
  const lower = clampValue(Math.min(context.value[0], context.value[1]), context.min, max);
  const upper = clampValue(Math.max(context.value[0], context.value[1]), context.min, max);

  return [lower, upper];
}

export function rangeSliderTransition(
  context: RangeSliderContext,
  event: RangeSliderEvent,
): RangeSliderResult {
  switch (event.type) {
    case "INPUT":
    case "COMMIT": {
      const max = safeSliderMax(context.min, context.max);
      const [lower, upper] = normalizeRangeValue(context);
      const snapped = snapToStep(event.raw, context.min, context.step);
      // A thumb cannot cross its sibling: lower clamps to [min, upper], upper to [lower, max].
      const value: [number, number] =
        event.thumb === "lower"
          ? [clampValue(snapped, context.min, upper), upper]
          : [lower, clampValue(snapped, lower, max)];

      return {
        context: { ...context, value },
        effects: [
          event.type === "INPUT"
            ? { type: "emitValueChange", value }
            : { type: "emitValueCommit", value },
        ],
      };
    }
    case "SET_VALUE": {
      return { context: { ...context, value: event.value }, effects: [] };
    }
  }
}
