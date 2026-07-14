import "@poodle/styles/detail-section.css";

import type { CSSProperties, ReactNode } from "react";

import { useUiPresentation } from "./presentation";
import type { ControlDensity } from "./types";

export interface DetailSectionProps {
  density?: ControlDensity | null;
  title?: string | null;
  description?: string | null;
  separated?: boolean;
  ariaLabel?: string | null;
  columns?: "auto" | 1 | 2 | 3 | 4;
  itemMinColumnWidth?: string | null;
  maxAutoColumns?: 2 | 3 | 4 | 5;
  actions?: ReactNode;
  children?: ReactNode;
}

export function DetailSection({
  density = null,
  title = null,
  description = null,
  separated = true,
  ariaLabel = null,
  columns = "auto",
  itemMinColumnWidth = null,
  maxAutoColumns = 4,
  actions,
  children,
}: DetailSectionProps) {
  const uiPresentation = useUiPresentation();

  const resolvedDensity = density ?? uiPresentation.density;
  const style: CSSProperties | undefined = itemMinColumnWidth
    ? ({ "--poodle-detail-section-item-min": itemMinColumnWidth } as CSSProperties)
    : undefined;

  return (
    <section
      className="poodle-detail-section"
      data-density={resolvedDensity}
      data-separated={separated}
      data-columns={columns}
      data-max-auto-columns={maxAutoColumns}
      aria-label={ariaLabel ?? undefined}
      style={style}
    >
      {title || description || actions ? (
        <div className="poodle-detail-section__header">
          <div className="poodle-detail-section__title-block">
            {title ? <h3 className="poodle-detail-section__title">{title}</h3> : null}
            {description ? <p className="poodle-detail-section__description">{description}</p> : null}
          </div>
          {actions ? <div className="poodle-detail-section__actions">{actions}</div> : null}
        </div>
      ) : null}
      <div className="poodle-detail-section__body">{children}</div>
    </section>
  );
}
