import type { CSSProperties, ReactNode } from "react";

import "@inflatable-cookie/poodle-styles/field.css";

import { Icon } from "./Icon";
import { Popover } from "./Popover";
import { resolveSemanticControlSize, UiPresentationProvider, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState } from "./types";

export interface FieldControlProps {
  describedBy: string | null;
  descriptionId: string | null;
  errorId: string | null;
  messageId: string | null;
  validationState: ValidationState;
}

export interface FieldProps {
  id: string;
  label: string;
  description?: string | null;
  hint?: string | null;
  error?: string | null;
  pendingMessage?: string | null;
  validationState?: ValidationState;
  required?: boolean;
  optionalLabel?: string | null;
  span?: number | "full" | null;
  gridArea?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  control?: (props: FieldControlProps) => ReactNode;
  children?: ReactNode;
}

export function Field({
  id,
  label,
  description = null,
  hint = null,
  error = null,
  pendingMessage = null,
  validationState = "none",
  required = false,
  optionalLabel = null,
  span = null,
  gridArea = null,
  size = null,
  sizeRole = "control",
  density = null,
  control,
  children,
}: FieldProps) {
  const uiPresentation = useUiPresentation();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const infoText = description ?? hint;
  const descriptionId = description ? `${id}-description` : null;
  const errorId = error ? `${id}-error` : null;
  const pendingId = pendingMessage ? `${id}-pending` : null;
  const messageId =
    validationState === "invalid" && errorId
      ? errorId
      : validationState === "pending" && pendingId
        ? pendingId
        : null;
  const describedBy = [descriptionId, messageId].filter(Boolean).join(" ") || null;
  const fieldStyle: CSSProperties = {
    ...(span ? (span === "full" ? { gridColumn: "1 / -1" } : { gridColumn: `span ${span}` }) : null),
    ...(gridArea ? { gridArea } : null),
  };

  return (
    <div
      className="poodle-field"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-validation-state={validationState}
      style={fieldStyle}
    >
      <div className="poodle-field__header">
        <div className="poodle-field__label-row">
          <label className="poodle-field__label" htmlFor={id}>
            {label}
            {required ? (
              <span className="poodle-field__required" aria-hidden="true">
                *
              </span>
            ) : null}
          </label>
          {infoText ? (
            <Popover
              placement="top"
              offset={6}
              ariaLabel="Field description"
              trigger={
                <span className="poodle-field__info-trigger-wrap">
                  <span className="poodle-field__info-icon" aria-label="More information">
                    <Icon name="info" />
                  </span>
                </span>
              }
            >
              <p className="poodle-field__info-content">{infoText}</p>
            </Popover>
          ) : null}
        </div>
        {!required && optionalLabel ? <span className="poodle-field__optional">{optionalLabel}</span> : null}
      </div>

      <div className="poodle-field__control">
        <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
          {control
            ? control({ describedBy, descriptionId, errorId, messageId, validationState })
            : children}
        </UiPresentationProvider>
      </div>

      {description ? (
        <span id={descriptionId ?? undefined} className="poodle-field__sr-description">
          {description}
        </span>
      ) : null}

      {validationState === "invalid" && error ? (
        <p className="poodle-field__message poodle-field__message--error" id={errorId ?? undefined} aria-live="polite">
          {error}
        </p>
      ) : validationState === "pending" && pendingMessage ? (
        <p
          className="poodle-field__message poodle-field__message--pending"
          id={pendingId ?? undefined}
          aria-live="polite"
        >
          {pendingMessage}
        </p>
      ) : null}
    </div>
  );
}
