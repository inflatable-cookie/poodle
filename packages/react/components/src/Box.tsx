import type { CSSProperties, ReactNode } from "react";

import "@inflatable-cookie/poodle-styles/box.css";

import { overflowValue, scaleToSpace } from "./internal";
import type { OverflowMode, SpaceScale } from "./types";

export interface BoxProps {
  padding?: SpaceScale;
  width?: string | null;
  height?: string | null;
  minWidth?: string | null;
  minHeight?: string | null;
  overflow?: OverflowMode;
  asRole?: string | null;
  ariaLabel?: string | null;
  children?: ReactNode;
}

export function Box({
  padding = "none",
  width = null,
  height = null,
  minWidth = null,
  minHeight = null,
  overflow = "visible",
  asRole = null,
  ariaLabel = null,
  children,
}: BoxProps) {
  const style: CSSProperties = {
    padding: scaleToSpace(padding),
    overflow: overflowValue(overflow),
    ...(width ? { width } : null),
    ...(height ? { height } : null),
    ...(minWidth ? { minWidth } : null),
    ...(minHeight ? { minHeight } : null),
  };
  return (
    <div className="poodle-box" role={asRole ?? undefined} aria-label={ariaLabel ?? undefined} style={style}>
      {children}
    </div>
  );
}
