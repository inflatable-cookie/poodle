import "@poodle/styles/selection-summary.css";

import { Icon } from "./Icon";
import { TextLink } from "./TextLink";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface SelectionSummaryItem {
  id: string;
  label: string;
}

export interface SelectionSummaryProps {
  items?: SelectionSummaryItem[];
  maxVisibleItems?: number;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  /** When set, each chip splits into a separate activation button (the label)
   * and a remove button — no nested buttons. When null, the whole chip removes
   * the item (backward-compatible default). */
  onActivate?: ((id: string) => void) | null;
  onRemove?: ((id: string) => void) | null;
  onClear?: (() => void) | null;
}

export function SelectionSummary({
  items = [],
  maxVisibleItems = 4,
  size = null,
  sizeRole = "control",
  density = null,
  onActivate = null,
  onRemove = null,
  onClear = null,
}: SelectionSummaryProps) {
  const uiPresentation = useUiPresentation();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const visibleItems = items.slice(0, maxVisibleItems);
  const overflowCount = Math.max(0, items.length - visibleItems.length);

  return (
    <section
      className="poodle-selection-summary"
      aria-label="Current selection"
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <div className="poodle-selection-summary__chips">
        {items.length === 0 ? (
          <span className="poodle-selection-summary__empty">No selection</span>
        ) : (
          <>
            {visibleItems.map((item) =>
              onActivate ? (
                <span
                  key={item.id}
                  className="poodle-selection-summary__chip poodle-selection-summary__chip--split"
                >
                  <button
                    type="button"
                    className="poodle-selection-summary__chip-activate"
                    onClick={() => onActivate?.(item.id)}
                    aria-label={`Edit ${item.label}`}
                  >
                    {item.label}
                  </button>
                  <button
                    type="button"
                    className="poodle-selection-summary__chip-remove"
                    onClick={() => onRemove?.(item.id)}
                    aria-label={`Remove ${item.label}`}
                  >
                    <Icon name="x" size="xs" />
                  </button>
                </span>
              ) : (
                <button
                  key={item.id}
                  type="button"
                  className="poodle-selection-summary__chip"
                  onClick={() => onRemove?.(item.id)}
                  aria-label={`Remove ${item.label}`}
                >
                  {item.label}
                  <span aria-hidden="true">
                    <Icon name="x" />
                  </span>
                </button>
              ),
            )}
            {overflowCount > 0 ? (
              <span className="poodle-selection-summary__overflow">+{overflowCount} more</span>
            ) : null}
            <TextLink className="poodle-selection-summary__clear" onClick={() => onClear?.()}>
              Clear
            </TextLink>
          </>
        )}
      </div>
    </section>
  );
}
