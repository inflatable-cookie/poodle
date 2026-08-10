import { describe, expect, test } from "bun:test";
import {
  createModMatrixContext,
  modMatrixTransition,
  modMatrixVisualState,
  normalizeModMatrixCells,
} from "../src/audio/mod-matrix";

const sources = [{ id: "lfo", label: "LFO" }, { id: "env", label: "Envelope" }];
const destinations = [{ id: "pitch", label: "Pitch" }, { id: "gain", label: "Gain" }];

describe("mod matrix machine", () => {
  test("normalizes sparse cells in row-major caller order", () => {
    expect(normalizeModMatrixCells(sources, destinations, [
      { sourceId: "env", destinationId: "gain", amount: 2, enabled: true },
      { sourceId: "unknown", destinationId: "gain", amount: 1, enabled: true },
    ])).toEqual([
      { sourceId: "lfo", destinationId: "pitch", amount: 0, enabled: false },
      { sourceId: "lfo", destinationId: "gain", amount: 0, enabled: false },
      { sourceId: "env", destinationId: "pitch", amount: 0, enabled: false },
      { sourceId: "env", destinationId: "gain", amount: 1, enabled: true },
    ]);
    expect(() => createModMatrixContext({ sources: [sources[0]!, sources[0]!] })).toThrow(RangeError);
  });

  test("navigates bounded rows and columns", () => {
    let context = createModMatrixContext({ sources, destinations, focusRow: 0, focusColumn: 0 });
    context = modMatrixTransition(context, { type: "MOVE_FOCUS", rows: 9, columns: 9 }).context;
    expect(context).toMatchObject({ focusRow: 1, focusColumn: 1 });
    context = modMatrixTransition(context, { type: "BOUND_FOCUS", bound: "grid-start" }).context;
    expect(context).toMatchObject({ focusRow: 0, focusColumn: 0 });
    context = modMatrixTransition(context, { type: "BOUND_FOCUS", bound: "row-end" }).context;
    expect(context).toMatchObject({ focusRow: 0, focusColumn: 1 });
  });

  test("toggles and nudges the focused bipolar cell", () => {
    let context = createModMatrixContext({ sources, destinations, focusRow: 0, focusColumn: 0, step: 0.25 });
    let result = modMatrixTransition(context, { type: "TOGGLE_FOCUSED" });
    expect(result.effects.at(-1)).toEqual({
      type: "emitCellCommit", cell: { sourceId: "lfo", destinationId: "pitch", amount: 0, enabled: true },
    });
    context = result.context;
    result = modMatrixTransition(context, { type: "NUDGE_FOCUSED", direction: -1 });
    expect(result.context.cells[0]!.amount).toBe(-0.25);
    expect(modMatrixVisualState(result.context).cells[0]!.amountNorm).toBe(0.375);
  });

  test("pairs drag effects and publishes serializable VisualState", () => {
    let result = modMatrixTransition(createModMatrixContext({ sources, destinations }), {
      type: "DRAG_BEGIN", row: 1, column: 0, amountNorm: 0.75,
    });
    expect(result.effects[0]).toEqual({ type: "beginGesture" });
    expect(result.context.cells[2]!.amount).toBe(0.5);
    result = modMatrixTransition(result.context, { type: "DRAG_END" });
    expect(result.effects.at(-1)).toEqual({ type: "endGesture" });
    const visual = modMatrixVisualState(result.context);
    expect(visual.focus).toEqual({ sourceId: "env", destinationId: "pitch" });
    expect(JSON.parse(JSON.stringify(visual))).toEqual(visual);
  });
});
