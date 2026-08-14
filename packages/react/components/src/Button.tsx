import { useState, type FocusEvent, type MouseEvent, type ReactNode } from "react";

import "@inflatable-cookie/poodle-core/styles/button.css";

import type {
  ButtonPortableEvents,
  ButtonPortableProps,
} from "@inflatable-cookie/poodle-core/conformance/button";

import { Icon } from "./Icon";
import { resolveSemanticControlSize, resolveSupportingVisualSize, useUiPresentation } from "./presentation";
import { Spinner } from "./Spinner";
import type { IconProp } from "./types";

/**
 * Portable props and events come from the conformance interface authority
 * (`packages/core/src/conformance/button.ts`): renaming a portable prop or
 * event there fails this interface and the component body without editing
 * a second type mirror. The label region is rendered through `children`;
 * web-only HTML and styling props stay extensions here.
 */
/**
 * The props the web shell carries through framework channels (label →
 * children, icons → ReactNode/IconProp). The `satisfies` check binds these
 * names to the interface: renaming a portable prop fails this file.
 */
const carrierProps = ["label", "leadingIcon", "trailingIcon"] as const satisfies
  readonly (keyof ButtonPortableProps)[];
type Portable = Omit<ButtonPortableProps, (typeof carrierProps)[number]>;
type PortableEvents = ButtonPortableEvents;

export interface ButtonProps extends Partial<Portable> {
  type?: "button" | "submit" | "reset";
  form?: string | null;
  formAction?: string | null;
  formNoValidate?: boolean;
  formTarget?: string | null;
  leadingIcon?: IconProp | null;
  trailingIcon?: IconProp | null;
  className?: string;
  onClick?: PortableEvents["press"] | null;
  onFocus?: ((event: FocusEvent<HTMLButtonElement>) => void) | null;
  onBlur?: ((event: FocusEvent<HTMLButtonElement>) => void) | null;
  onPressedChange?: PortableEvents["pressedChange"] | null;
  children?: ReactNode;
  leading?: ReactNode;
  trailing?: ReactNode;
}

export function Button({
  variant = "secondary",
  tone = "default",
  size = null,
  sizeRole = "control",
  density = null,
  type = "button",
  form = null,
  formAction = null,
  formNoValidate = false,
  formTarget = null,
  disabled = false,
  loading = false,
  leadingIcon = null,
  trailingIcon = null,
  chevron = false,
  truncate = false,
  fit = "default",
  maxWidth = null,
  pressed = null,
  defaultPressed = null,
  ariaLabel = null,
  ariaExpanded = null,
  describedBy = null,
  className = "",
  onClick = null,
  onFocus = null,
  onBlur = null,
  onPressedChange = null,
  children,
  leading,
  trailing,
}: ButtonProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledPressed, setUncontrolledPressed] = useState(defaultPressed === true);

  const isToggle = pressed !== null || defaultPressed !== null;
  const pressedControlled = pressed !== null;
  const currentPressed = pressedControlled ? pressed === true : uncontrolledPressed;
  const isUnavailable = disabled || loading;
  const iconOnly = !children;
  const hasLeading = Boolean(leading) || Boolean(leadingIcon) || loading;
  const hasTrailing = Boolean(trailing) || Boolean(trailingIcon) || chevron;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const resolvedIconSize = resolveSupportingVisualSize(resolvedSize);

  function handleClick(event: MouseEvent<HTMLButtonElement>): void {
    if (isToggle) {
      const next = !currentPressed;
      if (!pressedControlled) setUncontrolledPressed(next);
      onPressedChange?.(next);
    }
    // The portable `press` handler takes no payload; the framework carries
    // the DOM event at the boundary.
    (onClick as ((event: MouseEvent<HTMLButtonElement>) => void) | null)?.(event);
  }

  return (
    <button
      type={type}
      form={form ?? undefined}
      formAction={formAction ?? undefined}
      formNoValidate={formNoValidate || undefined}
      formTarget={formTarget ?? undefined}
      className={`poodle-button ${className}`.trim()}
      style={maxWidth ? { maxWidth } : undefined}
      data-variant={variant}
      data-tone={tone !== "default" ? tone : undefined}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-icon-only={iconOnly || undefined}
      data-has-leading={hasLeading || undefined}
      data-has-trailing={hasTrailing || undefined}
      data-truncate={truncate || undefined}
      data-fit={fit !== "default" ? fit : undefined}
      data-loading={loading}
      data-pressed={isToggle ? currentPressed : undefined}
      disabled={isUnavailable}
      aria-label={ariaLabel ?? undefined}
      aria-pressed={isToggle ? currentPressed : undefined}
      aria-expanded={ariaExpanded === null ? undefined : ariaExpanded}
      aria-describedby={describedBy ?? undefined}
      aria-busy={loading ? "true" : undefined}
      onClick={handleClick}
      onFocus={(event) => onFocus?.(event)}
      onBlur={(event) => onBlur?.(event)}
    >
      {loading ? (
        <span className="poodle-button__spinner" aria-hidden="true">
          <Spinner variant="ring" size={resolvedIconSize} tone="current" />
        </span>
      ) : null}

      {leading || leadingIcon ? (
        <span className="poodle-button__icon" aria-hidden="true">
          {leading ?? (leadingIcon ? <Icon icon={leadingIcon} size={resolvedIconSize} /> : null)}
        </span>
      ) : null}

      {children ? <span className="poodle-button__label">{children}</span> : null}

      {trailing || trailingIcon ? (
        <span className="poodle-button__icon" aria-hidden="true">
          {trailing ?? (trailingIcon ? <Icon icon={trailingIcon} size={resolvedIconSize} /> : null)}
        </span>
      ) : null}

      {chevron ? (
        <span className="poodle-button__chevron" aria-hidden="true">
          <Icon name="chevron-down" size={resolvedIconSize} />
        </span>
      ) : null}
    </button>
  );
}
