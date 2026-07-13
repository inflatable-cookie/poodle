import type { ButtonHTMLAttributes, ReactNode } from "react";

import "@poodle/styles/button.css";

export interface ButtonProps extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, "onClick"> {
  variant?: "primary" | "secondary" | "ghost";
  tone?: "default" | "danger" | "warning";
  size?: "xs" | "sm" | "md" | "lg" | "xl";
  disabled?: boolean;
  onClick?: () => void;
  children?: ReactNode;
}

/**
 * Styled-only shell (classification per the button contract): no machine,
 * markup + shared CSS + tokens/recipes.
 */
export function Button({
  variant = "secondary",
  tone = "default",
  size = "md",
  disabled = false,
  onClick,
  children,
  ...rest
}: ButtonProps) {
  return (
    <button
      type="button"
      className="poodle-button"
      data-variant={variant}
      data-tone={tone !== "default" ? tone : undefined}
      data-size={size}
      disabled={disabled}
      onClick={onClick}
      {...rest}
    >
      <span className="poodle-button__label">{children}</span>
    </button>
  );
}
