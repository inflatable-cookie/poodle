import { describe, expect, test } from "bun:test";
import { createXYPadContext, xyPadPointToNorm, xyPadTransition, xyPadVisualState } from "../src/audio/xy-pad";

describe("xy pad machine", () => {
  test("maps a coarse drag atomically and pairs gesture effects", () => {
    let context = createXYPadContext();
    let result = xyPadTransition(context, { type: "DRAG_BEGIN", xNorm: 0.25, yNorm: 0.75, fine: false });
    expect(result.context).toMatchObject({ x: 0.25, y: 0.75, drag: "coarse" });
    expect(result.effects).toEqual([{ type: "beginGesture" }, { type: "emitValueChange", x: 0.25, y: 0.75 }]);
    context = result.context;
    result = xyPadTransition(context, { type: "DRAG_MOVE", xNorm: 0.5, yNorm: 0.4, fine: false });
    expect(result.effects).toEqual([{ type: "emitValueChange", x: 0.5, y: 0.4 }]);
    expect(xyPadTransition(result.context, { type: "DRAG_END" }).effects).toEqual([
      { type: "emitValueCommit", x: 0.5, y: 0.4 }, { type: "endGesture" },
    ]);
  });

  test("fine drag and modifier switching rebase without jumps", () => {
    let context = createXYPadContext({ x: 0.5, y: 0.5 });
    context = xyPadTransition(context, { type: "DRAG_BEGIN", xNorm: 0.5, yNorm: 0.5, fine: true }).context;
    context = xyPadTransition(context, { type: "DRAG_MOVE", xNorm: 1, yNorm: 0, fine: true }).context;
    expect(context.x).toBeCloseTo(0.55, 12);
    expect(context.y).toBeCloseTo(0.45, 12);
    const switched = xyPadTransition(context, { type: "DRAG_MOVE", xNorm: 1, yNorm: 0, fine: false });
    expect(switched.context).toMatchObject({ x: context.x, y: context.y, drag: "coarse" });
  });

  test("keys, bounds, reset, geometry, and VisualState share constraints", () => {
    let context = createXYPadContext({ x: 0.5, y: 0.5, defaultX: 0.2, defaultY: 0.8, keyboardStepX: 0.1 });
    context = xyPadTransition(context, { type: "NUDGE", axis: "x", direction: 1 }).context;
    expect(context.x).toBe(0.6);
    context = xyPadTransition(context, { type: "BOUND", axis: "y", bound: "min" }).context;
    expect(context.y).toBe(0);
    context = xyPadTransition(context, { type: "RESET" }).context;
    expect(context).toMatchObject({ x: 0.2, y: 0.8 });
    expect(xyPadPointToNorm({ x: 60, y: 45 }, { left: 10, top: 20, width: 100, height: 50 })).toEqual({ xNorm: 0.5, yNorm: 0.5 });
    expect(JSON.parse(JSON.stringify(xyPadVisualState(context)))).toEqual(xyPadVisualState(context));
  });
});
