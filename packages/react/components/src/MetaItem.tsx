import type { ReactNode } from "react";

import "@inflatable-cookie/poodle-core/styles/meta-item.css";

export interface MetaItemProps {
  label?: string | null;
  ariaLabel?: string | null;
  typography?: "body" | "inherit";
  separator?: boolean;
  children?: ReactNode;
}

export function MetaItem({
  label = null,
  ariaLabel = null,
  typography = "body",
  separator = true,
  children,
}: MetaItemProps) {
  return (
    <span
      className="poodle-meta-item"
      data-typography={typography}
      data-separator={separator}
      aria-label={ariaLabel ?? undefined}
    >
      {label ? <span className="poodle-meta-item__label">{label}</span> : null}
      <span className="poodle-meta-item__value">{children}</span>
    </span>
  );
}
