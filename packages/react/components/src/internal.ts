import type { LayoutAlign, LayoutJustify, OverflowMode, SpaceScale } from "./types";

/** Layout helpers mirroring the Svelte package's `internal.ts` tables. */

export function scaleToSpace(scale: SpaceScale): string {
  switch (scale) {
    case "sm":
      return "var(--poodle-space-inline-sm)";
    case "md":
      return "var(--poodle-space-panel-y)";
    case "lg":
      return "var(--poodle-space-panel-x)";
    default:
      return "0";
  }
}

export function alignItemsValue(align: LayoutAlign): string {
  switch (align) {
    case "start":
      return "flex-start";
    case "end":
      return "flex-end";
    case "center":
      return "center";
    default:
      return "stretch";
  }
}

export function justifyContentValue(justify: LayoutJustify): string {
  switch (justify) {
    case "center":
      return "center";
    case "end":
      return "flex-end";
    case "between":
      return "space-between";
    default:
      return "flex-start";
  }
}

export function overflowValue(mode: OverflowMode): string {
  return mode;
}

export function snapToStep(value: number, min: number, step: number): number {
  if (step <= 0) {
    return value;
  }
  const steps = Math.round((value - min) / step);
  return min + steps * step;
}

export function formatNumber(value: number | null, precision: number | null = null): string {
  if (value === null || Number.isNaN(value)) {
    return "";
  }
  if (precision === null) {
    return String(value);
  }
  return value.toFixed(precision);
}
