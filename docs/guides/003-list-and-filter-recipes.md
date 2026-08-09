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
  import { ListContainer, FilterToolbar } from "@inflatable-cookie/poodle-svelte";
  import {
    Button,
    Field,
    IconButton,
    TextInput,
    Select
  } from "@inflatable-cookie/poodle-svelte";

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
          options={[
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

## Choosing A Filter Surface

Four generic surfaces compose a browse page. They have distinct jobs and should
not absorb each other:

| Surface | Job | Use when |
|---------|-----|----------|
| `FilterToolbar` | Compact horizontal grouping of a *fixed, small* set of controls (a search box, one or two selects, a refresh action) | The filters are few and always visible |
| `FilterBuilder` | A *growing/arbitrary* set of filter clauses built in a popover, shown as editable pills under one `Match all` / `Match any` combinator | You would otherwise accumulate one dropdown per filter and the toolbar is becoming unmanageable |
| `OrderBy` | An ordered stack of sort directives (field + direction) in a popover | The list needs multi-field sorting |
| `SelectionSummary` | A compact display of the current selection as removable chips | You need to show/clear what is currently selected |

`FilterToolbar` and `FilterBuilder` compose cleanly side by side — the toolbar
holds always-on controls (search, refresh, the `FilterBuilder` trigger itself);
`FilterBuilder` owns the open-ended clause stack. Do **not** extend
`FilterToolbar` into a clause builder, and do not put fixed search/refresh
controls inside `FilterBuilder`.

`FilterBuilder` is generic: the host supplies field definitions (`key`, `label`,
`kind`, `options`, `operators`, `allowMultiple`) and evaluates the emitted
`FilterExpression`. Poodle understands fields, operators, operands and a single
AND/OR combinator — never app vocabulary (formats, tags, vendors) and never the
evaluation or serialization of the expression.

```svelte
<script lang="ts">
  import { FilterBuilder, type FilterExpression, type FilterFieldDefinition } from "@inflatable-cookie/poodle-svelte";

  const fields: FilterFieldDefinition[] = [
    { key: "format", label: "Format", kind: "multi-enum", options: [
      { value: "clap", label: "CLAP" }, { value: "vst3", label: "VST3" },
    ] },
    { key: "hidden", label: "Hidden", kind: "boolean" },
    { key: "tag-count", label: "Tag count", kind: "number" },
  ];
  let filter: FilterExpression = { combinator: "and", clauses: [] };
</script>

<FilterBuilder {fields} value={filter} onChange={(next) => (filter = next)} />
```

## What Stays Out

- row rendering
- domain-specific filters and filter *evaluation* (`FilterBuilder` emits a
  declarative expression; the host evaluates it)
- order-by semantics
- selection state
- command wiring
- list query serialization

Those stay in host code unless Poodle promotes a narrower generic surface at the
root.

## Decision

- use `ListContainer` for list-page shell structure
- use `FilterToolbar` for compact control grouping
- use `FilterBuilder` when the filter set is open-ended (clause stack + pills)
- use `OrderBy` for multi-field sort; `SelectionSummary` for selection chips
- keep query orchestration, list content, filter *evaluation*, and app
  vocabulary in host code

## Related Contracts

- [ListContainer](../contracts/components/list-container.md)
- [FilterToolbar](../contracts/components/filter-toolbar.md)
- [FilterBuilder](../contracts/components/filter-builder.md)
- [OrderBy](../contracts/components/order-by.md)
- [SelectionSummary](../contracts/components/selection-summary.md)
- [PageHeader](../contracts/components/page-header.md)
