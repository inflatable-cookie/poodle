import {
  useEffect,
  useMemo,
  useRef,
  useState,
  type KeyboardEvent as ReactKeyboardEvent,
  type MouseEvent as ReactMouseEvent,
  type ReactNode,
} from "react";

import {
  menuTransition,
  pointAnchor,
  registerDismissLayer,
  type MenuEvent as MenuMachineEvent,
} from "@inflatable-cookie/poodle-core";

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
  const rootRef = useRef<HTMLDivElement | null>(null);
  const pendingFocus = useRef(false);
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [uncontrolledAnchorPoint, setUncontrolledAnchorPoint] = useState<{ x: number; y: number } | null>(anchorPoint);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = open !== null;
  const isOpen = isControlled ? open === true : uncontrolledOpen;
  const currentAnchorPoint = anchorPoint ?? uncontrolledAnchorPoint;
  // A right-click has no element behind it, so the menu anchors to the point
  // itself; the shared resolver then does the edge-flipping that used to be
  // hand-rolled here.
  const anchor = useMemo(
    () =>
      currentAnchorPoint
        ? pointAnchor(currentAnchorPoint.x, currentAnchorPoint.y, rootRef.current)
        : null,
    [currentAnchorPoint],
  );

  useEffect(() => {
    if (!isOpen) return;
    if (!pendingFocus.current) return;
    pendingFocus.current = false;
    surfaceRef.current?.focusFirstItem();
  }, [isOpen]);

  function send(event: MenuMachineEvent): void {
    const result = menuTransition(isOpen ? "open" : "closed", {}, event);

    for (const effect of result.effects) {
      if (effect.type === "emitOpenChange") {
        if (!isControlled) {
          setUncontrolledOpen(effect.open);
        }
        if (effect.open) {
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
      ref={rootRef}
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
          anchor={anchor}
          placement="bottom-start"
          offset={0}
          onAction={(value) => send({ type: "ACTION", value })}
        />
      ) : null}
    </div>
  );
}
