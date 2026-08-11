import type { CSSProperties } from "react";
import type { ModMatrixVisualState } from "@inflatable-cookie/poodle-core";

export function ModMatrixVisual({ visualState }: { visualState: ModMatrixVisualState }) {
  return <span className="poodle-mod-matrix-grid-visual" aria-hidden="true">
    <span />
    {visualState.destinations.map((destination) => <span key={destination.id} className="poodle-mod-matrix-grid-visual__header">{destination.label}</span>)}
    {visualState.sources.flatMap((source) => [
      <span key={`h:${source.id}`} className="poodle-mod-matrix-grid-visual__header">{source.label}</span>,
      ...visualState.cells.filter((cell) => cell.sourceId === source.id).map((cell) => <span key={`${cell.sourceId}:${cell.destinationId}`} className="poodle-mod-matrix-grid-visual__cell poodle-slider" data-variant="embedded" data-orientation="horizontal" data-polarity={cell.parameters.min < 0 && cell.parameters.max > 0 ? "bipolar" : "unipolar"} data-source-id={cell.sourceId} data-destination-id={cell.destinationId} data-enabled={cell.enabled} data-negative={cell.amountNorm < cell.zeroNorm} data-focused={cell.focused} style={{ "--poodle-slider-center": `${cell.zeroNorm * 100}%`, "--poodle-slider-fill-start": `${cell.fillStartNorm * 100}%`, "--poodle-slider-fill-span": `${cell.fillSpanNorm * 100}%` } as CSSProperties}><span className="poodle-slider__track"><span className="poodle-slider__fill" /><span className="poodle-slider__center" /></span></span>),
    ])}
  </span>;
}
