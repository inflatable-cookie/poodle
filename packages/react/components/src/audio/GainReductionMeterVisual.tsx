import type { CSSProperties } from "react";
import type { GainReductionMeterVisualState } from "@inflatable-cookie/poodle-core";

export function GainReductionMeterVisual({ visualState, style = "segments", orientation = "vertical", segments = 20 }: { visualState: GainReductionMeterVisualState; style?: "bar" | "segments"; orientation?: "horizontal" | "vertical"; segments?: number }) {
  const count = Math.max(segments, 1);
  return <span className="poodle-gain-reduction-meter-visual" data-style={style} data-orientation={orientation} data-enabled={visualState.enabled} aria-hidden="true" style={{ "--poodle-gain-reduction-value": visualState.ballisticValue } as CSSProperties}>
    <span className="poodle-gain-reduction-meter-visual__track">
      {style === "segments" ? Array.from({ length: count }, (_, segment) => <span key={segment} className="poodle-gain-reduction-meter-visual__segment" data-active={(segment + 1) / count <= visualState.ballisticValue} />) : <span className="poodle-gain-reduction-meter-visual__bar" />}
    </span>
  </span>;
}
