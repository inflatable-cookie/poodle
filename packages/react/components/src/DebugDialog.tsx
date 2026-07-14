import { useState } from "react";

import { Button } from "./Button";
import { Code } from "./Code";
import { Dialog } from "./Dialog";
import type { ButtonVariant, ControlSize } from "./types";

export interface DebugDialogProps {
  value?: unknown | null;
  title?: string;
  triggerLabel?: string;
  maxHeight?: string;
  triggerVariant?: ButtonVariant;
  triggerSize?: ControlSize | null;
  showCloseButton?: boolean;
  closeLabel?: string;
}

function stringifyValue(input: unknown): string {
  if (input === null || input === undefined) {
    return "";
  }

  try {
    return JSON.stringify(input, null, 2);
  } catch {
    return String(input);
  }
}

export function DebugDialog({
  value = null,
  title = "Debug data",
  triggerLabel = "View debug data",
  maxHeight = "min(60vh, 32rem)",
  triggerVariant = "ghost",
  triggerSize = "sm",
  showCloseButton = true,
  closeLabel = "Close debug dialog",
}: DebugDialogProps) {
  const [open, setOpen] = useState(false);

  const hasValue = value !== null && value !== undefined;
  const source = stringifyValue(value);

  if (!hasValue) {
    return null;
  }

  return (
    <>
      <Button type="button" variant={triggerVariant} size={triggerSize} onClick={() => setOpen(true)}>
        {triggerLabel}
      </Button>

      <Dialog
        open={open}
        title={title}
        width="lg"
        showCloseButton={showCloseButton}
        closeLabel={closeLabel}
        onOpenChange={setOpen}
      >
        <Code source={source} language="json" maxHeight={maxHeight} />
      </Dialog>
    </>
  );
}
