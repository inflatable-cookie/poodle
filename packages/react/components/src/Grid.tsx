import type { CSSProperties, ReactNode } from "react";

import "@poodle/styles/grid.css";

import { scaleToSpace } from "./internal";
import type { SpaceScale } from "./types";

export interface GridProps {
  columns?: string;
  rows?: string | null;
  gap?: SpaceScale;
  padding?: SpaceScale;
  asRole?: string | null;
  ariaLabel?: string | null;
  children?: ReactNode;
}

export function Grid({
  columns = "1fr",
  rows = null,
  gap = "md",
  padding = "none",
  asRole = null,
  ariaLabel = null,
  children,
}: GridProps) {
  const style: CSSProperties = {
    gridTemplateColumns: columns,
    gap: scaleToSpace(gap),
    padding: scaleToSpace(padding),
    ...(rows ? { gridTemplateRows: rows } : null),
  };
  return (
    <div className="poodle-grid" role={asRole ?? undefined} aria-label={ariaLabel ?? undefined} style={style}>
      {children}
    </div>
  );
}
