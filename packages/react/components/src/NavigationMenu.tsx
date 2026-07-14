import "@poodle/styles/navigation-menu.css";

import { useEffect, useId, useRef, useState, type KeyboardEvent as ReactKeyboardEvent, type ReactNode } from "react";

import { findNextEnabledIndex, firstEnabledIndex, registerDismissLayer } from "@poodle/headless";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, NavigationMenuItem, SemanticControlSizeRole } from "./types";

export interface NavigationMenuProps {
  value?: string | null;
  defaultValue?: string | null;
  items?: NavigationMenuItem[];
  ariaLabel?: string | null;
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  onValueChange?: ((value: string | null) => void) | undefined;
  children?: (value: string | null, item: NavigationMenuItem | null) => ReactNode;
}

export function NavigationMenu({
  value: controlledValue,
  defaultValue = null,
  items = [],
  ariaLabel = null,
  sizeRole = "chrome",
  size = null,
  density = null,
  onValueChange = undefined,
  children,
}: NavigationMenuProps) {
  const menuId = useId();
  const uiPresentation = useUiPresentation();

  const rootRef = useRef<HTMLDivElement | null>(null);
  const triggerRefs = useRef<Array<HTMLButtonElement | null>>([]);

  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);
  const [focusIndex, setFocusIndex] = useState(0);

  const isControlled = controlledValue !== undefined && controlledValue !== null;
  const currentValue = isControlled ? controlledValue : uncontrolledValue;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const currentItem = items.find((item) => item.value === currentValue) ?? null;
  const selectedIndex = items.findIndex((item) => item.value === currentValue);

  const firstEnabled = firstEnabledIndex(items);
  const resolvedFocusIndex =
    selectedIndex >= 0 ? selectedIndex : firstEnabled >= 0 && focusIndex === 0 ? firstEnabled : focusIndex;

  function setValue(nextValue: string | null): void {
    if (!isControlled) {
      setUncontrolledValue(nextValue);
    }
    onValueChange?.(nextValue);
  }

  function toggleValue(nextValue: string): void {
    setValue(currentValue === nextValue ? null : nextValue);
  }

  function moveFocus(nextIndex: number): void {
    setFocusIndex(nextIndex);
    triggerRefs.current[nextIndex]?.focus();
  }

  function handleKeydown(event: ReactKeyboardEvent, index: number): void {
    if (event.key === "ArrowRight") {
      event.preventDefault();
      moveFocus(findNextEnabledIndex(items, index, 1));
    }

    if (event.key === "ArrowLeft") {
      event.preventDefault();
      moveFocus(findNextEnabledIndex(items, index, -1));
    }

    if (event.key === "Home") {
      event.preventDefault();
      moveFocus(firstEnabledIndex(items));
    }

    if (event.key === "End") {
      event.preventDefault();
      moveFocus(findNextEnabledIndex(items, 0, -1));
    }

    if (event.key === "ArrowDown" || event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      const nextValue = items[index]?.value;

      if (nextValue) {
        setValue(nextValue);
      }
    }

    if (event.key === "Escape") {
      event.preventDefault();
      setValue(null);
    }
  }

  useEffect(() => {
    if (!currentValue) {
      return;
    }

    return registerDismissLayer({
      contains: (target) => rootRef.current?.contains(target) ?? false,
      dismissOnOutsideInteract: true,
      onDismiss: () => setValue(null),
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [currentValue]);

  return (
    <div ref={rootRef} className="poodle-navigation-menu" data-size={resolvedSize} data-density={resolvedDensity}>
      <nav className="poodle-navigation-menu__list" aria-label={ariaLabel ?? undefined}>
        {items.map((item, index) => (
          <button
            key={item.value}
            ref={(el) => {
              triggerRefs.current[index] = el;
            }}
            type="button"
            className="poodle-navigation-menu__trigger"
            data-open={currentValue === item.value}
            disabled={item.disabled === true}
            tabIndex={index === resolvedFocusIndex ? 0 : -1}
            id={`poodle-navigation-menu-trigger-${menuId}-${item.value}`}
            aria-expanded={currentValue === item.value ? "true" : "false"}
            aria-controls={
              currentValue === item.value ? `poodle-navigation-menu-panel-${menuId}-${item.value}` : undefined
            }
            onFocus={() => setFocusIndex(index)}
            onClick={() => toggleValue(item.value)}
            onKeyDown={(event) => handleKeydown(event, index)}
          >
            <span className="poodle-navigation-menu__label">{item.label}</span>
          </button>
        ))}
      </nav>

      {currentItem ? (
        <div
          className="poodle-navigation-menu__viewport"
          id={`poodle-navigation-menu-panel-${menuId}-${currentItem.value}`}
          aria-labelledby={`poodle-navigation-menu-trigger-${menuId}-${currentItem.value}`}
        >
          {children?.(currentItem.value, currentItem)}
        </div>
      ) : null}
    </div>
  );
}
