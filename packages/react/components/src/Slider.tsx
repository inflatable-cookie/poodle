import { useRef, useState, type CSSProperties, type FormEvent, type KeyboardEvent, type PointerEvent } from "react";
import { createSliderControlContext, normalizeSliderValue, safeSliderMax, sliderControlTransition, sliderTransition, sliderVisualState, type AudioValueLaw, type SliderContext, type SliderControlContext, type SliderPolarity, type SliderVariant } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/slider.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";
import type { Orientation } from "./Separator";

export interface SliderProps {
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  value?: number;
  defaultValue?: number;
  min?: number;
  max?: number;
  step?: number;
  variant?: SliderVariant;
  polarity?: SliderPolarity;
  centerValue?: number | null;
  law?: AudioValueLaw;
  orientation?: Orientation;
  disabled?: boolean;
  ariaLabel?: string | null;
  valueText?: string | null;
  onValueChange?: (value: number) => void;
  onValueCommit?: (value: number) => void;
}

export function Slider({
  size = null,
  sizeRole = "control",
  density = null,
  value,
  defaultValue = 0,
  min = 0,
  max = 100,
  step = 1,
  variant = "standard",
  polarity = "unipolar",
  centerValue = null,
  law = { type: "linear" },
  orientation = "horizontal",
  disabled = false,
  ariaLabel = null,
  valueText = null,
  onValueChange,
  onValueCommit,
}: SliderProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState(defaultValue);
  const [controlMachine, setControlMachine] = useState(createSliderControlContext);
  const controlRef = useRef(createSliderControlContext());
  const root = useRef<HTMLDivElement>(null);
  const activePointer = useRef<number | null>(null);

  const isControlled = value !== undefined;
  const currentValue = isControlled ? value : uncontrolledValue;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const machineContext: SliderContext = { value: currentValue, min, max, step, disabled };
  const controlContext: SliderControlContext = { ...controlMachine, value: currentValue, min, max, step, disabled, law, polarity, centerValue };
  controlRef.current = { ...controlRef.current, ...controlContext };
  const visualState = sliderVisualState(controlContext);
  const safeMax = safeSliderMax(min, max);
  const displayValue = normalizeSliderValue(machineContext, currentValue);
  const sliderStyle = {
    "--poodle-slider-percent": `${visualState.valueNorm * 100}%`,
    "--poodle-slider-fill-start": `${variant === "standard" ? 0 : visualState.fillStartNorm * 100}%`,
    "--poodle-slider-fill-span": `${(variant === "standard" ? visualState.valueNorm : visualState.fillSpanNorm) * 100}%`,
    "--poodle-slider-center": `${visualState.centerNorm * 100}%`,
  } as CSSProperties;

  function send(type: "INPUT" | "COMMIT", event: FormEvent<HTMLInputElement>): void {
    const raw = Number(event.currentTarget.value);
    const result = sliderTransition(machineContext, { type, raw });
    for (const effect of result.effects) {
      if (!isControlled) setUncontrolledValue(effect.value);
      if (effect.type === "emitValueChange") {
        onValueChange?.(effect.value);
      } else if (effect.type === "emitValueCommit") {
        onValueCommit?.(effect.value);
      }
    }
  }

  function runControl(event: Parameters<typeof sliderControlTransition>[1]): void {
    const result = sliderControlTransition(controlRef.current, event);
    controlRef.current = result.context;
    setControlMachine(result.context);
    for (const effect of result.effects) {
      if (!isControlled) setUncontrolledValue(effect.value);
      if (effect.type === "emitValueChange") onValueChange?.(effect.value); else onValueCommit?.(effect.value);
    }
  }
  function pointNorm(event: PointerEvent<HTMLDivElement>): number {
    const rect = root.current!.getBoundingClientRect();
    return orientation === "horizontal"
      ? Math.min(Math.max((event.clientX - rect.left) / Math.max(rect.width, 1), 0), 1)
      : 1 - Math.min(Math.max((event.clientY - rect.top) / Math.max(rect.height, 1), 0), 1);
  }
  function pointerDown(event: PointerEvent<HTMLDivElement>): void {
    if (variant !== "embedded" || event.button !== 0 || disabled || !root.current) return;
    event.preventDefault(); activePointer.current = event.pointerId; root.current.setPointerCapture(event.pointerId);
    runControl({ type: "POINTER_BEGIN", valueNorm: pointNorm(event) });
  }
  function pointerMove(event: PointerEvent<HTMLDivElement>): void { if (activePointer.current === event.pointerId) runControl({ type: "POINTER_MOVE", valueNorm: pointNorm(event) }); }
  function pointerEnd(event: PointerEvent<HTMLDivElement>): void { if (activePointer.current === event.pointerId) { activePointer.current = null; runControl({ type: "POINTER_END" }); } }
  function embeddedKey(event: KeyboardEvent<HTMLDivElement>): void {
    const direction = ({ ArrowLeft: -1, ArrowDown: -1, ArrowRight: 1, ArrowUp: 1 } as Record<string, -1 | 1>)[event.key];
    const raw = event.key === "Home" ? min : event.key === "End" ? safeMax : direction ? currentValue + direction * step : null;
    if (raw == null) return;
    event.preventDefault();
    const changed = sliderTransition(machineContext, { type: "INPUT", raw });
    const committed = sliderTransition(changed.context, { type: "COMMIT", raw: changed.context.value });
    for (const effect of [...changed.effects, ...committed.effects]) {
      if (!isControlled) setUncontrolledValue(effect.value);
      if (effect.type === "emitValueChange") onValueChange?.(effect.value); else onValueCommit?.(effect.value);
    }
  }

  return (
    <div ref={root}
      className="poodle-slider"
      data-orientation={orientation}
      data-disabled={disabled}
      style={sliderStyle}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-variant={variant}
      data-polarity={visualState.polarity}
      data-fill-tone={visualState.fillTone}
      data-state={visualState.pointerActive ? "active" : "idle"}
      role={variant === "embedded" ? "slider" : undefined}
      tabIndex={variant === "embedded" && !disabled ? 0 : undefined}
      aria-label={variant === "embedded" ? ariaLabel ?? undefined : undefined}
      aria-valuemin={variant === "embedded" ? min : undefined}
      aria-valuemax={variant === "embedded" ? safeMax : undefined}
      aria-valuenow={variant === "embedded" ? visualState.value : undefined}
      aria-valuetext={variant === "embedded" ? valueText ?? undefined : undefined}
      aria-orientation={variant === "embedded" ? orientation : undefined}
      aria-disabled={variant === "embedded" ? disabled : undefined}
      onPointerDown={pointerDown} onPointerMove={pointerMove} onPointerUp={pointerEnd} onPointerCancel={pointerEnd}
      onKeyDown={variant === "embedded" ? embeddedKey : undefined}
    >
      <span className="poodle-slider__track" aria-hidden="true">
        <span className="poodle-slider__fill" />
        <span className="poodle-slider__center" />
      </span>
      {variant === "standard" && <input
        className="poodle-slider__control"
        type="range"
        min={min}
        max={safeMax}
        step={step}
        value={displayValue}
        disabled={disabled}
        aria-label={ariaLabel ?? undefined}
        aria-valuetext={valueText ?? undefined}
        onInput={(event) => send("INPUT", event)}
        onChange={(event) => send("COMMIT", event)}
      />}
    </div>
  );
}
