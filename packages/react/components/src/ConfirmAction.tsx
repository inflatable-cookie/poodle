import { useState, type ReactNode } from "react";

import { AlertDialog } from "./AlertDialog";
import { Button } from "./Button";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { AlertDialogTone, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface ConfirmActionProps {
  title: string;
  description?: string | null;
  tone?: AlertDialogTone;
  triggerLabel?: string;
  confirmLabel?: string;
  cancelLabel?: string;
  onConfirm?: (() => void | Promise<void>) | null;
  onCancel?: (() => void) | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  trigger?: ReactNode;
  children?: ReactNode;
}

export function ConfirmAction({
  title,
  description = null,
  tone = "danger",
  triggerLabel = "Delete",
  confirmLabel = "Confirm",
  cancelLabel = "Cancel",
  onConfirm = null,
  onCancel = null,
  size = null,
  sizeRole = "control",
  density = null,
  trigger,
  children,
}: ConfirmActionProps) {
  const uiPresentation = useUiPresentation();

  const [open, setOpen] = useState(false);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const triggerTone = tone === "danger" ? ("danger" as const) : ("default" as const);

  async function handleConfirm(): Promise<void> {
    await onConfirm?.();
    setOpen(false);
  }

  function handleCancel(): void {
    onCancel?.();
    setOpen(false);
  }

  return (
    <>
      {trigger ? (
        <span
          className="poodle-confirm-action__trigger"
          data-size={resolvedSize}
          data-density={resolvedDensity}
          role="presentation"
          onClick={() => setOpen(true)}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              setOpen(true);
            }
          }}
        >
          {trigger}
        </span>
      ) : (
        <Button
          variant="secondary"
          tone={triggerTone}
          size={resolvedSize}
          density={resolvedDensity}
          onClick={() => setOpen(true)}
        >
          {triggerLabel}
        </Button>
      )}

      <AlertDialog
        open={open}
        title={title}
        description={description}
        tone={tone}
        confirmLabel={confirmLabel}
        cancelLabel={cancelLabel}
        size={resolvedSize}
        density={resolvedDensity}
        onConfirm={() => void handleConfirm()}
        onCancel={handleCancel}
        onOpenChange={setOpen}
      >
        {children}
      </AlertDialog>
    </>
  );
}
