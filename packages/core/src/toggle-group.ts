/**
 * ToggleGroup behavior machine.
 * Contract: docs/contracts/components/toggle-group.md, "Behavior Machine".
 *
 * Single mode selects (optionally deactivates on reselect); multiple mode
 * toggles membership of a value array. One implicit state; value in context.
 */

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
