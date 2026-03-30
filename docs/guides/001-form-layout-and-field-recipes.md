# Form Layout And Field Recipes

Reusable form composition rules for Poodle-based Svelte apps.

## Purpose

Use this guide when contracts tell you what `Field`, `FieldSet`, `TextInput`,
or `FormLayout` do, but you need a stable answer for how to compose them in
real forms.

## Form Layout

### Default posture

Use `FormLayout` as the default way to lay out fields in a form.

Use `FieldSet` only when the group itself needs a semantic legend.

Use raw `Grid` only when the layout stops being a normal field layout and
becomes genuinely custom.

### Top-level form

```svelte
<script lang="ts">
  import { FormLayout } from "@poodle/svelte-composites";
  import { Field, TextInput } from "@poodle/svelte-primitives";
</script>

<FormLayout>
  <Field id="title" label="Title" span={3} required>
    <TextInput id="title" />
  </Field>

  <Field id="slug" label="Slug" span={3}>
    <TextInput id="slug" />
  </Field>

  <Field id="description" label="Description" span={6}>
    <TextInput id="description" />
  </Field>
</FormLayout>
```

### Inside a semantic section

```svelte
<script lang="ts">
  import { FormLayout } from "@poodle/svelte-composites";
  import { FieldSet, Field, TextInput } from "@poodle/svelte-primitives";
</script>

<FieldSet legend="Publishing">
  <FormLayout>
    <Field id="publishedAt" label="Publish date" span={3}>
      <TextInput id="publishedAt" />
    </Field>

    <Field id="expiresAt" label="Expiry date" span={3}>
      <TextInput id="expiresAt" />
    </Field>
  </FormLayout>
</FieldSet>
```

### Decision

- `FormLayout` owns field rhythm and responsive column behavior
- `FieldSet` owns semantic grouping
- `Grid` is not the default replacement for old form-grid wrappers

## Field Messaging

### Default posture

- optional markers are opt-in
- validation posture belongs to both `Field` and the control
- async validation should use the built-in `pending`, `valid`, and `invalid`
  states instead of custom suffix icons

### Recommended pattern

```svelte
<script lang="ts">
  import { Field, TextInput } from "@poodle/svelte-primitives";
</script>

<Field
  id="slug"
  label="Slug"
  hint="Lowercase letters, numbers, and hyphens only."
  validationState="pending"
  pendingMessage="Checking availability..."
>
  <TextInput id="slug" validationState="pending" />
</Field>
```

### Decision

- do not show optional labels by default
- do not build one-off async validation affordances outside `TextInput`
- prefer `hint`, `error`, and `pendingMessage` over custom helper blocks

For validator-driven checks, use
[Async Validation Recipes](./005-async-validation-recipes.md).

## Slug Inputs

### Default posture

Poodle should not own a dedicated `SlugField` composite.

Slug generation and source-to-slug synchronization are workflow logic. Poodle
owns the field and text-input capabilities needed to express that workflow.

### Recommended pattern

```svelte
<script lang="ts">
  import { Field, TextInput } from "@poodle/svelte-primitives";

  let title = "";
  let slug = "";
  let isManualSlug = false;

  function slugify(value: string) {
    return value
      .toLowerCase()
      .trim()
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/^-+|-+$/g, "");
  }

  function handleTitleInput(next: string) {
    title = next;
    if (!isManualSlug) slug = slugify(next);
  }

  function handleSlugInput(next: string) {
    isManualSlug = true;
    slug = next;
  }
</script>

<Field id="title" label="Title" span={3} required>
  <TextInput id="title" value={title} on:valueChange={(e) => handleTitleInput(e.detail.value)} />
</Field>

<Field id="slug" label="Slug" span={3}>
  <TextInput id="slug" value={slug} on:valueChange={(e) => handleSlugInput(e.detail.value)} />
</Field>
```

### Decision

- keep slug orchestration in app/shared workflow code
- improve `TextInput` or `Field` if a missing primitive capability blocks this
- do not add a canonical `SlugField` back to Poodle

## Related Contracts

- [Field](../contracts/foundation/field.md)
- [TextInput](../contracts/foundation/text-input.md)
- [FieldSet](../contracts/foundation/field-set.md)
- [FormLayout](../contracts/composites/form-layout.md)

## Next Task

Add the next form guide once another reusable pattern is proven in app work,
most likely a dedicated recipe for relation selectors and form-dialog posture so
teams have a documented rule for when to stay in app code versus when to extend
Poodle.
