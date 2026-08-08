import "@inflatable-cookie/poodle-core/styles/surface.css";

import type { ReactNode } from "react";

import type { SpaceScale, SurfaceBorder, SurfaceTone } from "./types";

export interface SurfaceProps {
  tone?: SurfaceTone;
  border?: SurfaceBorder;
  padding?: SpaceScale;
  elevated?: boolean;
  asRole?: "region" | "group" | null;
  label?: string | null;
  children?: ReactNode;
}

function surfacePadding(scale: SpaceScale): string {
  switch (scale) {
    case "sm":
      return "var(--poodle-space-panel-y)";
    case "md":
      return "1rem";
    case "lg":
      return "1.5rem";
    default:
      return "0";
  }
}

export function Surface({
  tone = "panel",
  border = "subtle",
  padding = "md",
  elevated = false,
  asRole = null,
  label = null,
  children = undefined,
}: SurfaceProps) {
  return (
    <div
      className="poodle-surface"
      data-tone={tone}
      data-border={border}
      data-elevated={elevated}
      role={asRole ?? undefined}
      aria-label={label ?? undefined}
      style={{ padding: surfacePadding(padding) }}
    >
      {children}
    </div>
  );
}
