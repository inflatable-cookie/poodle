# Admin Feature Delivery Recipes

Implementation-order guidance for delivering real admin features with
Poodle-first UI composition.

## Purpose

Use this guide when the task is not just "what should this one component look
like?" but "how should a full list/detail/edit admin feature be assembled
without rebuilding Underlay-era wrapper layers?"

## Default Delivery Order

1. Define the entity scope and route family.
2. Build backend/data commands before UI wiring.
3. Compose the visible UI directly from Poodle surfaces.
4. Keep app-local menus, redirects, and destructive wording in host code.
5. Validate the route family as one coherent unit instead of testing tiny UI
   fragments in isolation.

## Default UI Stack

- browse pages: `ListContainer` + `FilterToolbar`
- detail pages: `PageHeader` + `MetaBar` + `Tabs`
- modal workflows: `FormDialog` or `AlertDialog`
- forms: `Field`, `TextInput`, `Select`, `FormActions`
- status and compact metadata: `Pill`, `Code`, `MetaItem`
- diagnostics and operational browse pages: `PageHeader` + stats cards +
  filter control + `DataTable`

## Reference Implementations

Use the ACME admin route family in the separate `underlay-reference`
repository as the concrete delivery reference, especially the project
list/detail/edit/tasks routes plus the user and media detail flows.

## Browse -> Detail -> Edit Posture

```svelte
<script lang="ts">
  import { FilterToolbar, ListContainer, PageHeader } from "@inflatable-cookie/poodle-svelte";
  import { Button, Code, Field, MetaBar, MetaItem, Pill, TextInput, Tabs } from "@inflatable-cookie/poodle-svelte";
</script>

<ListContainer title="Projects" state="ready" currentPage={page} totalPages={totalPages} totalItems={totalItems}>
  {#snippet actions()}
    <Button href="/projects/new">Add project</Button>
  {/snippet}

  {#snippet filters()}
    <FilterToolbar summaryText={summaryText}>
      <Field id="projects-search" label="Search">
        <TextInput id="projects-search" type="search" bind:value={query} />
      </Field>
    </FilterToolbar>
  {/snippet}

  <!-- host-owned table/list body -->
</ListContainer>

<PageHeader title={project.name} backHref="/projects" backLabel="Back to projects">
  {#snippet actions()}
    <ProjectActionsMenu {project} />
  {/snippet}
</PageHeader>

<MetaBar ariaLabel="Project metadata">
  <MetaItem label="ID">
    <Code inline source={project.id} />
  </MetaItem>
  <Pill tone={project.active ? "success" : "neutral"} appearance="badge">
    {project.active ? "Active" : "Inactive"}
  </Pill>
</MetaBar>

<Tabs
  value={tab}
  items={[
    { value: "details", label: "Details" },
    { value: "activity", label: "Activity" }
  ]}
  ariaLabel="Project sections"
/>
```

## User Management Pattern

Use this when the admin family needs a user list, user detail route, and
user edit flow with sessions or activity tabs.

Rules for this pattern:

- keep the list page on the standard browse posture: `PageHeader` + `DataTable`
- keep the detail page on `PageHeader` + `MetaBar` + `Tabs variant="card"`
- use badge-style `Pill` status and role chips in both the list and detail
  shells
- render the overview tab as one `Card` with `DetailSection` groups for account
  and security fields
- keep sessions and activity as separate tabbed `DataTable` surfaces with
  row-level actions
- use `SpaFormShell` for edit routes when save vs save-close intent and
  navigation-context return handling are still useful

## Nested Child Form Pattern

Use this when the route creates or edits a child entity inside a parent detail
family, like project tasks or module aliases.

```svelte
<PageHeader
  title="New Task"
  backHref={`/projects/${project.id}`}
  backLabel={`Back to ${project.name}`}
  subtitle={`For project: ${project.name}`}
/>

<Card>
  <form onsubmit={handleSubmit}>
    <Field id="title" label="Title" required>
      <TextInput id="title" bind:value={title} />
    </Field>

    <Field id="description" label="Description">
      <TextInput id="description" bind:value={description} rows={4} />
    </Field>

    <FormActions align="end">
      <Button type="button" variant="secondary">Cancel</Button>
      <Button type="submit">Save</Button>
    </FormActions>
  </form>
</Card>
```

Rules for this pattern:

- keep the route on the normal `PageHeader` shell rather than inventing a
  separate nested-page frame
- show the parent context in the back link and subtitle so the child route is
  never visually detached from its parent
- keep the editable body inside one carded form section
- use direct `form` + `FormActions` for simple single-submit child flows
- switch to `SpaFormShell` only when the child route needs save-vs-save-close,
  delete intent, or navigation-context return orchestration

## Child Collection Pattern

Use this when a parent detail route needs to surface child entities like tasks,
aliases, notices, variants, or similar scoped records.

Rules for this pattern:

- keep compact related child collections inside the parent detail route as one
  local carded section with a title, count badge, add action, empty copy, and
  row-level actions
- keep richer child collections on their own tab or section body when they need
  filters, reorder, selection mode, or batch actions
- use `InlineListSection` only for very small read-mostly related collections
  like versions or usages, not as the default for editable child browse
  surfaces
- avoid introducing a second inner page header for the child collection; the
  parent `PageHeader` and `MetaBar` still own the route shell
- keep tab labels and child counts host-owned rather than pushing those into a
  generic child-collection wrapper

## Diagnostics Browse Pattern

Use this when the page is primarily about operational inspection and safe
actions rather than entity CRUD.

```svelte
<PageHeader title="Job Queue" backHref="/system" backLabel="Back to system">
  {#snippet actions()}
    <Button type="button" variant="ghost" onClick={() => pageData.refetch()}>
      Refresh
    </Button>
  {/snippet}
</PageHeader>

<div class="stats-grid">
  <Card>{/* pending/running/failed/recent cards */}</Card>
</div>

<Field id="system-jobs-status-filter" label="Status">
  <Select id="system-jobs-status-filter" bind:value={statusFilter} options={statusOptions} />
</Field>

<DataTable
  {columns}
  {rows}
  expandedRowIds={rows.filter((row) => row.data.errorMessage).map((row) => row.id)}
  emptyMessage="No jobs found"
  showRowActions={false}
/>
```

Rules for this pattern:

- use `PageHeader` rather than a custom system-page shell
- keep summary metrics as host-owned cards above the table
- keep the filter control simple and local unless multiple filters justify a
  full `FilterToolbar`
- use `expandedRowIds`, not legacy row-predicate props, when rows expose inline
  diagnostics like error text or fetched detail
- keep retry/cancel/refresh behavior in host code; only the visible layout is
  shared

## Review Queue Pattern

Use this when the browse surface is a work queue for review, marking, triage,
or claim/release flows rather than a passive diagnostics table.

Rules for this pattern:

- keep the same `PageHeader` shell as other queue pages, but use a task-oriented
  subtitle that tells operators what work they are continuing
- use a small action-oriented stats band above the table when the queue has
  meaningful staged states like queued, in-progress, or completed
- let those stats act as fast queue filters when that improves triage speed
- keep the filter rail simple: one `FilterToolbar` with local filters and a
  small ghost refresh action
- keep row-level next actions host-owned, such as claim, release, retry, or
  open detail
- use a `DataTable` browse surface for the queue body; only introduce a
  dedicated detail route when the review workflow truly needs drill-in context

## Review Detail Pattern

Use this when the queue has a drill-in route for one submission, review item,
or claimed work unit.

Rules for this pattern:

- keep the same `PageHeader` shell as other admin detail pages, with actions in
  the header for claim, release, retry, refresh, or continue-work behavior
- use a `MetaBar` for the stable queue metadata like ID, status, and source
- start the body with one carded summary block using `DetailSection` and
  `DetailItem` for identifiers, timestamps, and current claim state
- follow that summary with app-owned cards for answer content, AI results,
  reviewer forms, or scoring history
- treat richer content-inspection routes as local exceptions; do not force every
  assessment or workflow detail page into the same minimal ops-detail shape

## Ops Detail Pattern

Use this when a system page drills into one job, scheduled task, or operational
record.

```svelte
<PageHeader title={jobTypeLabel} backHref="/system/jobs" backLabel="Back to jobs">
  {#snippet actions()}
    <Button variant="secondary">Cancel</Button>
    <Button variant="primary">Retry</Button>
    <IconButton variant="secondary" icon="refresh-cw" ariaLabel="Refresh job" />
  {/snippet}
</PageHeader>

<MetaBar ariaLabel="Job metadata">
  <MetaItem label="ID">
    <Code inline source={job.id} showCopyButton />
  </MetaItem>
  <Pill tone="danger" appearance="badge" size="lg">Failed</Pill>
</MetaBar>

<Card>
  <DetailSection title="Details" columns={2} separated={false}>
    <DetailItem presentation="surface" label="Type">
      <Code inline source={job.jobType} />
    </DetailItem>
    <DetailItem presentation="surface" label="Attempts" value={`${job.attempts} / ${job.maxAttempts}`} />
  </DetailSection>

  <DetailSection title="Timestamps" columns={2} separated={false}>
    <DetailItem presentation="surface" label="Created" value={createdAtLabel} />
    <DetailItem presentation="surface" label="Finished" value={finishedAtLabel} />
  </DetailSection>
</Card>

<Card>
  <h3>Payload</h3>
  <pre>{jsonPayload}</pre>
</Card>
```

Rules for this pattern:

- keep the same `PageHeader` + `MetaBar` shell as the rest of the admin detail
  family
- keep retry/cancel/trigger/refresh actions in the header, not in a secondary
  action row below it
- use one carded summary block for details and timestamps before payload/error
  sections
- use additional `Card` sections for worker identity, last error, payload,
  progress, and history blocks
- keep JSON/code content host-owned even when the surrounding section posture is
  shared
- use a dedicated tab only when the detail page truly has a second collection
  surface, like task job runs
- for error logs, prefer an expandable `DataTable` detail row as the default
  inspection flow
- keep a dedicated error-detail route only when the app needs a permalink or
  cross-navigation target, and reuse the same metadata/code posture if it
  exists

## Trash Recovery Pattern

Use this when the primary task is recovering or purging soft-deleted records.

```svelte
<PageHeader title="Media Trash" backHref="/media" backLabel="Back to media" />

<p class="trash-info">
  Items in trash can be restored or permanently deleted. Permanently deleted
  items cannot be recovered.
</p>

<ListGrid minItemWidth={26}>
  <ListCard title={item.title} href={`/media/${item.id}`}>
    {#snippet leading()}
      <MediaThumbnail kind={item.kind} presentation="compact" />
    {/snippet}

    {#snippet trailing()}
      <Pill tone="neutral" appearance="badge">{item.kindLabel}</Pill>
      <Pill tone="danger" appearance="badge">Deleted</Pill>
    {/snippet}

    {#snippet footer()}
      <span>Deleted {item.deletedAtLabel}</span>
    {/snippet}

    {#snippet actions()}
      <div class="trash-actions">
        <Button type="button" variant="ghost" size="sm">Restore</Button>
        <Button type="button" variant="ghost" tone="danger" size="sm">Delete</Button>
      </div>
    {/snippet}
  </ListCard>
</ListGrid>

<AlertDialog
  title="Permanently delete media?"
  confirmLabel="Delete forever"
  tone="danger"
/>
```

Rules for this pattern:

- use `ListGrid` for card-based trash pages instead of ad hoc grid markup
- show the reversible/irreversible lifecycle note above the grid, not buried in
  dialog copy
- keep `Restore` as the fast action and `Delete` guarded by `AlertDialog`
- keep item metadata compact: kind, deleted state, deleted date, and one stable
  identifier or title
- keep purge wording and permissions app-owned even when the visible layout is
  shared

## Selection And Bulk Actions Pattern

Use this when a browse surface needs explicit selection mode for destructive or
transform-style actions.

```svelte
<PageHeader title="Media Library">
  {#snippet actions()}
    {#if items.length > 0}
      <IconButton
        type="button"
        variant="secondary"
        tone={isSelectionMode ? "danger" : "default"}
        icon={squareCheckIcon}
        ariaLabel={isSelectionMode ? "Cancel selection" : "Select items"}
        tooltip={isSelectionMode ? "Cancel Selection" : "Select Items"}
        onClick={toggleSelectionMode}
      />
    {/if}
    {#if !isSelectionMode}
      <IconButton type="button" variant="secondary" tone="danger" icon={trash2Icon} ariaLabel="View trash" />
      <IconButton type="button" variant="primary" icon={uploadIcon} ariaLabel="Add item" />
    {/if}
  {/snippet}
</PageHeader>

<ListCard
  href={isSelectionMode ? undefined : detailHref}
  selectable={isSelectionMode}
  selected={selection.isSelected(item.id)}
  onSelectedChange={(selected) => {
    if (isSelectionMode) selection.toggle(item.id, selected);
  }}
/>

<BulkActionBar
  selectionCount={selection.count}
  totalCount={items.length}
  actions={batchActions}
  showSelectAll
  allSelected={selection.count > 0 && selection.count === items.length}
  onClear={handleClearSelection}
  onSelectAll={handleSelectAll}
  onAction={handleBatchAction}
/>
```

Rules for this pattern:

- keep selection mode explicit instead of making cards always selectable
- hide normal create/trash/row-action affordances while selection mode is active
- render `BulkActionBar` only when something is selected; do not pin it as
  permanent page chrome
- treat selection mode and reorder mode as mutually exclusive workflows
- keep destructive confirmation and command execution in host code
- use one `handleSelectAll` path that toggles between selecting visible items
  and clearing the current selection

## Rules

- keep routing, API commands, and navigation context in host code
- do not create new shared page-shell wrappers when `PageHeader`, `MetaBar`,
  `Tabs`, and `ListContainer` already express the right contract
- add to Poodle only when the missing behavior is generic and reusable across
  multiple apps
- prefer app-local composition over a new shared wrapper when the difference is
  workflow vocabulary rather than visual semantics

## What Stays Out

- entity command modules
- navigation-context orchestration
- auth/session requirements
- route-specific destructive wording
- permission gating

Those remain host-owned or live in retained runtime/workflow layers outside
Poodle.

## Related Guides

- [Page Shell And Admin Recipes](./011-page-shell-and-admin-recipes.md)
- [Form Layout And Field Recipes](./001-form-layout-and-field-recipes.md)
- [List And Filter Recipes](./003-list-and-filter-recipes.md)
- [Dialog And Detail Recipes](./004-dialog-and-detail-recipes.md)
- [Admin App Shell Recipes](./014-admin-app-shell-recipes.md)
