/**
 * Slider-family rendering foundation.
 *
 * `Slider` and `RangeSlider` are separate public components with separate
 * value machines, but they share one visual family. This module owns the
 * framework-free web mechanics both adapters must use: the block-axis
 * capsule span, pointer-to-value mapping, the horizontal collision docking
 * law and the bounded inset marker geometry. The matching stylesheet
 * foundation is `styles/slider-family.css`.
 *
 * Contracts: docs/contracts/components/slider.md,
 * docs/contracts/components/range-slider.md ("Slider-family implementation
 * boundary").
 */

import { physicalToValueNorm, type SliderDirection } from "./slider";

export type SliderFamilyOrientation = "horizontal" | "vertical";

/** Logical-pixel effective target for every block handle. */
export const SLIDER_FAMILY_BLOCK_HIT_PX = 44;
/** Handle inset from the capsule edge on the block axis. */
export const SLIDER_FAMILY_BLOCK_MARKER_INSET_PX = 4;
/** Handle line thickness on the inline axis. */
export const SLIDER_FAMILY_BLOCK_MARKER_THICKNESS_PX = 4;
/** Distance from the capsule edge to the handle centre. */
export const SLIDER_FAMILY_BLOCK_MARKER_OFFSET_PX =
  SLIDER_FAMILY_BLOCK_MARKER_INSET_PX + SLIDER_FAMILY_BLOCK_MARKER_THICKNESS_PX / 2;
/** Gap between a docked value glyph and the handle. */
export const SLIDER_FAMILY_TEXT_DOCK_GAP_PX = 4;
/** Reserved inline end inset that the docked value must clear. */
export const SLIDER_FAMILY_TEXT_END_INSET_PX = 12;
/** Minimum breathing room between independently positioned inline text. */
export const SLIDER_FAMILY_INLINE_TEXT_GAP_PX = 8;

export interface SliderFamilyRect {
  left: number;
  top: number;
  width: number;
  height: number;
}

/** Block-axis span (the axis the value advances along) in logical pixels. */
export function sliderFamilyCapsuleSpan(
  rect: Pick<SliderFamilyRect, "width" | "height">,
  orientation: SliderFamilyOrientation,
): number {
  return orientation === "vertical" ? rect.height : rect.width;
}

/** Canvas-usable font shorthand from a rendered Slider-family element. */
export function sliderFamilyResolvedFont(element: Element): string {
  const style = getComputedStyle(element);
  if (style.font) return style.font;
  // Chromium can expose an empty computed shorthand even though every longhand
  // is resolved. Canvas then silently falls back to 10px sans-serif and
  // under-measures values, so construct the usable subset explicitly.
  return `${style.fontStyle} ${style.fontWeight} ${style.fontSize} ${style.fontFamily}`;
}

/** Physical pointer position -> logical value coordinate in `[0, 1]`. */
export function sliderFamilyValueNorm(input: {
  rect: SliderFamilyRect;
  orientation: SliderFamilyOrientation;
  direction: SliderDirection;
  clientX: number;
  clientY: number;
}): number {
  const { rect, orientation, direction, clientX, clientY } = input;
  const physical = orientation === "horizontal"
    ? (clientX - rect.left) / Math.max(rect.width, 1)
    : 1 - (clientY - rect.top) / Math.max(rect.height, 1);

  // Vertical never mirrors; horizontal mirrors under rtl.
  return physicalToValueNorm(physical, orientation === "horizontal" ? direction : "ltr");
}

/**
 * Horizontal collision docking (Slider contract §4): once the marker enters
 * the reserved end text box the value docks one gap before the handle. The
 * accepted web law measures the physical end distance (`1 - valueNorm`); the
 * glyph clipping layers already share one placement in every direction.
 */
export function sliderFamilyValueDockedToMarker(input: {
  valueNorm: number;
  span: number;
  advance: number;
}): boolean {
  const { valueNorm, span, advance } = input;
  if (span <= 0) return false;
  return (1 - valueNorm) * span < advance + SLIDER_FAMILY_TEXT_END_INSET_PX + SLIDER_FAMILY_TEXT_DOCK_GAP_PX;
}

/** Whether a proposed Slider value attachment clears its fixed start label. */
export function sliderBlockDockingFits(input: {
  span: number;
  labelAdvance: number;
  valueAdvance: number;
  valueNorm: number;
  markerInterior: 1 | -1;
}): boolean {
  const { span, labelAdvance, valueAdvance, valueNorm, markerInterior } = input;
  if (span <= 0) return false;
  const contentSpan = Math.max(span - 2 * SLIDER_FAMILY_TEXT_END_INSET_PX, 0);
  const marker = Math.min(
    Math.max(
      valueNorm * contentSpan + markerInterior * SLIDER_FAMILY_BLOCK_MARKER_OFFSET_PX,
      SLIDER_FAMILY_BLOCK_MARKER_OFFSET_PX,
    ),
    contentSpan - SLIDER_FAMILY_BLOCK_MARKER_OFFSET_PX,
  );
  const labelRight = SLIDER_FAMILY_TEXT_END_INSET_PX + labelAdvance;
  const valueLeft = SLIDER_FAMILY_TEXT_END_INSET_PX + marker
    - SLIDER_FAMILY_TEXT_DOCK_GAP_PX - valueAdvance;
  // A numeric-only Slider still needs enough space on the marker's interior
  // side. Otherwise attachment clips the value through the capsule's start
  // edge; leave it on the stable end anchor instead.
  if (valueLeft < SLIDER_FAMILY_TEXT_END_INSET_PX) return false;
  if (labelAdvance <= 0) return true;
  return labelRight + SLIDER_FAMILY_INLINE_TEXT_GAP_PX <= valueLeft;
}

/** Whether the exact-centre RangeSlider label clears both endpoint values. */
interface RangeSliderBlockValueGeometryInput {
  span: number;
  lowerAdvance: number;
  upperAdvance: number;
  lowerNorm: number;
  upperNorm: number;
  lowerDocked: boolean;
  upperDocked: boolean;
}

function rangeSliderBlockValueEdges(input: RangeSliderBlockValueGeometryInput): {
  lowerRight: number;
  upperLeft: number;
} {
  const {
    span, lowerAdvance, upperAdvance,
    lowerNorm, upperNorm, lowerDocked, upperDocked,
  } = input;

  // Docked value offsets resolve against the inline row's content box. The
  // row sits inside the family's 12px-per-side text padding, so using the
  // outer capsule span predicts each value too far toward its edge and hides
  // the centre label late.
  const contentSpan = Math.max(span - 2 * SLIDER_FAMILY_TEXT_END_INSET_PX, 0);
  const contentStart = SLIDER_FAMILY_TEXT_END_INSET_PX;
  const start = contentStart + lowerNorm * contentSpan;
  const end = contentStart + upperNorm * contentSpan;
  const midpoint = (start + end) / 2;
  const halfMarker = SLIDER_FAMILY_BLOCK_MARKER_THICKNESS_PX / 2;
  const lowerMarker = Math.max(
    contentStart + SLIDER_FAMILY_BLOCK_MARKER_OFFSET_PX,
    Math.min(start + SLIDER_FAMILY_BLOCK_MARKER_OFFSET_PX, midpoint - halfMarker),
  );
  const upperMarker = Math.min(
    contentStart + contentSpan - SLIDER_FAMILY_BLOCK_MARKER_OFFSET_PX,
    Math.max(end - SLIDER_FAMILY_BLOCK_MARKER_OFFSET_PX, midpoint + halfMarker),
  );
  const lowerRight = lowerDocked
    ? lowerMarker + halfMarker + SLIDER_FAMILY_TEXT_DOCK_GAP_PX + lowerAdvance
    : SLIDER_FAMILY_TEXT_END_INSET_PX + lowerAdvance;
  const upperLeft = upperDocked
    ? upperMarker - halfMarker - SLIDER_FAMILY_TEXT_DOCK_GAP_PX - upperAdvance
    : span - SLIDER_FAMILY_TEXT_END_INSET_PX - upperAdvance;
  return { lowerRight, upperLeft };
}

/** Whether the proposed attached RangeSlider values fit without overlap. */
export function rangeSliderBlockDockingFits(input: RangeSliderBlockValueGeometryInput): boolean {
  if (input.span <= 0) return false;
  const { lowerRight, upperLeft } = rangeSliderBlockValueEdges(input);
  return lowerRight + SLIDER_FAMILY_INLINE_TEXT_GAP_PX <= upperLeft;
}

/** Whether the exact-centre RangeSlider label clears both endpoint values. */
export function rangeSliderBlockLabelFits(input: RangeSliderBlockValueGeometryInput & {
  labelAdvance: number;
}): boolean {
  const { span, labelAdvance } = input;
  if (span <= 0 || labelAdvance <= 0) return false;
  const { lowerRight, upperLeft } = rangeSliderBlockValueEdges(input);
  const labelLeft = (span - labelAdvance) / 2;
  const labelRight = labelLeft + labelAdvance;

  return lowerRight + SLIDER_FAMILY_TEXT_DOCK_GAP_PX <= labelLeft
    && upperLeft - SLIDER_FAMILY_TEXT_DOCK_GAP_PX >= labelRight;
}

/**
 * Bounded inset marker position as a percentage of the block axis.
 *
 * The handle is clamped to the family marker offset so it never overhangs
 * the capsule, and is inset toward the fill interior (`interior` is `1` when
 * the fill extends toward increasing values, `-1` otherwise). The CSS
 * foundation expresses the same law with `clamp(...)`.
 */
export function sliderFamilyBlockMarkerPercent(input: {
  valuePercent: number;
  spanPx: number;
  interior: 1 | -1;
  markerOffsetPx?: number;
}): number {
  const { valuePercent, spanPx, interior, markerOffsetPx = SLIDER_FAMILY_BLOCK_MARKER_OFFSET_PX } = input;
  if (spanPx <= 0) return valuePercent;
  const offsetPercent = (markerOffsetPx / spanPx) * 100;
  const position = valuePercent + interior * offsetPercent;
  return Math.min(Math.max(position, offsetPercent), 100 - offsetPercent);
}
