/**
 * Disclosure behavior machine (Collapsible, CollapseToggle).
 * Contracts: docs/contracts/components/collapsible.md, collapse-toggle.md,
 * "Behavior Machine" sections.
 *
 * Single open/closed boolean in context; toggling emits the callback as an
 * effect. Accordion is not this machine — it reuses toggleGroupTransition
 * (open values are a selection over items).
 */

export interface DisclosureContext {
  open: boolean;
  disabled: boolean;
}

export type DisclosureEvent = { type: "TOGGLE" } | { type: "SET_OPEN"; open: boolean };

export type DisclosureEffect = { type: "emitOpenChange"; open: boolean };

export interface DisclosureResult {
  context: DisclosureContext;
  effects: DisclosureEffect[];
}

export function disclosureTransition(
  context: DisclosureContext,
  event: DisclosureEvent,
): DisclosureResult {
  switch (event.type) {
    case "TOGGLE": {
      if (context.disabled) {
        return { context, effects: [] };
      }

      const open = !context.open;

      return { context: { ...context, open }, effects: [{ type: "emitOpenChange", open }] };
    }
    case "SET_OPEN": {
      return { context: { ...context, open: event.open }, effects: [] };
    }
  }
}
