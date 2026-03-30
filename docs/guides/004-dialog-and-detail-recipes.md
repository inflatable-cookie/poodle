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

## Detail Pages

### Default posture

- use `DetailShell` for page-level readonly identity and state handling
- use `DetailSection` to group related readonly information under a local
  heading
- use `DetailRow` for label/value presentation
- keep routing, fetch state, actions, and domain data shaping in host code

### Standard detail page

```svelte
<script lang="ts">
  import { DetailShell, DetailSection } from "@poodle/svelte-composites";
  import { DetailRow, Button } from "@poodle/svelte-primitives";
</script>

<DetailShell
  title="Project details"
  eyebrow="Projects"
  subtitle="Readonly project metadata and current configuration."
  state="ready"
>
  <svelte:fragment slot="actions">
    <Button variant="secondary">Edit</Button>
  </svelte:fragment>

  <DetailSection title="Overview">
    <DetailRow label="Name" value="Marketing site rebuild" />
    <DetailRow label="Owner" value="Alice Johnson" />
    <DetailRow label="Status" value="Active" />
  </DetailSection>

  <DetailSection title="Publishing">
    <DetailRow label="Published at" value="2026-03-25" />
    <DetailRow label="Expires at" emptyText="No expiry set" />
  </DetailSection>
</DetailShell>
```

## Detail Actions

Keep page-level actions in `DetailShell` actions. Keep section-local actions in
`DetailSection` actions. Do not turn every page-specific action cluster into a
new Poodle wrapper.

```svelte
<DetailSection title="Billing">
  <svelte:fragment slot="actions">
    <Button variant="secondary">Edit billing</Button>
  </svelte:fragment>

  <DetailRow label="Plan" value="Pro" />
</DetailSection>
```

## State Handling

Let `DetailShell` own ready vs loading vs error vs empty posture in the same way
`ListContainer` owns browse-page state.

```svelte
<DetailShell
  title="User profile"
  state="loading"
/>
```

Override state slots only when the page needs custom recovery or empty-state
content.

## What Stays Out

- route orchestration
- mutation commands
- confirm-action vocabulary
- domain-specific cards
- page-specific navigation behavior

Those remain host-owned unless Poodle later promotes a narrower generic surface.

## Decision

- `FormDialog` is the default modal form shell
- `DetailShell` is the default readonly page shell
- `DetailSection` and `DetailRow` are the default building blocks for grouped
  detail content

## Related Contracts

- [FormDialog](../contracts/composites/form-dialog.md)
- [DetailShell](../contracts/composites/detail-shell.md)
- [DetailSection](../contracts/composites/detail-section.md)
- [DetailRow](../contracts/foundation/detail-row.md)

## Next Task

Add the next modal/detail guide once destructive-flow and confirm-action
patterns harden in real app work, so teams have a documented rule for when to
compose `Dialog`, `Menu`, and danger-tone buttons directly versus when a new
generic workflow surface is justified.
