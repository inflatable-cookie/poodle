import { useRef, useState, type KeyboardEvent, type PointerEvent } from "react";
import { audioSwitchTransition, audioSwitchVisualState, createAudioSwitchContext, type AudioSwitchContext, type AudioSwitchEffect, type AudioSwitchMode } from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/audio-switch.css";
import { AudioSwitchVisual } from "./audio/AudioSwitchVisual";
import { useAudioPresentation, type AudioPresentationProps } from "./audio/useAudioPresentation";

export interface AudioSwitchProps extends AudioPresentationProps {
  mode?: AudioSwitchMode;
  state?: number;
  stateCount?: number;
  lampOn?: boolean | null;
  stateLabels?: string[];
  disabled?: boolean;
  ariaLabel?: string | null;
  onStateChange?: (state: number) => void;
  onStateCommit?: (state: number) => void;
}

export function AudioSwitch({ size, sizeRole, density, mode = "latch", state, stateCount = 2, lampOn = null, stateLabels = [], disabled = false, ariaLabel = "Audio switch", onStateChange, onStateCommit }: AudioSwitchProps) {
  const presentation = useAudioPresentation({ size, sizeRole, density });
  const [uncontrolled, setUncontrolled] = useState(0);
  const [machine, setMachine] = useState(createAudioSwitchContext);
  const activePointer = useRef<number | null>(null);
  const count = Math.max(Math.floor(Number.isFinite(stateCount) ? stateCount : 2), 2);
  const rawState = state ?? uncontrolled;
  const currentState = Math.min(Math.max(Math.round(Number.isFinite(rawState) ? rawState : 0), 0), count - 1);
  const context: AudioSwitchContext = { ...machine, mode, state: currentState, stateCount: count, lampOn, disabled };
  const visualState = audioSwitchVisualState(context);
  const stateText = stateLabels[currentState] ?? `State ${currentState + 1} of ${count}`;
  const accessibleLabel = mode === "multi" ? `${ariaLabel ?? "Audio switch"}, ${stateText}` : ariaLabel ?? "Audio switch";
  function run(effects: AudioSwitchEffect[]) {
    for (const effect of effects) {
      if (state === undefined) setUncontrolled(effect.state);
      if (effect.type === "emitStateChange") onStateChange?.(effect.state);
      else onStateCommit?.(effect.state);
    }
  }
  function send(event: Parameters<typeof audioSwitchTransition>[1]) { const result = audioSwitchTransition(context, event); setMachine(result.context); run(result.effects); }
  function pointerDown(event: PointerEvent<HTMLButtonElement>) { if (event.button !== 0 || disabled) return; event.preventDefault(); activePointer.current = event.pointerId; event.currentTarget.setPointerCapture(event.pointerId); send({ type: "PRESS" }); }
  function pointerUp(event: PointerEvent<HTMLButtonElement>) { if (activePointer.current !== event.pointerId) return; activePointer.current = null; send({ type: "RELEASE" }); }
  function key(event: KeyboardEvent<HTMLButtonElement>, down: boolean) { if (event.key !== " " && event.key !== "Enter") return; event.preventDefault(); send({ type: down ? "PRESS" : "RELEASE" }); }
  return <button type="button" className="poodle-audio-switch" data-size={presentation.size} data-density={presentation.density} aria-label={accessibleLabel} aria-pressed={mode === "multi" ? undefined : currentState > 0} disabled={disabled} data-scope="audio-switch" data-part="root" data-mode={mode} onPointerDown={pointerDown} onPointerUp={pointerUp} onPointerCancel={(event) => { if (activePointer.current === event.pointerId) { activePointer.current = null; send({ type: "CANCEL" }); } }} onMouseEnter={() => send({ type: "HOVER", value: true })} onMouseLeave={() => send({ type: "HOVER", value: false })} onFocus={() => send({ type: "FOCUS", value: true })} onBlur={() => { send({ type: "FOCUS", value: false }); send({ type: "CANCEL" }); }} onKeyDown={(event) => key(event, true)} onKeyUp={(event) => key(event, false)}>
    <AudioSwitchVisual visualState={visualState} />
  </button>;
}
