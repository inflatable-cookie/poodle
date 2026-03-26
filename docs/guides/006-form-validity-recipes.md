# Form Validity Recipes

Reusable patterns for form-wide validity when building with Poodle fields and
controls.

## Purpose

Use this guide when a form needs a disabled submit state, save-intent gating,
or section-level validity logic across multiple fields.

## Default Posture

- Poodle owns field-level behavior and messaging
- app or shared workflow code owns form-wide validity
- do not treat form-wide validity as a primitive concern
- do not rebuild a hidden field-registration wrapper just to mirror browser or
  workflow state back into one `isValid` flag

## Recommended Pattern

```svelte
<script lang="ts">
  import { Button, Field, TextInput } from "@poodle/svelte-primitives";
  import type { InputValidationStatus } from "@poodle/svelte-primitives";

  let name = "";
  let slug = "";
  let slugStatus: InputValidationStatus = "idle";
  let slugMessage = "";

  const isFormValid =
    name.trim().length > 0 &&
    slug.trim().length > 0 &&
    slugStatus !== "validating" &&
    slugStatus !== "invalid";
</script>

<Field id="name" label="Name" required>
  <TextInput
    id="name"
    value={name}
    on:valueChange={(event) => {
      name = event.detail.value;
    }}
    required
  />
</Field>

<Field
  id="slug"
  label="Slug"
  required
  validationState={slugStatus === "validating" ? "pending" : slugStatus === "invalid" ? "invalid" : slugStatus === "valid" ? "valid" : "none"}
  error={slugStatus === "invalid" ? slugMessage : null}
  pendingMessage={slugStatus === "validating" ? "Checking availability..." : null}
>
  <TextInput
    id="slug"
    value={slug}
    required
    validate={async (value) => {
      const exists = value === "taken";
      return exists ? { valid: false, message: "That slug is already in use." } : { valid: true };
    }}
    on:valueChange={(event) => {
      slug = event.detail.value;
    }}
    on:validationChange={(event) => {
      slugStatus = event.detail.status;
      slugMessage = event.detail.message;
    }}
  />
</Field>

<Button type="submit" disabled={!isFormValid}>
  Save
</Button>
```

## Decision

- compute form-wide validity from real field values and field validation status
- keep requiredness and cross-field business rules in app/shared code
- use `validationChange` for async field checks instead of hidden provider
  registration
- let browser submission, server errors, and workflow-specific save rules remain
  host-owned

## When To Extend Poodle

Improve Poodle if repeated apps need a generic field-level capability such as:

- richer validator result semantics
- better native input coverage
- shared presentational affordances for pending, valid, and invalid states

Do not extend Poodle with a hidden app-wide form registry unless a generic,
framework-agnostic contract is clearly justified.

## Related Guides

- [Form Layout And Field Recipes](./001-form-layout-and-field-recipes.md)
- [Async Validation Recipes](./005-async-validation-recipes.md)

## Next Task

Add the next focused guide only when another repeated workflow decision is
proven in real migrations, especially picker and table-shell composition.
