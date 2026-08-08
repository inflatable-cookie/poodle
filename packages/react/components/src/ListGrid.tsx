import type { CSSProperties, ReactNode } from "react";

import "@inflatable-cookie/poodle-styles/list-grid.css";

export interface ListGridProps {
  variant?: "default" | "compact";
  minItemWidth?: number | string | null;
  maxColumns?: number | null;
  gap?: number | string | null;
  className?: string;
  actions?: ReactNode;
  children?: ReactNode;
}

function formatValue(value: number | string | null, defaultUnit: string): string | null {
  if (value == null) return null;
  if (typeof value === "number") return `${value}${defaultUnit}`;
  return value;
}

export function ListGrid({
  variant = "default",
  minItemWidth = null,
  maxColumns = 3,
  gap = null,
  className = "",
  actions,
  children,
}: ListGridProps) {
  const min = formatValue(minItemWidth, "em") ?? "360px";
  const gridGap = formatValue(gap, "px") ?? (variant === "compact" ? "0.5rem" : "1.25rem");
  const columnCap = maxColumns == null ? null : Math.max(1, Math.floor(maxColumns));
  const columns =
    variant === "compact"
      ? "1fr"
      : columnCap == null
        ? `repeat(auto-fill, minmax(min(${min}, 100%), 1fr))`
        : `repeat(auto-fill, minmax(min(100%, max(${min}, calc((100% - (${columnCap} - 1) * ${gridGap}) / ${columnCap}))), 1fr))`;

  return (
    <div
      className={`poodle-list-grid ${className}`.trim()}
      style={{ "--poodle-list-grid-gap": gridGap } as CSSProperties}
    >
      {actions ? <div className="poodle-list-grid__header">{actions}</div> : null}
      <div className="poodle-list-grid__content" style={{ gridTemplateColumns: columns, gap: gridGap }}>
        {children}
      </div>
    </div>
  );
}
