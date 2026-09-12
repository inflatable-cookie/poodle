import { useEffect, useLayoutEffect, useRef, useState, type CSSProperties, type KeyboardEvent as ReactKeyboardEvent, type PointerEvent as ReactPointerEvent } from "react";
import {
  createSliderControlContext, layoutSliderBlock, measureInlineAdvance,
  normalizeSliderValue, resolveSliderVisibleValue, safeSliderMax,
  sliderControlTransition, sliderFamilyCapsuleSpan, sliderFamilyValueDockedToMarker,
  sliderFamilyValueNorm, sliderTransition, sliderVisualState,
  type AudioValueLaw, type SliderContext, type SliderControlContext,
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
  variant = "block",
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
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState(defaultValue);
  const [controlMachine, setControlMachine] = useState(createSliderControlContext);
  const controlRef = useRef(createSliderControlContext());
  const live = useRef<SliderControlContext | null>(null);
  const cancelOnUnmount = useRef<(pointerId?: number | null) => void>(() => {});
  const onValueChangeRef = useRef(onValueChange);
  const onValueCommitRef = useRef(onValueCommit);
  const isControlledRef = useRef(false);
  const root = useRef<HTMLDivElement>(null);
  const capsule = useRef<HTMLSpanElement>(null);
  const [capsuleSpan, setCapsuleSpan] = useState(0);
  const activePointer = useRef<number | null>(null);

  const isControlled = value !== undefined;
  onValueChangeRef.current = onValueChange;
  onValueCommitRef.current = onValueCommit;
  isControlledRef.current = isControlled;
  const currentValue = isControlled ? value : uncontrolledValue;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const machineContext: SliderContext = { value: currentValue, min, max, step, disabled };
  const controlContext: SliderControlContext = { ...controlMachine, value: currentValue, min, max, step, disabled, law, polarity, centerValue };
  controlRef.current = { ...controlRef.current, ...controlContext };
  const visualState = sliderVisualState(controlContext);
  const safeMax = safeSliderMax(min, max);
  const displayValue = normalizeSliderValue(machineContext, currentValue);
  const block = variant === "block";
  const sliderStyle = {
    "--poodle-slider-percent": `${visualState.valueNorm * 100}%`,
    "--poodle-slider-fill-start": `${visualState.fillStartNorm * 100}%`,
    "--poodle-slider-fill-span": `${visualState.fillSpanNorm * 100}%`,
    "--poodle-slider-center": `${visualState.centerNorm * 100}%`,
  } as CSSProperties;
  const visibleValueText = resolveSliderVisibleValue(displayValue, min, step, formatVisibleValue);
  const visibleLabelText = visibleLabel && visibleLabel !== "" ? visibleLabel : null;
  const font = capsule.current ? getComputedStyle(capsule.current).font : "14px sans-serif";
  const blockLayout = block
    ? layoutSliderBlock({
      capsuleSpan,
      label: visibleLabelText,
      valueText: visibleValueText,
      measure: (text) => measureInlineAdvance(text, font),
    })
    : { labelInline: false, valueInline: false };
  const valueDockedToMarker = block
    && orientation === "horizontal"
    && capsuleSpan > 0
    && visibleValueText != null
    && sliderFamilyValueDockedToMarker({
      valueNorm: visualState.valueNorm,
      span: capsuleSpan,
      advance: measureInlineAdvance(visibleValueText, font),
    });

  function runControl(event: Parameters<typeof sliderControlTransition>[1]): void {
    const result = sliderControlTransition(controlRef.current, event);
    controlRef.current = result.context;
    live.current = result.context;
    setControlMachine(result.context);
    for (const effect of result.effects) {
      if (!isControlledRef.current) setUncontrolledValue(effect.value);
      if (effect.type === "emitValueChange") onValueChangeRef.current?.(effect.value);
      else onValueCommitRef.current?.(effect.value);
    }
  }
  function pointNorm(event: ReactPointerEvent<HTMLElement>): number {
    return sliderFamilyValueNorm({
      rect: root.current!.getBoundingClientRect(),
      orientation,
      direction,
      clientX: event.clientX,
      clientY: event.clientY,
    });
  }
  function pointerDown(event: ReactPointerEvent<HTMLElement>): void {
    if (event.button !== 0 || disabled) return;
    const target = block ? event.currentTarget : root.current;
    if (!target) return;
    event.preventDefault();
    event.stopPropagation();
    activePointer.current = event.pointerId;
    target.setPointerCapture(event.pointerId);
    runControl({ type: "POINTER_BEGIN", valueNorm: pointNorm(event) });
  }
  function pointerMove(event: ReactPointerEvent<HTMLElement>): void {
    if (activePointer.current === event.pointerId) {
      event.stopPropagation();
      runControl({ type: "POINTER_MOVE", valueNorm: pointNorm(event) });
    }
  }
  function terminate(pointerId: number | null = null): void {
    if (activePointer.current === null || (pointerId !== null && activePointer.current !== pointerId)) return;
    activePointer.current = null;
    const snapshot = live.current ?? controlRef.current;
    const result = sliderControlTransition(snapshot, { type: "POINTER_END" });
    controlRef.current = result.context;
    live.current = result.context;
    setControlMachine(result.context);
    for (const effect of result.effects) {
      if (!isControlledRef.current) setUncontrolledValue(effect.value);
      if (effect.type === "emitValueChange") onValueChangeRef.current?.(effect.value);
      else onValueCommitRef.current?.(effect.value);
    }
  }
  function pointerEnd(event: ReactPointerEvent<HTMLElement>): void {
    event.stopPropagation();
    terminate(event.pointerId);
  }
  const pointerHandlers = {
    onPointerDown: pointerDown,
    onPointerMove: pointerMove,
    onPointerUp: pointerEnd,
    onPointerCancel: pointerEnd,
    onLostPointerCapture: pointerEnd,
  };
  function controlKey(event: ReactKeyboardEvent<HTMLElement>): void {
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
    const observer = new ResizeObserver(() => {
      const rect = node.getBoundingClientRect();
      setCapsuleSpan(sliderFamilyCapsuleSpan(rect, orientation));
    });
    observer.observe(node);
    const rect = node.getBoundingClientRect();
    setCapsuleSpan(sliderFamilyCapsuleSpan(rect, orientation));
    return () => observer.disconnect();
  }, [block, orientation]);

  useEffect(() => {
    if (disabled) terminate();
  }, [disabled]);

  cancelOnUnmount.current = terminate;
  useEffect(() => () => cancelOnUnmount.current(), []);

  return (
    <div ref={root}
      className="poodle-slider"
      data-orientation={orientation}
      data-disabled={disabled}
      style={sliderStyle}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-variant={variant}
      data-direction={direction === "rtl" ? direction : undefined}
      data-polarity={visualState.polarity}
      data-fill-tone={visualState.fillTone}
      data-state={visualState.pointerActive ? "active" : "idle"}
      data-value-docked={valueDockedToMarker ? "marker" : undefined}
      dir={direction === "rtl" ? direction : undefined}
      role="slider"
      tabIndex={disabled ? undefined : 0}
      aria-label={ariaLabel ?? undefined}
      aria-valuemin={min}
      aria-valuemax={safeMax}
      aria-valuenow={visualState.value}
      aria-valuetext={valueText ?? undefined}
      aria-orientation={orientation}
      aria-disabled={disabled}
      {...pointerHandlers}
      onKeyDown={controlKey}
    >
      {block ? (
        <>
          <span ref={capsule} className="poodle-slider__capsule" aria-hidden="true">
            <span className="poodle-slider__track">
              <span className="poodle-slider__fill" />
              <span className="poodle-slider__remainder" />
              {blockLayout.labelInline || blockLayout.valueInline ? (
                <>
                  {/* One stable text layout painted twice; the clip moves, the glyphs never do. */}
                  {orientation === "vertical" ? (
                    <>
                      <span className="poodle-slider__inline poodle-slider__inline--selected">
                        <span className="poodle-slider__inline-row poodle-slider__inline-row--vertical">
                          <span className="poodle-slider__inline-value">{blockLayout.valueInline ? visibleValueText : ""}</span>
                          <span className="poodle-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
                          <span className="poodle-slider__inline-spacer" />
                        </span>
                      </span>
                      <span className="poodle-slider__inline poodle-slider__inline--remainder">
                        <span className="poodle-slider__inline-row poodle-slider__inline-row--vertical">
                          <span className="poodle-slider__inline-value">{blockLayout.valueInline ? visibleValueText : ""}</span>
                          <span className="poodle-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
                          <span className="poodle-slider__inline-spacer" />
                        </span>
                      </span>
                    </>
                  ) : (
                    <>
                      <span className="poodle-slider__inline poodle-slider__inline--selected">
                        <span className="poodle-slider__inline-row">
                          <span className="poodle-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
                          <span className="poodle-slider__inline-value">{blockLayout.valueInline ? visibleValueText : ""}</span>
                        </span>
                      </span>
                      <span className="poodle-slider__inline poodle-slider__inline--remainder">
                        <span className="poodle-slider__inline-row">
                          <span className="poodle-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
                          <span className="poodle-slider__inline-value">{blockLayout.valueInline ? visibleValueText : ""}</span>
                        </span>
                      </span>
                    </>
                  )}
                </>
              ) : null}
              <span className="poodle-slider__center" />
            </span>
            <span className="poodle-slider__hit" data-part="hit" {...pointerHandlers}><span className="poodle-slider__thumb" /></span>
          </span>
        </>
      ) : (
        <>
          <span className="poodle-slider__track" aria-hidden="true">
            <span className="poodle-slider__fill" />
            <span className="poodle-slider__center" />
          </span>
        </>
      )}
    </div>
  );
}
