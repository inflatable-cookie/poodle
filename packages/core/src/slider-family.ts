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
