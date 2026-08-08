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
import { hoverTransition, type HoverEvent as HoverMachineEvent, type HoverState } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/icon-button.css";

import { Icon } from "./Icon";
import { AnchoredSurface } from "./AnchoredSurface";
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
  expanded?: boolean | null;
  controls?: string | null;
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
  expanded = null,
  controls = null,
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
  const [uncontrolledPressed, setUncontrolledPressed] = useState(defaultPressed === true);

  // The button is state, not a ref: the portalled tooltip has to re-render
  // once it exists so it can be positioned against it.
  const [buttonElement, setButtonElement] = useState<HTMLButtonElement | null>(null);
  const tooltipRef = useRef<HTMLElement | null>(null);
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
        ref={setButtonElement}
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
        aria-expanded={expanded ?? undefined}
        aria-controls={controls ?? undefined}
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
        <AnchoredSurface
          id={tooltipId}
          ref={tooltipRef}
          tag="span"
          anchor={buttonElement}
          placement={tooltipPlacement}
          onPlacement={setResolvedTooltipPlacement}
          className="poodle-icon-button__tooltip"
          data-placement={resolvedTooltipPlacement}
          role="tooltip"
        >
          {tooltipText}
        </AnchoredSurface>
      ) : null}
    </span>
  );
}
