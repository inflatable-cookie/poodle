import type { CSSProperties } from "react";

import "@inflatable-cookie/poodle-core/styles/skeleton.css";

import { useMotionReady } from "./motion-policy";
import type { SkeletonPreset, SkeletonShape } from "./types";

export interface SkeletonProps {
  shape?: SkeletonShape;
  preset?: SkeletonPreset | null;
  width?: string | null;
  height?: string | null;
  lines?: number;
  animated?: boolean;
}

const pxToRem = (px: number) => `${px / 16}rem`;

function cellStyle(width: string): CSSProperties & Record<string, string> {
  return { "--poodle-skeleton-width": width };
}

export function Skeleton({
  shape = "line",
  preset = null,
  width = null,
  height = null,
  lines = 3,
  animated = true,
}: SkeletonProps) {
  const motionReady = useMotionReady(animated);
  if (preset === "table-row") {
    return (
      <div className="poodle-skeleton-preset poodle-skeleton-preset--table-row" data-animated={animated} data-motion-ready={motionReady} aria-hidden="true">
        {[0, 1, 2, 3].map((i) => (
          <span
            key={i}
            className="poodle-skeleton poodle-skeleton--cell"
            style={cellStyle(i === 0 ? "40%" : i === 3 ? "20%" : "60%")}
          />
        ))}
      </div>
    );
  }
  if (preset === "card") {
    return (
      <div className="poodle-skeleton-preset poodle-skeleton-preset--card" data-animated={animated} data-motion-ready={motionReady} aria-hidden="true">
        <span className="poodle-skeleton poodle-skeleton--block-header" />
        <div className="poodle-skeleton-preset__card-body">
          <span className="poodle-skeleton poodle-skeleton--line" style={cellStyle("80%")} />
          <span className="poodle-skeleton poodle-skeleton--line" style={cellStyle("100%")} />
          <span className="poodle-skeleton poodle-skeleton--line" style={cellStyle("60%")} />
        </div>
        <div className="poodle-skeleton-preset__card-footer">
          <span className="poodle-skeleton poodle-skeleton--pill" />
          <span className="poodle-skeleton poodle-skeleton--pill" />
        </div>
      </div>
    );
  }
  if (preset === "list-item") {
    return (
      <div className="poodle-skeleton-preset poodle-skeleton-preset--list-item" data-animated={animated} data-motion-ready={motionReady} aria-hidden="true">
        <span className="poodle-skeleton poodle-skeleton--avatar" />
        <div className="poodle-skeleton-preset__list-text">
          <span className="poodle-skeleton poodle-skeleton--line" style={cellStyle("60%")} />
          <span className="poodle-skeleton poodle-skeleton--line-sm" style={cellStyle("40%")} />
        </div>
      </div>
    );
  }
  if (preset === "detail-section") {
    return (
      <div className="poodle-skeleton-preset poodle-skeleton-preset--detail" data-animated={animated} data-motion-ready={motionReady} aria-hidden="true">
        <span className="poodle-skeleton poodle-skeleton--heading" />
        {Array.from({ length: lines }, (_, i) => (
          <div key={i} className="poodle-skeleton-preset__detail-item">
            <span className="poodle-skeleton poodle-skeleton--label" />
            <span className="poodle-skeleton poodle-skeleton--value" />
          </div>
        ))}
      </div>
    );
  }
  if (preset === "avatar-line") {
    return (
      <div className="poodle-skeleton-preset poodle-skeleton-preset--avatar-line" data-animated={animated} data-motion-ready={motionReady} aria-hidden="true">
        <span className="poodle-skeleton poodle-skeleton--avatar" />
        <span className="poodle-skeleton poodle-skeleton--line" style={cellStyle("10rem")} />
      </div>
    );
  }

  const resolvedWidth = width ?? (shape === "circle" ? pxToRem(40) : "100%");
  const resolvedHeight = height ?? (shape === "circle" ? pxToRem(40) : shape === "block" ? pxToRem(96) : pxToRem(14));
  return (
    <span
      className="poodle-skeleton"
      data-shape={shape}
      data-animated={animated} data-motion-ready={motionReady}
      style={{ "--poodle-skeleton-width": resolvedWidth, "--poodle-skeleton-height": resolvedHeight } as CSSProperties}
      aria-hidden="true"
    />
  );
}
