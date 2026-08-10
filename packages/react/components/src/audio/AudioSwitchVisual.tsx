import type { CSSProperties } from "react";
import type { AudioSwitchVisualState } from "@inflatable-cookie/poodle-core";

export function AudioSwitchVisual({ visualState }: { visualState: AudioSwitchVisualState }) {
  const position = visualState.stateCount <= 1 ? 0 : visualState.state / (visualState.stateCount - 1);
  return <span className="poodle-audio-switch-visual" data-state={visualState.state} data-pressed={visualState.pressed} data-lamp={visualState.lampOn} data-hover={visualState.hover} data-focus={visualState.focus} data-enabled={visualState.enabled} aria-hidden="true" style={{ "--poodle-audio-switch-position": position } as CSSProperties}>
    <span className="poodle-audio-switch-visual__body"><span className="poodle-audio-switch-visual__handle" /></span>
    <span className="poodle-audio-switch-visual__lamp" />
  </span>;
}
