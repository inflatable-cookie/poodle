import { useRef, useState, type CSSProperties, type FormEvent, type KeyboardEvent, type PointerEvent } from "react";
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
import { rangeSliderDefinition } from "./generated/range-slider";

// The definition owns the rendered vocabulary (card 045 R2): the anatomy's
// DOM classes, the eight data-* attribute names, and the seven
// fill-geometry custom properties. A rename in
// packages/codegen/src/models/range_slider.rs moves the DOM here with no
// hand edit; `effigy ir:check` gates drift in the artifact.
const parts = new Map<string, string>(rangeSliderDefinition.parts.map((part) => [part.id, part.className]));
const attributes = new Map<string, string>(rangeSliderDefinition.attributes.map((attribute) => [attribute.id, attribute.name]));
const styleProps = new Map<string, string>(rangeSliderDefinition.styleProps.map((prop) => [prop.id, prop.name]));

function partClass(id: string): string {
  const className = parts.get(id);
  if (!className) throw new Error(`RangeSlider definition has no part '${id}'`);
  return className;
}

function attributeName(id: string): string {
  const name = attributes.get(id);
  if (!name) throw new Error(`RangeSlider definition has no attribute '${id}'`);
  return name;
}

function stylePropName(id: string): string {
  const name = styleProps.get(id);
  if (!name) throw new Error(`RangeSlider definition has no style prop '${id}'`);
  return name;
}

const rootClass = partClass("root");
const trackClass = partClass("track");
const fillNegativeClass = partClass("fill-negative");
const fillPositiveClass = partClass("fill-positive");
const centerClass = partClass("center");
const controlLowerClass = partClass("control-lower");
const controlUpperClass = partClass("control-upper");
const embeddedLowerClass = partClass("embedded-lower");
const embeddedUpperClass = partClass("embedded-upper");

const dataOrientation = attributeName("orientation");
const dataDisabled = attributeName("disabled");
const dataVariant = attributeName("variant");
const dataPolarity = attributeName("polarity");
const dataFillSplit = attributeName("fill-split");
const dataState = attributeName("state");
const dataSize = attributeName("size");
const dataDensity = attributeName("density");

const styleRangeStart = stylePropName("range-start");
const styleRangeEnd = stylePropName("range-end");
const styleRangeCenter = stylePropName("range-center");
const styleNegativeStart = stylePropName("range-negative-start");
const styleNegativeSpan = stylePropName("range-negative-span");
const stylePositiveStart = stylePropName("range-positive-start");
const stylePositiveSpan = stylePropName("range-positive-span");

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
  // The eight data-* attributes: names come from the definition's
  // attributes, values are the runtime's projection (CROSS-13).
  const dataAttributes: Record<string, unknown> = {
    [dataOrientation]: orientation,
    [dataDisabled]: disabled,
    [dataVariant]: variant,
    [dataPolarity]: visualState.polarity,
    [dataFillSplit]: visualState.fillSplitAtCenter,
    [dataState]: visualState.pointerActive ? "active" : "idle",
    [dataSize]: resolvedSize,
    [dataDensity]: resolvedDensity,
  };
  // The fill geometry (RNG-17): the property names come from the
  // definition's styleProps; the values are the machine's visual-state
  // numbers projected to percentages (CROSS-14, IR-06).
  const rangeStyle = {
    [styleRangeStart]: `${lowerPercent}%`,
    [styleRangeEnd]: `${upperPercent}%`,
    [styleRangeCenter]: `${visualState.centerNorm * 100}%`,
    [styleNegativeStart]: `${visualState.negativeFillStartNorm * 100}%`,
    [styleNegativeSpan]: `${visualState.negativeFillSpanNorm * 100}%`,
    [stylePositiveStart]: `${visualState.positiveFillStartNorm * 100}%`,
    [stylePositiveSpan]: `${visualState.positiveFillSpanNorm * 100}%`,
  } as CSSProperties;

  function send(type: "INPUT" | "COMMIT", thumb: "lower" | "upper", event: FormEvent<HTMLInputElement>): void {
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
      className={rootClass}
      {...dataAttributes}
      style={rangeStyle}
      onPointerDown={pointerDown} onPointerMove={pointerMove} onPointerUp={pointerEnd} onPointerCancel={pointerEnd}
    >
      <span className={trackClass} aria-hidden="true">
        <span className={fillNegativeClass} />
        <span className={fillPositiveClass} />
        <span className={centerClass} />
      </span>

      {variant === "standard" && <><input
        className={controlLowerClass}
        type="range"
        min={min}
        max={safeMax}
        step={step}
        value={displayLower}
        disabled={disabled}
        aria-label={ariaLabel ? `${ariaLabel} minimum` : "Minimum value"}
        aria-valuetext={lowerValueText ?? undefined}
        onInput={(event) => send("INPUT", "lower", event)}
        onChange={(event) => send("COMMIT", "lower", event)}
      />

      <input
        className={controlUpperClass}
        type="range"
        min={min}
        max={safeMax}
        step={step}
        value={displayUpper}
        disabled={disabled}
        aria-label={ariaLabel ? `${ariaLabel} maximum` : "Maximum value"}
        aria-valuetext={upperValueText ?? undefined}
        onInput={(event) => send("INPUT", "upper", event)}
        onChange={(event) => send("COMMIT", "upper", event)}
      /></>}
      {variant === "embedded" && <>
        <div className={embeddedLowerClass} role="slider" tabIndex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} minimum` : "Minimum value"} aria-valuemin={min} aria-valuemax={displayUpper} aria-valuenow={displayLower} aria-valuetext={lowerValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onKeyDown={(event) => embeddedKey(event, "lower")} />
        <div className={embeddedUpperClass} role="slider" tabIndex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} maximum` : "Maximum value"} aria-valuemin={displayLower} aria-valuemax={safeMax} aria-valuenow={displayUpper} aria-valuetext={upperValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onKeyDown={(event) => embeddedKey(event, "upper")} />
      </>}
    </div>
  );
}
