import "@inflatable-cookie/poodle-core/styles/metric-tile.css";

import { Icon } from "./Icon";
import { useUiPresentation } from "./presentation";
import type { ControlDensity } from "./types";

export interface MetricTileProps {
  label: string;
  value: string;
  ariaLabel?: string | null;
  trend?: "up" | "down" | "flat" | null;
  trendLabel?: string | null;
  sparklineData?: number[] | null;
  density?: ControlDensity | null;
}

function buildSparkline(data: number[]): string {
  const min = Math.min(...data);
  const max = Math.max(...data);
  const range = max - min || 1;
  const w = 64;
  const h = 24;
  const pad = 1;

  return data
    .map((v, i) => {
      const x = pad + (i / (data.length - 1)) * (w - pad * 2);
      const y = pad + (1 - (v - min) / range) * (h - pad * 2);
      return `${i === 0 ? "M" : "L"}${x.toFixed(1)} ${y.toFixed(1)}`;
    })
    .join(" ");
}

export function MetricTile({
  label,
  value,
  ariaLabel = null,
  trend = null,
  trendLabel = null,
  sparklineData = null,
  density = null,
}: MetricTileProps) {
  const uiPresentation = useUiPresentation();
  const resolvedDensity = density ?? uiPresentation.density;

  const sparklinePath = sparklineData && sparklineData.length > 1 ? buildSparkline(sparklineData) : null;

  return (
    <div className="poodle-state-tile" data-density={resolvedDensity} aria-label={ariaLabel ?? `${label}: ${value}`}>
      <span className="poodle-state-tile__label">{label}</span>
      <div className="poodle-state-tile__body">
        <strong className="poodle-state-tile__value">{value}</strong>
        {sparklinePath ? (
          <svg className="poodle-state-tile__sparkline" viewBox="0 0 64 24" fill="none" aria-hidden="true">
            <path
              d={sparklinePath}
              stroke="currentColor"
              strokeWidth="1.5"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        ) : null}
      </div>
      {trend ? (
        <span className="poodle-state-tile__trend" data-trend={trend}>
          <span className="poodle-state-tile__trend-arrow" aria-hidden="true">
            {trend === "up" ? <Icon name="trending-up" /> : trend === "down" ? <Icon name="trending-down" /> : <Icon name="arrow-right" />}
          </span>
          {trendLabel ? <span>{trendLabel}</span> : null}
        </span>
      ) : null}
    </div>
  );
}
