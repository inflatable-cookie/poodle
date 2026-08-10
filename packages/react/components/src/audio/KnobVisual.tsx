import type { CSSProperties } from "react";
import type { AudioControlVisualState } from "@inflatable-cookie/poodle-core";

export function KnobVisual({ visualState }: { visualState: AudioControlVisualState }) {
  const sweep = visualState.valueNorm * 270;
  const style = {
    "--poodle-knob-sweep": `${sweep}deg`,
    "--poodle-knob-indicator-rotation": `${-135 + sweep}deg`,
  } as CSSProperties;
  return <span className="poodle-knob-visual" data-hover={visualState.hover} data-focus={visualState.focus} data-drag={visualState.drag} data-enabled={visualState.enabled} aria-hidden="true" style={style}>
    <span className="poodle-knob-visual__track" />
    <span className="poodle-knob-visual__arc" />
    <span className="poodle-knob-visual__indicator" />
  </span>;
}
