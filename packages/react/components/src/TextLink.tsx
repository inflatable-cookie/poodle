import type { MouseEvent, ReactNode } from "react";

import "@poodle/styles/text-link.css";

export interface TextLinkProps {
  href?: string | null;
  target?: string | null;
  rel?: string | null;
  ariaLabel?: string | null;
  disabled?: boolean;
  tone?: "accent" | "inherit" | "secondary";
  className?: string;
  onClick?: ((event: MouseEvent<HTMLElement>) => void) | null;
  children?: ReactNode;
}

export function TextLink({
  href = null,
  target = null,
  rel = null,
  ariaLabel = null,
  disabled = false,
  tone = "accent",
  className = "",
  onClick = null,
  children,
}: TextLinkProps) {
  const rootClassName = `poodle-text-link${className ? ` ${className}` : ""}`;

  const handleClick = (event: MouseEvent<HTMLElement>) => {
    if (disabled) {
      event.preventDefault();
      return;
    }
    onClick?.(event);
  };

  if (href && !disabled) {
    return (
      <a
        className={rootClassName}
        data-tone={tone}
        href={href}
        target={target ?? undefined}
        rel={rel ?? undefined}
        aria-label={ariaLabel ?? undefined}
        onClick={handleClick}
      >
        {children}
      </a>
    );
  }
  return (
    <button
      type="button"
      className={rootClassName}
      data-tone={tone}
      disabled={disabled}
      aria-label={ariaLabel ?? undefined}
      onClick={handleClick}
    >
      {children}
    </button>
  );
}
