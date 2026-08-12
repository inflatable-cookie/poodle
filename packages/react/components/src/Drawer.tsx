import { useEffect, useId, useRef, useState, type KeyboardEvent, type ReactNode } from "react";
import {
  getFocusableElements,
  modalTransition,
  registerDismissLayer,
  trapFocusKeydown,
  type ModalEvent,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/drawer.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, DrawerEdge, SemanticControlSizeRole } from "./types";

export interface DrawerProps {
  open?: boolean;
  defaultOpen?: boolean;
  edge?: DrawerEdge;
  modal?: boolean;
  title?: string | null;
  description?: string | null;
  dismissOnEscape?: boolean;
  dismissOnBackdrop?: boolean;
  dismissOnOutsideInteract?: boolean;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onOpenChange?: (open: boolean) => void;
  onRequestClose?: () => void;
  children?: ReactNode;
  actions?: ReactNode;
}

export function Drawer({
  open,
  defaultOpen = false,
  edge = "right",
  modal = true,
  title = null,
  description = null,
  dismissOnEscape = true,
  dismissOnBackdrop = true,
  dismissOnOutsideInteract = false,
  ariaLabel = null,
  size = null,
  sizeRole = "control",
  density = null,
  onOpenChange,
  onRequestClose,
  children,
  actions,
}: DrawerProps) {
  const uiPresentation = useUiPresentation();

  const surfaceRef = useRef<HTMLDivElement | null>(null);
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const lastFocusedElement = useRef<HTMLElement | null>(null);
  const bodyOverflow = useRef<string | null>(null);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  // Titled drawers take their accessible name from the rendered title via
  // aria-labelledby; ariaLabel is the fallback only when there is no title.
  const titleId = useId();
  const labelledBy = title ? titleId : undefined;
  const isControlled = open !== undefined;
  const isOpen = isControlled ? open === true : uncontrolledOpen;

  useEffect(() => {
    if (isOpen) {
      lastFocusedElement.current = document.activeElement as HTMLElement | null;
      const focusable = getFocusableElements(surfaceRef.current);
      (focusable[0] ?? surfaceRef.current)?.focus();
      return () => {
        lastFocusedElement.current?.focus();
      };
    }
  }, [isOpen]);

  useEffect(() => {
    if (isOpen && modal) {
      if (bodyOverflow.current === null) {
        bodyOverflow.current = document.body.style.overflow;
        document.body.style.overflow = "hidden";
      }
      return () => {
        if (bodyOverflow.current !== null) {
          document.body.style.overflow = bodyOverflow.current;
          bodyOverflow.current = null;
        }
      };
    }
  }, [isOpen, modal]);

  const sendRef = useRef<(event: ModalEvent) => void>(() => {});
  sendRef.current = (event: ModalEvent) => {
    const result = modalTransition(isOpen ? "open" : "closed", { dismissOnEscape, dismissOnBackdrop }, event);
    for (const effect of result.effects) {
      if (effect.type === "emitRequestClose") {
        onRequestClose?.();
      } else if (effect.type === "emitOpenChange") {
        if (!isControlled) setUncontrolledOpen(effect.open);
        onOpenChange?.(effect.open);
      }
    }
  };

  useEffect(() => {
    if (!isOpen) return;
    return registerDismissLayer({
      // The surface is the inside of the layer; the backdrop and the rest of
      // the page count as outside so the boolean stays meaningful for a
      // consumer that opts into outside dismissal. Defaults off: a modal that
      // vanishes on an outside click loses work (backdrop click stays the
      // modal's own dismissal path, guarded by `dismissOnBackdrop`).
      contains: (target) => surfaceRef.current?.contains(target as Node) ?? false,
      dismissOnOutsideInteract,
      onDismiss: () => sendRef.current({ type: "ESCAPE" }),
    });
  }, [isOpen, dismissOnOutsideInteract]);

  if (!isOpen) return null;

  return (
    <div className="poodle-drawer" data-edge={edge} data-modal={modal} data-size={resolvedSize} data-density={resolvedDensity}>
      {modal ? (
        <button
          type="button"
          className="poodle-drawer__backdrop"
          aria-label="Dismiss drawer backdrop"
          onClick={() => sendRef.current({ type: "BACKDROP_CLICK" })}
        />
      ) : null}

      <div
        ref={surfaceRef}
        className="poodle-drawer__surface"
        role="dialog"
        tabIndex={-1}
        aria-modal={modal ? "true" : undefined}
        aria-labelledby={labelledBy}
        aria-label={labelledBy ? undefined : (ariaLabel ?? undefined)}
        onKeyDown={(event: KeyboardEvent) => {
          if (modal) trapFocusKeydown(surfaceRef.current, event.nativeEvent);
        }}
      >
        {title || description ? (
          <div className="poodle-drawer__header">
            {title ? <strong id={titleId}>{title}</strong> : null}
            {description ? <p>{description}</p> : null}
          </div>
        ) : null}

        <div className="poodle-drawer__body">{children}</div>

        {actions ? <div className="poodle-drawer__actions">{actions}</div> : null}
      </div>
    </div>
  );
}
