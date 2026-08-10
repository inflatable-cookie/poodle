import type { CSSProperties } from "react";
import type { AudioControlVisualState } from "@inflatable-cookie/poodle-core";

export function FaderVisual({ visualState, orientation, detents }: { visualState: AudioControlVisualState; orientation: "horizontal" | "vertical"; detents: number[] }) {
  return <span className="poodle-fader-visual" data-orientation={orientation} data-hover={visualState.hover} data-focus={visualState.focus} data-drag={visualState.drag} data-enabled={visualState.enabled} aria-hidden="true" style={{ "--poodle-fader-value": visualState.valueNorm } as CSSProperties}>
    <span className="poodle-fader-visual__track" />
    <span className="poodle-fader-visual__fill" />
    {detents.map((detent, index) => <span key={`${detent}-${index}`} className="poodle-fader-visual__detent" style={{ "--poodle-fader-detent": detent } as CSSProperties} />)}
    <span className="poodle-fader-visual__thumb" />
  </span>;
}
