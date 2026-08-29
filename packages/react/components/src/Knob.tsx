import { useEffect, useRef, useState, type KeyboardEvent, type PointerEvent, type WheelEvent } from "react";
import { audioValueText, createKnobContext, hitTestCircle, knobPointToNorm, knobTransition, knobVisualState, type AudioAutomationState, type AudioValueEffect, type AudioValueFormat, type AudioValueLaw, type KnobContext } from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/knob.css";
import { KnobVisual } from "./audio/KnobVisual";
import { useAudioPresentation, type AudioPresentationProps } from "./audio/useAudioPresentation";

export interface KnobProps extends AudioPresentationProps {
  value?: number; min?: number; max?: number; law?: AudioValueLaw;
  defaultValue?: number; dragMode?: "vertical" | "circular";
  dragSensitivity?: number; keyboardStep?: number; format?: AudioValueFormat;
  automation?: AudioAutomationState; disabled?: boolean; ariaLabel?: string | null;
  onValueChange?: (value: number) => void; onValueCommit?: (value: number) => void;
  onGestureBegin?: () => void; onGestureEnd?: () => void;
}

export function Knob({ size, sizeRole, density, value, min = 0, max = 1, law = { type: "linear" }, defaultValue = 0, dragMode = "vertical", dragSensitivity = 160, keyboardStep = 0.01, format = { type: "number", decimals: 2 }, automation = "none", disabled = false, ariaLabel = null, onValueChange, onValueCommit, onGestureBegin, onGestureEnd }: KnobProps) {
  const presentation = useAudioPresentation({ size, sizeRole, density });
  const [uncontrolled, setUncontrolled] = useState(0);
  const [machine, setMachine] = useState(createKnobContext);
  const [entryDraft, setEntryDraft] = useState("");
  const root = useRef<HTMLDivElement>(null);
  const entry = useRef<HTMLInputElement>(null);
  const activePointer = useRef<number | null>(null);
  const skipEntryBlur = useRef(false);
  const cancelOnUnmount = useRef<(pointerId?: number | null) => void>(() => {});
  /**
   * The machine state this adapter last produced, written before any host
   * callback runs. Terminal cleanup reads it rather than a render's `context`,
   * because a host that unmounts the control from inside `onGestureBegin` or
   * `onValueChange` tears down before React commits the render that opened the
   * gesture.
   */
  const live = useRef<KnobContext | null>(null);
  const currentValue = value ?? uncontrolled;
  const context: KnobContext = { ...machine, value: currentValue, min, max, law, defaultValue, dragMode, dragSensitivity, keyboardStep, format, automation, disabled };
  const visualState = knobVisualState(context);
  const valueText = audioValueText(context);
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
  function commit(result: { context: KnobContext; effects: Parameters<typeof run>[0] }) {
    live.current = result.context;
    setMachine(result.context);
    run(result.effects);
  }
  function send(event: Parameters<typeof knobTransition>[1]) { commit(knobTransition(context, event)); }
  /** Terminals resolve from the live snapshot, never from a render. */
  function terminate(type: "DRAG_END" | "DRAG_CANCEL") { commit(knobTransition(live.current ?? context, { type })); }
  function pointerDown(event: PointerEvent<HTMLDivElement>) {
    // One primary pointer owns the gesture. A second pointer-down cannot
    // replace the active pointer or open a second gesture.
    if (event.button !== 0 || disabled || activePointer.current !== null || !root.current) return;
    const rect = root.current.getBoundingClientRect();
    if (!hitTestCircle({ x: event.clientX, y: event.clientY }, rect)) return;
    event.preventDefault(); activePointer.current = event.pointerId; root.current.setPointerCapture(event.pointerId);
    const circularNorm = knobPointToNorm({ x: event.clientX, y: event.clientY }, rect);
    const begun = knobTransition(context, { type: "DRAG_BEGIN", position: dragMode === "circular" ? circularNorm : event.clientY, fine: event.shiftKey });
    if (dragMode === "circular") {
      const moved = knobTransition(begun.context, { type: "DRAG_SET_NORM", valueNorm: circularNorm, fine: event.shiftKey });
      commit({ context: moved.context, effects: [...begun.effects, ...moved.effects] });
    } else {
      commit(begun);
    }
  }
  function pointerMove(event: PointerEvent<HTMLDivElement>) {
    if (activePointer.current !== event.pointerId || !root.current) return;
    if (dragMode === "circular") send({ type: "DRAG_SET_NORM", valueNorm: knobPointToNorm({ x: event.clientX, y: event.clientY }, root.current.getBoundingClientRect()), fine: event.shiftKey });
    else send({ type: "DRAG_MOVE", position: event.clientY, fine: event.shiftKey });
  }
  function pointerUp(event: PointerEvent<HTMLDivElement>) { if (activePointer.current === event.pointerId) { activePointer.current = null; terminate("DRAG_END"); } }
  /**
   * Pointer cancel, lost capture, and teardown all close the gesture the same
   * way, so a captured gesture can never outlive its pointer or its component.
   * A stale pointer id is ignored, and the machine makes a repeat inert.
   */
  function cancelGesture(pointerId: number | null = null) {
    if (activePointer.current === null || (pointerId !== null && activePointer.current !== pointerId)) return;
    activePointer.current = null; terminate("DRAG_CANCEL");
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
  return <div ref={root} className="poodle-knob" data-size={presentation.size} data-density={presentation.density} role="slider" tabIndex={disabled ? undefined : 0} aria-label={ariaLabel ?? undefined} aria-valuemin={min} aria-valuemax={max} aria-valuenow={currentValue} aria-valuetext={valueText} aria-disabled={disabled} data-scope="knob" data-part="root" data-state={visualState.drag === "none" ? "idle" : visualState.drag} onPointerDown={pointerDown} onPointerMove={pointerMove} onPointerUp={pointerUp} onPointerCancel={(event) => cancelGesture(event.pointerId)} onLostPointerCapture={(event) => cancelGesture(event.pointerId)} onMouseEnter={() => send({ type: "HOVER", value: true })} onMouseLeave={() => send({ type: "HOVER", value: false })} onFocus={() => send({ type: "FOCUS", value: true })} onBlur={() => send({ type: "FOCUS", value: false })} onWheel={wheel} onDoubleClick={(event) => { event.preventDefault(); send({ type: "RESET" }); }} onKeyDown={keydown}>
    <KnobVisual visualState={visualState} />
    {context.entryOpen && <input ref={entry} className="poodle-knob__entry" aria-label={`${ariaLabel ?? "Knob"} value`} value={entryDraft} onChange={(event) => setEntryDraft(event.currentTarget.value)} onKeyDown={(event) => {
      if (event.key === "Enter") { event.preventDefault(); skipEntryBlur.current = true; send({ type: "ENTRY_COMMIT", text: entryDraft }); root.current?.focus(); }
      else if (event.key === "Escape") { event.preventDefault(); skipEntryBlur.current = true; send({ type: "ENTRY_CANCEL" }); root.current?.focus(); }
    }} onBlur={() => { if (skipEntryBlur.current) skipEntryBlur.current = false; else send({ type: "ENTRY_COMMIT", text: entryDraft }); }} />}
  </div>;
}
