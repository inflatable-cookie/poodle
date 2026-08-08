import "@inflatable-cookie/poodle-core/styles/bulk-action-bar.css";

import { isValidElement, type ReactNode } from "react";

import { IconButton } from "./IconButton";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { BulkAction, ControlDensity, ControlSize, IconProp, SemanticControlSizeRole } from "./types";

export interface BulkActionBarProps {
  selectionCount?: number;
  totalCount?: number | null;
  actions?: BulkAction[];
  loading?: boolean;
  disabled?: boolean;
  showSelectAll?: boolean;
  allSelected?: boolean;
  selectAllLabel?: string;
  onAction?: ((id: string) => void) | null;
  onClear?: (() => void) | null;
  onSelectAll?: (() => void) | null;
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
}

/** Custom icon content (a rendered element) vs a named/nodes icon prop. */
function isIconElement(icon: BulkAction["icon"]): icon is ReactNode {
  return isValidElement(icon);
}

function isNamedIcon(icon: BulkAction["icon"]): icon is IconProp {
  return icon !== undefined && icon !== null && !isIconElement(icon);
}

export function BulkActionBar({
  selectionCount = 0,
  totalCount = null,
  actions = [],
  loading = false,
  disabled = false,
  showSelectAll = false,
  allSelected = false,
  selectAllLabel = "Select all",
  onAction = null,
  onClear = null,
  onSelectAll = null,
  sizeRole = "control",
  size = null,
  density = null,
}: BulkActionBarProps) {
  const uiPresentation = useUiPresentation();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isUnavailable = disabled || loading;
  const actionsDisabled = isUnavailable || selectionCount === 0;

  return (
    <div
      className="poodle-bulk-action-bar"
      role="region"
      aria-label="Bulk actions"
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <div className="poodle-bulk-action-bar__summary">
        <strong>{selectionCount} selected</strong>
        {totalCount !== null ? <span>of {totalCount}</span> : null}
        {showSelectAll && !allSelected ? (
          <IconButton
            icon="check-check"
            ariaLabel={totalCount !== null ? `${selectAllLabel} (${totalCount})` : selectAllLabel}
            tooltip={totalCount !== null ? `${selectAllLabel} (${totalCount})` : selectAllLabel}
            variant="ghost"
            sizeRole="chrome"
            disabled={isUnavailable}
            onClick={() => onSelectAll?.()}
          />
        ) : null}
      </div>

      <div className="poodle-bulk-action-bar__actions">
        {actions.map((action) => {
          const actionTone = action.tone ?? "default";
          const fallbackIcon = actionTone === "danger" ? "trash-2" : "circle";

          return (
            <span
              key={action.id}
              className="poodle-bulk-action-bar__icon-action"
              data-tone={actionTone !== "default" ? actionTone : undefined}
            >
              {action.icon && isIconElement(action.icon) ? (
                <IconButton
                  icon={fallbackIcon}
                  ariaLabel={action.label}
                  tooltip={action.label}
                  variant="ghost"
                  tone={actionTone === "danger" ? "danger" : "default"}
                  size={resolvedSize}
                  disabled={actionsDisabled || action.disabled}
                  onClick={() => onAction?.(action.id)}
                >
                  {action.icon}
                </IconButton>
              ) : (
                <IconButton
                  icon={isNamedIcon(action.icon) ? action.icon : fallbackIcon}
                  ariaLabel={action.label}
                  tooltip={action.label}
                  variant="ghost"
                  tone={actionTone === "danger" ? "danger" : "default"}
                  size={resolvedSize}
                  disabled={actionsDisabled || action.disabled}
                  onClick={() => onAction?.(action.id)}
                />
              )}
            </span>
          );
        })}
        <IconButton
          icon="x"
          ariaLabel="Clear selection"
          variant="ghost"
          size={resolvedSize}
          disabled={isUnavailable}
          onClick={() => onClear?.()}
        />
      </div>
    </div>
  );
}
