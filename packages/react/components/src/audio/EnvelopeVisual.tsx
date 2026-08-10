import { envelopeSegmentValueAt, type EnvelopeVisualPoint, type EnvelopeVisualState } from "@inflatable-cookie/poodle-core";

function segmentPath(point: EnvelopeVisualPoint, next: EnvelopeVisualPoint): string {
  const from = { id: point.id, x: point.xNorm, y: point.yNorm, curve: point.curve };
  const to = { id: next.id, x: next.xNorm, y: next.yNorm, curve: next.curve };
  return Array.from({ length: 25 }, (_, index) => {
    const t = index / 24;
    const x = (point.xNorm + (next.xNorm - point.xNorm) * t) * 100;
    const y = (1 - envelopeSegmentValueAt(from, to, t)) * 100;
    return `${index === 0 ? "M" : "L"} ${x} ${y}`;
  }).join(" ");
}

export function EnvelopeVisual({ visualState }: { visualState: EnvelopeVisualState }) {
  return <svg className="poodle-envelope-editor-visual" viewBox="0 0 100 100" preserveAspectRatio="none" data-focus={visualState.focus} data-enabled={visualState.enabled} aria-hidden="true">
    <path className="poodle-envelope-editor-visual__grid" d="M 25 0 V 100 M 50 0 V 100 M 75 0 V 100 M 0 25 H 100 M 0 50 H 100 M 0 75 H 100" />
    {visualState.points.slice(0, -1).map((point, index) => <path key={point.id} className="poodle-envelope-editor-visual__curve" d={segmentPath(point, visualState.points[index + 1]!)} />)}
    {visualState.points.map((point) => <circle key={point.id} className="poodle-envelope-editor-visual__point" cx={point.xNorm * 100} cy={(1 - point.yNorm) * 100} r="2.8" data-selected={point.selected} data-dragging={point.dragging} data-hover={visualState.hoverPointId === point.id} />)}
  </svg>;
}
