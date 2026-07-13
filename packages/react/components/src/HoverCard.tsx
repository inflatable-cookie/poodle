import {
  useCallback,
  useEffect,
  useId,
  useLayoutEffect,
  useRef,
  useState,
  type CSSProperties,
  type ReactNode,
} from "react";
import { hoverTransition, type HoverEvent as HoverMachineEvent, type HoverState } from "@poodle/headless";

import "@poodle/styles/hover-card.css";

import { resolveOverlayPosition } from "./overlay-position";
import type { OverlayPlacement } from "./types";

export interface HoverCardProps {
  open?: boolean | null;
  defaultOpen?: boolean;
  openDelayMs?: number;
  closeDelayMs?: number;
  placement?: OverlayPlacement;
  ariaLabel?: string | null;
  onOpenChange?: (open: boolean) => void;
  trigger?: ReactNode;
  children?: ReactNode;
}

export function HoverCard({
  open = null,
  defaultOpen = false,
  openDelayMs = 180,
  closeDelayMs = 120,
  placement = "top",
  ariaLabel = null,
  onOpenChange,
  trigger,
  children,
}: HoverCardProps) {
  const hoverCardId = useId();
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [surfaceStyle, setSurfaceStyle] = useState<CSSProperties>({});

  const triggerRef = useRef<HTMLSpanElement | null>(null);
  const surfaceRef = useRef<HTMLSpanElement | null>(null);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const machineState = useRef<HoverState>("closed");

  const isControlled = open !== null;
  const isOpen = isControlled ? open === true : uncontrolledOpen;

  const clearTimers = useCallback(() => {
    if (timerRef.current) {
      clearTimeout(timerRef.current);
      timerRef.current = null;
    }
  }, []);

  const send = useCallback(
    (event: HoverMachineEvent) => {
      const result = hoverTransition(machineState.current, { openDelayMs, closeDelayMs }, event);
      machineState.current = result.state;
      for (const effect of result.effects) {
        if (effect.type === "clearTimer") {
          clearTimers();
        } else if (effect.type === "startTimer") {
          clearTimers();
          timerRef.current = setTimeout(() => send({ type: "TIMER_FIRE" }), effect.ms);
        } else if (effect.type === "emitOpenChange") {
          if (!isControlled) setUncontrolledOpen(effect.open);
          onOpenChange?.(effect.open);
        }
      }
    },
    [clearTimers, closeDelayMs, isControlled, onOpenChange, openDelayMs],
  );

  const positionSurface = useCallback(() => {
    const triggerEl = triggerRef.current;
    const surfaceEl = surfaceRef.current;
    if (!triggerEl || !surfaceEl) return;
    const position = resolveOverlayPosition(
      triggerEl.getBoundingClientRect(),
      surfaceEl.getBoundingClientRect(),
      placement,
      8,
    );
    setSurfaceStyle((prev) => {
      const top = `${position.top}px`;
      const left = `${position.left}px`;
      return prev.top === top && prev.left === left ? prev : { top, left };
    });
  }, [placement]);

  useLayoutEffect(() => {
    if (isOpen) positionSurface();
  }, [isOpen, positionSurface]);

  useEffect(() => clearTimers, [clearTimers]);

  return (
    <span
      className="poodle-hover-card"
      role="presentation"
      onMouseEnter={() => send({ type: "ENTER" })}
      onMouseLeave={() => send({ type: "LEAVE" })}
      onFocus={() => send({ type: "ENTER" })}
      onBlur={() => send({ type: "LEAVE" })}
      onKeyDown={(event) => {
        if (event.key === "Escape") {
          clearTimers();
          if (!isControlled) setUncontrolledOpen(false);
          onOpenChange?.(false);
          machineState.current = "closed";
        }
      }}
    >
      <span
        ref={triggerRef}
        className="poodle-hover-card__trigger"
        role="button"
        tabIndex={0}
        aria-expanded={isOpen}
        aria-controls={isOpen ? hoverCardId : undefined}
      >
        {trigger}
      </span>

      {isOpen ? (
        <span
          ref={surfaceRef}
          id={hoverCardId}
          className="poodle-hover-card__surface"
          role="dialog"
          tabIndex={-1}
          aria-label={ariaLabel ?? undefined}
          style={surfaceStyle}
          onMouseEnter={clearTimers}
          onMouseLeave={() => send({ type: "LEAVE" })}
        >
          {children}
        </span>
      ) : null}
    </span>
  );
}
