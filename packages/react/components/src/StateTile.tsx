import "@inflatable-cookie/poodle-core/styles/state-tile.css";
import type { ReactNode } from "react";
import { Icon } from "./Icon";

export interface StateTileProps {
  label: string;
  value: string;
  trend?: string | null;
  trendLabel?: string | null;
  hasSparkline?: boolean;
  sparkline?: ReactNode;
}

export function StateTile({ label, value, trend = null, trendLabel = null, hasSparkline = false, sparkline }: StateTileProps) {
  const trendIcon = trend === "up" ? "trending-up" : trend === "down" ? "trending-down" : "arrow-right";
  return (
    <div className="poodle-state-tile" data-component="state-tile">
      <span className="poodle-state-tile__label">{label}</span>
      <div className="poodle-state-tile__body">
        <strong className="poodle-state-tile__value">{value}</strong>
        {hasSparkline ? <div className="poodle-state-tile__sparkline">{sparkline}</div> : null}
      </div>
      {trend ? (
        <span className="poodle-state-tile__trend" data-trend={trend}>
          <span className="poodle-state-tile__trend-arrow" aria-hidden="true"><Icon name={trendIcon} /></span>
          <span>{trendLabel ?? trend}</span>
        </span>
      ) : null}
    </div>
  );
}
