import type { CSSProperties } from "react";
import type { ModMatrixVisualState } from "@inflatable-cookie/poodle-core";

export function ModMatrixVisual({ visualState }: { visualState: ModMatrixVisualState }) {
  return <span className="poodle-mod-matrix-grid-visual" aria-hidden="true">
    <span />
    {visualState.destinations.map((destination) => <span key={destination.id} className="poodle-mod-matrix-grid-visual__header">{destination.label}</span>)}
    {visualState.sources.flatMap((source) => [
      <span key={`h:${source.id}`} className="poodle-mod-matrix-grid-visual__header">{source.label}</span>,
      ...visualState.cells.filter((cell) => cell.sourceId === source.id).map((cell) => <span key={`${cell.sourceId}:${cell.destinationId}`} className="poodle-mod-matrix-grid-visual__cell" data-source-id={cell.sourceId} data-destination-id={cell.destinationId} data-enabled={cell.enabled} data-negative={cell.amountNorm < cell.zeroNorm} data-focused={cell.focused} style={{ "--poodle-mod-zero": cell.zeroNorm } as CSSProperties}><span className="poodle-mod-matrix-grid-visual__amount" style={{ "--poodle-mod-fill-start": cell.fillStartNorm, "--poodle-mod-fill-span": cell.fillSpanNorm } as CSSProperties} /></span>),
    ])}
  </span>;
}
