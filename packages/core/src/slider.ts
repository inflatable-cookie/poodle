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
export type SliderAppearance = "track" | "block";
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

export function assertHorizontalBlockAppearance(
  appearance: SliderAppearance,
  orientation: "horizontal" | "vertical",
  component: "Slider" | "RangeSlider" = "Slider",
): void {
  if (appearance === "block" && orientation === "vertical") {
    throw new Error(`${component} appearance="block" rejects orientation="vertical"`);
  }
}

export function omitEmptyVisibleText(text: string | null | undefined): string | null {
  if (text == null || text === "") return null;
  return text;
}

export function defaultVisibleValueText(value: number): string {
  return String(value);
}

export function defaultVisibleRangeText(lower: number, upper: number): string {
  return `${defaultVisibleValueText(lower)} – ${defaultVisibleValueText(upper)}`;
}

export function resolveSliderVisibleValue(
  value: number,
  formatVisibleValue?: ((value: number) => string) | null,
): string | null {
  return omitEmptyVisibleText((formatVisibleValue ?? defaultVisibleValueText)(value));
}

export function resolveRangeVisibleValue(
  value: number,
  thumb: "lower" | "upper",
  formatVisibleValue?: ((value: number, thumb: "lower" | "upper") => string) | null,
): string | null {
  return omitEmptyVisibleText(
    formatVisibleValue ? formatVisibleValue(value, thumb) : defaultVisibleValueText(value),
  );
}

export function resolveRangeVisibleRange(
  lower: number,
  upper: number,
  formatVisibleRange?: ((lower: number, upper: number) => string) | null,
  formatVisibleValue?: ((value: number, thumb: "lower" | "upper") => string) | null,
): string | null {
  if (formatVisibleRange) return omitEmptyVisibleText(formatVisibleRange(lower, upper));
  const lowerText = resolveRangeVisibleValue(lower, "lower", formatVisibleValue) ?? defaultVisibleValueText(lower);
  const upperText = resolveRangeVisibleValue(upper, "upper", formatVisibleValue) ?? defaultVisibleValueText(upper);
  return omitEmptyVisibleText(`${lowerText} – ${upperText}`);
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

export function sliderFallbackText(label: string | null, valueText: string | null): string | null {
  return omitEmptyVisibleText([label, valueText].filter((part): part is string => part != null && part !== "").join(" "));
}

export function rangeSliderFallbackText(label: string | null, rangeText: string | null): string | null {
  return omitEmptyVisibleText([label, rangeText].filter((part): part is string => part != null && part !== "").join(" "));
}

export function physicalToValueNorm(physicalNorm: number, direction: SliderDirection): number {
  const clamped = Math.min(Math.max(physicalNorm, 0), 1);
  return direction === "rtl" ? 1 - clamped : clamped;
}

export function layoutSliderBlock(input: {
  capsuleSpan: number;
  selectedNorm: number;
  label: string | null;
  valueText: string | null;
  measure: (text: string) => number;
}): { inline: boolean; fallback: string | null } {
  const selectedSpan = Math.max(input.selectedNorm, 0) * input.capsuleSpan;
  const remainderSpan = Math.max(1 - input.selectedNorm, 0) * input.capsuleSpan;
  const inline = blockInlineFits(
    [
      { text: input.label, unoccludedSpan: selectedSpan },
      { text: input.valueText, unoccludedSpan: remainderSpan },
    ],
    input.measure,
  );
  return { inline, fallback: inline ? null : sliderFallbackText(input.label, input.valueText) };
}

export function layoutRangeSliderBlock(input: {
  capsuleSpan: number;
  lowerNorm: number;
  upperNorm: number;
  label: string | null;
  lowerText: string | null;
  upperText: string | null;
  rangeText: string | null;
  measure: (text: string) => number;
}): { inline: boolean; fallback: string | null; selectedText: string | null } {
  const selectedSpan = Math.max(input.upperNorm - input.lowerNorm, 0) * input.capsuleSpan;
  const lowerSpan = Math.max(input.lowerNorm, 0) * input.capsuleSpan;
  const upperSpan = Math.max(1 - input.upperNorm, 0) * input.capsuleSpan;
  const selectedText = input.label ?? input.rangeText;
  const inline = blockInlineFits(
    [
      { text: selectedText, unoccludedSpan: selectedSpan },
      { text: input.lowerText, unoccludedSpan: lowerSpan },
      { text: input.upperText, unoccludedSpan: upperSpan },
    ],
    input.measure,
  );
  return {
    inline,
    selectedText,
    fallback: inline ? null : rangeSliderFallbackText(input.label, input.rangeText),
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
