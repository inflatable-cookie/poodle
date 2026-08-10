import type { AudioPoint, AudioRect, EnvelopeVisualState } from "./types";

export interface EnvelopePoint {
  id: string;
  x: number;
  y: number;
  curve: number;
}

export interface EnvelopeContext {
  points: EnvelopePoint[];
  selectedPointId: string | null;
  hoverPointId: string | null;
  dragPointId: string | null;
  focus: boolean;
  step: number;
  curveStep: number;
  disabled: boolean;
}

export type EnvelopeEffect =
  | { type: "emitPointsChange"; points: EnvelopePoint[] }
  | { type: "emitPointsCommit"; points: EnvelopePoint[] }
  | { type: "beginGesture" }
  | { type: "endGesture" };

export type EnvelopeEvent =
  | { type: "SET_POINTS"; points: EnvelopePoint[] }
  | { type: "FOCUS"; value: boolean }
  | { type: "HOVER_POINT"; id: string | null }
  | { type: "SELECT_POINT"; id: string | null }
  | { type: "ADD_POINT"; point: EnvelopePoint }
  | { type: "REMOVE_SELECTED" }
  | { type: "DRAG_BEGIN"; id: string }
  | { type: "DRAG_MOVE"; point: Pick<EnvelopePoint, "x" | "y"> }
  | { type: "DRAG_END" }
  | { type: "NUDGE_SELECTED"; axis: "x" | "y"; direction: -1 | 1; multiplier?: number; fine?: boolean }
  | { type: "NUDGE_CURVE"; direction: -1 | 1; fine?: boolean };

export interface EnvelopeResult {
  context: EnvelopeContext;
  effects: EnvelopeEffect[];
}

const clamp01 = (value: number): number => Math.min(Math.max(Number.isFinite(value) ? value : 0, 0), 1);
const clampCurve = (value: number): number => Math.min(Math.max(Number.isFinite(value) ? value : 0, -1), 1);

export function normalizeEnvelopePoints(points: EnvelopePoint[]): EnvelopePoint[] {
  const ids = new Set<string>();
  const normalized = points.map((point) => {
    if (!point.id || ids.has(point.id)) throw new RangeError("Envelope point ids must be non-empty and unique");
    ids.add(point.id);
    return { id: point.id, x: clamp01(point.x), y: clamp01(point.y), curve: clampCurve(point.curve) };
  });
  return normalized.sort((left, right) => left.x - right.x);
}

export function createEnvelopeContext(input: Partial<EnvelopeContext> = {}): EnvelopeContext {
  return {
    selectedPointId: null,
    hoverPointId: null,
    dragPointId: null,
    focus: false,
    step: 0.01,
    curveStep: 0.1,
    disabled: false,
    ...input,
    points: normalizeEnvelopePoints(input.points ?? []),
  };
}

function withPoint(context: EnvelopeContext, id: string, update: (point: EnvelopePoint) => EnvelopePoint): EnvelopePoint[] {
  return normalizeEnvelopePoints(context.points.map((point) => point.id === id ? update(point) : point));
}

function atomic(context: EnvelopeContext, points: EnvelopePoint[], selectedPointId = context.selectedPointId): EnvelopeResult {
  const next = { ...context, points, selectedPointId };
  return { context: next, effects: [
    { type: "emitPointsChange", points },
    { type: "emitPointsCommit", points },
  ] };
}

export function envelopeTransition(context: EnvelopeContext, event: EnvelopeEvent): EnvelopeResult {
  switch (event.type) {
    case "SET_POINTS": return { context: { ...context, points: normalizeEnvelopePoints(event.points) }, effects: [] };
    case "FOCUS": return { context: { ...context, focus: event.value }, effects: [] };
    case "HOVER_POINT": return { context: { ...context, hoverPointId: event.id }, effects: [] };
    case "SELECT_POINT": return { context: { ...context, selectedPointId: event.id }, effects: [] };
    case "ADD_POINT": {
      if (context.disabled) return { context, effects: [] };
      const points = normalizeEnvelopePoints([...context.points, event.point]);
      return atomic(context, points, event.point.id);
    }
    case "REMOVE_SELECTED": {
      if (context.disabled || context.selectedPointId === null) return { context, effects: [] };
      return atomic(context, context.points.filter((point) => point.id !== context.selectedPointId), null);
    }
    case "DRAG_BEGIN": {
      if (context.disabled || !context.points.some((point) => point.id === event.id)) return { context, effects: [] };
      return { context: { ...context, selectedPointId: event.id, dragPointId: event.id }, effects: [{ type: "beginGesture" }] };
    }
    case "DRAG_MOVE": {
      if (context.disabled || context.dragPointId === null) return { context, effects: [] };
      const points = withPoint(context, context.dragPointId, (point) => ({ ...point, x: event.point.x, y: event.point.y }));
      return { context: { ...context, points }, effects: [{ type: "emitPointsChange", points }] };
    }
    case "DRAG_END": {
      if (context.dragPointId === null) return { context, effects: [] };
      return { context: { ...context, dragPointId: null }, effects: [
        { type: "emitPointsCommit", points: context.points }, { type: "endGesture" },
      ] };
    }
    case "NUDGE_SELECTED": {
      if (context.disabled || context.selectedPointId === null) return { context, effects: [] };
      const delta = event.direction * context.step * (event.multiplier ?? 1) * (event.fine ? 0.1 : 1);
      const points = withPoint(context, context.selectedPointId, (point) => ({ ...point, [event.axis]: point[event.axis] + delta }));
      return atomic(context, points);
    }
    case "NUDGE_CURVE": {
      if (context.disabled || context.selectedPointId === null) return { context, effects: [] };
      const delta = event.direction * context.curveStep * (event.fine ? 0.1 : 1);
      const points = withPoint(context, context.selectedPointId, (point) => ({ ...point, curve: point.curve + delta }));
      return atomic(context, points);
    }
  }
}

export function envelopeVisualState(context: EnvelopeContext): EnvelopeVisualState {
  return {
    points: context.points.map((point) => ({
      id: point.id,
      xNorm: point.x,
      yNorm: point.y,
      curve: point.curve,
      selected: point.id === context.selectedPointId,
      dragging: point.id === context.dragPointId,
    })),
    hoverPointId: context.hoverPointId,
    focus: context.focus,
    enabled: !context.disabled,
  };
}

export function envelopePointToNorm(point: AudioPoint, rect: AudioRect): Pick<EnvelopePoint, "x" | "y"> {
  return {
    x: clamp01((point.x - rect.left) / Math.max(rect.width, 1)),
    y: clamp01(1 - (point.y - rect.top) / Math.max(rect.height, 1)),
  };
}

export function envelopeHitTest(points: EnvelopePoint[], point: AudioPoint, rect: AudioRect, radiusPx = 10): string | null {
  let closest: string | null = null;
  let distance = Math.max(radiusPx, 0);
  for (const candidate of points) {
    const x = rect.left + candidate.x * rect.width;
    const y = rect.top + (1 - candidate.y) * rect.height;
    const next = Math.hypot(point.x - x, point.y - y);
    if (next <= distance) { closest = candidate.id; distance = next; }
  }
  return closest;
}

/** Evaluate one monotonic envelope segment at normalized time `t`. */
export function envelopeSegmentValueAt(from: EnvelopePoint, to: EnvelopePoint, t: number): number {
  const position = clamp01(t);
  const shaped = from.curve === 0
    ? position
    : from.curve > 0
      ? Math.pow(position, 1 + from.curve * 4)
      : 1 - Math.pow(1 - position, 1 + Math.abs(from.curve) * 4);
  return from.y + (to.y - from.y) * shaped;
}
