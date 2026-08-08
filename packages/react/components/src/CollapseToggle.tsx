import "@inflatable-cookie/poodle-core/styles/collapse-toggle.css";

import { Icon } from "./Icon";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { CollapseDirection, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface CollapseToggleProps {
  collapsed?: boolean;
  direction?: CollapseDirection;
  disabled?: boolean;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onToggle?: ((isCollapsed: boolean) => void) | null;
}

const expandDirections: Record<CollapseDirection, CollapseDirection> = {
  left: "right",
  right: "left",
  up: "down",
  down: "up",
};

export function CollapseToggle({
  collapsed = false,
  direction = "left",
  disabled = false,
  ariaLabel = null,
  size = null,
  sizeRole = "chrome",
  density = null,
  onToggle = null,
}: CollapseToggleProps) {
  const uiPresentation = useUiPresentation();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const iconName = `chevron-${collapsed ? expandDirections[direction] : direction}`;
  const label = ariaLabel ?? (collapsed ? "Expand" : "Collapse");

  function handleClick(): void {
    if (disabled) return;
    onToggle?.(!collapsed);
  }

  return (
    <button
      type="button"
      className="poodle-collapse-toggle"
      data-collapsed={collapsed || undefined}
      data-direction={direction}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      disabled={disabled}
      aria-expanded={!collapsed}
      aria-label={label}
      onClick={handleClick}
    >
      <Icon name={iconName} size={resolvedSize} />
    </button>
  );
}
