import "@inflatable-cookie/poodle-styles/separator.css";

import type { SeparatorTone } from "./types";

export type Orientation = "horizontal" | "vertical";

export interface SeparatorProps {
  orientation?: Orientation;
  decorative?: boolean;
  tone?: SeparatorTone;
}

export function Separator({ orientation = "horizontal", decorative = true, tone = "subtle" }: SeparatorProps) {
  return (
    <div
      className="poodle-separator"
      data-orientation={orientation}
      data-tone={tone}
      role={decorative ? undefined : "separator"}
      aria-hidden={decorative ? "true" : undefined}
      aria-orientation={decorative ? undefined : orientation}
    />
  );
}
