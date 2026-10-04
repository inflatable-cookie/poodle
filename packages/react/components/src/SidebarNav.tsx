import "@inflatable-cookie/poodle-core/styles/sidebar-nav.css";

import { useEffect, useId, useRef, useState, type KeyboardEvent as ReactKeyboardEvent, type MouseEvent as ReactMouseEvent, type ReactNode } from "react";

import { ContextMenu } from "./ContextMenu";
import type { ControlDensity, ControlSize, SemanticControlSizeRole, SidebarNavGroup, SidebarNavItem } from "./types";

export interface SidebarNavProps {
  groups?: SidebarNavGroup[];
  value?: string | null;
  defaultValue?: string | null;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onValueChange?: ((value: string) => void) | undefined;
  onContextAction?: ((itemValue: string, actionValue: string) => void) | undefined;
}

export function SidebarNav({
  groups = [],
  value: controlledValue,
  defaultValue = null,
  ariaLabel = null,
  size = null,
  sizeRole = "chrome",
  density = null,
  onValueChange = undefined,
  onContextAction = undefined,
}: SidebarNavProps) {
  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);
  const isControlled = controlledValue !== undefined && controlledValue !== null;
  const value = isControlled ? controlledValue : uncontrolledValue;

  const sidebarNavId = useId();
  const navRef = useRef<HTMLElement | null>(null);
  const visibleGroups = groups.filter((group) => group.items.length > 0);

  const [contextMenuOpen, setContextMenuOpen] = useState(false);
  const [contextMenuAnchor, setContextMenuAnchor] = useState<{ x: number; y: number } | null>(null);
  const [contextMenuItemValue, setContextMenuItemValue] = useState<string | null>(null);
  const contextMenuFocusCandidates = useRef<HTMLElement[]>([]);
  const contextMenuWasOpen = useRef(false);

  useEffect(() => {
    if (contextMenuOpen) {
      contextMenuWasOpen.current = true;
      return;
    }

    if (!contextMenuWasOpen.current) return;
    contextMenuWasOpen.current = false;
    const candidates = contextMenuFocusCandidates.current;
    contextMenuFocusCandidates.current = [];
    setTimeout(() => {
      for (const candidate of candidates) {
        if (!candidate.isConnected) continue;
        candidate.focus();
        if (candidate.ownerDocument.activeElement === candidate) return;
      }
    }, 0);
  }, [contextMenuOpen]);

  const contextMenuHost =
    visibleGroups.flatMap((group) => group.items).find((item) => item.value === contextMenuItemValue) ?? null;
  const contextMenuItems = contextMenuHost?.contextMenuItems ?? [];
  const contextMenuAriaLabel =
    contextMenuHost?.contextMenuAriaLabel ?? (contextMenuHost ? `${contextMenuHost.label} actions` : null);

  function handleItemActivation(item: SidebarNavItem): void {
    if (item.disabled) return;
    if (!isControlled) {
      setUncontrolledValue(item.value);
    }
    onValueChange?.(item.value);
  }

  function itemHasContextMenu(item: SidebarNavItem): boolean {
    return !item.disabled && (item.contextMenuItems?.length ?? 0) > 0;
  }

  function openContextMenu(item: SidebarNavItem, x: number, y: number, invoker: HTMLElement): void {
    if (!itemHasContextMenu(item)) return;
    const navItems = Array.from(navRef.current?.querySelectorAll<HTMLElement>(".poodle-sidebar-nav__item") ?? []);
    const invokerIndex = navItems.indexOf(invoker);
    const peers = navItems
      .filter((candidate) => candidate !== invoker && !candidate.hasAttribute("disabled"))
      .sort((left, right) => {
        const leftDistance = Math.abs(navItems.indexOf(left) - invokerIndex);
        const rightDistance = Math.abs(navItems.indexOf(right) - invokerIndex);
        return leftDistance - rightDistance || navItems.indexOf(left) - navItems.indexOf(right);
      });
    contextMenuFocusCandidates.current = [invoker, ...peers, ...(navRef.current ? [navRef.current] : [])];
    setContextMenuItemValue(item.value);
    setContextMenuAnchor({ x, y });
    setContextMenuOpen(true);
  }

  function handleItemContextMenu(item: SidebarNavItem, event: ReactMouseEvent): void {
    if (!itemHasContextMenu(item)) return;
    const invoker = event.currentTarget;
    if (!(invoker instanceof HTMLElement)) return;
    event.preventDefault();
    event.stopPropagation();
    openContextMenu(item, event.clientX, event.clientY, invoker);
  }

  function handleItemKeydown(item: SidebarNavItem, event: ReactKeyboardEvent): void {
    if (!itemHasContextMenu(item)) return;
    if (event.key !== "ContextMenu" && !(event.shiftKey && event.key === "F10")) {
      return;
    }

    event.preventDefault();
    event.stopPropagation();
    const target = event.currentTarget;
    if (!(target instanceof HTMLElement)) return;
    const rect = target.getBoundingClientRect();
    openContextMenu(item, rect.left + 16, rect.top + 16, target);
  }

  function handleContextAction(actionValue: string): void {
    if (contextMenuItemValue == null) return;
    onContextAction?.(contextMenuItemValue, actionValue);
  }

  function itemClassName(item: SidebarNavItem): string {
    return [
      "poodle-sidebar-nav__item",
      item.value === value ? "poodle-sidebar-nav__item--active" : "",
    ]
      .filter(Boolean)
      .join(" ");
  }

  function itemContent(item: SidebarNavItem, endLabelId: string): ReactNode {
    if (!item.endLabel) return item.label;
    return (
      <>
        <span className="poodle-sidebar-nav__label">{item.label}</span>
        <span className="poodle-sidebar-nav__end-label" id={endLabelId} aria-hidden="true">
          {item.endLabel}
        </span>
      </>
    );
  }

  return (
    <nav
      ref={navRef}
      className="poodle-sidebar-nav"
      tabIndex={-1}
      data-size={size ?? undefined}
      data-density={density ?? undefined}
      data-size-role={sizeRole}
      aria-label={ariaLabel ?? undefined}
    >
      {visibleGroups.map((group, groupIndex) => (
        <section
          key={group.id}
          className="poodle-sidebar-nav__group"
          data-separated={visibleGroups.length > 1}
          aria-label={group.label ?? undefined}
        >
          {group.label ? <h2 className="poodle-sidebar-nav__group-title" title={group.label}>{group.label}</h2> : null}

          <ul className="poodle-sidebar-nav__list">
            {group.items.map((item, itemIndex) => {
              const endLabelId = `${sidebarNavId}-${groupIndex}-${itemIndex}-end-label`;
              const hasMenu = itemHasContextMenu(item);
              return (
                <li key={item.value}>
                  {item.href && !item.disabled ? (
                    <a
                      className={itemClassName(item)}
                      href={item.href}
                      aria-describedby={item.endLabel ? endLabelId : undefined}
                      data-end-label={item.endLabel ? "true" : undefined}
                      aria-current={item.value === value ? "page" : undefined}
                      onClick={() => handleItemActivation(item)}
                      onContextMenu={hasMenu ? (event) => handleItemContextMenu(item, event) : undefined}
                      onKeyDown={hasMenu ? (event) => handleItemKeydown(item, event) : undefined}
                    >
                      {itemContent(item, endLabelId)}
                    </a>
                  ) : (
                    <button
                      type="button"
                      className={itemClassName(item)}
                      aria-current={item.value === value ? "page" : undefined}
                      disabled={item.disabled}
                      aria-describedby={item.endLabel ? endLabelId : undefined}
                      data-end-label={item.endLabel ? "true" : undefined}
                      onClick={() => handleItemActivation(item)}
                      onContextMenu={hasMenu ? (event) => handleItemContextMenu(item, event) : undefined}
                      onKeyDown={hasMenu ? (event) => handleItemKeydown(item, event) : undefined}
                    >
                      {itemContent(item, endLabelId)}
                    </button>
                  )}
                </li>
              );
            })}
          </ul>
        </section>
      ))}

      <ContextMenu
        trigger={false}
        items={contextMenuItems}
        open={contextMenuOpen}
        anchorPoint={contextMenuAnchor}
        ariaLabel={contextMenuAriaLabel}
        size={size}
        sizeRole={sizeRole}
        density={density}
        onOpenChange={setContextMenuOpen}
        onAction={handleContextAction}
      />
    </nav>
  );
}
