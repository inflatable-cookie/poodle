# Page Shell And Admin Recipes

Reusable page-shell composition rules for Poodle-based admin and back-office
apps.

## Purpose

Use this guide when contracts tell you what `PageHeader`, `MetaBar`,
`ListContainer`, `Tabs`, `FormDialog`, and related Poodle surfaces do, but you
need a stable answer for how to assemble real admin pages without recreating a
second shared shell layer.

## Default Posture

- use `PageHeader` directly for page framing
- use `MetaBar` and `MetaItem` for compact metadata rows
- use `ListContainer` and `FilterToolbar` for browse pages
- use `Tabs` directly for section switching
- keep route wiring, command execution, and app-local menus in host code

## Reference Implementations

Use the ACME admin route family in the separate `underlay-reference`
repository as the concrete reference family for this recipe, especially
projects, categories, and scheduled-task list/detail/edit pages.

## Detail Page Header

```svelte
<script lang="ts">
  import { PageHeader } from "@poodle/svelte";
  import { Breadcrumbs, Code, MetaBar, MetaItem, Pill } from "@poodle/svelte";
</script>

<div class="detail-page__header">
  <PageHeader
    section="Project"
    title={project.name}
    backHref="/projects"
    backLabel="Back to projects"
    bannerMessage={!project.isLive ? "This project is not live." : undefined}
  >
    {#snippet breadcrumbs()}
      <Breadcrumbs items={breadcrumbs} />
    {/snippet}
    {#snippet actions()}
      <ProjectActionsMenu {project} />
    {/snippet}
  </PageHeader>

  <MetaBar ariaLabel="Project metadata">
    <MetaItem label="ID">
      <Code inline source={project.id} showCopyButton />
    </MetaItem>
    <Pill tone={project.isLive ? "success" : "danger"} appearance="badge">
      {project.isLive ? "Live" : "Draft"}
    </Pill>
  </MetaBar>
</div>
```

## List Page Shell

```svelte
<script lang="ts">
  import { FilterToolbar, ListContainer } from "@poodle/svelte";
  import { Button, Field, TextInput, Select } from "@poodle/svelte";
</script>

<ListContainer
  title="Projects"
  subtitle="Browse and maintain project records."
  state="ready"
  currentPage={page}
  totalPages={totalPages}
  totalItems={total}
  pageSize={pageSize}
>
  {#snippet actions()}
    <Button href="/projects/new">Add project</Button>
  {/snippet}

  {#snippet filters()}
    <FilterToolbar summaryText={summaryText}>
      <Field id="project-search" label="Search">
        <TextInput id="project-search" type="search" bind:value={query} />
      </Field>

      <Field id="project-status" label="Status">
        <Select id="project-status" value={status} options={statusOptions} />
      </Field>
    </FilterToolbar>
  {/snippet}

  <!-- host-owned list/card/table content -->
</ListContainer>
```

## Overview Page Pattern

Use this when the route is an app landing page, admin dashboard, or overview
surface rather than a CRUD browse page.

```svelte
<script lang="ts">
  import { MetricTile, NavCard, PageHeader } from "@poodle/svelte";
  import { Callout, Grid } from "@poodle/svelte";
</script>

<PageHeader
  title="Dashboard"
  subtitle="Platform overview and key metrics"
/>

{#if loadError}
  <Callout tone="danger" message={loadError} />
{/if}

<div class="metrics-grid">
  <a class="metric-link" href="/users">
    <MetricTile label="Active users" value={activeUsersLabel} />
  </a>
</div>

<section>
  <h2>Key areas</h2>
  <Grid columns="repeat(auto-fit, minmax(16rem, 1fr))" gap="md">
    <NavCard href="/system" title="System" description="Diagnostics and tools." />
  </Grid>
</section>
```

Rules for this pattern:

- use `PageHeader` for the route title and subtitle instead of ad hoc header
  markup
- keep summary metrics as host-owned `MetricTile` link cards
- use one or more secondary sections below the metrics grid:
  `NavCard`, `LogList`, or another local host-owned section
- keep the metric and section contents app-owned; the shared pattern is the
  page shape, not a canned dashboard data model
- keep loading and error posture simple and local

## Workflow Overview Variant

Use this when the route is still an overview page, but it primarily launches
workflow queues or review surfaces rather than summarizing metrics.

Rules for this variant:

- keep the same `PageHeader` shell with a clear task-oriented subtitle
- use grouped `NavCard` bands with short section copy when the page is mainly a
  workflow launcher
- add small host-owned badges like `Queue` or `Review` on the cards when they
  help distinguish staged next actions
- skip the metric band entirely when the route has no stable summary numbers;
  do not invent fake stats just to satisfy the overview layout
- keep the sections app-owned even when the shared page shape is reused

## Section Tabs

Use `Tabs` directly. Keep mount policy app-owned.

```svelte
<script lang="ts">
  import { Tabs } from "@poodle/svelte";

  let activeTab = "details";
</script>

<Tabs
  bind:value={activeTab}
  items={[
    { value: "details", label: "Details" },
    { value: "activity", label: "Activity", count: 12 }
  ]}
  variant="card"
  size="sm"
  ariaLabel="Project sections"
/>
```

## Modal Workflows

Use `FormDialog` or `AlertDialog` directly. Do not build new generic admin
dialog wrappers when the Poodle shells already express the right contract.

## What Stays Out

- route-level navigation context
- entity command modules
- app-local action menus and destructive wording
- data fetching and mutation orchestration
- app-specific admin shell branding

Those stay in host code or in a retained host runtime layer, not in Poodle.

## Decision

- use pure Poodle composition for admin shelling
- keep admin wrappers app-local unless a clearly generic contract emerges
- keep workflow/runtime orchestration out of Poodle unless it is truly design-system-worthy

## Related Contracts

- [PageHeader](../contracts/components/page-header.md)
- [ListContainer](../contracts/components/list-container.md)
- [FilterToolbar](../contracts/components/filter-toolbar.md)
- [FormDialog](../contracts/components/form-dialog.md)
- [MetaBar](../contracts/components/meta-bar.md)
- [MetaItem](../contracts/components/meta-item.md)

## Next Task

Add the next page-shell recipe only when a real generic shell decision is
proven in multiple apps, instead of recreating an Underlay-style wrapper layer
inside Poodle.
