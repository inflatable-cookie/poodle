import { useEffect, useId, useRef, useState, type ReactNode } from "react";
import { toggleGroupTransition } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/accordion.css";

import { Icon } from "./Icon";
import { useMotionReady } from "./motion-policy";
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
  const motionReady = useMotionReady();
  const accordionId = useId();
  const [uncontrolledValue, setUncontrolledValue] = useState<string | string[] | null>(
    () => defaultValue ?? (selectionMode === "multiple" ? [] : null),
  );

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = value !== null;
  const currentValue = isControlled ? value : uncontrolledValue;
  const openValues = Array.isArray(currentValue) ? currentValue : currentValue ? [currentValue] : [];
  const [closing, setClosing] = useState(() => new Set<string>());
  const previousOpen = useRef(new Set(openValues));

  useEffect(() => {
    const next = new Set(openValues);
    setClosing((current) => {
      const nextClosing = new Set(current);
      for (const value of next) {
        nextClosing.delete(value);
      }
      if (motionReady) {
        for (const value of previousOpen.current) {
          if (!next.has(value)) {
            nextClosing.add(value);
          }
        }
      } else {
        nextClosing.clear();
      }
      return nextClosing;
    });
    previousOpen.current = next;
  }, [motionReady, openValues.join("\u001f")]);

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
        if (motionReady) {
          const next = Array.isArray(effect.value)
            ? effect.value
            : effect.value
              ? [effect.value]
              : [];
          setClosing((current) => {
            const nextClosing = new Set(current);
            for (const value of next) {
              nextClosing.delete(value);
            }
            for (const value of openValues) {
              if (!next.includes(value)) {
                nextClosing.add(value);
              }
            }
            return nextClosing;
          });
        }
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
      data-motion-ready={motionReady}
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

            <div
              className="poodle-accordion__panel-clip"
              onTransitionEnd={() => {
                if (!open) {
                  setClosing((current) => {
                    const nextClosing = new Set(current);
                    nextClosing.delete(item.value);
                    return nextClosing;
                  });
                }
              }}
            >
              <div
                className="poodle-accordion__panel"
                id={panelId}
                role="region"
                aria-labelledby={triggerId}
                hidden={!open && !closing.has(item.value)}
                inert={!open}
                aria-hidden={!open}
              >
                {children?.(item, open)}
              </div>
            </div>
          </section>
        );
      })}
    </div>
  );
}
