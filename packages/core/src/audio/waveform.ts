import type {
  AudioPoint,
  AudioRect,
  WaveformPeakLevel,
  WaveformPeakPair,
  WaveformPeakPyramid,
  WaveformSelection,
  WaveformVisualState,
} from "./types";

export const WAVEFORM_MAX_COLUMNS = 4096;

export interface WaveformContext {
  pyramid: WaveformPeakPyramid;
  visibleStart: number;
  visibleEnd: number;
  columnCount: number;
  cursorSample: number | null;
  selection: WaveformSelection | null;
  selectionAnchor: number | null;
  selecting: boolean;
  focus: boolean;
  disabled: boolean;
}

export type WaveformEffect =
  | { type: "emitCursorChange"; sample: number }
  | { type: "emitSelectionChange"; selection: WaveformSelection | null }
  | { type: "emitSelectionCommit"; selection: WaveformSelection | null };

export type WaveformEvent =
  | { type: "SET_PYRAMID"; pyramid: WaveformPeakPyramid }
  | { type: "SET_VIEWPORT"; start: number; end: number; columnCount?: number }
  | { type: "SET_CURSOR"; sample: number | null }
  | { type: "SET_SELECTION"; selection: WaveformSelection | null }
  | { type: "FOCUS"; value: boolean }
  | { type: "SELECT_BEGIN"; sample: number }
  | { type: "SELECT_MOVE"; sample: number }
  | { type: "SELECT_END" }
  | { type: "MOVE_CURSOR"; delta: number; extend?: boolean }
  | { type: "BOUND_CURSOR"; bound: "start" | "end"; extend?: boolean }
  | { type: "CLEAR_SELECTION" };

export interface WaveformResult { context: WaveformContext; effects: WaveformEffect[] }

const finiteInt = (value: number, fallback = 0): number => Math.round(Number.isFinite(value) ? value : fallback);

export function validateWaveformPeakPyramid(pyramid: WaveformPeakPyramid): WaveformPeakPyramid {
  const sampleCount = Math.max(finiteInt(pyramid.sampleCount), 0);
  let previous = 0;
  const levels = pyramid.levels.map((level) => {
    const samplesPerPeak = finiteInt(level.samplesPerPeak);
    if (samplesPerPeak <= previous || samplesPerPeak <= 0) {
      throw new RangeError("Waveform pyramid levels must have increasing positive integer samplesPerPeak");
    }
    previous = samplesPerPeak;
    const peaks = level.peaks.map((peak) => {
      if (!Number.isFinite(peak.min) || !Number.isFinite(peak.max) || peak.min > peak.max) {
        throw new RangeError("Waveform peaks require finite min <= max");
      }
      return {
        min: Math.min(Math.max(peak.min, -1), 1),
        max: Math.min(Math.max(peak.max, -1), 1),
      };
    });
    return { samplesPerPeak, peaks };
  });
  return { sampleCount, levels };
}

function clampSample(sample: number, context: Pick<WaveformContext, "visibleStart" | "visibleEnd">): number {
  return Math.min(Math.max(finiteInt(sample), context.visibleStart), Math.max(context.visibleEnd - 1, context.visibleStart));
}

function orderedSelection(anchor: number, sample: number): WaveformSelection {
  return { start: Math.min(anchor, sample), end: Math.max(anchor, sample) };
}

function normalizedViewport(sampleCount: number, start: number, end: number): [number, number] {
  const boundedStart = Math.min(Math.max(finiteInt(start), 0), sampleCount);
  const boundedEnd = Math.min(Math.max(finiteInt(end, sampleCount), boundedStart), sampleCount);
  return [boundedStart, boundedEnd];
}

export function createWaveformContext(input: Partial<WaveformContext> = {}): WaveformContext {
  const pyramid = validateWaveformPeakPyramid(input.pyramid ?? { sampleCount: 0, levels: [] });
  const [visibleStart, visibleEnd] = normalizedViewport(
    pyramid.sampleCount,
    input.visibleStart ?? 0,
    input.visibleEnd ?? pyramid.sampleCount,
  );
  const base = { visibleStart, visibleEnd };
  return {
    pyramid,
    visibleStart,
    visibleEnd,
    columnCount: Math.min(Math.max(finiteInt(input.columnCount ?? 512), 1), WAVEFORM_MAX_COLUMNS),
    cursorSample: input.cursorSample == null ? null : clampSample(input.cursorSample, base),
    selection: input.selection == null ? null : orderedSelection(
      clampSample(input.selection.start, base), clampSample(input.selection.end, base),
    ),
    selectionAnchor: input.selectionAnchor == null ? null : clampSample(input.selectionAnchor, base),
    selecting: input.selecting ?? false,
    focus: input.focus ?? false,
    disabled: input.disabled ?? false,
  };
}

export function selectWaveformPeakLevel(context: WaveformContext): WaveformPeakLevel | null {
  const span = Math.max(context.visibleEnd - context.visibleStart, 1);
  const target = Math.min(Math.max(context.columnCount, 1), WAVEFORM_MAX_COLUMNS);
  return context.pyramid.levels.find((level) => Math.ceil(span / level.samplesPerPeak) <= target)
    ?? context.pyramid.levels.at(-1)
    ?? null;
}

export function waveformColumns(context: WaveformContext): WaveformPeakPair[] {
  const level = selectWaveformPeakLevel(context);
  if (!level || context.visibleEnd <= context.visibleStart) return [];
  const first = Math.max(Math.floor(context.visibleStart / level.samplesPerPeak), 0);
  const last = Math.min(Math.ceil(context.visibleEnd / level.samplesPerPeak), level.peaks.length);
  const source = level.peaks.slice(first, last);
  const count = Math.min(context.columnCount, WAVEFORM_MAX_COLUMNS, source.length);
  if (count <= 0) return [];
  return Array.from({ length: count }, (_, column) => {
    const start = Math.floor(column * source.length / count);
    const end = Math.max(Math.ceil((column + 1) * source.length / count), start + 1);
    const group = source.slice(start, end);
    return {
      min: Math.min(...group.map((peak) => peak.min)),
      max: Math.max(...group.map((peak) => peak.max)),
    };
  });
}

export function waveformTransition(context: WaveformContext, event: WaveformEvent): WaveformResult {
  switch (event.type) {
    case "SET_PYRAMID": return { context: createWaveformContext({ ...context, pyramid: event.pyramid }), effects: [] };
    case "SET_VIEWPORT": return { context: createWaveformContext({
      ...context,
      visibleStart: event.start,
      visibleEnd: event.end,
      columnCount: event.columnCount ?? context.columnCount,
    }), effects: [] };
    case "SET_CURSOR": return { context: { ...context, cursorSample: event.sample == null ? null : clampSample(event.sample, context) }, effects: [] };
    case "SET_SELECTION": return { context: {
      ...context,
      selection: event.selection == null ? null : orderedSelection(clampSample(event.selection.start, context), clampSample(event.selection.end, context)),
    }, effects: [] };
    case "FOCUS": return { context: { ...context, focus: event.value }, effects: [] };
    case "SELECT_BEGIN": {
      if (context.disabled) return { context, effects: [] };
      const sample = clampSample(event.sample, context);
      const selection = { start: sample, end: sample };
      return { context: { ...context, cursorSample: sample, selection, selectionAnchor: sample, selecting: true }, effects: [
        { type: "emitCursorChange", sample }, { type: "emitSelectionChange", selection },
      ] };
    }
    case "SELECT_MOVE": {
      if (context.disabled || !context.selecting || context.selectionAnchor == null) return { context, effects: [] };
      const sample = clampSample(event.sample, context);
      const selection = orderedSelection(context.selectionAnchor, sample);
      return { context: { ...context, cursorSample: sample, selection }, effects: [
        { type: "emitCursorChange", sample }, { type: "emitSelectionChange", selection },
      ] };
    }
    case "SELECT_END": return !context.selecting ? { context, effects: [] } : {
      context: { ...context, selecting: false, selectionAnchor: null },
      effects: [{ type: "emitSelectionCommit", selection: context.selection }],
    };
    case "MOVE_CURSOR": {
      if (context.disabled) return { context, effects: [] };
      const origin = context.cursorSample ?? context.visibleStart;
      const sample = clampSample(origin + finiteInt(event.delta), context);
      const effects: WaveformEffect[] = [{ type: "emitCursorChange", sample }];
      if (!event.extend) return { context: { ...context, cursorSample: sample, selectionAnchor: null }, effects };
      const anchor = context.selectionAnchor ?? origin;
      const selection = orderedSelection(anchor, sample);
      return { context: { ...context, cursorSample: sample, selection, selectionAnchor: anchor }, effects: [
        ...effects, { type: "emitSelectionChange", selection }, { type: "emitSelectionCommit", selection },
      ] };
    }
    case "BOUND_CURSOR": {
      const target = event.bound === "start" ? context.visibleStart : Math.max(context.visibleEnd - 1, context.visibleStart);
      return waveformTransition(context, { type: "MOVE_CURSOR", delta: target - (context.cursorSample ?? context.visibleStart), extend: event.extend });
    }
    case "CLEAR_SELECTION": return context.disabled ? { context, effects: [] } : {
      context: { ...context, selection: null, selectionAnchor: null },
      effects: [{ type: "emitSelectionChange", selection: null }, { type: "emitSelectionCommit", selection: null }],
    };
  }
}

export function waveformVisualState(context: WaveformContext): WaveformVisualState {
  return {
    sampleCount: context.pyramid.sampleCount,
    visibleStart: context.visibleStart,
    visibleEnd: context.visibleEnd,
    columns: waveformColumns(context),
    cursorSample: context.cursorSample,
    selection: context.selection,
    focus: context.focus,
    enabled: !context.disabled,
  };
}

export function waveformPointToSample(point: AudioPoint, rect: AudioRect, visibleStart: number, visibleEnd: number): number {
  const norm = Math.min(Math.max((point.x - rect.left) / Math.max(rect.width, 1), 0), 1);
  return Math.min(Math.floor(visibleStart + norm * Math.max(visibleEnd - visibleStart, 1)), Math.max(visibleEnd - 1, visibleStart));
}
