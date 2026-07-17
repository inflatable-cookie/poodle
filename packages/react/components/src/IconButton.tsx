import {
  useCallback,
  useEffect,
  useId,
  useLayoutEffect,
  useRef,
  useState,
  type CSSProperties,
  type FocusEvent,
  type MouseEvent,
  type ReactNode,
} from "react";
import { hoverTransition, type HoverEvent as HoverMachineEvent, type HoverState } from "@poodle/headless";

import "@poodle/styles/icon-button.css";

import { Icon } from "./Icon";
import { resolveOverlayPosition } from "./overlay-position";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import { Spinner } from "./Spinner";
import type {
  ButtonTone,
  ButtonVariant,
  ControlDensity,
  ControlSize,
  IconProp,
  OverlayPlacement,
  SemanticControlSizeRole,
} from "./types";

export interface IconButtonProps {
  variant?: ButtonVariant;
  tone?: ButtonTone;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  icon: IconProp;
  ariaLabel: string;
  tooltip?: string | null;
  tooltipPlacement?: OverlayPlacement;
  disabled?: boolean;
  loading?: boolean;
  pressed?: boolean | null;
  defaultPressed?: boolean | null;
  describedBy?: string | null;
  type?: "button" | "submit" | "reset";
  onClick?: ((event: MouseEvent<HTMLButtonElement>) => void) | null;
  onFocus?: ((event: FocusEvent<HTMLButtonElement>) => void) | null;
  onBlur?: ((event: FocusEvent<HTMLButtonElement>) => void) | null;
  onPressedChange?: ((pressed: boolean) => void) | null;
  children?: ReactNode;
}

export function IconButton({
  variant = "primary",
  tone = "default",
  size = null,
  sizeRole = "control",
  density = null,
  icon,
  ariaLabel,
  tooltip = null,
  tooltipPlacement = "top",
  disabled = false,
  loading = false,
  pressed = null,
  defaultPressed = null,
  describedBy = null,
  type = "button",
  onClick = null,
  onFocus = null,
  onBlur = null,
  onPressedChange = null,
  children,
}: IconButtonProps) {
  const uiPresentation = useUiPresentation();
  const tooltipId = useId();

  const [tooltipOpen, setTooltipOpen] = useState(false);
  const [resolvedTooltipPlacement, setResolvedTooltipPlacement] = useState<OverlayPlacement>(tooltipPlacement);
  const [tooltipStyle, setTooltipStyle] = useState<CSSProperties>({});
  const [uncontrolledPressed, setUncontrolledPressed] = useState(defaultPressed === true);

  const buttonRef = useRef<HTMLButtonElement | null>(null);
  const tooltipRef = useRef<HTMLSpanElement | null>(null);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const hoverStateRef = useRef<HoverState>("closed");

  const isUnavailable = disabled || loading;
  const isToggle = pressed !== null || defaultPressed !== null;
  const pressedControlled = pressed !== null;
  const currentPressed = pressedControlled ? pressed === true : uncontrolledPressed;
  const tooltipText = tooltip ?? ariaLabel;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  const clearTimer = useCallback(() => {
    if (timerRef.current) {
      clearTimeout(timerRef.current);
      timerRef.current = null;
    }
  }, []);

  const sendHover = useCallback(
    (event: HoverMachineEvent) => {
      const result = hoverTransition(hoverStateRef.current, { openDelayMs: 300, closeDelayMs: 0 }, event);
      hoverStateRef.current = result.state;
      for (const effect of result.effects) {
        if (effect.type === "clearTimer") {
          clearTimer();
        } else if (effect.type === "startTimer") {
          clearTimer();
          timerRef.current = setTimeout(() => sendHover({ type: "TIMER_FIRE" }), effect.ms);
        } else if (effect.type === "emitOpenChange") {
          setTooltipOpen(effect.open);
        }
      }
    },
    [clearTimer],
  );

  const updateTooltipPosition = useCallback(() => {
    const button = buttonRef.current;
    const tip = tooltipRef.current;
    if (!button || !tip) return;
    const next = resolveOverlayPosition(button.getBoundingClientRect(), tip.getBoundingClientRect(), tooltipPlacement);
    // Guard: only set state on real change, or the position pass re-renders
    // itself into an update loop.
    setResolvedTooltipPlacement((prev) => (prev === next.placement ? prev : next.placement));
    setTooltipStyle((prev) => {
      const top = `${next.top}px`;
      const left = `${next.left}px`;
      return prev.top === top && prev.left === left ? prev : { top, left };
    });
  }, [tooltipPlacement]);

  useLayoutEffect(() => {
    if (tooltipOpen && tooltipText) updateTooltipPosition();
  }, [tooltipOpen, tooltipText, updateTooltipPosition]);

  useEffect(() => {
    const onViewportChange = () => {
      if (tooltipRef.current) updateTooltipPosition();
    };
    window.addEventListener("resize", onViewportChange);
    window.addEventListener("scroll", onViewportChange, true);
    return () => {
      window.removeEventListener("resize", onViewportChange);
      window.removeEventListener("scroll", onViewportChange, true);
    };
  }, [updateTooltipPosition]);

  useEffect(() => clearTimer, [clearTimer]);

  const handleClick = (event: MouseEvent<HTMLButtonElement>) => {
    sendHover({ type: "DISMISS" });
    if (isToggle) {
      const next = !currentPressed;
      if (!pressedControlled) setUncontrolledPressed(next);
      onPressedChange?.(next);
    }
    onClick?.(event);
  };

  return (
    <span
      className="poodle-icon-button-wrap"
      role="presentation"
      onMouseEnter={() => sendHover({ type: "ENTER" })}
      onMouseLeave={() => sendHover({ type: "DISMISS" })}
    >
      <button
        type={type}
        ref={buttonRef}
        className="poodle-icon-button"
        data-variant={variant}
        data-tone={tone !== "default" ? tone : undefined}
        data-size={resolvedSize}
        data-density={resolvedDensity}
        data-loading={loading}
        data-pressed={isToggle ? currentPressed : undefined}
        disabled={isUnavailable}
        aria-label={ariaLabel}
        aria-describedby={tooltipOpen ? tooltipId : (describedBy ?? undefined)}
        aria-busy={loading ? "true" : undefined}
        aria-pressed={isToggle ? currentPressed : undefined}
        onClick={handleClick}
        onFocus={(event) => {
          sendHover({ type: "ENTER" });
          onFocus?.(event);
        }}
        onBlur={(event) => {
          sendHover({ type: "DISMISS" });
          onBlur?.(event);
        }}
        onKeyDown={(event) => {
          if (event.key === "Escape") sendHover({ type: "DISMISS" });
        }}
      >
        {loading ? (
          <span className="poodle-icon-button__spinner" aria-hidden="true">
            <Spinner variant="ring" size={resolvedSize} tone="current" />
          </span>
        ) : (
          <span className="poodle-icon-button__glyph" aria-hidden="true">
            {children ?? <Icon icon={icon} size={resolvedSize} />}
          </span>
        )}
      </button>

      {tooltipOpen && tooltipText ? (
        <span
          id={tooltipId}
          ref={tooltipRef}
          className="poodle-icon-button__tooltip"
          data-placement={resolvedTooltipPlacement}
          style={tooltipStyle}
          role="tooltip"
        >
          {tooltipText}
        </span>
      ) : null}
    </span>
  );
}
