import "@inflatable-cookie/poodle-styles/list-container.css";

import type { ReactNode } from "react";

import { Callout } from "./Callout";
import { EmptyState } from "./EmptyState";
import { PageHeader } from "./PageHeader";
import { Pagination } from "./Pagination";
import { PaginationSummary } from "./PaginationSummary";
import type { BrowseState, EmptyStateVariant } from "./types";

export interface ListContainerProps {
  title: string;
  subtitle?: string | null;
  eyebrow?: string | null;
  ariaLabel?: string | null;
  state?: Exclude<BrowseState, "no-results">;
  loadingMessage?: string | null;
  errorTitle?: string | null;
  errorMessage?: string | null;
  emptyTitle?: string | null;
  emptyMessage?: string | null;
  emptyVariant?: EmptyStateVariant;
  currentPage?: number;
  totalPages?: number;
  totalItems?: number | null;
  pageSize?: number | null;
  siblingCount?: number;
  paginationAriaLabel?: string | null;
  showPagination?: boolean;
  showPaginationSummary?: boolean;
  onPageChange?: ((page: number) => void) | null;
  children?: ReactNode;
  breadcrumbs?: ReactNode;
  actions?: ReactNode;
  filters?: ReactNode;
  batch?: ReactNode;
  pagination?: ReactNode;
  loading?: ReactNode;
  error?: ReactNode;
  empty?: ReactNode;
}

export function ListContainer({
  title,
  subtitle = null,
  eyebrow = null,
  ariaLabel = null,
  state = "ready",
  loadingMessage = "Loading items...",
  errorTitle = "Unable to load list",
  errorMessage = null,
  emptyTitle = "Nothing here yet",
  emptyMessage = null,
  emptyVariant = "neutral",
  currentPage = 1,
  totalPages = 1,
  totalItems = null,
  pageSize = null,
  siblingCount = 1,
  paginationAriaLabel = null,
  showPagination = true,
  showPaginationSummary = true,
  onPageChange = null,
  children,
  breadcrumbs,
  actions,
  filters,
  batch,
  pagination,
  loading,
  error,
  empty,
}: ListContainerProps) {
  const shouldShowPagination = showPagination && state === "ready" && totalPages > 1;
  const shouldShowPaginationSummary =
    shouldShowPagination && showPaginationSummary && totalItems !== null && pageSize !== null;

  return (
    <section className="poodle-list-container" aria-label={ariaLabel ?? title} data-state={state}>
      <PageHeader title={title} subtitle={subtitle} eyebrow={eyebrow} breadcrumbs={breadcrumbs} actions={actions} />

      {state === "ready" ? (
        <>
          {filters ? <div className="poodle-list-container__filters">{filters}</div> : null}

          {batch ? <div className="poodle-list-container__batch">{batch}</div> : null}

          <div className="poodle-list-container__content">{children}</div>

          {shouldShowPagination || pagination ? (
            <div className="poodle-list-container__pagination">
              {pagination ?? (
                <>
                  {shouldShowPaginationSummary ? (
                    <PaginationSummary
                      currentPage={currentPage}
                      totalPages={totalPages}
                      totalItems={totalItems ?? 0}
                      pageSize={pageSize ?? 1}
                    />
                  ) : null}

                  <Pagination
                    currentPage={currentPage}
                    totalPages={totalPages}
                    siblingCount={siblingCount}
                    ariaLabel={paginationAriaLabel ?? "List pagination"}
                    onPageChange={(page) => onPageChange?.(page)}
                  />
                </>
              )}
            </div>
          ) : null}
        </>
      ) : state === "loading" ? (
        <div className="poodle-list-container__state">
          {loading ?? <Callout tone="pending" message={loadingMessage} />}
        </div>
      ) : state === "error" ? (
        <div className="poodle-list-container__state">
          {error ?? <Callout tone="danger" title={errorTitle} message={errorMessage} announceMode="assertive" />}
        </div>
      ) : (
        <div className="poodle-list-container__state">
          {empty ?? <EmptyState title={emptyTitle ?? "Nothing here yet"} message={emptyMessage} variant={emptyVariant} />}
        </div>
      )}
    </section>
  );
}
