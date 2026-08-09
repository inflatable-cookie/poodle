import "@inflatable-cookie/poodle-core/styles/split-button.css";

import {
  useEffect,
  useRef,
  useState,
  type KeyboardEvent as ReactKeyboardEvent,
  type MouseEvent as ReactMouseEvent,
  type ReactNode,
} from "react";

import { menuNavigableItems, registerDismissLayer, layerContains } from "@inflatable-cookie/poodle-core";

import { AnchoredSurface } from "./AnchoredSurface";
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

  // The root is state, not a ref: the portalled menu has to re-render once
  // it exists so it can be positioned against it.
  const [rootElement, setRootElement] = useState<HTMLDivElement | null>(null);
  const toggleRef = useRef<HTMLButtonElement | null>(null);
  const menuRef = useRef<HTMLElement | null>(null);
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

  /** Cap the menu at the room available on whichever side it opened, so a long
   * list scrolls inside the surface instead of running off the viewport. */
  function syncMenuHeight(placement: "bottom-start" | "top-start"): void {
    const root = rootElement;
    if (!root) return;

    const rootRect = root.getBoundingClientRect();
    const gutter = 6;
    const available =
      placement === "top-start" ? rootRect.top - gutter : window.innerHeight - rootRect.bottom - gutter;

    setMenuMaxHeight(available > 0 ? `${Math.floor(available)}px` : null);
  }

  useEffect(() => {
    if (!menuOpen) {
      return;
    }

    if (pendingMenuFocus.current) {
      pendingMenuFocus.current = false;
      itemRefs.current[highlightRef.current]?.focus();
    }
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
      // The menu is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target, rootElement, menuRef.current),
      dismissOnOutsideInteract: true,
      onDismiss: (reason) => {
        closeMenu();

        if (reason === "escape") {
          toggleRef.current?.focus();
        }
      },
    });
  }, [menuOpen]);


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
      ref={setRootElement}
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
        <AnchoredSurface
          ref={menuRef}
          anchor={rootElement}
          placement="bottom-start"
          offset={6}
          onPlacement={(next) => {
            const side = next.startsWith("top") ? "top-start" : "bottom-start";
            setMenuPlacement(side);
            syncMenuHeight(side);
          }}
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
        </AnchoredSurface>
      ) : null}
    </div>
  );
}
