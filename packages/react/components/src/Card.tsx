import "@inflatable-cookie/poodle-core/styles/card.css";

import type { ReactNode } from "react";

import { useUiPresentation } from "./presentation";
import type { CardVariant, ControlDensity } from "./types";

export interface CardProps {
  className?: string;
  variant?: CardVariant;
  layout?: "vertical" | "horizontal" | "compact";
  density?: ControlDensity | null;
  interactive?: boolean;
  selected?: boolean;
  media?: boolean;
  ariaLabel?: string | null;
  mediaContent?: ReactNode;
  header?: ReactNode;
  footer?: ReactNode;
  children?: ReactNode;
}

export function Card({
  className = "",
  variant = "default",
  layout = "vertical",
  density = null,
  interactive = false,
  selected = false,
  media = false,
  ariaLabel = null,
  mediaContent,
  header,
  footer,
  children,
}: CardProps) {
  const uiPresentation = useUiPresentation();
  const resolvedDensity = density ?? uiPresentation.density;

  return (
    <article
      className={`poodle-card ${className}`.trim()}
      data-variant={variant}
      data-layout={layout}
      data-density={resolvedDensity}
      data-interactive={interactive}
      data-selected={selected}
      aria-label={ariaLabel ?? undefined}
    >
      {mediaContent ? (
        <div className="poodle-card__media" data-has-media={media}>
          {mediaContent}
        </div>
      ) : null}

      {header ? <div className="poodle-card__header">{header}</div> : null}

      <div className="poodle-card__body">{children}</div>

      {footer ? <div className="poodle-card__footer">{footer}</div> : null}
    </article>
  );
}
