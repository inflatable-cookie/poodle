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
import {
  layerContains,
  menuTransition,
  registerDismissLayer,
  type MenuEvent as MenuMachineEvent,
  type OverlaySurfaceGeometryChangeHandler,
} from "@poodle/headless";

import "@poodle/styles/menu.css";

import { MenuSurface, type MenuSurfaceHandle } from "./MenuSurface";
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
  onSurfaceGeometryChange?: OverlaySurfaceGeometryChangeHandler;
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
  onSurfaceGeometryChange,
  trigger,
}: MenuProps) {
  const uiPresentation = useUiPresentation();

  const rootRef = useRef<HTMLDivElement | null>(null);
  // The trigger is state, not a ref: the portalled surface has to re-render
  // once it exists so it can be positioned against it.
  const [triggerElement, setTriggerElement] = useState<HTMLDivElement | null>(null);
  const surfaceRef = useRef<MenuSurfaceHandle | null>(null);
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);

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

  useLayoutEffect(() => {
    if (!isOpen) return;
    surfaceRef.current?.focusFirstItem();
  }, [isOpen]);

  useEffect(() => {
    if (!isOpen) return;
    return registerDismissLayer({
      // The surface is portalled out of the root, so both are "inside".
      contains: (target) =>
        layerContains(target as Node, rootRef.current, surfaceRef.current?.element),
      dismissOnOutsideInteract: true,
      onDismiss: (reason) => sendRef.current(reason === "escape" ? { type: "ESCAPE" } : { type: "OUTSIDE_INTERACT" }),
    });
  }, [isOpen]);

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
        ref={setTriggerElement}
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
          anchor={triggerElement}
          placement={placement}
          onSurfaceGeometryChange={onSurfaceGeometryChange}
          onAction={(value) => sendRef.current({ type: "ACTION", value })}
        />
      ) : null}
    </div>
  );
}
