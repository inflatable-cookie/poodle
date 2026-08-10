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
