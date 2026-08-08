import type { MouseEvent, ReactNode } from "react";

import "@inflatable-cookie/poodle-core/styles/list-card-counter.css";

import { Icon } from "./Icon";
import { Tooltip } from "./Tooltip";
import type { IconProp } from "./types";

export interface ListCardCounterProps {
  icon: IconProp;
  count: number;
  tooltip?: string | null;
  href?: string | null;
  onClick?: ((event: MouseEvent<HTMLElement>) => void) | null;
  typography?: "label" | "inherit";
}

export function ListCardCounter({
  icon,
  count,
  tooltip = null,
  href = null,
  onClick = null,
  typography = "label",
}: ListCardCounterProps) {
  const handleClick = (e: MouseEvent<HTMLElement>) => {
    if (href) e.stopPropagation();
    onClick?.(e);
  };

  const body: ReactNode = href ? (
    <a className="poodle-list-card-counter" data-typography={typography} href={href} onClick={handleClick}>
      <Icon icon={icon} />
      <span>{count}</span>
    </a>
  ) : (
    <span className="poodle-list-card-counter" data-typography={typography}>
      <Icon icon={icon} />
      <span>{count}</span>
    </span>
  );

  return tooltip ? <Tooltip content={tooltip}>{body}</Tooltip> : body;
}
