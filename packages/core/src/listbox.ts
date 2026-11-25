/**
 * Listbox behavior machine.
 * Contract: docs/contracts/components/listbox.md.
 *
 * Owns option focus, typeahead and selection transitions. Rendering and DOM
 * focus remain adapter-side so rows can contain host-rendered content.
 */

import { moveActiveOption } from "./active-option";
import { selectionTransition } from "./selection";

export interface ListboxItem {
  value: string;
  label: string;
  disabled?: boolean;
}

export type ListboxSelectionMode = "single" | "multiple";
export type ListboxOrientation = "vertical" | "horizontal";

export interface ListboxContext {
  items: ListboxItem[];
  selectionMode: ListboxSelectionMode;
  orientation: ListboxOrientation;
  disabled: boolean;
  selectedValues: string[];
  focusedValue: string | null;
  anchorValue: string | null;
  typeahead: string;
  typeaheadAt: number;
}

export type ListboxEvent =
  | { type: "FOCUS"; value: string }
  | { type: "MOVE"; direction: -1 | 1; extendSelection?: boolean }
  | { type: "BOUNDARY"; boundary: "first" | "last" }
  | { type: "TYPEAHEAD"; character: string; now: number }
  | { type: "SPACE" }
  | { type: "SELECT"; value: string; additive?: boolean; range?: boolean }
  | { type: "SELECT_ALL" }
  | { type: "ACTIVATE"; value?: string };

export type ListboxEffect =
  | { type: "focus"; value: string }
  | { type: "selectionChanged"; values: string[] }
  | { type: "activate"; value: string };

export interface ListboxResult {
  context: ListboxContext;
  effects: ListboxEffect[];
}

const TYPEAHEAD_TIMEOUT_MS = 500;

export function listboxEnabledItems(context: Pick<ListboxContext, "items" | "disabled">): ListboxItem[] {
  return context.disabled ? [] : context.items.filter((item) => item.disabled !== true);
}

export function listboxInitialFocus(
  items: readonly ListboxItem[],
  selectedValues: readonly string[],
  disabled = false,
): string | null {
  if (disabled) return null;
  return items.find((item) => !item.disabled && selectedValues.includes(item.value))?.value ??
    moveActiveOption(items, null, "first");
}

function normalizeValues(values: readonly string[], context: ListboxContext): string[] {
  const available = new Set(context.items.filter((item) => !item.disabled).map((item) => item.value));
  const items = values.filter((value) => available.has(value));
  return [...selectionTransition([], { type: "REPLACE", items }, { mode: context.selectionMode })];
}

function focusResult(context: ListboxContext, value: string, anchorValue = context.anchorValue): ListboxResult {
  return {
    context: { ...context, focusedValue: value, anchorValue },
    effects: [{ type: "focus", value }],
  };
}

function withSelection(context: ListboxContext, values: readonly string[]): ListboxResult {
  const normalized = normalizeValues(values, context);
  const changed = normalized.length !== context.selectedValues.length ||
    normalized.some((value, index) => value !== context.selectedValues[index]);
  return {
    context: { ...context, selectedValues: normalized },
    effects: changed ? [{ type: "selectionChanged", values: normalized }] : [],
  };
}

function enabledIndex(context: ListboxContext, value: string | null): number {
  return listboxEnabledItems(context).findIndex((item) => item.value === value);
}

function moveFocus(context: ListboxContext, direction: -1 | 1, extendSelection = false): ListboxResult {
  const next = moveActiveOption(context.items, context.focusedValue, direction > 0 ? "next" : "previous", false);
  if (next === null || next === context.focusedValue) return { context, effects: [] };

  const nextContext = { ...context, focusedValue: next };
  if (context.selectionMode === "single") {
    const selected = withSelection(nextContext, [next]);
    return { context: { ...selected.context, anchorValue: next }, effects: [{ type: "focus", value: next }, ...selected.effects] };
  }

  if (extendSelection) {
    const anchor = context.anchorValue ?? context.focusedValue ?? next;
    const all = context.items;
    const anchorIndex = all.findIndex((item) => item.value === anchor);
    const targetIndex = all.findIndex((item) => item.value === next);
    const [start, end] = anchorIndex <= targetIndex ? [anchorIndex, targetIndex] : [targetIndex, anchorIndex];
    const range = all.slice(start, end + 1).filter((item) => !item.disabled).map((item) => item.value);
    const selected = withSelection({ ...nextContext, anchorValue: anchor }, range);
    return { context: selected.context, effects: [{ type: "focus", value: next }, ...selected.effects] };
  }

  return { context: nextContext, effects: [{ type: "focus", value: next }] };
}

function focusBoundary(context: ListboxContext, boundary: "first" | "last"): ListboxResult {
  const value = moveActiveOption(context.items, null, boundary);
  if (value === null) return { context, effects: [] };
  const next = focusResult(context, value, value);
  if (context.selectionMode !== "single") return next;
  const selected = withSelection(next.context, [value]);
  return { context: selected.context, effects: [...next.effects, ...selected.effects] };
}

function focusTypeahead(context: ListboxContext, character: string, now: number): ListboxResult {
  if (character.length !== 1 || character.trim().length === 0) return { context, effects: [] };
  const lower = character.toLocaleLowerCase();
  const continuing = now - context.typeaheadAt <= TYPEAHEAD_TIMEOUT_MS;
  const repeated = continuing && context.typeahead.length > 0 && [...context.typeahead].every((item) => item === lower);
  const query = continuing ? (repeated ? lower : context.typeahead + lower) : lower;
  const enabled = listboxEnabledItems(context);
  if (enabled.length === 0) return { context: { ...context, typeahead: query, typeaheadAt: now }, effects: [] };

  const start = enabledIndex(context, context.focusedValue);
  const focusedItem = enabled[start];
  if (
    continuing &&
    !repeated &&
    context.typeahead.length > 0 &&
    focusedItem?.label.trim().toLocaleLowerCase().startsWith(query)
  ) {
    const focused = focusResult({ ...context, typeahead: query, typeaheadAt: now }, focusedItem.value, focusedItem.value);
    if (context.selectionMode !== "single") return focused;
    const selected = withSelection(focused.context, [focusedItem.value]);
    return { context: selected.context, effects: [...focused.effects, ...selected.effects] };
  }

  for (let offset = 1; offset <= enabled.length; offset += 1) {
    const index = (Math.max(start, -1) + offset) % enabled.length;
    const item = enabled[index];
    if (item?.label.trim().toLocaleLowerCase().startsWith(query)) {
      const focused = focusResult({ ...context, typeahead: query, typeaheadAt: now }, item.value, item.value);
      if (context.selectionMode !== "single") return focused;
      const selected = withSelection(focused.context, [item.value]);
      return { context: selected.context, effects: [...focused.effects, ...selected.effects] };
    }
  }
  return { context: { ...context, typeahead: query, typeaheadAt: now }, effects: [] };
}

function selectRange(context: ListboxContext, value: string, additive: boolean): ListboxResult {
  const anchor = context.anchorValue ?? context.focusedValue ?? value;
  const anchorIndex = context.items.findIndex((item) => item.value === anchor);
  const targetIndex = context.items.findIndex((item) => item.value === value);
  if (anchorIndex < 0 || targetIndex < 0) return { context, effects: [] };
  const [start, end] = anchorIndex <= targetIndex ? [anchorIndex, targetIndex] : [targetIndex, anchorIndex];
  const range = context.items.slice(start, end + 1).filter((item) => !item.disabled).map((item) => item.value);
  return withSelection({ ...context, focusedValue: value, anchorValue: anchor }, additive
    ? [...context.selectedValues, ...range]
    : range);
}

export function listboxTransition(context: ListboxContext, event: ListboxEvent): ListboxResult {
  if (context.disabled) return { context, effects: [] };

  switch (event.type) {
    case "FOCUS": {
      const item = context.items.find((candidate) => candidate.value === event.value && !candidate.disabled);
      return item ? focusResult(context, item.value) : { context, effects: [] };
    }
    case "MOVE":
      return moveFocus(context, event.direction, event.extendSelection === true);
    case "BOUNDARY":
      return focusBoundary(context, event.boundary);
    case "TYPEAHEAD":
      return focusTypeahead(context, event.character, event.now);
    case "SPACE": {
      const value = context.focusedValue;
      if (!value || !context.items.some((item) => item.value === value && !item.disabled)) return { context, effects: [] };
      if (context.selectionMode === "single") return withSelection(context, [value]);
      const selected = selectionTransition(context.selectedValues, { type: "TOGGLE", item: value }, { mode: "multiple" });
      return withSelection({ ...context, anchorValue: value }, selected);
    }
    case "SELECT": {
      const item = context.items.find((candidate) => candidate.value === event.value && !candidate.disabled);
      if (!item) return { context, effects: [] };
      if (context.selectionMode === "single") {
        const result = withSelection({ ...context, focusedValue: item.value, anchorValue: item.value }, [item.value]);
        return { context: result.context, effects: [{ type: "focus", value: item.value }, ...result.effects] };
      }
      if (event.range) {
        const result = selectRange(context, item.value, event.additive === true);
        return { context: result.context, effects: [{ type: "focus", value: item.value }, ...result.effects] };
      }
      if (event.additive) {
        const values = selectionTransition(context.selectedValues, { type: "TOGGLE", item: item.value }, { mode: "multiple" });
        const result = withSelection({ ...context, focusedValue: item.value, anchorValue: item.value }, values);
        return { context: result.context, effects: [{ type: "focus", value: item.value }, ...result.effects] };
      }
      const result = withSelection({ ...context, focusedValue: item.value, anchorValue: item.value }, [item.value]);
      return { context: result.context, effects: [{ type: "focus", value: item.value }, ...result.effects] };
    }
    case "SELECT_ALL":
      return context.selectionMode === "multiple"
        ? withSelection(context, listboxEnabledItems(context).map((item) => item.value))
        : { context, effects: [] };
    case "ACTIVATE": {
      const value = event.value ?? context.focusedValue;
      return value && context.items.some((item) => item.value === value && !item.disabled)
        ? { context, effects: [{ type: "activate", value }] }
        : { context, effects: [] };
    }
  }
}
