import "@poodle/styles/list-card.css";

import {
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
  type CSSProperties,
  type KeyboardEvent as ReactKeyboardEvent,
  type MouseEvent as ReactMouseEvent,
  type ReactNode,
} from "react";

import { menuNavigableItems, registerDismissLayer, pointAnchor } from "@poodle/headless";

import { AnchoredSurface } from "./AnchoredSurface";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, MenuItem, SemanticControlSizeRole } from "./types";

export interface ListCardProps {
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  title: string;
  subtitle?: string | null;
  meta?: string | null;
  href?: string | null;
  leadingShape?: "circle" | "rounded-square";
  leadingFill?: "tint" | "solid";
  leadingSizeOffset?: number;
  accentColor?: string | null;
  layout?: "default" | "compact" | "stacked";
  interactive?: boolean;
  disabled?: boolean;
  selectable?: boolean;
  selected?: boolean;
  /**
   * The card you are currently on — always on for one card in a list.
   *
   * Distinct from `selected`, which marks a card picked out for an action and
   * is styled loudly to match. See list-card.md §4.
   */
  active?: boolean;
  highlighted?: boolean;
  selectionIndicator?: "none" | "checkbox";
  showReorderHandle?: boolean;
  notLive?: boolean;
  sash?: string | null;
  sashColor?: string | null;
  ariaLabel?: string | null;
  contextMenuItems?: MenuItem[] | null;
  contextMenuAriaLabel?: string | null;
  contextMenuTrigger?: "context" | "leading";
  onClick?: ((event: ReactMouseEvent | MouseEvent) => void) | null;
  onSelectedChange?: ((selected: boolean) => void) | null;
  onContextAction?: ((value: string) => void) | null;
  titleContent?: ReactNode;
  subtitleContent?: ReactNode;
  metaContent?: ReactNode;
  sashContent?: ReactNode;
  leading?: ReactNode;
  badges?: ReactNode;
  corner?: ReactNode;
  footer?: ReactNode;
  actions?: ReactNode;
  trailing?: ReactNode;
}

const controlSizes: ControlSize[] = ["xs", "sm", "md", "lg", "xl"];

function offsetControlSize(size: ControlSize, offset: number): ControlSize {
  const baseIndex = controlSizes.indexOf(size);
  const nextIndex = Math.max(0, Math.min(controlSizes.length - 1, baseIndex + Math.round(offset)));
  return controlSizes[nextIndex] ?? size;
}

function SelectionBox({ selected }: { selected: boolean }) {
  return (
    <span className="poodle-list-card__selection-box">
      {selected ? (
        <svg
          viewBox="0 0 16 16"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <path d="M3.5 8.25l2.75 2.75L12.5 4.75" />
        </svg>
      ) : null}
    </span>
  );
}

export function ListCard({
  size = null,
  sizeRole = "control",
  density = null,
  title,
  subtitle = null,
  meta = null,
  href = null,
  leadingShape = "circle",
  leadingFill = "tint",
  leadingSizeOffset = 0,
  accentColor = null,
  layout = "default",
  interactive = false,
  disabled = false,
  selectable = false,
  selected = false,
  active = false,
  highlighted = false,
  selectionIndicator = "none",
  showReorderHandle = false,
  notLive = false,
  sash = null,
  sashColor = null,
  ariaLabel = null,
  contextMenuItems = null,
  contextMenuAriaLabel = null,
  contextMenuTrigger = "context",
  onClick = null,
  onSelectedChange = null,
  onContextAction = null,
  titleContent,
  subtitleContent,
  metaContent,
  sashContent,
  leading,
  badges,
  corner,
  footer,
  actions,
  trailing,
}: ListCardProps) {
  const uiPresentation = useUiPresentation();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const resolvedLeadingSize = offsetControlSize(resolvedSize, leadingSizeOffset);
  const isCompact = layout === "compact";
  const isStacked = layout === "stacked";
  const showMeta = !trailing && Boolean(meta || metaContent) && !isCompact;
  const showActions = !trailing && Boolean(actions);
  const showUtilityRail = isStacked && (Boolean(trailing) || showMeta || showActions);
  const isInteractive = Boolean(href) || interactive || selectable;
  const showSelectionIndicator = selectable && selectionIndicator === "checkbox";
  const showSelectionOverlay = showSelectionIndicator && Boolean(leading);
  const actionableContextMenuItems = menuNavigableItems(contextMenuItems ?? []);
  const hasContextMenu = (contextMenuItems?.length ?? 0) > 0;
  const useLeadingContextMenu = contextMenuTrigger === "leading" && hasContextMenu && !selectable;

  const rootRef = useRef<HTMLElement | null>(null);
  const leadingRef = useRef<HTMLElement | null>(null);
  const overlayRef = useRef<HTMLElement | null>(null);
  const itemRefs = useRef<Array<HTMLButtonElement | null>>([]);
  const highlightRef = useRef(0);
  const pendingMenuFocus = useRef(false);

  const [contextMenuOpen, setContextMenuOpen] = useState(false);
  const [contextMenuAnchorPoint, setContextMenuAnchorPoint] = useState<{ x: number; y: number } | null>(null);

  function openContextMenuAt(x: number, y: number) {
    if (!hasContextMenu) return;
    highlightRef.current = 0;
    setContextMenuAnchorPoint({ x, y });
    pendingMenuFocus.current = true;
    setContextMenuOpen(true);
  }

  function closeContextMenu() {
    setContextMenuOpen(false);
    highlightRef.current = 0;
  }

  function moveContextMenuHighlight(direction: 1 | -1): void {
    const count = actionableContextMenuItems.length;
    if (count === 0) return;

    let nextIndex = highlightRef.current;
    for (let step = 0; step < count; step += 1) {
      nextIndex = (nextIndex + direction + count) % count;
      if (!actionableContextMenuItems[nextIndex]?.disabled) {
        highlightRef.current = nextIndex;
        itemRefs.current[nextIndex]?.focus();
        return;
      }
    }
  }

  function activateContextMenuItem(item: MenuItem): void {
    if (item.disabled || item.kind === "separator") return;
    onContextAction?.(item.value);
    closeContextMenu();
  }

  function handleClick(event: ReactMouseEvent) {
    if (disabled) return;

    if (selectable) {
      event.preventDefault();
      onSelectedChange?.(!selected);
      return;
    }

    if (interactive || href) {
      onClick?.(event);
    }
  }

  function handleKeydown(event: ReactKeyboardEvent) {
    if (disabled || href) return;

    if ((interactive || selectable) && (event.key === "Enter" || event.key === " ")) {
      event.preventDefault();
      if (selectable) {
        onSelectedChange?.(!selected);
      } else {
        onClick?.(new MouseEvent("click"));
      }
    }
  }

  function handleContextMenu(event: ReactMouseEvent) {
    if (disabled || !hasContextMenu) return;
    if (useLeadingContextMenu) return;
    event.preventDefault();
    event.stopPropagation();
    openContextMenuAt(event.clientX, event.clientY);
  }

  function toggleContextMenuFromLeading(event: ReactMouseEvent | ReactKeyboardEvent) {
    if (disabled || !useLeadingContextMenu || !leadingRef.current) return;
    event.preventDefault();
    event.stopPropagation();

    if (contextMenuOpen) {
      closeContextMenu();
      return;
    }

    const rect = leadingRef.current.getBoundingClientRect();
    openContextMenuAt(rect.left + rect.width / 2, rect.bottom + 4);
  }

  function handleContextMenuKeydown(event: ReactKeyboardEvent) {
    if (disabled || !hasContextMenu) return;
    if (event.key === "ContextMenu" || (event.shiftKey && event.key === "F10")) {
      event.preventDefault();
      event.stopPropagation();
      if (useLeadingContextMenu) {
        toggleContextMenuFromLeading(event);
        return;
      }
      const rect = rootRef.current?.getBoundingClientRect();
      if (!rect) return;
      openContextMenuAt(rect.left + 16, rect.top + 16);
    }
  }

  function handleRootKeydown(event: ReactKeyboardEvent) {
    handleKeydown(event);
    handleContextMenuKeydown(event);
  }

  useEffect(() => {
    if (!contextMenuOpen) return;
    if (!pendingMenuFocus.current) return;
    pendingMenuFocus.current = false;
    itemRefs.current[highlightRef.current]?.focus();
  }, [contextMenuOpen]);

  useEffect(() => {
    if (!contextMenuOpen) return;

    return registerDismissLayer({
      // The overlay and the leading trigger area count as inside.
      contains: (target) =>
        (overlayRef.current?.contains(target) ?? false) || (leadingRef.current?.contains(target) ?? false),
      dismissOnOutsideInteract: true,
      onDismiss: () => closeContextMenu(),
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [contextMenuOpen]);

  const rootStyle: CSSProperties | undefined =
    accentColor || sashColor
      ? ({
          ...(accentColor ? { "--list-card-accent": accentColor } : {}),
          ...(sashColor ? { "--list-card-sash": sashColor } : {}),
        } as CSSProperties)
      : undefined;

  const rootClassName = [
    "poodle-list-card",
    isInteractive ? "poodle-list-card--interactive" : "",
    sash ? "poodle-list-card--has-sash" : "",
  ]
    .filter(Boolean)
    .join(" ");

  const selectionOverlay = showSelectionOverlay ? (
    <span
      className="poodle-list-card__selection-indicator poodle-list-card__selection-indicator--overlay"
      aria-hidden="true"
    >
      <SelectionBox selected={selected} />
    </span>
  ) : null;

  const leadingContent = leading ? (
    useLeadingContextMenu && !disabled ? (
      <button
        ref={(el) => {
          leadingRef.current = el;
        }}
        type="button"
        className="poodle-list-card__leading poodle-list-card__leading-button"
        data-interactive={true}
        data-selection-overlay={showSelectionOverlay}
        aria-label={contextMenuAriaLabel ?? `${title} actions`}
        onClick={toggleContextMenuFromLeading}
        onKeyDown={(event) => {
          if (event.key === "Enter" || event.key === " ") {
            toggleContextMenuFromLeading(event);
          }
        }}
      >
        <span className="poodle-list-card__leading-content" aria-hidden={showSelectionOverlay ? "true" : undefined}>
          {leading}
        </span>
        {selectionOverlay}
      </button>
    ) : (
      <span
        ref={(el) => {
          leadingRef.current = el;
        }}
        className="poodle-list-card__leading"
        data-interactive={useLeadingContextMenu}
        data-selection-overlay={showSelectionOverlay}
      >
        <span className="poodle-list-card__leading-content" aria-hidden={showSelectionOverlay ? "true" : undefined}>
          {leading}
        </span>
        {selectionOverlay}
      </span>
    )
  ) : showSelectionIndicator ? (
    <span className="poodle-list-card__selection-indicator" aria-hidden="true">
      <SelectionBox selected={selected} />
    </span>
  ) : null;

  const cardInner = (
    <>
      {sashContent || sash ? (
        <span className="poodle-list-card__sash" aria-label={sash ?? undefined}>
          {sashContent ?? sash}
        </span>
      ) : null}

      {showReorderHandle ? (
        <span className="poodle-list-card__handle" aria-hidden="true">
          <svg viewBox="0 0 16 16" fill="currentColor">
            <circle cx="5" cy="4" r="1.1" />
            <circle cx="5" cy="8" r="1.1" />
            <circle cx="5" cy="12" r="1.1" />
            <circle cx="11" cy="4" r="1.1" />
            <circle cx="11" cy="8" r="1.1" />
            <circle cx="11" cy="12" r="1.1" />
          </svg>
        </span>
      ) : null}

      {leadingContent}

      <div className="poodle-list-card__body">
        <div className="poodle-list-card__header">
          <span className="poodle-list-card__title">{titleContent ?? title}</span>
          {badges || corner ? (
            <span className="poodle-list-card__header-accessories">
              {badges ? <span className="poodle-list-card__badges">{badges}</span> : null}
              {corner ? <span className="poodle-list-card__corner">{corner}</span> : null}
            </span>
          ) : null}
        </div>
        {subtitleContent ? (
          <span className="poodle-list-card__subtitle">{subtitleContent}</span>
        ) : subtitle ? (
          <span className="poodle-list-card__subtitle">{subtitle}</span>
        ) : null}
        {footer ? <div className="poodle-list-card__footer">{footer}</div> : null}
      </div>

      {showUtilityRail ? (
        <div className="poodle-list-card__utility-rail">
          {showMeta ? <span className="poodle-list-card__meta">{metaContent ?? meta}</span> : null}
          {showActions ? <span className="poodle-list-card__actions">{actions}</span> : null}
          {trailing ? <span className="poodle-list-card__trailing">{trailing}</span> : null}
        </div>
      ) : (
        <>
          {showMeta ? <span className="poodle-list-card__meta">{metaContent ?? meta}</span> : null}
          {showActions ? <span className="poodle-list-card__actions">{actions}</span> : null}
          {trailing ? <span className="poodle-list-card__trailing">{trailing}</span> : null}
        </>
      )}
    </>
  );

  const sharedDataProps = {
    "data-density": resolvedDensity,
    "data-disabled": disabled,
    "data-not-live": notLive,
    "data-leading-shape": leadingShape,
    "data-leading-fill": leadingFill,
    "data-leading-size": resolvedLeadingSize,
    "data-layout": layout,
    "data-selected": selected,
    "data-active": active,
    "data-highlighted": highlighted,
    "data-reorder": showReorderHandle,
  };

  const contextMenuOverlay =
    contextMenuOpen && hasContextMenu && contextMenuAnchorPoint ? (
      <AnchoredSurface
        ref={overlayRef}
        // A right-click has no element behind it, so the menu anchors to the
        // point itself and the shared resolver handles the edge flipping.
        anchor={pointAnchor(contextMenuAnchorPoint.x, contextMenuAnchorPoint.y, rootRef.current)}
        placement="bottom-start"
        offset={0}
        className="poodle-list-card__context-menu"
        data-size={resolvedSize}
        data-density={resolvedDensity}
        role="menu"
        aria-label={contextMenuAriaLabel ?? undefined}
      >
        {(contextMenuItems ?? []).map((item) =>
          item.kind === "separator" ? (
            <div key={item.value} className="poodle-list-card__context-separator" role="separator" />
          ) : (
            <button
              key={item.value}
              ref={(el) => {
                const index = actionableContextMenuItems.findIndex((candidate) => candidate.value === item.value);
                if (index >= 0) {
                  itemRefs.current[index] = el;
                }
              }}
              type="button"
              className="poodle-list-card__context-item"
              disabled={item.disabled === true}
              data-tone={item.tone ?? "default"}
              role={item.kind === "checkbox" || item.kind === "radio" ? `menuitem${item.kind}` : "menuitem"}
              aria-checked={
                item.kind === "checkbox" || item.kind === "radio" ? (item.checked ? "true" : "false") : undefined
              }
              onClick={() => activateContextMenuItem(item)}
              onKeyDown={(event) => {
                if (event.key === "ArrowDown") {
                  event.preventDefault();
                  moveContextMenuHighlight(1);
                }

                if (event.key === "ArrowUp") {
                  event.preventDefault();
                  moveContextMenuHighlight(-1);
                }

                if (event.key === "Home") {
                  event.preventDefault();
                  highlightRef.current = 0;
                  itemRefs.current[0]?.focus();
                }

                if (event.key === "End") {
                  event.preventDefault();
                  highlightRef.current = actionableContextMenuItems.length - 1;
                  itemRefs.current[actionableContextMenuItems.length - 1]?.focus();
                }

                if (event.key === "Enter" || event.key === " ") {
                  event.preventDefault();
                  activateContextMenuItem(item);
                }
              }}
            >
              <span>{item.label}</span>

              {item.checked ? (
                <span className="poodle-list-card__context-meta" aria-hidden="true">
                  ✓
                </span>
              ) : item.shortcutLabel ? (
                <span className="poodle-list-card__context-meta" aria-hidden="true">
                  {item.shortcutLabel}
                </span>
              ) : null}
            </button>
          ),
        )}
      </AnchoredSurface>
    ) : null;

  if (href && !disabled && !selectable) {
    return (
      <>
        <a
          ref={(el) => {
            rootRef.current = el;
          }}
          className={rootClassName}
          href={href}
          data-size={resolvedSize}
          {...sharedDataProps}
          aria-label={ariaLabel ?? title}
          style={rootStyle}
          onClick={handleClick}
          onContextMenu={handleContextMenu}
          onKeyDown={handleContextMenuKeydown}
        >
          {cardInner}
        </a>
        {contextMenuOverlay}
      </>
    );
  }

  return (
    <>
      <div
        ref={(el) => {
          rootRef.current = el;
        }}
        className={rootClassName}
        data-size={resolvedSize}
        {...sharedDataProps}
        role={isInteractive ? "button" : undefined}
        aria-pressed={selectable ? selected : undefined}
        aria-current={active ? "true" : undefined}
        tabIndex={isInteractive && !disabled ? 0 : -1}
        aria-label={ariaLabel ?? title}
        style={rootStyle}
        onClick={handleClick}
        onKeyDown={handleRootKeydown}
        onContextMenu={handleContextMenu}
      >
        {cardInner}
      </div>
      {contextMenuOverlay}
    </>
  );
}
