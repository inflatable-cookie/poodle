<script lang="ts">
  import type { Snippet } from "svelte";

  import Callout from "./Callout.svelte";
  import Pagination from "./Pagination.svelte";
  import PaginationSummary from "./PaginationSummary.svelte";

  import EmptyState from "./EmptyState.svelte";
  import PageHeader from "./PageHeader.svelte";

  import type { BrowseState, EmptyStateVariant } from "./types";

  interface Props {
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
    children?: Snippet<[]>;
    breadcrumbs?: Snippet<[]>;
    actions?: Snippet<[]>;
    filters?: Snippet<[]>;
    batch?: Snippet<[]>;
    pagination?: Snippet<[]>;
    loading?: Snippet<[]>;
    error?: Snippet<[]>;
    empty?: Snippet<[]>;
  }

  let {
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
  }: Props = $props();

  const shouldShowPagination = $derived(showPagination && state === "ready" && totalPages > 1);
  const shouldShowPaginationSummary = $derived(
    shouldShowPagination &&
      showPaginationSummary &&
      totalItems !== null &&
      pageSize !== null
  );

  function forwardPageChange(page: number): void {
    onPageChange?.(page);
  }
</script>

<section class="poodle-list-container" aria-label={ariaLabel ?? title} data-state={state}>
  {#if breadcrumbs && actions}
    <PageHeader {title} {subtitle} {eyebrow}>
      {#snippet breadcrumbs()}
        {@render breadcrumbs()}
      {/snippet}
      {#snippet actions()}
        {@render actions()}
      {/snippet}
    </PageHeader>
  {:else if breadcrumbs}
    <PageHeader {title} {subtitle} {eyebrow}>
      {#snippet breadcrumbs()}
        {@render breadcrumbs()}
      {/snippet}
    </PageHeader>
  {:else if actions}
    <PageHeader {title} {subtitle} {eyebrow}>
      {#snippet actions()}
        {@render actions()}
      {/snippet}
    </PageHeader>
  {:else}
    <PageHeader {title} {subtitle} {eyebrow} />
  {/if}

  {#if state === "ready"}
    {#if filters}
      <div class="poodle-list-container__filters">
        {@render filters()}
      </div>
    {/if}

    {#if batch}
      <div class="poodle-list-container__batch">
        {@render batch()}
      </div>
    {/if}

    <div class="poodle-list-container__content">
      {@render children?.()}
    </div>

    {#if shouldShowPagination || pagination}
      <div class="poodle-list-container__pagination">
        {#if pagination}
          {@render pagination()}
        {:else}
          {#if shouldShowPaginationSummary}
            <PaginationSummary
              {currentPage}
              {totalPages}
              totalItems={totalItems ?? 0}
              pageSize={pageSize ?? 1}
            />
          {/if}

          <Pagination
            {currentPage}
            {totalPages}
            {siblingCount}
            ariaLabel={paginationAriaLabel ?? "List pagination"}
            onPageChange={forwardPageChange}
          />
        {/if}
      </div>
    {/if}
  {:else if state === "loading"}
    <div class="poodle-list-container__state">
      {#if loading}
        {@render loading()}
      {:else}
        <Callout tone="pending" message={loadingMessage} />
      {/if}
    </div>
  {:else if state === "error"}
    <div class="poodle-list-container__state">
      {#if error}
        {@render error()}
      {:else}
        <Callout tone="danger" title={errorTitle} message={errorMessage} announceMode="assertive" />
      {/if}
    </div>
  {:else}
    <div class="poodle-list-container__state">
      {#if empty}
        {@render empty()}
      {:else}
        <EmptyState title={emptyTitle ?? "Nothing here yet"} message={emptyMessage} variant={emptyVariant} />
      {/if}
    </div>
  {/if}
</section>

<style>
  .poodle-list-container {
    display: grid;
    gap: var(--poodle-space-stack-lg);
  }

  .poodle-list-container__filters,
  .poodle-list-container__batch,
  .poodle-list-container__content,
  .poodle-list-container__state {
    display: grid;
    gap: var(--poodle-space-stack-md);
  }

  .poodle-list-container__pagination {
    display: grid;
    gap: var(--poodle-space-stack-md);
  }

  .poodle-list-container__pagination :global(.poodle-pagination-summary) {
    width: 100%;
  }

  .poodle-list-container__pagination :global(.poodle-pagination) {
    justify-self: end;
  }
</style>
