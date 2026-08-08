# Slug Field Recipes

Reusable composition rules for slug inputs built on Poodle primitives.

Poodle does not provide a canonical `SlugField` composite. Slug generation,
format checks, reserved-word handling, and uniqueness rules are app-owned
because they depend on route vocabulary, backend validation, and form submit
rules.

## Default Rule

- use `Field` plus `TextInput`
- keep slug generation and submit gating in the form
- use app or shared helpers for `slugify`, reserved-word checks, and format
  checks
- use `TextInput.validate` for async availability checks
- use `validationChange` to drive form-level validity

## Basic Pattern

```svelte
<script lang="ts">
  import { Field, TextInput, type InputValidationStatus, type ValidationResult } from "@inflatable-cookie/poodle-svelte";
  import { slugify, isReservedSlug, isValidSlugFormat } from "@inflatable-cookie/underlay/patterns";

  let title = "";
  let slug = "";
  let lastAutoSlug = "";
  let slugStatus: InputValidationStatus = "idle";
  let slugError: string | null = null;

  $: {
    const nextAutoSlug = slugify(title);
    if (!slug.trim() || slug === lastAutoSlug) {
      slug = nextAutoSlug;
    }
    lastAutoSlug = nextAutoSlug;
  }

  async function validateSlug(value: string): Promise<ValidationResult> {
    const normalized = value.trim();

    if (normalized.length < 2) {
      return { valid: false, message: "Too short (min 2 characters)" };
    }

    if (!isValidSlugFormat(normalized, 64)) {
      return { valid: false, message: "Use lowercase letters, numbers, and hyphens only." };
    }

    if (isReservedSlug(normalized)) {
      return { valid: false, message: "This slug is reserved." };
    }

    return await api.validateSlug(normalized);
  }

  function handleSlugBlur() {
    slug = slugify(slug);
  }
</script>

<Field
  id="slug"
  label="Slug"
  error={slugStatus === "invalid" ? slugError : null}
  validationState={slugStatus === "validating" ? "pending" : slugStatus === "invalid" ? "invalid" : slugStatus === "valid" ? "valid" : "none"}
>
  <TextInput
    id="slug"
    name="slug"
    value={slug}
    autocomplete="off"
    required
    pattern="[a-z0-9]+(?:-[a-z0-9]+)*"
    maxLength={64}
    validate={validateSlug}
    onValueChange={(nextValue) => {
      slug = nextValue;
    }}
    onValidationChange={(detail) => {
      slugStatus = detail.status;
      slugError = detail.status === "invalid" ? detail.message || null : null;
    }}
    onBlur={handleSlugBlur}
  />
</Field>
```

## Auto-Generation Rule

- auto-generate from the source field only while the slug is empty or still
  matches the last generated value
- stop auto-generation once the user diverges from that generated value
- normalize on blur so pasted or edited values still become canonical slugs

## Validation Rule

- do local format and reserved-word checks before async availability checks
- pass sibling IDs or parent IDs through `validationContext` when uniqueness is
  scoped
- use `validationChange` for form submit gating instead of hidden registries

## Keep Out Of Poodle

- route-specific reserved-word lists
- entity-specific uniqueness endpoints
- app vocabulary like `"module"`, `"pathway"`, or `"category"`
- save-intent rules tied to slug validity

## Next Task

Add the next focused recipe for relation and picker-driven form fields so teams
have the same level of migration-backed guidance for selector-heavy forms as
they now do for async validation, form validity, and slug inputs.
