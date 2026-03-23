import type {
  MenuItem,
  LayoutAlign,
  LayoutJustify,
  OverflowMode,
  ScrollDirection,
  SpaceScale,
} from "./types";

export function joinStyles(parts: Array<string | null | undefined | false>): string {
  return parts.filter(Boolean).join("; ");
}

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

export function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
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

export function overflowForDirection(direction: ScrollDirection): string {
  switch (direction) {
    case "horizontal":
      return "overflow-x: auto; overflow-y: hidden";
    case "both":
      return "overflow: auto";
    default:
      return "overflow-y: auto; overflow-x: hidden";
  }
}

export function overflowValue(mode: OverflowMode): string {
  return mode;
}

export function getFocusableElements(root: HTMLElement | null): HTMLElement[] {
  if (!root) {
    return [];
  }

  return Array.from(
    root.querySelectorAll<HTMLElement>(
      'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])'
    )
  ).filter((element) => !element.hasAttribute("hidden") && element.offsetParent !== null);
}

export function findNextEnabledIndex<T extends { isDisabled?: boolean }>(
  items: T[],
  startIndex: number,
  direction: 1 | -1
): number {
  const count = items.length;

  if (count === 0) {
    return -1;
  }

  let index = startIndex;

  for (let step = 0; step < count; step += 1) {
    index = (index + direction + count) % count;

    if (!items[index]?.isDisabled) {
      return index;
    }
  }

  return startIndex;
}

export function firstEnabledIndex<T extends { isDisabled?: boolean }>(items: T[]): number {
  return items.findIndex((item) => !item.isDisabled);
}

export function menuNavigableItems(items: MenuItem[]): MenuItem[] {
  return items.filter((item) => item.kind !== "separator");
}
