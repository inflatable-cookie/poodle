/**
 * ToggleGroup behavior machine.
 * Contract: docs/contracts/components/toggle-group.md, "Behavior Machine".
 *
 * Single mode selects (optionally deactivates on reselect); multiple mode
 * toggles membership of a value array. One implicit state; value in context.
 * Single-mode tab-stop and Left/Right targets are pure helpers; adapters own
 * the focus request.
 */

import { findNextEnabledIndex } from "./nav";

export type ToggleGroupValue = string | string[] | null;

export interface ToggleGroupContext {
  value: ToggleGroupValue;
  options: { value: string; disabled?: boolean }[];
  selectionMode: "single" | "multiple";
  allowDeactivation: boolean;
  disabled: boolean;
}

export type ToggleGroupEvent =
  | { type: "TOGGLE"; value: string }
  | { type: "SET_VALUE"; value: ToggleGroupValue };

export type ToggleGroupEffect = { type: "emitValueChange"; value: ToggleGroupValue };

export interface ToggleGroupResult {
  context: ToggleGroupContext;
  effects: ToggleGroupEffect[];
}

export function toggleGroupIsSelected(context: ToggleGroupContext, optionValue: string): boolean {
  if (context.selectionMode === "multiple") {
    return Array.isArray(context.value) && context.value.includes(optionValue);
  }

  return context.value === optionValue;
}

/** Enabled option values in declared order. Empty when the group is disabled. */
export function toggleGroupEnabledValues(context: ToggleGroupContext): string[] {
  if (context.disabled) {
    return [];
  }

  return context.options.filter((option) => !option.disabled).map((option) => option.value);
}

/**
 * Single-mode tab stop: the selected enabled option, otherwise the first
 * enabled option. `null` when the group is multiple, disabled, or empty of
 * enabled options.
 */
export function toggleGroupTabStopValue(context: ToggleGroupContext): string | null {
  if (context.selectionMode !== "single") {
    return null;
  }

  const enabled = toggleGroupEnabledValues(context);

  if (enabled.length === 0) {
    return null;
  }

  if (typeof context.value === "string" && enabled.includes(context.value)) {
    return context.value;
  }

  return enabled[0] ?? null;
}

/**
 * Single-mode Left/Right target from `fromValue`. Wraps and skips disabled
 * options. `null` in multiple mode, when the group is disabled, when
 * `fromValue` is unknown, or when no other enabled option exists.
 */
export function toggleGroupArrowTarget(
  context: ToggleGroupContext,
  fromValue: string,
  direction: 1 | -1,
): string | null {
  if (context.selectionMode !== "single" || context.disabled) {
    return null;
  }

  const startIndex = context.options.findIndex((option) => option.value === fromValue);

  if (startIndex < 0) {
    return null;
  }

  const nextIndex = findNextEnabledIndex(context.options, startIndex, direction);

  if (nextIndex < 0 || nextIndex === startIndex) {
    return null;
  }

  return context.options[nextIndex]?.value ?? null;
}

export function toggleGroupTransition(
  context: ToggleGroupContext,
  event: ToggleGroupEvent,
): ToggleGroupResult {
  switch (event.type) {
    case "TOGGLE": {
      const option = context.options.find((candidate) => candidate.value === event.value);

      if (context.disabled || !option || option.disabled) {
        return { context, effects: [] };
      }

      let nextValue: ToggleGroupValue;

      if (context.selectionMode === "multiple") {
        const current = Array.isArray(context.value) ? context.value : [];
        nextValue = current.includes(event.value)
          ? current.filter((item) => item !== event.value)
          : [...current, event.value];
      } else if (context.allowDeactivation && context.value === event.value) {
        nextValue = null;
      } else {
        nextValue = event.value;
      }

      return {
        context: { ...context, value: nextValue },
        effects: [{ type: "emitValueChange", value: nextValue }],
      };
    }
    case "SET_VALUE": {
      return { context: { ...context, value: event.value }, effects: [] };
    }
  }
}
