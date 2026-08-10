import type { CSSProperties } from "react";
import type { AudioMeterVisualState } from "@inflatable-cookie/poodle-core";

export function AudioMeterVisual({ visualState, style = "bar", orientation = "vertical", segments = 20 }: { visualState: AudioMeterVisualState; style?: "bar" | "segments"; orientation?: "horizontal" | "vertical"; segments?: number }) {
  const count = Math.max(segments, 1);
  return <span className="poodle-audio-meter-visual" data-style={style} data-orientation={orientation} data-enabled={visualState.enabled} aria-hidden="true" style={{ "--poodle-audio-meter-value": visualState.ballisticValue, "--poodle-audio-meter-peak": visualState.peakHold ?? 0 } as CSSProperties}>
    <span className="poodle-audio-meter-visual__track">
      {style === "segments" ? Array.from({ length: count }, (_, segment) => <span key={segment} className="poodle-audio-meter-visual__segment" data-active={(segment + 1) / count <= visualState.ballisticValue} data-warning={segment / count >= 0.75} data-clip={segment / count >= 0.95} />) : <span className="poodle-audio-meter-visual__bar" />}
      {visualState.peakHold !== null && <span className="poodle-audio-meter-visual__peak" />}
    </span>
    <span className="poodle-audio-meter-visual__clip" data-active={visualState.clip} />
  </span>;
}
