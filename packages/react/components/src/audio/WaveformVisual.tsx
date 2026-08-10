import type { CSSProperties } from "react";
import type { WaveformVisualState } from "@inflatable-cookie/poodle-core";

export function WaveformVisual({ visualState }: { visualState: WaveformVisualState }) {
  const span = Math.max(visualState.visibleEnd - visualState.visibleStart, 1);
  return <span className="poodle-waveform-display-visual" data-focus={visualState.focus} data-enabled={visualState.enabled} aria-hidden="true">
    {visualState.columns.map((column, index) => <span key={index} className="poodle-waveform-display-visual__column" style={{ "--poodle-wave-min": column.min, "--poodle-wave-max": column.max } as CSSProperties} />)}
    {visualState.selection && <span className="poodle-waveform-display-visual__selection" style={{ left: `${(visualState.selection.start - visualState.visibleStart) / span * 100}%`, width: `${(visualState.selection.end - visualState.selection.start + 1) / span * 100}%` }} />}
    {visualState.cursorSample !== null && <span className="poodle-waveform-display-visual__cursor" style={{ left: `${(visualState.cursorSample - visualState.visibleStart) / span * 100}%` }} />}
  </span>;
}
