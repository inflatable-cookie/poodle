import "@poodle/styles/detail-item.css";

import type { ReactNode } from "react";

import { Icon } from "./Icon";
import { Popover } from "./Popover";
import { useUiPresentation } from "./presentation";
import type { ControlDensity } from "./types";

export interface DetailItemProps {
  density?: ControlDensity | null;
  label: string;
  description?: string | null;
  value?: string | number | null;
  emptyText?: string;
  truncateValue?: boolean;
  ariaLabel?: string | null;
  layout?: "inline" | "stacked";
  presentation?: "simple" | "surface";
  span?: "full" | "half" | 1 | 2 | 3 | 4 | null;
  valueContent?: ReactNode;
  action?: ReactNode;
  children?: ReactNode;
}

export function DetailItem({
  density = null,
  label,
  description = null,
  value = null,
  emptyText = "—",
  truncateValue = false,
  ariaLabel = null,
  layout = "inline",
  presentation = "surface",
  span = null,
  valueContent,
  action,
  children,
}: DetailItemProps) {
  const uiPresentation = useUiPresentation();

  const resolvedDensity = density ?? uiPresentation.density;
  const resolvedLayout = layout === "stacked" ? "stacked" : "inline";
  const renderedValue = value === null ? emptyText : String(value);

  return (
    <div
      className="poodle-detail-item"
      data-density={resolvedDensity}
      data-layout={resolvedLayout}
      data-presentation={presentation}
      data-span={span ?? undefined}
      aria-label={ariaLabel ?? undefined}
    >
      <div className="poodle-detail-item__label-block">
        <span className="poodle-detail-item__label">
          {label}
          {description ? (
            <Popover
              placement="top"
              offset={6}
              ariaLabel="More information"
              trigger={
                <span className="poodle-detail-item__info-trigger">
                  <span className="poodle-detail-item__info-icon" aria-label="More information">
                    <Icon name="info" />
                  </span>
                </span>
              }
            >
              <p className="poodle-detail-item__info-content">{description}</p>
            </Popover>
          ) : null}
        </span>
      </div>

      <div className={truncateValue ? "poodle-detail-item__value poodle-truncate" : "poodle-detail-item__value"}>
        {valueContent ?? children ?? renderedValue}
      </div>

      {action ? <div className="poodle-detail-item__action">{action}</div> : null}
    </div>
  );
}
