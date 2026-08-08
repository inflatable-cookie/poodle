import "@inflatable-cookie/poodle-styles/status-bar.css";

import type { ReactNode } from "react";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface StatusBarProps {
  summary?: string | null;
  ariaLabel?: string | null;
  chrome?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  leading?: ReactNode;
  trailing?: ReactNode;
}

export function StatusBar({
  summary = null,
  ariaLabel = null,
  chrome = false,
  size = null,
  sizeRole = "chrome",
  density = null,
  leading,
  trailing,
}: StatusBarProps) {
  const uiPresentation = useUiPresentation();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  return (
    <footer
      className={chrome ? "poodle-status-bar poodle-status-bar--chrome" : "poodle-status-bar"}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      aria-label={ariaLabel ?? summary ?? "Status"}
    >
      <div className="poodle-status-bar__leading">{leading ?? (summary ? <span>{summary}</span> : null)}</div>

      {trailing ? <div className="poodle-status-bar__trailing">{trailing}</div> : null}
    </footer>
  );
}
