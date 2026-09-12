/**
 * Slider and RangeSlider behavior machines.
 * Contracts: docs/contracts/components/slider.md, range-slider.md,
 * "Behavior Machine" sections.
 *
 * Keyboard and pointer interaction come from the native range input; the
 * machine owns value normalization (step snapping, clamping, degenerate-range
 * guard, thumb-crossing prevention) and the change/commit callback split.
 *
 * Step quantization follows one portable tie law: a raw index exactly halfway
 * between two steps rounds toward positive infinity (`Math.round`). The Rust
 * mirror (`poodle-headless::slider`) implements the same law explicitly
 * because `f64::round` rounds half away from zero instead.
 */

import {
  clampAudioValue,
  denormalizeAudioValue,
  linearValueLaw,
  normalizeAudioValue,
  type AudioValueLaw,
} from "./audio/laws";

export type SliderVariant = "block" | "embedded";
export type SliderPolarity = "unipolar" | "bipolar";
export type SliderDirection = "ltr" | "rtl";

/** Logical-pixel effective target for every block thumb. */
export const SLIDER_BLOCK_HIT_PX = 44;
/** Internal inline inset used by the block fit law. Not a public metric. */
export const SLIDER_BLOCK_CONTENT_INSET_PX = 8;

export function clampValue(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}

export function snapToStep(value: number, min: number, step: number): number {
  if (!Number.isFinite(step) || step <= 0) {
    return value;
  }

  // Portable tie law: half ties round toward positive infinity via
  // Math.round; poodle-headless mirrors this with ((index) + 0.5).floor().
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
  fillTone: "positive" | "negative";
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
    fillTone: context.polarity === "bipolar" && valueNorm < centerNorm ? "negative" : "positive",
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
  negativeFillStartNorm: number;
  negativeFillSpanNorm: number;
  positiveFillStartNorm: number;
  positiveFillSpanNorm: number;
  fillSplitAtCenter: boolean;
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
  const negativeFillSpanNorm = context.polarity === "bipolar"
    ? Math.max(0, Math.min(upperNorm, centerNorm) - lowerNorm)
    : 0;
  const positiveFillStartNorm = context.polarity === "bipolar" ? Math.max(lowerNorm, centerNorm) : lowerNorm;
  const positiveFillSpanNorm = context.polarity === "bipolar"
    ? Math.max(0, upperNorm - positiveFillStartNorm)
    : upperNorm - lowerNorm;
  return {
    value,
    lowerNorm,
    upperNorm,
    centerNorm,
    fillStartNorm: lowerNorm,
    fillSpanNorm: upperNorm - lowerNorm,
    negativeFillStartNorm: lowerNorm,
    negativeFillSpanNorm,
    positiveFillStartNorm,
    positiveFillSpanNorm,
    fillSplitAtCenter: negativeFillSpanNorm > 0 && positiveFillSpanNorm > 0,
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

export function omitEmptyVisibleText(text: string | null | undefined): string | null {
  if (text == null || text === "") return null;
  return text;
}

/** Decimal places implied by a finite number's shortest representation. */
function impliedDecimalPlaces(value: number): number {
  const text = String(value);
  const expIndex = text.indexOf("e");
  if (expIndex === -1) {
    const dotIndex = text.indexOf(".");
    return dotIndex === -1 ? 0 : text.length - dotIndex - 1;
  }
  const mantissa = text.slice(0, expIndex);
  const mantissaPlaces = mantissa.includes(".") ? mantissa.length - mantissa.indexOf(".") - 1 : 0;
  return Math.max(0, mantissaPlaces - Number(text.slice(expIndex + 1)));
}

/**
 * Decimal precision implied by `min` and a finite positive `step` (g18.024).
 * The snapped value `min + n * step` is exact to at most that many decimal
 * places, so binary tails beyond it are arithmetic noise, never data.
 */
export function sliderDisplayPrecision(min: number, step: number): number {
  const stepPlaces = Number.isFinite(step) && step > 0 ? impliedDecimalPlaces(step) : 0;
  return Math.min(100, Math.max(impliedDecimalPlaces(min), stepPlaces));
}

/**
 * g18.024 default visible value: a short step-aware decimal. The value must
 * already be step-snapped; with a finite positive `step` it is rounded to
 * the precision implied by `min` and `step`, insignificant zeroes are
 * trimmed, and negative zero normalizes to `"0"`. Binary tails never
 * survive. Without a finite positive step no snapping happened, so the
 * value keeps its shortest exact form.
 */
export function defaultVisibleValueText(value: number, min: number, step: number): string {
  if (!Number.isFinite(value)) return String(value);
  if (!Number.isFinite(step) || step <= 0) return String(value);
  const rounded = Number(value.toFixed(sliderDisplayPrecision(min, step)));
  // Number→String prints the shortest exact decimal (and `-0` as `"0"`),
  // which performs the trimming and negative-zero normalization for free.
  return String(rounded);
}


export function resolveSliderVisibleValue(
  value: number,
  min: number,
  step: number,
  formatVisibleValue?: ((value: number) => string) | null,
): string | null {
  return omitEmptyVisibleText(
    formatVisibleValue ? formatVisibleValue(value) : defaultVisibleValueText(value, min, step),
  );
}

export function resolveRangeVisibleValue(
  value: number,
  min: number,
  step: number,
  thumb: "lower" | "upper",
  formatVisibleValue?: ((value: number, thumb: "lower" | "upper") => string) | null,
): string | null {
  return omitEmptyVisibleText(
    formatVisibleValue
      ? formatVisibleValue(value, thumb)
      : defaultVisibleValueText(value, min, step),
  );
}

export function blockRegionAvailable(unoccludedSpan: number, contentInset = SLIDER_BLOCK_CONTENT_INSET_PX): number {
  return Math.floor(unoccludedSpan - 2 * contentInset);
}

export function blockItemFits(available: number, requiredAdvance: number): boolean {
  return available >= Math.ceil(requiredAdvance);
}

export interface BlockAssignedItem {
  text: string | null;
  unoccludedSpan: number;
}

export function blockInlineFits(
  items: BlockAssignedItem[],
  measure: (text: string) => number,
  contentInset = SLIDER_BLOCK_CONTENT_INSET_PX,
): boolean {
  return items.every((item) => {
    if (!item.text) return true;
    return blockItemFits(blockRegionAvailable(item.unoccludedSpan, contentInset), measure(item.text));
  });
}

export function physicalToValueNorm(physicalNorm: number, direction: SliderDirection): number {
  const clamped = Math.min(Math.max(physicalNorm, 0), 1);
  return direction === "rtl" ? 1 - clamped : clamped;
}

/** Result of the single-Slider whole-track collision law (contract §4). */
export interface SliderBlockTextLayout {
  /** The optional visible label paints inline at the logical start. */
  labelInline: boolean;
  /** The exact numeric value paints inline at the logical end. Never suppressed. */
  valueInline: boolean;
}

/**
 * Fixed block inline placement for a single Slider (g18.017).
 *
 * Geometry is whole-track: the label and value are pinned to the logical
 * inline edges at every value, so the fit decision never depends on the
 * selected span. When the two strings cannot coexist across the track the
 * optional label is suppressed and the exact value stays put. There is no
 * external fallback in this appearance.
 */
export function layoutSliderBlock(input: {
  capsuleSpan: number;
  label: string | null;
  valueText: string | null;
  measure: (text: string) => number;
}): SliderBlockTextLayout {
  const available = blockRegionAvailable(input.capsuleSpan);
  const labelAdvance = input.label ? Math.ceil(input.measure(input.label)) : 0;
  const valueAdvance = input.valueText ? Math.ceil(input.measure(input.valueText)) : 0;
  const valueInline = input.valueText != null;
  const labelInline = input.label != null && available >= labelAdvance + valueAdvance;
  return { labelInline, valueInline };
}

export interface RangeSliderBlockTextLayout {
  /** The optional visible label paints at the capsule center. */
  labelInline: boolean;
  /** The required lower endpoint value paints at the logical-start anchor. */
  lowerInline: boolean;
  /** The required upper endpoint value paints at the logical-end anchor. */
  upperInline: boolean;
}

/**
 * Fixed whole-capsule anchors for RangeSlider block text (g18.022).
 *
 * Geometry never depends on the selected window: the lower value is pinned
 * to the logical start, the upper value to the logical end, and the optional
 * label to the center, at every value pair. The endpoint values are required
 * and always paint when present; when the three items cannot coexist inside
 * the capsule the optional label is suppressed. There is no external
 * fallback in this variant. `capsuleSpan` is measured along the text axis
 * (horizontal width or vertical height).
 */
export function layoutRangeSliderBlock(input: {
  capsuleSpan: number;
  label: string | null;
  lowerText: string | null;
  upperText: string | null;
  measure: (text: string) => number;
}): RangeSliderBlockTextLayout {
  const lowerText = input.lowerText != null;
  const upperText = input.upperText != null;
  const lowerAdvance = input.lowerText ? Math.ceil(input.measure(input.lowerText)) : 0;
  const upperAdvance = input.upperText ? Math.ceil(input.measure(input.upperText)) : 0;
  const labelAdvance = input.label ? Math.ceil(input.measure(input.label)) : 0;
  // One content inset between each of the three fixed anchors.
  const coexist = blockRegionAvailable(input.capsuleSpan)
    >= lowerAdvance + upperAdvance + labelAdvance + 2 * SLIDER_BLOCK_CONTENT_INSET_PX;
  return {
    labelInline: input.label != null && coexist,
    lowerInline: lowerText,
    upperInline: upperText,
  };
}

export function measureInlineAdvance(text: string, font: string): number {
  if (!text) return 0;
  if (typeof document === "undefined") return text.length * 8;
  try {
    const canvas = document.createElement("canvas");
    const context = canvas.getContext("2d");
    if (!context) return text.length * 8;
    context.font = font;
    const width = context.measureText(text).width;
    return width > 0 ? width : text.length * 8;
  } catch {
    return text.length * 8;
  }
}
