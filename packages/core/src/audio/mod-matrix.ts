import { clampAudioValue, constrainAudioValue, linearValueLaw, normalizeAudioValue } from "./laws";
import type { AudioValueLaw } from "./laws";
import type { AudioDragState, ModMatrixCell, ModMatrixHeader, ModMatrixVisualState, ResolvedModMatrixCellParameters } from "./types";
import { createSliderControlContext, sliderControlTransition, sliderVisualState, type SliderControlEvent, type SliderPolarity } from "../slider";

const bipolarLaw = { type: "bipolar-center", center: 0 } as const;

export interface ModMatrixContext {
  sources: ModMatrixHeader[];
  destinations: ModMatrixHeader[];
  cells: ModMatrixCell[];
  focusRow: number | null;
  focusColumn: number | null;
  step: number;
  drag: AudioDragState;
  disabled: boolean;
}

export type ModMatrixEffect =
  | { type: "emitCellChange"; cell: ModMatrixCell }
  | { type: "emitCellCommit"; cell: ModMatrixCell }
  | { type: "beginGesture" }
  | { type: "endGesture" };

export type ModMatrixEvent =
  | { type: "SET_DATA"; sources: ModMatrixHeader[]; destinations: ModMatrixHeader[]; cells: ModMatrixCell[] }
  | { type: "FOCUS_CELL"; row: number; column: number }
  | { type: "MOVE_FOCUS"; rows?: number; columns?: number }
  | { type: "BOUND_FOCUS"; bound: "row-start" | "row-end" | "grid-start" | "grid-end" }
  | { type: "TOGGLE_FOCUSED" }
  | { type: "NUDGE_FOCUSED"; direction: -1 | 1; multiplier?: number; fine?: boolean }
  | { type: "DRAG_BEGIN"; row: number; column: number; amountNorm: number; fine?: boolean }
  | { type: "DRAG_MOVE"; amountNorm: number; fine?: boolean }
  | { type: "DRAG_END" };

export interface ModMatrixResult { context: ModMatrixContext; effects: ModMatrixEffect[] }

function normalizeHeaders(headers: ModMatrixHeader[], axis: string): ModMatrixHeader[] {
  const ids = new Set<string>();
  return headers.map((header) => {
    if (!header.id || ids.has(header.id)) throw new RangeError(`Mod matrix ${axis} ids must be non-empty and unique`);
    ids.add(header.id);
    return { id: header.id, label: header.label };
  });
}

const cellKey = (sourceId: string, destinationId: string): string => `${sourceId}\u0000${destinationId}`;

export function resolveModMatrixCellParameters(
  cell: Pick<ModMatrixCell, "parameters">,
  fallbackStep = 0.01,
): ResolvedModMatrixCellParameters {
  const min = cell.parameters?.min ?? -1;
  const max = cell.parameters?.max ?? 1;
  if (!Number.isFinite(min) || !Number.isFinite(max) || max <= min) {
    throw new RangeError("Mod matrix cell parameters require finite min < max");
  }
  const law = cell.parameters?.law ?? (min < 0 && max > 0 ? bipolarLaw : linearValueLaw);
  normalizeAudioValue(min, min, max, law);
  const lawStep = law.type === "stepped" ? law.step : undefined;
  const step = cell.parameters?.step ?? lawStep ?? fallbackStep;
  if (!Number.isFinite(step) || step < 0) throw new RangeError("Mod matrix cell step must be finite and non-negative");
  return { min, max, step, law };
}

export function normalizeModMatrixCells(
  sources: ModMatrixHeader[], destinations: ModMatrixHeader[], cells: ModMatrixCell[], fallbackStep = 0.01,
): ModMatrixCell[] {
  const sourceIds = new Set(sources.map((source) => source.id));
  const destinationIds = new Set(destinations.map((destination) => destination.id));
  const supplied = new Map<string, ModMatrixCell>();
  for (const cell of cells) {
    if (!sourceIds.has(cell.sourceId) || !destinationIds.has(cell.destinationId)) continue;
    const parameters = resolveModMatrixCellParameters(cell, fallbackStep);
    const fallback = clampAudioValue(0, parameters.min, parameters.max);
    const amount = constrainAudioValue(Number.isFinite(cell.amount) ? cell.amount : fallback, parameters.min, parameters.max, parameters.law);
    supplied.set(cellKey(cell.sourceId, cell.destinationId), { ...cell, parameters, amount, enabled: Boolean(cell.enabled) });
  }
  return sources.flatMap((source) => destinations.map((destination) => {
    const suppliedCell = supplied.get(cellKey(source.id, destination.id));
    if (suppliedCell) return suppliedCell;
    const parameters = resolveModMatrixCellParameters({}, fallbackStep);
    return { sourceId: source.id, destinationId: destination.id, amount: 0, enabled: false, parameters };
  }));
}

export function createModMatrixContext(input: Partial<ModMatrixContext> = {}): ModMatrixContext {
  const sources = normalizeHeaders(input.sources ?? [], "source");
  const destinations = normalizeHeaders(input.destinations ?? [], "destination");
  const step = Math.max(Number.isFinite(input.step) ? input.step ?? 0.01 : 0.01, 0);
  return {
    sources,
    destinations,
    cells: normalizeModMatrixCells(sources, destinations, input.cells ?? [], step),
    focusRow: input.focusRow ?? null,
    focusColumn: input.focusColumn ?? null,
    step,
    drag: input.drag ?? "none",
    disabled: input.disabled ?? false,
  };
}

function boundedFocus(context: ModMatrixContext, row: number, column: number): Pick<ModMatrixContext, "focusRow" | "focusColumn"> {
  if (context.sources.length === 0 || context.destinations.length === 0) return { focusRow: null, focusColumn: null };
  return {
    focusRow: Math.min(Math.max(Math.round(row), 0), context.sources.length - 1),
    focusColumn: Math.min(Math.max(Math.round(column), 0), context.destinations.length - 1),
  };
}

function focusedIndex(context: ModMatrixContext): number | null {
  if (context.focusRow == null || context.focusColumn == null || context.destinations.length === 0) return null;
  return context.focusRow * context.destinations.length + context.focusColumn;
}

function updateFocused(context: ModMatrixContext, update: (cell: ModMatrixCell) => ModMatrixCell, commit: boolean): ModMatrixResult {
  const index = focusedIndex(context);
  if (index == null || !context.cells[index]) return { context, effects: [] };
  const updated = update(context.cells[index]);
  const parameters = resolveModMatrixCellParameters(updated, context.step);
  const cell = {
    ...updated,
    parameters,
    amount: constrainAudioValue(updated.amount, parameters.min, parameters.max, parameters.law),
  };
  const cells = context.cells.map((candidate, candidateIndex) => candidateIndex === index ? cell : candidate);
  return { context: { ...context, cells }, effects: [
    { type: "emitCellChange", cell },
    ...(commit ? [{ type: "emitCellCommit", cell } as const] : []),
  ] };
}

function cellPolarity(parameters: ResolvedModMatrixCellParameters): SliderPolarity {
  return parameters.min < 0 && parameters.max > 0 ? "bipolar" : "unipolar";
}

function updateFocusedFromSlider(context: ModMatrixContext, event: SliderControlEvent, commit = false): ModMatrixResult {
  const index = focusedIndex(context);
  const cell = index == null ? null : context.cells[index] ?? null;
  if (!cell) return { context, effects: [] };
  const parameters = resolveModMatrixCellParameters(cell, context.step);
  const control = createSliderControlContext({
    value: cell.amount,
    min: parameters.min,
    max: parameters.max,
    step: parameters.step,
    law: parameters.law,
    polarity: cellPolarity(parameters),
    centerValue: null,
    pointerActive: context.drag !== "none",
    disabled: context.disabled,
  });
  const result = sliderControlTransition(control, event);
  const effect = result.effects.at(-1);
  if (!effect) return { context, effects: [] };
  return updateFocused(context, (candidate) => ({ ...candidate, amount: effect.value }), commit);
}

export function modMatrixTransition(context: ModMatrixContext, event: ModMatrixEvent): ModMatrixResult {
  switch (event.type) {
    case "SET_DATA": return { context: createModMatrixContext({ ...context, ...event }), effects: [] };
    case "FOCUS_CELL": return { context: { ...context, ...boundedFocus(context, event.row, event.column) }, effects: [] };
    case "MOVE_FOCUS": {
      if (context.disabled) return { context, effects: [] };
      const row = (context.focusRow ?? 0) + (event.rows ?? 0);
      const column = (context.focusColumn ?? 0) + (event.columns ?? 0);
      return { context: { ...context, ...boundedFocus(context, row, column) }, effects: [] };
    }
    case "BOUND_FOCUS": {
      if (context.disabled) return { context, effects: [] };
      const row = event.bound === "grid-start" ? 0 : event.bound === "grid-end" ? context.sources.length - 1 : context.focusRow ?? 0;
      const column = event.bound === "row-start" || event.bound === "grid-start" ? 0 : context.destinations.length - 1;
      return { context: { ...context, ...boundedFocus(context, row, column) }, effects: [] };
    }
    case "TOGGLE_FOCUSED": return context.disabled ? { context, effects: [] } : updateFocused(context, (cell) => ({ ...cell, enabled: !cell.enabled }), true);
    case "NUDGE_FOCUSED": {
      if (context.disabled) return { context, effects: [] };
      const index = focusedIndex(context);
      const focused = index == null ? null : context.cells[index] ?? null;
      if (!focused) return { context, effects: [] };
      const delta = event.direction * resolveModMatrixCellParameters(focused, context.step).step * (event.multiplier ?? 1) * (event.fine ? 0.1 : 1);
      return updateFocused(context, (cell) => ({ ...cell, amount: cell.amount + delta }), true);
    }
    case "DRAG_BEGIN": {
      if (context.disabled) return { context, effects: [] };
      const focused = { ...context, ...boundedFocus(context, event.row, event.column), drag: event.fine ? "fine" as const : "coarse" as const };
      const result = updateFocusedFromSlider(focused, { type: "POINTER_BEGIN", valueNorm: event.amountNorm });
      return { context: result.context, effects: [{ type: "beginGesture" }, ...result.effects] };
    }
    case "DRAG_MOVE": {
      if (context.disabled || context.drag === "none") return { context, effects: [] };
      return updateFocusedFromSlider(
        { ...context, drag: event.fine ? "fine" : "coarse" },
        { type: "POINTER_MOVE", valueNorm: event.amountNorm },
      );
    }
    case "DRAG_END": {
      const index = focusedIndex(context);
      const cell = index == null ? null : context.cells[index] ?? null;
      if (context.drag === "none" || !cell) return { context, effects: [] };
      return { context: { ...context, drag: "none" }, effects: [{ type: "emitCellCommit", cell }, { type: "endGesture" }] };
    }
  }
}

export function modMatrixVisualState(context: ModMatrixContext): ModMatrixVisualState {
  const index = focusedIndex(context);
  const focusedCell = index == null ? undefined : context.cells[index];
  return {
    sources: context.sources,
    destinations: context.destinations,
    cells: context.cells.map((cell, cellIndex) => {
      const parameters = resolveModMatrixCellParameters(cell, context.step);
      const slider = sliderVisualState(createSliderControlContext({
        value: cell.amount,
        min: parameters.min,
        max: parameters.max,
        step: parameters.step,
        law: parameters.law,
        polarity: cellPolarity(parameters),
      }));
      const amountNorm = slider.valueNorm;
      const zeroNorm = slider.centerNorm;
      return {
        ...cell,
        parameters,
        amountNorm,
        zeroNorm,
        fillStartNorm: slider.fillStartNorm,
        fillSpanNorm: slider.fillSpanNorm,
        focused: cellIndex === index,
      };
    }),
    focus: focusedCell == null ? null : {
      sourceId: focusedCell.sourceId,
      destinationId: focusedCell.destinationId,
    },
    enabled: !context.disabled,
  };
}
