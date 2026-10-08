import { useEffect, useMemo, useRef, useState, type KeyboardEvent, type ReactNode } from "react";
import {
  listboxInitialFocus,
  listboxTransition,
  type ListboxContext,
  type ListboxEvent,
  type ListboxItem,
  type ListboxOrientation,
  type ListboxSelectionMode,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/listbox.css";

const isDevelopment = (import.meta as ImportMeta & { env?: { DEV?: boolean } }).env?.DEV ??
  (typeof process !== "undefined" && process.env.NODE_ENV !== "production");

export interface ListboxProps {
  items?: ListboxItem[];
  selectionMode?: ListboxSelectionMode;
  value?: string | null;
  values?: string[];
  defaultValue?: string | null;
  defaultValues?: string[];
  orientation?: ListboxOrientation;
  disabled?: boolean;
  ariaLabel?: string | null;
  ariaLabelledby?: string | null;
  onValueChange?: (value: string) => void;
  onValuesChange?: (values: string[]) => void;
  onActivate?: (value: string) => void;
  renderItem?: (item: ListboxItem, selected: boolean, focused: boolean) => ReactNode;
}

export function Listbox({
  items = [],
  selectionMode = "single",
  value,
  values,
  defaultValue = null,
  defaultValues = [],
  orientation = "vertical",
  disabled = false,
  ariaLabel = null,
  ariaLabelledby = null,
  onValueChange,
  onValuesChange,
  onActivate,
  renderItem,
}: ListboxProps) {
  const rootRef = useRef<HTMLDivElement | null>(null);
  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);
  const [uncontrolledValues, setUncontrolledValues] = useState<string[]>(defaultValues);
  const [localFocus, setLocalFocus] = useState<string | null>(null);
  const [rangeAnchor, setRangeAnchor] = useState<string | null>(null);
  const [typeahead, setTypeahead] = useState("");
  const [typeaheadAt, setTypeaheadAt] = useState(0);
  const isControlled = selectionMode === "single" ? value !== undefined : values !== undefined;
  const selectedValues = selectionMode === "single"
    ? (isControlled ? (value == null ? [] : [value]) : (uncontrolledValue == null ? [] : [uncontrolledValue]))
    : (isControlled ? (values ?? []) : uncontrolledValues);
  const focusedValue = useMemo(() => {
    return items.some((item) => item.value === localFocus && !item.disabled)
      ? localFocus
      : listboxInitialFocus(items, selectedValues, disabled);
  }, [disabled, items, localFocus, selectedValues]);
  const enabledCount = disabled ? 0 : items.filter((item) => !item.disabled).length;

  function context(): ListboxContext {
    return {
      items,
      selectionMode,
      orientation,
      disabled,
      selectedValues: [...selectedValues],
      focusedValue,
      anchorValue: rangeAnchor,
      typeahead,
      typeaheadAt,
    };
  }

  function focusOption(valueToFocus: string): void {
    setLocalFocus(valueToFocus);
    const option = [...(rootRef.current?.querySelectorAll<HTMLElement>("[data-listbox-option]") ?? [])]
      .find((candidate) => candidate.dataset.listboxOption === valueToFocus);
    option?.focus();
  }

  function run(event: ListboxEvent): void {
    const result = listboxTransition(context(), event);
    setLocalFocus(result.context.focusedValue);
    setRangeAnchor(result.context.anchorValue);
    setTypeahead(result.context.typeahead);
    setTypeaheadAt(result.context.typeaheadAt);
    for (const effect of result.effects) {
      if (effect.type === "focus") focusOption(effect.value);
      else if (effect.type === "activate") onActivate?.(effect.value);
      else if (effect.type === "selectionChanged") {
        if (selectionMode === "single") {
          const next = effect.values[0] ?? null;
          if (!isControlled) setUncontrolledValue(next);
          if (next !== null) onValueChange?.(next);
        } else {
          if (!isControlled) setUncontrolledValues(effect.values);
          onValuesChange?.(effect.values);
        }
      }
    }
  }

  function handleKeydown(event: KeyboardEvent<HTMLDivElement>): void {
    if (disabled) return;
    if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "a" && selectionMode === "multiple") {
      event.preventDefault(); run({ type: "SELECT_ALL" }); return;
    }
    if ((orientation === "vertical" && event.key === "ArrowDown") || (orientation === "horizontal" && event.key === "ArrowRight")) {
      event.preventDefault(); run({ type: "MOVE", direction: 1, extendSelection: event.shiftKey }); return;
    }
    if ((orientation === "vertical" && event.key === "ArrowUp") || (orientation === "horizontal" && event.key === "ArrowLeft")) {
      event.preventDefault(); run({ type: "MOVE", direction: -1, extendSelection: event.shiftKey }); return;
    }
    if (event.key === "Home" || event.key === "End") {
      event.preventDefault(); run({ type: "BOUNDARY", boundary: event.key === "Home" ? "first" : "last" }); return;
    }
    if (event.key === " " || event.key === "Spacebar") {
      event.preventDefault(); run({ type: "SPACE" }); return;
    }
    if (event.key === "Enter") {
      event.preventDefault(); run({ type: "ACTIVATE" }); return;
    }
    if (!event.altKey && !event.ctrlKey && !event.metaKey && event.key.length === 1) {
      run({ type: "TYPEAHEAD", character: event.key, now: performance.now() });
    }
  }

  useEffect(() => {
    const root = rootRef.current;
    if (!root) return;
    let warned = false;
    const interactive = 'a[href],button,input:not([type="hidden"]),select,textarea,[tabindex]:not([tabindex="-1"]),[contenteditable]:not([contenteditable="false"]),[role="button"],[role="link"],[role="checkbox"],[role="radio"],[role="switch"],[role="combobox"],[role="textbox"],[role="slider"],[role="menuitem"],[role="option"]';
    const check = (): void => {
      if (warned) return;
      const hasInteractive = [...root.querySelectorAll<HTMLElement>("[data-listbox-option]")]
        .some((option) => option.querySelector(interactive) !== null);
      if (isDevelopment && hasInteractive) {
        warned = true;
        console.warn("Listbox options must not contain interactive descendants; Listbox owns option keyboard interaction.");
      }
    };
    check();
    const observer = new MutationObserver(check);
    observer.observe(root, { childList: true, subtree: true, attributes: true, attributeFilter: ["href", "tabindex", "role", "contenteditable", "type"] });
    return () => observer.disconnect();
  }, []);

  return (
    <div
      ref={rootRef}
      className="poodle-listbox"
      data-scope="listbox"
      data-part="root"
      data-orientation={orientation}
      data-disabled={disabled}
      role="listbox"
      aria-orientation={orientation}
      aria-label={ariaLabel ?? undefined}
      aria-labelledby={ariaLabelledby ?? undefined}
      aria-disabled={disabled || undefined}
      aria-multiselectable={selectionMode === "multiple" || undefined}
      tabIndex={disabled ? -1 : enabledCount === 0 ? 0 : undefined}
      onKeyDown={handleKeydown}
      onFocus={(event) => {
        const option = (event.target as HTMLElement).closest<HTMLElement>("[data-listbox-option]");
        if (option?.dataset.listboxOption) run({ type: "FOCUS", value: option.dataset.listboxOption });
      }}
    >
      {items.map((item) => {
        const selected = selectedValues.includes(item.value);
        const focused = focusedValue === item.value && !disabled && !item.disabled;
        return (
          <div
            key={item.value}
            className="poodle-listbox__option"
            data-scope="listbox"
            data-part="option"
            data-listbox-option={item.value}
            data-selected={selected}
            data-focused={focused}
            role="option"
            aria-label={item.label}
            aria-selected={selected}
            aria-disabled={disabled || item.disabled || undefined}
            tabIndex={focused ? 0 : -1}
            onClick={(event) => run({ type: "SELECT", value: item.value, additive: event.ctrlKey || event.metaKey, range: event.shiftKey })}
            onDoubleClick={() => run({ type: "ACTIVATE", value: item.value })}
          >
            {renderItem ? renderItem(item, selected, focused) : item.label}
          </div>
        );
      })}
    </div>
  );
}
