/**
 * Switch behavior machine.
 * Contract: docs/contracts/components/switch.md, "Behavior Machine".
 *
 * Checkbox semantics without the mixed state: single implicit state, value in
 * context, readOnly reverts, callbacks as effects.
 */

export interface SwitchContext {
  checked: boolean;
  disabled: boolean;
  readOnly: boolean;
}

export type SwitchEvent =
  | { type: "TOGGLE"; nextChecked: boolean }
  | { type: "SET_CHECKED"; checked: boolean };

export type SwitchEffect =
  | { type: "revertNativeChecked" }
  | { type: "emitCheckedChange"; checked: boolean };

export interface SwitchResult {
  context: SwitchContext;
  effects: SwitchEffect[];
}

export function switchTransition(context: SwitchContext, event: SwitchEvent): SwitchResult {
  switch (event.type) {
    case "TOGGLE": {
      if (context.disabled) {
        return { context, effects: [] };
      }

      if (context.readOnly) {
        return { context, effects: [{ type: "revertNativeChecked" }] };
      }

      return {
        context: { ...context, checked: event.nextChecked },
        effects: [{ type: "emitCheckedChange", checked: event.nextChecked }],
      };
    }
    case "SET_CHECKED": {
      return { context: { ...context, checked: event.checked }, effects: [] };
    }
  }
}

export function switchState(context: SwitchContext): "checked" | "unchecked" {
  return context.checked ? "checked" : "unchecked";
}
