/**
 * Checkbox behavior machine.
 * Contract: docs/contracts/components/checkbox.md, "Behavior Machine".
 *
 * Trivial case: one implicit state; the value lives in context. Transitions
 * are pure; callbacks and DOM fixups are emitted as effects.
 */

import type { PartAttrs } from "./machine";

export interface CheckboxContext {
  checked: boolean;
  mixed: boolean;
  disabled: boolean;
  readOnly: boolean;
}

export type CheckboxEvent =
  | { type: "TOGGLE"; nextChecked: boolean }
  | { type: "SET_CHECKED"; checked: boolean };

export type CheckboxEffect =
  | { type: "revertNativeChecked" }
  | { type: "emitCheckedChange"; checked: boolean };

export interface CheckboxResult {
  context: CheckboxContext;
  effects: CheckboxEffect[];
}

export function checkboxTransition(context: CheckboxContext, event: CheckboxEvent): CheckboxResult {
  switch (event.type) {
    case "TOGGLE": {
      if (context.disabled) {
        return { context, effects: [] };
      }

      if (context.readOnly) {
        return { context, effects: [{ type: "revertNativeChecked" }] };
      }

      const checked = context.mixed ? true : event.nextChecked;

      return {
        context: { ...context, checked },
        effects: [{ type: "emitCheckedChange", checked }],
      };
    }
    case "SET_CHECKED": {
      return { context: { ...context, checked: event.checked }, effects: [] };
    }
  }
}

export function checkboxState(context: CheckboxContext): "checked" | "unchecked" | "mixed" {
  if (context.mixed) {
    return "mixed";
  }

  return context.checked ? "checked" : "unchecked";
}

export interface CheckboxPartProps {
  id?: string | undefined;
  ariaLabel?: string | null;
  describedBy?: string | null;
  hasVisibleLabel: boolean;
}

export interface CheckboxParts {
  root: PartAttrs;
  control: PartAttrs;
  indicator: PartAttrs;
  label: PartAttrs;
}

export function checkboxParts(context: CheckboxContext, props: CheckboxPartProps): CheckboxParts {
  return {
    root: {
      "data-scope": "checkbox",
      "data-part": "root",
      "data-state": checkboxState(context),
      "data-disabled": context.disabled,
    },
    control: {
      "data-part": "control",
      id: props.id,
      type: "checkbox",
      checked: context.checked,
      disabled: context.disabled,
      "aria-label": props.hasVisibleLabel ? undefined : props.ariaLabel ?? undefined,
      "aria-describedby": props.describedBy ?? undefined,
      "aria-readonly": context.readOnly ? "true" : undefined,
    },
    indicator: {
      "data-part": "indicator",
      "aria-hidden": "true",
    },
    label: {
      "data-part": "label",
    },
  };
}
