# List And Filter Recipes

Reusable browse-page composition rules for Poodle-based Svelte apps.

## Purpose

Use this guide when contracts tell you what `ListContainer` and
`FilterToolbar` do, but you need a stable answer for how to assemble a real
list page without rebuilding old shell wrappers.

## Default Posture

- `ListContainer` owns the page-level list shell
- `FilterToolbar` owns compact filter/search grouping
- cards, rows, tables, and query state stay host-owned
- batch actions stay caller-owned unless Poodle later promotes a narrower,
  clearly generic strip

## Standard List Page

Use `ListContainer` for title, actions, filters, state switching, and
pagination placement. Render the actual list or grid in the default slot.

```svelte
<script lang="ts">
  import { ListContainer, FilterToolbar } from "@poodle/svelte";
  import {
    Button,
    Field,
    IconButton,
    TextInput,
    Select
  } from "@poodle/svelte";

  let collapsed = false;
</script>

<ListContainer
  title="Projects"
  subtitle="Browse, filter, and maintain project records."
  state="ready"
  currentPage={2}
  totalPages={8}
  totalItems={24}
  pageSize={3}
>
  <svelte:fragment slot="actions">
    <Button variant="primary">Create project</Button>
  </svelte:fragment>

  <svelte:fragment slot="filters">
    <FilterToolbar
      summaryText="Showing 4-6 of 24"
      collapsible
      bind:collapsed
    >
      <svelte:fragment slot="actions">
        <IconButton
          icon="refresh-cw"
          variant="ghost"
          tooltip="Refresh"
        />
      </svelte:fragment>

      <Field id="project-search" label="Search">
        <TextInput id="project-search" type="search" placeholder="Search projects" />
      </Field>

      <Field id="project-status" label="Status">
        <Select
          id="project-status"
          items={[
            { value: "all", label: "All statuses" },
            { value: "active", label: "Active" },
            { value: "archived", label: "Archived" }
          ]}
        />
      </Field>
    </FilterToolbar>
  </svelte:fragment>

  <!-- Host-owned list or grid content here -->
</ListContainer>
```

## Batch Actions

Keep batch actions in the `batch` slot rather than pushing them into a custom
list wrapper.

```svelte
<svelte:fragment slot="batch">
  <Button variant="secondary">Archive selected</Button>
  <Button variant="secondary" tone="danger">Delete selected</Button>
</svelte:fragment>
```

## State Handling

Let `ListContainer` own ready vs loading vs error vs empty posture. Do not
build page-specific conditional wrappers around the entire browse shell unless
the page has genuinely custom state treatment.

```svelte
<ListContainer
  title="Incidents"
  state="empty"
  emptyTitle="No incidents found"
  emptyMessage="Try widening the current filters or create a new incident."
/>
```

Use state overrides only when the page truly needs a custom region:

```svelte
<ListContainer title="Jobs" state="error" errorTitle="Unable to load jobs">
  <svelte:fragment slot="error">
    <!-- custom recovery UI -->
  </svelte:fragment>
</ListContainer>
```

When host code owns the state region directly:

- use `PageLoading` for the main loading branch
- use `Callout tone="danger"` for recoverable load failures
- include a small ghost `Retry` action when the route can refetch locally
- use `EmptyState` for no-results posture, with copy that reflects the current
  filters or browse context instead of a generic failure message

## FilterToolbar Rules

- Use `summaryText` for result summary or active-filter summary
- Keep refresh and similar utility actions in the `actions` slot
- Use `IconButton variant="ghost"` for toolbar utility actions
- Use `secondary` rather than `ghost` for non-primary clustered actions outside
  the filter toolbar

## What Stays Out

- row rendering
- domain-specific filters
- order-by semantics
- selection state
- command wiring
- list query serialization

Those stay in host code unless Poodle promotes a narrower generic surface at the
root.

## Decision

- use `ListContainer` for list-page shell structure
- use `FilterToolbar` for compact control grouping
- keep query orchestration, list content, and app vocabulary in host code

## Related Contracts

- [ListContainer](../contracts/components/list-container.md)
- [FilterToolbar](../contracts/components/filter-toolbar.md)
- [PageHeader](../contracts/components/page-header.md)

## Next Task

Add the next browse-surface guide once the table-shell decision hardens in real
app migration work, starting with `DataTable` plus `ListContainer` composition
and then documenting when a page should stay table-first versus card/grid-first.
