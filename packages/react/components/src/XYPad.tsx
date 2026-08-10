import { useRef, useState, type FocusEvent, type KeyboardEvent, type PointerEvent } from "react";
import { createXYPadContext, formatAudioValue, linearValueLaw, xyPadPointToNorm, xyPadTransition, xyPadVisualState, type AudioAutomationState, type AudioValueFormat, type AudioValueLaw, type XYPadContext, type XYPadEffect } from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/xy-pad.css";
import { XYPadVisual } from "./audio/XYPadVisual";
import { useAudioPresentation, type AudioPresentationProps } from "./audio/useAudioPresentation";

export interface XYPadProps extends AudioPresentationProps {
  x?: number; y?: number; minX?: number; maxX?: number; minY?: number; maxY?: number;
  lawX?: AudioValueLaw; lawY?: AudioValueLaw; defaultX?: number; defaultY?: number;
  keyboardStepX?: number; keyboardStepY?: number; formatX?: AudioValueFormat; formatY?: AudioValueFormat;
  automation?: AudioAutomationState; disabled?: boolean; ariaLabel?: string | null;
  onValueChange?: (x: number, y: number) => void; onValueCommit?: (x: number, y: number) => void;
  onGestureBegin?: () => void; onGestureEnd?: () => void;
}

export function XYPad({ size, sizeRole, density, x, y, minX = 0, maxX = 1, minY = 0, maxY = 1, lawX = linearValueLaw, lawY = linearValueLaw, defaultX = 0, defaultY = 0, keyboardStepX = 0.01, keyboardStepY = 0.01, formatX = { type: "number", decimals: 2 }, formatY = { type: "number", decimals: 2 }, automation = "none", disabled = false, ariaLabel = null, onValueChange, onValueCommit, onGestureBegin, onGestureEnd }: XYPadProps) {
  const presentation = useAudioPresentation({ size, sizeRole, density });
  const [uncontrolled, setUncontrolled] = useState({ x: 0, y: 0 });
  const [machine, setMachine] = useState(createXYPadContext);
  const root = useRef<HTMLDivElement>(null);
  const activePointer = useRef<number | null>(null);
  const currentX = x ?? uncontrolled.x;
  const currentY = y ?? uncontrolled.y;
  const context: XYPadContext = { ...machine, x: currentX, y: currentY, minX, maxX, minY, maxY, lawX, lawY, defaultX, defaultY, keyboardStepX, keyboardStepY, automation, disabled };
  const visualState = xyPadVisualState(context);
  function run(effects: XYPadEffect[]) {
    for (const effect of effects) {
      if (effect.type === "emitValueChange" || effect.type === "emitValueCommit") {
        setUncontrolled((current) => ({ x: x === undefined ? effect.x : current.x, y: y === undefined ? effect.y : current.y }));
        if (effect.type === "emitValueChange") onValueChange?.(effect.x, effect.y); else onValueCommit?.(effect.x, effect.y);
      } else if (effect.type === "beginGesture") onGestureBegin?.(); else onGestureEnd?.();
    }
  }
  function send(event: Parameters<typeof xyPadTransition>[1]) { const result = xyPadTransition(context, event); setMachine(result.context); run(result.effects); }
  function norm(event: PointerEvent<HTMLDivElement>) { return xyPadPointToNorm({ x: event.clientX, y: event.clientY }, root.current!.getBoundingClientRect()); }
  function pointerDown(event: PointerEvent<HTMLDivElement>) { if (event.button !== 0 || disabled || !root.current) return; event.preventDefault(); activePointer.current = event.pointerId; root.current.setPointerCapture(event.pointerId); send({ type: "DRAG_BEGIN", ...norm(event), fine: event.shiftKey }); }
  function pointerMove(event: PointerEvent<HTMLDivElement>) { if (activePointer.current === event.pointerId) send({ type: "DRAG_MOVE", ...norm(event), fine: event.shiftKey }); }
  function pointerEnd(event: PointerEvent<HTMLDivElement>) { if (activePointer.current === event.pointerId) { activePointer.current = null; send({ type: "DRAG_END" }); } }
  function axisKeydown(event: KeyboardEvent<HTMLDivElement>, axis: "x" | "y") {
    const negative = axis === "x" ? ["ArrowLeft", "ArrowDown", "PageDown"] : ["ArrowDown", "ArrowLeft", "PageDown"];
    const positive = axis === "x" ? ["ArrowRight", "ArrowUp", "PageUp"] : ["ArrowUp", "ArrowRight", "PageUp"];
    if (negative.includes(event.key) || positive.includes(event.key)) { event.preventDefault(); send({ type: "NUDGE", axis, direction: positive.includes(event.key) ? 1 : -1, multiplier: event.key.startsWith("Page") ? 10 : 1, fine: event.shiftKey }); }
    else if (event.key === "Home" || event.key === "End") { event.preventDefault(); send({ type: "BOUND", axis, bound: event.key === "Home" ? "min" : "max" }); }
  }
  function blur(event: FocusEvent<HTMLDivElement>) { if (!root.current?.contains(event.relatedTarget as Node | null)) send({ type: "FOCUS", value: false }); }
  return <div ref={root} className="poodle-xy-pad" data-size={presentation.size} data-density={presentation.density} role="group" aria-label={ariaLabel ?? undefined} aria-disabled={disabled} data-scope="xy-pad" data-part="root" onPointerDown={pointerDown} onPointerMove={pointerMove} onPointerUp={pointerEnd} onPointerCancel={pointerEnd} onMouseEnter={() => send({ type: "HOVER", value: true })} onMouseLeave={() => send({ type: "HOVER", value: false })} onDoubleClick={() => send({ type: "RESET" })} onFocus={() => send({ type: "FOCUS", value: true })} onBlur={blur}>
    <XYPadVisual visualState={visualState} />
    <div className="poodle-xy-pad__axis" role="slider" tabIndex={disabled ? undefined : 0} aria-label={`${ariaLabel ?? "XY pad"} X`} aria-valuemin={minX} aria-valuemax={maxX} aria-valuenow={currentX} aria-valuetext={formatAudioValue(currentX, formatX)} aria-disabled={disabled} onKeyDown={(event) => axisKeydown(event, "x")} />
    <div className="poodle-xy-pad__axis" role="slider" tabIndex={disabled ? undefined : 0} aria-label={`${ariaLabel ?? "XY pad"} Y`} aria-valuemin={minY} aria-valuemax={maxY} aria-valuenow={currentY} aria-valuetext={formatAudioValue(currentY, formatY)} aria-disabled={disabled} onKeyDown={(event) => axisKeydown(event, "y")} />
  </div>;
}
