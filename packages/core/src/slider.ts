/**
 * Slider and RangeSlider behavior machines.
 * Contracts: docs/contracts/components/slider.md, range-slider.md,
 * "Behavior Machine" sections.
 *
 * Keyboard and pointer interaction come from the native range input; the
 * machine owns value normalization (step snapping, clamping, degenerate-range
 * guard, thumb-crossing prevention) and the change/commit callback split.
 */

import {
  clampAudioValue,
  denormalizeAudioValue,
  linearValueLaw,
  normalizeAudioValue,
  type AudioValueLaw,
} from "./audio/laws";

export type SliderVariant = "standard" | "embedded";
export type SliderPolarity = "unipolar" | "bipolar";

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

export interface SliderControlContext extends SliderContext {
  law: AudioValueLaw;
  polarity: SliderPolarity;
  centerValue: number | null;
  pointerActive: boolean;
}

export type SliderControlEvent =
  | { type: "POINTER_BEGIN"; valueNorm: number }
  | { type: "POINTER_MOVE"; valueNorm: number }
  | { type: "POINTER_END" }
  | { type: "SET_VALUE"; value: number };

export interface SliderVisualState {
  value: number;
  valueNorm: number;
  centerNorm: number;
  fillStartNorm: number;
  fillSpanNorm: number;
  polarity: SliderPolarity;
  pointerActive: boolean;
  enabled: boolean;
}

export function createSliderControlContext(input: Partial<SliderControlContext> = {}): SliderControlContext {
  return {
    value: 0,
    min: 0,
    max: 100,
    step: 1,
    disabled: false,
    law: linearValueLaw,
    polarity: "unipolar",
    centerValue: null,
    pointerActive: false,
    ...input,
  };
}

function sliderCenterValue(context: Pick<SliderControlContext, "min" | "max" | "polarity" | "centerValue">): number {
  if (context.polarity === "unipolar") return clampAudioValue(0, context.min, safeSliderMax(context.min, context.max));
  if (context.centerValue != null) return clampAudioValue(context.centerValue, context.min, safeSliderMax(context.min, context.max));
  return context.min < 0 && context.max > 0 ? 0 : context.min + (safeSliderMax(context.min, context.max) - context.min) / 2;
}

function sliderControlValueAt(context: SliderControlContext, valueNorm: number): number {
  const value = denormalizeAudioValue(valueNorm, context.min, safeSliderMax(context.min, context.max), context.law);
  return normalizeSliderValue(context, value);
}

export function sliderVisualState(context: SliderControlContext): SliderVisualState {
  const max = safeSliderMax(context.min, context.max);
  const value = normalizeSliderValue(context, context.value);
  const valueNorm = normalizeAudioValue(value, context.min, max, context.law);
  const centerNorm = normalizeAudioValue(sliderCenterValue(context), context.min, max, context.law);
  return {
    value,
    valueNorm,
    centerNorm,
    fillStartNorm: Math.min(valueNorm, centerNorm),
    fillSpanNorm: Math.abs(valueNorm - centerNorm),
    polarity: context.polarity,
    pointerActive: context.pointerActive,
    enabled: !context.disabled,
  };
}

export function sliderControlTransition(context: SliderControlContext, event: SliderControlEvent): SliderResult & { context: SliderControlContext } {
  switch (event.type) {
    case "POINTER_BEGIN": {
      if (context.disabled) return { context, effects: [] };
      const value = sliderControlValueAt(context, event.valueNorm);
      return { context: { ...context, value, pointerActive: true }, effects: [{ type: "emitValueChange", value }] };
    }
    case "POINTER_MOVE": {
      if (context.disabled || !context.pointerActive) return { context, effects: [] };
      const value = sliderControlValueAt(context, event.valueNorm);
      return { context: { ...context, value }, effects: [{ type: "emitValueChange", value }] };
    }
    case "POINTER_END": return context.pointerActive
      ? { context: { ...context, pointerActive: false }, effects: [{ type: "emitValueCommit", value: context.value }] }
      : { context, effects: [] };
    case "SET_VALUE": return { context: { ...context, value: normalizeSliderValue(context, event.value) }, effects: [] };
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

export interface RangeSliderControlContext extends RangeSliderContext {
  law: AudioValueLaw;
  polarity: SliderPolarity;
  centerValue: number | null;
  pointerActive: boolean;
  activeThumb: "lower" | "upper" | null;
}

export type RangeSliderControlEvent =
  | { type: "POINTER_BEGIN"; valueNorm: number }
  | { type: "POINTER_MOVE"; valueNorm: number }
  | { type: "POINTER_END" }
  | { type: "SET_VALUE"; value: [number, number] };

export interface RangeSliderVisualState {
  value: [number, number];
  lowerNorm: number;
  upperNorm: number;
  centerNorm: number;
  fillStartNorm: number;
  fillSpanNorm: number;
  polarity: SliderPolarity;
  pointerActive: boolean;
  activeThumb: "lower" | "upper" | null;
  enabled: boolean;
}

export function createRangeSliderControlContext(input: Partial<RangeSliderControlContext> = {}): RangeSliderControlContext {
  return {
    value: [0, 100],
    min: 0,
    max: 100,
    step: 1,
    disabled: false,
    law: linearValueLaw,
    polarity: "unipolar",
    centerValue: null,
    pointerActive: false,
    activeThumb: null,
    ...input,
  };
}

function rangeControlValueAt(context: RangeSliderControlContext, valueNorm: number): number {
  const value = denormalizeAudioValue(valueNorm, context.min, safeSliderMax(context.min, context.max), context.law);
  return clampValue(snapToStep(value, context.min, context.step), context.min, safeSliderMax(context.min, context.max));
}

export function rangeSliderVisualState(context: RangeSliderControlContext): RangeSliderVisualState {
  const max = safeSliderMax(context.min, context.max);
  const value = normalizeRangeValue(context);
  const lowerNorm = normalizeAudioValue(value[0], context.min, max, context.law);
  const upperNorm = normalizeAudioValue(value[1], context.min, max, context.law);
  const centerNorm = normalizeAudioValue(sliderCenterValue(context), context.min, max, context.law);
  return {
    value,
    lowerNorm,
    upperNorm,
    centerNorm,
    fillStartNorm: lowerNorm,
    fillSpanNorm: upperNorm - lowerNorm,
    polarity: context.polarity,
    pointerActive: context.pointerActive,
    activeThumb: context.activeThumb,
    enabled: !context.disabled,
  };
}

export function rangeSliderControlTransition(
  context: RangeSliderControlContext,
  event: RangeSliderControlEvent,
): RangeSliderResult & { context: RangeSliderControlContext } {
  switch (event.type) {
    case "POINTER_BEGIN": {
      if (context.disabled) return { context, effects: [] };
      const visual = rangeSliderVisualState(context);
      const thumb = Math.abs(event.valueNorm - visual.lowerNorm) <= Math.abs(visual.upperNorm - event.valueNorm) ? "lower" : "upper";
      const raw = rangeControlValueAt(context, event.valueNorm);
      const result = rangeSliderTransition(context, { type: "INPUT", thumb, raw });
      return { context: { ...context, value: result.context.value, pointerActive: true, activeThumb: thumb }, effects: result.effects };
    }
    case "POINTER_MOVE": {
      if (context.disabled || !context.pointerActive || !context.activeThumb) return { context, effects: [] };
      const raw = rangeControlValueAt(context, event.valueNorm);
      const result = rangeSliderTransition(context, { type: "INPUT", thumb: context.activeThumb, raw });
      return { context: { ...context, value: result.context.value }, effects: result.effects };
    }
    case "POINTER_END": return context.pointerActive
      ? { context: { ...context, pointerActive: false, activeThumb: null }, effects: [{ type: "emitValueCommit", value: context.value }] }
      : { context, effects: [] };
    case "SET_VALUE": return { context: { ...context, value: normalizeRangeValue({ ...context, value: event.value }) }, effects: [] };
  }
}
