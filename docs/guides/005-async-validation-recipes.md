# Async Validation Recipes

Reusable patterns for async field validation with `Field` and `TextInput`.

## Purpose

Use this guide when a form needs live validation feedback such as slug or handle
availability, uniqueness checks, or remote validation against app-owned rules.

## Default Posture

- `TextInput` owns validation timing and validation status
- `Field` owns the visible message posture
- app code owns the validator function and any workflow-specific context
- use `validationContext` when validation depends on sibling values or existing
  record ids
- use `validationDebounce` for repeated remote checks instead of rebuilding
  debounce wiring at every call site

## Recommended Pattern

```svelte
<script lang="ts">
  import { Field, TextInput } from "@inflatable-cookie/poodle-svelte";
  import type { InputValidationStatus } from "@inflatable-cookie/poodle-svelte";

  let slug = "";
  let slugStatus: InputValidationStatus = "idle";
  let slugError: string | null = null;
  let entryId: string | null = null;

  async function validateSlug(value: string, context?: { entryId: string | null }) {
    const next = value.trim().toLowerCase();
    if (!next) return { valid: true };

    const response = await fetch(`/api/slugs/check?value=${encodeURIComponent(next)}&id=${context?.entryId ?? ""}`);
    const result = await response.json();

    return result.available
      ? { valid: true }
      : { valid: false, message: "That slug is already in use." };
  }
</script>

<Field
  id="slug"
  label="Slug"
  hint="Lowercase letters, numbers, and hyphens only."
  validationState={slugStatus === "validating" ? "pending" : slugStatus === "invalid" ? "invalid" : slugStatus === "valid" ? "valid" : "none"}
  pendingMessage={slugStatus === "validating" ? "Checking availability..." : null}
  error={slugStatus === "invalid" ? slugError : null}
>
  <TextInput
    id="slug"
    value={slug}
    required
    pattern="[a-z0-9-]+"
    validate={validateSlug}
    validationContext={{ entryId }}
    validationDebounce={300}
    onValueChange={(nextValue) => (slug = nextValue)}
    onValidationChange={(detail) => {
      slugStatus = detail.status;
      slugError = detail.message || null;
    }}
  />
</Field>
```

## Decision

- keep validator logic in app/shared workflow code
- surface validation feedback through `Field`, not custom suffix blocks
- prefer `validateOnBlur={true}` for availability checks unless a workflow
  genuinely needs validation only on submit
- do not reintroduce wrapper components just to hide `validationContext` or
  `validationChange`

## When To Extend Poodle

Improve Poodle when repeated apps need a missing generic capability such as:

- more native input attribute passthrough
- richer validation result semantics
- shared validation controller helpers that stay app-agnostic

Do not extend Poodle with app-specific validation rules or endpoint knowledge.

## Related Contracts

- [TextInput](../contracts/components/text-input.md)
- [Field](../contracts/components/field.md)

## Next Task

Add the next focused guide only when another repeated workflow stabilizes,
especially table and picker recipes that replace retained Underlay workflow
shells with direct Poodle composition.
