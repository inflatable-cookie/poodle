import type { CSSProperties, ReactNode } from "react";

import "@inflatable-cookie/poodle-core/styles/stack.css";

import { alignItemsValue, justifyContentValue, overflowValue, scaleToSpace } from "./internal";
import type { LayoutAlign, LayoutJustify, OverflowMode, SpaceScale } from "./types";

export interface StackProps {
  direction?: "column" | "row";
  gap?: SpaceScale;
  align?: LayoutAlign;
  justify?: LayoutJustify;
  wrap?: boolean;
  padding?: SpaceScale;
  width?: string | null;
  height?: string | null;
  minWidth?: string | null;
  minHeight?: string | null;
  overflow?: OverflowMode;
  asRole?: string | null;
  ariaLabel?: string | null;
  className?: string;
  children?: ReactNode;
}

export function Stack({
  direction = "column",
  gap = "md",
  align,
  justify = "start",
  wrap = false,
  padding = "none",
  width = null,
  height = null,
  minWidth = null,
  minHeight = null,
  overflow = "visible",
  asRole = null,
  ariaLabel = null,
  className = "",
  children,
}: StackProps) {
  const resolvedAlign = align ?? (direction === "column" ? "stretch" : "center");
  const style: CSSProperties = {
    flexDirection: direction,
    gap: scaleToSpace(gap),
    padding: scaleToSpace(padding),
    alignItems: alignItemsValue(resolvedAlign),
    justifyContent: justifyContentValue(justify),
    flexWrap: wrap ? "wrap" : "nowrap",
    overflow: overflowValue(overflow),
    ...(width ? { width } : null),
    ...(height ? { height } : null),
    ...(minWidth ? { minWidth } : null),
    ...(minHeight ? { minHeight } : null),
  };
  return (
    <div
      className={`poodle-stack ${className}`.trim()}
      role={asRole ?? undefined}
      aria-label={ariaLabel ?? undefined}
      style={style}
    >
      {children}
    </div>
  );
}
