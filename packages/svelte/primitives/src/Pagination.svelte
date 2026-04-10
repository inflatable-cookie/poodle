<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface PaginationControllerLike {
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

  export let controller: PaginationControllerLike | null = null;
  export let currentPage: number | null = null;
  export let totalPages: number | null = null;
  export let page: number | null = null;
  export let limit: number | null = null;
  export let total: number | null = null;
  export let siblingCount = 1;
  export let showLimitSelector = false;
  export let limitOptions: number[] = [30, 50, 100];
  export let showInfo = true;
  export let compact = false;
  export let variant: "numbered" | "full" | "simple" = "numbered";
  export let scrollTarget: HTMLElement | string | false = false;
  export let scrollOffset = 16;
  export let className = "";
  export let loading = false;
  /** When true, renders without container padding, border, and background. */
  export let standalone = false;
  export let ariaLabel: string | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let size: ControlSize | null = null;
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    pageChange: { page: number };
    limitChange: { limit: number };
  }>();

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: effectiveLimit = controller?.pageSize ?? limit ?? 20;
  $: effectiveTotal = controller?.total ?? total;
  $: rawTotalPages =
    controller?.totalPages ??
    totalPages ??
    (effectiveTotal !== null && effectiveLimit > 0
      ? Math.ceil(effectiveTotal / effectiveLimit)
      : 1);
  $: safeTotalPages = Math.max(1, rawTotalPages ?? 1);
  $: rawCurrentPage = controller?.currentPage ?? page ?? currentPage ?? 1;
  $: safeCurrentPage = Math.min(Math.max(1, rawCurrentPage), safeTotalPages);
  $: startItem =
    controller?.showingFrom ??
    ((effectiveTotal ?? 0) === 0 ? 0 : (safeCurrentPage - 1) * effectiveLimit + 1);
  $: endItem =
    controller?.showingTo ??
    Math.min(safeCurrentPage * effectiveLimit, effectiveTotal ?? safeCurrentPage * effectiveLimit);
  $: hasPrevPage = controller?.hasPrevPage ?? safeCurrentPage > 1;
  $: hasNextPage = controller?.hasNextPage ?? safeCurrentPage < safeTotalPages;
  $: isLoading = controller?.loading ?? loading;
  $: supportsGoToPage = typeof controller?.goToPage === "function";
  $: visiblePages = buildVisiblePages(safeCurrentPage, safeTotalPages, siblingCount);

  function buildVisiblePages(
    page: number,
    count: number,
    siblings: number
  ): Array<number | "ellipsis"> {
    const pages = new Set<number>([1, count]);

    for (let candidate = page - siblings; candidate <= page + siblings; candidate += 1) {
      if (candidate >= 1 && candidate <= count) {
        pages.add(candidate);
      }
    }

    const sorted = Array.from(pages).sort((left, right) => left - right);
    const result: Array<number | "ellipsis"> = [];

    for (let index = 0; index < sorted.length; index += 1) {
      const current = sorted[index];
      const previous = sorted[index - 1];

      if (previous && current - previous > 1) {
        result.push("ellipsis");
      }

      result.push(current);
    }

    return result;
  }

  function requestPage(page: number): void {
    if (page < 1 || page > safeTotalPages || page === safeCurrentPage) {
      return;
    }

    dispatch("pageChange", { page });
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

  function scrollIntoView(): void {
    if (scrollTarget === false) return;

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

    if (relativeTop < scrollOffset && relativeTop > -scrollOffset) {
      return;
    }

    const targetScroll = scrollParent.scrollTop + relativeTop - scrollOffset;
    const maxScroll = scrollParent.scrollHeight - scrollParent.clientHeight;
    const clampedScroll = Math.max(0, Math.min(targetScroll, maxScroll));

    scrollParent.scrollTo({ top: clampedScroll, behavior: "smooth" });
  }

  async function handlePageRequest(nextPage: number): Promise<void> {
    if (nextPage < 1 || nextPage > safeTotalPages || nextPage === safeCurrentPage) {
      return;
    }

    if (controller) {
      if (nextPage === safeCurrentPage - 1) {
        await controller.prevPage();
      } else if (nextPage === safeCurrentPage + 1) {
        await controller.nextPage();
      } else if (controller.goToPage) {
        await controller.goToPage(nextPage);
      } else {
        dispatch("pageChange", { page: nextPage });
      }
    } else {
      dispatch("pageChange", { page: nextPage });
    }

    scrollIntoView();
  }

  async function handleLimitChange(event: Event): Promise<void> {
    const nextLimit = Number((event.target as HTMLSelectElement).value);

    if (controller) {
      await controller.setPageSize(nextLimit);
    } else {
      dispatch("limitChange", { limit: nextLimit });
    }

    scrollIntoView();
  }
</script>

{#if safeTotalPages > 1 || showLimitSelector}
  <nav
    class="pagination {className}"
    class:pagination--compact={compact}
    class:pagination--loading={isLoading}
    class:pagination--standalone={standalone}
    aria-label={ariaLabel ?? "Pagination"}
    data-size={resolvedSize}
    data-density={resolvedDensity}
  >
    {#if showInfo && (effectiveTotal ?? 0) > 0}
      <div class="pagination__info">
        {#if effectiveTotal !== null}
          Showing {startItem} to {endItem} of {effectiveTotal.toLocaleString()}
        {:else}
          Showing {startItem} to {endItem}
        {/if}
      </div>
    {/if}

    <div class="pagination__controls-wrapper">
      {#if showLimitSelector && limitOptions.length > 0}
        <div class="pagination__limit">
          <label for="pagination-limit">Show</label>
          <select
            id="pagination-limit"
            value={effectiveLimit}
            disabled={isLoading}
            on:change={handleLimitChange}
          >
            {#each limitOptions as option}
              <option value={option}>{option}</option>
            {/each}
          </select>
          <span>per page</span>
        </div>
      {/if}

      {#if safeTotalPages > 1 || hasPrevPage || hasNextPage}
        <div class="pagination__controls">
          {#if variant === "full" && supportsGoToPage}
            <button
              type="button"
              class="pagination__button"
              disabled={!hasPrevPage || isLoading}
              aria-label="First page"
              on:click={() => handlePageRequest(1)}
            >
              ««
            </button>
          {/if}

          <button
            type="button"
            class="pagination__button"
            disabled={!hasPrevPage || isLoading}
            aria-label="Previous page"
            on:click={() => handlePageRequest(safeCurrentPage - 1)}
          >
            {#if variant === "simple"}Prev{:else}Previous{/if}
          </button>

          {#if variant === "numbered"}
            <div class="pagination__pages">
              {#each visiblePages as visiblePage}
                {#if visiblePage === "ellipsis"}
                  <span class="pagination__ellipsis" aria-hidden="true">…</span>
                {:else}
                  <button
                    type="button"
                    class="pagination__button"
                    data-current={visiblePage === safeCurrentPage}
                    aria-current={visiblePage === safeCurrentPage ? "page" : undefined}
                    aria-label={`Page ${visiblePage}`}
                    on:click={() => handlePageRequest(visiblePage)}
                  >
                    {visiblePage}
                  </button>
                {/if}
              {/each}
            </div>
          {:else if variant === "full"}
            <span class="pagination__summary">
              Page {safeCurrentPage} of {safeTotalPages}
            </span>
          {:else}
            <span class="pagination__summary">
              {startItem}–{endItem}{#if effectiveTotal !== null}&nbsp;of {effectiveTotal.toLocaleString()}{/if}
            </span>
          {/if}

          <button
            type="button"
            class="pagination__button"
            disabled={!hasNextPage || isLoading}
            aria-label="Next page"
            on:click={() => handlePageRequest(safeCurrentPage + 1)}
          >
            {#if variant === "simple"}Next{:else}Next{/if}
          </button>

          {#if variant === "full" && supportsGoToPage}
            <button
              type="button"
              class="pagination__button"
              disabled={!hasNextPage || isLoading}
              aria-label="Last page"
              on:click={() => handlePageRequest(safeTotalPages)}
            >
              »»
            </button>
          {/if}
        </div>
      {/if}
    </div>
  </nav>
{/if}

<style>
  .pagination {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    flex-wrap: wrap;
    margin-top: var(--poodle-space-panel-y);
    padding: var(--poodle-space-control-y) var(--poodle-space-panel-x);
    border-top: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 92%, transparent);
  }

  .pagination--standalone {
    margin-top: 0;
    padding: 0;
    border-top: none;
    background: transparent;
  }

  .pagination--compact {
    padding: 0.5rem 0.75rem;
    gap: 0.75rem;
  }

  .pagination--loading {
    opacity: 0.7;
    pointer-events: none;
  }

  .pagination__info {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .pagination__controls-wrapper {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .pagination__limit {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
  }

  .pagination__limit select {
    padding: 0.25rem 0.5rem;
    font: inherit;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 78%, transparent);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
  }

  .pagination__limit select:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .pagination__controls {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    flex-wrap: wrap;
  }

  .pagination__pages {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
  }

  .pagination__button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: var(--poodle-size-control-height);
    height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 78%, transparent);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
  }

  .pagination__button[data-current="true"] {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent);
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 42%, var(--poodle-color-border-default));
  }

  .pagination__button:hover:not(:disabled),
  .pagination__button:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
    outline: none;
  }

  .pagination__ellipsis {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 1.5rem;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
  }

  .pagination__summary {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    white-space: nowrap;
    padding: 0 0.5rem;
  }

  .pagination__button:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Size variants */
  .pagination[data-size="xs"] .pagination__button { min-width: var(--poodle-size-control-height); height: var(--poodle-size-control-height); font-size: 0.6875rem; }
  .pagination[data-size="sm"] .pagination__button { min-width: var(--poodle-size-control-height); height: var(--poodle-size-control-height); }
  .pagination[data-size="lg"] .pagination__button { min-width: var(--poodle-size-control-height); height: var(--poodle-size-control-height); font-size: 0.875rem; }
  .pagination[data-size="xl"] .pagination__button { min-width: var(--poodle-size-control-height); height: var(--poodle-size-control-height); font-size: 0.9375rem; }

  /* Density variants */
  .pagination[data-density="compact"] .pagination__controls,
  .pagination[data-density="compact"] .pagination__pages {
    gap: 0.0625rem;
  }

  .pagination[data-density="comfortable"] .pagination__controls,
  .pagination[data-density="comfortable"] .pagination__pages {
    gap: 0.25rem;
  }

  @media (max-width: 40rem) {
    .pagination {
      flex-direction: column;
      align-items: stretch;
    }

    .pagination--compact .pagination__info {
      display: none;
    }

    .pagination__controls-wrapper {
      justify-content: center;
    }
  }
</style>
