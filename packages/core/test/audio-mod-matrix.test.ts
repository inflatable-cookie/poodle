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
    ])).toMatchObject([
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
    expect(result.effects.at(-1)).toMatchObject({
      type: "emitCellCommit", cell: { sourceId: "lfo", destinationId: "pitch", amount: 0, enabled: true },
    });
    context = result.context;
    result = modMatrixTransition(context, { type: "NUDGE_FOCUSED", direction: -1 });
    expect(result.context.cells[0]!.amount).toBe(-0.25);
    expect(modMatrixVisualState(result.context).cells[0]!.amountNorm).toBe(0.375);
  });

  test("mixes per-cell unipolar and bipolar parameters", () => {
    let context = createModMatrixContext({
      sources: sources.slice(0, 1), destinations,
      cells: [
        { sourceId: "lfo", destinationId: "pitch", amount: 0.4, enabled: true, parameters: { min: 0, max: 1, step: 0.1 } },
        { sourceId: "lfo", destinationId: "gain", amount: 2, enabled: true, parameters: { min: -2, max: 6, step: 0.5, law: { type: "linear" } } },
      ],
      focusRow: 0, focusColumn: 0,
    });
    let visual = modMatrixVisualState(context);
    expect(visual.cells[0]).toMatchObject({ amountNorm: 0.4, zeroNorm: 0, fillStartNorm: 0, fillSpanNorm: 0.4 });
    expect(visual.cells[1]).toMatchObject({ amountNorm: 0.5, zeroNorm: 0.25, fillStartNorm: 0.25, fillSpanNorm: 0.25 });
    context = modMatrixTransition(context, { type: "NUDGE_FOCUSED", direction: 1 }).context;
    expect(context.cells[0]!.amount).toBe(0.5);
    context = modMatrixTransition(context, { type: "NUDGE_FOCUSED", direction: 1, fine: true }).context;
    expect(context.cells[0]!.amount).toBeCloseTo(0.51);
    context = modMatrixTransition(context, { type: "DRAG_BEGIN", row: 0, column: 0, amountNorm: 0.75 }).context;
    expect(context.cells[0]!.amount).toBe(0.8); // Embedded Slider snaps pointer values to the cell's step.
    expect(() => createModMatrixContext({ sources, destinations, cells: [{ sourceId: "lfo", destinationId: "pitch", amount: 0, enabled: true, parameters: { min: 1, max: 1 } }] })).toThrow(RangeError);
    visual = modMatrixVisualState(context);
    expect(JSON.parse(JSON.stringify(visual))).toEqual(visual);
    const negative = modMatrixVisualState(createModMatrixContext({
      sources: sources.slice(0, 1), destinations: destinations.slice(0, 1),
      cells: [{ sourceId: "lfo", destinationId: "pitch", amount: -0.25, enabled: true, parameters: { min: -1, max: 0 } }],
    }));
    expect(negative.cells[0]).toMatchObject({ amountNorm: 0.75, zeroNorm: 1, fillStartNorm: 0.75, fillSpanNorm: 0.25 });
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
