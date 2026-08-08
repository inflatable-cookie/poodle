import { useId, useState, type ReactNode } from "react";
import { toggleGroupTransition } from "@inflatable-cookie/poodle-headless";

import "@inflatable-cookie/poodle-styles/accordion.css";

import { Icon } from "./Icon";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { AccordionItem, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface AccordionProps {
  items?: AccordionItem[];
  value?: string | string[] | null;
  defaultValue?: string | string[] | null;
  selectionMode?: "single" | "multiple";
  collapsible?: boolean;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onValueChange?: (value: string | string[] | null) => void;
  children?: (item: AccordionItem, open: boolean) => ReactNode;
}

export function Accordion({
  items = [],
  value = null,
  defaultValue = null,
  selectionMode = "single",
  collapsible = true,
  ariaLabel = null,
  size = null,
  sizeRole = "control",
  density = null,
  onValueChange,
  children,
}: AccordionProps) {
  const uiPresentation = useUiPresentation();
  const accordionId = useId();
  const [uncontrolledValue, setUncontrolledValue] = useState<string | string[] | null>(
    () => defaultValue ?? (selectionMode === "multiple" ? [] : null),
  );

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = value !== null;
  const currentValue = isControlled ? value : uncontrolledValue;
  const openValues = Array.isArray(currentValue) ? currentValue : currentValue ? [currentValue] : [];

  function toggle(itemValue: string): void {
    const result = toggleGroupTransition(
      {
        value: selectionMode === "multiple" ? openValues : typeof currentValue === "string" ? currentValue : null,
        options: items.map((item) => ({ value: item.value, disabled: item.disabled === true })),
        selectionMode,
        allowDeactivation: collapsible,
        disabled: false,
      },
      { type: "TOGGLE", value: itemValue },
    );
    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        if (!isControlled) setUncontrolledValue(effect.value);
        onValueChange?.(effect.value);
      }
    }
  }

  return (
    <div
      className="poodle-accordion"
      role={selectionMode === "multiple" ? "group" : undefined}
      aria-label={ariaLabel ?? undefined}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      {items.map((item) => {
        const open = openValues.includes(item.value);
        const triggerId = `${accordionId}-trigger-${item.value}`;
        const panelId = `${accordionId}-panel-${item.value}`;
        return (
          <section key={item.value} className="poodle-accordion__item" data-open={open}>
            <h3 className="poodle-accordion__heading">
              <button
                type="button"
                className="poodle-accordion__trigger"
                id={triggerId}
                disabled={item.disabled === true}
                aria-expanded={open}
                aria-controls={panelId}
                onClick={() => toggle(item.value)}
              >
                <span className="poodle-accordion__summary">
                  <span className="poodle-accordion__title">{item.label}</span>
                  {item.description ? <span className="poodle-accordion__description">{item.description}</span> : null}
                </span>
                <span className="poodle-accordion__indicator" aria-hidden="true">
                  <Icon name="chevron-down" />
                </span>
              </button>
            </h3>

            {open ? (
              <div className="poodle-accordion__panel" id={panelId} role="region" aria-labelledby={triggerId}>
                {children?.(item, true)}
              </div>
            ) : null}
          </section>
        );
      })}
    </div>
  );
}
