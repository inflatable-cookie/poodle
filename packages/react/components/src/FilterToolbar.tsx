import "@poodle/styles/filter-toolbar.css";

import { useState, type CSSProperties, type MouseEvent as ReactMouseEvent, type ReactNode } from "react";

import { CollapseToggle } from "./CollapseToggle";
import { UiPresentationProvider, resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface FilterToolbarProps {
  ariaLabel?: string;
  summaryText?: string | null;
  collapsible?: boolean;
  collapsed?: boolean;
  defaultCollapsed?: boolean;
  columns?: number;
  minItemWidth?: string;
  sticky?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onCollapsedChange?: ((collapsed: boolean) => void) | undefined;
  children?: ReactNode;
  summary?: ReactNode;
  actions?: ReactNode;
  secondary?: ReactNode;
}

export function FilterToolbar({
  ariaLabel = "Filters",
  summaryText = null,
  collapsible = true,
  collapsed: controlledCollapsed,
  defaultCollapsed = false,
  columns = 4,
  minItemWidth = "10rem",
  sticky = false,
  size = null,
  sizeRole = "chrome",
  density = null,
  onCollapsedChange = undefined,
  children,
  summary,
  actions,
  secondary,
}: FilterToolbarProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledCollapsed, setUncontrolledCollapsed] = useState(defaultCollapsed);

  const isControlled = controlledCollapsed !== undefined;
  const collapsed = isControlled ? controlledCollapsed : uncontrolledCollapsed;

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  function setCollapsed(next: boolean): void {
    if (!isControlled) {
      setUncontrolledCollapsed(next);
    }
    onCollapsedChange?.(next);
  }

  function handleHeaderClick(event: ReactMouseEvent): void {
    if (!collapsible) return;

    const target = event.target as HTMLElement;
    if (target.closest(".poodle-filter-toolbar__actions") || target.closest(".poodle-collapse-toggle")) return;

    setCollapsed(!collapsed);
  }

  const summaryNode = summary ? (
    <span className="poodle-filter-toolbar__summary">{summary}</span>
  ) : summaryText ? (
    <span className="poodle-filter-toolbar__summary">{summaryText}</span>
  ) : null;

  const actionsNode = actions ? <span className="poodle-filter-toolbar__actions">{actions}</span> : null;

  return (
    <div
      className="poodle-filter-toolbar"
      data-sticky={sticky}
      data-collapsed={collapsible && collapsed}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      role="toolbar"
      aria-label={ariaLabel}
    >
      <UiPresentationProvider
        sizeScale={size ?? uiPresentation.sizeScale}
        density={density ?? uiPresentation.density}
      >
        {collapsible && collapsed ? (
          <button
            type="button"
            className="poodle-filter-toolbar__header poodle-filter-toolbar__header--button"
            onClick={handleHeaderClick}
            aria-expanded="false"
            aria-label={summaryText ? `Show filters. ${summaryText}` : "Show filters"}
          >
            <CollapseToggle
              collapsed={collapsed}
              ariaLabel="Show filters"
              onToggle={(isCollapsed) => setCollapsed(isCollapsed)}
            />
            {summaryNode}
            {actionsNode}
          </button>
        ) : collapsible ? (
          <button
            type="button"
            className="poodle-filter-toolbar__header poodle-filter-toolbar__header--button poodle-filter-toolbar__header--clickable"
            aria-expanded={!collapsed}
            onClick={handleHeaderClick}
          >
            <CollapseToggle
              collapsed={collapsed}
              ariaLabel={collapsed ? "Show filters" : "Hide filters"}
              onToggle={(isCollapsed) => setCollapsed(isCollapsed)}
            />
            {summaryNode}
            {actionsNode}
          </button>
        ) : (
          <div className="poodle-filter-toolbar__header">
            {summary ? (
              <div className="poodle-filter-toolbar__summary">{summary}</div>
            ) : summaryText ? (
              <p className="poodle-filter-toolbar__summary">{summaryText}</p>
            ) : null}
            {actions ? <div className="poodle-filter-toolbar__actions">{actions}</div> : null}
          </div>
        )}

        {!collapsible || !collapsed ? (
          <div
            className="poodle-filter-toolbar__controls"
            style={{ "--ft-columns": columns, "--ft-min-width": minItemWidth } as CSSProperties}
          >
            {children}
          </div>
        ) : null}

        {secondary ? <div className="poodle-filter-toolbar__secondary">{secondary}</div> : null}
      </UiPresentationProvider>
    </div>
  );
}
