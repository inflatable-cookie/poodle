import {
  useCallback,
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
  type CSSProperties,
  type KeyboardEvent,
  type MouseEvent,
  type ReactNode,
} from "react";
import { menuTransition, registerDismissLayer, type MenuEvent as MenuMachineEvent } from "@poodle/headless";

import "@poodle/styles/menu.css";

import { MenuSurface, type MenuSurfaceHandle } from "./MenuSurface";
import { resolveOverlayPosition } from "./overlay-position";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, MenuItem, OverlayPlacement, SemanticControlSizeRole } from "./types";

export interface MenuProps {
  items?: MenuItem[];
  open?: boolean | null;
  defaultOpen?: boolean;
  placement?: OverlayPlacement;
  ariaLabel?: string | null;
  triggerAriaLabel?: string | null;
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  onOpenChange?: (open: boolean) => void;
  onAction?: (value: string) => void;
  trigger?: ReactNode;
}

export function Menu({
  items = [],
  open = null,
  defaultOpen = false,
  placement = "bottom-start",
  ariaLabel = null,
  triggerAriaLabel = null,
  sizeRole = "chrome",
  size = null,
  density = null,
  onOpenChange,
  onAction,
  trigger,
}: MenuProps) {
  const uiPresentation = useUiPresentation();

  const rootRef = useRef<HTMLDivElement | null>(null);
  const triggerRef = useRef<HTMLDivElement | null>(null);
  const surfaceRef = useRef<MenuSurfaceHandle | null>(null);
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [resolvedPlacement, setResolvedPlacement] = useState<OverlayPlacement>(placement);
  const [overlayStyle, setOverlayStyle] = useState<CSSProperties>({});

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = open !== null;
  const isOpen = isControlled ? open === true : uncontrolledOpen;

  const sendRef = useRef<(event: MenuMachineEvent) => void>(() => {});
  sendRef.current = (event: MenuMachineEvent) => {
    const result = menuTransition(isOpen ? "open" : "closed", {}, event);
    for (const effect of result.effects) {
      if (effect.type === "emitOpenChange") {
        if (!isControlled) setUncontrolledOpen(effect.open);
        onOpenChange?.(effect.open);
      } else if (effect.type === "emitAction") {
        onAction?.(effect.value);
      }
      // focusFirstItem intent runs after positioning, in the isOpen effect.
    }
  };

  const updateOverlayPosition = useCallback(() => {
    const triggerEl = triggerRef.current;
    const overlayEl = surfaceRef.current?.element ?? null;
    if (!triggerEl || !overlayEl) return;
    const next = resolveOverlayPosition(triggerEl.getBoundingClientRect(), overlayEl.getBoundingClientRect(), placement);
    setResolvedPlacement((prev) => (prev === next.placement ? prev : next.placement));
    setOverlayStyle((prev) => {
      const top = `${next.top}px`;
      const left = `${next.left}px`;
      return prev.top === top && prev.left === left ? prev : { top, left };
    });
  }, [placement]);

  useLayoutEffect(() => {
    if (!isOpen) return;
    updateOverlayPosition();
    surfaceRef.current?.focusFirstItem();
  }, [isOpen, updateOverlayPosition]);

  useEffect(() => {
    if (!isOpen) return;
    return registerDismissLayer({
      contains: (target) => rootRef.current?.contains(target as Node) ?? false,
      dismissOnOutsideInteract: true,
      onDismiss: (reason) => sendRef.current(reason === "escape" ? { type: "ESCAPE" } : { type: "OUTSIDE_INTERACT" }),
    });
  }, [isOpen]);

  useEffect(() => {
    const onViewportChange = () => {
      if (surfaceRef.current?.element) updateOverlayPosition();
    };
    window.addEventListener("resize", onViewportChange);
    window.addEventListener("scroll", onViewportChange, true);
    return () => {
      window.removeEventListener("resize", onViewportChange);
      window.removeEventListener("scroll", onViewportChange, true);
    };
  }, [updateOverlayPosition]);

  function handleTriggerClick(event: MouseEvent): void {
    event.preventDefault();
    event.stopPropagation();
    sendRef.current({ type: "TOGGLE" });
  }

  function handleTriggerKeydown(event: KeyboardEvent): void {
    if (event.key === "Enter" || event.key === " " || event.key === "ArrowDown") {
      event.preventDefault();
      event.stopPropagation();
      sendRef.current({ type: "OPEN" });
    }
  }

  return (
    <div className="poodle-menu" ref={rootRef} data-size={resolvedSize} data-density={resolvedDensity}>
      <div
        ref={triggerRef}
        className="poodle-menu__trigger"
        role="button"
        tabIndex={0}
        aria-expanded={isOpen}
        aria-label={triggerAriaLabel ?? undefined}
        onClick={handleTriggerClick}
        onKeyDown={handleTriggerKeydown}
      >
        {trigger}
      </div>

      {isOpen ? (
        <MenuSurface
          ref={surfaceRef}
          items={items}
          ariaLabel={ariaLabel}
          size={resolvedSize}
          density={resolvedDensity}
          placement={resolvedPlacement}
          overlayStyle={overlayStyle}
          onAction={(value) => sendRef.current({ type: "ACTION", value })}
        />
      ) : null}
    </div>
  );
}
