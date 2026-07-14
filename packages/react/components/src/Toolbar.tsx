import "@poodle/styles/toolbar.css";

import { useRef, type CSSProperties, type ReactNode } from "react";

import { getFocusableElements } from "@poodle/headless";

import { controlHeightRem, resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, Orientation, SemanticControlSizeRole } from "./types";

export interface ToolbarProps {
  orientation?: Orientation;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  ariaLabel?: string | null;
  children?: ReactNode;
}

export function Toolbar({
  orientation = "horizontal",
  size = null,
  sizeRole = "chrome",
  density = null,
  ariaLabel = null,
  children,
}: ToolbarProps) {
  const rootRef = useRef<HTMLDivElement | null>(null);
  const uiPresentation = useUiPresentation();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const toolbarStyle = {
    "--poodle-toolbar-control-height": `${controlHeightRem(resolvedSize)}rem`,
  } as CSSProperties;

  function focusSibling(direction: 1 | -1): void {
    const focusable = getFocusableElements(rootRef.current);

    if (focusable.length === 0) {
      return;
    }

    const currentIndex = focusable.indexOf(document.activeElement as HTMLElement);
    const nextIndex = currentIndex === -1 ? 0 : (currentIndex + direction + focusable.length) % focusable.length;
    focusable[nextIndex]?.focus();
  }

  return (
    <div
      ref={rootRef}
      className="poodle-toolbar"
      data-orientation={orientation}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      style={toolbarStyle}
      role="toolbar"
      tabIndex={0}
      aria-label={ariaLabel ?? undefined}
      onKeyDown={(event) => {
        if (orientation === "horizontal" && (event.key === "ArrowRight" || event.key === "ArrowLeft")) {
          event.preventDefault();
          focusSibling(event.key === "ArrowRight" ? 1 : -1);
        }

        if (orientation === "vertical" && (event.key === "ArrowDown" || event.key === "ArrowUp")) {
          event.preventDefault();
          focusSibling(event.key === "ArrowDown" ? 1 : -1);
        }
      }}
    >
      {children}
    </div>
  );
}
