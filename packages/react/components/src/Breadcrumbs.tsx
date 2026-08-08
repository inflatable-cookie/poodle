import "@inflatable-cookie/poodle-styles/breadcrumbs.css";

import { Icon } from "./Icon";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { BreadcrumbItem, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface BreadcrumbsProps {
  items?: BreadcrumbItem[];
  ariaLabel?: string;
  maxVisibleItems?: number | null;
  forceLastItemCurrent?: boolean;
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  onNavigate?: ((value: string) => void) | undefined;
}

export function Breadcrumbs({
  items = [],
  ariaLabel = "Breadcrumb",
  maxVisibleItems = null,
  forceLastItemCurrent = true,
  sizeRole = "chrome",
  size = null,
  density = null,
  onNavigate = undefined,
}: BreadcrumbsProps) {
  const uiPresentation = useUiPresentation();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const visibleItems: BreadcrumbItem[] =
    maxVisibleItems !== null && items.length > maxVisibleItems
      ? [
          items[0],
          { value: "__ellipsis__", label: "…", current: false },
          ...items.slice(items.length - (maxVisibleItems - 1)),
        ]
      : items;

  function handleNavigate(item: BreadcrumbItem): void {
    if (item.current || item.value === "__ellipsis__") {
      return;
    }

    onNavigate?.(item.value);
  }

  return (
    <nav className="poodle-breadcrumbs" aria-label={ariaLabel} data-size={resolvedSize} data-density={resolvedDensity}>
      <ol className="poodle-breadcrumbs__list">
        {visibleItems.map((item, index) => (
          <li key={`${item.value}-${index}`} className="poodle-breadcrumbs__item">
            {item.current || (forceLastItemCurrent && index === visibleItems.length - 1) ? (
              <span aria-current="page">{item.label}</span>
            ) : item.href ? (
              <a href={item.href}>{item.label}</a>
            ) : item.value === "__ellipsis__" ? (
              <span aria-hidden="true">{item.label}</span>
            ) : (
              <button type="button" onClick={() => handleNavigate(item)}>
                {item.label}
              </button>
            )}
            {index < visibleItems.length - 1 ? (
              <span className="poodle-breadcrumbs__separator" aria-hidden="true">
                <Icon name="chevron-right" />
              </span>
            ) : null}
          </li>
        ))}
      </ol>
    </nav>
  );
}
