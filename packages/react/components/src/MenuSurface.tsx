import { forwardRef, useImperativeHandle, useRef, useState, type CSSProperties } from "react";
import { menuListCanActivate, menuListNavigate, menuNavigableItems } from "@poodle/headless";

import "@poodle/styles/menu-surface.css";

import type { ControlDensity, ControlSize, MenuItem, OverlayPlacement } from "./types";

export interface MenuSurfaceHandle {
  focusFirstItem: () => void;
  moveHighlight: (direction: 1 | -1) => void;
  moveToBoundary: (boundary: "start" | "end") => void;
  element: HTMLDivElement | null;
}

export interface MenuSurfaceProps {
  items?: MenuItem[];
  ariaLabel?: string | null;
  size?: ControlSize;
  density?: ControlDensity;
  overlayStyle?: CSSProperties;
  placement?: OverlayPlacement | null;
  onAction?: (value: string) => void;
}

export const MenuSurface = forwardRef<MenuSurfaceHandle, MenuSurfaceProps>(function MenuSurface(
  { items = [], ariaLabel = null, size = "md", density = "default", overlayStyle, placement = null, onAction },
  ref,
) {
  const overlayRef = useRef<HTMLDivElement | null>(null);
  const itemRefs = useRef<Array<HTMLButtonElement | null>>([]);
  const [highlightIndex, setHighlightIndex] = useState(0);

  const actionableItems = menuNavigableItems(items);
  const effectiveHighlight =
    actionableItems.length === 0
      ? 0
      : highlightIndex >= actionableItems.length || actionableItems[highlightIndex]?.disabled
        ? menuListNavigate(actionableItems, 0, "first")
        : highlightIndex;

  function focusIndex(index: number): void {
    setHighlightIndex(index);
    itemRefs.current[index]?.focus();
  }

  function moveHighlight(direction: 1 | -1): void {
    if (actionableItems.length === 0) return;
    focusIndex(menuListNavigate(actionableItems, effectiveHighlight, direction === 1 ? "next" : "prev"));
  }

  function moveToBoundary(boundary: "start" | "end"): void {
    if (actionableItems.length === 0) return;
    focusIndex(menuListNavigate(actionableItems, effectiveHighlight, boundary === "start" ? "first" : "last"));
  }

  useImperativeHandle(ref, () => ({
    focusFirstItem: () => {
      if (actionableItems.length === 0) return;
      focusIndex(menuListNavigate(actionableItems, effectiveHighlight, "first"));
    },
    moveHighlight,
    moveToBoundary,
    get element() {
      return overlayRef.current;
    },
  }));

  function activateItem(item: MenuItem): void {
    if (!menuListCanActivate(item)) return;
    onAction?.(item.value);
  }

  return (
    <div
      ref={overlayRef}
      className="poodle-menu-surface"
      data-size={size}
      data-density={density}
      data-placement={placement ?? undefined}
      style={overlayStyle}
      role="menu"
      aria-label={ariaLabel ?? undefined}
    >
      {items.map((item, itemIndex) =>
        item.kind === "separator" ? (
          <div key={`sep-${itemIndex}`} className="poodle-menu-surface__separator" role="separator" />
        ) : (
          <button
            key={item.value}
            ref={(node) => {
              const idx = actionableItems.findIndex((candidate) => candidate.value === item.value);
              if (idx >= 0) itemRefs.current[idx] = node;
            }}
            type="button"
            className="poodle-menu-surface__item"
            disabled={item.disabled === true}
            data-kind={item.kind ?? "action"}
            data-tone={item.tone ?? "default"}
            role={item.kind === "checkbox" || item.kind === "radio" ? `menuitem${item.kind}` : "menuitem"}
            aria-checked={item.kind === "checkbox" || item.kind === "radio" ? item.checked === true : undefined}
            onClick={() => activateItem(item)}
            onKeyDown={(event) => {
              if (event.key === "ArrowDown") {
                event.preventDefault();
                moveHighlight(1);
              }
              if (event.key === "ArrowUp") {
                event.preventDefault();
                moveHighlight(-1);
              }
              if (event.key === "Home") {
                event.preventDefault();
                moveToBoundary("start");
              }
              if (event.key === "End") {
                event.preventDefault();
                moveToBoundary("end");
              }
              if (event.key === "Enter" || event.key === " ") {
                event.preventDefault();
                activateItem(item);
              }
            }}
          >
            <span className="poodle-menu-surface__label">{item.label}</span>
            {item.checked ? (
              <span className="poodle-menu-surface__meta" aria-hidden="true">
                ✓
              </span>
            ) : item.shortcutLabel ? (
              <span className="poodle-menu-surface__meta" aria-hidden="true">
                {item.shortcutLabel}
              </span>
            ) : null}
          </button>
        ),
      )}
    </div>
  );
});
