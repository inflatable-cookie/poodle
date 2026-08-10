import { denormalizeAudioValue, normalizeAudioValue } from "./laws";
import type { AudioDragState, ModMatrixCell, ModMatrixHeader, ModMatrixVisualState } from "./types";

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

const clampAmount = (amount: number): number => Math.min(Math.max(Number.isFinite(amount) ? amount : 0, -1), 1);
const cellKey = (sourceId: string, destinationId: string): string => `${sourceId}\u0000${destinationId}`;

export function normalizeModMatrixCells(
  sources: ModMatrixHeader[], destinations: ModMatrixHeader[], cells: ModMatrixCell[],
): ModMatrixCell[] {
  const sourceIds = new Set(sources.map((source) => source.id));
  const destinationIds = new Set(destinations.map((destination) => destination.id));
  const supplied = new Map<string, ModMatrixCell>();
  for (const cell of cells) {
    if (!sourceIds.has(cell.sourceId) || !destinationIds.has(cell.destinationId)) continue;
    supplied.set(cellKey(cell.sourceId, cell.destinationId), { ...cell, amount: clampAmount(cell.amount), enabled: Boolean(cell.enabled) });
  }
  return sources.flatMap((source) => destinations.map((destination) => supplied.get(cellKey(source.id, destination.id)) ?? {
    sourceId: source.id, destinationId: destination.id, amount: 0, enabled: false,
  }));
}

export function createModMatrixContext(input: Partial<ModMatrixContext> = {}): ModMatrixContext {
  const sources = normalizeHeaders(input.sources ?? [], "source");
  const destinations = normalizeHeaders(input.destinations ?? [], "destination");
  return {
    sources,
    destinations,
    cells: normalizeModMatrixCells(sources, destinations, input.cells ?? []),
    focusRow: input.focusRow ?? null,
    focusColumn: input.focusColumn ?? null,
    step: Math.max(Number.isFinite(input.step) ? input.step ?? 0.01 : 0.01, 0),
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
  const cell = { ...updated, amount: clampAmount(updated.amount) };
  const cells = context.cells.map((candidate, candidateIndex) => candidateIndex === index ? cell : candidate);
  return { context: { ...context, cells }, effects: [
    { type: "emitCellChange", cell },
    ...(commit ? [{ type: "emitCellCommit", cell } as const] : []),
  ] };
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
      const delta = event.direction * context.step * (event.multiplier ?? 1) * (event.fine ? 0.1 : 1);
      return updateFocused(context, (cell) => ({ ...cell, amount: cell.amount + delta }), true);
    }
    case "DRAG_BEGIN": {
      if (context.disabled) return { context, effects: [] };
      const focused = { ...context, ...boundedFocus(context, event.row, event.column), drag: event.fine ? "fine" as const : "coarse" as const };
      const result = updateFocused(focused, (cell) => ({ ...cell, amount: denormalizeAudioValue(event.amountNorm, -1, 1, bipolarLaw) }), false);
      return { context: result.context, effects: [{ type: "beginGesture" }, ...result.effects] };
    }
    case "DRAG_MOVE": {
      if (context.disabled || context.drag === "none") return { context, effects: [] };
      return updateFocused({ ...context, drag: event.fine ? "fine" : "coarse" }, (cell) => ({
        ...cell, amount: denormalizeAudioValue(event.amountNorm, -1, 1, bipolarLaw),
      }), false);
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
    cells: context.cells.map((cell, cellIndex) => ({
      ...cell,
      amountNorm: normalizeAudioValue(cell.amount, -1, 1, bipolarLaw),
      focused: cellIndex === index,
    })),
    focus: focusedCell == null ? null : {
      sourceId: focusedCell.sourceId,
      destinationId: focusedCell.destinationId,
    },
    enabled: !context.disabled,
  };
}
