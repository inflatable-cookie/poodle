/**
 * Pagination behavior machinery.
 * Contract: docs/contracts/components/pagination.md, "Behavior Machine".
 *
 * Pure page-window math and navigation guards; the adapter owns controller
 * integration, scrolling, and callbacks.
 */

export type VisiblePage = number | "ellipsis";

/**
 * Page-number window: first and last pages always visible, `siblings` pages
 * around the current one, ellipsis markers for gaps of more than one page.
 */
export function buildVisiblePages(page: number, count: number, siblings: number): VisiblePage[] {
  const pages = new Set<number>([1, count]);

  for (let candidate = page - siblings; candidate <= page + siblings; candidate += 1) {
    if (candidate >= 1 && candidate <= count) {
      pages.add(candidate);
    }
  }

  const sorted = Array.from(pages).sort((left, right) => left - right);
  const result: VisiblePage[] = [];

  for (let index = 0; index < sorted.length; index += 1) {
    const current = sorted[index];
    const previous = sorted[index - 1];

    if (current === undefined) {
      continue;
    }

    if (previous !== undefined && current - previous > 1) {
      result.push("ellipsis");
    }

    result.push(current);
  }

  return result;
}

/** A page request is valid when in bounds and actually a navigation. */
export function canRequestPage(nextPage: number, currentPage: number, totalPages: number): boolean {
  return Number.isInteger(nextPage) && nextPage >= 1 && nextPage <= totalPages && nextPage !== currentPage;
}
