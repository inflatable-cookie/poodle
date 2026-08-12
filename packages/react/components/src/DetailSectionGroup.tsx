import "@inflatable-cookie/poodle-core/styles/detail-section-group.css";

import type { CSSProperties, ReactNode } from "react";

import { useUiPresentation } from "./presentation";
import type { ControlDensity } from "./types";

export interface DetailSectionGroupProps {
  density?: ControlDensity | null;
  layout?: "grid" | "stack";
  minColumnWidth?: string;
  itemMinColumnWidth?: string;
  maxColumns?: 2 | 3 | 4 | 5;
  ariaLabel?: string | null;
  children?: ReactNode;
}

export function DetailSectionGroup({
  density = null,
  layout = "grid",
  minColumnWidth = "14rem",
  itemMinColumnWidth = "12rem",
  maxColumns = 4,
  ariaLabel = null,
  children,
}: DetailSectionGroupProps) {
  const uiPresentation = useUiPresentation();
  const resolvedDensity = density ?? uiPresentation.density;
  const style = {
    "--poodle-detail-section-group-min": minColumnWidth,
    "--poodle-detail-section-group-item-min": itemMinColumnWidth,
  } as CSSProperties;

  return (
    <div
      className="poodle-detail-section-group"
      data-density={resolvedDensity}
      data-layout={layout}
      data-max-columns={maxColumns}
      aria-label={ariaLabel ?? undefined}
      style={style}
    >
      <div className="poodle-detail-section-group__grid">{children}</div>
    </div>
  );
}
