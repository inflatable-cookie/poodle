import { describe, expect, test } from "bun:test";
import { normalizeAudioValue } from "../src/audio/laws";
import {
  createDragNumberContext,
  createFaderContext,
  createKnobContext,
  dragNumberTransition,
  faderPointToNorm,
  faderTransition,
  knobPointToNorm,
  knobTransition,
  knobVisualState,
} from "../src/audio/value-controls";

describe("knob machine", () => {
  test("emits paired gesture effects and serializable drag state", () => {
    let context = createKnobContext({ value: 0.5 });
    let result = knobTransition(context, { type: "DRAG_BEGIN", position: 100, fine: false });
    expect(result.effects).toEqual([{ type: "beginGesture" }]);
    context = result.context;
    result = knobTransition(context, { type: "DRAG_MOVE", position: 84, fine: false });
    expect(result.context.value).toBeCloseTo(0.6, 12);
    expect(knobVisualState(result.context).drag).toBe("coarse");
    result = knobTransition(result.context, { type: "DRAG_END" });
    expect(result.effects).toEqual([{ type: "emitValueCommit", value: 0.6 }, { type: "endGesture" }]);
  });

  test("fine drag, wheel, reset, keyboard, and type-in share constraints", () => {
    const context = createKnobContext({ value: 0.5, defaultValue: 0.25, keyboardStep: 0.1, format: { type: "percent" } });
    const begun = knobTransition(context, { type: "DRAG_BEGIN", position: 100, fine: true }).context;
    expect(knobTransition(begun, { type: "DRAG_MOVE", position: 84, fine: true }).context.value).toBeCloseTo(0.51, 12);
    expect(knobTransition(context, { type: "WHEEL", direction: 1, fine: false }).context.value).toBe(0.6);
    expect(knobTransition(context, { type: "RESET" }).context.value).toBe(0.25);
    expect(knobTransition(context, { type: "KEY_NUDGE", direction: -1, multiplier: 10 }).context.value).toBe(0);
    expect(knobTransition(context, { type: "ENTRY_COMMIT", text: "75%" }).context.value).toBe(0.75);
  });

  test("logarithmic travel stays anchored to the gesture start", () => {
    const law = { type: "logarithmic" } as const;
    let context = createKnobContext({ value: 1000, min: 20, max: 50000, law, dragSensitivity: 100 });
    context = knobTransition(context, { type: "DRAG_BEGIN", position: 100, fine: false }).context;
    const startNorm = normalizeAudioValue(1000, 20, 50000, law);
    const moved = knobTransition(context, { type: "DRAG_MOVE", position: 90, fine: false }).context;
    expect(normalizeAudioValue(moved.value, 20, 50000, law)).toBeCloseTo(startNorm + 0.1, 9);
    // Anchoring means the second sample is measured from the start, not the
    // value the first sample produced.
    const again = knobTransition(moved, { type: "DRAG_MOVE", position: 90, fine: false }).context;
    expect(again.value).toBeCloseTo(moved.value, 9);
  });

  test("circular geometry maps the standard sweep", () => {
    const rect = { left: 0, top: 0, width: 100, height: 100 };
    expect(knobPointToNorm({ x: 50, y: 0 }, rect)).toBe(0.5);
    expect(knobPointToNorm({ x: 100, y: 50 }, rect)).toBeCloseTo(0.8333333333, 8);
  });

  test("circular fine adjustment uses one tenth of pointer travel", () => {
    let context = createKnobContext({ value: 0.5, dragMode: "circular" });
    context = knobTransition(context, { type: "DRAG_BEGIN", position: 0.5, fine: true }).context;
    expect(knobTransition(context, { type: "DRAG_SET_NORM", valueNorm: 1, fine: true }).context.value).toBeCloseTo(0.55, 12);
  });

  test("switching fine mode rebases without a value jump", () => {
    let context = createKnobContext({ value: 0.5 });
    context = knobTransition(context, { type: "DRAG_BEGIN", position: 100, fine: false }).context;
    context = knobTransition(context, { type: "DRAG_MOVE", position: 84, fine: false }).context;
    expect(context.value).toBeCloseTo(0.6, 12);
    const switched = knobTransition(context, { type: "DRAG_MOVE", position: 84, fine: true });
    expect(switched.context.value).toBeCloseTo(0.6, 12);
    expect(switched.context.drag).toBe("fine");
    expect(knobTransition(switched.context, { type: "DRAG_MOVE", position: 68, fine: true }).context.value).toBeCloseTo(0.61, 12);
  });

  test("disabled controls ignore every value-changing path", () => {
    const context = createKnobContext({ value: 0.5, disabled: true });
    expect(knobTransition(context, { type: "WHEEL", direction: 1, fine: false })).toEqual({ context, effects: [] });
    expect(knobTransition(context, { type: "RESET" })).toEqual({ context, effects: [] });
    expect(knobTransition(context, { type: "DRAG_BEGIN", position: 0, fine: false })).toEqual({ context, effects: [] });
    expect(knobTransition(context, { type: "ENTRY_COMMIT", text: "0.75" })).toEqual({ context: { ...context, entryOpen: false }, effects: [] });
  });
});

describe("fader and drag-number machines", () => {
  test("fader snaps to detents and exposes axis geometry", () => {
    let context = createFaderContext({ value: 0.4, detents: [0.5], detentSnap: 0.05 });
    context = faderTransition(context, { type: "DRAG_BEGIN", position: 0, fine: false }).context;
    const moved = faderTransition(context, { type: "DRAG_SET_NORM", valueNorm: 0.47, fine: false });
    expect(moved.context.value).toBe(0.5);
    expect(faderPointToNorm({ x: 20, y: 25 }, { left: 10, top: 5, width: 100, height: 100 }, "horizontal")).toBe(0.1);
    expect(faderPointToNorm({ x: 20, y: 25 }, { left: 10, top: 5, width: 100, height: 100 }, "vertical")).toBe(0.8);
  });

  test("fader fine drag uses one tenth of axis travel", () => {
    let context = createFaderContext({ value: 0.4 });
    context = faderTransition(context, { type: "DRAG_BEGIN", position: 0.4, fine: true }).context;
    expect(faderTransition(context, { type: "DRAG_SET_NORM", valueNorm: 0.9, fine: true }).context.value).toBeCloseTo(0.45, 12);
  });

  test("fader detents resolve through a non-linear law", () => {
    // 1 kHz is the normalized midpoint of a 20 Hz .. 50 kHz logarithmic range.
    const law = { type: "logarithmic" } as const;
    let context = createFaderContext({ value: 200, min: 20, max: 50000, law, detents: [1000], detentSnap: 0.02 });
    context = faderTransition(context, { type: "DRAG_BEGIN", position: 0, fine: false }).context;
    const midpoint = normalizeAudioValue(1000, 20, 50000, law);
    expect(faderTransition(context, { type: "DRAG_SET_NORM", valueNorm: midpoint + 0.01, fine: false }).context.value).toBeCloseTo(1000, 9);
    expect(faderTransition(context, { type: "DRAG_SET_NORM", valueNorm: midpoint + 0.1, fine: false }).context.value).not.toBeCloseTo(1000, 9);
  });

  test("drag-number emits live values then commits", () => {
    let context = createDragNumberContext({ value: 10, min: 0, max: 20, dragSensitivity: 0.5 });
    context = dragNumberTransition(context, { type: "DRAG_BEGIN", position: 100, fine: false }).context;
    const moved = dragNumberTransition(context, { type: "DRAG_MOVE", position: 110, fine: false });
    expect(moved.context.value).toBe(15);
    expect(moved.effects).toEqual([{ type: "emitValueChange", value: 15 }]);
    expect(dragNumberTransition(moved.context, { type: "DRAG_END" }).effects).toEqual([
      { type: "emitValueCommit", value: 15 }, { type: "endGesture" },
    ]);
  });
});
