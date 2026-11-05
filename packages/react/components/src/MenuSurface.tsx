import { forwardRef, useImperativeHandle, useLayoutEffect, useRef, useState, type CSSProperties } from "react";
import {
  createInstanceId,
  isAnchorClipped,
  menuItemHasSubmenu,
  observeAnchorMovement,
  observeOverlaySurfaceGeometry,
  resolveClipRect,
  menuListCanActivate,
  menuListNavigate,
  menuNavigableItems,
  type AnchorTarget,
  type OverlaySurfaceGeometryChangeHandler,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/menu-surface.css";

import { AnchoredSurface } from "./AnchoredSurface";
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
  /** Anchor for a root surface. Null leaves the surface in place, positioned
   * by `overlayStyle` — the pointer-anchored and nested-flyout cases. */
  anchor?: AnchorTarget | null;
  offset?: number;
  placement?: OverlayPlacement | null;
  /** Internal cascading surface; retains DOM ancestry for root dismissal. */
  nested?: boolean;
  onRequestClose?: () => void;
  onAction?: (value: string) => void;
  onSurfaceGeometryChange?: OverlaySurfaceGeometryChangeHandler;
}

export const MenuSurface = forwardRef<MenuSurfaceHandle, MenuSurfaceProps>(function MenuSurfaceImpl(
  {
    items = [],
    ariaLabel = null,
    size = "md",
    density = "default",
    overlayStyle,
    anchor = null,
    offset = 6,
    placement = null,
    nested = false,
    onRequestClose,
    onAction,
    onSurfaceGeometryChange,
  },
  ref,
) {
  const overlayRef = useRef<HTMLDivElement | null>(null);
  // `placement` is the request; this is what survived collision resolution.
  const [resolvedPlacement, setResolvedPlacement] = useState<OverlayPlacement | null>(null);
  const itemRefs = useRef<Array<HTMLButtonElement | null>>([]);
  const [highlightIndex, setHighlightIndex] = useState(0);
  const [openSubmenuValue, setOpenSubmenuValue] = useState<string | null>(null);
  const submenuRef = useRef<MenuSurfaceHandle | null>(null);
  const pendingSubmenuFocus = useRef(false);
  const geometryHandler = useRef(onSurfaceGeometryChange);
  geometryHandler.current = onSurfaceGeometryChange;

  function attachSubmenu(handle: MenuSurfaceHandle | null): void {
    submenuRef.current = handle;
    if (handle && pendingSubmenuFocus.current) {
      pendingSubmenuFocus.current = false;
      handle.focusFirstItem();
    }
  }

  useLayoutEffect(() => {
    const node = overlayRef.current;
    const anchor = node?.parentElement;
    if (!nested || !node || !anchor) return;
    const geometry = observeOverlaySurfaceGeometry(node, createInstanceId("overlay-surface"), {
      onChange: (change) => geometryHandler.current?.(change),
      placement: null,
      reportInitial: false,
    });
    let previous = "";
    function reposition() {
      const row = anchor!.getBoundingClientRect();
      const viewport = { width: window.innerWidth, height: window.innerHeight };
      const parentSurface = anchor!.closest<HTMLElement>('[role="menu"]');
      const hidden = String(parentSurface?.dataset.anchorHidden === "true" ||
        isAnchorClipped(row, resolveClipRect(anchor!, viewport)));
      if (node!.dataset.anchorHidden !== hidden) node!.dataset.anchorHidden = hidden;
      const maxHeight = `${Math.max(0, viewport.height - 16)}px`;
      const maxWidth = `${Math.max(0, viewport.width - 16)}px`;
      if (node!.style.maxHeight !== maxHeight) node!.style.maxHeight = maxHeight;
      if (node!.style.maxWidth !== maxWidth) node!.style.maxWidth = maxWidth;
      const rect = node!.getBoundingClientRect();
      const padding = Number.parseFloat(getComputedStyle(node!).paddingTop) || 0;
      const flipped = row.right + rect.width > viewport.width - 8;
      const left = Math.max(8, Math.min(flipped ? row.left - rect.width : row.right, viewport.width - rect.width - 8));
      const top = Math.max(8, Math.min(row.top - padding, viewport.height - rect.height - 8));
      const next = `${left},${top},${flipped}`;
      if (next !== previous) {
        node!.style.left = `${Math.round(left)}px`;
        node!.style.top = `${Math.round(top)}px`;
        node!.classList.toggle("poodle-menu-surface--flipped", flipped);
        previous = next;
      }
      geometry.report();
    }
    reposition();
    const stop = observeAnchorMovement(anchor, node, reposition);
    let frame = 0;
    function track() {
      reposition();
      frame = requestAnimationFrame(track);
    }
    frame = requestAnimationFrame(track);
    return () => {
      stop();
      cancelAnimationFrame(frame);
      geometry.destroy();
    };
  }, [nested]);

  const actionableItems = menuNavigableItems(items);
  const firstTabStopIndex =
    actionableItems.length === 0 ? -1 : menuListNavigate(actionableItems, 0, "first");
  const effectiveHighlight =
    actionableItems.length === 0
      ? 0
      : highlightIndex >= actionableItems.length || actionableItems[highlightIndex]?.disabled
        ? menuListNavigate(actionableItems, 0, "first")
        : highlightIndex;

  function focusIndex(index: number): void {
    setHighlightIndex(index);
    const item = itemRefs.current[index];
    item?.focus({ preventScroll: true });
    item?.scrollIntoView?.({ block: "nearest", inline: "nearest" });
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

  function openSubmenu(item: MenuItem, focusFirst: boolean): void {
    if (item.disabled || !menuItemHasSubmenu(item)) return;
    pendingSubmenuFocus.current = focusFirst;
    if (openSubmenuValue === item.value && focusFirst) {
      pendingSubmenuFocus.current = false;
      submenuRef.current?.focusFirstItem();
    }
    setOpenSubmenuValue(item.value);
  }

  function closeSubmenu(refocusParent: boolean): void {
    const parentValue = openSubmenuValue;
    setOpenSubmenuValue(null);
    if (refocusParent) {
      const index = actionableItems.findIndex((item) => item.value === parentValue);
      if (index >= 0) focusIndex(index);
    }
  }

  function activateItem(item: MenuItem): void {
    if (!menuListCanActivate(item)) return;
    if (menuItemHasSubmenu(item)) {
      if (openSubmenuValue === item.value) closeSubmenu(true);
      else openSubmenu(item, false);
      return;
    }
    onAction?.(item.value);
  }

  const rows = (
    <>
      {items.map((item, itemIndex) =>
        item.kind === "separator" ? (
          <div key={`sep-${itemIndex}`} className="poodle-menu-surface__separator" role="separator" />
        ) : (
          <div key={item.value} className="poodle-menu-surface__item-anchor">
            <button
              ref={(node) => {
                const idx = actionableItems.findIndex((candidate) => candidate.value === item.value);
                if (idx >= 0) itemRefs.current[idx] = node;
              }}
              type="button"
              className="poodle-menu-surface__item"
              disabled={item.disabled === true}
              data-value={item.value}
              data-kind={menuItemHasSubmenu(item) ? "submenu" : (item.kind ?? "action")}
              data-tone={item.tone ?? "default"}
              role={item.kind === "checkbox" || item.kind === "radio" ? `menuitem${item.kind}` : "menuitem"}
              aria-label={item.label}
              tabIndex={actionableItems.findIndex((candidate) => candidate.value === item.value) === firstTabStopIndex ? 0 : -1}
              aria-checked={item.kind === "checkbox" || item.kind === "radio" ? item.checked === true : undefined}
              aria-haspopup={menuItemHasSubmenu(item) ? "menu" : undefined}
              aria-expanded={menuItemHasSubmenu(item) ? openSubmenuValue === item.value : undefined}
              onClick={() => activateItem(item)}
              onPointerEnter={() => {
                if (menuItemHasSubmenu(item) && !item.disabled) openSubmenu(item, false);
                else setOpenSubmenuValue(null);
              }}
              onKeyDown={(event) => {
                if (["ArrowDown", "ArrowUp", "Home", "End", "ArrowRight", "ArrowLeft", "Enter", " "].includes(event.key)) {
                  event.stopPropagation();
                }
                if (event.key === "ArrowRight" && menuItemHasSubmenu(item)) {
                  event.preventDefault();
                  openSubmenu(item, true);
                }
                if (event.key === "ArrowLeft" && nested) {
                  event.preventDefault();
                  onRequestClose?.();
                }
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
                  if (menuItemHasSubmenu(item)) openSubmenu(item, true);
                  else activateItem(item);
                }
              }}
            >
              <span className="poodle-menu-surface__label">{item.label}</span>
              {menuItemHasSubmenu(item) ? (
                <span className="poodle-menu-surface__submenu-indicator" aria-hidden="true">›</span>
              ) : item.checked ? (
                <span className="poodle-menu-surface__meta" aria-hidden="true">
                  ✓
                </span>
              ) : item.shortcutLabel ? (
                <span className="poodle-menu-surface__meta" aria-hidden="true">
                  {item.shortcutLabel}
                </span>
              ) : null}
            </button>
            {menuItemHasSubmenu(item) && openSubmenuValue === item.value ? (
              <MenuSurface
                ref={attachSubmenu}
                items={item.children}
                ariaLabel={item.label}
                size={size}
                density={density}
                nested
                onAction={onAction}
                onSurfaceGeometryChange={onSurfaceGeometryChange}
                onRequestClose={() => closeSubmenu(true)}
              />
            ) : null}
          </div>
        ),
      )}
    </>
  );

  const shell = {
    className: `poodle-menu-surface${nested ? " poodle-menu-surface--submenu" : ""}`,
    "data-size": size,
    "data-density": density,
    "data-placement": (anchor ? resolvedPlacement : placement) ?? undefined,
    role: "menu",
    "aria-label": ariaLabel ?? undefined,
  } as const;

  // Anchored surfaces portal to the theme root; an unanchored one stays put and
  // is positioned by the caller's `overlayStyle`.
  return anchor ? (
    <AnchoredSurface
      ref={overlayRef}
      anchor={anchor}
      placement={placement ?? "bottom-start"}
      offset={offset}
      onPlacement={setResolvedPlacement}
      onSurfaceGeometryChange={onSurfaceGeometryChange}
      {...shell}
    >
      {rows}
    </AnchoredSurface>
  ) : (
    <div ref={overlayRef} style={overlayStyle} {...shell}>
      {rows}
    </div>
  );
});
