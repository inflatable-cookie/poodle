<script lang="ts">
  import { FilterToolbar, ListContainer } from "@poodle/svelte-composites";
  import { Button, IconButton, Pill, SearchInput, Select, Eyebrow, Surface } from "@poodle/svelte-primitives";

  type QueueItem = {
    id: string;
    name: string;
    status: "healthy" | "warning" | "error";
    summary: string;
  };

  const items: QueueItem[] = [
    {
      id: "q-001",
      name: "Media ingest",
      status: "healthy",
      summary: "Thumbnail generation and metadata extraction are within expected latency."
    },
    {
      id: "q-002",
      name: "Search index",
      status: "warning",
      summary: "The queue is recovering after three transient upstream timeouts."
    },
    {
      id: "q-003",
      name: "Email capture",
      status: "error",
      summary: "SMTP relay is failing health checks and needs manual attention."
    }
  ];

  let page = 2;
  let state: "ready" | "loading" | "empty" | "error" = "ready";
  let filtersCollapsed = false;

  $: visibleFrom = items.length === 0 ? 0 : ((page - 1) * 3) + 1;
  $: visibleTo = Math.min(page * 3, 24);
  $: filterSummaryText = `Showing ${visibleFrom}-${visibleTo} of 24`;

  function toneFor(status: QueueItem["status"]): "success" | "neutral" | "danger" {
    if (status === "healthy") return "success";
    if (status === "error") return "danger";
    return "neutral";
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Ready with slots and built-in pagination</Eyebrow>
    <Surface padding="lg" border="subtle">
      <ListContainer
        title="Operational queues"
        subtitle="Monitor health, triage issues, and navigate paged result sets."
        eyebrow="Systems"
        currentPage={page}
        totalPages={8}
        totalItems={24}
        pageSize={3}
        showPaginationSummary={false}
        on:pageChange={(event) => (page = event.detail.page)}
      >
        <svelte:fragment slot="actions">
          <IconButton icon="plus" variant="primary" sizeRole="chrome" ariaLabel="Create incident" tooltip="Create incident" />
        </svelte:fragment>

        <svelte:fragment slot="filters">
          <FilterToolbar
            ariaLabel="Operational queue filters"
            columns={2}
            collapsible
            bind:collapsed={filtersCollapsed}
            summaryText={filterSummaryText}
          >
            <svelte:fragment slot="actions">
              <IconButton
                icon="refresh-cw"
                variant="ghost"
                sizeRole="chrome"
                ariaLabel="Refresh queues"
                tooltip="Refresh queues"
              />
            </svelte:fragment>
            <SearchInput id="list-container-search-input" placeholder="Search queues" ariaLabel="Search queues" />
            <Select
              id="list-container-status-select"
              value=""
              ariaLabel="Status"
              options={[
                { value: "", label: "All statuses" },
                { value: "healthy", label: "Healthy" },
                { value: "warning", label: "Warning" },
                { value: "error", label: "Error" }
              ]}
            />
          </FilterToolbar>
        </svelte:fragment>

        <svelte:fragment slot="batch">
          <div class="specimen__batch">
            <Pill tone="neutral" appearance="badge">3 selected</Pill>
            <Button variant="ghost" sizeRole="chrome">Archive</Button>
            <Button variant="ghost" sizeRole="chrome">Mute alerts</Button>
          </div>
        </svelte:fragment>

        <div class="specimen__list">
          {#each items as item}
            <article class="specimen__card">
              <div class="specimen__card-header">
                <strong>{item.name}</strong>
                <Pill tone={toneFor(item.status)} appearance="badge">{item.status}</Pill>
              </div>
              <p>{item.summary}</p>
            </article>
          {/each}
        </div>
      </ListContainer>
    </Surface>
  </div>

  <div class="specimen__group">
    <Eyebrow>State handling</Eyebrow>
    <Surface padding="lg" border="subtle">
      <div class="specimen__state-buttons">
        <Button variant={state === "ready" ? "primary" : "ghost"} sizeRole="chrome" on:click={() => (state = "ready")}>Ready</Button>
        <Button variant={state === "loading" ? "primary" : "ghost"} sizeRole="chrome" on:click={() => (state = "loading")}>Loading</Button>
        <Button variant={state === "empty" ? "primary" : "ghost"} sizeRole="chrome" on:click={() => (state = "empty")}>Empty</Button>
        <Button variant={state === "error" ? "primary" : "ghost"} sizeRole="chrome" on:click={() => (state = "error")}>Error</Button>
      </div>

      <ListContainer
        title="Workflow incidents"
        subtitle="Review how the built-in state surfaces read inside the list shell."
        {state}
        loadingMessage="Loading workflow incidents..."
        errorMessage="The list failed to load after the latest refresh."
        emptyTitle="No incidents found"
        emptyMessage="Try widening the filters or create a new workflow incident."
      >
        <div class="specimen__card">
          This content is only shown in the ready state.
        </div>
      </ListContainer>
    </Surface>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__group :global(.surface) {
    display: grid;
    gap: 1rem;
  }

  .specimen__batch {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.75rem;
  }

  .specimen__list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__card {
    display: grid;
    gap: 0.5rem;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 92%, transparent);
  }

  .specimen__card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
  }

  .specimen__card p {
    margin: 0;
    color: var(--poodle-color-text-secondary);
  }

  .specimen__state-buttons {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
</style>
