import { useEffect, useLayoutEffect, useRef, useState, type CSSProperties, type FormEvent, type KeyboardEvent, type PointerEvent } from "react";
import {
  assertHorizontalBlockAppearance,
  createSliderControlContext, layoutSliderBlock, measureInlineAdvance,
  normalizeSliderValue, physicalToValueNorm, resolveSliderVisibleValue, safeSliderMax,
  sliderControlTransition, sliderTransition, sliderVisualState,
  type AudioValueLaw, type SliderAppearance, type SliderContext, type SliderControlContext,
  type SliderDirection, type SliderPolarity, type SliderVariant,
} from "@inflatable-cookie/poodle-core";

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
  appearance?: SliderAppearance;
  direction?: SliderDirection;
  polarity?: SliderPolarity;
  centerValue?: number | null;
  law?: AudioValueLaw;
  orientation?: Orientation;
  disabled?: boolean;
  ariaLabel?: string | null;
  valueText?: string | null;
  visibleLabel?: string | null;
  formatVisibleValue?: (value: number) => string;
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
  appearance = "track",
  direction = "ltr",
  polarity = "unipolar",
  centerValue = null,
  law = { type: "linear" },
  orientation = "horizontal",
  disabled = false,
  ariaLabel = null,
  valueText = null,
  visibleLabel = null,
  formatVisibleValue,
  onValueChange,
  onValueCommit,
}: SliderProps) {
  assertHorizontalBlockAppearance(appearance, orientation);
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState(defaultValue);
  const [controlMachine, setControlMachine] = useState(createSliderControlContext);
  const controlRef = useRef(createSliderControlContext());
  const root = useRef<HTMLDivElement>(null);
  const capsule = useRef<HTMLSpanElement>(null);
  const [capsuleSpan, setCapsuleSpan] = useState(0);
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
  const block = appearance === "block";
  const usesControlPointer = block || variant === "embedded";
  const sliderStyle = {
    "--poodle-slider-percent": `${visualState.valueNorm * 100}%`,
    "--poodle-slider-fill-start": `${variant === "standard" ? 0 : visualState.fillStartNorm * 100}%`,
    "--poodle-slider-fill-span": `${(variant === "standard" ? visualState.valueNorm : visualState.fillSpanNorm) * 100}%`,
    "--poodle-slider-center": `${visualState.centerNorm * 100}%`,
  } as CSSProperties;
  const visibleValueText = resolveSliderVisibleValue(displayValue, formatVisibleValue);
  const visibleLabelText = visibleLabel && visibleLabel !== "" ? visibleLabel : null;
  const font = capsule.current ? getComputedStyle(capsule.current).font : "14px sans-serif";
  const blockLayout = block
    ? layoutSliderBlock({
      capsuleSpan,
      selectedNorm: visualState.valueNorm,
      label: visibleLabelText,
      valueText: visibleValueText,
      measure: (text) => measureInlineAdvance(text, font),
    })
    : { inline: false, fallback: null };

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
    const physical = orientation === "horizontal"
      ? (event.clientX - rect.left) / Math.max(rect.width, 1)
      : 1 - (event.clientY - rect.top) / Math.max(rect.height, 1);
    return physicalToValueNorm(physical, orientation === "horizontal" ? direction : "ltr");
  }
  function pointerDown(event: PointerEvent<HTMLDivElement>): void {
    if (!usesControlPointer || event.button !== 0 || disabled || !root.current) return;
    event.preventDefault(); activePointer.current = event.pointerId; root.current.setPointerCapture(event.pointerId);
    runControl({ type: "POINTER_BEGIN", valueNorm: pointNorm(event) });
  }
  function pointerMove(event: PointerEvent<HTMLDivElement>): void { if (activePointer.current === event.pointerId) runControl({ type: "POINTER_MOVE", valueNorm: pointNorm(event) }); }
  function terminate(pointerId: number | null = null): void {
    if (activePointer.current === null || (pointerId !== null && activePointer.current !== pointerId)) return;
    activePointer.current = null;
    runControl({ type: "POINTER_END" });
  }
  function pointerEnd(event: PointerEvent<HTMLDivElement>): void { terminate(event.pointerId); }
  function embeddedKey(event: KeyboardEvent<HTMLDivElement>): void {
    if (disabled) return;
    const keyDirection = ({ ArrowLeft: -1, ArrowDown: -1, ArrowRight: 1, ArrowUp: 1 } as Record<string, -1 | 1>)[event.key];
    const raw = event.key === "Home" ? min : event.key === "End" ? safeMax : keyDirection ? currentValue + keyDirection * step : null;
    if (raw == null) return;
    event.preventDefault();
    const changed = sliderTransition(machineContext, { type: "INPUT", raw });
    const committed = sliderTransition(changed.context, { type: "COMMIT", raw: changed.context.value });
    for (const effect of [...changed.effects, ...committed.effects]) {
      if (!isControlled) setUncontrolledValue(effect.value);
      if (effect.type === "emitValueChange") onValueChange?.(effect.value); else onValueCommit?.(effect.value);
    }
  }

  useLayoutEffect(() => {
    if (!block || !capsule.current) return;
    const node = capsule.current;
    const observer = new ResizeObserver(() => setCapsuleSpan(node.getBoundingClientRect().width));
    observer.observe(node);
    setCapsuleSpan(node.getBoundingClientRect().width);
    return () => observer.disconnect();
  }, [block]);

  useEffect(() => {
    if (disabled) terminate();
  }, [disabled]);

  useEffect(() => () => terminate(), []);

  return (
    <div ref={root}
      className="poodle-slider"
      data-orientation={orientation}
      data-disabled={disabled}
      style={sliderStyle}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-variant={variant}
      data-appearance={block ? "block" : undefined}
      data-direction={block || direction === "rtl" ? direction : undefined}
      data-polarity={visualState.polarity}
      data-fill-tone={visualState.fillTone}
      data-state={visualState.pointerActive ? "active" : "idle"}
      dir={block || direction === "rtl" ? direction : undefined}
      role={usesControlPointer ? "slider" : undefined}
      tabIndex={usesControlPointer && !disabled ? 0 : undefined}
      aria-label={usesControlPointer ? ariaLabel ?? undefined : undefined}
      aria-valuemin={usesControlPointer ? min : undefined}
      aria-valuemax={usesControlPointer ? safeMax : undefined}
      aria-valuenow={usesControlPointer ? visualState.value : undefined}
      aria-valuetext={usesControlPointer ? valueText ?? undefined : undefined}
      aria-orientation={usesControlPointer ? orientation : undefined}
      aria-disabled={usesControlPointer ? disabled : undefined}
      onPointerDown={pointerDown} onPointerMove={pointerMove} onPointerUp={pointerEnd} onPointerCancel={pointerEnd}
      onLostPointerCapture={pointerEnd}
      onKeyDown={usesControlPointer ? embeddedKey : undefined}
    >
      {block ? (
        <>
          <span ref={capsule} className="poodle-slider__capsule" aria-hidden="true">
            <span className="poodle-slider__track">
              <span className="poodle-slider__fill">{blockLayout.inline && visibleLabelText ? visibleLabelText : null}</span>
              <span className="poodle-slider__remainder">{blockLayout.inline && visibleValueText ? visibleValueText : null}</span>
              <span className="poodle-slider__center" />
            </span>
            <span className="poodle-slider__hit" data-part="hit"><span className="poodle-slider__thumb" /></span>
          </span>
          {blockLayout.fallback ? <span className="poodle-slider__fallback" aria-hidden="true">{blockLayout.fallback}</span> : null}
        </>
      ) : (
        <>
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
        </>
      )}
    </div>
  );
}
