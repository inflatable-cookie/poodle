import "@poodle/styles/split-button.css";

import {
  useEffect,
  useRef,
  useState,
  type KeyboardEvent as ReactKeyboardEvent,
  type MouseEvent as ReactMouseEvent,
  type ReactNode,
} from "react";

import { menuNavigableItems, registerDismissLayer } from "@poodle/headless";

import { Spinner } from "./Spinner";
import { resolveSemanticControlSize, resolveSupportingVisualSize, useUiPresentation } from "./presentation";
import type {
  ButtonTone,
  ButtonVariant,
  ControlDensity,
  ControlSize,
  MenuItem,
  SemanticControlSizeRole,
} from "./types";

export interface SplitButtonProps {
  variant?: ButtonVariant;
  tone?: ButtonTone;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  type?: "button" | "submit" | "reset";
  items?: MenuItem[];
  disabled?: boolean;
  loading?: boolean;
  ariaLabel?: string | null;
  menuAriaLabel?: string;
  onClick?: ((event: ReactMouseEvent) => void) | undefined;
  onAction?: ((value: string) => void) | undefined;
  children?: ReactNode;
}

function getScrollContainer(element: HTMLElement | null): HTMLElement | null {
  let current = element?.parentElement ?? null;

  while (current) {
    const style = getComputedStyle(current);
    const overflowY = style.overflowY;
    if (
      (overflowY === "auto" || overflowY === "scroll" || overflowY === "overlay") &&
      current.scrollHeight > current.clientHeight
    ) {
      return current;
    }
    current = current.parentElement;
  }

  return null;
}

export function SplitButton({
  variant = "secondary",
  tone = "default",
  size = null,
  sizeRole = "control",
  density = null,
  type = "button",
  items = [],
  disabled = false,
  loading = false,
  ariaLabel = null,
  menuAriaLabel = "More actions",
  onClick = undefined,
  onAction = undefined,
  children,
}: SplitButtonProps) {
  const uiPresentation = useUiPresentation();

  const rootRef = useRef<HTMLDivElement | null>(null);
  const toggleRef = useRef<HTMLButtonElement | null>(null);
  const menuRef = useRef<HTMLDivElement | null>(null);
  const itemRefs = useRef<Array<HTMLButtonElement | null>>([]);
  const highlightRef = useRef(0);
  const pendingMenuFocus = useRef(false);

  const [menuOpen, setMenuOpen] = useState(false);
  const [menuPlacement, setMenuPlacement] = useState<"bottom-start" | "top-start">("bottom-start");
  const [menuMaxHeight, setMenuMaxHeight] = useState<string | null>(null);

  const isUnavailable = disabled || loading;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const resolvedVisualSize = resolveSupportingVisualSize(resolvedSize);
  const actionableItems = menuNavigableItems(items);

  function syncMenuLayout(): void {
    const root = rootRef.current;
    const menu = menuRef.current;
    if (!root || !menu) return;

    const rootRect = root.getBoundingClientRect();
    const menuRect = menu.getBoundingClientRect();
    const scrollContainer = getScrollContainer(root);
    const boundaryTop = scrollContainer?.getBoundingClientRect().top ?? 0;
    const boundaryBottom = scrollContainer?.getBoundingClientRect().bottom ?? window.innerHeight;
    const gutter = 6;
    const availableBelow = Math.max(0, boundaryBottom - rootRect.bottom - gutter);
    const availableAbove = Math.max(0, rootRect.top - boundaryTop - gutter);
    const shouldOpenUpward = availableBelow < menuRect.height && availableAbove > availableBelow;
    const availableSpace = shouldOpenUpward ? availableAbove : availableBelow;

    setMenuPlacement(shouldOpenUpward ? "top-start" : "bottom-start");
    setMenuMaxHeight(availableSpace > 0 ? `${Math.floor(availableSpace)}px` : null);
  }

  useEffect(() => {
    if (!menuOpen) {
      return;
    }

    syncMenuLayout();
    if (pendingMenuFocus.current) {
      pendingMenuFocus.current = false;
      itemRefs.current[highlightRef.current]?.focus();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [menuOpen]);

  function openMenu(): void {
    highlightRef.current = 0;
    pendingMenuFocus.current = true;
    setMenuOpen(true);
  }

  function toggleMenu(): void {
    if (isUnavailable) return;
    if (menuOpen) {
      closeMenu();
    } else {
      openMenu();
    }
  }

  function closeMenu(): void {
    setMenuOpen(false);
    highlightRef.current = 0;
    setMenuPlacement("bottom-start");
    setMenuMaxHeight(null);
  }

  function moveHighlight(direction: 1 | -1): void {
    const count = actionableItems.length;
    if (count === 0) return;

    let nextIndex = highlightRef.current;
    for (let step = 0; step < count; step += 1) {
      nextIndex = (nextIndex + direction + count) % count;
      if (!actionableItems[nextIndex]?.disabled) {
        highlightRef.current = nextIndex;
        itemRefs.current[nextIndex]?.focus();
        return;
      }
    }
  }

  function activateItem(item: MenuItem): void {
    if (item.disabled || item.kind === "separator") return;
    onAction?.(item.value);
    closeMenu();
  }

  useEffect(() => {
    if (!menuOpen) {
      return;
    }

    return registerDismissLayer({
      contains: (target) => rootRef.current?.contains(target) ?? false,
      dismissOnOutsideInteract: true,
      onDismiss: (reason) => {
        closeMenu();

        if (reason === "escape") {
          toggleRef.current?.focus();
        }
      },
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [menuOpen]);

  useEffect(() => {
    function handleBoundaryChange(): void {
      syncMenuLayout();
    }

    window.addEventListener("resize", handleBoundaryChange);
    document.addEventListener("scroll", handleBoundaryChange, true);

    return () => {
      window.removeEventListener("resize", handleBoundaryChange);
      document.removeEventListener("scroll", handleBoundaryChange, true);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  function handleToggleKeydown(event: ReactKeyboardEvent): void {
    if (event.key === "ArrowDown") {
      event.preventDefault();
      if (!menuOpen) {
        openMenu();
      } else {
        moveHighlight(1);
      }
    }
    if (event.key === "ArrowUp") {
      event.preventDefault();
      if (menuOpen) moveHighlight(-1);
    }
  }

  return (
    <div
      className="poodle-split-button"
      data-variant={variant}
      data-tone={tone !== "default" ? tone : undefined}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      ref={rootRef}
    >
      <button
        type={type}
        className="poodle-split-button__primary"
        disabled={isUnavailable}
        aria-label={ariaLabel ?? undefined}
        aria-busy={loading ? "true" : undefined}
        onClick={(event) => onClick?.(event)}
      >
        {loading ? (
          <span className="poodle-split-button__spinner" aria-hidden="true">
            <Spinner variant="ring" size={resolvedVisualSize} tone="current" />
          </span>
        ) : null}
        <span className="poodle-split-button__label">{children}</span>
      </button>

      <div className="poodle-split-button__divider" aria-hidden="true" />

      <button
        type="button"
        className="poodle-split-button__toggle"
        ref={toggleRef}
        disabled={isUnavailable}
        aria-haspopup="true"
        aria-expanded={menuOpen ? "true" : "false"}
        aria-label={menuAriaLabel}
        onClick={toggleMenu}
        onKeyDown={handleToggleKeydown}
      >
        <svg className="poodle-split-button__chevron" viewBox="0 0 16 16" fill="none" aria-hidden="true">
          <path d="M4 6l4 4 4-4" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
        </svg>
      </button>

      {menuOpen ? (
        <div
          ref={menuRef}
          className="poodle-split-button__menu"
          data-placement={menuPlacement}
          role="menu"
          aria-label={menuAriaLabel}
          style={menuMaxHeight ? { maxHeight: menuMaxHeight } : undefined}
        >
          {items.map((item) =>
            item.kind === "separator" ? (
              <div key={item.value} className="poodle-split-button__separator" role="separator" />
            ) : (
              <button
                key={item.value}
                ref={(el) => {
                  const index = actionableItems.findIndex((c) => c.value === item.value);
                  if (index >= 0) {
                    itemRefs.current[index] = el;
                  }
                }}
                type="button"
                className="poodle-split-button__item"
                disabled={item.disabled === true}
                role="menuitem"
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
                    highlightRef.current = 0;
                    itemRefs.current[0]?.focus();
                  }
                  if (event.key === "End") {
                    event.preventDefault();
                    highlightRef.current = actionableItems.length - 1;
                    itemRefs.current[actionableItems.length - 1]?.focus();
                  }
                  if (event.key === "Enter" || event.key === " ") {
                    event.preventDefault();
                    activateItem(item);
                  }
                }}
              >
                <span className="poodle-split-button__item-label">{item.label}</span>
              </button>
            ),
          )}
        </div>
      ) : null}
    </div>
  );
}
