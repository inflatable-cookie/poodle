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
- forms: `Field`, `TextInput`, `Select`, `TextArea`, `FormActions`
- status and compact metadata: `Pill`, `Code`, `MetaItem`

## Reference Implementations

Use the ACME admin route family in the separate `underlay-reference`
repository as the concrete delivery reference, especially the project
list/detail/edit/tasks routes plus the user and media detail flows.

## Browse -> Detail -> Edit Posture

```svelte
<script lang="ts">
  import { FilterToolbar, ListContainer, PageHeader } from "@poodle/svelte-composites";
  import { Button, Code, Field, MetaBar, MetaItem, Pill, SearchInput, Tabs } from "@poodle/svelte-primitives";
</script>

<ListContainer title="Projects" state="ready" currentPage={page} totalPages={totalPages} totalItems={totalItems}>
  {#snippet actions()}
    <Button href="/projects/new">Add project</Button>
  {/snippet}

  {#snippet filters()}
    <FilterToolbar summaryText={summaryText}>
      <Field id="projects-search" label="Search">
        <SearchInput id="projects-search" bind:value={query} />
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

## Next Task

Only add more admin-delivery guidance here when a repeated implementation order
or page-shape decision shows up across multiple real apps, instead of turning
this file into a generic project playbook.
