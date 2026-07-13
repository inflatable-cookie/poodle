import type { CSSProperties } from "react";

import "@poodle/styles/region.css";

export interface RegionProps {
  label?: string;
  color?: string | null;
  minHeight?: string;
}

export function Region({ label = "", color = null, minHeight = "4rem" }: RegionProps) {
  const style: CSSProperties & Record<string, string> = { minHeight };
  if (color) style["--region-color"] = color;
  return (
    <div className="poodle-region" style={style} role="presentation">
      {label ? <span className="poodle-region__label">{label}</span> : null}
    </div>
  );
}
