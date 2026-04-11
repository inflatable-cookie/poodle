# Dialog And Detail Recipes

Reusable modal-form and readonly-detail composition rules for Poodle-based
Svelte apps.

## Purpose

Use this guide when contracts tell you what `FormDialog`, `DetailShell`, and
`DetailSection` do, but you need a stable answer for how to assemble everyday
detail pages and modal forms without recreating old app-shell helpers.

## Form Dialogs

### Default posture

- use `FormDialog` for standard modal form workflows
- keep field-level validation inside `Field` and input components
- keep submit orchestration and mutation logic in host code
- do not build app-specific dialog wrappers just to get a title, error banner,
  and standard action row

### Standard modal form

```svelte
<script lang="ts">
  import { FormDialog } from "@poodle/svelte-composites";
  import { Field, TextInput, Select } from "@poodle/svelte-primitives";

  let open = false;
  let submitting = false;
  let error: string | null = null;
</script>

<FormDialog
  bind:open
  title="Invite user"
  description="Add a new user to this workspace."
  submitLabel="Invite"
  cancelLabel="Cancel"
  {submitting}
  {error}
  on:submit={() => {
    submitting = true;
  }}
>
  <Field id="invite-email" label="Email" required>
    <TextInput id="invite-email" type="email" />
  </Field>

  <Field id="invite-role" label="Role" required>
    <Select
      id="invite-role"
      items={[
        { value: "admin", label: "Administrator" },
        { value: "editor", label: "Editor" }
      ]}
    />
  </Field>
</FormDialog>
```

### Decision

- `FormDialog` is the default modal form shell
- `FormLayout` already sits inside it; do not recreate that composition in app
  code unless the dialog is truly non-standard

### Inline detail-route dialogs

For create or edit actions launched from an existing detail route:

- keep the trigger in the local section header or page actions
- keep the dialog title focused on the mutation itself
- use `subtitle` to carry the parent context, for example `For module FA-101`
  or `For handbook.pdf`
- keep submit, close, and refetch orchestration in host code
- after a successful mutation, close the dialog and patch or refetch the parent
  detail surface locally instead of inventing a new shared workflow wrapper

This keeps the visible modal shell stable while leaving mutation sequencing and
detail-page updates app-owned.

## Destructive Confirm Flows

### Default posture

- use `AlertDialog` for destructive confirm and guarded irreversible actions
- keep the title action-specific, for example `Permanently delete version?`
- keep the description focused on the consequence, not generic warning text
- use `itemLabel` and `itemValue` to show the affected record or version
  identifier instead of hand-rolling local inline markup
- use `tone="warning"` for guarded non-destructive state changes like activate,
  promote, or restore
- use `tone="danger"` for irreversible delete and purge actions
- keep success handling, refetch, and local state updates in host code

### Media-detail confirm example

```svelte
<AlertDialog
  bind:open={deleteDialogOpen}
  title="Permanently delete version?"
  description="This will permanently delete this version and its stored file. This cannot be undone."
  itemLabel="Version"
  itemValue={selectedVersionLabel}
  confirmLabel="Permanently delete version"
  tone="danger"
  onConfirm={confirmDelete}
  onCancel={cancelDelete}
/>
```

## Detail Pages

### Default posture

- use `PageHeader` + `MetaBar` for most app-level detail routes
- use top-level `Tabs` when the route has multiple detail/list/workflow surfaces
- use `DetailSection` to group related readonly information under a local
  heading
- use `DetailItem` for label/value presentation
- use local `Card` sections and inline list content under that shell rather than
  introducing a second inner page header
- use `DetailShell` only when the page really wants one composite readonly shell
  to own identity and state handling end-to-end
- keep routing, fetch state, actions, and domain data shaping in host code

### Standard app detail page

```svelte
<script lang="ts">
  import { PageHeader } from "@poodle/svelte-composites";
  import { Card, Code, DetailItem, MetaBar, MetaItem, Pill, Tabs } from "@poodle/svelte-primitives";
  import { DetailSection } from "@poodle/svelte-composites";
</script>

<PageHeader title="Marketing site rebuild" backHref="/projects" backLabel="Back to projects">
  <svelte:fragment slot="actions">
    <!-- host-owned actions -->
  </svelte:fragment>
</PageHeader>

<MetaBar ariaLabel="Project metadata">
  <MetaItem label="ID">
    <Code inline source={project.id} showCopyButton />
  </MetaItem>
  <Pill tone="success" appearance="badge" size="lg">Active</Pill>
</MetaBar>

<Tabs
  value={activeTab}
  items={[
    { value: "details", label: "Details" },
    { value: "tasks", label: "Tasks", count: taskCount }
  ]}
  variant="card"
  size="sm"
  ariaLabel="Project sections"
/>

<Card>
  <DetailSection title="Overview">
    <DetailItem label="Name" value="Marketing site rebuild" />
    <DetailItem label="Owner" value="Alice Johnson" />
    <DetailItem label="Status" value="Active" />
  </DetailSection>

  <DetailSection title="Publishing">
    <DetailItem label="Published at" value="2026-03-25" />
    <DetailItem label="Expires at" emptyText="No expiry set" />
  </DetailSection>
</Card>
```

### Inline related sections

For versions, usages, aliases, notices, and similar related lists:

- keep them as host-owned related-item sections under the stable parent shell
- prefer `InlineListSection` for compact versions/usages/related-item shells
- keep row actions local
- do not add a second inner `PageHeader` or nested route shell inside the
  details tab just to frame those related items

## Detail Actions

Keep page-level actions in `PageHeader` actions. Keep section-local actions in
`DetailSection` actions or local inline-list headers. Do not turn every
page-specific action cluster into a new Poodle wrapper.

```svelte
<DetailSection title="Billing">
  <svelte:fragment slot="actions">
    <Button variant="secondary">Edit billing</Button>
  </svelte:fragment>

  <DetailItem label="Plan" value="Pro" />
</DetailSection>
```

## State Handling

Let the route own loading/error/empty posture around `PageHeader` + `MetaBar` +
`Tabs` in the common app-detail case. Use `DetailShell` when you explicitly
want one composite shell to own ready vs loading vs error vs empty in the same
way `ListContainer` owns browse-page state.

```svelte
<DetailShell
  title="User profile"
  state="loading"
/>
```

Override state slots only when the page needs custom recovery or empty-state
content.

For host-owned detail and tab content:

- use `PageLoading` for loading branches
- use `Callout tone="danger"` for recoverable load failures
- include a small ghost `Retry` action when the route or tab can refetch
  locally
- use compact `EmptyState` for tab-level no-data posture such as usage,
  related-items, or preview gaps
- keep these recovery actions local to the route or tab instead of creating a
  shared retry wrapper

## What Stays Out

- route orchestration
- mutation commands
- confirm-action vocabulary
- domain-specific cards
- page-specific navigation behavior

Those remain host-owned unless Poodle later promotes a narrower generic surface.

## Decision

- `FormDialog` is the default modal form shell
- `PageHeader` + `MetaBar` + optional top-level `Tabs` is the default app
  detail shell
- `DetailShell` is still available when one composite readonly shell is the
  right abstraction
- `DetailSection` and `DetailItem` are the default building blocks for grouped
  detail content and tab-level overview cards

## Related Contracts

- [FormDialog](../contracts/components/form-dialog.md)
- [DetailShell](../contracts/components/detail-shell.md)
- [DetailSection](../contracts/components/detail-section.md)
- [InlineListSection](../contracts/components/inline-list-section.md)
- [DetailItem](../contracts/components/detail-item.md)

## Next Task

Add the next modal/detail guide once destructive-flow and confirm-action
patterns harden in real app work, so teams have a documented rule for when to
compose `Dialog`, `Menu`, and danger-tone buttons directly versus when a new
generic workflow surface is justified.
