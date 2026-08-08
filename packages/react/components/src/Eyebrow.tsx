import type { ReactNode } from "react";

import "@inflatable-cookie/poodle-styles/eyebrow.css";

export interface EyebrowProps {
  as?: "span" | "p" | "h2" | "h3" | "h4";
  ariaLabel?: string | null;
  size?: "xs" | "sm" | "md";
  spacing?: "none" | "bottom";
  children?: ReactNode;
}

export function Eyebrow({
  as: As = "span",
  ariaLabel = null,
  size = "sm",
  spacing = "none",
  children,
}: EyebrowProps) {
  return (
    <As className="poodle-eyebrow" data-size={size} data-spacing={spacing} aria-label={ariaLabel ?? undefined}>
      {children}
    </As>
  );
}
