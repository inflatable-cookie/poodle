import "@inflatable-cookie/poodle-core/styles/empty-state.css";

import type { ReactNode } from "react";

import { Icon } from "./Icon";
import { useUiPresentation } from "./presentation";
import type { ControlDensity, EmptyStateSize, EmptyStateVariant } from "./types";

export interface EmptyStateProps {
  title: string;
  message?: string | null;
  variant?: EmptyStateVariant;
  size?: EmptyStateSize;
  density?: ControlDensity | null;
  ariaLabel?: string | null;
  visual?: ReactNode;
  actions?: ReactNode;
}

export function EmptyState({
  title,
  message = null,
  variant = "neutral",
  size = "default",
  density = null,
  ariaLabel = null,
  visual,
  actions,
}: EmptyStateProps) {
  const uiPresentation = useUiPresentation();
  const resolvedDensity = density ?? uiPresentation.density;

  return (
    <section
      className="poodle-empty-state"
      data-variant={variant}
      data-size={size}
      data-density={resolvedDensity}
      aria-label={ariaLabel ?? title}
    >
      <div className="poodle-empty-state__visual" aria-hidden="true">
        {visual ?? (
          variant === "search" ? <Icon name="search" /> : variant === "firstRun" ? <Icon name="plus" /> : <Icon name="inbox" />
        )}
      </div>

      <div className="poodle-empty-state__copy">
        <h3>{title}</h3>
        {message ? <p>{message}</p> : null}
      </div>

      {actions ? <div className="poodle-empty-state__actions">{actions}</div> : null}
    </section>
  );
}
