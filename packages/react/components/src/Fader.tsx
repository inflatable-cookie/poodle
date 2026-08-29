import { useEffect, useRef, useState, type KeyboardEvent, type PointerEvent, type WheelEvent } from "react";
import { audioValueText, createFaderContext, faderPointToNorm, faderTransition, faderVisualState, normalizeAudioValue, type AudioAutomationState, type AudioValueEffect, type AudioValueFormat, type AudioValueLaw, type FaderContext } from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/fader.css";
import { FaderVisual } from "./audio/FaderVisual";
import { useAudioPresentation, type AudioPresentationProps } from "./audio/useAudioPresentation";

export interface FaderProps extends AudioPresentationProps {
  value?: number; min?: number; max?: number; law?: AudioValueLaw;
  orientation?: "horizontal" | "vertical"; detents?: number[]; detentSnap?: number;
  defaultValue?: number; keyboardStep?: number; format?: AudioValueFormat;
  automation?: AudioAutomationState; disabled?: boolean; ariaLabel?: string | null;
  onValueChange?: (value: number) => void; onValueCommit?: (value: number) => void;
  onGestureBegin?: () => void; onGestureEnd?: () => void;
}

export function Fader({ size, sizeRole, density, value, min = 0, max = 1, law = { type: "linear" }, orientation = "vertical", detents = [], detentSnap = 0.015, defaultValue = 0, keyboardStep = 0.01, format = { type: "number", decimals: 2 }, automation = "none", disabled = false, ariaLabel = null, onValueChange, onValueCommit, onGestureBegin, onGestureEnd }: FaderProps) {
  const presentation = useAudioPresentation({ size, sizeRole, density });
  const [uncontrolled, setUncontrolled] = useState(0);
  const [machine, setMachine] = useState(createFaderContext);
  const [entryDraft, setEntryDraft] = useState("");
  const root = useRef<HTMLDivElement>(null);
  const entry = useRef<HTMLInputElement>(null);
  const activePointer = useRef<number | null>(null);
  const skipEntryBlur = useRef(false);
  const cancelOnUnmount = useRef<(pointerId?: number | null) => void>(() => {});
  const currentValue = value ?? uncontrolled;
  const context: FaderContext = { ...machine, value: currentValue, min, max, law, orientation, detents, detentSnap, defaultValue, keyboardStep, format, automation, disabled };
  const visualState = faderVisualState(context);
  const valueText = audioValueText(context);
  const detentNorms = detents.map((detent) => normalizeAudioValue(detent, min, max, law));
  useEffect(() => { if (context.entryOpen) { entry.current?.focus(); entry.current?.select(); } }, [context.entryOpen]);
  function run(effects: AudioValueEffect[]) {
    for (const effect of effects) {
      if (effect.type === "emitValueChange") { if (value === undefined) setUncontrolled(effect.value); onValueChange?.(effect.value); }
      else if (effect.type === "emitValueCommit") { if (value === undefined) setUncontrolled(effect.value); onValueCommit?.(effect.value); }
      else if (effect.type === "beginGesture") onGestureBegin?.();
      else if (effect.type === "endGesture") onGestureEnd?.();
      else if (effect.type === "requestEntryFocus") { skipEntryBlur.current = false; setEntryDraft(valueText); }
    }
  }
  function send(event: Parameters<typeof faderTransition>[1]) { const result = faderTransition(context, event); setMachine(result.context); run(result.effects); }
  function pointNorm(event: PointerEvent<HTMLDivElement>) { return faderPointToNorm({ x: event.clientX, y: event.clientY }, root.current!.getBoundingClientRect(), orientation); }
  function pointerDown(event: PointerEvent<HTMLDivElement>) {
    // One primary pointer owns the gesture. A second pointer-down cannot
    // replace the active pointer or open a second gesture.
    if (event.button !== 0 || disabled || activePointer.current !== null || !root.current) return;
    event.preventDefault(); activePointer.current = event.pointerId; root.current.setPointerCapture(event.pointerId);
    const valueNorm = pointNorm(event);
    const begun = faderTransition(context, { type: "DRAG_BEGIN", position: valueNorm, fine: event.shiftKey });
    const moved = faderTransition(begun.context, { type: "DRAG_SET_NORM", valueNorm, fine: event.shiftKey });
    setMachine(moved.context); run([...begun.effects, ...moved.effects]);
  }
  function pointerMove(event: PointerEvent<HTMLDivElement>) { if (activePointer.current === event.pointerId) send({ type: "DRAG_SET_NORM", valueNorm: pointNorm(event), fine: event.shiftKey }); }
  function pointerUp(event: PointerEvent<HTMLDivElement>) { if (activePointer.current === event.pointerId) { activePointer.current = null; send({ type: "DRAG_END" }); } }
  /**
   * Pointer cancel, lost capture, and teardown all close the gesture the same
   * way, so a captured gesture can never outlive its pointer or its component.
   * A stale pointer id is ignored, and the machine makes a repeat inert.
   */
  function cancelGesture(pointerId: number | null = null) {
    if (activePointer.current === null || (pointerId !== null && activePointer.current !== pointerId)) return;
    activePointer.current = null; send({ type: "DRAG_CANCEL" });
  }
  cancelOnUnmount.current = cancelGesture;
  useEffect(() => () => cancelOnUnmount.current(), []);
  function keydown(event: KeyboardEvent<HTMLDivElement>) {
    if (event.key === "Enter") { event.preventDefault(); send({ type: "ENTRY_OPEN" }); return; }
    const directions: Record<string, -1 | 1> = { ArrowLeft: -1, ArrowDown: -1, ArrowRight: 1, ArrowUp: 1, PageDown: -1, PageUp: 1 };
    const direction = directions[event.key];
    if (direction) { event.preventDefault(); send({ type: "KEY_NUDGE", direction, multiplier: event.key.startsWith("Page") ? 10 : 1, fine: event.shiftKey }); }
    else if (event.key === "Home" || event.key === "End") { event.preventDefault(); send({ type: "KEY_BOUND", bound: event.key === "Home" ? "min" : "max" }); }
  }
  function wheel(event: WheelEvent<HTMLDivElement>) { event.preventDefault(); send({ type: "WHEEL", direction: event.deltaY < 0 ? 1 : -1, fine: event.shiftKey }); }
  return <div ref={root} className="poodle-fader" data-size={presentation.size} data-density={presentation.density} role="slider" tabIndex={disabled ? undefined : 0} aria-label={ariaLabel ?? undefined} aria-orientation={orientation} aria-valuemin={min} aria-valuemax={max} aria-valuenow={currentValue} aria-valuetext={valueText} aria-disabled={disabled} data-scope="fader" data-part="root" data-orientation={orientation} data-state={visualState.drag === "none" ? "idle" : visualState.drag} onPointerDown={pointerDown} onPointerMove={pointerMove} onPointerUp={pointerUp} onPointerCancel={(event) => cancelGesture(event.pointerId)} onLostPointerCapture={(event) => cancelGesture(event.pointerId)} onMouseEnter={() => send({ type: "HOVER", value: true })} onMouseLeave={() => send({ type: "HOVER", value: false })} onFocus={() => send({ type: "FOCUS", value: true })} onBlur={() => send({ type: "FOCUS", value: false })} onWheel={wheel} onDoubleClick={(event) => { event.preventDefault(); send({ type: "RESET" }); }} onKeyDown={keydown}>
    <FaderVisual visualState={visualState} orientation={orientation} detents={detentNorms} />
    {context.entryOpen && <input ref={entry} className="poodle-fader__entry" aria-label={`${ariaLabel ?? "Fader"} value`} value={entryDraft} onChange={(event) => setEntryDraft(event.currentTarget.value)} onKeyDown={(event) => {
      if (event.key === "Enter") { event.preventDefault(); skipEntryBlur.current = true; send({ type: "ENTRY_COMMIT", text: entryDraft }); root.current?.focus(); }
      else if (event.key === "Escape") { event.preventDefault(); skipEntryBlur.current = true; send({ type: "ENTRY_CANCEL" }); root.current?.focus(); }
    }} onBlur={() => { if (skipEntryBlur.current) skipEntryBlur.current = false; else send({ type: "ENTRY_COMMIT", text: entryDraft }); }} />}
  </div>;
}
