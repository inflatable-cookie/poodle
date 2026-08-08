<script module lang="ts">
  let nextPaginationId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/pagination.css";
  import { buildVisiblePages, canRequestPage } from "@inflatable-cookie/poodle-core";

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

  interface Props {
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
    standalone?: boolean | undefined;
    ariaLabel?: string | null;
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    onPageChange?: ((page: number) => void) | undefined;
    onLimitChange?: ((limit: number) => void) | undefined;
  }

  let {
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
    standalone = undefined,
    ariaLabel = null,
    sizeRole = "control",
    size = null,
    density = null,
    onPageChange = undefined,
    onLimitChange = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const limitSelectId = `poodle-pagination-limit-${++nextPaginationId}`;

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const effectiveLimit = $derived(controller?.pageSize ?? limit ?? 20);
  const effectiveTotal = $derived(controller?.total ?? total);
  const rawTotalPages = $derived(
    controller?.totalPages ??
      totalPages ??
      (effectiveTotal !== null && effectiveLimit > 0
        ? Math.ceil(effectiveTotal / effectiveLimit)
        : 1),
  );
  const safeTotalPages = $derived(Math.max(1, rawTotalPages ?? 1));
  const rawCurrentPage = $derived(controller?.currentPage ?? page ?? currentPage ?? 1);
  const safeCurrentPage = $derived(Math.min(Math.max(1, rawCurrentPage), safeTotalPages));
  const startItem = $derived(
    controller?.showingFrom ??
      ((effectiveTotal ?? 0) === 0 ? 0 : (safeCurrentPage - 1) * effectiveLimit + 1),
  );
  const endItem = $derived(
    controller?.showingTo ??
      Math.min(safeCurrentPage * effectiveLimit, effectiveTotal ?? safeCurrentPage * effectiveLimit),
  );
  const hasPrevPage = $derived(controller?.hasPrevPage ?? safeCurrentPage > 1);
  const hasNextPage = $derived(controller?.hasNextPage ?? safeCurrentPage < safeTotalPages);
  const isLoading = $derived(controller?.loading ?? loading);
  const supportsGoToPage = $derived(typeof controller?.goToPage === "function");
  const visiblePages = $derived(buildVisiblePages(safeCurrentPage, safeTotalPages, siblingCount));
  const showRoot = $derived(safeTotalPages > 1 || showLimitSelector);
  const showInfoSummary = $derived(showInfo && (effectiveTotal ?? 0) > 0);
  const rootClassName = $derived(`poodle-pagination${className ? ` ${className}` : ""}`);


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

    if (relativeTop < scrollOffset && relativeTop > -scrollOffset) {
      return;
    }

    const targetScroll = scrollParent.scrollTop + relativeTop - scrollOffset;
    const maxScroll = scrollParent.scrollHeight - scrollParent.clientHeight;
    const clampedScroll = Math.max(0, Math.min(targetScroll, maxScroll));

    scrollParent.scrollTo({ top: clampedScroll, behavior: "smooth" });
  }

  async function handlePageRequest(nextPage: number): Promise<void> {
    if (!canRequestPage(nextPage, safeCurrentPage, safeTotalPages)) {
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
        onPageChange?.(nextPage);
      }
    } else {
      onPageChange?.(nextPage);
    }

    scrollIntoView();
  }

  async function handleLimitChange(event: Event): Promise<void> {
    const nextLimit = Number((event.target as HTMLSelectElement).value);

    if (controller) {
      await controller.setPageSize(nextLimit);
    } else {
      onLimitChange?.(nextLimit);
    }

    scrollIntoView();
  }
</script>

{#if showRoot}
  <nav
    class={rootClassName}
    class:poodle-pagination--compact={compact}
    class:poodle-pagination--loading={isLoading}
    class:poodle-pagination--chrome={standalone !== undefined ? !standalone : chrome}
    aria-label={ariaLabel ?? "Pagination"}
    data-size={resolvedSize}
    data-density={resolvedDensity}
  >
    {#if showInfoSummary}
      <div class="poodle-pagination__info">
        {#if effectiveTotal !== null}
          Showing {startItem} to {endItem} of {effectiveTotal.toLocaleString()}
        {:else}
          Showing {startItem} to {endItem}
        {/if}
      </div>
    {/if}

    <div class="poodle-pagination__controls-wrapper">
      {#if showLimitSelector && limitOptions.length > 0}
        <div class="poodle-pagination__limit">
          <label for={limitSelectId}>Show</label>
          <select
            id={limitSelectId}
            value={effectiveLimit}
            disabled={isLoading}
            onchange={handleLimitChange}
          >
            {#each limitOptions as option}
              <option value={option}>{option}</option>
            {/each}
          </select>
          <span>per page</span>
        </div>
      {/if}

      {#if safeTotalPages > 1 || hasPrevPage || hasNextPage}
        <div class="poodle-pagination__controls">
          {#if variant === "full" && supportsGoToPage}
            <button
              type="button"
              class="poodle-pagination__button"
              disabled={!hasPrevPage || isLoading}
              aria-label="First page"
              onclick={() => handlePageRequest(1)}
            >
              ««
            </button>
          {/if}

          <button
            type="button"
            class="poodle-pagination__button"
            disabled={!hasPrevPage || isLoading}
            aria-label="Previous page"
            onclick={() => handlePageRequest(safeCurrentPage - 1)}
          >
            {#if variant === "simple"}Prev{:else}Previous{/if}
          </button>

          {#if variant === "numbered"}
            <div class="poodle-pagination__pages">
              {#each visiblePages as visiblePage}
                {#if visiblePage === "ellipsis"}
                  <span class="poodle-pagination__ellipsis" aria-hidden="true">…</span>
                {:else}
                  <button
                    type="button"
                    class="poodle-pagination__button"
                    data-current={visiblePage === safeCurrentPage}
                    aria-current={visiblePage === safeCurrentPage ? "page" : undefined}
                    aria-label={`Page ${visiblePage}`}
                    onclick={() => handlePageRequest(visiblePage)}
                  >
                    {visiblePage}
                  </button>
                {/if}
              {/each}
            </div>
          {:else if variant === "full"}
            <span class="poodle-pagination__summary">
              Page {safeCurrentPage} of {safeTotalPages}
            </span>
          {:else}
            <span class="poodle-pagination__summary">
              {startItem}–{endItem}{#if effectiveTotal !== null}&nbsp;of {effectiveTotal.toLocaleString()}{/if}
            </span>
          {/if}

          <button
            type="button"
            class="poodle-pagination__button"
            disabled={!hasNextPage || isLoading}
            aria-label="Next page"
            onclick={() => handlePageRequest(safeCurrentPage + 1)}
          >
            Next
          </button>

          {#if variant === "full" && supportsGoToPage}
            <button
              type="button"
              class="poodle-pagination__button"
              disabled={!hasNextPage || isLoading}
              aria-label="Last page"
              onclick={() => handlePageRequest(safeTotalPages)}
            >
              »»
            </button>
          {/if}
        </div>
      {/if}
    </div>
  </nav>
{/if}

