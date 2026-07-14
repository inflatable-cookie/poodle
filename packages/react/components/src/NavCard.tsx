import "@poodle/styles/nav-card.css";

import type { MouseEvent as ReactMouseEvent, ReactNode } from "react";

import { useUiPresentation } from "./presentation";
import type { ControlDensity } from "./types";

export interface NavCardProps {
  title: string;
  description?: string | null;
  href?: string | null;
  badge?: string | null;
  disabled?: boolean;
  ariaLabel?: string | null;
  density?: ControlDensity | null;
  onClick?: ((event: ReactMouseEvent) => void) | null;
  icon?: ReactNode;
}

export function NavCard({
  title,
  description = null,
  href = null,
  badge = null,
  disabled = false,
  ariaLabel = null,
  density = null,
  onClick = null,
  icon,
}: NavCardProps) {
  const uiPresentation = useUiPresentation();
  const resolvedDensity = density ?? uiPresentation.density;

  function handleClick(event: ReactMouseEvent): void {
    if (disabled) {
      event.preventDefault();
      return;
    }
    onClick?.(event);
  }

  const inner = (
    <>
      {icon ? (
        <span className="poodle-nav-card__icon" aria-hidden="true">
          {icon}
        </span>
      ) : null}
      <div className="poodle-nav-card__content">
        <span className="poodle-nav-card__title">
          {title}
          {badge ? <span className="poodle-nav-card__badge">{badge}</span> : null}
        </span>
        {description ? <span className="poodle-nav-card__description">{description}</span> : null}
      </div>
      <svg className="poodle-nav-card__arrow" viewBox="0 0 16 16" fill="none" aria-hidden="true">
        <path d="M6 4l4 4-4 4" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
      </svg>
    </>
  );

  if (href && !disabled) {
    return (
      <a
        className="poodle-nav-card"
        href={href}
        aria-label={ariaLabel ?? title}
        data-disabled={disabled}
        data-density={resolvedDensity}
        onClick={handleClick}
      >
        {inner}
      </a>
    );
  }

  return (
    <button
      type="button"
      className="poodle-nav-card"
      aria-label={ariaLabel ?? title}
      disabled={disabled}
      data-disabled={disabled}
      data-density={resolvedDensity}
      onClick={handleClick}
    >
      {inner}
    </button>
  );
}
