import { useRef, useState, type ReactNode } from "react";
import {
  tabsKeydownEvent,
  tabsTransition,
  type TabsContext as HeadlessTabsContext,
  type TabsEvent,
} from "@poodle/headless";

import "./tabs.css";

export interface TabsItem {
  value: string;
  label: string;
  disabled?: boolean;
}

export interface TabsProps {
  items: TabsItem[];
  value?: string;
  defaultValue?: string;
  activationMode?: "automatic" | "manual";
  ariaLabel?: string;
  onValueChange?: (value: string) => void;
  children?: (value: string) => ReactNode;
}

let instanceCounter = 0;

/**
 * Machine-backed shell: selection, roving focus, and keyboard behavior come
 * from the shared `tabsTransition` machine — the same one the Svelte layer
 * and the Rust mirror run.
 */
export function Tabs({
  items,
  value,
  defaultValue,
  activationMode = "automatic",
  ariaLabel,
  onValueChange,
  children,
}: TabsProps) {
  const instanceId = useRef(++instanceCounter).current;
  const tabRefs = useRef<Array<HTMLButtonElement | null>>([]);
  const isControlled = value !== undefined;
  const firstEnabled = items.find((item) => !item.disabled)?.value ?? null;
  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue ?? null);
  const currentValue = (isControlled ? value : uncontrolledValue) ?? firstEnabled;
  const selectedIndex = items.findIndex((item) => item.value === currentValue);
  const [focusIndex, setFocusIndex] = useState(Math.max(selectedIndex, 0));

  const machineContext: HeadlessTabsContext<TabsItem> = {
    items,
    value: currentValue,
    focusIndex,
    activationMode,
    reorderable: false,
  };

  function send(event: TabsEvent) {
    const result = tabsTransition(machineContext, event);

    if (result.context.focusIndex !== focusIndex) {
      setFocusIndex(result.context.focusIndex);
    }

    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        if (!isControlled) {
          setUncontrolledValue(effect.value);
        }

        onValueChange?.(effect.value);
      } else if (effect.type === "focusTab") {
        const index = effect.index;
        queueMicrotask(() => tabRefs.current[index]?.focus());
      }
    }
  }

  return (
    <div className="poodle-tabs" data-variant="text" data-bordered="true" data-orientation="horizontal" data-size="md" data-density="default">
      <div className="poodle-tabs__list" role="tablist" aria-label={ariaLabel}>
        {items.map((item, index) => (
          <div className="poodle-tabs__item" role="presentation" data-selected={currentValue === item.value} key={item.value}>
            <button
              ref={(node) => {
                tabRefs.current[index] = node;
              }}
              type="button"
              className="poodle-tabs__tab"
              disabled={item.disabled === true}
              id={`poodle-react-tab-${instanceId}-${item.value}`}
              role="tab"
              tabIndex={focusIndex === index ? 0 : -1}
              aria-selected={currentValue === item.value ? "true" : "false"}
              aria-controls={children ? `poodle-react-tabpanel-${instanceId}-${item.value}` : undefined}
              onFocus={() => setFocusIndex(index)}
              onClick={() => send({ type: "SELECT", value: item.value })}
              onKeyDown={(event) => {
                const machineEvent = tabsKeydownEvent(
                  event.key,
                  event.altKey,
                  "horizontal",
                  { reorderable: false, activationMode },
                  index,
                );

                if (machineEvent) {
                  event.preventDefault();
                  send(machineEvent);
                }
              }}
            >
              <span className="poodle-tabs__label">{item.label}</span>
            </button>
          </div>
        ))}
      </div>

      {children && currentValue && (
        <div
          className="poodle-tabs__panel"
          id={`poodle-react-tabpanel-${instanceId}-${currentValue}`}
          role="tabpanel"
          tabIndex={0}
          aria-labelledby={`poodle-react-tab-${instanceId}-${currentValue}`}
        >
          {children(currentValue)}
        </div>
      )}
    </div>
  );
}
