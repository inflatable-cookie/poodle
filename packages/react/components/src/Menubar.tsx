import "@poodle/styles/menubar.css";

import { useEffect, useId, useRef, useState } from "react";

import {
  findNextEnabledIndex,
  firstEnabledIndex,
  menuListCanActivate,
  menuListNavigate,
  menuNavigableItems,
  registerDismissLayer,
} from "@poodle/headless";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, MenubarItem, MenuItem, SemanticControlSizeRole } from "./types";

export interface MenubarProps {
  value?: string | null;
  defaultValue?: string | null;
  items?: MenubarItem[];
  ariaLabel?: string | null;
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  onValueChange?: ((value: string | null) => void) | undefined;
  onAction?: ((value: string) => void) | undefined;
}

export function Menubar({
  value: controlledValue,
  defaultValue = null,
  items = [],
  ariaLabel = null,
  sizeRole = "chrome",
  size = null,
  density = null,
  onValueChange = undefined,
  onAction = undefined,
}: MenubarProps) {
  const menubarId = useId();
  const uiPresentation = useUiPresentation();

  const rootRef = useRef<HTMLDivElement | null>(null);
  const triggerRefs = useRef<Array<HTMLButtonElement | null>>([]);
  const menuItemRefs = useRef<Array<HTMLButtonElement | null>>([]);
  const focusIndexRef = useRef(firstEnabledIndex(items));
  const highlightRef = useRef(0);
  const pendingMenuFocus = useRef(false);

  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);

  const isControlled = controlledValue !== undefined && controlledValue !== null;
  const currentValue = controlledValue ?? uncontrolledValue;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const currentMenu = items.find((item) => item.value === currentValue) ?? null;
  const actionableItems = menuNavigableItems(currentMenu?.items ?? []);
  const selectedIndex = items.findIndex((item) => item.value === currentValue);

  if (selectedIndex >= 0) {
    focusIndexRef.current = selectedIndex;
  }

  function setValue(nextValue: string | null): void {
    if (!isControlled) {
      setUncontrolledValue(nextValue);
    }
    if (nextValue !== null) {
      highlightRef.current = 0;
      pendingMenuFocus.current = true;
    }
    onValueChange?.(nextValue);
  }

  useEffect(() => {
    if (currentValue && actionableItems.length > 0 && pendingMenuFocus.current) {
      pendingMenuFocus.current = false;
      menuItemRefs.current[highlightRef.current]?.focus();
    }
  });

  function moveTriggerFocus(nextIndex: number): void {
    focusIndexRef.current = nextIndex;
    triggerRefs.current[nextIndex]?.focus();
  }

  function openMenuAtIndex(index: number): void {
    const nextValue = items[index]?.value;

    if (!nextValue) {
      return;
    }

    focusIndexRef.current = index;
    setValue(nextValue);
  }

  function moveMenuHighlight(direction: 1 | -1): void {
    if (actionableItems.length === 0) {
      return;
    }

    const nextIndex = menuListNavigate(actionableItems, highlightRef.current, direction === 1 ? "next" : "prev");
    highlightRef.current = nextIndex;
    menuItemRefs.current[nextIndex]?.focus();
  }

  function activateItem(item: MenuItem): void {
    if (!menuListCanActivate(item)) {
      return;
    }

    onAction?.(item.value);
    setValue(null);
    triggerRefs.current[focusIndexRef.current]?.focus();
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
    <div ref={rootRef} className="poodle-menubar" data-size={resolvedSize} data-density={resolvedDensity}>
      <div className="poodle-menubar__list" role="menubar" aria-label={ariaLabel ?? undefined}>
        {items.map((item, index) => (
          <div key={item.value} className="poodle-menubar__group">
            <button
              ref={(el) => {
                triggerRefs.current[index] = el;
              }}
              type="button"
              className="poodle-menubar__trigger"
              data-open={currentValue === item.value}
              disabled={item.disabled === true}
              role="menuitem"
              aria-haspopup="menu"
              aria-expanded={currentValue === item.value ? "true" : "false"}
              aria-controls={
                currentValue === item.value ? `poodle-menubar-menu-${menubarId}-${item.value}` : undefined
              }
              onFocus={() => {
                focusIndexRef.current = index;
              }}
              onClick={() => setValue(currentValue === item.value ? null : item.value)}
              onMouseEnter={() => {
                if (currentValue !== null && currentValue !== item.value && !item.disabled) {
                  openMenuAtIndex(index);
                }
              }}
              onKeyDown={(event) => {
                if (event.key === "ArrowRight") {
                  event.preventDefault();
                  moveTriggerFocus(findNextEnabledIndex(items, index, 1));
                }

                if (event.key === "ArrowLeft") {
                  event.preventDefault();
                  moveTriggerFocus(findNextEnabledIndex(items, index, -1));
                }

                if (event.key === "Home") {
                  event.preventDefault();
                  moveTriggerFocus(firstEnabledIndex(items));
                }

                if (event.key === "End") {
                  event.preventDefault();
                  moveTriggerFocus(findNextEnabledIndex(items, 0, -1));
                }

                if (event.key === "ArrowDown" || event.key === "Enter" || event.key === " ") {
                  event.preventDefault();
                  openMenuAtIndex(index);
                }

                if (event.key === "Escape") {
                  event.preventDefault();
                  setValue(null);
                }
              }}
            >
              {item.label}
            </button>

            {currentValue === item.value ? (
              <div
                id={`poodle-menubar-menu-${menubarId}-${item.value}`}
                className="poodle-menubar__overlay"
                role="menu"
                aria-label={item.label}
              >
                {item.items.map((menuItem) =>
                  menuItem.kind === "separator" ? (
                    <div key={menuItem.value} className="poodle-menubar__separator" role="separator" />
                  ) : (
                    <button
                      key={menuItem.value}
                      ref={(el) => {
                        const itemIndex = actionableItems.findIndex(
                          (candidate) => candidate.value === menuItem.value,
                        );
                        if (itemIndex >= 0) {
                          menuItemRefs.current[itemIndex] = el;
                        }
                      }}
                      type="button"
                      className="poodle-menubar__item"
                      disabled={menuItem.disabled === true}
                      role={
                        menuItem.kind === "checkbox" || menuItem.kind === "radio"
                          ? `menuitem${menuItem.kind}`
                          : "menuitem"
                      }
                      aria-checked={
                        menuItem.kind === "checkbox" || menuItem.kind === "radio"
                          ? menuItem.checked
                            ? "true"
                            : "false"
                          : undefined
                      }
                      onClick={() => activateItem(menuItem)}
                      onKeyDown={(event) => {
                        if (event.key === "ArrowDown") {
                          event.preventDefault();
                          moveMenuHighlight(1);
                        }

                        if (event.key === "ArrowUp") {
                          event.preventDefault();
                          moveMenuHighlight(-1);
                        }

                        if (event.key === "Home") {
                          event.preventDefault();
                          highlightRef.current = 0;
                          menuItemRefs.current[0]?.focus();
                        }

                        if (event.key === "End") {
                          event.preventDefault();
                          highlightRef.current = actionableItems.length - 1;
                          menuItemRefs.current[actionableItems.length - 1]?.focus();
                        }

                        if (event.key === "ArrowRight") {
                          event.preventDefault();
                          const nextIndex = findNextEnabledIndex(items, index, 1);
                          openMenuAtIndex(nextIndex);
                        }

                        if (event.key === "ArrowLeft") {
                          event.preventDefault();
                          const nextIndex = findNextEnabledIndex(items, index, -1);
                          openMenuAtIndex(nextIndex);
                        }

                        if (event.key === "Escape") {
                          event.preventDefault();
                          setValue(null);
                          triggerRefs.current[index]?.focus();
                        }

                        if (event.key === "Enter" || event.key === " ") {
                          event.preventDefault();
                          activateItem(menuItem);
                        }
                      }}
                    >
                      <span className="poodle-menubar__label">{menuItem.label}</span>

                      {menuItem.checked ? (
                        <span className="poodle-menubar__meta" aria-hidden="true">
                          ✓
                        </span>
                      ) : menuItem.shortcutLabel ? (
                        <span className="poodle-menubar__meta" aria-hidden="true">
                          {menuItem.shortcutLabel}
                        </span>
                      ) : null}
                    </button>
                  ),
                )}
              </div>
            ) : null}
          </div>
        ))}
      </div>
    </div>
  );
}
