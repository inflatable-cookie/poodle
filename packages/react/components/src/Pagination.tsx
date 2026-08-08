import { useId, type ChangeEvent } from "react";
import { buildVisiblePages, canRequestPage } from "@inflatable-cookie/poodle-headless";

import "@inflatable-cookie/poodle-styles/pagination.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface PaginationControllerLike {
  currentPage: number;
  pageSize: number;
  total: number | null;
  totalPages: number | null;
  showingFrom: number;
  showingTo: number;
  hasPrevPage: boolean;
  hasNextPage: boolean;
  loading?: boolean;
  prevPage: () => void | Promise<void>;
  nextPage: () => void | Promise<void>;
  setPageSize: (pageSize: number) => void | Promise<void>;
  goToPage?: (page: number) => void | Promise<void>;
}

export interface PaginationProps {
  controller?: PaginationControllerLike | null;
  currentPage?: number | null;
  totalPages?: number | null;
  page?: number | null;
  limit?: number | null;
  total?: number | null;
  siblingCount?: number;
  showLimitSelector?: boolean;
  limitOptions?: number[];
  showInfo?: boolean;
  compact?: boolean;
  variant?: "numbered" | "full" | "simple";
  scrollTarget?: HTMLElement | string | false;
  scrollOffset?: number;
  className?: string;
  loading?: boolean;
  chrome?: boolean;
  standalone?: boolean;
  ariaLabel?: string | null;
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  onPageChange?: (page: number) => void;
  onLimitChange?: (limit: number) => void;
}

function getScrollParent(element: HTMLElement): HTMLElement | null {
  let parent = element.parentElement;
  while (parent) {
    const style = getComputedStyle(parent);
    const overflowY = style.overflowY;
    if ((overflowY === "auto" || overflowY === "scroll") && parent.scrollHeight > parent.clientHeight) {
      return parent;
    }
    parent = parent.parentElement;
  }
  return null;
}

export function Pagination({
  controller = null,
  currentPage = null,
  totalPages = null,
  page = null,
  limit = null,
  total = null,
  siblingCount = 1,
  showLimitSelector = false,
  limitOptions = [30, 50, 100],
  showInfo = true,
  compact = false,
  variant = "numbered",
  scrollTarget = false,
  scrollOffset = 16,
  className = "",
  loading = false,
  chrome = false,
  standalone,
  ariaLabel = null,
  sizeRole = "control",
  size = null,
  density = null,
  onPageChange,
  onLimitChange,
}: PaginationProps) {
  const uiPresentation = useUiPresentation();
  const limitSelectId = useId();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const effectiveLimit = controller?.pageSize ?? limit ?? 20;
  const effectiveTotal = controller?.total ?? total;
  const rawTotalPages =
    controller?.totalPages ??
    totalPages ??
    (effectiveTotal !== null && effectiveLimit > 0 ? Math.ceil(effectiveTotal / effectiveLimit) : 1);
  const safeTotalPages = Math.max(1, rawTotalPages ?? 1);
  const rawCurrentPage = controller?.currentPage ?? page ?? currentPage ?? 1;
  const safeCurrentPage = Math.min(Math.max(1, rawCurrentPage), safeTotalPages);
  const startItem =
    controller?.showingFrom ?? ((effectiveTotal ?? 0) === 0 ? 0 : (safeCurrentPage - 1) * effectiveLimit + 1);
  const endItem =
    controller?.showingTo ?? Math.min(safeCurrentPage * effectiveLimit, effectiveTotal ?? safeCurrentPage * effectiveLimit);
  const hasPrevPage = controller?.hasPrevPage ?? safeCurrentPage > 1;
  const hasNextPage = controller?.hasNextPage ?? safeCurrentPage < safeTotalPages;
  const isLoading = controller?.loading ?? loading;
  const supportsGoToPage = typeof controller?.goToPage === "function";
  const visiblePages = buildVisiblePages(safeCurrentPage, safeTotalPages, siblingCount);
  const showRoot = safeTotalPages > 1 || showLimitSelector;
  const showInfoSummary = showInfo && (effectiveTotal ?? 0) > 0;
  const rootClassName = [
    "poodle-pagination",
    className,
    compact ? "poodle-pagination--compact" : "",
    isLoading ? "poodle-pagination--loading" : "",
    (standalone !== undefined ? !standalone : chrome) ? "poodle-pagination--chrome" : "",
  ]
    .filter(Boolean)
    .join(" ");

  function scrollIntoView(): void {
    if (scrollTarget === false || typeof document === "undefined") return;
    let element: HTMLElement | null = null;
    if (typeof scrollTarget === "string") {
      element = document.querySelector<HTMLElement>(scrollTarget);
    } else if (scrollTarget instanceof HTMLElement) {
      element = scrollTarget;
    }
    if (!element) return;
    const scrollParent = getScrollParent(element);
    if (!scrollParent) return;
    const containerRect = scrollParent.getBoundingClientRect();
    const elementRect = element.getBoundingClientRect();
    const relativeTop = elementRect.top - containerRect.top;
    if (relativeTop < scrollOffset && relativeTop > -scrollOffset) return;
    const targetScroll = scrollParent.scrollTop + relativeTop - scrollOffset;
    const maxScroll = scrollParent.scrollHeight - scrollParent.clientHeight;
    scrollParent.scrollTo({ top: Math.max(0, Math.min(targetScroll, maxScroll)), behavior: "smooth" });
  }

  async function handlePageRequest(nextPage: number): Promise<void> {
    if (!canRequestPage(nextPage, safeCurrentPage, safeTotalPages)) return;

    if (controller) {
      if (nextPage === safeCurrentPage - 1) {
        await controller.prevPage();
      } else if (nextPage === safeCurrentPage + 1) {
        await controller.nextPage();
      } else if (controller.goToPage) {
        await controller.goToPage(nextPage);
      } else {
        onPageChange?.(nextPage);
      }
    } else {
      onPageChange?.(nextPage);
    }

    scrollIntoView();
  }

  async function handleLimitChange(event: ChangeEvent<HTMLSelectElement>): Promise<void> {
    const nextLimit = Number(event.target.value);
    if (controller) {
      await controller.setPageSize(nextLimit);
    } else {
      onLimitChange?.(nextLimit);
    }
    scrollIntoView();
  }

  if (!showRoot) return null;

  return (
    <nav
      className={rootClassName}
      aria-label={ariaLabel ?? "Pagination"}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      {showInfoSummary ? (
        <div className="poodle-pagination__info">
          {effectiveTotal !== null
            ? `Showing ${startItem} to ${endItem} of ${effectiveTotal.toLocaleString()}`
            : `Showing ${startItem} to ${endItem}`}
        </div>
      ) : null}

      <div className="poodle-pagination__controls-wrapper">
        {showLimitSelector && limitOptions.length > 0 ? (
          <div className="poodle-pagination__limit">
            <label htmlFor={limitSelectId}>Show</label>
            <select id={limitSelectId} value={effectiveLimit} disabled={isLoading} onChange={handleLimitChange}>
              {limitOptions.map((option) => (
                <option key={option} value={option}>
                  {option}
                </option>
              ))}
            </select>
            <span>per page</span>
          </div>
        ) : null}

        {safeTotalPages > 1 || hasPrevPage || hasNextPage ? (
          <div className="poodle-pagination__controls">
            {variant === "full" && supportsGoToPage ? (
              <button
                type="button"
                className="poodle-pagination__button"
                disabled={!hasPrevPage || isLoading}
                aria-label="First page"
                onClick={() => handlePageRequest(1)}
              >
                ««
              </button>
            ) : null}

            <button
              type="button"
              className="poodle-pagination__button"
              disabled={!hasPrevPage || isLoading}
              aria-label="Previous page"
              onClick={() => handlePageRequest(safeCurrentPage - 1)}
            >
              {variant === "simple" ? "Prev" : "Previous"}
            </button>

            {variant === "numbered" ? (
              <div className="poodle-pagination__pages">
                {visiblePages.map((visiblePage, index) =>
                  visiblePage === "ellipsis" ? (
                    <span key={`e-${index}`} className="poodle-pagination__ellipsis" aria-hidden="true">
                      …
                    </span>
                  ) : (
                    <button
                      key={visiblePage}
                      type="button"
                      className="poodle-pagination__button"
                      data-current={visiblePage === safeCurrentPage}
                      aria-current={visiblePage === safeCurrentPage ? "page" : undefined}
                      aria-label={`Page ${visiblePage}`}
                      onClick={() => handlePageRequest(visiblePage)}
                    >
                      {visiblePage}
                    </button>
                  ),
                )}
              </div>
            ) : variant === "full" ? (
              <span className="poodle-pagination__summary">
                Page {safeCurrentPage} of {safeTotalPages}
              </span>
            ) : (
              <span className="poodle-pagination__summary">
                {startItem}–{endItem}
                {effectiveTotal !== null ? ` of ${effectiveTotal.toLocaleString()}` : null}
              </span>
            )}

            <button
              type="button"
              className="poodle-pagination__button"
              disabled={!hasNextPage || isLoading}
              aria-label="Next page"
              onClick={() => handlePageRequest(safeCurrentPage + 1)}
            >
              Next
            </button>

            {variant === "full" && supportsGoToPage ? (
              <button
                type="button"
                className="poodle-pagination__button"
                disabled={!hasNextPage || isLoading}
                aria-label="Last page"
                onClick={() => handlePageRequest(safeTotalPages)}
              >
                »»
              </button>
            ) : null}
          </div>
        ) : null}
      </div>
    </nav>
  );
}
