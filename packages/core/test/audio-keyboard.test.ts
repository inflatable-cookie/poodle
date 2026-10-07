import { describe, expect, test } from "bun:test";
import {
  createKeyboardContext,
  keyboardHitTest,
  keyboardTransition,
  keyboardVelocityAtPoint,
  keyboardVisualState,
} from "../src/audio/keyboard";

describe("audio keyboard machine", () => {
  test("pairs note effects and reference-counts inputs", () => {
    let context = createKeyboardContext({ firstNote: 60, lastNote: 72 });
    let result = keyboardTransition(context, { type: "PRESS", inputId: "pointer:1", note: 60, velocity: 64 });
    expect(result.effects).toEqual([{ type: "noteOn", note: 60, velocity: 64 }]);
    context = result.context;
    result = keyboardTransition(context, { type: "PRESS", inputId: "pointer:2", note: 60, velocity: 100 });
    expect(result.effects).toEqual([]);
    context = keyboardTransition(result.context, { type: "RELEASE", inputId: "pointer:1" }).context;
    expect(keyboardTransition(context, { type: "RELEASE", inputId: "pointer:2" }).effects).toEqual([{ type: "noteOff", note: 60 }]);
  });

  test("computer keys ignore repeat and octave changes close gestures", () => {
    let result = keyboardTransition(createKeyboardContext({ firstNote: 48, lastNote: 96 }), {
      type: "COMPUTER_KEY_DOWN", key: "a", velocity: 90,
    });
    expect(result.effects).toEqual([{ type: "noteOn", note: 60, velocity: 90 }]);
    expect(keyboardTransition(result.context, { type: "COMPUTER_KEY_DOWN", key: "a", repeat: true }).effects).toEqual([]);
    result = keyboardTransition(result.context, { type: "SET_OCTAVE_SHIFT", value: 1 });
    expect(result.effects).toEqual([{ type: "noteOff", note: 60 }]);
    expect(result.context.octaveShift).toBe(1);
  });

  test("captured pointer retargets across keys and releases outside", () => {
    let result = keyboardTransition(createKeyboardContext({ firstNote: 60, lastNote: 72 }), {
      type: "PRESS", inputId: "pointer:1", note: 60, velocity: 40,
    });
    result = keyboardTransition(result.context, {
      type: "RETARGET", inputId: "pointer:1", note: 62, velocity: 96,
    });
    expect(result.effects).toEqual([
      { type: "noteOff", note: 60 },
      { type: "noteOn", note: 62, velocity: 96 },
    ]);
    expect(keyboardVisualState(result.context).heldNotes).toEqual([62]);
    result = keyboardTransition(result.context, {
      type: "RETARGET", inputId: "pointer:1", note: 62, velocity: 110,
    });
    expect(result.effects).toEqual([]);
    expect(result.context.activeInputs["pointer:1"]?.velocity).toBe(110);
    result = keyboardTransition(result.context, {
      type: "RETARGET", inputId: "pointer:1", note: null, velocity: 1,
    });
    expect(result.effects).toEqual([{ type: "noteOff", note: 62 }]);
    expect(keyboardVisualState(result.context).heldNotes).toEqual([]);
  });

  test("external highlights stay distinct and emit nothing", () => {
    const context = keyboardTransition(createKeyboardContext(), { type: "SET_EXTERNAL_HELD", notes: [64, 60, 64] }).context;
    const visual = keyboardVisualState(context);
    expect(visual.externalHeldNotes).toEqual([60, 64]);
    expect(visual.heldNotes).toEqual([]);
    expect(visual.keys.find((key) => key.note === 64)?.externallyHeld).toBe(true);
    expect(JSON.parse(JSON.stringify(visual))).toEqual(visual);
  });

  test("horizontal and gutter geometry own hit testing and velocity", () => {
    const rect = { left: 0, top: 0, width: 100, height: 100 };
    expect(keyboardVelocityAtPoint({ x: 50, y: 0 }, rect, "horizontal")).toBe(1);
    expect(keyboardVelocityAtPoint({ x: 50, y: 100 }, rect, "horizontal")).toBe(127);
    expect(keyboardVelocityAtPoint({ x: 100, y: 50 }, rect, "vertical")).toBe(127);
    const horizontal = createKeyboardContext({ firstNote: 60, lastNote: 61 });
    expect(keyboardHitTest(horizontal, { x: 50, y: 90 }, rect)).toBe(60);
    const vertical = createKeyboardContext({ firstNote: 60, lastNote: 61, orientation: "vertical" });
    expect(keyboardHitTest(vertical, { x: 90, y: 75 }, rect)).toBe(60);
  });

  test("equal rows share pixel geometry and hit testing across a partial scrolled row", () => {
    const context = createKeyboardContext({
      firstNote: 60,
      lastNote: 62,
      orientation: "vertical",
      keyLayout: "equal-rows",
      rowHeightPx: 10,
      scrollOffsetPx: 5,
    });
    const visual = keyboardVisualState(context);
    expect(visual.keys.map(({ note, startNorm, lengthNorm }) => ({ note, startNorm, lengthNorm }))).toEqual([
      { note: 60, startNorm: 2 / 3, lengthNorm: 1 / 3 },
      { note: 61, startNorm: 1 / 3, lengthNorm: 1 / 3 },
      { note: 62, startNorm: 0, lengthNorm: 1 / 3 },
    ]);
    expect(visual.keys.map(({ note, startPx, lengthPx }) => ({ note, startPx, lengthPx }))).toEqual([
      { note: 60, startPx: 15, lengthPx: 10 },
      { note: 61, startPx: 5, lengthPx: 10 },
      { note: 62, startPx: -5, lengthPx: 10 },
    ]);
    expect(visual.keys.every((key) => key.breadthNorm === 1)).toBe(true);
    const rect = { left: 0, top: 0, width: 100, height: 30 };
    expect(keyboardHitTest(context, { x: 90, y: 0 }, rect)).toBe(62);
    expect(keyboardHitTest(context, { x: 90, y: 7 }, rect)).toBe(61);
    expect(keyboardHitTest(context, { x: 10, y: 20 }, rect)).toBe(60);
    expect(keyboardHitTest(context, { x: 101, y: 7 }, rect)).toBeNull();
    expect(keyboardHitTest(context, { x: 90, y: 30 }, rect)).toBeNull();
  });

  test("range and disable changes close held notes", () => {
    let context = keyboardTransition(createKeyboardContext({ firstNote: 48, lastNote: 72 }), {
      type: "PRESS", inputId: "pointer", note: 60, velocity: 127,
    }).context;
    let result = keyboardTransition(context, { type: "SET_RANGE", firstNote: 61, lastNote: 72 });
    expect(result.effects).toEqual([{ type: "noteOff", note: 60 }]);
    context = keyboardTransition(createKeyboardContext(), { type: "PRESS", inputId: "pointer", note: 60, velocity: 127 }).context;
    result = keyboardTransition(context, { type: "SET_DISABLED", value: true });
    expect(result.effects).toEqual([{ type: "noteOff", note: 60 }]);
    expect(result.context.disabled).toBe(true);
  });
});
