import {
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
  type CSSProperties,
  type KeyboardEvent as ReactKeyboardEvent,
  type MouseEvent as ReactMouseEvent,
  type ReactNode,
} from "react";

import { menuTransition, registerDismissLayer, type MenuEvent as MenuMachineEvent } from "@poodle/headless";

import { MenuSurface, type MenuSurfaceHandle } from "./MenuSurface";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, MenuItem, SemanticControlSizeRole } from "./types";

export interface ContextMenuProps {
  items?: MenuItem[];
  open?: boolean | null;
  defaultOpen?: boolean;
  anchorPoint?: { x: number; y: number } | null;
  ariaLabel?: string | null;
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  onOpenChange?: ((open: boolean) => void) | undefined;
  onAction?: ((value: string) => void) | undefined;
  children?: ReactNode;
}

export function ContextMenu({
  items = [],
  open = null,
  defaultOpen = false,
  anchorPoint = null,
  ariaLabel = null,
  sizeRole = "chrome",
  size = null,
  density = null,
  onOpenChange = undefined,
  onAction = undefined,
  children,
}: ContextMenuProps) {
  const uiPresentation = useUiPresentation();

  const surfaceRef = useRef<MenuSurfaceHandle | null>(null);
  const pendingFocus = useRef(false);
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [uncontrolledAnchorPoint, setUncontrolledAnchorPoint] = useState<{ x: number; y: number } | null>(anchorPoint);
  const [adjustedPosition, setAdjustedPosition] = useState<{ left: string; top: string } | null>(null);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = open !== null;
  const isOpen = isControlled ? open === true : uncontrolledOpen;
  const currentAnchorPoint = anchorPoint ?? uncontrolledAnchorPoint;
  const overlayStyle: CSSProperties = adjustedPosition
    ? { left: adjustedPosition.left, top: adjustedPosition.top }
    : currentAnchorPoint
      ? { left: `${currentAnchorPoint.x}px`, top: `${currentAnchorPoint.y}px`, visibility: "hidden" }
      : {};

  // Measure the overlay after the open render, clamp to the viewport, then
  // reveal and focus (measure-then-reveal — same pattern as ListCard).
  useLayoutEffect(() => {
    if (!isOpen || !currentAnchorPoint) return;
    if (adjustedPosition) return;
    const overlay = surfaceRef.current?.element;
    if (!overlay) return;

    const rect = overlay.getBoundingClientRect();
    const vw = window.innerWidth;
    const vh = window.innerHeight;
    const pad = 8;
    let x = currentAnchorPoint.x;
    let y = currentAnchorPoint.y;

    if (x + rect.width > vw - pad) {
      x = Math.max(pad, x - rect.width);
    }

    if (y + rect.height > vh - pad) {
      y = Math.max(pad, vh - rect.height - pad);
    }

    setAdjustedPosition({ left: `${x}px`, top: `${y}px` });
  }, [isOpen, currentAnchorPoint, adjustedPosition]);

  useEffect(() => {
    if (!isOpen || !adjustedPosition) return;
    if (!pendingFocus.current) return;
    pendingFocus.current = false;
    surfaceRef.current?.focusFirstItem();
  }, [isOpen, adjustedPosition]);

  function send(event: MenuMachineEvent): void {
    const result = menuTransition(isOpen ? "open" : "closed", {}, event);

    for (const effect of result.effects) {
      if (effect.type === "emitOpenChange") {
        if (!isControlled) {
          setUncontrolledOpen(effect.open);
        }
        if (effect.open) {
          setAdjustedPosition(null);
          pendingFocus.current = true;
        }

        onOpenChange?.(effect.open);
      } else if (effect.type === "emitAction") {
        onAction?.(effect.value);
      }
    }
  }

  function handleContextMenu(event: ReactMouseEvent): void {
    event.preventDefault();
    setUncontrolledAnchorPoint({ x: event.clientX, y: event.clientY });
    setAdjustedPosition(null);
    pendingFocus.current = true;
    send({ type: "OPEN" });
  }

  function handleTriggerKeydown(event: ReactKeyboardEvent): void {
    if (event.key !== "ContextMenu" && !(event.shiftKey && event.key === "F10")) {
      return;
    }

    event.preventDefault();
    const target = event.currentTarget;
    if (!(target instanceof HTMLElement)) {
      return;
    }

    const rect = target.getBoundingClientRect();
    setUncontrolledAnchorPoint({ x: rect.left + 16, y: rect.top + 16 });
    setAdjustedPosition(null);
    pendingFocus.current = true;
    send({ type: "OPEN" });
  }

  useEffect(() => {
    if (!isOpen) {
      return;
    }

    return registerDismissLayer({
      // Only the overlay itself counts as inside; clicking the trigger area
      // closes, matching the previous document-listener behavior.
      contains: (target) => surfaceRef.current?.element?.contains(target) ?? false,
      dismissOnOutsideInteract: true,
      onDismiss: (reason) => send(reason === "escape" ? { type: "ESCAPE" } : { type: "OUTSIDE_INTERACT" }),
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isOpen]);

  return (
    <div
      className="poodle-context-menu"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      role="button"
      tabIndex={0}
      aria-haspopup="menu"
      onContextMenu={handleContextMenu}
      onKeyDown={handleTriggerKeydown}
    >
      {children}

      {isOpen && currentAnchorPoint ? (
        <MenuSurface
          ref={surfaceRef}
          items={items}
          ariaLabel={ariaLabel}
          size={resolvedSize}
          density={resolvedDensity}
          overlayStyle={overlayStyle}
          onAction={(value) => send({ type: "ACTION", value })}
        />
      ) : null}
    </div>
  );
}
