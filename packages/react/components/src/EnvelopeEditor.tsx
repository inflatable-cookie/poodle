import { useRef, useState, type FocusEvent, type KeyboardEvent, type MouseEvent, type PointerEvent } from "react";
import { createEnvelopeContext, envelopeHitTest, envelopePointToNorm, envelopeTransition, envelopeVisualState, type EnvelopeContext, type EnvelopeEffect, type EnvelopePoint } from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/envelope-editor.css";
import { EnvelopeVisual } from "./audio/EnvelopeVisual";
import { useAudioPresentation, type AudioPresentationProps } from "./audio/useAudioPresentation";

export interface EnvelopeEditorProps extends AudioPresentationProps {
  points?: EnvelopePoint[]; step?: number; disabled?: boolean; ariaLabel?: string | null;
  snapPoint?: (point: Pick<EnvelopePoint, "x" | "y">, points: EnvelopePoint[]) => Pick<EnvelopePoint, "x" | "y">;
  onPointsChange?: (points: EnvelopePoint[]) => void; onPointsCommit?: (points: EnvelopePoint[]) => void;
  onGestureBegin?: () => void; onGestureEnd?: () => void;
}

export function EnvelopeEditor({ size, sizeRole, density, points, step = 0.01, disabled = false, ariaLabel = null, snapPoint = (point) => point, onPointsChange, onPointsCommit, onGestureBegin, onGestureEnd }: EnvelopeEditorProps) {
  const presentation = useAudioPresentation({ size, sizeRole, density });
  const [uncontrolled, setUncontrolled] = useState<EnvelopePoint[]>([]);
  const [machine, setMachine] = useState(createEnvelopeContext);
  const root = useRef<HTMLDivElement>(null);
  const activePointer = useRef<number | null>(null);
  const pointSequence = useRef(0);
  const currentPoints = points ?? uncontrolled;
  const context: EnvelopeContext = { ...machine, points: currentPoints, step, disabled };
  const visualState = envelopeVisualState(context);
  function run(effects: EnvelopeEffect[]) {
    for (const effect of effects) {
      if (effect.type === "emitPointsChange" || effect.type === "emitPointsCommit") {
        if (points === undefined) setUncontrolled(effect.points);
        if (effect.type === "emitPointsChange") onPointsChange?.(effect.points); else onPointsCommit?.(effect.points);
      } else if (effect.type === "beginGesture") onGestureBegin?.(); else onGestureEnd?.();
    }
  }
  function send(event: Parameters<typeof envelopeTransition>[1]) { const result = envelopeTransition(context, event); setMachine(result.context); run(result.effects); }
  function pointerPoint(event: PointerEvent<HTMLDivElement> | MouseEvent<HTMLDivElement>) {
    const proposed = envelopePointToNorm({ x: event.clientX, y: event.clientY }, root.current!.getBoundingClientRect());
    return snapPoint(proposed, context.points);
  }
  function pointerDown(event: PointerEvent<HTMLDivElement>) {
    if (event.button !== 0 || disabled || !root.current) return;
    const id = envelopeHitTest(context.points, { x: event.clientX, y: event.clientY }, root.current.getBoundingClientRect());
    if (id === null) return;
    event.preventDefault(); activePointer.current = event.pointerId; root.current.setPointerCapture(event.pointerId); send({ type: "DRAG_BEGIN", id });
  }
  function pointerMove(event: PointerEvent<HTMLDivElement>) {
    if (activePointer.current !== null && activePointer.current === event.pointerId) send({ type: "DRAG_MOVE", point: pointerPoint(event) });
    else if (root.current) send({ type: "HOVER_POINT", id: envelopeHitTest(context.points, { x: event.clientX, y: event.clientY }, root.current.getBoundingClientRect()) });
  }
  function pointerEnd(event: PointerEvent<HTMLDivElement>) { if (activePointer.current === event.pointerId) { activePointer.current = null; send({ type: "DRAG_END" }); } }
  function addPoint(event: MouseEvent<HTMLDivElement>) {
    if (disabled || !root.current || envelopeHitTest(context.points, { x: event.clientX, y: event.clientY }, root.current.getBoundingClientRect()) !== null) return;
    event.preventDefault();
    let id: string;
    do id = `envelope-point-${++pointSequence.current}`; while (context.points.some((candidate) => candidate.id === id));
    send({ type: "ADD_POINT", point: { id, ...pointerPoint(event), curve: 0 } });
  }
  function pointKeydown(event: KeyboardEvent<HTMLButtonElement>, id: string) {
    const multiplier = event.key === "PageUp" || event.key === "PageDown" ? 10 : 1;
    let action: Parameters<typeof envelopeTransition>[1] | null = null;
    if (["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown", "PageUp", "PageDown"].includes(event.key)) {
      event.preventDefault(); const vertical = ["ArrowUp", "ArrowDown", "PageUp", "PageDown"].includes(event.key); const positive = ["ArrowRight", "ArrowUp", "PageUp"].includes(event.key);
      action = { type: "NUDGE_SELECTED", axis: vertical ? "y" : "x", direction: positive ? 1 : -1, multiplier, fine: event.shiftKey };
    } else if (event.key === "Delete" || event.key === "Backspace") { event.preventDefault(); action = { type: "REMOVE_SELECTED" }; }
    else if (event.key === "[" || event.key === "]") { event.preventDefault(); action = { type: "NUDGE_CURVE", direction: event.key === "]" ? 1 : -1, fine: event.shiftKey }; }
    const selected = envelopeTransition(context, { type: "SELECT_POINT", id });
    if (action === null) { setMachine(selected.context); run(selected.effects); return; }
    const result = envelopeTransition(selected.context, action);
    setMachine(result.context); run([...selected.effects, ...result.effects]);
  }
  function blur(event: FocusEvent<HTMLDivElement>) { if (!root.current?.contains(event.relatedTarget as Node | null)) send({ type: "FOCUS", value: false }); }
  return <div ref={root} className="poodle-envelope-editor" data-size={presentation.size} data-density={presentation.density} role="group" aria-label={ariaLabel ?? undefined} aria-disabled={disabled} data-scope="envelope-editor" data-part="root" onPointerDown={pointerDown} onPointerMove={pointerMove} onPointerUp={pointerEnd} onPointerCancel={pointerEnd} onPointerLeave={() => send({ type: "HOVER_POINT", id: null })} onDoubleClick={addPoint} onFocus={() => send({ type: "FOCUS", value: true })} onBlur={blur}>
    <EnvelopeVisual visualState={visualState} />
    {visualState.points.map((point, index) => <button key={point.id} type="button" className="poodle-envelope-editor__point-control" style={{ left: `${point.xNorm * 100}%`, top: `${(1 - point.yNorm) * 100}%` }} aria-label={`Point ${index + 1}, X ${Math.round(point.xNorm * 100)} percent, Y ${Math.round(point.yNorm * 100)} percent, curve ${point.curve.toFixed(2)}`} aria-pressed={point.selected} disabled={disabled} onFocus={() => send({ type: "SELECT_POINT", id: point.id })} onKeyDown={(event) => pointKeydown(event, point.id)} />)}
  </div>;
}
