import { useEffect, useLayoutEffect, useRef, useState, type CSSProperties, type KeyboardEvent as ReactKeyboardEvent, type PointerEvent as ReactPointerEvent } from "react";
import {
  createRangeSliderControlContext,
  layoutRangeSliderBlock,
  measureInlineAdvance,
  normalizeRangeValue,
  physicalToValueNorm,
  rangeSliderControlTransition,
  rangeSliderTransition,
  rangeSliderVisualState,
  resolveRangeVisibleValue,
  safeSliderMax,
  type AudioValueLaw, type RangeSliderContext, type RangeSliderControlContext,
  type SliderDirection, type SliderPolarity, type SliderVariant,
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
  direction?: SliderDirection;
  polarity?: SliderPolarity;
  centerValue?: number | null;
  law?: AudioValueLaw;
  orientation?: Orientation;
  disabled?: boolean;
  ariaLabel?: string | null;
  lowerValueText?: string | null;
  upperValueText?: string | null;
  visibleLabel?: string | null;
  formatVisibleValue?: (value: number, thumb: "lower" | "upper") => string;
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
  variant = "block",
  direction = "ltr",
  polarity = "unipolar",
  centerValue = null,
  law = { type: "linear" },
  orientation = "horizontal",
  disabled = false,
  ariaLabel = null,
  lowerValueText = null,
  upperValueText = null,
  visibleLabel = null,
  formatVisibleValue,
  onValueChange,
  onValueCommit,
}: RangeSliderProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState<[number, number]>(defaultValue);
  const [controlMachine, setControlMachine] = useState(createRangeSliderControlContext);
  const controlRef = useRef(createRangeSliderControlContext());
  const live = useRef<RangeSliderControlContext | null>(null);
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

  function runControl(event: Parameters<typeof rangeSliderControlTransition>[1]): void {
    const result = rangeSliderControlTransition(controlRef.current, event);
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
    const rect = root.current!.getBoundingClientRect();
    const physical = orientation === "horizontal"
      ? (event.clientX - rect.left) / Math.max(rect.width, 1)
      : 1 - (event.clientY - rect.top) / Math.max(rect.height, 1);
    return physicalToValueNorm(physical, orientation === "horizontal" ? direction : "ltr");
  }
  const block = variant === "block";
  const visibleLabelText = visibleLabel && visibleLabel !== "" ? visibleLabel : null;
  const lowerVisible = resolveRangeVisibleValue(displayLower, "lower", formatVisibleValue);
  const upperVisible = resolveRangeVisibleValue(displayUpper, "upper", formatVisibleValue);
  const font = capsule.current ? getComputedStyle(capsule.current).font : "14px sans-serif";
  const blockLayout = block
    ? layoutRangeSliderBlock({
      capsuleSpan,
      label: visibleLabelText,
      lowerText: lowerVisible,
      upperText: upperVisible,
      measure: (text) => measureInlineAdvance(text, font),
    })
    : { labelInline: false, lowerInline: false, upperInline: false };
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
    const result = rangeSliderControlTransition(snapshot, { type: "POINTER_END" });
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
  function controlKey(event: ReactKeyboardEvent<HTMLElement>, thumb: "lower" | "upper"): void {
    const keyDirection = ({ ArrowLeft: -1, ArrowDown: -1, ArrowRight: 1, ArrowUp: 1 } as Record<string, -1 | 1>)[event.key];
    const current = thumb === "lower" ? displayLower : displayUpper;
    const raw = event.key === "Home" ? min : event.key === "End" ? safeMax : keyDirection ? current + keyDirection * step : null;
    if (raw == null) return;
    event.preventDefault();
    const changed = rangeSliderTransition(machineContext, { type: "INPUT", thumb, raw });
    const committed = rangeSliderTransition(changed.context, { type: "COMMIT", thumb, raw: thumb === "lower" ? changed.context.value[0] : changed.context.value[1] });
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
      setCapsuleSpan(orientation === "vertical" ? rect.height : rect.width);
    });
    observer.observe(node);
    const rect = node.getBoundingClientRect();
    setCapsuleSpan(orientation === "vertical" ? rect.height : rect.width);
    return () => observer.disconnect();
  }, [block, orientation]);
  useEffect(() => { if (disabled) terminate(); }, [disabled]);
  cancelOnUnmount.current = terminate;
  useEffect(() => () => cancelOnUnmount.current(), []);

  const paintText = blockLayout.lowerInline || blockLayout.upperInline || blockLayout.labelInline;

  return (
    <div ref={root}
      className="poodle-range-slider"
      role="group"
      data-orientation={orientation}
      data-disabled={disabled}
      style={rangeStyle}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-variant={variant}
      data-direction={direction === "rtl" ? direction : undefined}
      data-polarity={visualState.polarity}
      data-fill-split={visualState.fillSplitAtCenter}
      data-state={visualState.pointerActive ? "active" : "idle"}
      dir={direction === "rtl" ? direction : undefined}
      {...pointerHandlers}
    >
      {block ? (
        <>
          <span ref={capsule} className="poodle-range-slider__capsule" aria-hidden="true">
            <span className="poodle-range-slider__track">
              <span className="poodle-range-slider__fill poodle-range-slider__fill--negative" />
              <span className="poodle-range-slider__fill poodle-range-slider__fill--positive" />
              <span className="poodle-range-slider__center" />
            </span>
            {paintText ? (
              <>
                {/* One stable text layout painted through window/remainder clips; glyphs never move. */}
                {orientation === "vertical" ? (
                  <>
                    <span className="poodle-range-slider__inline poodle-range-slider__inline--selected">
                      <span className="poodle-range-slider__inline-row poodle-range-slider__inline-row--vertical">
                        <span className="poodle-range-slider__inline-value poodle-range-slider__inline-value--upper">{blockLayout.upperInline ? upperVisible : ""}</span>
                        <span className="poodle-range-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
                        <span className="poodle-range-slider__inline-value poodle-range-slider__inline-value--lower">{blockLayout.lowerInline ? lowerVisible : ""}</span>
                      </span>
                    </span>
                    <span className="poodle-range-slider__inline poodle-range-slider__inline--remainder-top">
                      <span className="poodle-range-slider__inline-row poodle-range-slider__inline-row--vertical">
                        <span className="poodle-range-slider__inline-value poodle-range-slider__inline-value--upper">{blockLayout.upperInline ? upperVisible : ""}</span>
                        <span className="poodle-range-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
                        <span className="poodle-range-slider__inline-value poodle-range-slider__inline-value--lower">{blockLayout.lowerInline ? lowerVisible : ""}</span>
                      </span>
                    </span>
                    <span className="poodle-range-slider__inline poodle-range-slider__inline--remainder-bottom">
                      <span className="poodle-range-slider__inline-row poodle-range-slider__inline-row--vertical">
                        <span className="poodle-range-slider__inline-value poodle-range-slider__inline-value--upper">{blockLayout.upperInline ? upperVisible : ""}</span>
                        <span className="poodle-range-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
                        <span className="poodle-range-slider__inline-value poodle-range-slider__inline-value--lower">{blockLayout.lowerInline ? lowerVisible : ""}</span>
                      </span>
                    </span>
                  </>
                ) : (
                  <>
                    <span className="poodle-range-slider__inline poodle-range-slider__inline--selected">
                      <span className="poodle-range-slider__inline-row">
                        <span className="poodle-range-slider__inline-value poodle-range-slider__inline-value--lower">{blockLayout.lowerInline ? lowerVisible : ""}</span>
                        <span className="poodle-range-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
                        <span className="poodle-range-slider__inline-value poodle-range-slider__inline-value--upper">{blockLayout.upperInline ? upperVisible : ""}</span>
                      </span>
                    </span>
                    <span className="poodle-range-slider__inline poodle-range-slider__inline--remainder-start">
                      <span className="poodle-range-slider__inline-row">
                        <span className="poodle-range-slider__inline-value poodle-range-slider__inline-value--lower">{blockLayout.lowerInline ? lowerVisible : ""}</span>
                        <span className="poodle-range-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
                        <span className="poodle-range-slider__inline-value poodle-range-slider__inline-value--upper">{blockLayout.upperInline ? upperVisible : ""}</span>
                      </span>
                    </span>
                    <span className="poodle-range-slider__inline poodle-range-slider__inline--remainder-end">
                      <span className="poodle-range-slider__inline-row">
                        <span className="poodle-range-slider__inline-value poodle-range-slider__inline-value--lower">{blockLayout.lowerInline ? lowerVisible : ""}</span>
                        <span className="poodle-range-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
                        <span className="poodle-range-slider__inline-value poodle-range-slider__inline-value--upper">{blockLayout.upperInline ? upperVisible : ""}</span>
                      </span>
                    </span>
                  </>
                )}
              </>
            ) : null}
          </span>
          <div className="poodle-range-slider__hit poodle-range-slider__hit--lower" data-part="hit" data-thumb="lower" role="slider" tabIndex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} minimum` : "Minimum value"} aria-valuemin={min} aria-valuemax={displayUpper} aria-valuenow={displayLower} aria-valuetext={lowerValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onKeyDown={(event) => controlKey(event, "lower")} {...pointerHandlers}><span className="poodle-range-slider__thumb" /></div>
          <div className="poodle-range-slider__hit poodle-range-slider__hit--upper" data-part="hit" data-thumb="upper" role="slider" tabIndex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} maximum` : "Maximum value"} aria-valuemin={displayLower} aria-valuemax={safeMax} aria-valuenow={displayUpper} aria-valuetext={upperValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onKeyDown={(event) => controlKey(event, "upper")} {...pointerHandlers}><span className="poodle-range-slider__thumb" /></div>
        </>
      ) : (
        <>
          <span className="poodle-range-slider__track" aria-hidden="true">
            <span className="poodle-range-slider__fill poodle-range-slider__fill--negative" />
            <span className="poodle-range-slider__fill poodle-range-slider__fill--positive" />
            <span className="poodle-range-slider__center" />
          </span>
          <div className="poodle-range-slider__embedded-control poodle-range-slider__embedded-control--lower" role="slider" tabIndex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} minimum` : "Minimum value"} aria-valuemin={min} aria-valuemax={displayUpper} aria-valuenow={displayLower} aria-valuetext={lowerValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onKeyDown={(event) => controlKey(event, "lower")} />
          <div className="poodle-range-slider__embedded-control poodle-range-slider__embedded-control--upper" role="slider" tabIndex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} maximum` : "Maximum value"} aria-valuemin={displayLower} aria-valuemax={safeMax} aria-valuenow={displayUpper} aria-valuetext={upperValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onKeyDown={(event) => controlKey(event, "upper")} />
        </>
      )}
    </div>
  );
}
