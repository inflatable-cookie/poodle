import {
  useEffect,
  useId,
  useRef,
  useState,
  type CSSProperties,
  type KeyboardEvent,
  type ReactNode,
} from "react";
import {
  getFocusableElements,
  modalTransition,
  registerDismissLayer,
  trapFocusKeydown,
  type ModalEvent,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/dialog.css";

import { IconButton } from "./IconButton";
import { ThemePortal } from "./portal";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface DialogProps {
  open?: boolean;
  defaultOpen?: boolean;
  title?: string | null;
  description?: string | null;
  role?: "dialog" | "alertdialog";
  dismissOnEscape?: boolean;
  dismissOnBackdrop?: boolean;
  ariaLabel?: string | null;
  contentClassName?: string;
  contentStyle?: CSSProperties;
  overlayClassName?: string;
  overlayStyle?: CSSProperties;
  showCloseButton?: boolean;
  closeLabel?: string;
  width?: "sm" | "md" | "lg" | "xl" | "full";
  bare?: boolean;
  size?: ControlSize | null;
  closeButtonSize?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  initialFocus?: "auto" | "none" | string;
  onOpenChange?: (open: boolean) => void;
  onRequestClose?: () => void;
  kind?: "dialog" | "alertdialog";
  children?: ReactNode;
  header?: ReactNode;
  footer?: ReactNode;
  actions?: ReactNode;
}

export function Dialog({
  open,
  defaultOpen = false,
  title = null,
  description = null,
  role = "dialog",
  dismissOnEscape = true,
  dismissOnBackdrop = true,
  ariaLabel = null,
  contentClassName = "",
  contentStyle,
  overlayClassName = "",
  overlayStyle,
  showCloseButton = false,
  closeLabel = "Close dialog",
  width = "md",
  bare = false,
  size = null,
  closeButtonSize = null,
  sizeRole = "control",
  density = null,
  initialFocus = "auto",
  onOpenChange,
  onRequestClose,
  kind,
  children,
  header,
  footer,
  actions,
}: DialogProps) {
  const uiPresentation = useUiPresentation();

  const surfaceRef = useRef<HTMLDivElement | null>(null);
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const lastFocusedElement = useRef<HTMLElement | null>(null);
  const bodyOverflow = useRef<string | null>(null);

  const effectiveRole = kind ?? role;
  // A titled dialog takes its accessible name from the rendered title via
  // aria-labelledby; ariaLabel is the fallback only when there is no title.
  // A custom `header` node replaces the title element, so there is nothing to
  // point at — fall back to ariaLabel there too.
  const titleId = useId();
  const labelledBy = !header && title ? titleId : undefined;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedCloseButtonSize = closeButtonSize ?? resolveSemanticControlSize(uiPresentation.sizeScale, "chrome");
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = open !== undefined;
  const isOpen = isControlled ? open === true : uncontrolledOpen;

  // focus save/restore + body scroll lock on open edges. The surface mounts
  // through the theme portal one render after isOpen flips, so initial focus
  // runs from the surface ref callback (pendingFocus) rather than here.
  const pendingFocus = useRef(false);
  useEffect(() => {
    if (isOpen) {
      lastFocusedElement.current = document.activeElement as HTMLElement | null;
      pendingFocus.current = true;
      bodyOverflow.current = document.body.style.overflow;
      document.body.style.overflow = "hidden";
      return () => {
        pendingFocus.current = false;
        if (bodyOverflow.current !== null) {
          document.body.style.overflow = bodyOverflow.current;
          bodyOverflow.current = null;
        }
        lastFocusedElement.current?.focus();
      };
    }
  }, [isOpen]);

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

  /**
   * Resolve where focus lands on the open edge, per the `initialFocus` prop.
   * The already-focused guard runs first (see the surface ref callback), so
   * the active element is outside the surface here.
   *
   * - "none": focus nothing; the surface still traps focus.
   * - a CSS selector string: resolved within the surface; an unmatched
   *   selector falls back to "auto" behaviour rather than throwing.
   * - "auto" (default): first focusable in the content region
   *   (`.poodle-dialog__body`), skipping header chrome such as the close
   *   button; the surface itself when the body has no focusable element (and
   *   always in `bare` mode, where no body region exists).
   */
  function resolveInitialFocus(node: HTMLElement): void {
    if (initialFocus === "none") {
      return;
    }

    if (initialFocus !== "auto") {
      const target = node.querySelector<HTMLElement>(initialFocus);
      if (target) {
        target.focus();
        return;
      }
    }

    const body = node.querySelector<HTMLElement>(".poodle-dialog__body");
    const focusable = getFocusableElements(body ?? node);
    (focusable[0] ?? node).focus();
  }

  useEffect(() => {
    if (!isOpen) return;
    return registerDismissLayer({
      contains: () => true,
      dismissOnOutsideInteract: false,
      onDismiss: () => sendRef.current({ type: "ESCAPE" }),
    });
  }, [isOpen]);

  if (!isOpen) return null;

  const closeButton = (
    <IconButton
      type="button"
      icon="x"
      ariaLabel={closeLabel}
      variant="ghost"
      sizeRole="chrome"
      size={resolvedCloseButtonSize}
      onClick={() => sendRef.current({ type: "REQUEST_CLOSE" })}
    />
  );

  return (
    <ThemePortal>
      <div className="poodle-dialog" data-size={resolvedSize} data-density={resolvedDensity} data-width={width}>
        <button
          type="button"
          className={`poodle-dialog__backdrop ${overlayClassName}`}
          style={overlayStyle}
          aria-label="Dismiss dialog backdrop"
          onClick={() => sendRef.current({ type: "BACKDROP_CLICK" })}
        />
        <div
          ref={(node) => {
            surfaceRef.current = node;
            if (node && pendingFocus.current) {
              pendingFocus.current = false;
              // Already-focused guard (parity with b1a4a5e7 Svelte): never
              // steal focus when something inside the surface is already
              // focused (e.g. a consumer ref that focuses on attach). Runs
              // before any initialFocus resolution.
              if (!node.contains(document.activeElement)) {
                resolveInitialFocus(node);
              }
            }
          }}
          className={`poodle-dialog__surface ${contentClassName}${bare ? " poodle-dialog__surface--bare" : ""}`}
          style={contentStyle}
          role={effectiveRole}
          tabIndex={-1}
          aria-labelledby={labelledBy}
          aria-label={labelledBy ? undefined : (ariaLabel ?? undefined)}
          aria-modal="true"
          onKeyDown={(event: KeyboardEvent) => trapFocusKeydown(surfaceRef.current, event.nativeEvent)}
        >
          {bare ? (
            <>
              {showCloseButton ? <div className="poodle-dialog__close poodle-dialog__close--overlay">{closeButton}</div> : null}
              {children}
            </>
          ) : (
            <>
              {header || title || description || showCloseButton ? (
                <div className="poodle-dialog__header-row">
                  {header ? (
                    <div className="poodle-dialog__header">{header}</div>
                  ) : title || description ? (
                    <div className="poodle-dialog__header">
                      {title ? <strong id={titleId} className="poodle-dialog__title">{title}</strong> : null}
                      {description ? <p>{description}</p> : null}
                    </div>
                  ) : null}
                  {showCloseButton ? <div className="poodle-dialog__close">{closeButton}</div> : null}
                </div>
              ) : null}

              <div className="poodle-dialog__body">{children}</div>

              {footer ? (
                <div className="poodle-dialog__footer">{footer}</div>
              ) : actions ? (
                <div className="poodle-dialog__actions">{actions}</div>
              ) : null}
            </>
          )}
        </div>
      </div>
    </ThemePortal>
  );
}
