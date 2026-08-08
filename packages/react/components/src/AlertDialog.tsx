import { useState, type ReactNode } from "react";

import "@inflatable-cookie/poodle-styles/alert-dialog.css";

import { Button } from "./Button";
import { Dialog } from "./Dialog";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { AlertDialogTone, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface AlertDialogProps {
  open?: boolean;
  title: string;
  description?: string | null;
  itemLabel?: string | null;
  itemValue?: string | null;
  tone?: AlertDialogTone;
  confirmLabel?: string;
  cancelLabel?: string;
  ariaLabel?: string | null;
  workingLabel?: string;
  onConfirm?: (() => void | Promise<void>) | null;
  onCancel?: (() => void) | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onOpenChange?: (open: boolean) => void;
  children?: ReactNode;
}

export function AlertDialog({
  open,
  title,
  description = null,
  itemLabel = null,
  itemValue = null,
  tone = "danger",
  confirmLabel = "Confirm",
  cancelLabel = "Cancel",
  ariaLabel = null,
  workingLabel = "Working…",
  onConfirm = null,
  onCancel = null,
  size = null,
  sizeRole = "control",
  density = null,
  onOpenChange,
  children,
}: AlertDialogProps) {
  const uiPresentation = useUiPresentation();

  const [working, setWorking] = useState(false);
  const [uncontrolledOpen, setUncontrolledOpen] = useState(false);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const confirmTone = tone === "danger" ? ("danger" as const) : ("default" as const);
  const isControlled = open !== undefined;
  const isOpen = isControlled ? open === true : uncontrolledOpen;

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) setUncontrolledOpen(nextOpen);
    onOpenChange?.(nextOpen);
  }

  async function handleConfirm(): Promise<void> {
    if (working) return;
    setWorking(true);
    try {
      if (onConfirm) await onConfirm();
      setOpen(false);
    } catch {
      // Keep the dialog open so the caller can recover or retry.
    } finally {
      setWorking(false);
    }
  }

  function handleCancel(): void {
    onCancel?.();
    setOpen(false);
  }

  return (
    <div data-size={resolvedSize} data-density={resolvedDensity}>
      <Dialog
        open={isOpen}
        title={title}
        description={description}
        role="alertdialog"
        width="sm"
        ariaLabel={ariaLabel}
        size={resolvedSize}
        density={resolvedDensity}
        dismissOnEscape={!working}
        dismissOnBackdrop={!working}
        showCloseButton={!working}
        onOpenChange={(nextOpen) => {
          if (!nextOpen && !working) onCancel?.();
          setOpen(nextOpen);
        }}
        actions={
          <>
            <Button variant="ghost" size={resolvedSize} density={resolvedDensity} onClick={handleCancel} disabled={working}>
              {cancelLabel}
            </Button>
            <Button
              variant="primary"
              tone={confirmTone}
              size={resolvedSize}
              density={resolvedDensity}
              onClick={handleConfirm}
              disabled={working}
            >
              {working ? workingLabel : confirmLabel}
            </Button>
          </>
        }
      >
        {itemLabel && itemValue ? (
          <p className="poodle-alert-dialog__item-detail">
            <strong>{itemLabel}:</strong> {itemValue}
          </p>
        ) : null}
        {children}
      </Dialog>
    </div>
  );
}
