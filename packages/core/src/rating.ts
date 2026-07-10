/**
 * Rating behavior machinery.
 * Contract: docs/contracts/components/rating.md, "Behavior Machine".
 *
 * Pure value math: step resolution, snapping, clamping, clear-on-reselect,
 * pointer-position value resolution, fill ratios, and keyboard stepping.
 * Hover state, focus calls, and DOM geometry reads stay adapter-side.
 */

/** Steps are capped at 1 and fall back to 1 when invalid or non-positive. */
export function resolveRatingStep(step: number): number {
  if (!Number.isFinite(step) || step <= 0) {
    return 1;
  }

  return Math.min(1, step);
}

export function roundRatingToStep(value: number, step: number): number {
  const rounded = Math.round(value / step) * step;

  return Number(rounded.toFixed(4));
}

export function clampRatingDisplayValue(value: number | null | undefined, max: number): number | null {
  if (value === null || value === undefined) {
    return null;
  }

  return Number(Math.max(0, Math.min(max, value)).toFixed(4));
}

export function normalizeRatingValue(
  value: number | null | undefined,
  max: number,
  step: number,
): number | null {
  const clamped = clampRatingDisplayValue(value, max);

  if (clamped === null) {
    return null;
  }

  return roundRatingToStep(clamped, step);
}

/** Display formatting: integers bare, fractions trimmed to two places. */
export function trimRatingFraction(value: number): string {
  return value % 1 === 0 ? `${value}` : value.toFixed(2).replace(/0+$/, "").replace(/\.$/, "");
}

/** Fill ratio of the item at `index` for a given value, in [0, 1]. */
export function ratingFillRatio(index: number, value: number): number {
  return Math.max(0, Math.min(1, value - index));
}

/**
 * Value for a pointer position within the item at `index`:
 * `ratioWithinItem` in [0, 1] snaps UP to the next step (minimum one step).
 */
export function ratingPointerValue(
  ratioWithinItem: number,
  index: number,
  step: number,
  itemCount: number,
): number {
  const snapped = Math.max(step, Math.ceil(ratioWithinItem / step) * step);

  return Math.min(itemCount, index + Math.min(1, snapped));
}

/** Selection with clear-on-reselect: returns the next value (null clears). */
export function ratingSelectValue(
  nextValue: number,
  currentValue: number | null,
  allowClear: boolean,
): number | null {
  if (allowClear && currentValue === nextValue) {
    return null;
  }

  return nextValue;
}

/** Keyboard step: both directions floor at `minSelectableValue`; up caps at `itemCount`. */
export function ratingKeyboardStep(
  currentValue: number,
  direction: 1 | -1,
  step: number,
  itemCount: number,
  minSelectableValue: number,
): number {
  if (direction === 1) {
    return Math.min(itemCount, Math.max(minSelectableValue, currentValue + step));
  }

  return Math.max(minSelectableValue, currentValue - step);
}
