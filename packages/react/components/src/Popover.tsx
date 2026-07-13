import { useEffect, useRef, useState, type CSSProperties, type ReactNode } from "react";
import {
  createInstanceId,
  getFocusableElements,
  popoverParts,
  popoverTransition,
  registerDismissLayer,
  type PopoverContext,
  type PopoverEvent,
} from "@poodle/headless";

import "@poodle/styles/popover.css";

import { reactifyPart } from "./parts";
import type { OverlayPlacement, PopoverInitialFocus } from "./types";

export interface PopoverProps {
  open?: boolean | null;
  defaultOpen?: boolean;
  placement?: OverlayPlacement;
  offset?: number;
  dismissOnOutsideInteract?: boolean;
  initialFocus?: PopoverInitialFocus;
  ariaLabel?: string | null;
  block?: boolean;
  disabled?: boolean;
  surfaceWidth?: "content" | "trigger";
  surfaceMinWidth?: string | null;
  surfaceMaxWidth?: string | null;
  onOpenChange?: (open: boolean) => void;
  trigger?: ReactNode;
  children?: ReactNode;
}

export function Popover({
  open = null,
  defaultOpen = false,
  placement = "bottom-start",
  offset = 8,
  dismissOnOutsideInteract = true,
  initialFocus = "first-focusable",
  ariaLabel = null,
  block = false,
  disabled = false,
  surfaceWidth = "content",
  surfaceMinWidth = null,
  surfaceMaxWidth = null,
  onOpenChange,
  trigger,
  children,
}: PopoverProps) {
  const popoverId = useRef<string | null>(null);
  if (popoverId.current === null) popoverId.current = createInstanceId("popover");

  const rootRef = useRef<HTMLDivElement | null>(null);
  const triggerRef = useRef<HTMLDivElement | null>(null);
  const surfaceRef = useRef<HTMLDivElement | null>(null);
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const previousOpen = useRef(false);

  const isControlled = open !== null;
  const isOpen = isControlled ? open === true : uncontrolledOpen;

  const machineContext: PopoverContext = { disabled, dismissOnOutsideInteract, initialFocus };

  // initial focus on open (after the surface renders)
  useEffect(() => {
    if (isOpen && !previousOpen.current && surfaceRef.current) {
      if (initialFocus === "content") {
        surfaceRef.current.focus();
      } else if (initialFocus === "first-focusable") {
        getFocusableElements(surfaceRef.current)[0]?.focus();
      }
    }
    previousOpen.current = isOpen;
  }, [isOpen, initialFocus]);

  const sendRef = useRef<(event: PopoverEvent) => void>(() => {});
  sendRef.current = (event: PopoverEvent) => {
    const result = popoverTransition(isOpen ? "open" : "closed", machineContext, event);
    for (const effect of result.effects) {
      switch (effect.type) {
        case "emitOpenChange":
          if (!isControlled) setUncontrolledOpen(effect.open);
          onOpenChange?.(effect.open);
          break;
        case "restoreTriggerFocus":
          triggerRef.current?.focus();
          break;
        case "focusOnOpen":
          // handled by the isOpen effect above
          break;
      }
    }
  };

  useEffect(() => {
    if (!isOpen) return;
    return registerDismissLayer({
      contains: (target) => rootRef.current?.contains(target as Node) ?? false,
      dismissOnOutsideInteract,
      onDismiss: (reason) =>
        sendRef.current(reason === "escape" ? { type: "ESCAPE" } : { type: "OUTSIDE_INTERACT" }),
    });
  }, [isOpen, dismissOnOutsideInteract]);

  const parts = popoverParts(isOpen ? "open" : "closed", machineContext, {
    surfaceId: popoverId.current,
    ariaLabel,
    block,
    placement,
    surfaceWidth,
  });

  const surfaceStyle: CSSProperties & Record<string, string> = {
    "--poodle-popover-offset": `${offset}px`,
    ...(surfaceMinWidth ? { "--poodle-popover-surface-min-width": surfaceMinWidth } : null),
    ...(surfaceMaxWidth ? { "--poodle-popover-surface-max-width": surfaceMaxWidth } : null),
  };

  return (
    <div {...reactifyPart(parts.root)} className="poodle-popover" ref={rootRef}>
      <div
        ref={triggerRef}
        {...reactifyPart(parts.trigger)}
        className="poodle-popover__trigger"
        onClick={() => sendRef.current({ type: "TOGGLE" })}
        onKeyDown={(event) => {
          if (disabled) return;
          if (event.key === "Enter" || event.key === " ") {
            event.preventDefault();
            sendRef.current({ type: "TOGGLE" });
          }
        }}
      >
        {trigger}
      </div>

      {isOpen ? (
        <div ref={surfaceRef} {...reactifyPart(parts.surface)} className="poodle-popover__surface" style={surfaceStyle}>
          {children}
        </div>
      ) : null}
    </div>
  );
}
