import { describe, expect, test } from "bun:test";
import {
  WAVEFORM_MAX_COLUMNS,
  createWaveformContext,
  selectWaveformPeakLevel,
  validateWaveformPeakPyramid,
  waveformColumns,
  waveformPointToSample,
  waveformTransition,
  waveformVisualState,
} from "../src/audio/waveform";

const pyramid = {
  sampleCount: 8,
  levels: [
    { samplesPerPeak: 1, peaks: [
      { min: -0.1, max: 0.2 }, { min: -0.3, max: 0.4 }, { min: -0.2, max: 0.1 }, { min: -0.8, max: 0.7 },
      { min: -0.4, max: 0.5 }, { min: -0.1, max: 0.3 }, { min: -0.6, max: 0.2 }, { min: -0.2, max: 0.9 },
    ] },
    { samplesPerPeak: 2, peaks: [
      { min: -0.3, max: 0.4 }, { min: -0.8, max: 0.7 }, { min: -0.4, max: 0.5 }, { min: -0.6, max: 0.9 },
    ] },
  ],
};

describe("waveform display core", () => {
  test("validates reduced pyramids and rejects malformed levels", () => {
    expect(validateWaveformPeakPyramid(pyramid).levels).toHaveLength(2);
    expect(() => validateWaveformPeakPyramid({ sampleCount: 2, levels: [
      { samplesPerPeak: 2, peaks: [] }, { samplesPerPeak: 1, peaks: [] },
    ] })).toThrow(RangeError);
    expect(() => validateWaveformPeakPyramid({ sampleCount: 1, levels: [
      { samplesPerPeak: 1, peaks: [{ min: 0.5, max: -0.5 }] },
    ] })).toThrow(RangeError);
  });

  test("chooses a fitting pyramid level and preserves extrema", () => {
    const context = createWaveformContext({ pyramid, columnCount: 4 });
    expect(selectWaveformPeakLevel(context)?.samplesPerPeak).toBe(2);
    expect(waveformColumns(context)).toEqual(pyramid.levels[1]!.peaks);
    const reduced = waveformColumns(createWaveformContext({ pyramid, columnCount: 2 }));
    expect(reduced).toEqual([{ min: -0.8, max: 0.7 }, { min: -0.6, max: 0.9 }]);
  });

  test("hard-caps renderer columns", () => {
    const peaks = Array.from({ length: WAVEFORM_MAX_COLUMNS + 200 }, () => ({ min: -0.5, max: 0.5 }));
    const context = createWaveformContext({
      pyramid: { sampleCount: peaks.length, levels: [{ samplesPerPeak: 1, peaks }] },
      columnCount: WAVEFORM_MAX_COLUMNS + 1000,
    });
    expect(context.columnCount).toBe(WAVEFORM_MAX_COLUMNS);
    expect(waveformVisualState(context).columns).toHaveLength(WAVEFORM_MAX_COLUMNS);
  });

  test("owns cursor and ordered selection transitions", () => {
    let result = waveformTransition(createWaveformContext({ pyramid }), { type: "SELECT_BEGIN", sample: 6 });
    result = waveformTransition(result.context, { type: "SELECT_MOVE", sample: 2 });
    expect(result.context.selection).toEqual({ start: 2, end: 6 });
    expect(waveformTransition(result.context, { type: "SELECT_END" }).effects).toEqual([
      { type: "emitSelectionCommit", selection: { start: 2, end: 6 } },
    ]);
    result = waveformTransition(result.context, { type: "MOVE_CURSOR", delta: -2, extend: true });
    expect(result.context.cursorSample).toBe(0);
    expect(result.context.selection).toEqual({ start: 0, end: 6 });
  });

  test("maps pointer geometry to bounded sample indices", () => {
    const rect = { left: 10, top: 0, width: 100, height: 20 };
    expect(waveformPointToSample({ x: 10, y: 0 }, rect, 100, 200)).toBe(100);
    expect(waveformPointToSample({ x: 110, y: 0 }, rect, 100, 200)).toBe(199);
    expect(JSON.parse(JSON.stringify(waveformVisualState(createWaveformContext({ pyramid }))))).toEqual(
      waveformVisualState(createWaveformContext({ pyramid })),
    );
  });
});
