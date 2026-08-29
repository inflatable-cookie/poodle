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

// Focus and index-navigation helpers moved to the headless core; re-exported
// so existing internal imports keep working.
export { findNextEnabledIndex, firstEnabledIndex, getFocusableElements } from "@inflatable-cookie/poodle-core";

export { menuNavigableItems } from "@inflatable-cookie/poodle-core";
