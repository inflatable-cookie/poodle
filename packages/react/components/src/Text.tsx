import type { ReactNode } from "react";

import "@inflatable-cookie/poodle-core/styles/text.css";

export interface TextProps {
  as?: "p" | "span" | "div";
  tone?: "default" | "secondary" | "muted" | "success" | "danger" | "warning";
  size?: "xs" | "sm" | "md";
  weight?: "normal" | "medium" | "semibold" | "bold";
  leading?: "normal" | "relaxed";
  spacing?: "none" | "compact";
  clamp?: "none" | 1 | 2 | 3;
  children?: ReactNode;
}

export function Text({
  as: As = "p",
  tone = "default",
  size = "md",
  weight = "normal",
  leading = "normal",
  spacing = "none",
  clamp = "none",
  children,
}: TextProps) {
  return (
    <As
      className="poodle-text"
      data-tone={tone}
      data-size={size}
      data-weight={weight}
      data-leading={leading}
      data-spacing={spacing}
      data-clamp={clamp}
    >
      {children}
    </As>
  );
}
