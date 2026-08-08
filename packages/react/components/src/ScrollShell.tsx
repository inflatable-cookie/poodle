import "@inflatable-cookie/poodle-core/styles/scroll-shell.css";

import type { CSSProperties, ReactNode, UIEvent } from "react";

import { scaleToSpace } from "./internal";
import type { ScrollDirection, SpaceScale } from "./types";

export interface ScrollShellProps {
  direction?: ScrollDirection;
  padding?: Extract<SpaceScale, "none" | "sm" | "md">;
  asRole?: "region" | "group" | null;
  label?: string | null;
  focusable?: boolean;
  onScroll?: ((event: UIEvent) => void) | null;
  children?: ReactNode;
}

function overflowForDirection(direction: ScrollDirection): CSSProperties {
  switch (direction) {
    case "horizontal":
      return { overflowX: "auto", overflowY: "hidden" };
    case "both":
      return { overflow: "auto" };
    default:
      return { overflowY: "auto", overflowX: "hidden" };
  }
}

export function ScrollShell({
  direction = "vertical",
  padding = "none",
  asRole = null,
  label = null,
  focusable = false,
  onScroll = null,
  children,
}: ScrollShellProps) {
  const needsHorizontal = direction === "horizontal" || direction === "both";

  const viewportStyle: CSSProperties = {
    ...overflowForDirection(direction),
    padding: scaleToSpace(padding),
    minWidth: 0,
    minHeight: 0,
  };

  return (
    <div className="poodle-scroll-shell">
      <div
        className="poodle-scroll-shell__viewport"
        tabIndex={focusable ? 0 : undefined}
        data-focusable={focusable}
        role={asRole ?? (focusable ? "region" : undefined)}
        aria-label={label ?? (focusable ? "Scrollable content" : undefined)}
        style={viewportStyle}
        onScroll={(event) => onScroll?.(event)}
      >
        <div
          className={
            needsHorizontal
              ? "poodle-scroll-shell__content poodle-scroll-shell__content--h"
              : "poodle-scroll-shell__content"
          }
        >
          {children}
        </div>
      </div>
    </div>
  );
}
