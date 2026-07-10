/**
 * Single-select behavior machine, shared by RadioGroup, SegmentedControl, and
 * TriStateSwitch (contracts: radio-group.md, segmented-control.md,
 * tri-state-switch.md, "Behavior Machine" sections).
 *
 * One implicit state; the selected value lives in context. Selecting the
 * current value is inert — native radio inputs cannot re-fire an unchanged
 * selection, so the guard is behavior-invariant for UI paths.
 */

export interface SelectOption {
  value: string;
  disabled?: boolean;
}

export interface SingleSelectContext {
  value: string | null;
  options: SelectOption[];
  disabled: boolean;
}

export type SingleSelectEvent =
  | { type: "SELECT"; value: string }
  | { type: "SET_VALUE"; value: string | null };

export type SingleSelectEffect = { type: "emitValueChange"; value: string };

export interface SingleSelectResult {
  context: SingleSelectContext;
  effects: SingleSelectEffect[];
}

export function singleSelectTransition(
  context: SingleSelectContext,
  event: SingleSelectEvent,
): SingleSelectResult {
  switch (event.type) {
    case "SELECT": {
      const option = context.options.find((candidate) => candidate.value === event.value);

      if (context.disabled || !option || option.disabled || context.value === event.value) {
        return { context, effects: [] };
      }

      return {
        context: { ...context, value: event.value },
        effects: [{ type: "emitValueChange", value: event.value }],
      };
    }
    case "SET_VALUE": {
      return { context: { ...context, value: event.value }, effects: [] };
    }
  }
}
