import { useState, type FocusEvent, type MouseEvent, type ReactNode } from "react";

import "@inflatable-cookie/poodle-core/styles/button.css";

import { buttonDefinition } from "./generated/button";

import { Icon } from "./Icon";
import { resolveSemanticControlSize, resolveSupportingVisualSize, useUiPresentation } from "./presentation";
import { Spinner } from "./Spinner";
import type {
  ButtonTone,
  ButtonVariant,
  ControlDensity,
  ControlSize,
  IconProp,
  SemanticControlSizeRole,
} from "./types";

// The definition owns the rendered vocabulary (card 041 R2): the anatomy's
// DOM classes and the eleven data-* attribute names. A rename in
// packages/codegen/src/models/button.rs moves the DOM here with no hand
// edit; `effigy ir:check` gates drift in the artifact.
const parts = new Map<string, string>(buttonDefinition.parts.map((part) => [part.id, part.className]));
const attributes = new Map<string, string>(buttonDefinition.attributes.map((attribute) => [attribute.id, attribute.name]));

function partClass(id: string): string {
  const className = parts.get(id);
  if (!className) throw new Error(`Button definition has no part '${id}'`);
  return className;
}

function attributeName(id: string): string {
  const name = attributes.get(id);
  if (!name) throw new Error(`Button definition has no attribute '${id}'`);
  return name;
}

export interface ButtonProps {
  variant?: ButtonVariant;
  tone?: ButtonTone;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  type?: "button" | "submit" | "reset";
  form?: string | null;
  formAction?: string | null;
  formNoValidate?: boolean;
  formTarget?: string | null;
  disabled?: boolean;
  loading?: boolean;
  leadingIcon?: IconProp | null;
  trailingIcon?: IconProp | null;
  chevron?: boolean;
  truncate?: boolean;
  fit?: "default" | "content";
  maxWidth?: string | null;
  pressed?: boolean | null;
  defaultPressed?: boolean | null;
  ariaLabel?: string | null;
  ariaExpanded?: boolean | null;
  describedBy?: string | null;
  className?: string;
  onClick?: ((event: MouseEvent<HTMLButtonElement>) => void) | null;
  onFocus?: ((event: FocusEvent<HTMLButtonElement>) => void) | null;
  onBlur?: ((event: FocusEvent<HTMLButtonElement>) => void) | null;
  onPressedChange?: ((pressed: boolean) => void) | null;
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

  // The eleven data-* attributes, emitted from the definition's attribute
  // names (R2). The value derivation stays here — it is the runtime's
  // projection (CROSS-14) — but the names come from button.rs.
  const dataAttributes = {
    [attributeName("variant")]: variant,
    [attributeName("tone")]: tone !== "default" ? tone : undefined,
    [attributeName("size")]: resolvedSize,
    [attributeName("density")]: resolvedDensity,
    [attributeName("icon-only")]: iconOnly || undefined,
    [attributeName("has-leading")]: hasLeading || undefined,
    [attributeName("has-trailing")]: hasTrailing || undefined,
    [attributeName("truncate")]: truncate || undefined,
    [attributeName("fit")]: fit !== "default" ? fit : undefined,
    [attributeName("loading")]: loading,
    [attributeName("pressed")]: isToggle ? currentPressed : undefined,
  };

  function handleClick(event: MouseEvent<HTMLButtonElement>): void {
    if (isToggle) {
      const next = !currentPressed;
      if (!pressedControlled) setUncontrolledPressed(next);
      onPressedChange?.(next);
    }
    onClick?.(event);
  }

  return (
    <button
      type={type}
      form={form ?? undefined}
      formAction={formAction ?? undefined}
      formNoValidate={formNoValidate || undefined}
      formTarget={formTarget ?? undefined}
      className={`${partClass("root")} ${className}`.trim()}
      style={maxWidth ? { maxWidth } : undefined}
      {...dataAttributes}
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
        <span className={partClass("spinner")} aria-hidden="true">
          <Spinner variant="ring" size={resolvedIconSize} tone="current" />
        </span>
      ) : null}

      {leading || leadingIcon ? (
        <span className={partClass("leading-icon")} aria-hidden="true">
          {leading ?? (leadingIcon ? <Icon icon={leadingIcon} size={resolvedIconSize} /> : null)}
        </span>
      ) : null}

      {children ? <span className={partClass("label")}>{children}</span> : null}

      {trailing || trailingIcon ? (
        <span className={partClass("trailing-icon")} aria-hidden="true">
          {trailing ?? (trailingIcon ? <Icon icon={trailingIcon} size={resolvedIconSize} /> : null)}
        </span>
      ) : null}

      {chevron ? (
        <span className={partClass("chevron")} aria-hidden="true">
          <Icon name="chevron-down" size={resolvedIconSize} />
        </span>
      ) : null}
    </button>
  );
}
