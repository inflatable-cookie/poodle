import { createElement } from "react";

import "@poodle/styles/icon.css";

import { resolveIconNodes } from "./icon-registry";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, IconNodes, SemanticControlSizeRole } from "./types";

export interface IconProps {
  icon?: IconNodes | string | null;
  name?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  ariaLabel?: string | null;
}

const SVG_TAGS = new Set(["path", "circle", "rect", "line", "polyline", "polygon", "ellipse"]);

export function Icon({
  icon = null,
  name = null,
  size = null,
  sizeRole = "chrome",
  density = null,
  ariaLabel = null,
}: IconProps) {
  const uiPresentation = useUiPresentation();
  const resolvedIcon = icon ?? name;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const nodes = resolveIconNodes(resolvedIcon);

  return (
    <svg
      className="poodle-icon"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      role={ariaLabel ? "img" : "presentation"}
      aria-label={ariaLabel ?? undefined}
      aria-hidden={ariaLabel ? undefined : "true"}
    >
      {nodes.map(([tag, attrs], i) => (SVG_TAGS.has(tag) ? createElement(tag, { ...attrs, key: i }) : null))}
    </svg>
  );
}
