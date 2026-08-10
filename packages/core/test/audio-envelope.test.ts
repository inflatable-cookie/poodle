import { describe, expect, test } from "bun:test";
import {
  createEnvelopeContext,
  envelopeHitTest,
  envelopePointToNorm,
  envelopeSegmentValueAt,
  envelopeTransition,
  envelopeVisualState,
  normalizeEnvelopePoints,
} from "../src/audio/envelope";

const points = [
  { id: "a", x: 0, y: 0, curve: 0 },
  { id: "b", x: 1, y: 1, curve: 0.5 },
];

describe("envelope machine", () => {
  test("normalizes points, preserves ids, and rejects duplicates", () => {
    expect(normalizeEnvelopePoints([{ id: "b", x: 2, y: -1, curve: 2 }, points[0]!])).toEqual([
      points[0], { id: "b", x: 1, y: 0, curve: 1 },
    ]);
    expect(() => normalizeEnvelopePoints([points[0]!, points[0]!])).toThrow(RangeError);
  });

  test("adds, removes, nudges, and curves atomically", () => {
    let context = createEnvelopeContext({ points });
    let result = envelopeTransition(context, { type: "ADD_POINT", point: { id: "c", x: 0.5, y: 0.25, curve: 0 } });
    expect(result.context.points.map((point) => point.id)).toEqual(["a", "c", "b"]);
    expect(result.effects.map((effect) => effect.type)).toEqual(["emitPointsChange", "emitPointsCommit"]);
    context = result.context;
    context = envelopeTransition(context, { type: "NUDGE_SELECTED", axis: "y", direction: 1 }).context;
    expect(context.points.find((point) => point.id === "c")?.y).toBe(0.26);
    context = envelopeTransition(context, { type: "NUDGE_CURVE", direction: -1 }).context;
    expect(context.points.find((point) => point.id === "c")?.curve).toBe(-0.1);
    expect(envelopeTransition(context, { type: "REMOVE_SELECTED" }).context.points.map((point) => point.id)).toEqual(["a", "b"]);
  });

  test("drag effects pair and VisualState contains renderer-complete points", () => {
    let context = createEnvelopeContext({ points });
    let result = envelopeTransition(context, { type: "DRAG_BEGIN", id: "a" });
    expect(result.effects).toEqual([{ type: "beginGesture" }]);
    context = result.context;
    result = envelopeTransition(context, { type: "DRAG_MOVE", point: { x: 0.4, y: 0.7 } });
    expect(result.effects[0]).toEqual({ type: "emitPointsChange", points: result.context.points });
    const visual = envelopeVisualState(result.context);
    expect(visual.points.find((point) => point.id === "a")).toMatchObject({ xNorm: 0.4, yNorm: 0.7, selected: true, dragging: true });
    expect(JSON.parse(JSON.stringify(visual))).toEqual(visual);
    expect(envelopeTransition(result.context, { type: "DRAG_END" }).effects.map((effect) => effect.type)).toEqual(["emitPointsCommit", "endGesture"]);
  });

  test("geometry maps and hit-tests independently of rendering", () => {
    const rect = { left: 10, top: 20, width: 100, height: 50 };
    expect(envelopePointToNorm({ x: 60, y: 45 }, rect)).toEqual({ x: 0.5, y: 0.5 });
    expect(envelopeHitTest(points, { x: 10, y: 70 }, rect, 5)).toBe("a");
    expect(envelopeHitTest(points, { x: 60, y: 45 }, rect, 5)).toBeNull();
  });

  test("segment curves are canonical, monotonic, and preserve flat segments", () => {
    const from = { id: "a", x: 0, y: 0, curve: 0 };
    const to = { id: "b", x: 1, y: 1, curve: 0 };
    expect(envelopeSegmentValueAt(from, to, 0.5)).toBe(0.5);
    expect(envelopeSegmentValueAt({ ...from, curve: 0.5 }, to, 0.5)).toBe(0.125);
    expect(envelopeSegmentValueAt({ ...from, curve: -0.5 }, to, 0.5)).toBe(0.875);
    expect(envelopeSegmentValueAt({ ...from, y: 0.4, curve: 1 }, { ...to, y: 0.4 }, 0.75)).toBe(0.4);
  });
});
