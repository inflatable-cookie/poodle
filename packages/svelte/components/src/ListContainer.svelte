<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Callout from "./Callout.svelte";
  import Pagination from "./Pagination.svelte";
  import PaginationSummary from "./PaginationSummary.svelte";

  import EmptyState from "./EmptyState.svelte";
  import PageHeader from "./PageHeader.svelte";

  import type { BrowseState, EmptyStateVariant } from "./types";

  export let title: string;
  export let subtitle: string | null = null;
  export let eyebrow: string | null = null;
  export let ariaLabel: string | null = null;
  export let state: Exclude<BrowseState, "no-results"> = "ready";
  export let loadingMessage: string | null = "Loading items...";
  export let errorTitle: string | null = "Unable to load list";
  export let errorMessage: string | null = null;
  export let emptyTitle: string | null = "Nothing here yet";
  export let emptyMessage: string | null = null;
  export let emptyVariant: EmptyStateVariant = "neutral";
  export let currentPage = 1;
  export let totalPages = 1;
  export let totalItems: number | null = null;
  export let pageSize: number | null = null;
  export let siblingCount = 1;
  export let paginationAriaLabel: string | null = null;
  export let showPagination = true;
  export let showPaginationSummary = true;

  const dispatch = createEventDispatcher<{
    pageChange: { page: number };
  }>();

  $: shouldShowPagination =
    showPagination && state === "ready" && totalPages > 1;
  $: shouldShowPaginationSummary =
    shouldShowPagination &&
    showPaginationSummary &&
    totalItems !== null &&
    pageSize !== null;

  function forwardPageChange(page: number): void {
    dispatch("pageChange", { page });
  }
</script>

<section class="list-container" aria-label={ariaLabel ?? title} data-state={state}>
  {#if $$slots.breadcrumbs && $$slots.actions}
    <PageHeader {title} {subtitle} {eyebrow}>
      {#snippet breadcrumbs()}
        <slot name="breadcrumbs" />
      {/snippet}
      {#snippet actions()}
        <slot name="actions" />
      {/snippet}
    </PageHeader>
  {:else if $$slots.breadcrumbs}
    <PageHeader {title} {subtitle} {eyebrow}>
      {#snippet breadcrumbs()}
        <slot name="breadcrumbs" />
      {/snippet}
    </PageHeader>
  {:else if $$slots.actions}
    <PageHeader {title} {subtitle} {eyebrow}>
      {#snippet actions()}
        <slot name="actions" />
      {/snippet}
    </PageHeader>
  {:else}
    <PageHeader {title} {subtitle} {eyebrow} />
  {/if}

  {#if state === "ready"}
    {#if $$slots.filters}
      <div class="list-container__filters">
        <slot name="filters" />
      </div>
    {/if}

    {#if $$slots.batch}
      <div class="list-container__batch">
        <slot name="batch" />
      </div>
    {/if}

    <div class="list-container__content">
      <slot />
    </div>

    {#if shouldShowPagination || $$slots.pagination}
      <div class="list-container__pagination">
        {#if $$slots.pagination}
          <slot name="pagination" />
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
            on:pageChange={(event) => forwardPageChange(event.detail.page)}
          />
        {/if}
      </div>
    {/if}
  {:else if state === "loading"}
    <div class="list-container__state">
      {#if $$slots.loading}
        <slot name="loading" />
      {:else}
        <Callout tone="pending" message={loadingMessage} />
      {/if}
    </div>
  {:else if state === "error"}
    <div class="list-container__state">
      {#if $$slots.error}
        <slot name="error" />
      {:else}
        <Callout tone="danger" title={errorTitle} message={errorMessage} announceMode="assertive" />
      {/if}
    </div>
  {:else}
    <div class="list-container__state">
      {#if $$slots.empty}
        <slot name="empty" />
      {:else}
        <EmptyState title={emptyTitle ?? "Nothing here yet"} message={emptyMessage} variant={emptyVariant} />
      {/if}
    </div>
  {/if}
</section>

<style>
  .list-container {
    display: grid;
    gap: var(--poodle-space-stack-lg);
  }

  .list-container__filters,
  .list-container__batch,
  .list-container__content,
  .list-container__state {
    display: grid;
    gap: var(--poodle-space-stack-md);
  }

  .list-container__pagination {
    display: grid;
    gap: var(--poodle-space-stack-md);
  }

  .list-container__pagination :global(.pagination-summary) {
    width: 100%;
  }

  .list-container__pagination :global(.pagination) {
    justify-self: start;
  }
</style>
