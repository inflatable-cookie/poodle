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

import { resolveOverlayPosition } from "./overlay-position";
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
  const [bubbleStyle, setBubbleStyle] = useState<CSSProperties>({});

  const rootRef = useRef<HTMLSpanElement | null>(null);
  const triggerRef = useRef<HTMLElement | null>(null);
  const bubbleRef = useRef<HTMLSpanElement | null>(null);
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
    send({ type: "ENTER" });
  }

  function dismissUnlessWithin(relatedTarget: EventTarget | null): void {
    const root = rootRef.current;
    if (root && relatedTarget instanceof Node && root.contains(relatedTarget)) return;
    send({ type: "DISMISS" });
  }

  const updateTooltipPosition = useCallback(() => {
    const trigger = triggerRef.current;
    const bubble = bubbleRef.current;
    if (!trigger || !bubble) return;
    const next = resolveOverlayPosition(trigger.getBoundingClientRect(), bubble.getBoundingClientRect(), placement);
    setResolvedPlacement((prev) => (prev === next.placement ? prev : next.placement));
    setBubbleStyle((prev) => {
      const top = `${next.top}px`;
      const left = `${next.left}px`;
      return prev.top === top && prev.left === left ? prev : { top, left };
    });
    trigger.setAttribute("aria-describedby", tooltipId);
  }, [placement, tooltipId]);

  useLayoutEffect(() => {
    if (isOpen) updateTooltipPosition();
  }, [isOpen, updateTooltipPosition]);

  useEffect(() => {
    const onViewportChange = () => {
      if (bubbleRef.current) updateTooltipPosition();
    };
    window.addEventListener("resize", onViewportChange);
    window.addEventListener("scroll", onViewportChange, true);
    return () => {
      window.removeEventListener("resize", onViewportChange);
      window.removeEventListener("scroll", onViewportChange, true);
    };
  }, [updateTooltipPosition]);

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
        <span id={tooltipId} ref={bubbleRef} className="poodle-tooltip__bubble" data-placement={resolvedPlacement} style={bubbleStyle} role="tooltip">
          {content}
        </span>
      ) : null}
    </span>
  );
}
