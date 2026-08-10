import { useEffect, useRef, useState, type KeyboardEvent, type PointerEvent } from "react";
import { audioValueText, createDragNumberContext, dragNumberTransition, dragNumberVisualState, type AudioValueEffect, type AudioValueFormat, type AudioValueLaw, type DragNumberContext } from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/drag-number-field.css";
import { ValueVisual } from "./audio/ValueVisual";
import { useAudioPresentation, type AudioPresentationProps } from "./audio/useAudioPresentation";

export interface DragNumberFieldProps extends AudioPresentationProps {
  value?: number; min?: number; max?: number; step?: number; dragSensitivity?: number;
  format?: AudioValueFormat; disabled?: boolean; ariaLabel?: string | null;
  onValueChange?: (value: number) => void; onValueCommit?: (value: number) => void;
  onGestureBegin?: () => void; onGestureEnd?: () => void;
}

export function DragNumberField({ size, sizeRole, density, value, min = Number.MIN_SAFE_INTEGER, max = Number.MAX_SAFE_INTEGER, step = 1, dragSensitivity = 0.1, format = { type: "number", decimals: 2 }, disabled = false, ariaLabel = null, onValueChange, onValueCommit, onGestureBegin, onGestureEnd }: DragNumberFieldProps) {
  const presentation = useAudioPresentation({ size, sizeRole, density });
  const [uncontrolled, setUncontrolled] = useState(0);
  const [machine, setMachine] = useState(createDragNumberContext);
  const [entryDraft, setEntryDraft] = useState("");
  const root = useRef<HTMLDivElement>(null);
  const entry = useRef<HTMLInputElement>(null);
  const activePointer = useRef<number | null>(null);
  const pointerStart = useRef(0);
  const dragging = useRef(false);
  const skipEntryBlur = useRef(false);
  const currentValue = value ?? uncontrolled;
  const law: AudioValueLaw = { type: "stepped", step };
  const context: DragNumberContext = { ...machine, value: currentValue, min, max, law, defaultValue: currentValue, keyboardStep: step, format, dragSensitivity, disabled };
  const visualState = dragNumberVisualState(context);
  const valueText = audioValueText(context);
  useEffect(() => { if (context.entryOpen) { entry.current?.focus(); entry.current?.select(); } }, [context.entryOpen]);
  function run(effects: AudioValueEffect[]) {
    for (const effect of effects) {
      if (effect.type === "emitValueChange") { if (value === undefined) setUncontrolled(effect.value); onValueChange?.(effect.value); }
      else if (effect.type === "emitValueCommit") { if (value === undefined) setUncontrolled(effect.value); onValueCommit?.(effect.value); }
      else if (effect.type === "beginGesture") onGestureBegin?.();
      else if (effect.type === "endGesture") onGestureEnd?.();
      else if (effect.type === "requestEntryFocus") setEntryDraft(valueText);
    }
  }
  function send(event: Parameters<typeof dragNumberTransition>[1]) { const result = dragNumberTransition(context, event); setMachine(result.context); run(result.effects); }
  function pointerDown(event: PointerEvent<HTMLDivElement>) { if (event.button !== 0 || disabled || !root.current) return; event.preventDefault(); activePointer.current = event.pointerId; pointerStart.current = event.clientX; dragging.current = false; root.current.setPointerCapture(event.pointerId); }
  function pointerMove(event: PointerEvent<HTMLDivElement>) {
    if (activePointer.current !== event.pointerId) return;
    if (!dragging.current) {
      if (Math.abs(event.clientX - pointerStart.current) < 2) return;
      dragging.current = true;
      const begun = dragNumberTransition(context, { type: "DRAG_BEGIN", position: pointerStart.current, fine: event.shiftKey });
      const moved = dragNumberTransition(begun.context, { type: "DRAG_MOVE", position: event.clientX, fine: event.shiftKey });
      setMachine(moved.context); run([...begun.effects, ...moved.effects]); return;
    }
    send({ type: "DRAG_MOVE", position: event.clientX, fine: event.shiftKey });
  }
  function pointerEnd(event: PointerEvent<HTMLDivElement>, openEntry = true) {
    if (activePointer.current !== event.pointerId) return;
    activePointer.current = null;
    if (dragging.current) send({ type: "DRAG_END" });
    else if (openEntry) send({ type: "ENTRY_OPEN" });
    dragging.current = false;
  }
  function keydown(event: KeyboardEvent<HTMLDivElement>) {
    if (event.key === "Enter") { event.preventDefault(); send({ type: "ENTRY_OPEN" }); }
    else if (["ArrowDown", "ArrowLeft", "ArrowUp", "ArrowRight"].includes(event.key)) { event.preventDefault(); send({ type: "KEY_NUDGE", direction: event.key === "ArrowDown" || event.key === "ArrowLeft" ? -1 : 1, fine: event.shiftKey }); }
    else if (event.key === "Home" || event.key === "End") { event.preventDefault(); send({ type: "KEY_BOUND", bound: event.key === "Home" ? "min" : "max" }); }
  }
  return <div ref={root} className="poodle-drag-number-field" data-size={presentation.size} data-density={presentation.density} role="spinbutton" tabIndex={disabled ? undefined : 0} aria-label={ariaLabel ?? undefined} aria-valuemin={Number.isSafeInteger(min) ? min : undefined} aria-valuemax={Number.isSafeInteger(max) ? max : undefined} aria-valuenow={currentValue} aria-valuetext={valueText} aria-disabled={disabled} data-scope="drag-number-field" data-part="root" data-state={visualState.drag === "none" ? "idle" : visualState.drag} onPointerDown={pointerDown} onPointerMove={pointerMove} onPointerUp={pointerEnd} onPointerCancel={(event) => pointerEnd(event, false)} onMouseEnter={() => send({ type: "HOVER", value: true })} onMouseLeave={() => send({ type: "HOVER", value: false })} onFocus={() => send({ type: "FOCUS", value: true })} onBlur={() => send({ type: "FOCUS", value: false })} onKeyDown={keydown}>
    <ValueVisual visualState={visualState} text={valueText} kind="drag-number" />
    {context.entryOpen && <input ref={entry} className="poodle-drag-number-field__entry" aria-label={`${ariaLabel ?? "Number"} value`} value={entryDraft} onChange={(event) => setEntryDraft(event.currentTarget.value)} onKeyDown={(event) => {
      if (event.key === "Enter") { event.preventDefault(); skipEntryBlur.current = true; send({ type: "ENTRY_COMMIT", text: entryDraft }); root.current?.focus(); }
      else if (event.key === "Escape") { event.preventDefault(); skipEntryBlur.current = true; send({ type: "ENTRY_CANCEL" }); root.current?.focus(); }
    }} onBlur={() => { if (skipEntryBlur.current) skipEntryBlur.current = false; else send({ type: "ENTRY_COMMIT", text: entryDraft }); }} />}
  </div>;
}
