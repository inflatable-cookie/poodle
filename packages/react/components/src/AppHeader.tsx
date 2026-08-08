import "@inflatable-cookie/poodle-core/styles/app-header.css";

import type { ReactNode } from "react";

import { UiPresentationProvider, resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface AppHeaderProps {
  title?: string | null;
  subtitle?: string | null;
  dragRegion?: boolean;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  identity?: ReactNode;
  actions?: ReactNode;
  utility?: ReactNode;
}

export function AppHeader({
  title = null,
  subtitle = null,
  dragRegion = false,
  ariaLabel = null,
  size = null,
  sizeRole = "control",
  density = null,
  identity,
  actions,
  utility,
}: AppHeaderProps) {
  const uiPresentation = useUiPresentation();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  return (
    <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
      <header
        className="poodle-app-header"
        data-drag-region={dragRegion}
        data-size={resolvedSize}
        data-density={resolvedDensity}
        aria-label={ariaLabel ?? title ?? undefined}
      >
        <div className="poodle-app-header__identity">
          {identity ??
            (title ? (
              <div className="poodle-app-header__title-group">
                <strong>{title}</strong>
                {subtitle ? <span className="poodle-app-header__subtitle">{subtitle}</span> : null}
              </div>
            ) : null)}
        </div>

        {actions ? <div className="poodle-app-header__actions">{actions}</div> : null}

        {utility ? <div className="poodle-app-header__utility">{utility}</div> : null}
      </header>
    </UiPresentationProvider>
  );
}
