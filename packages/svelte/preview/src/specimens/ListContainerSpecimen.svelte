<script lang="ts">
  import { FilterToolbar, ListContainer } from "@poodle/svelte";
  import { Button, IconButton, Pill, TextInput, Select, Surface } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

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

<div class="poodle-specimen">
  <SpecimenGroup label="Ready with slots and built-in pagination">
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
            <TextInput id="list-container-search-input" type="search" placeholder="Search queues" ariaLabel="Search queues" />
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
          <div class="poodle-specimen__batch">
            <Pill tone="neutral" appearance="badge">3 selected</Pill>
            <Button variant="ghost" sizeRole="chrome">Archive</Button>
            <Button variant="ghost" sizeRole="chrome">Mute alerts</Button>
          </div>
        </svelte:fragment>

        <div class="poodle-specimen__list">
          {#each items as item}
            <article class="poodle-specimen__card">
              <div class="poodle-specimen__card-header">
                <strong>{item.name}</strong>
                <Pill tone={toneFor(item.status)} appearance="badge">{item.status}</Pill>
              </div>
              <p>{item.summary}</p>
            </article>
          {/each}
        </div>
      </ListContainer>
    </Surface>
  </SpecimenGroup>

  <SpecimenGroup label="State handling">
    <Surface padding="lg" border="subtle">
      <div class="poodle-specimen__state-buttons">
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
        <div class="poodle-specimen__card">
          This content is only shown in the ready state.
        </div>
      </ListContainer>
    </Surface>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__batch {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.75rem;
  }

  .poodle-specimen__list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .poodle-specimen__card {
    display: grid;
    gap: 0.5rem;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 92%, transparent);
  }

  .poodle-specimen__card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
  }

  .poodle-specimen__card p {
    margin: 0;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-specimen__state-buttons {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
</style>
