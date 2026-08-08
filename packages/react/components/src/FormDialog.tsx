import "@inflatable-cookie/poodle-core/styles/form-dialog.css";

import { useState, type CSSProperties, type ReactNode } from "react";

import { Button } from "./Button";
import { Dialog } from "./Dialog";
import { FormLayout } from "./FormLayout";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface FormDialogProps {
  open?: boolean | null | undefined;
  title: string;
  subtitle?: string | null;
  description?: string | null;
  submitLabel?: string;
  cancelLabel?: string;
  submitting?: boolean;
  error?: string | null;
  success?: string | null;
  ariaLabel?: string | null;
  width?: string | null;
  columns?: number;
  showDefaultActions?: boolean;
  bare?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onSubmit?: (() => void) | undefined;
  onCancel?: (() => void) | undefined;
  onOpenChange?: ((open: boolean) => void) | undefined;
  children?: ReactNode;
  body?: (submitting: boolean) => ReactNode;
  actions?: (submitting: boolean) => ReactNode;
  subtitleContent?: (submitting: boolean) => ReactNode;
}

export function FormDialog({
  open,
  title,
  subtitle = null,
  description = null,
  submitLabel = "Submit",
  cancelLabel = "Cancel",
  submitting = false,
  error = null,
  success = null,
  ariaLabel = null,
  width = null,
  columns = 6,
  showDefaultActions = true,
  bare = false,
  size = null,
  sizeRole = "control",
  density = null,
  onSubmit = undefined,
  onCancel = undefined,
  onOpenChange = undefined,
  children,
  body,
  actions: actionContent,
  subtitleContent,
}: FormDialogProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledOpen, setUncontrolledOpen] = useState(false);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const resolvedShowActions = bare ? false : showDefaultActions;
  const resolvedDescription = subtitle ?? description;
  const contentStyle: CSSProperties | undefined = width
    ? ({ "--poodle-form-dialog-width": width } as CSSProperties)
    : undefined;
  const contentClassName = width ? "form-dialog__surface" : undefined;
  const isControlled = open !== undefined;
  const isOpen = isControlled ? open === true : uncontrolledOpen;

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) {
      setUncontrolledOpen(nextOpen);
    }

    onOpenChange?.(nextOpen);
  }

  function handleCancel(): void {
    onCancel?.();
    setOpen(false);
  }

  function handleDialogOpenChange(nextOpen: boolean): void {
    if (!nextOpen && !submitting) {
      onCancel?.();
    }

    setOpen(nextOpen);
  }

  const bodyContent = body ? body(submitting) : children;

  return (
    <Dialog
      open={isOpen}
      title={title}
      description={subtitleContent ? null : resolvedDescription}
      role="dialog"
      ariaLabel={ariaLabel}
      size={resolvedSize}
      density={resolvedDensity}
      dismissOnEscape={!submitting}
      dismissOnBackdrop={!submitting}
      showCloseButton={true}
      contentClassName={contentClassName}
      contentStyle={contentStyle}
      onOpenChange={handleDialogOpenChange}
      header={
        subtitleContent ? (
          <div className="poodle-form-dialog__header">
            <div className="poodle-form-dialog__subtitle">{subtitleContent(submitting)}</div>
          </div>
        ) : undefined
      }
      actions={
        actionContent ? (
          <div className="poodle-form-dialog__custom-actions">{actionContent(submitting)}</div>
        ) : resolvedShowActions ? (
          <>
            <Button
              variant="ghost"
              size={resolvedSize}
              density={resolvedDensity}
              onClick={handleCancel}
              disabled={submitting}
            >
              {cancelLabel}
            </Button>
            <Button
              variant="primary"
              size={resolvedSize}
              density={resolvedDensity}
              onClick={() => onSubmit?.()}
              disabled={submitting}
            >
              {submitting ? "Submitting..." : submitLabel}
            </Button>
          </>
        ) : undefined
      }
    >
      {bare ? (
        bodyContent
      ) : (
        <FormLayout error={error} success={success} columns={columns}>
          {bodyContent}
        </FormLayout>
      )}
    </Dialog>
  );
}
