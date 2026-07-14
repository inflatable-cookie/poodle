import "@poodle/styles/pagination-summary.css";

export interface PaginationSummaryProps {
  currentPage?: number;
  totalPages?: number;
  totalItems?: number;
  pageSize?: number;
}

export function PaginationSummary({
  currentPage = 1,
  totalPages = 1,
  totalItems = 0,
  pageSize = 5,
}: PaginationSummaryProps) {
  const fromItem = totalItems === 0 ? 0 : (currentPage - 1) * pageSize + 1;
  const toItem = Math.min(currentPage * pageSize, totalItems);

  return (
    <div
      className="poodle-pagination-summary"
      aria-live="polite"
      aria-label={`Showing ${fromItem}-${toItem} of ${totalItems} across ${totalPages} pages`}
    >
      <p className="poodle-pagination-summary__copy">
        Showing {fromItem}-{toItem} of {totalItems}
      </p>
    </div>
  );
}
