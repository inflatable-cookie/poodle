import {
  useCallback,
  useEffect,
  useId,
  useLayoutEffect,
  useRef,
  useState,
  type CSSProperties,
  type FocusEvent,
  type PointerEvent,
  type ReactNode,
} from "react";
import { hoverTransition, type HoverEvent as HoverMachineEvent, type HoverState } from "@poodle/headless";

import "@poodle/styles/tooltip.css";

import { AnchoredSurface } from "./AnchoredSurface";
import type { OverlayPlacement } from "./types";

export interface TooltipProps {
  content: string;
  open?: boolean | null;
  defaultOpen?: boolean;
  delayMs?: number;
  placement?: OverlayPlacement;
  onOpenChange?: (open: boolean) => void;
  children?: ReactNode;
}

export function Tooltip({
  content,
  open = null,
  defaultOpen = false,
  delayMs = 300,
  placement = "top",
  onOpenChange,
  children,
}: TooltipProps) {
  const tooltipId = useId();
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [resolvedPlacement, setResolvedPlacement] = useState<OverlayPlacement>(placement);

  const rootRef = useRef<HTMLSpanElement | null>(null);
  const triggerRef = useRef<HTMLElement | null>(null);
  const bubbleRef = useRef<HTMLElement | null>(null);
  // The anchor is state, not just a ref: the portalled bubble has to
  // re-render once the hovered element is known so it can be placed.
  const [anchorElement, setAnchorElement] = useState<HTMLElement | null>(null);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const machineState = useRef<HoverState>("closed");

  const isControlled = open !== null;
  const isOpen = isControlled ? open === true : uncontrolledOpen;

  const clearTimer = useCallback(() => {
    if (timerRef.current) {
      clearTimeout(timerRef.current);
      timerRef.current = null;
    }
  }, []);

  const send = useCallback(
    (event: HoverMachineEvent) => {
      const result = hoverTransition(machineState.current, { openDelayMs: delayMs, closeDelayMs: 0 }, event);
      machineState.current = result.state;
      for (const effect of result.effects) {
        if (effect.type === "clearTimer") {
          clearTimer();
        } else if (effect.type === "startTimer") {
          clearTimer();
          timerRef.current = setTimeout(() => send({ type: "TIMER_FIRE" }), effect.ms);
        } else if (effect.type === "emitOpenChange") {
          if (!effect.open && triggerRef.current) {
            triggerRef.current.removeAttribute("aria-describedby");
          }
          if (!isControlled) setUncontrolledOpen(effect.open);
          onOpenChange?.(effect.open);
        }
      }
    },
    [clearTimer, delayMs, isControlled, onOpenChange],
  );

  function getDefaultAnchor(): HTMLElement | null {
    const root = rootRef.current;
    if (!root) return null;
    return root.firstElementChild instanceof HTMLElement ? root.firstElementChild : null;
  }

  function resolveAnchor(target: EventTarget | null): HTMLElement | null {
    const root = rootRef.current;
    if (!root) return null;
    if (!(target instanceof HTMLElement)) return getDefaultAnchor();
    if (target === root) return getDefaultAnchor();
    return root.contains(target) ? target : getDefaultAnchor();
  }

  function anchorAndOpen(target: EventTarget | null): void {
    const anchor = resolveAnchor(target);
    if (!anchor) return;
    if (triggerRef.current && triggerRef.current !== anchor) {
      triggerRef.current.removeAttribute("aria-describedby");
    }
    triggerRef.current = anchor;
    setAnchorElement(anchor);
    send({ type: "ENTER" });
  }

  function dismissUnlessWithin(relatedTarget: EventTarget | null): void {
    const root = rootRef.current;
    if (root && relatedTarget instanceof Node && root.contains(relatedTarget)) return;
    send({ type: "DISMISS" });
  }

  useLayoutEffect(() => {
    if (!isOpen) return;
    // Announced only while shown: a stale describedby outlives the bubble.
    triggerRef.current?.setAttribute("aria-describedby", tooltipId);
  }, [isOpen, tooltipId]);

  useEffect(
    () => () => {
      clearTimer();
      triggerRef.current?.removeAttribute("aria-describedby");
    },
    [clearTimer],
  );

  return (
    <span
      ref={rootRef}
      className="poodle-tooltip"
      role="presentation"
      // React's onPointerEnter synthesizes from bubbling pointerover, so
      // event.target is the deepest element — Svelte's non-bubbling
      // pointerenter always saw the root. Anchor to the direct child.
      onPointerEnter={() => anchorAndOpen(rootRef.current)}
      onPointerLeave={(event: PointerEvent) => dismissUnlessWithin(event.relatedTarget)}
      onFocus={(event: FocusEvent) => anchorAndOpen(event.target)}
      onBlur={(event: FocusEvent) => dismissUnlessWithin(event.relatedTarget)}
      onKeyDown={(event) => {
        if (event.key === "Escape") send({ type: "DISMISS" });
      }}
    >
      {children}

      {isOpen ? (
        <AnchoredSurface
          id={tooltipId}
          ref={bubbleRef}
          tag="span"
          anchor={anchorElement}
          placement={placement}
          onPlacement={setResolvedPlacement}
          className="poodle-tooltip__bubble"
          data-placement={resolvedPlacement}
          role="tooltip"
        >
          {content}
        </AnchoredSurface>
      ) : null}
    </span>
  );
}
