import { useRef, useState, type CSSProperties, type KeyboardEvent, type PointerEvent } from "react";
import {
  createRangeSliderControlContext,
  normalizeRangeValue,
  rangeSliderControlTransition,
  rangeSliderTransition,
  rangeSliderVisualState,
  safeSliderMax,
  type AudioValueLaw, type RangeSliderContext, type RangeSliderControlContext,
  type SliderPolarity, type SliderVariant,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/range-slider.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";
import type { Orientation } from "./Separator";

export interface RangeSliderProps {
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  value?: [number, number];
  defaultValue?: [number, number];
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
  lowerValueText?: string | null;
  upperValueText?: string | null;
  onValueChange?: (value: [number, number]) => void;
  onValueCommit?: (value: [number, number]) => void;
}

export function RangeSlider({
  size = null,
  sizeRole = "control",
  density = null,
  value,
  defaultValue = [0, 100],
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
  lowerValueText = null,
  upperValueText = null,
  onValueChange,
  onValueCommit,
}: RangeSliderProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState<[number, number]>(defaultValue);
  const [controlMachine, setControlMachine] = useState(createRangeSliderControlContext);
  const controlRef = useRef(createRangeSliderControlContext());
  const root = useRef<HTMLDivElement>(null);
  const activePointer = useRef<number | null>(null);

  const isControlled = value !== undefined;
  const currentValue = isControlled ? value : uncontrolledValue;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const machineContext: RangeSliderContext = { value: currentValue, min, max, step, disabled };
  const controlContext: RangeSliderControlContext = { ...controlMachine, value: currentValue, min, max, step, disabled, law, polarity, centerValue };
  controlRef.current = { ...controlRef.current, ...controlContext };
  const visualState = rangeSliderVisualState(controlContext);
  const safeMax = safeSliderMax(min, max);
  const [displayLower, displayUpper] = normalizeRangeValue(machineContext);
  const lowerPercent = visualState.lowerNorm * 100;
  const upperPercent = visualState.upperNorm * 100;
  const rangeStyle = {
    "--poodle-range-start": `${lowerPercent}%`,
    "--poodle-range-end": `${upperPercent}%`,
    "--poodle-range-center": `${visualState.centerNorm * 100}%`,
    "--poodle-range-negative-start": `${visualState.negativeFillStartNorm * 100}%`,
    "--poodle-range-negative-span": `${visualState.negativeFillSpanNorm * 100}%`,
    "--poodle-range-positive-start": `${visualState.positiveFillStartNorm * 100}%`,
    "--poodle-range-positive-span": `${visualState.positiveFillSpanNorm * 100}%`,
  } as CSSProperties;

  function send(
    type: "INPUT" | "COMMIT",
    thumb: "lower" | "upper",
    event: { currentTarget: EventTarget & HTMLInputElement },
  ): void {
    const raw = Number(event.currentTarget.value);
    const result = rangeSliderTransition(machineContext, { type, thumb, raw });
    for (const effect of result.effects) {
      if (!isControlled) setUncontrolledValue(effect.value);
      if (effect.type === "emitValueChange") {
        onValueChange?.(effect.value);
      } else if (effect.type === "emitValueCommit") {
        onValueCommit?.(effect.value);
      }
    }
  }

  function runControl(event: Parameters<typeof rangeSliderControlTransition>[1]): void {
    const result = rangeSliderControlTransition(controlRef.current, event); controlRef.current = result.context; setControlMachine(result.context);
    for (const effect of result.effects) {
      if (!isControlled) setUncontrolledValue(effect.value);
      if (effect.type === "emitValueChange") onValueChange?.(effect.value); else onValueCommit?.(effect.value);
    }
  }
  function pointNorm(event: PointerEvent<HTMLDivElement>): number {
    const rect = root.current!.getBoundingClientRect();
    return orientation === "horizontal" ? Math.min(Math.max((event.clientX - rect.left) / Math.max(rect.width, 1), 0), 1) : 1 - Math.min(Math.max((event.clientY - rect.top) / Math.max(rect.height, 1), 0), 1);
  }
  function pointerDown(event: PointerEvent<HTMLDivElement>): void {
    if (variant !== "embedded" || event.button !== 0 || disabled || !root.current) return;
    event.preventDefault(); activePointer.current = event.pointerId; root.current.setPointerCapture(event.pointerId); runControl({ type: "POINTER_BEGIN", valueNorm: pointNorm(event) });
  }
  function pointerMove(event: PointerEvent<HTMLDivElement>): void { if (activePointer.current === event.pointerId) runControl({ type: "POINTER_MOVE", valueNorm: pointNorm(event) }); }
  function pointerEnd(event: PointerEvent<HTMLDivElement>): void { if (activePointer.current === event.pointerId) { activePointer.current = null; runControl({ type: "POINTER_END" }); } }
  function embeddedKey(event: KeyboardEvent<HTMLDivElement>, thumb: "lower" | "upper"): void {
    const direction = ({ ArrowLeft: -1, ArrowDown: -1, ArrowRight: 1, ArrowUp: 1 } as Record<string, -1 | 1>)[event.key];
    const current = thumb === "lower" ? displayLower : displayUpper;
    const raw = event.key === "Home" ? min : event.key === "End" ? safeMax : direction ? current + direction * step : null;
    if (raw == null) return;
    event.preventDefault();
    const changed = rangeSliderTransition(machineContext, { type: "INPUT", thumb, raw });
    const committed = rangeSliderTransition(changed.context, { type: "COMMIT", thumb, raw: thumb === "lower" ? changed.context.value[0] : changed.context.value[1] });
    for (const effect of [...changed.effects, ...committed.effects]) {
      if (!isControlled) setUncontrolledValue(effect.value);
      if (effect.type === "emitValueChange") onValueChange?.(effect.value); else onValueCommit?.(effect.value);
    }
  }

  return (
    <div ref={root}
      className="poodle-range-slider"
      role="group"
      data-orientation={orientation}
      data-disabled={disabled}
      style={rangeStyle}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-variant={variant} data-polarity={visualState.polarity} data-fill-split={visualState.fillSplitAtCenter} data-state={visualState.pointerActive ? "active" : "idle"}
      onPointerDown={pointerDown} onPointerMove={pointerMove} onPointerUp={pointerEnd} onPointerCancel={pointerEnd}
    >
      <span className="poodle-range-slider__track" aria-hidden="true">
        <span className="poodle-range-slider__fill poodle-range-slider__fill--negative" />
        <span className="poodle-range-slider__fill poodle-range-slider__fill--positive" />
        <span className="poodle-range-slider__center" />
      </span>

      {variant === "standard" && <><input
        className="poodle-range-slider__control poodle-range-slider__control--lower"
        type="range"
        min={min}
        max={safeMax}
        step={step}
        value={displayLower}
        disabled={disabled}
        aria-label={ariaLabel ? `${ariaLabel} minimum` : "Minimum value"}
        aria-valuetext={lowerValueText ?? undefined}
        onInput={(event) => send("INPUT", "lower", event)}
        onMouseUp={(event) => send("COMMIT", "lower", event)}
        onKeyUp={(event) => send("COMMIT", "lower", event)}
        onTouchEnd={(event) => send("COMMIT", "lower", event)}
      />

      <input
        className="poodle-range-slider__control poodle-range-slider__control--upper"
        type="range"
        min={min}
        max={safeMax}
        step={step}
        value={displayUpper}
        disabled={disabled}
        aria-label={ariaLabel ? `${ariaLabel} maximum` : "Maximum value"}
        aria-valuetext={upperValueText ?? undefined}
        onInput={(event) => send("INPUT", "upper", event)}
        onMouseUp={(event) => send("COMMIT", "upper", event)}
        onKeyUp={(event) => send("COMMIT", "upper", event)}
        onTouchEnd={(event) => send("COMMIT", "upper", event)}
      /></>}
      {variant === "embedded" && <>
        <div className="poodle-range-slider__embedded-control poodle-range-slider__embedded-control--lower" role="slider" tabIndex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} minimum` : "Minimum value"} aria-valuemin={min} aria-valuemax={displayUpper} aria-valuenow={displayLower} aria-valuetext={lowerValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onKeyDown={(event) => embeddedKey(event, "lower")} />
        <div className="poodle-range-slider__embedded-control poodle-range-slider__embedded-control--upper" role="slider" tabIndex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} maximum` : "Maximum value"} aria-valuemin={displayLower} aria-valuemax={safeMax} aria-valuenow={displayUpper} aria-valuetext={upperValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onKeyDown={(event) => embeddedKey(event, "upper")} />
      </>}
    </div>
  );
}
